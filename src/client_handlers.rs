use crate::request;
use crate::request::Request;
use crate::response;
use std::{
    error::Error,
    fs::File,
    io::{Read, Write},
    net::TcpStream,
};
pub fn handle_client(mut stream: TcpStream) -> Result<(), Box<dyn Error>> {
    let mut buffer = [0; 1024];
    let _ = stream.read(&mut buffer);
    let req_string = String::from_utf8_lossy(&buffer[..]);
    let request = request::Request::new(req_string.to_string());

    println!("{request:?}");
    match &request.method[..] {
        "GET" => match &request.path[..] {
            "/" => {
                let res = response::Response::new(200);
                stream.write(format!("{}", res.to_string()).as_bytes())?;
            }
            r if r.starts_with("/echo/") => {
                echo(&mut stream, &request)?;
            }
            r if r.starts_with("/user-agent") => {
                get_agent(&mut stream, &request)?;
            }
            r if r.starts_with("/files/") => {
                read_existent_file(&mut stream, &request)?;
            }
            _ => {
                stream.write(b"HTTP/1.1 404 Not Found\r\n\r\n")?;
            }
        },
        "POST" => match &request.path[..] {
            files if files.starts_with("/files/") => write_new_file(&mut stream, &request)?,
            _ => {
                stream.write(b"HTTP/1.1 404 Not Found\r\n\r\n")?;
            }
        },
        _ => {
            println!("Unknown method: {}", request.method);
        }
    }
    Ok(())
}

fn echo(stream: &mut TcpStream, request: &Request) -> Result<(), Box<dyn Error>> {
    let token = &request.path;
    let query = token.replace("/echo/", "");
    let mut response = response::Response::new(200);
    response.set_header("Content-Type", "text/plain");
    response.set_header("Content-Length", &format!("{}", query.len()));
    if let Some(compression) = request.headers.get("Accept-Encoding") {
        response.set_header("Content-Encoding", &compression);
    }
    response.set_body(&query);

    stream.write(format!("{}", response.to_string()).as_bytes())?;
    Ok(())
}
fn get_agent(stream: &mut TcpStream, request: &Request) -> Result<(), Box<dyn Error>> {
    let user_agent_header = request.headers.get("User-Agent");
    match user_agent_header {
        Some(header) => {
            let mut response = response::Response::new(200);
            response.set_header("Content-Type", "text/plain");
            response.set_header("Content-Length", &format!("{}", header.len()));
            response.set_body(&header);
            stream.write(format!("{}", response.to_string()).as_bytes())?;
        }
        None => {
            let response = response::Response::new(400).to_string();
            stream.write(format!("{}", response).as_bytes())?;
        }
    }
    Ok(())
}
fn read_file_content(file_name: String) -> Result<String, Box<dyn Error>> {
    let path = format!("tmp/{}", file_name);
    println!("reading file : {file_name}");
    let mut file = File::open(path)?;

    let mut file_buffer = Vec::new();
    file.read_to_end(&mut file_buffer)?;
    let response = String::from_utf8(file_buffer)?;
    Ok(response)
}
fn read_existent_file(stream: &mut TcpStream, request: &Request) -> Result<(), Box<dyn Error>> {
    let token = request.path.replace("/files/", "");
    let content = read_file_content(token);
    match content {
        Ok(file_data) => {
            let file_data = file_data.trim();
            let mut response = response::Response::new(200);
            response.set_header("Content-Type", "application/octet-stream");
            response.set_header("Content-Length", &format!("{}", file_data.len()));
            response.set_body(file_data);
            stream.write(format!("{}", response.to_string()).as_bytes())?;
        }
        Err(e) => {
            eprintln!("Error reading file {e}");
            let response = response::Response::new(400).to_string();
            stream.write(format!("{}", response).as_bytes())?;
        }
    }
    Ok(())
}
fn write_new_file(stream: &mut TcpStream, request: &Request) -> Result<(), Box<dyn Error>> {
    let file_name = request.path.replace("/files/", "");
    let file_body = &request.body;
    let expected_length: usize = request
        .headers
        .get("Content-Length")
        .unwrap_or(&String::from("0"))
        .parse()?;
    if file_body.len() != expected_length {
        Err(format!(
            "Content-Length doesnt match actual content : Expected: {} , Recieved: {}",
            expected_length,
            file_body.len(),
        ))?
    };
    let mut file =
        File::create(format!("tmp/{file_name}")).expect("creating file {file_name} failed");
    file.write(file_body.as_bytes())
        .expect("error writing to {file_name}");

    let res = response::Response::new(201);

    println!("writing {file_body} to {file_name}");
    stream.write(format!("{}", res.to_string()).as_bytes())?;
    Ok(())
}
