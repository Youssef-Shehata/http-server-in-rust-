use std::time::Instant;

use crate::http::{HttpRequest, HttpResponse};
use crate::router::RouteMatch;
use super::Next;

pub struct Logger;

impl Logger {
    pub fn new() -> Self {
        Logger
    }

    pub async fn handle(&self, req: HttpRequest, route: RouteMatch, next: Next<'_>) -> HttpResponse {
        let start = Instant::now();
        println!("-> {} {}", req.method, req.path);

        let response = next.await;
        
        let duration = start.elapsed();
        println!("<- {} {} ({:?})", response.status_code, req.path, duration);

        HttpResponse{
            status_code: response.status_code ,
            status_text: response.status_text,
            headers: response.headers,
            body: response.body,
        }
    }
}
