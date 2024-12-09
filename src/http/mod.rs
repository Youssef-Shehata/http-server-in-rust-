mod request;
mod method;
mod response;
pub use method::HttpMethod;
pub use request::{ HttpRequest, HttpVersion};
pub use response::HttpResponse;

#[derive(Debug, PartialEq,Clone)]
pub struct Headers {
    headers: Vec<(String, String)>,
}

impl Headers {
    pub fn new() -> Self {
        Headers { headers: Vec::new()}
    }

    //accept any kinda of strings, cuz there is like a bilion of'em
    pub fn add(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.headers.push((key.into(), value.into()));
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        self.headers
            .iter()
            .find(|(k, _)| k.eq_ignore_ascii_case(key)) //match header (case insensetive)
            .map(|(_, v)| v.as_str())
    }
}
