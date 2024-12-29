use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use crate::http::{HttpRequest, HttpResponse};
use crate::router::RouteMatch;

pub type Next<'a> = Pin<Box<dyn Future<Output = HttpResponse> + Send + 'a>>;
pub type MiddlewareFn = Arc<dyn Fn(HttpRequest, RouteMatch, Next) -> Pin<Box<dyn Future<Output = HttpResponse> + Send>> + Send + Sync>;

pub mod cors;
pub mod logger;
pub mod static_files;

pub use cors::Cors;
pub use logger::Logger;
pub use static_files::StaticFiles;
