use std::net::TcpStream;
use std::io::{Write, Read};

const SERVER_IP: &str = "127.0.0.1";
const SERVER_PORT: usize = 8080;

fn main() {
    // Connect to TCP Server
    println!("Connection To Server");
    let mut stream = TcpStream::connect(format!("{}:{}", SERVER_IP, SERVER_PORT))
        .expect("Could Not Connect To Server");
    println!("Connection Success!");

    let mode = [1];
    stream.write(&mode).unwrap();
    stream.write_all(&mode).expect("couldn't send message")

}
