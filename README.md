# 🚀 Rust HTTP Server

this is a practice project, its not meant for production, atleast so far.

**this used to be a toy binary server but now im in the process of turning this into a somewhat usable library, currently fighting with tokio**

## ✨ Features

- **Zero-Copy Parsing**: Uses `nom` for efficient, zero-copy parsing of HTTP requests
- **Async I/O**: Built on `tokio` for high-performance async I/O operations
- **Type-Safe**: Leverages Rust's type system for compile-time correctness
- **Memory Efficient**: Minimizes allocations and copies using `BytesMut`


## 🔥 Performance


Benchmarks run **dev env** on Intel(R) Core i7-7500U @2.70GHz with 16GB of RAM Ubuntu 24.04.2 LTS

```
Framework          Requests/sec    Latency (p95)    
──────────────────────────────────────────────────
This Server         947,000         94.95 ms       
```

*Benchmark details: 2k requests, 100 concurrent connections, Keep-Alive enabled*

## 🚀 Quick Start

1. Add to your project:

```toml
[dependencies]
http-server-starter-rust = "0.1.0"
tokio = { version = "1.23.0", features = ["full"] }
```

2. Create a basic server:

```rust
use http_server_starter_rust::server::HttpServer;

#[tokio::main]
async fn main() -> Result<()> {
    let mut s = HttpServer::new("127.0.0.1:4221");

    let handle  = Arc::new(handle_request);
    s= s.get("/", handle.clone());

    s.run().await?;

    Ok(())

}
```

## 🛠️ API Examples

### Basic Request Handler

```rust
use http_server_starter_rust::http::{HttpResponse, HttpMethod};

fn handle_request(req: &HttpRequest) -> HttpResponse {
    match (req.method, req.path.as_str()) {
        (HttpMethod::Get, "/") => HttpResponse::new(200, "OK").with_body("Hello, World!"),
        _ => HttpResponse::new(404, "Not Found").with_body("Not Found"),
    }
}

```

### Custom Headers

```rust
let mut response = HttpResponse::new(200, "OK");
response.add_header("Content-Type", "application/json");
response.add_header("X-Custom-Header", "custom-value");
```

## 🌟 What Makes It Different?

1. **Zero-Copy Design**
   - Uses Rust's zero-copy parsing with `nom`
   - Minimizes memory allocations and copies
   - Efficient handling of request bodies

2. **Type-Safe Architecture**
   - Compile-time guarantees for HTTP methods and versions
   - No runtime parsing errors for known HTTP structures
   - Strong typing for headers and status codes

3. **Modern Async Foundation**
   - Built on `tokio` for scalable async I/O
   - Efficient connection pooling
   - Non-blocking request handling

4. **Memory Efficiency**
   - Uses `BytesMut` for efficient buffer management
   - Minimal allocations during request processing
   - Smart memory reuse for headers and common strings

## 🤝 Contributing

We welcome contributions! Here's how you can help:

1. **Fork & Clone**

2. **Build & Test**

3. **Submit Changes**
   - Create a feature branch
   - Make your changes
   - Add tests for new functionality
   - Submit a pull request

### Development Guidelines

- Follow Rust standard coding style
- Add tests for new features
- Run `cargo fmt` before committing

## 📈 Future Roadmap

- [ ] HTTP/2 Support
- [ ] WebSocket Integration
- [ ] TLS/HTTPS Support
- [ ] Request/Response Streaming
- [ ] Request Rate Limiting
- [ ] Response Compression

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

