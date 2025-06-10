const R: f64 = 4.0;

struct LogisticMapOptions {
    x: f64,
    r: f64,
    size: usize,
}

fn generate_map(opt: LogisticMapOptions) -> Vec<f64> {
    let mut pixels = Vec::with_capacity(opt.size);
    let mut x = opt.x;

    for _ in 0..opt.size {
        x = opt.r * x * (1.0 - x);
        let color_value = x;
        pixels.push(color_value);
    };
    pixels
}

fn argsort<T: Ord>(vec: &Vec<T>) -> Vec<usize> {
    let mut indices: Vec<usize> = (0..vec.len()).collect();
    indices.sort_by(|&a, &b| vec[a].cmp(&vec[b]));
    indices
}

fn confuse_xor(im: &Vec<u8>, enc_map: &Vec<f64>) -> Vec<u8> {
    let mut enc_pixels = Vec::with_capacity(im.len());
    for (i, pixel) in enc_map.iter().map(
        |&x| (x * 255.0) as u8
    ).enumerate() {
        enc_pixels.push(im[i] ^ pixel);
    }
    enc_pixels
}

enum DiffuseDirection {
    Forward,
    Backward,
}
fn diffuse<T: Copy>(im: &Vec<T>, enc_map: &Vec<f64>, direction: DiffuseDirection, chunk_size: usize) -> Vec<T> 
{
    let enc_map = enc_map[..im.len()/chunk_size]
        .iter()
        .map(|&x| x.to_bits())
        .collect();
    let indices = argsort(&enc_map);
    let mut diffuse_pixels = Vec::with_capacity(im.len());

    match direction {
        DiffuseDirection::Forward => {
            for &index in &indices {
                for i in 0..chunk_size {
                    let index = index * chunk_size + i;
                    diffuse_pixels.push(im[index]);
                }
            }
        },
        DiffuseDirection::Backward => {
            let mut lookup: Vec<usize> = vec![0; im.len() / chunk_size];
            for (i, &index) in indices.iter().enumerate() {
                lookup[index] = i;
            }
            for &index in &lookup {
                for i in 0..chunk_size {
                    let index = index * chunk_size + i;
                    diffuse_pixels.push(im[index]);
                }
            }
        },
    }
    diffuse_pixels
}

// C: channel size, e.g. 3 for RGB
pub fn encode<const C: usize>(im: &Vec<u8>, x0: f64) -> Vec<u8> {
    let enc_map = generate_map(LogisticMapOptions {
        x: x0,
        r: R,  
        size: im.len(),
    });
    let im = diffuse(&im, &enc_map, DiffuseDirection::Forward, C);
    confuse_xor(&im, &enc_map)
}

pub fn decode<const C:usize>(im: &Vec<u8>, x0: f64) -> Vec<u8> {
    let enc_map = generate_map(LogisticMapOptions {
        x: x0,
        r: R,  
        size: im.len(),
    });
    let im = confuse_xor(&im, &enc_map);
    diffuse(&im, &enc_map, DiffuseDirection::Backward, C)
}