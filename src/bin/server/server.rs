use effnine::*;

use std::net::{TcpListener, TcpStream};
use std::io::{BufReader, prelude::*};
use std::io;
use std::thread;

pub struct Server {
    ip: String,
    port: String,
}

impl Server {
    pub fn new(ip: &str, port: &str) -> Server {
        Server {
            ip: ip.to_owned(),
            port: port.to_owned(),
        }
    }

    pub fn run(&self) -> io::Result<()> {
        let listener = TcpListener::bind(self.ip.to_owned() + ":" + &self.port)?;
        println!("TCP listener on port {}", &self.port);

        for connection in listener.incoming() {
            let stream = connection?;
            thread::spawn(|| {
                handle_connection(stream);
            });
        }

        Ok(())
    }
}

fn handle_connection(mut stream: TcpStream) {
    let reader = BufReader::new(&stream);
    let request = reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect::<Vec<_>>();
    

    let response = "HTTP/1.1 200 OK\r\n\r\n";
    stream.write_all(response.as_bytes()).unwrap();
}

// #[cfg(test)]
// mod tests {
//     use super::*;
// }

