# Instructions

## Rust

Add the WebAssembly target to your Rust toolchain

```
rustup target add wasm32-wasip1
```

Compile rust code into wasm
```
rustc src/main.rs --target wasm32-wasip1
```

Run the compiled wasm in the wasmtime runtime
```
wasmtime main.wasm
```
## Modules
### Rust as a guest module compiled in wasm

Compile rust module code into wasm
```
cd .\module\
cargo build --target wasm32-wasip1 --release
```

The compiled `.wasm` file will be at:
```
module/target/wasm32-wasip1/release/hello_world_wasmtime_rust_module.wasm
```

### Rust Host Application

**Note:** Requires wasmtime and wasmtime-wasi dependencies in Cargo.toml

Add dependencies to your host project's Cargo.toml:
```toml
[dependencies]
wasmtime = "41.0.3"
wasmtime-wasi = "41.0.3"
```

<br>

Build the host application
```
cd ..
cargo build --release
```

Run the host application
```
cargo run --release
```

---

## components

components use WASI Preview 2 so we need to use ```wasm32-wasip2 ``` so we need to install the correct one for the compiler

```
rustup target add wasm32-wasip2
```
### rust as a component compiled in wasm using wit

first of all unlike a module, a component needs abit more. namely a .wit file.
in the guess folder there is a  ```wit\world.wit``` file

```
package example:hello;

world hello-world {
    export add: func(a: s32, b: s32) -> s32;
    export greet: func();
}
```

To build the component

```
cd .\guest\
cargo build --target wasm32-wasip2 --release
```
### Rust Host Application

Build and run the host as native rust

```
cd ..\host\
cargo build --release
cargo run --release
```
**Note:** cargo run will automatically build if you forget that command

```
hello-world-wasmtime-rust-component/
├── guest/
│   ├── Cargo.toml
│   ├── wit/
│   │   └── world.wit
│   └── src/
│       └── lib.rs
└── host/
    ├── Cargo.toml
    └── src/
        └── main.rs
```

---

source:
https://wasmtime.dev/  
https://github.com/bytecodealliance/wasmtime/blob/main/examples/hello.rs  
https://docs.rs/wasmtime/41.0.3/wasmtime/
