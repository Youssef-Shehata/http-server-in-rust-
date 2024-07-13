// Uncomment this block to pass the first stage
 use std::{io::{Read, Write}, net::TcpListener};


fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    //
     let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

     let mut buffer = [0;1024];
     for stream in listener.incoming() {
       match stream {

           Ok(mut _stream) => {
               let _ = _stream.read(&mut buffer);
               let req = String::from_utf8_lossy(&buffer[..]);
               
               if  let Some(path )= extract_path(&req){
                   if path == "/"{
                       _stream.write(b"HTTP/1.1 200 OK\r\n\r\n").expect("200 \n");


                   }
                   
                   else{
                        _stream.write(b"HTTP/1.1 404 NOT FOUND\r\n\r\n").expect("404 \n");

                   }

           }
           }

           Err(e) => {
               println!("error: {}", e);
           }
       }
    }

}


fn extract_path(req: &str) -> Option<&str>{
   
     let req_lines: Vec<&str>  =  req.lines().collect();
     let req_line = req_lines[0];
     let req_parts : Vec<&str> = req_line.split_whitespace().collect();
     
     println!("request : {}", req_parts[1]);
     Some(req_parts[1])
}
