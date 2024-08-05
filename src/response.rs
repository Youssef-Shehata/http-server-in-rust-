use crate::time;
use std::{array, collections::HashMap};
#[derive(Debug)]
pub struct Response {
    status: i32,
    headers: HashMap<String, String>,
    body: String,
}

impl Response {
    fn get_supported_compressions() -> Vec<String> {
        return vec![String::from("gzip")];
    }
    fn compress(body: &mut String, compression: &String) -> String {
        return body.clone();
    }
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
    pub fn to_string(mut self) -> String {
        if let Some(comp_string) = self.headers.get("Content-Encoding") {
            let mut compressed = false;
            let compressions: Vec<String> = comp_string
                .trim()
                .split(",")
                .map(|x| x.trim().to_string())
                .collect();
            for comp in compressions.iter() {
                if Response::get_supported_compressions().contains(comp) {
                    Response::compress(&mut self.body, comp);
                    self.headers
                        .entry(String::from("Content-Encoding"))
                        .and_modify(|x| *x = comp.clone());
                    println!("found com {comp}");
                    compressed = true;
                    break;
                }
            }
            if !compressed {
                println!("didnt find any comp : {compressions:?}");
                self.headers.remove("Content-Encoding");
            }
        }
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
