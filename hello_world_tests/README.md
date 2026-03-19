# hello_world_tests

A collection of **Hello World** examples across multiple languages and runtimes, used to validate that standard project structures, toolchains, and WebAssembly integrations work correctly.

Each subdirectory contains its own `README.md` with setup and run instructions for that specific example.

---

## Repository Structure

```
hello_world_tests
├───Java
│   └───GraalWasm
│       └───adder-graalwasm-module-java✅
├───Cpp
│   ├───native
│   │   └───hello-world-cpp✅
│   └───Wasmtime
│       ├───hello-world-wasmtime-cpp✅
│       ├───hello-world-wasmtime-cpp-module✅
│       └───hello-world-wasmtime-cpp-component
│           ├───guest✅
│           └───rust-host⚠️(host is in rust simply to test guest)
├───Go
│   ├───native
│   │   └───hello-world-go✅
│   └───Wasmtime
│       ├───hello-world-wasmtime-go✅
│       └───hello-world-wasmtime-go-component❌
└───Rust
    ├───native
    │   └───hello-world-rust✅
    ├───WasmEdge
    │   ├───hello-world-wasmedge-rust✅
    │   └───hello-world-wasmedge-rust-docker⚠️
    └───Wasmtime
        ├───hello-world-wasmtime-rust✅
        ├───hello-world-wasmtime-rust-component✅
        └───hello-world-wasmtime-rust-module✅
```

### Status Legend

| Symbol | Meaning |
|--------|---------|
| ✅ | Working |
| ⚠️ | Partial / known issues |
| ❌ | Not working |

---

## Languages & Runtimes

These are just examples of area's we felt usefull for our research, this means that other runtimes, languages and options are possible.

| Language | Native | Wasmtime | WasmEdge | GraalWasm |
|----------|:------:|:--------:|:--------:|:---------:|
| Rust     | ✅     | ✅       | ✅       |           |
| C++      | ✅     | ✅       |          |           |
| Go       | ✅     | ✅       |          |           |
| Java     |        |          |          | ✅        |

Examples are organized by **module**, **component**, and **native** variants where applicable, covering the full spectrum from raw Wasm modules to the Component Model. Since we didnt find an option to compile java into wasm. Java only has an example of hosting a wasm module.

---

## Learning Resources

| Resource | Level | What it covers |
|----------|-------|----------------|
| [Wasmtime Docs](https://docs.wasmtime.dev) | Practical | How to use Wasmtime — modules, WASI, embedding |
| [Why the Component Model](https://component-model.bytecodealliance.org/design/why-component-model.html) | Conceptual | Why modules are limited, how memory passing works, why components exist |
| [WebAssembly Spec](https://webassembly.github.io/spec/core/intro/overview.html) | Theoretical | The official Wasm spec — types, linear memory, modules, instantiation |
| [Component Model Explainer](https://github.com/WebAssembly/component-model/blob/main/design/mvp/Explainer.md) | Deep | Component model internals, canonical ABI, how types cross boundaries |
| [Wasm Reference Manual](https://github.com/sunfishcode/wasm-reference-manual/blob/master/WebAssembly.md) | Reference | Detailed module structure — sections, imports, exports, memory layout |