// Uncomment this block to pass the first stage
use std::{
    error::Error,
    fmt::Debug,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};
#[derive(Debug)]
struct Request {
    method: String,
    path: String,
    //host: String,
    user_agent: String,
}

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    //
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                if let Err(e) = handle_client(stream) {
                    eprintln!("Failed to handle client : {}", e);
                }
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
    let agent_line = req_lines[3];

    let method_line_parts: Vec<&str> = method_line.split_whitespace().collect();
    //let host_line_parts: Vec<&str> = host_line.split_whitespace().collect();
    let agent_parts: Vec<&str> = agent_line.split_whitespace().collect();
    println!("agent parts : {:?}", agent_parts);
    let req = Request {
        method: method_line_parts[0].into(),
        path: method_line_parts[1].into(),
        //host: host_line_parts[1].into(),
        user_agent: agent_parts[1].into(),
    };
    Some(req)
}
fn handle_client(mut stream: TcpStream) -> Result<(), Box<dyn Error>> {
    let mut buffer = [0; 1024];
    let _ = stream.read(&mut buffer);
    let req = String::from_utf8_lossy(&buffer[..]);
    if let Some(request) = parse_request(&req) {
        let path = request.path;
        let agent = request.user_agent;
        println!("agent : {agent}");
        if path == String::from("/") {
            stream.write(b"HTTP/1.1 200 OK\r\n\r\n")?;
        } else if path.starts_with("/echo/") {
            let (_, data) = path.split_at(6);
            stream.write(
                format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
                    data.len(),
                    data
                )
                .as_bytes(),
            )?;
        } else if path.starts_with("/user-agent") {
            let data = agent;
            stream.write(
                format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
                    data.len(),
                    data
                )
                .as_bytes(),
            )?;
        } else {
            stream.write(b"HTTP/1.1 404 Not Found\r\n\r\n")?;
        }
        stream.flush().unwrap();
    }
    Ok(())
}
