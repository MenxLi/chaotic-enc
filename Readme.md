A simple image encryption and decryption tool. 

The code runs at browser side, using WebAssembly.   
The encryption is based on Chaotic Logistic Map.

A hosted version is available at [here](https://menxli.github.io/chaotic-enc/).

To build for your own use, you need to install Rust and wasm-pack.  
Then run `make wasm`. The frontend will be built at `app/` directory.