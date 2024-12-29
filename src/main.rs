use std::sync::Arc;

use http_server_starter_rust::http::{HttpMethod, HttpRequest, HttpResponse};
use http_server_starter_rust::server::HttpServer;
use http_server_starter_rust::Error;

fn handle_request(req: &HttpRequest) -> HttpResponse {
    match (req.method, req.path.as_str()) {
        (HttpMethod::Get, "/") => HttpResponse::new(200, "OK").with_body("Hello, World!"),
        _ => HttpResponse::new(404, "Not Found").with_body("Not Found"),
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let mut s = HttpServer::new("127.0.0.1:4221");

    let handle  = Arc::new(handle_request);
    s= s.get("/", handle.clone());

    s.run().await?;

    Ok(())
}
