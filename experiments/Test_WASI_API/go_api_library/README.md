# WASI HTTP TinyGo Component

TinyGo WebAssembly component implementing `wasi:http/incoming-handler` via [wasi-go-sdk](https://github.com/rajatjindal/wasi-go-sdk). Returns "Hello from TinyGo + WASI!" on `GET /`.

---

## Prerequisites

- **TinyGo** 0.37+  
- **Go** 1.19–1.25 (TinyGo does not support Go 1.26 yet)  
- **wasmtime**  
- **wasm-tools** (used by TinyGo for wasip2 component embedding)

```bash
brew install tinygo wasmtime wasm-tools
```

---

## Build & Run

```bash
go mod tidy

tinygo build -target=wasip2 \
  --wit-package $(go list -mod=readonly -m -f '{{.Dir}}' github.com/rajatjindal/wasi-go-sdk)/wit \
  --wit-world sdk \
  -o main.wasm .

wasmtime serve -Scli main.wasm
# curl http://localhost:8080 → "Hello from TinyGo + WASI!"
```

`-Scli` is required because the SDK imports `wasi:cli/environment`.

---

## Key findings (from research)

- **TinyGo vs standard Go for WASM:** TinyGo produces much smaller binaries (no full Go runtime); better for edge/serverless. Use `-target=wasip2` for Component Model HTTP.
- **`net/http` + `ListenAndServe` does not work in pure WASI.** WASI has no sockets; the runtime provides HTTP via `wasi:http` imports. Both Rust and Go must use wasi-http bindings (e.g. wit-bindgen in Rust, wasi-go-sdk in Go), not native networking.
- **wasip1 vs wasip2:** wasip1 = single ABI, linear memory; wasip2 = Component Model, WIT interfaces (`wasi:http`), modular. This project uses wasip2 for native HTTP handler support.
- **wit-bindgen** is for the Component Model (works with wasip1 and wasip2); it generates host/guest bindings from WIT files. wasm-tools is used to validate, convert, and wrap core modules into components (TinyGo’s wasip2 build uses it under the hood).
- **go.mod:** wasi-go-sdk has no semver tag; use pseudo-version e.g. `v0.0.0-20241019020410-17b0b9ed651f` (not `v0.0.1`).

---

## vs Rust (`rust_api_library`)

| | Rust | TinyGo |
|---|---|---|
| Bindings | wit-bindgen + `Guest` | wasi-go-sdk + `wasihttp.Handle` |
| Entry | `export!(HttpServer)` | `init()` + empty `main()` |
| Run | `wasmtime serve -Scli` | same |
