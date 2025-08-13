mod api;
mod connection;
mod game;

use connection::Clients;

#[tokio::main]
async fn main() {
    let ip = [127, 0, 0, 1];
    let port = 3000;
    start_server(ip, port).await;
}

async fn start_server(ip: [u8; 4], port: u16) {
    let clients = Clients::default();
    warp::serve(api::routes(clients)).run((ip, port)).await;
}