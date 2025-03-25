use std::{
    fs,
    net::TcpStream,
    io::prelude::*,
};

pub fn handle_connection(stream: &mut TcpStream) {
    println!("{:?}", stream);
    let mut buffer = [0; 1024];
    
    // Read request (non-blocking)
    match stream.read(&mut buffer) {
        Ok(0) => return, // Connection closed by client
        Ok(_) => {
            let request = String::from_utf8_lossy(&buffer);
            let request_line = request.lines().next().unwrap_or("");

            let (status_line, filename) = match request_line {
                "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
                _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
            };

            let contents = serve_file(filename);
            let response = create_response(status_line, contents.as_str());

            // Write response (non-blocking)
            let _ = stream.write_all(response.as_bytes());
        }
        Err(_) => return, // Ignore non-blocking read errors
    }
}

pub fn serve_file(path: &str) -> String {
    match fs::read_to_string(path) {
        Ok(content) => create_response("200 OK", &content),
        Err(_) => create_response("404 Not Found", "<h1>404 - Not Found</h1>"),
    }
}

pub fn create_response(status: &str, body: &str) -> String {
    format!(
        "HTTP/1.1 {}\r\nContent-Length: {}\r\nContent-Type: text/html\r\n\r\n{}",
        status,
        body.len(),
        body
    )
}