mod connect;
mod handle;
mod config;

use crate::connect::epoll;
use config::config::*;
use crate::handle::handler;
// use connect::epoll;
// use config::config_output;

use std::{
    os::fd::AsRawFd,
    collections::HashMap,
    net::TcpListener,
};


fn main() {
    let config_path = format!("{}/src/config/config.txt", *PATH_SERVER);
    let configs = config_output(config_path.as_str());
    let mut listeners = Vec::new(); 

    for (ip , port ,domain_name) in configs {
        let addr = format!("{}:{}", ip, port);
        
        // println!("adresse {:?}", addr);
        match TcpListener::bind(&addr) {
            Ok(listener) => {
                if domain_name.is_empty() {
                    println!("Domain name empty listening on http://{}", addr);
                } else {
                    println!("Listening on http://{} for the domain http://{}", addr, domain_name);
                }
                listeners.push(listener);
            }
            Err(e) => {
                eprintln!("Error binding to address {}: {}", port, e);
            }
        }
        println!("listeners {:?}", listeners);
    }
    let listener = &listeners[0];
    listener.set_nonblocking(true).unwrap(); // Set non-blocking mode

    let epoll = epoll::Epoll::new();
    epoll.add_fd(listener.as_raw_fd());

    let mut clients = HashMap::new();

    loop {
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
                handler::handle_connection(stream);
            }
        }
    }
}
