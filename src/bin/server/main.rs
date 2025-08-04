mod server;

use crate::server::Server;

fn main() {
    let ip = "127.0.0.1";
    let port = "3000";
    Server::new(ip, port).run().unwrap();
}
