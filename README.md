# rust-wasm-experiment
Experimenting with WASM using rust
Try to create a random file in /tmp directory

```
// Download and install the necessary components for compiling Rust code into WebAssembly that targets the WASI runtime. 
rustup target add wasm32-wasi

// Compile app specifically for the WebAssembly and WASI runtime environment.
cargo build --target wasm32-wasi

// run wasm without permissions which must fail
wasmtime run target/wasm32-wasi/debug/save-random-file.wasm

// run wasm with permissions which passes
wasmtime run --dir=/tmp target/wasm32-wasi/debug/save-random-file.wasm
```