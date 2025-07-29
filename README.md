# 🌐 Rust HTTP / Localhost

A lightweight, HTTP/1.1-compliant web server built from scratch in Rust.

This project was developed to demonstrate how a real-world web server works under the hood — including socket management, epoll-based I/O multiplexing, request parsing, static content serving, and basic dynamic capabilities via CGI — all without relying on external web frameworks like `tokio` or `hyper`.

---

## 📌 Purpose

The main objectives of this server are:

- Serve static HTML files and resources (GET)
- Support file uploads and request bodies (POST)
- Allow resource deletion (DELETE)
- Correctly implement and respond to HTTP/1.1 requests
- Handle cookies, sessions, and error pages
- Safely manage routes, file access, and directory traversal
- Provide dynamic content using a CGI interface
- Bind multiple listeners (IP:PORT), with optional domain name mapping
- Use epoll for efficient single-threaded I/O

---

## 🗂️ Project Structure

ressources/
├── index.html
├── upload/
│ └── upload_files.txt
src/
├── main.rs # Entrypoint: loads config, starts server
├── config/ # Parses config.txt (listener and domain setup)
├── server/ # Binds sockets, runs epoll loop, handles connections
├── http/ # Request parsing, routing, and response building
│ └── methods/ # GET, POST, DELETE logic
├── cgi/ # Forks & runs CGI scripts based on extension
├── utils/ # Reusable helpers: path safety, cookies, MIME types
├── errors/ # Sends custom error responses (403, 404, etc.)

---

## ⚙️ Configuration

Located in: `etc/config.txt`

Each line defines a listener:
127.0.1.5:7980
127.0.0.2:7879 myserver.test
127.0.1.2:8080

📌 Optional domain name can be mapped per IP:PORT combo.

---

## 🔧 Usage

```bash
cargo build --release
cargo run
```

Or after building:

```bash
RUST_LOG=info cargo run
```

👨‍💻 Author
Created by Roi2pique as part of a systems programming curriculum.
Feel free to fork or contribute !
