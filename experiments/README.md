# Test WASI API — Language Comparison

Three server-side WASM components all implementing the same thing: `wasi:http/incoming-handler` returning a hello-world response, built with three different languages/toolchains and run with `wasmtime serve`.


|                     | [Rust](./rust_api_library/)                     | [TinyGo](./go_api_library/)                     | [C++](./cpp_api_library/)                              |
| ------------------- | ----------------------------------------------- | ----------------------------------------------- | ------------------------------------------------------ |
| **Binding tool**    | `wit_bindgen::generate!()` (compile-time macro) | `wasi-go-sdk` (wraps raw WIT bindings)          | `wit-bindgen c` (manual codegen step)                  |
| **Target**          | `wasm32-wasip2` (built in to rustc)             | `tinygo -target=wasip2`                         | WASI SDK `wasm32-wasip2-clang++`                       |
| **Entry point**     | `export!(HttpServer)` — no `main()`             | `init()` + empty `main()`                       | `extern "C" void exports_...handle(...)` — no `main()` |
| **API style**       | High-level Rust types + trait                   | Familiar `http.Handler` style                   | Raw C structs and handles                              |
| **Toolchain setup** | `rustup target add wasm32-wasip2`               | `brew install tinygo` + Go 1.19–1.25            | Download WASI SDK + `cargo install wit-bindgen-cli`    |
| **Build command**   | `cargo build --target wasm32-wasip2`            | `tinygo build -target=wasip2 --wit-package ...` | 3-step: codegen → compile C → compile C++ → link       |
| **Run command**     | `wasmtime serve -Scli *.wasm`                   | same                                            | same                                                   |
| **Dev friction**    | Low                                             | Medium                                          | High                                                   |


---

## Binding approach

All three use the **same WIT world** (`wasi:http/incoming-handler@0.2.6`) but each language expresses it differently:

- **Rust:** `wit-bindgen` runs as a proc-macro at compile time. You implement a generated `Guest` trait — no extra steps, no extra files.
- **TinyGo:** `wasi-go-sdk` pre-wraps the WIT bindings behind a standard Go `http.Handler`. You call `wasihttp.Handle(...)` in `init()` and build with `tinygo`.
- **C++:** You run `wit-bindgen c` yourself to generate `.h` and `.c` files, include them with `extern "C"`, implement the raw C function signature by hand, compile C and C++ separately to avoid name mangling, and link everything with the WASI SDK linker.

---

## Key challenges per language

**Rust**

- Body handling order: get body handle → set `ResponseOutparam` → write → `OutgoingBody::finish()`
- `-S cli` required because `wasm32-wasip2` stdlib imports `wasi:cli/environment`

**TinyGo**

- Standard Go `net/http` + `ListenAndServe` does NOT work in WASI (no sockets); must use `wasi-go-sdk`
- TinyGo only supports Go 1.19–1.25 (Go 1.26 breaks the build)
- `wasi-go-sdk` has no semver release; use pseudo-version in `go.mod`
- `wasm-tools` must be installed separately for wasip2 component embedding

**C++**

- `wit-bindgen c` emits C symbols; must wrap with `extern "C"` when including from C++, or name mangling breaks the linker
- System `clang++` uses macOS `ld` — cannot target WASM; must use the WASI SDK's `clang++`
- `hello.c` must be compiled with the C compiler, not C++, for the same reason
- No C++stdlib (`-nostdlib++`) and no exceptions (`-fno-exceptions`) in WASM/WASI

---

## Runtime comparison


| Tier | Runtime                         | Strengths                              | Fit                            |
| ---- | ------------------------------- | -------------------------------------- | ------------------------------ |
| S    | Wasmtime                        | Lowest tail latency, ~95% native speed | Components, production servers |
| A    | WasmEdge, Wasmer (LLVM)         | Fast JIT/cold start                    | Edge, multi-lang               |
| B    | Spin (Wasmtime-based), WAMR AOT | Very high req/s (45k+), low memory     | Serverless, embedded           |
| C    | Wazero (Go)                     | Simple embed                           | Go-native hosts                |


wasip1 = single ABI, linear memory. wasip2 = Component Model, WIT interfaces, modular. All three projects here use **wasip2**.