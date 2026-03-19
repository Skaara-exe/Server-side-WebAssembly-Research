// host.cpp
// Compile with (after downloading wasmtime C-API release):
//
//   Linux:
//     g++ -std=c++17 -I./wasmtime-c-api/include \
//         -o host host.cpp \
//         ./wasmtime-c-api/lib/libwasmtime.a \
//         -lpthread -ldl -lm
//
//   macOS:
//     g++ -std=c++17 -I./wasmtime-c-api/include \
//         -o host host.cpp \
//         ./wasmtime-c-api/lib/libwasmtime.a
//
// Then run:
//   ./host guest.wasm

#include <fstream>
#include <iostream>
#include <sstream>
#include <string>
#include <vector>
#include <variant>
#include <wasmtime.hh>
#include <cstring>

using namespace wasmtime;

// ── helpers ────────────────────────────────────────────────────────────────

template <typename T, typename E>
T must(Result<T, E> r) {
    if (r) return r.ok();
    std::cerr << "wasmtime error: " << r.err().message() << "\n";
    std::abort();
}

static std::vector<uint8_t> read_file(const std::string& path) {
    std::ifstream f(path, std::ios::binary);
    if (!f) { std::cerr << "Cannot open: " << path << "\n"; std::abort(); }
    return std::vector<uint8_t>(
        std::istreambuf_iterator<char>(f),
        std::istreambuf_iterator<char>()
    );
}

// ── main ───────────────────────────────────────────────────────────────────

int main(int argc, char** argv) {
    if (argc < 2) {
        std::cerr << "Usage: " << argv[0] << " <guest.wasm>\n";
        return 1;
    }

    // 1. Engine + Store
    Engine engine;
    Store  store(engine);

    // 2. Configure WASI (guest printf → host stdout)
    WasiConfig wasi;
    wasi.inherit_stdout();
    wasi.inherit_stderr();
    must(store.context().set_wasi(std::move(wasi)));

    // 3. Compile the guest module
    auto wasm_bytes = read_file(argv[1]);
    Module mod = must(Module::compile(engine, wasm_bytes));

    // 4. Linker with WASI pre-populated
    Linker linker(engine);
    must(linker.define_wasi());

    // 5. Instantiate
    Instance instance = must(linker.instantiate(store, mod));

    // ── Grab exports ───────────────────────────────────────────────────────

    // memory – needed so we can write the greeting string into wasm RAM
    auto mem_exp = instance.get(store, "memory");
    if (!mem_exp) { std::cerr << "No 'memory' export\n"; return 1; }
    Memory memory = std::get<Memory>(*mem_exp);

    // alloc(size) → i32  (pointer inside wasm memory)
    auto alloc_exp = instance.get(store, "alloc");
    if (!alloc_exp) { std::cerr << "No 'alloc' export\n"; return 1; }
    Func alloc_fn = std::get<Func>(*alloc_exp);

    // dealloc(ptr, size)
    auto dealloc_exp = instance.get(store, "dealloc");
    if (!dealloc_exp) { std::cerr << "No 'dealloc' export\n"; return 1; }
    Func dealloc_fn = std::get<Func>(*dealloc_exp);

    // add(i32, i32) → i32
    auto add_exp = instance.get(store, "add");
    if (!add_exp) { std::cerr << "No 'add' export\n"; return 1; }
    Func add_fn = std::get<Func>(*add_exp);

    // greet(ptr: i32, len: i32)
    auto greet_exp = instance.get(store, "greet");
    if (!greet_exp) { std::cerr << "No 'greet' export\n"; return 1; }
    Func greet_fn = std::get<Func>(*greet_exp);

    // ── Call add ───────────────────────────────────────────────────────────
    {
        int a = 21, b = 21;
        std::cout << "[host] Calling add(" << a << ", " << b << ")\n";

        auto results = must(add_fn.call(store, {Val(a), Val(b)}));
        int  sum     = results[0].i32();

        std::cout << "[host] add returned: " << sum << "\n\n";
    }

    // ── Call greet ─────────────────────────────────────────────────────────
    {
        std::string greeting = "Hello from the host!";
        int len = static_cast<int>(greeting.size());

        std::cout << "[host] Calling greet(\"" << greeting << "\")\n";

        // Allocate space inside wasm linear memory
        auto alloc_result = must(alloc_fn.call(store, {Val(len)}));
        int32_t wasm_ptr  = alloc_result[0].i32();

        // Copy the string bytes into wasm memory
        uint8_t* wasm_mem_base = memory.data(store).data();
        std::memcpy(wasm_mem_base + wasm_ptr, greeting.data(), len);

        // Call greet with (ptr, len)
        must(greet_fn.call(store, {Val(wasm_ptr), Val(len)}));

        // Free the allocation
        must(dealloc_fn.call(store, {Val(wasm_ptr), Val(len)}));

        std::cout << "[host] greet call complete\n";
    }

    return 0;
}