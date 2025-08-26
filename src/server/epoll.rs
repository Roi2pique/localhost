use crate::http::{request, router};
// use crate::server::connection;
use libc::{close, epoll_create1, epoll_ctl, epoll_wait, EPOLLIN, EPOLL_CTL_ADD, EPOLL_CTL_DEL};
use std::io::Read;
use std::net::TcpStream;
use std::{
    collections::HashMap,
    net::TcpListener,
    os::{fd::AsRawFd, unix::io::RawFd},
    process::Command,
    thread,
};
#[derive(Debug)]
pub struct Epoll {
    epoll_fd: RawFd,
}

struct Client {
    stream: TcpStream,
    buffer: Vec<u8>,
    parsed: bool,
}

impl Epoll {
    pub fn new() -> Self {
        let epoll_fd = unsafe { epoll_create1(0) };
        assert!(epoll_fd >= 0);
        Self { epoll_fd }
    }

    pub fn add_fd(&self, fd: RawFd) {
        let mut event = libc::epoll_event {
            events: EPOLLIN as u32,
            u64: fd as u64,
        };
        unsafe {
            epoll_ctl(self.epoll_fd, EPOLL_CTL_ADD, fd, &mut event);
        }
    }

    pub fn remove_fd(&self, fd: RawFd) {
        let res = unsafe { epoll_ctl(self.epoll_fd, EPOLL_CTL_DEL, fd, std::ptr::null_mut()) };
        if res < 0 {
            eprintln!("epoll_ctl DEL failed for fd {}", fd);
        }
        // ❌ do not call close(fd) here!
    }

    pub fn wait(&self, max_events: usize) -> Vec<RawFd> {
        let mut events = vec![libc::epoll_event { events: 0, u64: 0 }; max_events];
        let num_events =
            unsafe { epoll_wait(self.epoll_fd, events.as_mut_ptr(), max_events as i32, -1) };
        assert!(num_events >= 0);

        events
            .into_iter()
            .take(num_events as usize)
            .map(|e| e.u64 as RawFd)
            .collect()
    }
}

pub fn path_server() -> String {
    if cfg!(target_os = "linux") {
        match Command::new("pwd").output() {
            Ok(output) => {
                if output.status.success() {
                    match String::from_utf8(output.stdout) {
                        Ok(stdout) => stdout.trim().to_string(),
                        Err(_) => {
                            eprintln!("Error converting output");
                            String::new()
                        }
                    }
                } else {
                    eprintln!("Error when searching for path");
                    String::new()
                }
            }
            Err(_) => {
                eprintln!("Error when executing command for path");
                String::new()
            }
        }
    } else {
        eprintln!("This OS is not supported");
        String::new()
    }
}

pub fn run_epoll(listerners: Vec<TcpListener>) {
    let mut handler = Vec::new();
    for listener in listerners {
        let handle = thread::spawn(move || {
            listener.set_nonblocking(true).unwrap();

            let epoll = Epoll::new();
            epoll.add_fd(listener.as_raw_fd());
            let mut clients: HashMap<i32, Client> = HashMap::new();

            loop {
                let mut to_remove = Vec::new();

                for fd in epoll.wait(10) {
                    if fd == listener.as_raw_fd() {
                        // Accept new connection
                        if let Ok((stream, _)) = listener.accept() {
                            let fd = stream.as_raw_fd();
                            stream.set_nonblocking(true).unwrap();
                            epoll.add_fd(fd);
                            clients.insert(
                                fd,
                                Client {
                                    stream,
                                    buffer: Vec::new(),
                                    parsed: false,
                                },
                            );
                        }
                    } else if let Some(client) = clients.get_mut(&fd) {
                        let mut tmp = [0u8; 4096];
                        match client.stream.read(&mut tmp) {
                            Ok(0) => {
                                // connection closed
                                to_remove.push(fd);
                            }
                            Ok(n) => {
                                client.buffer.extend_from_slice(&tmp[..n]);
                                while let Some(req) =
                                    request::parse_request_from_buffer(&mut client.buffer)
                                {
                                    router::route_request(req, &mut client.stream);
                                }
                            }
                            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                                // just wait for next epoll event
                            }
                            Err(e) => {
                                eprintln!("Error on fd {}: {}", fd, e);
                                to_remove.push(fd);
                            }
                        }
                    }
                }

                // ✅ Cleanup safely
                for fd in to_remove {
                    epoll.remove_fd(fd); // just deregister, no close
                    clients.remove(&fd); // TcpStream dropped -> fd closed here
                }
            }
        });
        handler.push(handle);
    }

    for handle in handler {
        handle.join().unwrap();
    }
}
