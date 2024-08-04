use std::collections::HashMap;

#[derive(Debug)]
pub struct Request {
    pub method: String,
    pub path: String,
    pub headers: HashMap<String, String>,
    pub body: String,
}

impl Request {
    pub fn new(req_string: String) -> Request {
        let request_parts: Vec<&str> = req_string.split("\r\n\r\n").collect();
        let body = request_parts[1].split_once("\0\0\0").unwrap_or(("", "")).0;
        let lines: Vec<&str> = request_parts[0].split("\r\n").collect();
        let tokens: Vec<&str> = lines[0].split(" ").collect();

        let mut headers_map = HashMap::new();
        for header in lines[1..].iter() {
            let header_string: Vec<&str> = header.split(":").collect();
            headers_map.insert(
                header_string[0].trim().to_string(),
                header_string[1].trim().to_string(),
            );
        }
        Request {
            method: tokens[0].to_string(),
            path: tokens[1].to_string(),
            headers: headers_map,
            body: body.to_string(),
        }
    }
}
