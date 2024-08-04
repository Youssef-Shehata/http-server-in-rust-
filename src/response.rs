use crate::time;
use std::collections::HashMap;
#[derive(Debug)]
pub struct Response {
    status: i32,
    headers: HashMap<String, String>,
    body: String,
}

impl Response {
    pub fn new(status: i32) -> Response {
        let mut map = HashMap::new();
        map.insert(String::from("Date"), time::get_current_time());
        Response {
            status,
            headers: map,
            body: String::from(""),
        }
    }
    //pub fn set_status(self: &mut Self, num: i32) {
    //self.status = num;
    //}
    pub fn set_header(self: &mut Self, header_name: &str, header_value: &str) {
        self.headers
            .insert(String::from(header_name), String::from(header_value));
    }
    pub fn set_body(self: &mut Self, body: &str) {
        self.body = String::from(body);
    }
    pub fn to_string(self) -> String {
        let status_line = match self.status {
            200 => "HTTP/1.1 200 OK",
            201 => "HTTP/1.1 200 Created",
            _ => "HTTP/1.1 404 Not Found",
        };
        let mut res = String::new();
        res.push_str(&format!("{}\r\n", status_line));
        for (key, value) in self.headers {
            res.push_str(&format!("{}: {}\r\n", &key[..], &value[..]));
        }

        res.push_str("\r\n");
        res.push_str(&self.body[..]);

        res
    }
}
