use http_server_starter_rust::ThreadPool;
mod client_handlers;
use std::env;
use std::process;
use std::{
    error::Error,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};
fn main() {
    if let None = env::args().nth(1) {
        println!("please provide a port number to listen on..");
        process::exit(1);
    }
    println!("Logs from your program will appear here!");

    let port = env::args().nth(1).unwrap();

    let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).unwrap();
    let pool = ThreadPool::new(5);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                pool.execute(|| {
                    handle_client(stream).unwrap_or_else(|e| {
                        eprint!("coudlnt handle request ERROR : {}", e);
                    });
                });
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
    match tokens[0] {
        "GET" => {
            if tokens[1] == "/" {
                stream.write(b"HTTP/1.1 200 OK\r\n\r\n")?;
            } else if tokens[1].starts_with("/echo/") {
                client_handlers::echo(&mut stream, tokens[1])?;
            } else if tokens[1].starts_with("/user-agent") {
                client_handlers::get_agent(&mut stream, lines)?;
            } else if tokens[1].starts_with("/files/") {
                client_handlers::get_file(&mut stream, tokens[1].replace("/files/", ""))?;
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
