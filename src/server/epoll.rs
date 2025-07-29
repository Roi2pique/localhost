use crate::server::connection;
use libc::{epoll_create1, epoll_ctl, epoll_wait, EPOLLIN, EPOLL_CTL_ADD};
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

// modify to accept the multiple possiblities of listeners
pub fn run_epoll(listerners: Vec<TcpListener>) {
    let mut handler = Vec::new();
    for listener in listerners {
        let handle = thread::spawn(move || {
            listener.set_nonblocking(true).unwrap();

            let epoll = Epoll::new();
            epoll.add_fd(listener.as_raw_fd());
            let mut clients = HashMap::new();

            loop {
                for fd in epoll.wait(10) {
                    if fd == listener.as_raw_fd() {
                        if let Ok((stream, _)) = listener.accept() {
                            let fd = stream.as_raw_fd();
                            stream.set_nonblocking(true).unwrap();
                            epoll.add_fd(fd);
                            clients.insert(fd, stream);
                        }
                    } else if let Some(stream) = clients.remove(&fd) {
                        connection::handle_connection(stream);
                        // Maybe reinsert the stream if needed after handling
                        // clients.insert(fd, stream);
                    }
                }
            }
        });
        handler.push(handle);
    }
    for handle in handler {
        handle.join().unwrap();
    }
}
