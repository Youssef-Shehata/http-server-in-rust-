use std::io::Write;

use nom::AsBytes;
#[derive(Debug)]
pub struct HttpResponse {
    pub status_code: u16,
    pub status_text: String,
    pub headers: Vec<(String, String)>,
    pub body: Vec<u8>,
}

impl HttpResponse {
    pub fn new(status_code: u16, status_text: impl Into<String>) -> Self {
        HttpResponse {
            status_code,
            status_text: status_text.into(),
            headers: Vec::new(),
            body: Vec::new(),
        }
    }

    pub fn with_body(mut self, body: impl Into<Vec<u8>>) -> Self {
        self.body = body.into();
        self.add_header("Content-Length", self.body.len().to_string());
        self
    }

    pub fn add_header(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.headers.push((key.into(), value.into()));
    }

    pub fn to_bytes(&self) -> anyhow::Result<Vec<u8>> {
        let mut res = Vec::new();

        writeln!(res, "HTTP/1.1 {} {}", self.status_code, self.status_text)?;

        for (key, value) in &self.headers {
            writeln!(&mut res, "{}: {}", key, value)?;
        }

        writeln!(res)?;

        res.extend(&self.body);
        let bytes = res.as_bytes();
        Ok(bytes.to_vec())
    }
}
