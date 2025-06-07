mod imlib;
mod parser;
mod logistic_map;

fn main() {
    let args = parser::parse_args();

    let (im, shape) = imlib::load_image(&args.input).expect("Failed to load image");

    let enc_im = logistic_map::decode(&im, args.seed);

    imlib::save_image(&args.output, enc_im, shape).expect("Failed to save image");
}
