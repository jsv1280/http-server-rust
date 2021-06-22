use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;
use std::fs;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7373").unwrap();

    for stream in listener.incoming() {
    	let stream = stream.unwrap();
        handle_connection(stream);
	}
}

fn handle_connection(mut stream: TcpStream) {

    let mut buffer = [0; 256];
    let get = b"GET / HTTP/1.1\r\n";

    stream.read(&mut buffer).unwrap();

    if buffer.starts_with(get) {

        let contents = fs::read_to_string("index.html").unwrap();
        let response = format!("HTTP/1.1 200 OK\r\nContent-Length:{}\r\n\r\n{}",contents.len(), contents);

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
    else {

        let contents = fs::read_to_string("404.html").unwrap();
        let response = format!("HTTP/1.1 404 NOT FOUND\r\nContent-Length:{}\r\n\r\n{}",contents.len(), contents);
    
    	stream.write(response.as_bytes()).unwrap();
    	stream.flush().unwrap();
    }

    

}