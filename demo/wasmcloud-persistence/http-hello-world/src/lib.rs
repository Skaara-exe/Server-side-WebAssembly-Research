use wstd::http::{Body, Request, Response, StatusCode};
use wasmcloud_component::wasi::keyvalue::*;

#[wstd::http_server]
async fn main(req: Request<Body>) -> Result<Response<Body>, wstd::http::Error> {
    match req.uri().path_and_query().unwrap().as_str() {
        "/" => home(req).await,
        _ => not_found(req).await,
    }
}

async fn home(_req: Request<Body>) -> Result<Response<Body>, wstd::http::Error> {
    // Return a simple response with a string body
    let bucket = store::open("default").unwrap();
    let count = atomics::increment(&bucket, "counter", 1).unwrap();
    Ok(Response::new(format!("Hello from wasmCloud! I was called {count} times\n").into()))
}

async fn not_found(_req: Request<Body>) -> Result<Response<Body>, wstd::http::Error> {
    Ok(Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body("Not found\n".into())
        .unwrap())
}
