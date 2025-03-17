mod epoll;
use epoll::Epoll;
use std::{
    os::fd::AsRawFd,
    collections::HashMap,
    // thread,
    fs,
    io::prelude::*,
    net::{TcpListener, TcpStream},
    // time::Duration,
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    listener.set_nonblocking(true).unwrap(); // Set non-blocking mode

    let epoll = Epoll::new();
    epoll.add_fd(listener.as_raw_fd());
    let mut clients = HashMap::new();

    loop {
        // for fd in epoll.wait(10) {
        //     if fd == listener.as_raw_fd() {
        //         let (stream, _) = listener.accept().unwrap();
        //         stream.set_nonblocking(true).unwrap();
        //         epoll.add_fd(stream.as_raw_fd());
        //     } else {
        //         handle_client(fd);
        //     }
        // }
        for fd in epoll.wait(10) {
            if fd == listener.as_raw_fd() {
                // Accept new client connections
                if let Ok((stream, _)) = listener.accept() {
                    let fd = stream.as_raw_fd();
                    stream.set_nonblocking(true).unwrap(); // Set non-blocking mode
                    epoll.add_fd(fd);
                    clients.insert(fd, stream);
                }
            } else if let Some(stream) = clients.get_mut(&fd) {
                handle_connection(stream);
            }
        }
    }
}

fn handle_connection(stream: &mut TcpStream) {
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

            let contents = fs::read_to_string(filename).unwrap_or_else(|_| "404 Not Found".to_string());
            let length = contents.len();

            let response = format!(
                "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
            );

            // Write response (non-blocking)
            let _ = stream.write_all(response.as_bytes());
        }
        Err(_) => return, // Ignore non-blocking read errors
    }
}

/*
fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };
    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response = format!(
        "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
    );

    stream.write_all(response.as_bytes()).unwrap();
}
*/