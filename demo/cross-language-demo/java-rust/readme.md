# Cross Language Demo

This demo shows how code written in **Rust** can be called from **Java** — without any networking or glue code. The trick is WebAssembly (WASM): Rust compiles down to a `.wasm` file, and Java loads and runs it directly using GraalVM.

---

## What's in the box

| Folder | Language | Role |
|--------|----------|------|
| `rust-guest` | Rust | Calculates a Fibonacci sequence, compiled to WASM |
| `java-host` | Java | Loads the `.wasm` file and calls the Fibonacci function |

---

## What you'll need

- [Rust](https://rustup.rs/)
- [Java 21+](https://adoptium.net/) with [GraalVM](https://www.graalvm.org/) (or a standard JDK — GraalWasm is pulled in automatically via Gradle)

---

## How to run it

### 1. Build the Rust module

Navigate into the `rust-guest` folder and run:

```bash
rustup target add wasm32-wasip1
cargo build --release --target wasm32-wasip1
```

Then copy the output into the Java project's resources:

```bash
copy .\target\wasm32-wasip1\release\fibonacci.wasm ..\java-host\src\main\resources\fibonacci.wasm
```

### 2. Run the Java host

Navigate into the `java-host` folder and run:

```bash
./gradlew run
```

You should see:

```
Fibonacci(10): [0, 1, 1, 2, 3, 5, 8, 13, 21, 34]
```

---

## How it works

1. The Rust code calculates a Fibonacci sequence and exposes it as a function
2. Rust compiles that function into a `.wasm` binary
3. Java loads the `.wasm` file at runtime using GraalVM's WebAssembly support
4. Java calls the Fibonacci function as if it were any other function, and prints the result

No servers, no APIs — just two languages talking through a shared binary format.