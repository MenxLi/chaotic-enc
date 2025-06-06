
use image;

pub struct ImageOptions {
    pub width: u32,
    pub height: u32,
}

pub fn load_image(path: &str) -> Result<(Vec<[u8; 3]>, ImageOptions), &'static str> {
    let img = image::ImageReader::open(path).map_err(|_| "Failed to open image")?
        .decode().map_err(|_| "Failed to decode image")?;
    let img = img.to_rgb8();
    let (width, height) = img.dimensions();
    let mut pixels = Vec::with_capacity((width * height) as usize);
    for y in 0..height {
        for x in 0..width {
            let pixel = img.get_pixel(x, y);
            pixels.push([pixel[0], pixel[1], pixel[2]]); 
        }
    }
    Ok((pixels, ImageOptions { width, height }))
}

pub fn save_image(path: &str, pixels: Vec<[u8; 3]>, im_dim: ImageOptions) -> Result<(), &'static str> {
    let ImageOptions { width, height } = im_dim;
    assert_eq!(pixels.len() as u32, width * height);
    let mut img = image::RgbImage::new(width, height);
    for (i, pixel) in pixels.iter().enumerate() {
        let x = (i as u32) % width;
        let y = (i as u32) / width;
        img.put_pixel(x, y, image::Rgb(*pixel));
    }
    img.save(path).map_err(|_| "Failed to save image")?;
    Ok(())
}
