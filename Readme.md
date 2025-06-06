Chaotic image encryption using the logistic map.

Build with:
```sh
cargo build --release
```

Usage:
```sh
# encode 
./target/release/chaotic-enc -i original.png -o encoded.png -s your_secret_key

# decode, the same command
./target/release/chaotic-enc -i encoded.png -o recovered.png -s your_secret_key
```