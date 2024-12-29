use std::sync::Arc;

use anyhow::Context;
use http_server_starter_rust::{
    http::{HttpMethod, HttpRequest, HttpResponse, HttpVersion},
    parser::parse_request,
    server::HttpServer,
};
#[tokio::main]
async fn handle_request(req: &HttpRequest) -> HttpResponse {
    match (req.method, req.path.as_str()) {
        (HttpMethod::Get, "/") => HttpResponse::new(200, "OK").with_body("Hello, World!"),
        _ => HttpResponse::new(404, "Not Found").with_body("Not Found"),
    }
}
#[test]
fn req() -> Result<(), http_server_starter_rust::Error> {
    let server = Box::new(HttpServer::new("172.0.0.1:8080"));
    let mut server: &'static HttpServer= Box::leak(server);
    let handler = move |req:&HttpRequest|{
        return handle_request(req)

    };
    server.run();

    Ok(())
}
