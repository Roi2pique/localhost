use crate::http::request::HttpRequest;
use crate::http::session::*;
use std::net::TcpStream;

pub fn handle_session(req: &mut HttpRequest, _stream: &mut TcpStream) {
    let mut store = SESSION_STORE.lock().unwrap();

    let sid = req
        .headers
        .get("Cookie")
        .and_then(|cookie| parse_cookie_header(cookie).get("session_id").cloned())
        .filter(|id| store.get_session(id).is_some())
        .unwrap_or_else(|| {
            let new_id = store.create_session();
            let set_cookie = build_set_cookie_header(&new_id);

            req.extra_response_headers.push(set_cookie); // stash here
            new_id
        });

    req.session_id = Some(sid);
}
