mod connect;
mod handle;
mod config;

use crate::connect::epoll;
use log::{info, error};
use env_logger;
use config::config::*;
use connect::epoll::run_epoll;
use std::{
    net::TcpListener,
    process::exit,
};

fn main() {
    env_logger::init();
    info!("Starting server ?");
    let config_path = format!("{}/src/config/config.txt", *PATH_SERVER);
    let configs = config_output(config_path.as_str());
    let mut listeners = Vec::new(); 

    for (ip , port ,domain_name) in configs {
        let addr = format!("{}:{}", ip, port);
        
        match TcpListener::bind(&addr) {
            Ok(listener) => {
                if domain_name.is_empty() {
                    info!("Domain name empty listening on http://{}", addr);
                } else {
                    info!("Listening on http://{} for the domain http://{}:{}", addr, domain_name, port);
                }
                listeners.push(listener);
            }
            Err(e) => {
                error!("Error binding to address {}: {}", port, e);
            }
        }
    }
    if listeners.is_empty() {   
        eprintln!("No listeners found after parsing config file");
        exit(1);
    } else {
        run_epoll(listeners);
    }    
}

//fn exec_sudo() {}