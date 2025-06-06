

const R: f64 = 4.0;

struct LogisticMapOptions {
    x: f64,
    r: f64,
    size: usize,
}

fn generate_map(opt: LogisticMapOptions) -> Vec<u8> {
    let mut pixels = Vec::<u8>::with_capacity(opt.size);
    let mut x = opt.x;

    for _ in 0..opt.size {
        x = opt.r * x * (1.0 - x);
        let color_value = (x * 255.0) as u8;
        pixels.push(color_value);
    };

    pixels
}

pub fn encode(im: &Vec<[u8; 3]>, x0: f64) -> Vec<[u8; 3]> {
    let enc_map = generate_map(LogisticMapOptions {
        x: x0,
        r: R,  
        size: im.len(),
    });

    let mut enc_pixels = Vec::with_capacity(im.len());

    for (i, pixel) in enc_map.iter().enumerate() {
        // use xor to encode the pixel
        let r = im[i][0] ^ pixel;
        let g = im[i][1] ^ pixel;
        let b = im[i][2] ^ pixel;
        enc_pixels.push([r, g, b]);
    }

    enc_pixels
}

// pub fn decode(enc_im: Vec<[u8; 3]>, x0: f64) -> Vec<[u8; 3]> {
//     encode(enc_im, x0) // Decoding is the same as encoding
// }