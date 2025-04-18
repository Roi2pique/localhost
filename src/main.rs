mod server;
mod config;
mod http;
mod errors;


use log::{info, error};
use env_logger;
use config::loader::config_output;
use server::epoll::run_epoll;
use server::listener::{init_listeners, ListenerInfo};
use crate::config::loader::PATH_SERVER;


fn main() {
    env_logger::init();
    info!("Starting server...");

    let config_path = format!("{}/etc/config.txt", *PATH_SERVER);
    let configs = config_output(config_path.as_str());

    let listener_infos: Vec<ListenerInfo> = init_listeners(configs);
    let tcp_listeners = listener_infos.iter().map(|l| l.listener.try_clone().unwrap()).collect();

    run_epoll(tcp_listeners);
}

//fn exec_sudo() {}