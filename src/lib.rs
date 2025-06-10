mod logistic_map;

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::u8;
use wasm_bindgen::prelude::*;
use image::{self, ImageEncoder};

// https://rustwasm.github.io/wasm-bindgen/examples/console-log.html
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}
macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

pub struct ImageOptions {
    pub width: u32,
    pub height: u32,
    pub channels: u32,
}

fn str2f(s: &str) -> f64 {
    let mut hasher = DefaultHasher::new();
    s.hash(&mut hasher);
    let hash = hasher.finish();
    let x = (hash as f64) / (u64::MAX as f64);
    x
}

fn img2vec(im: &[u8], limit_max_side: Option<u32>) -> (Vec<u8>, ImageOptions) {
    let img = image::load_from_memory(im).expect("Failed to load image");
    let mut rgb = img.to_rgb8();

    if let Some(max_side) = limit_max_side {
        let (mut width, mut height) = rgb.dimensions();
        if width > max_side || height > max_side {
            let scale = max_side as f64 / width.max(height) as f64;
            width = (width as f64 * scale).round() as u32;
            height = (height as f64 * scale).round() as u32;
            rgb = image::imageops::resize(
                &rgb, width, height, 
                image::imageops::FilterType::Lanczos3
            );
        }
    }

    let mut pixels = Vec::with_capacity(rgb.width() as usize * rgb.height() as usize * 3);
    for pixel in rgb.pixels() {
        pixels.push(pixel[0]);
        pixels.push(pixel[1]);
        pixels.push(pixel[2]);
    };
    
    (pixels, ImageOptions {
        width: rgb.width(),
        height: rgb.height(),
        channels: 3,
    })
}

enum ImageType {
    Png, 
    Jpeg,
}

fn vec2imblob(pixels: &Vec<u8>, im_opt: ImageOptions, limit_max_side: Option<u32>, im_type: ImageType) -> Box<[u8]> {
    let ImageOptions { mut width, mut height , channels} = im_opt;

    let mut img = image::RgbImage::new(width, height);
    for (i, pixel) in pixels.chunks(channels as usize).enumerate() {
        let x = (i % width as usize) as u32;
        let y = (i / width as usize) as u32;
        img.put_pixel(x, y, image::Rgb([
            pixel[0],
            pixel[1],
            pixel[2],
        ]));
    };

    if let Some(max_side) = limit_max_side {
        if width > max_side || height > max_side {
            let scale = max_side as f64 / width.max(height) as f64;
            width = (width as f64 * scale).round() as u32;
            height = (height as f64 * scale).round() as u32;
            img = image::imageops::resize(
                &img, width, height, 
                image::imageops::FilterType::Lanczos3
            );
        }
    }

    // Encode the image to PNG format
    let mut buf = Vec::new();
    match im_type {
        ImageType::Png => {
            image::codecs::png::PngEncoder::new(&mut buf)
                .write_image(&img, width, height, image::ExtendedColorType::Rgb8)
                .expect("Failed to encode PNG image");
        },
        ImageType::Jpeg => {
            image::codecs::jpeg::JpegEncoder::new_with_quality(&mut buf, 90)
                .write_image(&img, width, height, image::ExtendedColorType::Rgb8)
                .expect("Failed to encode JPEG image");
        },
    }

    console_log!("Export image, dimensions: {}x{}", width, height);
    buf.into_boxed_slice()
}

#[wasm_bindgen]
pub fn encode(im: &[u8], secret: &str, max_side: i32, as_type: &str) -> Box<[u8]> {
    console_log!("Encoding image with secret: {}, max_side: {}", secret, max_side);
    let max_side = if max_side < 1 { None } else { Some(max_side as u32) };

    let (im_v, im_opt) = img2vec(im, max_side);
    let seed = str2f(&secret);
    console_log!("Seed: {}", seed);

    let pixels = logistic_map::encode(&im_v, seed);

    vec2imblob(&pixels, im_opt, None, match as_type {
        "png" => ImageType::Png,
        "jpeg" => ImageType::Jpeg,
        _ => panic!("Unsupported image type: {}", as_type),
    })
}

#[wasm_bindgen]
pub fn decode(im: &[u8], secret: &str, max_side: i32, as_type: &str) -> Box<[u8]> {
    console_log!("Decoding image with secret: {}, max_side: {}", secret, max_side);
    let max_side = if max_side < 1 { None } else { Some(max_side as u32) };

    let (im_v, im_opt) = img2vec(im, None);
    let seed = str2f(&secret);
    console_log!("Seed: {}", seed);

    let pixels = logistic_map::decode(&im_v, seed);

    vec2imblob(&pixels, im_opt, max_side, match as_type {
        "png" => ImageType::Png,
        "jpeg" => ImageType::Jpeg,
        _ => panic!("Unsupported image type: {}", as_type),
    })
}