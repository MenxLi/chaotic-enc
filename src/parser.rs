
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

pub struct FuncArgs {
    pub input: String,
    pub output: String,
    pub seed: f64,
}

pub fn parse_args() -> FuncArgs {
    let args = Args::parse();

    let seed = str2f(&args.secret);

    FuncArgs {
        input: args.input,
        output: args.output,
        seed,
    }
}