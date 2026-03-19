extern "C" {
#include "native/hello.h"
}
#include <string.h>

extern "C" void exports_wasi_http_incoming_handler_handle(
    exports_wasi_http_incoming_handler_own_incoming_request_t request,
    exports_wasi_http_incoming_handler_own_response_outparam_t response_out
) 

{
    // 1. Create empty headers and build the response
    wasi_http_types_own_fields_t headers = wasi_http_types_constructor_fields();
    wasi_http_types_own_outgoing_response_t response = wasi_http_types_constructor_outgoing_response(headers);

    // 2. Set status code to 200
    wasi_http_types_method_outgoing_response_set_status_code(
        (wasi_http_types_borrow_outgoing_response_t){ response.__handle },
        200
    );

    // 3. Get the body handle
    wasi_http_types_own_outgoing_body_t body;
    wasi_http_types_method_outgoing_response_body(
        (wasi_http_types_borrow_outgoing_response_t){ response.__handle },
        &body
    );

    // 4. Set the response outparam (sends headers)
    wasi_http_types_result_own_outgoing_response_error_code_t result;
    result.is_err = false;
    result.val.ok = response;
    wasi_http_types_static_response_outparam_set(response_out, &result);

    // 5. Write body content
    wasi_http_types_own_output_stream_t stream;
    wasi_http_types_method_outgoing_body_write(
        (wasi_http_types_borrow_outgoing_body_t){ body.__handle },
        &stream
    );

    const char *msg = "Hello from C++ + WASI!";
    hello_list_u8_t content;
    content.ptr = (uint8_t *)msg;
    content.len = strlen(msg);
    wasi_io_streams_method_output_stream_blocking_write_and_flush(
        (wasi_io_streams_borrow_output_stream_t){ stream.__handle },
        &content,
        NULL
    );

    // 6. Drop stream and finish body
    wasi_io_streams_output_stream_drop_own(stream);
    wasi_http_types_static_outgoing_body_finish(body, NULL, NULL);

    // Drop incoming request
    wasi_http_types_incoming_request_drop_own(request);
}
