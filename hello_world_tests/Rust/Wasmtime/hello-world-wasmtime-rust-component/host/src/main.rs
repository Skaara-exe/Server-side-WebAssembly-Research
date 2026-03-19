use wasmtime::component::{Component, Linker, ResourceTable};
use wasmtime::{Config, Engine, Result, Store};
use wasmtime_wasi::{WasiCtx, WasiCtxBuilder, WasiCtxView, WasiView};

// Generate bindings from the WIT file
wasmtime::component::bindgen!({
    path: "../guest/wit",
    world: "hello-world",
});

// Host state that implements WASI
struct MyState {
    wasi: WasiCtx,
    table: ResourceTable,
}

// Implement WasiView for v41
impl WasiView for MyState {
    fn ctx(&mut self) -> WasiCtxView<'_> {
        WasiCtxView {
            ctx: &mut self.wasi,
            table: &mut self.table,
        }
    }
}

fn main() -> Result<()> {
    println!("=== Wasmtime Component Model Example ===\n");
    
    println!("1. Initializing engine...");
    let mut config = Config::new();
    config.wasm_component_model(true);
    let engine = Engine::new(&config)?;
    
    println!("2. Creating WASI context...");
    let wasi = WasiCtxBuilder::new()
        .inherit_stdio()
        .inherit_args()  // No ? here - returns &mut self, not Result
        .build();
    
    let state = MyState {
        wasi,
        table: ResourceTable::new(),
    };
    
    let mut store = Store::new(&engine, state);
    let mut linker = Linker::new(&engine);
    
    println!("3. Adding WASI to linker...");
    // CRITICAL: Use p2 for WASI Preview 2 (components with wasm32-wasip2)
    wasmtime_wasi::p2::add_to_linker_sync(&mut linker)?;
    
    println!("4. Loading component...");
    let component = Component::from_file(
        &engine,
        "../guest/target/wasm32-wasip2/release/hello_world_wasmtime_rust_component.wasm"
    )?;
    
    println!("5. Instantiating component...");
    let bindings = HelloWorld::instantiate(&mut store, &component, &linker)?;
    
    println!("\n=== Calling Component Functions ===\n");
    
    println!("Host: Calling add(5, 7)...");
    let result = bindings.call_add(&mut store, 5, 7)?;
    println!("Host: Result = {}\n", result);
    
    println!("Host: Calling greet()...");
    bindings.call_greet(&mut store)?;
    println!();
    
    println!("Host: Calling add(15, 10)...");
    let result = bindings.call_add(&mut store, 15, 10)?;
    println!("Host: 15 + 10 = {}\n", result);
    
    println!("=== Done! ===");
    Ok(())
}