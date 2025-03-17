use libc::{epoll_create1, epoll_ctl, epoll_wait, EPOLLIN, EPOLL_CTL_ADD};
use std::os::unix::io::RawFd;

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
        let num_events = unsafe { epoll_wait(self.epoll_fd, events.as_mut_ptr(), max_events as i32, -1) };
        assert!(num_events >= 0);

        events.into_iter().take(num_events as usize).map(|e| e.u64 as RawFd).collect()
    }
}
