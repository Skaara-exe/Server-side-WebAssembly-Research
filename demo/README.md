# demo

this is a repository of the 3 demos we showed during our presentation. check out each directory to learn more about how the demos work.


## cross language demo

A simple demo showing how Rust and Java can talk through WebAssembly — a Fibonacci function is compiled to a .wasm module and called directly from Java using GraalVM.

## http ai faas demo

experiments/Test_AI_WASI/ is an edge-style AI inference demo: a Spin/WASI HTTP component written in Rust/WASM that accepts an uploaded image (POST) and returns a 512-d CLIP-style embedding (ONNX loaded once and cached for faster warm requests).

## wasmcloud presistence demo

The wasmcloud-persistence repository is a simple wasm project that showcases the possibility of persistence in wasm apps. The project is built with wasmcloud, which makes using libraries like kvnats very easy as it also starts up the necessary services surrounding the app. Configuration is similar to docker compose or kubernetes, with a simple .yaml file.