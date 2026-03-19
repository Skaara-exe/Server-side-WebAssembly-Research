# Instructions

## Rust

Add the WebAssembly target to your Rust toolchain

```
rustup target add wasm32-wasip1
```

Compile rust code into wasm
```
cargo build --target wasm32-wasip1 --release
```

Run the compiled wasm in the wasmEdge runtime
```
wasmedge target/wasm32-wasip1/release/hello-world-wasmedge-rust.wasm
```

## docker

**Note:** Requires Docker Desktop with [Wasm support](https://wasmedge.org/docs/start/build-and-run/docker_wasm/) enabled.

Build the Docker image.
```
docker buildx build --provenance=false --platform wasi/wasm --no-cache -t hello-world-wasmedge-rust .
```


```docker buildx build``` - buildx is needed for custom platforms like wasm/wasi


```--provenance=false``` - Disables build attestation metadata. Wasm images don't support this yet ```¯\_(ツ)_/¯```


```--platform wasi/wasm``` - Normal containers use a linux. container
wasi/wasm tells Docker this is a WebAssembly module, not a Linux container
(the reason why this container is cross platform with diffrent CPU architectures)

than the rest of the command is pretty normal docker use

<br>

Run the container with WasmEdge runtime
```
docker run --runtime=io.containerd.wasmedge.v1 --platform=wasi/wasm hello-world-wasmedge-rust:latest
```

---

source:
https://github.com/second-state/rust-examples/tree/main/hello

https://wasmedge.org/docs/start/install

https://docs.docker.com/desktop/features/wasm/