use http_server_starter_rust::ThreadPool;
mod client_handlers;
mod request;
mod response;
mod time;
use std::env;
use std::net::TcpListener;
use std::process;
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
                    client_handlers::handle_client(stream).unwrap_or_else(|e| {
                        eprint!("coudlnt handle request ERROR : {}", e);
                    });
                });
            }

            Err(e) => {
                println!("error in tcp stream: {}", e);
            }
        }
    }
}
