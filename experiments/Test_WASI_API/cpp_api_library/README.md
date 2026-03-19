# WASI HTTP C++ Component

C++WebAssembly component implementing `wasi:http/incoming-handler` via [wit-bindgen](https://github.com/bytecodealliance/wit-bindgen) (C backend) + [WASI SDK](https://github.com/WebAssembly/wasi-sdk). Returns "Hello from C++ + WASI!" on `GET /`.

---

## Prerequisites

- **WASI SDK** 30+ (provides `wasm32-wasip2-clang++`)
- **wit-bindgen** CLI (`cargo install wit-bindgen-cli`)
- **wasmtime**

```bash
brew install wasmtime
cargo install wit-bindgen-cli
```

Download the WASI SDK (arm64 macOS):

```bash
curl -LO https://github.com/WebAssembly/wasi-sdk/releases/download/wasi-sdk-30/wasi-sdk-30.0-arm64-macos.tar.gz
tar -xzf wasi-sdk-30.0-arm64-macos.tar.gz
```

---

## Generate Bindings

Uses the same WIT world as the Rust project:

```bash
wit-bindgen c ../rust_api_library/wit -w hello --out-dir ./native
```

Produces `native/hello.h`, `native/hello.c`, and `native/hello_component_type.o`.

---

## Build & Run

```bash
SDK=./wasi-sdk-30.0-arm64-macos/bin

# Compile hello.c as C (not C++ — avoids name mangling)
$SDK/wasm32-wasip2-clang -c native/hello.c -o native/hello.o -I .

# Compile main.cpp as C++
$SDK/wasm32-wasip2-clang++ -c main.cpp -o main.o -I . -fno-exceptions

# Link
$SDK/wasm32-wasip2-clang++ \
  -o main.wasm main.o native/hello.o native/hello_component_type.o \
  -Wl,--no-entry -Wl,--export-dynamic \
  -fno-exceptions

wasmtime serve -Scli main.wasm
# curl http://localhost:8080 → "Hello from C++ + WASI!"
```

---

## Key findings (from implementation)

- **wit-bindgen generates C, not C++.** The `wit-bindgen c` backend produces `.h` and `.c` files with C-linkage symbols. When including from C++, wrap with `extern "C" { #include "hello.h" }` and mark your exported handler as `extern "C"` — otherwise C++ name mangling breaks the linker.
- **WASI SDK is a separate toolchain.** Your system `clang++` targets macOS (arm64-apple-darwin) and uses macOS `ld`, which doesn't understand `--no-entry` or `--export-dynamic`. The WASI SDK provides `wasm32-wasip2-clang++` which targets WASM and uses `wasm-ld`.
- **Compile C and C++ separately.** `hello.c` and `hello_component_type.o` contain C symbols. If you compile `hello.c` as C++(by passing it to `clang++`), the symbols get mangled and linking fails. Compile` .c`files with the C compiler,`.cpp` with C++, then link.
- **No `main()`.** Like Rust's `cdylib`, the component has no entry point (`-Wl,--no-entry`). The runtime calls `exports_wasi_http_incoming_handler_handle` directly.
- **No C++ stdlib or exceptions.** `-nostdlib++ -fno-exceptions` — the C++ standard library is too heavy for WASM/WASI. You work with raw C types from the generated bindings.
- **Most friction of the three languages.** Rust has `wit_bindgen::generate!()` at compile time; TinyGo has `wasi-go-sdk` wrapping everything; C++ requires manual codegen, `extern "C"` wrangling, separate compilation, and verbose low-level API calls.

---

## vs Rust and TinyGo


|            | Rust                       | TinyGo                           | C++                                      |
| ---------- | -------------------------- | -------------------------------- | ---------------------------------------- |
| Bindings   | `wit_bindgen::generate!()` | `wasi-go-sdk`                    | `wit-bindgen c` (manual codegen)         |
| Entry      | `export!(HttpServer)`      | `init()` + empty `main()`        | `extern "C" void exports_...handle(...)` |
| Compiler   | `rustc` (built-in target)  | `tinygo`                         | WASI SDK `clang++`                       |
| Ergonomics | High (traits, types)       | Medium (familiar `http.Handler`) | Low (raw C types, manual memory)         |
| Run        | `wasmtime serve -Scli`     | same                             | same                                     |


