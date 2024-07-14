// Uncomment this block to pass the first stage
use std::{
    error::Error,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};
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

fn handle_client(mut stream: TcpStream) -> Result<(), Box<dyn Error>> {
    let mut buffer = [0; 1024];
    let _ = stream.read(&mut buffer);
    let req = String::from_utf8_lossy(&buffer[..]);
    let lines: Vec<&str> = req.split("\r\n").collect();
    let tokens: Vec<&str> = lines[0].split(" ").collect();
    let agent: Vec<&str> = lines[2].split(" ").collect();

    println!("{:?}", agent);
    match tokens[0] {
        "GET" => {
            if tokens[1] == "/" {
                stream.write(b"HTTP/1.1 200 OK\r\n\r\n")?;
            } else if tokens[1].starts_with("/echo/") {
                let response = tokens[1].replace("/echo/", "");
                stream.write(format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}", response.len(), response).as_bytes())?;
            } else if tokens[1].starts_with("/user-agent") {
                if let Some(response) = agent.get(1) {
                    stream.write(format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}", response.len(), response).as_bytes())?;
                } else {
                    stream.write(b"HTTP/1.1 404 NOT FOUND\r\n\r\n")?;
                }
            } else {
                stream.write(b"HTTP/1.1 404 Not Found\r\n\r\n")?;
            }
        }
        _ => {
            println!("Unknown method: {}", tokens[0])
        }
    }
    Ok(())
}
