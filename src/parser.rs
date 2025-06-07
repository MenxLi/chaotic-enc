
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

fn str2f(s: &str) -> f32 {
    let mut hasher = DefaultHasher::new();
    s.hash(&mut hasher);
    let hash = hasher.finish();
    let x = (hash as f32) / (u64::MAX as f32);
    x
}

pub struct FuncArgs {
    pub input: String,
    pub output: String,
    pub seed: f32,
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