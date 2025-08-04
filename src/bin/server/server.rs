use battlesship::*;

use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
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
                println!("Connection closed");
            });
        }

        Ok(())
    }
}

fn handle_connection(mut stream: TcpStream) {
    println!("Client {} has connected", stream.peer_addr().unwrap().ip());
    let mut buff = [0; 64];
    let mut board = battlesship::Board
    loop {
        match stream.read(&mut buff) {
            Ok(num_bytes) => {
                let received: String = String::from_utf8_lossy(&buff[..num_bytes]).into_owned(); // Copy the buffer into a string
                let received_words = received.trim().split(' ').collect::<Vec<_>>();
                for w in &received_words {
                    println!("{w}")
                }
                match *received_words.get(0).unwrap() {
                    "SHOOT" => {
                        let coords = received_words.get(1).unwrap().chars().collect::<Vec<_>>();
                        let x = coords[0] as u8 - 97;
                        let y = coords[1] as u8 - 49;
                        match my_game.p1_board.shoot(x, y) {
                            game::ShotResult::Hit => stream.write(format!("HIT {}\n", received_words[1]).as_bytes()).unwrap(),
                            game::ShotResult::Miss => stream.write("MISS\n".as_bytes()).unwrap(),
                            game::ShotResult::Invalid => stream.write("BADCOORDS\n".as_bytes()).unwrap(),
                            game::ShotResult::Sunk(ship_type) => stream.write(format!("SUNK {ship_type}\n").as_bytes()).unwrap(),
                        };
                    }
                    "FORFEIT" => {
                        break
                    }
                    _ => {
                        stream.write("BADCOMMAND\n".as_bytes()).unwrap();
                    }
                };
            },
            Err(e) => panic!("Failed to read {e}")
        }
    }
    stream.write("ok bye now\n".as_bytes()).unwrap();
}

