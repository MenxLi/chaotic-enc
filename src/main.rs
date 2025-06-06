mod imlib;
mod logistic_map;

use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    input: String,

    #[arg(short, long)]
    output: String,

    #[arg(short, long, default_value = "0.70827894")]
    x: f64,
}

fn main() {
    let args = Args::parse();

    let (im, shape) = imlib::load_image(&args.input).expect("Failed to load image");

    let enc_im = logistic_map::encode(im, args.x);

    imlib::save_image(&args.output, enc_im, shape).expect("Failed to save image");
}
