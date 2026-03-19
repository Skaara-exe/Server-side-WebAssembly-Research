# Instructions

## C++

Download and install the WASI SDK (bundles a clang++ pre-configured for WASI targets)

```
https://github.com/WebAssembly/wasi-sdk/releases
```

Extract and move to a permanent location, then set the environment variable:

**Windows** — recommended path: `C:\wasi-sdk`

Search "Edit environment variables" in the Start menu → User variables → New:

| Field | Value |
|---|---|
| Variable name | `WASI_SDK_PATH` |
| Variable value | `C:\wasi-sdk` |

**Linux/macOS** — Add to your shell profile (`~/.bashrc`, `~/.zshrc`, etc.):

```bash
export WASI_SDK_PATH=/opt/wasi-sdk
```

We use `printf` instead of `cout` for WASI compatibility. `cout` pulls in C++ exception
handling which WASI does not support. `printf` is plain C — no
exceptions, no rtti, and works in both C and C++.

```cpp
#include <cstdio>

int main()
{
    const char* msg[] = {"Hello", "World", "!"};

    for (const char* word : msg)
    {
        printf("%s ", word);
    }
    printf("\n");
}
```

Compile C++ code into wasm targeting wasip2

```bash
# Linux/macOS
$WASI_SDK_PATH/bin/clang++ --target=wasm32-wasip2 hello.cpp -o hello.wasm
```

```powershell
# Windows (PowerShell)
& "$env:WASI_SDK_PATH\bin\clang++" --target=wasm32-wasip2 hello.cpp -o hello.wasm
```

Run the compiled wasm in the wasmtime runtime

```
wasmtime hello.wasm
```



## Module Guest

this is a module example of using a shered memory block to transfer data between host and guest since modules only accept i64, i32, f64 or f32

compile the guest

```powershell
& "$env:WASI_SDK_PATH\bin\clang++" `
  --target=wasm32-wasip1 `
  -fno-exceptions `
  guest.cpp -o guest.wasm
```

## Module Host

this is a module example of using a shered memory block to transfer data between host and guest since modules only accept i64, i32, f64 or f32

Download the Wasmtime C-API release for your platform from(it needs the lib\libwasmtime.dll.a):
```
https://github.com/bytecodealliance/wasmtime/releases
```
Pick the file ending in `-c-api`, extract it, and place it next to `host.cpp` as `wasmtime-c-api/`.
```
your-project/
├── wasmtime-c-api/
│   ├── include/
│   └── lib/
├── guest/
│   ├── guest.cpp
│   └── guest.wasm
└── host/
    └── host.cpp
```


```powershell
& "g++" `
  -std=c++17 `
  -I../wasmtime-c-api/include `
  host.cpp `
  ../wasmtime-c-api/lib/libwasmtime.dll.a `
  -o host.exe `
  -lws2_32 -lbcrypt -lole32 -lntdll -luserenv
```
copy ```wasmtime.ddl``` into the host folder
```powershell
cp ../wasmtime-c-api/lib/wasmtime.dll .
```
Run it:
```powershell
./host.exe ..\guest\guest.wasm
```

## Component Guest

### Project structure

```
hello-world-wasmtime-cpp-component/
├── guest/
│   ├── wit/
│   │   └── world.wit
│   └── src/
│       └── hello.cpp
```

### WIT definition
using the same wit file as is used in rust since nothing changes

```wit
package example:hello;

world hello-world {
    export add: func(a: s32, b: s32) -> s32;
    export greet: func();
}
```

### Install wit-bindgen

`wit-bindgen` generates C bindings from your WIT file

```
cargo install wit-bindgen-cli
```

### Generate C bindings

Run from inside the `guest/` folder

```
cd guest
wit-bindgen c ./wit
```

This generates three files:

| File | Purpose |
|---|---|
| `hello_world.h` | Function signatures you need to implement |
| `hello_world.c` | ABI glue code — do not edit |
| `hello_world_component_type.o` | Component type metadata for linking |

`wit-bindgen` only generates C bindings, not C++ directly. You can still implement
in C++ because the header already has `extern "C"` guards — this tells the C++
compiler not to mangle the function names that the glue code expects.

### Implement the guest (`src/hello.cpp`)

```cpp
#include "../hello_world.h"
#include 

int32_t exports_hello_world_add(int32_t a, int32_t b) {
    return a + b;
}

void exports_hello_world_greet(void) {
    printf("Hello, world!\n");
}
```

### Compile the glue code as C

```bash
# Linux/macOS
$WASI_SDK_PATH/bin/clang --target=wasm32-wasip2 -c hello_world.c -o hello_world.o
```

```powershell
# Windows (PowerShell)
& "$env:WASI_SDK_PATH\bin\clang" --target=wasm32-wasip2 -c hello_world.c -o hello_world.o
```

### Compile wasm and link everything

```bash
# Linux/macOS
$WASI_SDK_PATH/bin/clang++ \
  --target=wasm32-wasip2 \
  hello_world.o hello_world_component_type.o src/hello.cpp \
  -o hello-core.wasm \
  -mexec-model=reactor \
  "-Wl,--export=__component_type_object_force_link_hello_world"
```

```powershell
# Windows (PowerShell)
& "$env:WASI_SDK_PATH\bin\clang++" `
  --target=wasm32-wasip2 `
  hello_world.o hello_world_component_type.o src/hello.cpp `
  -o hello-core.wasm `
  -mexec-model=reactor `
  "-Wl,--export=__component_type_object_force_link_hello_world"
```

> Targeting `wasm32-wasip2` causes the WASI SDK linker to automatically wrap the
> output into a component — no separate `wasm-tools component new` step needed.

## Component host

It is a simple reuse of the rust host since cpp doesn not have support yet to host wasm components. see [rust readme](../../Rust/Wasmtime/README.md)

---

sources:
- https://github.com/WebAssembly/wasi-sdk
- https://github.com/bytecodealliance/wit-bindgen
- https://github.com/bytecodealliance/wasm-tools
- https://docs.wasmtime.dev/c-api/
- https://github.com/bytecodealliance/wasmtime/releases
- https://docs.wasmtime.dev/lang-c.html