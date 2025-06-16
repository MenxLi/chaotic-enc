mod logistic_map;
mod stega;

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use image::{self, ImageEncoder};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}
macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

pub struct ImageOptions {
    pub width: u32,
    pub height: u32,
}

fn str2f(s: &str) -> f64 {
    let mut hasher = DefaultHasher::new();
    s.hash(&mut hasher);
    let hash = hasher.finish();
    (hash as f64) / (u64::MAX as f64)
}

fn maybe_resize(img: image::RgbImage, max_side: u32) -> (image::RgbImage, u32, u32) {
    let (width, height) = img.dimensions();

    if width > max_side || height > max_side {
        let scale = max_side as f64 / width.max(height) as f64;
        let new_width = (width as f64 * scale).round() as u32;
        let new_height = (height as f64 * scale).round() as u32;
        let resized_img = image::imageops::resize(
            &img, new_width, new_height, 
            image::imageops::FilterType::Lanczos3
        );
        (resized_img, new_width, new_height)
    } else {
        (img, width, height)
    }
}

fn img2vec(im: &[u8], limit_max_side: Option<u32>) -> Result<(Vec<u8>, ImageOptions), String> {
    let img = image::load_from_memory(im)
        .map_err(|e| format!("Failed to load image: {}", e))?;
    let mut rgb = img.to_rgb8();

    if let Some(max_side) = limit_max_side {
        (rgb, _, _) = maybe_resize(rgb, max_side);
    }

    let mut pixels = Vec::with_capacity(rgb.width() as usize * rgb.height() as usize * 3);
    for pixel in rgb.pixels() {
        pixels.push(pixel[0]);
        pixels.push(pixel[1]);
        pixels.push(pixel[2]);
    };

    Ok((pixels, ImageOptions {
        width: rgb.width(),
        height: rgb.height(),
    }))
}

enum ImageType {
    Png, 
    Jpeg,
}

fn vec2imblob(pixels: &Vec<u8>, im_opt: ImageOptions, limit_max_side: Option<u32>, im_type: ImageType) -> Result<Box<[u8]>, String> {
    let ImageOptions { mut width, mut height } = im_opt;
    let mut img = image::RgbImage::new(width, height);

    for (i, pixel) in pixels.chunks(3).enumerate() {
        let x = (i % width as usize) as u32;
        let y = (i / width as usize) as u32;
        img.put_pixel(x, y, image::Rgb([
            pixel[0],
            pixel[1],
            pixel[2],
        ]));
    };

    if let Some(max_side) = limit_max_side {
        (img, width, height) = maybe_resize(img, max_side);
    }

    let mut buf = Vec::new();
    match im_type {
        ImageType::Png => {
            image::codecs::png::PngEncoder::new(&mut buf)
                .write_image(&img, width, height, image::ExtendedColorType::Rgb8)
                .map_err(|e| format!("Failed to write PNG image - {}", e))?;
        },
        ImageType::Jpeg => {
            image::codecs::jpeg::JpegEncoder::new_with_quality(&mut buf, 80)
                .write_image(&img, width, height, image::ExtendedColorType::Rgb8)
                .map_err(|e| format!("Failed to write JPEG image - {}", e))?;
        },
    }
    console_log!("Export image, dimensions: {}x{}", width, height);

    Ok(buf.into_boxed_slice())
}

#[wasm_bindgen]
pub fn encode(im: &[u8], secret: &str, max_side: i32, as_type: &str) -> Result<Box<[u8]>, String> {
    console_log!("Encoding image with secret: {}, max_side: {}", secret, max_side);
    let max_side = if max_side < 1 { None } else { Some(max_side as u32) };

    let seed = str2f(&secret);
    console_log!("Seed: {}", seed);

    let (im_v, im_opt) = img2vec(im, max_side)?;
    let pixels = logistic_map::encode::<3>(&im_v, seed);
    vec2imblob(&pixels, im_opt, None, match as_type {
        "png" => ImageType::Png,
        "jpeg" => ImageType::Jpeg,
        _ => panic!("Unsupported image type: {}", as_type),
    })
}

#[wasm_bindgen]
pub fn decode(im: &[u8], secret: &str, max_side: i32, as_type: &str) -> Result<Box<[u8]>, String> {
    console_log!("Decoding image with secret: {}, max_side: {}", secret, max_side);
    let max_side = if max_side < 1 { None } else { Some(max_side as u32) };

    let seed = str2f(&secret);
    console_log!("Seed: {}", seed);

    let (im_v, im_opt) = img2vec(im, None)?;
    let pixels = logistic_map::decode::<3>(&im_v, seed);
    vec2imblob(&pixels, im_opt, max_side, match as_type {
        "png" => ImageType::Png,
        "jpeg" => ImageType::Jpeg,
        _ => panic!("Unsupported image type: {}", as_type),
    })
}

#[wasm_bindgen]
pub fn stega_encode(
    im: &[u8], 
    message: &str,
    secret: &str,
    max_side: i32, 
    ) -> Result<Box<[u8]>, String> 
{

    console_log!("Encoding stega image");
    let max_side = if max_side < 1 { None } else { Some(max_side as u32) };
    let seed: Option<f64> = match secret {
        "" => None,
        s => {
            console_log!("Seed: {}", s);
            Some(str2f(s))
        },
    };

    let (mut im_v, im_opt) = img2vec(im, max_side)?;
    stega::inject_lsb(&mut im_v[..], message, seed)?;

    vec2imblob(&im_v, im_opt, None, ImageType::Png)
}

#[wasm_bindgen]
pub fn stega_decode(
    im: &[u8],
    secret: &str,
    max_side: i32,
) -> Result<String, String> {
    console_log!("Decoding stega image");
    let max_side = if max_side < 1 { None } else { Some(max_side as u32) };
    let seed: Option<f64> = match secret {
        "" => None,
        s => {
            console_log!("Seed: {}", s);
            Some(str2f(s))
        },
    };

    let (im_v, _) = img2vec(im, max_side)?;
    stega::extract_lsb(&im_v[..], seed)
}