wasm:
	@echo "Building WASM module..."
	wasm-pack build --target web --out-dir ./app/pkg

.PHONY: wasm