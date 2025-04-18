use crate::server::epoll::path_server;
use lazy_static::lazy_static;
use std::fs::File;
use std::io::{BufRead, BufReader};

lazy_static! {
    pub static ref PATH_SERVER : String = path_server();
}

// Example of config.txt
// 127.0.0.1:7878
// 127.0.0.1:7879 host.name
// 127.0.0.1 : 7878

// Reads and parses the config file

pub fn config_output(path : &str) -> Vec<(String ,u16 ,String)> {
    let mut output = Vec::new();

    let file = match File::open(path) {
        Ok(file) => file,
        Err(_) => {
            eprintln!("Error opening config file at {}", path);
            return output; //empty output
        }
    };

    let reader = BufReader::new(file);
    
    for line in reader.lines() {
        if let Ok(line) = line {
            let parts = parse_str(&line, ' ');

            if parts.len() == 1 {
                let part = parse_str(parts[0], ':');
                let (ip, port) = (part[0], part[1]);

                if let Ok(port) = port.parse::<u16>() {
                    output.push((ip.to_string(), port,"".to_string()));
                } else {
                    eprintln!("Error parsing port number");
                }
            } else if parts.len() == 2 {
                let part = parse_str(parts[0], ':');
                let (ip, port) = (part[0], part[1]);
                let domain_name = parts[1];
                
                if let Ok(port) = port.parse::<u16>() {
                    output.push((ip.to_string(), port, domain_name.to_string()));
                } else {
                    eprintln!("Error parsing port number {}", port);
                }
            }
        }
    }
    return output;
}
 
fn parse_str(input: &str, sep : char) -> Vec<&str> {
    input.split(sep).collect()
}