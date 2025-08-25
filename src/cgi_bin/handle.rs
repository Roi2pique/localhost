// Launches and communicates with CGI scripts

use crate::errors::handler::error_response;
use crate::http::request::HttpRequest;
use std::io::Write;
use std::net::TcpStream;
use std::process::{Command, Stdio};

pub fn handle_cgi(req: &HttpRequest, stream: &mut TcpStream) {
    // Extract the script path: assuming /scripts/filename.cgi
    let script_name = req.path.trim_start_matches("/scripts/");
    let script_path = format!("./src/cgi_bin/scripts/{}", script_name);

    // Sanity check: prevent traversal
    if script_name.contains("..") {
        eprintln!("Forbidden path");
        error_response(403, stream);
        return;
    }

    // Prepare the command
    let mut command = if let Some(interpreter) = get_interpreter(&script_path) {
        let mut cmd = Command::new(interpreter);
        cmd.arg(&script_path);
        cmd
    } else {
        Command::new(&script_path)
    };

    command.env("REQUEST_METHOD", &req.method);
    command.env("PATH_INFO", &req.path);

    // Body
    let body = req.text_body.as_deref().unwrap_or("");
    command.env("CONTENT_LENGTH", body.len().to_string());

    println!("Executing CGI script: {:#?}", command);

    let mut child = match command
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
    {
        Ok(c) => c,
        Err(e) => {
            eprintln!("CGI spawn failed: {}", e);
            error_response(500, stream);
            return;
        }
    };

    // Write body to stdin of CGI
    if let Some(mut stdin) = child.stdin.take() {
        let _ = stdin.write_all(body.as_bytes());
    }

    let output = match child.wait_with_output() {
        Ok(out) => out,
        Err(e) => {
            eprintln!("CGI failed: {}", e);
            error_response(500, stream);
            return;
        }
    };

    let response = String::from_utf8_lossy(&output.stdout);
    let full_response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        response.len(),
        response
    );
    let _ = stream.write_all(full_response.as_bytes());
}

fn get_interpreter(path: &str) -> Option<&str> {
    if path.ends_with(".py") {
        Some("python3")
    } else if path.ends_with(".sh") || path.ends_with(".cgi") {
        Some("bash")
    } else {
        None // assume binary
    }
}
