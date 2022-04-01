use std::io::{Write, Read};
use std::net::{TcpListener,TcpStream};

fn main() {
    let listener=TcpListener::bind("127.0.0.1:3000").unwrap();

    println!("running on the 3000...");
    for stream in listener.incoming() {
        let mut stream=stream.unwrap();
        println!("connecting....");
        let mut buffer=[0;1024];
        stream.read(&mut buffer).unwrap();
        stream.write(&mut buffer).unwrap();
    }
}
