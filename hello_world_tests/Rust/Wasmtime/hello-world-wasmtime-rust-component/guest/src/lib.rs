wit_bindgen::generate!({
    world: "hello-world",
});

struct MyComponent;

export!(MyComponent);

impl Guest for MyComponent {
    fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    fn greet() {
        println!("Hello world!");
    }
}