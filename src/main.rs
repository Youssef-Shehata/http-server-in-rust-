// Uncomment this block to pass the first stage
use std::{
    fmt::Debug,
    io::{Read, Write},
    net::TcpListener,
};
#[derive(Debug)]
struct Request {
    method: String,
    path: String,
    host: String,
    user_agent: String,
}

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    //
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    let mut buffer = [0; 1024];
    for stream in listener.incoming() {
        match stream {
            Ok(mut _stream) => {
                let _ = _stream.read(&mut buffer);
                let req = String::from_utf8_lossy(&buffer[..]);
                let mut response = String::from("HTTP/1.1 404 Not Found\r\n");
                if let Some(request) = parse_request(&req) {
                    println!("{:?}", request);
                    if let Some(path_parts) = parse_path(request.path) {
                        if path_parts.is_empty() {
                            response = String::from("HTTP/1.1 200 OK\r\n");
                            _stream.write_all(response.as_bytes()).unwrap();
                        } else if path_parts.len() >= 1 && path_parts[0] == "echo" {
                            if let Some(body) = path_parts.get(1) {
                                response = format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",path_parts[1].len() , body);
                            } else {
                                response = format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",0 ,"");
                            }

                            _stream.write_all(response.as_bytes()).unwrap();
                        } else {
                            _stream.write_all(response.as_bytes()).unwrap();
                        }
                    }
                }
                _stream.flush().unwrap()
            }

            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn parse_request(req: &str) -> Option<Request> {
    let req_lines: Vec<&str> = req.lines().collect();
    let method_line = req_lines[0];
    let host_line = req_lines[1];
    let agent_line = req_lines[2];

    let method_line_parts: Vec<&str> = method_line.split_whitespace().collect();
    let host_line_parts: Vec<&str> = host_line.split_whitespace().collect();
    let agent_parts: Vec<&str> = agent_line.split_whitespace().collect();
    let req = Request {
        method: method_line_parts[0].into(),
        path: method_line_parts[1].into(),
        host: host_line_parts[1].into(),
        user_agent: agent_parts[1].into(),
    };
    Some(req)
}

fn parse_path(path: String) -> Option<Vec<String>> {
    let path_parts: Vec<&str> = path.split('/').collect();
    let filtered_path_parts: Vec<String> = path_parts
        .into_iter()
        .filter(|&s| !s.is_empty())
        .map(|s| s.to_string())
        .collect();
    Some(filtered_path_parts)
}
