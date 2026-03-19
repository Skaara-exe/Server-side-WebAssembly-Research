wit_bindgen::generate!({
    world: "hello",
    path: "wit",
    generate_all, // Generate bindings for all dependencies listed in the world
}); 

use exports::wasi::http::incoming_handler::Guest; // this is the guest interface that we want to use in Rust code,
use wasi::http::types::*; // so we can use the types from the guest interface in Rust code,

struct HttpServer;

impl Guest for HttpServer {
    fn handle(_req: IncomingRequest, res_out: ResponseOutparam) {
        // 1. Create the response with status 200
        let response = OutgoingResponse::new(Fields::new());
        response.set_status_code(200).expect("set status");

        // 2. Get body handle BEFORE setting the response outparam
        let body = response.body().expect("body");

        // 3. Set the response outparam — this sends headers to the client
        //    Must happen before writing to the body stream
        ResponseOutparam::set(res_out, Ok(response));

        // 4. Write the body content
        let stream = body.write().expect("write");
        stream
            .blocking_write_and_flush(b"Hello from Rust!")
            .expect("write to stream");

        // 5. Drop stream first, then finish the body to signal completion
        drop(stream);
        OutgoingBody::finish(body, None).expect("finish body");
    }
}

export!(HttpServer);
