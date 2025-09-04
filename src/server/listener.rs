use log::{error, info};
use std::net::TcpListener;
use std::process::exit;

pub struct ListenerInfo {
    pub listener: TcpListener,
    pub _domain: Option<String>,
    pub _address: String, // Useful for debugging/logging
}

pub fn init_listeners(configs: Vec<(String, u16, String)>) -> Vec<ListenerInfo> {
    let mut listeners = Vec::new();

    for (ip, port, domain_name) in configs {
        let addr = format!("{}:{}", ip, port);

        match TcpListener::bind(&addr) {
            Ok(listener) => {
                if domain_name.is_empty() {
                    info!("Listening on http://{}", addr);
                    listeners.push(ListenerInfo {
                        listener,
                        _domain: None,
                        _address: addr,
                    });
                } else {
                    info!(
                        "Listening on http://{} for domain 'http://{}:{}'",
                        addr, domain_name, port
                    );
                    listeners.push(ListenerInfo {
                        listener,
                        _domain: Some(domain_name),
                        _address: addr,
                    });
                }
            }
            Err(e) => {
                error!("Failed to bind to {}: {}", addr, e);
            }
        }
    }

    if listeners.is_empty() {
        error!("No valid listeners were created.");
        exit(1);
    }

    listeners
}
