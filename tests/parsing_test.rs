use anyhow::Context;
use http_server_starter_rust::{
    http::{HttpMethod, HttpRequest, HttpVersion},
    parser::parse_request,
};
use nom::AsBytes;
use pretty_assertions::assert_eq;
fn assert_request(
    request: HttpRequest,
    method: HttpMethod,
    path: &str,
    version: HttpVersion,
    headers: Vec<(&str, &str)>,
    body: &str,
) -> anyhow::Result<()> {
    assert_eq!(request.method, method);

    assert_eq!(request.path, path);

    assert_eq!(request.version, version);
    for header in headers.iter() {
        assert_eq!(
            request
                .headers
                .get(&header.0)
                .context("unwrapping content-type header")?,
            header.1
        );
    }
    assert_eq!(request.body, body.as_bytes());
    Ok(())
}

#[test]
fn normal_req() {
    let buffer = "GET / HTTP/1.1\r\nHost: localhost\r\nContent-Type: text/plain; charset=utf8\r\nContent-Length: 5\r\n\r\nhello" ;
    let (_, request) = parse_request(buffer.as_bytes()).unwrap();
    let _ = assert_request(
        request,
        HttpMethod::Get,
        "/",
        HttpVersion::Http11,
        vec![("Content-Type", "text/plain; charset=utf8"),
             ("Content-Length","5") ],
        "hello",
    );
}

#[test]
fn wrong_content_length() {
    let buffer = "GET / HTTP/1.1\r\nHost: localhost\r\nContent-Type: text/plain; charset=utf8\r\nContent-Length: 3\r\n\r\nhello";
    let (_, request) = parse_request(buffer.as_bytes()).unwrap();
    let _ = assert_request(
        request,
        HttpMethod::Get,
        "/",
        HttpVersion::Http11,
        vec![("Content-Type", "text/plain; charset=utf8"),
             ("Content-Length","3") ],
        "hel",
    );
}
