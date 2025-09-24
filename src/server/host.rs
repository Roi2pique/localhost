use std::process::Command;

#[derive(Debug)]
enum UpdateStatus {
    HostsAlreadyOk,
    HostsUpdated,
    HostsFailed,
    ConfigAlreadyOk,
    ConfigUpdated,
    Unknown(String),
}

pub fn update_hosts() {
    let output = Command::new("sudo")
        .arg("ressources/update_hosts.sh")
        .output()
        .expect("failed to execute process");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    if !stderr.trim().is_empty() {
        eprintln!("Script stderr: {}", stderr);
    }

    let mut statuses = Vec::new();

    for line in stdout.lines().map(str::trim).filter(|l| !l.is_empty()) {
        let status = match line {
            "ALREADY_OK" => UpdateStatus::HostsAlreadyOk,
            "UPDATED" => UpdateStatus::HostsUpdated,
            "FAILED" => UpdateStatus::HostsFailed,
            "CONFIG_ALREADY_OK" => UpdateStatus::ConfigAlreadyOk,
            "CONFIG_UPDATED" => UpdateStatus::ConfigUpdated,
            other => UpdateStatus::Unknown(other.to_string()),
        };
        statuses.push(status);
    }

    // Print results
    for status in &statuses {
        match status {
            UpdateStatus::HostsAlreadyOk => {
                println!("Hosts file already contains the correct mapping")
            }
            UpdateStatus::HostsUpdated => println!("Hosts file updated successfully"),
            UpdateStatus::HostsFailed => eprintln!("Failed to update hosts file"),
            UpdateStatus::ConfigAlreadyOk => {
                println!("Config file already contains the correct mapping")
            }
            UpdateStatus::ConfigUpdated => println!("Config file updated successfully"),
            UpdateStatus::Unknown(s) => eprintln!("Unknown script output: {}", s),
        }
    }
}

// pub fn update_hosts() {
//     let output = Command::new("sudo")
//         .arg("ressources/update_hosts.sh")
//         .output()
//         .expect("failed to execute process");

//     let stdout = String::from_utf8_lossy(&output.stdout);
//     // println!("Script output: {}", String::from_utf8_lossy(&output.stderr));
//     match stdout.trim() {
//         "ALREADY_OK" => println!("Hosts file already contains the correct mapping"),
//         // "WRONG_IP" => println!("Hosts file had wrong IP, replaced it"),
//         "UPDATED" => println!("Hosts file updated successfully"),
//         "FAILED" => eprintln!("Failed to update hosts file"),
//         other => eprintln!("Unknown script output: {}", other),
//     }
// }
