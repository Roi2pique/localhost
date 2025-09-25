use crate::server::epoll::path_server;
use lazy_static::lazy_static;
use std::fs::File;
use std::io::{BufRead, BufReader};

lazy_static! {
    pub static ref PATH_SERVER: String = path_server();
}

// Example of config.txt
// 0.0.0.0:7980
// 172.16.2.212:7879 myserver.test mynewtest.test myynewexample.org
// 127.0.0.1:7981

// Reads and parses the config file

pub fn config_output(path: &str) -> Vec<(String, u16, Vec<String>)> {
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
                    output.push((ip.to_string(), port, Vec::new()));
                } else {
                    eprintln!("Error parsing port number");
                }
            } else if parts.len() > 1 {
                let part = parse_str(parts[0], ':');
                let (ip, port) = (part[0], part[1]);
                if let Ok(port) = port.parse::<u16>() {
                    // loop through all domains after index 0
                    let mut domains = Vec::new();
                    for domain in &parts[1..] {
                        domains.push(domain.to_string());
                    }
                    output.push((ip.to_string(), port, domains));
                } else {
                    eprintln!("Error parsing port number {}", port);
                }
            }
        }
    }
    return output;
}

fn parse_str(input: &str, sep: char) -> Vec<&str> {
    input.split(sep).collect()
}
