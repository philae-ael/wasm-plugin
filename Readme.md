# Get: 
- [wasm-tools](https://github.com/bytecodealliance/wasm-tools)
- [wasi_snapshot_preview1.reactor.wasm](https://github.com/bytecodealliance/wasmtime/releases/tag/dev)


# Build:

```sh 
cargo build -p wasm-plugin --target wasm32-wasi

wasm-tools component new ./target/wasm32-wasi/debug/wasm_plugin.wasm \
          -o dist/wasm-plugin.wasm --adapt ./wasi_snapshot_preview1.reactor.wasm
```

# Run:

```sh 
cargo run -p wasm-runner
```


# References 
- https://github.com/bytecodealliance/wasmtime
- https://crates.io/crates/wasmtime
- https://docs.wasmtime.dev/
