use std::collections::HashMap;
use std::future::Future;
use std::sync::Arc;

use crate::http::{HttpMethod, HttpRequest, HttpResponse};
use crate::middleware::MiddlewareFn;

pub struct Router {
    routes: HashMap<(HttpMethod, String), RouteHandler>,
    middleware: Vec<MiddlewareFn>,
}

pub type RouteHandler = Arc<dyn Fn(&HttpRequest) -> HttpResponse + Send + Sync>;


#[derive(Clone)]
pub struct RouteMatch {
    pub params: HashMap<String, String>,
}

impl Router {
    pub fn new() -> Self {
        Router {
            routes: HashMap::new(),
            middleware: Vec::new(),
        }
    }

    pub fn add_route(&mut self, method: HttpMethod, path: &str, handler: RouteHandler) {
        self.routes.insert((method, path.to_string()), handler);
    }

    pub fn middleware(&mut self, middleware: MiddlewareFn) {
        self.middleware.push(middleware);
    }

    pub async fn handle(&self, req: HttpRequest) -> HttpResponse {
        let route_match = self.match_route(&req);

        if let Some((handler, route_match)) = route_match {
            let mut response = handler(&req);

            // Apply middleware in reverse order
            for middleware in self.middleware.iter().rev() {
                response = middleware(
                    req.clone(),
                    route_match.clone(),
                    Box::pin(async move { response }),
                )
                .await;
            }

            response
        } else {
            HttpResponse::new(404, "Not Found")
        }
    }

    fn match_route(&self, req: &HttpRequest) -> Option<(RouteHandler, RouteMatch)> {
        self.routes
            .get(&(req.method, req.path.clone()))
            .map(|handler| {
                (
                    handler.clone(),
                    RouteMatch {
                        params: HashMap::new(),
                    },
                )
            })
    }
}
