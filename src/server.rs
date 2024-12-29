use crate::http::HttpMethod;
use crate::router::RouteHandler;
use crate::Error;
use bytes::BytesMut;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::broadcast;

use crate::parser::parse_request;
use crate::{http::HttpResponse, pool::ConnectionPool, router::Router, Result};

pub struct HttpServer {
    addr: String,
    pub router: Arc<Router>,
    pool: ConnectionPool,
}

impl HttpServer {
    pub fn new(addr: impl Into<String>) -> Self {
        HttpServer {
            addr: addr.into(),
            router: Arc::new(Router::new()),
            pool: ConnectionPool::new(100), // Default pool size
        }
    }

    pub fn get(mut self, path: impl Into<String>, handler: RouteHandler) -> Self {
        let router = Arc::get_mut(&mut self.router).unwrap();
        router.add_route(HttpMethod::Get, &path.into(), handler);
        self
    }

    pub fn post(mut self, path: impl Into<String>, handler: RouteHandler) -> Self {
        let router = Arc::get_mut(&mut self.router).unwrap();
        router.add_route(HttpMethod::Post, &path.into(), handler);
        self
    }
    pub fn del(mut self, path: impl Into<String>, handler: RouteHandler) -> Self {
        let router = Arc::get_mut(&mut self.router).unwrap();
        router.add_route(HttpMethod::Delete, &path.into(), handler);
        self
    }
    pub async fn run(&self) -> Result<()> {
        let listener = TcpListener::bind(&self.addr).await?;
        println!("Server listening on {}", self.addr);

        let (shutdown_tx, _) = broadcast::channel(1);

        loop {
            let (stream, addr) = listener.accept().await?;
            println!("New connection from: {}", addr);

            let router = self.router.clone();
            let pool = self.pool.clone(); // Clone the pool as well

            let shutdown_rx = shutdown_tx.subscribe();

            tokio::spawn(async move {
                handle_connection(stream, router, &pool, shutdown_rx).await
            });
        }
    }
}

async fn handle_connection(
    mut stream: TcpStream,
    router: Arc<Router>,
    pool: &ConnectionPool,
    mut shutdown_rx: broadcast::Receiver<()>,
) {
    let mut buffer = BytesMut::with_capacity(1024);

    loop {
        tokio::select! {
            result = stream.read_buf(&mut buffer) => {
                match result {
                    Ok(0) => break, // EOF
                    Ok(_) => {
                        if is_websocket_request(&buffer) {
                            //handle_websocket(stream).await;
                            //break;
                            todo!();
                        } else {
                            match handle_http_request(&mut buffer, &router).await {
                                Ok(response) => {
                                    if let Err(e) = stream.write_all(&response.to_bytes().unwrap()).await {
                                        eprintln!("Failed to write response: {}", e);
                                        break;
                                    }
                                }
                                Err(e) => {
                                    eprintln!("Error handling request: {}", e);
                                    break;
                                }
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Failed to read from socket: {}", e);
                        break;
                    }
                }
            }
            _ = shutdown_rx.recv() => {
                break;
            }
        }
    }

    pool.release(stream).await;
}

fn is_websocket_request(buffer: &[u8]) -> bool {
    // Check for "Upgrade: websocket" header
    if let Ok(request) = std::str::from_utf8(buffer) {
        request.contains("Upgrade: websocket")
    } else {
        false
    }
}

async fn handle_http_request(buffer: &mut BytesMut, router: &Router) -> Result<HttpResponse> {
    let (_, request) = parse_request(buffer).map_err(|e| Error::Parse(e.to_string()))?;

    println!("{:?}", request);
    Ok(router.handle(request).await)
}

async fn handle_websocket() {
    todo!()
}
