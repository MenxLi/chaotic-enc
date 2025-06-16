use crate::logistic_map;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    fn error(s: &str);
}
macro_rules! console_log { ($($t:tt)*) => (log(&format_args!($($t)*).to_string())) }
macro_rules! console_err { ($($t:tt)*) => (error(&format_args!($($t)*).to_string())) }

pub fn inject_lsb(
    im: &mut [u8],
    msg: &str,
    encryption_seed: Option<f64>, 
) -> Result<(), String> {

    let msg_vec_raw: Vec<u8> = msg.as_bytes().to_vec();
    let msg_len_encode = (msg_vec_raw.len() as u32).to_le_bytes();

    let mut msg_vec = vec![0; (im.len() + 7) / 8];
    console_log!("Message channel length (encode): {}", msg_vec.len());
    if msg_vec.len() < msg_len_encode.len() + msg_vec_raw.len() {
        return Err("Channel capacity not enough for message".to_string());
    }

    // Copy the message length and raw message into the padded vector
    for (i, &byte) in msg_len_encode.iter().enumerate() {
        msg_vec[i] = byte;
    }
    for (i, &byte) in msg_vec_raw.iter().enumerate() {
        msg_vec[i + msg_len_encode.len()] = byte;
    }

    if let Some(seed) = encryption_seed {
        msg_vec = logistic_map::encode::<1>(&msg_vec, seed);
    }

    for i in 0..im.len() {
        let msg_idx = i / 8;
        let bit_idx = i % 8;

        if msg_idx > msg_vec.len() - 1 {
            break; // No more bits to inject
        }

        let bit = (msg_vec[msg_idx] >> (7 - bit_idx)) & 1;
        im[i] = im[i] & 0xFE | bit; 
    }

    Ok(())
}

pub fn extract_lsb(
    im: &[u8],
    encryption_seed: Option<f64>,
) -> Result<String, String> {
    if im.len() < 8 {
        return Err("Image too small to contain message".to_string());
    }

    let mut msg_bits = Vec::new();
    for i in 0..im.len() {
        let bit = im[i] & 1; // Extract the least significant bit
        msg_bits.push(bit);
    }
    let npad = msg_bits.len() % 8;
    if npad != 0 {
        msg_bits.extend(vec![0; 8 - npad]);
    }

    let mut msg_vec = Vec::new();
    for i in (0..msg_bits.len()).step_by(8) {
        let byte = msg_bits[i..i + 8].iter().fold(0, |acc, &bit| (acc << 1) | bit);
        msg_vec.push(byte);
    }
    console_log!("Message channel length (decode): {}", msg_vec.len());

    if let Some(seed) = encryption_seed {
        msg_vec = logistic_map::decode::<1>(&msg_vec, seed);
    }

    let msg_len = u32::from_le_bytes(msg_vec[0..4].try_into().unwrap()) as u32;
    if 4 + msg_len > msg_vec.len() as u32 {
        console_err!("Message length: {}, message channel length: {}", msg_len, msg_vec.len());
        return Err("Failed to decode message length".to_string());
    }

    let msg_raw = &msg_vec[4..4 + msg_len as usize];

    String::from_utf8(msg_raw.to_vec()).map_err(|e| e.to_string())
}