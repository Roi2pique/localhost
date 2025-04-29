use crate::server::connection::serve_file;
use std::io::Write;
use std::net::TcpStream;
pub fn handle_get(_path: &str, stream: &mut TcpStream) {
    // Handle GET request
    // For example, you can read a file and send its content as a response
    let response = serve_file("ressources/index.html", stream);
    stream.write_all(response.as_bytes()).unwrap();
}
