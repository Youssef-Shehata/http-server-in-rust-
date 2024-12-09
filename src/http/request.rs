
use crate::http::Headers;

use super::method::HttpMethod;


#[derive(Debug, PartialEq,Clone)]
pub enum HttpVersion {
    Http11,
}

impl HttpVersion {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "HTTP/1.1" => Some(HttpVersion::Http11),
            _ => None,
        }
    }
}

//TODO :: idk who needs this clone
#[derive(Debug, Clone)]
pub struct HttpRequest {
    pub method: HttpMethod,
    pub path: String,
    pub version: HttpVersion,
    pub headers: Headers,
    pub body: Vec<u8>,
}

impl HttpRequest {
    pub fn new(
        method: HttpMethod,
        path: String,
        version: HttpVersion,
        headers: Headers,
        body: Vec<u8>,
    ) -> Self {
        HttpRequest {
            method,
            path,
            version,
            headers,
            body,
        }
    }
}
