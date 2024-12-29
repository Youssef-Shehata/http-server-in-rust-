
use crate::http::{HttpRequest, HttpResponse};
use crate::router::RouteMatch;
use super::Next;


pub struct Cors {
    allow_origin: String,
    allow_methods: Vec<String>,
    allow_headers: Vec<String>,
}
impl Cors {
    pub fn new() -> Self {
        Cors {
            allow_origin: "*".to_string(),
            allow_methods: vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"]
                .into_iter()
                .map(String::from)
                .collect(),
                    
            allow_headers: vec!["Content-Type", "Authorization"]
                .into_iter()
                .map(String::from)
                .collect(),
        }
    }

    pub async fn handle(&self, req: HttpRequest, route: RouteMatch, next: Next<'_>) -> HttpResponse {
        let mut response = next.await;

        response.add_header(
            "Access-Control-Allow-Origin".to_string(),
            self.allow_origin.clone(),
        );
        response.add_header(
            "Access-Control-Allow-Methods".to_string(),
            self.allow_methods.join(", "),
        );
        response.add_header(
            "Access-Control-Allow-Headers".to_string(),
            self.allow_headers.join(", "),
        );

        response
    }
}
