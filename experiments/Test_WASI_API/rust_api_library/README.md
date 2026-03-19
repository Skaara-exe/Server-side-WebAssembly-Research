# WASI HTTP Rust Component

Rust WebAssembly component implementing `wasi:http/incoming-handler` via [wit-bindgen](https://docs.rs/wit-bindgen/). Returns "Hello from Rust!" on `GET /`.

---

## Prerequisites

- **Rust** (rustup)
- **wasm32-wasip2** target
- **wasmtime**

```bash
rustup target add wasm32-wasip2
brew install wasmtime
```

---

## Build & Run

```bash
cargo build --target wasm32-wasip2 --release

wasmtime serve -Scli target/wasm32-wasip2/release/api_library.wasm
# curl http://localhost:8080 → "Hello from Rust!"
```

`-Scli` is required because the crate imports `wasi:cli/environment`.

---

## Key findings (from implementation)

- **wit-bindgen** generates Rust bindings from WIT files (`wit/world.wit`): types like `IncomingRequest`, `ResponseOutparam`, and the `Guest` trait you implement. No separate codegen step—it runs at compile time.
- **Body handling order:** Get the response body handle, then set `ResponseOutparam` (sends headers), then write to the body stream, then call `OutgoingBody::finish()`. Writing before setting the outparam can break the response.
- **Library (cdylib) not binary:** This is a component that exports the handler; the runtime (wasmtime) loads the `.wasm` and calls in. No `main()` entry point.

---

## vs TinyGo (`go_api_library`)

| | Rust | TinyGo |
|---|---|---|
| Bindings | wit-bindgen + `Guest` | wasi-go-sdk + `wasihttp.Handle` |
| Entry | `export!(HttpServer)` | `init()` + empty `main()` |
| Run | `wasmtime serve -Scli` | same |
