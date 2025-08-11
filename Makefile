.PHONY: install browsers wasm bench

install:
	npm install

browsers: install
	npx playwright install firefox chromium
	npx playwright install-deps firefox chromium

wasm:
	rustup target add wasm32-unknown-unknown 2>/dev/null || true
	if ! command -v wasm-bindgen >/dev/null 2>&1; then cargo install wasm-bindgen-cli; fi
	cargo build --target wasm32-unknown-unknown --release -p sd_wasm
	wasm-bindgen --target web --out-dir pkg target/wasm32-unknown-unknown/release/sd_wasm.wasm

bench: wasm browsers
	node bench.js
