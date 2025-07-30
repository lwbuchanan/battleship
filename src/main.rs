use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};

fn main() {
    server();
}

fn handle_client(stream: TcpStream) {
    let mut buff = [0; 1024];
}

fn server() {
    let ip = "127.0.0.1";
    let port = "3000";

    let listener: TcpListener = TcpListener::bind(ip.to_owned() + ":" + port).unwrap();

    for stream in listener.incoming() {
        handle_client(stream.unwrap());
    }

}
