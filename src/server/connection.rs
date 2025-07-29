use crate::errors::handler::error_response;
use crate::http::session::{build_set_cookie_header, parse_cookie_header, SESSION_STORE};
use crate::http::{request, router};
use std::io::Write;
use std::net::TcpStream;

// handle connection
pub fn handle_connection(mut stream: TcpStream) {
    match request::parse_request(&mut stream) {
        Some(mut request) => {
            let mut session_store = SESSION_STORE.lock().unwrap();
            // let _ = session_store.create_session();
            // println!("session id mb {:#?}", session_store);
            let session_id = request
                .headers
                .get("Cookie")
                .and_then(|cookie| parse_cookie_header(cookie).get("session_id").cloned())
                .filter(|id| session_store.get_session(id).is_some())
                .unwrap_or_else(|| {
                    let new_id = session_store.create_session();
                    let set_cookie = build_set_cookie_header(&new_id);
                    let _ = stream.write(format!("{}\r\n", set_cookie).as_bytes());
                    new_id
                });
            request.headers.insert("X-Session-ID".into(), session_id);
            // println!("method: {} for path: {}", request.method, request.path);
            router::route_request(request, &mut stream);
        }
        None => {
            eprintln!("Failed to parse request");
            error_response(400, &mut stream);
        }
    }
}
