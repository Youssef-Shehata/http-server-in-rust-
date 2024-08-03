use std::{
    error::Error,
    fs::File,
    io::{Read, Write},
    net::TcpStream,
};

pub fn echo(stream: &mut TcpStream, token: &str) -> Result<(), Box<dyn Error>> {
    let response = token.replace("/echo/", "");
    stream.write(
        format!(
            "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
            response.len(),
            response
        )
        .as_bytes(),
    )?;
    Ok(())
}
pub fn get_agent(stream: &mut TcpStream, lines: Vec<&str>) -> Result<(), Box<dyn Error>> {
    let agent = lines
        .iter()
        .find(|line| line.starts_with("User-Agent:"))
        .map(|line| *line)
        .unwrap_or("");
    println!("Agent Line:{:?}", agent);
    if agent == "" {
        stream.write(b"HTTP/1.1 404 NOT FOUND\r\n\r\n")?;
        return Ok(());
    }
    let agent: Vec<&str> = agent.split(" ").collect();

    let user_agent = agent.get(1);
    match user_agent {
        Some(response) => {
            stream.write(
                format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
                    response.len(),
                    response
                )
                .as_bytes(),
            )?;
        }
        None => {
            stream.write(b"HTTP/1.1 404 NOT FOUND\r\n\r\n")?;
        }
    };
    Ok(())
}
pub fn read_file_content(file_name: String) -> Result<String, Box<dyn Error>> {
    let path = format!("tmp/{}", file_name);
    println!("{path}");
    let mut file = File::open(path)?;

    let mut file_buffer = Vec::new();
    file.read_to_end(&mut file_buffer)?;
    let response = String::from_utf8(file_buffer)?;
    Ok(response)
}
pub fn get_file(stream: &mut TcpStream, token: String) -> Result<(), Box<dyn Error>> {
    let content = read_file_content(token);
    match content {
        Ok(response) => {

            let response = response.trim();
            println!("{response}");
            stream.write(
                format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: application/octet-stream\r\nContent-Length: {}\r\n\r\n{}",
                    response.len(),
                   response
                
            ).as_bytes() , )?;
        }
        Err(e) => {
            eprintln!("Error reading file {e}");

            stream.write(b"HTTP/1.1 404 NOT FOUND\r\n\r\n")?;
        }
    }
    Ok(())
}
