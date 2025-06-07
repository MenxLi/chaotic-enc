mod imlib;
mod logistic_map;

use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;
use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    input: String,

    #[arg(short, long)]
    output: String,

    #[arg(short, long)]
    secret: String,
}

fn str2f(s: &str) -> f64 {
    let mut hasher = DefaultHasher::new();
    s.hash(&mut hasher);
    let hash = hasher.finish();
    let x = (hash as f64) / (u64::MAX as f64);
    x
}

fn main() {
    let args = Args::parse();

    let (im, shape) = imlib::load_image(&args.input).expect("Failed to load image");

    let seed = str2f(&args.secret);

    let enc_im = logistic_map::decode(&im, seed);

    imlib::save_image(&args.output, enc_im, shape).expect("Failed to save image");
}
