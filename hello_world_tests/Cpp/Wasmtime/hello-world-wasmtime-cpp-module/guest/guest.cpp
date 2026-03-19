// guest.cpp
// Compile with:
//   clang++ --target=wasm32-wasi --sysroot=$WASI_SDK/share/wasi-sysroot \
//           -o guest.wasm guest.cpp
//
// Or with emscripten (not recommended for WASI), use wasi-sdk instead.

#include <cstdio>
#include <cstring>

// Mark functions for export so the host can call them.
// __attribute__((export_name("..."))) is a Clang/wasi-sdk extension.

// add: receives two i32s, prints both and their sum inside the guest, returns the sum.
extern "C" __attribute__((export_name("add")))
int add(int a, int b) {
    int result = a + b;
    // This printf goes through WASI stdout (inherited by the host)
    printf("[guest] add called: %d + %d = %d\n", a, b, result);
    return result;
}

// greet: receives a pointer+length to a UTF-8 string in wasm linear memory,
//        prints it via WASI stdout.
// We use ptr+len instead of a null-terminated pointer so the host can pass
// arbitrary bytes without needing to write a NUL terminator.
extern "C" __attribute__((export_name("greet")))
void greet(const char* ptr, int len) {
    printf("[guest] greet called: %.*s\n", len, ptr);
}

// Expose the wasm linear-memory allocator so the host can place the string.
// A minimal bump allocator is enough for this demo.
static char heap[65536];
static int  heap_top = 0;

extern "C" __attribute__((export_name("alloc")))
char* alloc(int size) {
    if (heap_top + size > (int)sizeof(heap)) return nullptr;
    char* ptr = heap + heap_top;
    heap_top += size;
    return ptr;
}

extern "C" __attribute__((export_name("dealloc")))
void dealloc(char* /*ptr*/, int /*size*/) {
    // No-op for this demo; a real impl would track free blocks.
}