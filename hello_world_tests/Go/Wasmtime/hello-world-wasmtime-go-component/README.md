# hello-world-wasmtime-go-component

> **Research finding:** This example documents the current state of Go + WebAssembly Component Model support as of February 2026. It is intentionally incomplete — the goal is to show what works, what doesn't, and why.

---

## Comparison: Rust vs Go

| | Rust | Go |
|---|---|---|
| Guest → wasm component | ✅ `cargo build --target wasm32-wasip2` | ⚠️ Possible in theory, broken in practice |
| WIT bindgen for guest | ✅ `wit_bindgen::generate!()` macro | ⚠️ Multiple competing tools, one deprecated |
| Host runs component | ✅ Full support via `wasmtime` crate | ❌ Hard blocker — `wasmtime-go` is core modules only |
| Host bindgen from WIT | ✅ `wasmtime::component::bindgen!()` macro | ❌ No equivalent exists |

**Conclusion:** In Go the guest side is theoretically possible but the tooling is fragmented, partially deprecated, and underdocumented. The host side is a hard blocker with no ETA.

---

## What Was Attempted (Go Guest)

### Attempt 1 — TinyGo + wit-bindgen-go (go.bytecodealliance.org)

The first approach used TinyGo targeting `wasip2` with `go.bytecodealliance.org/cmd/wit-bindgen-go` to generate bindings.

**Problems encountered:**
- `go.bytecodealliance.org/cmd/wit-bindgen-go` is deprecated — the Bytecode Alliance recommends migrating away from it
- TinyGo 0.40.1 only supports up to Go 1.25.5, requiring a separate Go installation alongside 1.26
- The `--wit-package` flag expects an encoded `.wasm` file, not a package name string like `example:hello` — not documented clearly anywhere
- `world.wit` requires `include wasi:cli/imports@0.2.0` for TinyGo's wasip2 target, but resolving that requires `wkg` (a WIT package manager) to fetch WASI WIT definitions
- `wkg wit fetch` produced authentication errors against the OCI registry it fetches from

### Attempt 2 — Standard Go compiler + wit-bindgen CLI

The second approach dropped TinyGo entirely and used the official `wit-bindgen` Rust CLI (`cargo install wit-bindgen-cli`) to generate Go bindings, then compiled with the standard Go compiler targeting `wasip1`, then used `wasm-tools` to wrap the output into a component.

**Status:** This is the most promising path and the approach left in the code files. It requires:
1. `wit-bindgen go` to generate bindings
2. Standard `go build` with `GOOS=wasip1 GOARCH=wasm`
3. `wasm-tools component embed` to embed WIT metadata
4. `wasm-tools component new` with `wasi_snapshot_preview1.reactor.wasm` adapter to produce the final component

This path was not fully verified due to the cumulative complexity of the toolchain.

---

## Why the Host Side Is a Hard Blocker

`wasmtime-go` wraps the Wasmtime C API via CGO. The C API does not expose component model functionality. This means:

- No `Component::from_file()` equivalent
- No WIT bindgen for the host
- No type-safe calling of component exports

The Go SDK can only load and run core wasm **modules** (wasip1), not components. This is tracked at:
https://github.com/bytecodealliance/wasmtime-go/issues/204

---

## Practical Recommendation

1. **Rust guest + Rust host** — fully works, already demonstrated in `hello-world-wasmtime-rust-component`
2. **Go guest + Rust host** — the Rust host is language-agnostic and can load any valid `.wasm` component, so once the Go guest tooling matures this becomes viable
3. **Wait** — the Go component model story is actively being developed and will likely be solid within 6–12 months

\*rust recomendatin since rust seems to be the main contender for wasm/wasi

---

## Code Files

The `guest/` folder contains the Go source files for the guest approach that is closest to working (Attempt 2). They are provided for reference — do not expect them to compile without further debugging of the toolchain.

```
guest/
├── go.mod
├── main.go
└── wit/
    └── world.wit
```

---

## Sources

- https://pkg.go.dev/github.com/bytecodealliance/wasmtime-go
- https://github.com/bytecodealliance/wasmtime-go/issues/204
- https://github.com/bytecodealliance/wit-bindgen/blob/main/crates/go/README.md
- https://github.com/bytecodealliance/go-modules
- https://tinygo.org/getting-started/install/windows/
- https://wasmtime.dev/
- https://component-model.bytecodealliance.org/language-support/building-a-simple-component/go.html