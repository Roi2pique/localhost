// use std::io::prelude::*;
use std::net::TcpStream;

use crate::errors::handler::error_response;
use crate::http::{request, router};

// handle connection
pub fn handle_connection(mut stream: TcpStream) {
    match request::parse_request(&mut stream) {
        Some(request) => {
            println!(
                "Request:\n method:{} \n path:{} \n version:{} \n headers:{:#?} \n body:{:?} \n",
                request.method,
                request.path,
                request.version,
                request.headers,
                request.body.unwrap_or_default()
            );
            router::route_request(&request.method, &request.path, &mut stream);
        }
        None => {
            eprintln!("Failed to parse request");
            error_response(400, &mut stream);
        }
    }
    // let mut buffer = [0; 1024];
    // old one
    // match stream.read(&mut buffer) {
    //     Ok(0) => return,
    //     Ok(_) => {
    //         let request = String::from_utf8_lossy(&buffer[..]);
    // match request::parse_request(&request) {
    //             Some((method, path, version, domain)) => {
    //                 println!(
    //                     "Request: {} \n {} \n {} \n {} \n ",
    //                     method,
    //                     path,
    //                     version,
    //                     domain.unwrap(),
    //                 );
    //                 router::route_request(&method, &path, stream);
    //             }
    //             None => {
    //                 eprintln!("Failed to parse request");
    //                 error_response(400, stream);
    //             }
    //         }
    //     }
    //     Err(_) => return,
    // }
}

// use std::fs;
// pub fn serve_file(path: &str, _stream: &mut TcpStream) -> String {
//     match fs::read_to_string(path) {
//         Ok(content) => response::create_response("200 OK", &content),
//         Err(e) => response::create_response("404 Not Found", e.to_string().as_str()),
//     }
// }
