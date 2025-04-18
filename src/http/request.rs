// Parses raw HTTP into method, path, headers, etc
pub fn parse_request(request: &str) -> Option<(String, String, String, Option<String>)> { // option for the domain cause it is not always present
    let (mut method, mut path, mut version) = (String::new(), String::new(), String::new());
    let mut domain = None;

    let mut lines = request.lines();
    if let Some(line) = lines.next() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() == 3 {
            method = parts[0].to_string();
            path = parts[1].to_string();
            version = parts[2].to_string();
        }
    }
    for line in lines {
        if line.starts_with("Host:") {
            domain = Some(line[5..].trim().to_string());
            break;
        }
    }
    if !method.is_empty() && !path.is_empty() && !version.is_empty() {
        Some((method, path, version, domain))
    } else {
        return None;
    }    
}
