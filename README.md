# sd-proto-bench

This repository contains a small Wasm example showing how to use
[libcrux](https://crates.io/crates/libcrux) and
[hpke-rs](https://crates.io/crates/hpke-rs) together.

The crate `sd_wasm` exposes two functions to JavaScript using
`wasm-bindgen`:

- `generate_x25519_keypair()` – generate an X25519 key pair using libcrux.
- `hpke_encrypt(pk_r, msg)` – encrypt a message using HPKE with the
  libcrux crypto backend.

To build the WebAssembly package you need `wasm-bindgen-cli` installed:

```bash
cargo install wasm-bindgen-cli
```

Then build and generate the JS bindings:

```bash
cargo build --target wasm32-unknown-unknown --release -p sd_wasm
wasm-bindgen --target web --out-dir pkg target/wasm32-unknown-unknown/release/sd_wasm.wasm
```

Open `www/index.html` in a browser to test the functions.
