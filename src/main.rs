mod cgi_bin;
mod config;
mod errors;
mod http;
mod server;

use crate::config::loader::PATH_SERVER;
use crate::server::epoll::run_epoll;
use config::loader::config_output;
use env_logger;
use log::info;
use server::host::update_hosts;
use server::listener::{init_listeners, ListenerInfo};

fn main() {
    env_logger::init();
    info!("Starting server...");
    update_hosts(); //update the function host about config file change for multi domain support

    let config_path = format!("{}/etc/config.txt", *PATH_SERVER);
    let configs = config_output(config_path.as_str());
    println!("config : {:#?}", configs);
    let listener_infos: Vec<ListenerInfo> = init_listeners(configs);
    let tcp_listeners = listener_infos
        .iter()
        .map(|l| l.listener.try_clone().unwrap())
        .collect();

    run_epoll(tcp_listeners);
}
