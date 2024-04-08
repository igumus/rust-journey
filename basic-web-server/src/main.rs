use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    process, thread,
};

fn handle_request(mut stream: &TcpStream) {
    let mut buf = [0u8; 4096];
    match stream.read(&mut buf) {
        Ok(_) => {
            let req_str = String::from_utf8_lossy(&buf);
            println!("{}", req_str);
        }
        Err(e) => println!("Unable to read stream: {}", e),
    }
}

fn handle_response(mut stream: &TcpStream) {
    let response = b"HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n<html><body>Hello world</body></html>\r\n";
    match stream.write(response) {
        Ok(_) => println!("Response sent"),
        Err(e) => println!("Failed sending response: {}", e),
    }
}

fn handle_stream(stream: TcpStream) {
    handle_request(&stream);
    handle_response(&stream);
}

struct Server {
    listener: TcpListener,
}

impl Server {
    fn new() -> Self {
        let listener = TcpListener::bind("0.0.0.0:8080").unwrap();
        Self { listener }
    }

    fn run(&self) {
        for stream in self.listener.incoming() {
            match stream {
                Ok(stream) => {
                    thread::spawn(|| handle_stream(stream));
                }
                Err(err) => {
                    eprintln!("ERROR: unable to connect: {}", err);
                    process::exit(1);
                }
            }
        }
    }
}

fn main() {
    let server = Server::new();
    server.run();
}
