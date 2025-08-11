mod server;

use crate::server::start_server;

#[tokio::main]
async fn main() {
    let ip = [127, 0, 0, 1];
    let port = 3000;
    start_server(ip, port).await;
}
