# GraalWasm Java Host Demo

A minimal example of embedding a WebAssembly module inside a Java application using the GraalWasm library.

## Module

The Wasm module was written in WAT (WebAssembly Text Format) and compiled using the [wat2wasm online tool](https://webassembly.github.io/wabt/demo/wat2wasm/).

WAT is a human-readable text representation of WebAssembly. The online tool converts it to a binary `.wasm` file that can be loaded at runtime.

## Host

The host application is written in Java using the [GraalWasm](https://www.graalvm.org/webassembly/) library — a high-performance WebAssembly runtime that runs on the JVM. GraalWasm is distributed via Maven Central and requires no native dependencies.

Add the following to your `build.gradle.kts`:

```kotlin
implementation("org.graalvm.polyglot:polyglot:25.0.2")
implementation("org.graalvm.polyglot:wasm:25.0.2")
```

## Sources

https://www.graalvm.org/webassembly/
https://webassembly.github.io/wabt/demo/wat2wasm/
https://github.com/graalvm/graal-languages-demos/