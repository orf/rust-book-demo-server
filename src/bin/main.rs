use std::io::prelude::*;
use std::net::TcpStream;
use std::net::TcpListener;

extern crate rust_web_server;

use rust_web_server::ThreadPool;

extern crate num_cpus;

static TEMPLATE: &[u8] = include_bytes!("../template.html");


fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    let pool = ThreadPool::new(num_cpus::get() * 2);
    println!("Module path {} listening, {}", module_path!(), file!());
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(|| {
            handle_connection(stream)
        });
    }
}


fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    let response = "HTTP/1.1 200 OK\r\n\r\n".as_bytes();
    stream.write(response).unwrap();
    stream.write(TEMPLATE).unwrap();

    stream.flush().unwrap();
}