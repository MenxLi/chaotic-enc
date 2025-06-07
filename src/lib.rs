mod logistic_map;

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::u8;
use wasm_bindgen::prelude::*;
use image::{self, ImageEncoder};

pub struct ImageOptions {
    pub width: u32,
    pub height: u32,
}

fn str2f(s: &str) -> f32 {
    let mut hasher = DefaultHasher::new();
    s.hash(&mut hasher);
    let hash = hasher.finish();
    let x = (hash as f32) / (u64::MAX as f32);
    x
}

fn img2vec(im: &[u8]) -> (Vec<[u8; 3]>, ImageOptions) {
    let img = image::load_from_memory(im).expect("Failed to load image");
    let rgb = img.to_rgb8();
    let mut pixels = Vec::with_capacity(rgb.width() as usize * rgb.height() as usize);
    for pixel in rgb.pixels() {
        pixels.push([pixel[0], pixel[1], pixel[2]]);
    };
    
    (pixels, ImageOptions {
        width: rgb.width(),
        height: rgb.height(),
    })
}

fn vec2pngblob(pixels: &Vec<[u8; 3]>, im_opt: ImageOptions) -> Box<[u8]> {
    let ImageOptions { width, height } = im_opt;

    let mut img = image::RgbImage::new(width, height);
    for (i, pixel) in pixels.iter().enumerate() {
        let x = (i % width as usize) as u32;
        let y = (i / width as usize) as u32;
        img.put_pixel(x, y, image::Rgb(*pixel));
    };

    // Encode the image to PNG format
    let mut buf = Vec::new();
    let encoder = image::codecs::png::PngEncoder::new(&mut buf);
    encoder.write_image(
        &img,
        width,
        height,
        image::ExtendedColorType::Rgb8,
    ).expect("Failed to encode image");

    buf.into_boxed_slice()
}

#[wasm_bindgen]
pub fn encode(im: &[u8], secret: &str) -> Box<[u8]> {
    let (mut im_v, im_opt) = img2vec(im);

    let pixels = logistic_map::encode(
        &mut im_v,
        str2f(&secret)
    );

    vec2pngblob(&pixels, im_opt)
}

#[wasm_bindgen]
pub fn decode(im: &[u8], secret: &str) -> Box<[u8]> {
    let (im_v, im_opt) = img2vec(im);

    let pixels = logistic_map::decode(
        &im_v,
        str2f(&secret)
    );

    vec2pngblob(&pixels, im_opt)
}