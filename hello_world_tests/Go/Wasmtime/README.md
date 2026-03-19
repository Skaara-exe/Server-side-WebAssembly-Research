# Instructions

## Go

Run Go code
```bash
go run hello.go
```

Or run all files in the current directory:
```bash
go run .
```

Source:
https://go.dev/doc/tutorial/getting-started

---

## Wasmtime

**Note:** The standard Go compiler targets the browser JS runtime for wasm. To compile Go for WASI (which wasmtime runs), you need to set the `GOOS` and `GOARCH` environment variables before building.

### Compile Go code into wasm

Bash:
```bash
GOOS=wasip1 GOARCH=wasm go build -o hello.wasm hello.go
```
PowerShell:
```powershell
$env:GOOS="wasip1"; $env:GOARCH="wasm"; go build -o hello.wasm hello.go
```

### Run the compiled wasm in the wasmtime

```
wasmtime hello.wasm
```
Component
---
Sadly this part was more diccifult and we could not get it working. for more information check the [GO component readme](./hello-world-wasmtime-go-component/README.md)
---

Source:  
https://wasmtime.dev/  
https://go.dev/doc/tutorial/getting-started