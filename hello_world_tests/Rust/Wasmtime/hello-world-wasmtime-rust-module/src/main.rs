//! Small example of how to instantiate a wasm module that uses WASI for println!

use wasmtime::*;
use wasmtime_wasi::WasiCtx;

fn main() -> Result<()> {
    println!("Compiling module...");
    let engine = Engine::default();
    let module = Module::from_file(&engine, "module/target/wasm32-wasip1/release/hello_world_wasmtime_rust_module.wasm")?;

    println!("Initializing...");
    
    // Create WASI context - use WasiCtx::builder(), not WasiP1Ctx::builder()
    let wasi = WasiCtx::builder()
        .inherit_stdio()
        .inherit_args()
        .build_p1();
    
    let mut store = Store::new(&engine, wasi);

    println!("Creating linker with WASI...");
    let mut linker = Linker::new(&engine);
    
    // Add WASI to the linker - p1::add_to_linker_sync is the right function
    wasmtime_wasi::p1::add_to_linker_sync(&mut linker, |s| s)?;

    println!("Instantiating module...");
    let instance = linker.instantiate(&mut store, &module)?;

    println!("Extracting exports...");
    let add = instance.get_typed_func::<(i32, i32), i32>(&mut store, "add")?;
    let greet = instance.get_typed_func::<(), ()>(&mut store, "greet")?;

    println!("Calling add(5, 7)...");
    let result = add.call(&mut store, (5, 7))?;
    println!("Result: {}", result);

    println!("Calling greet()...");
    greet.call(&mut store, ())?;

    println!("Calling add()...");
    println!("15+10={}", add.call(&mut store, (15,10))?);

    println!("Done.");
    Ok(())
}