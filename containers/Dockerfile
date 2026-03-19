FROM scratch
#FROM alpine:3.18
#FROM debian:bookworm-slim

# Copy Wasmtime and your WASM binary
COPY wasmtime-musl /wasmtime-musl
COPY source-code/hello-world-rust.wasm /main.wasm

ENTRYPOINT ["/wasmtime-musl", "/main.wasm"]