Chaotic image encryption using the logistic map.

Build with:
```sh
cargo build --release
```

Usage:
```sh
# encode 
./target/release/cenc -i original.png -o encoded.png -s your_secret_key

# decode
./target/release/cdec -i encoded.png -o recovered.png -s your_secret_key
```