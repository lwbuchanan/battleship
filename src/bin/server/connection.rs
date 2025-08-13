use std::collections::HashMap;
use std::convert::Infallible;
use std::sync::Arc;

use futures::{FutureExt, StreamExt};
use tokio::sync::{mpsc, RwLock};
use tokio_stream::wrappers::UnboundedReceiverStream;
use warp::{
    Filter, 
    ws::{WebSocket, Message}
};

// Client object storing its owned user and send channel
#[derive(Clone)]
pub struct Client {
    pub user_id: usize,
    pub sender: Option<mpsc::UnboundedSender<Result<Message, warp::Error>>>,
}

// Thread safe, mutex locked map of clients
pub type Clients = Arc<RwLock<HashMap<String, Client>>>;

// Helper for passing client arc between threads
pub fn with_clients(clients: Clients) -> impl Filter<Extract = (Clients,), Error = Infallible> + Clone {
    warp::any().map(move || clients.clone())
}

pub async fn client_connection(ws: WebSocket, clients: Clients, mut client: Client, client_uuid: String) {
    // sends to ws, receives from ws
    let (ws_sink, mut ws_stream) = ws.split();

    // 
    let (client_sender, client_rcv) = mpsc::unbounded_channel();

    let client_rcv = UnboundedReceiverStream::new(client_rcv);
    tokio::task::spawn(client_rcv.forward(ws_sink).map(|result| {
        if let Err(e) = result {
            eprintln!("error sending websocket msg: {e}");
        }
    }));

    client.sender = Some(client_sender);
    clients.write().await.insert(client_uuid.clone(), client);

    println!("{client_uuid} connected");

    while let Some(result) = ws_stream.next().await {
        let msg = match result {
            Ok(msg) => msg,
            Err(e) => {
                eprintln!("error receiving ws message for id: {}): {}", client_uuid, e);
                break;
            }
        };
        client_msg(&client_uuid, msg, &clients).await;
    }

    clients.write().await.remove(&client_uuid);
    println!("{client_uuid} disconnected");
}

async fn client_msg(client_uuid: &str, msg: Message, clients: &Clients) {
    println!("received message from {client_uuid}: {msg:?}");
    let message = match msg.to_str() {
        Ok(v) => v,
        Err(_) => return,
    };

    if message == "ping" || message =="ping\n" {
        return;
    }
}


struct FindGameRequest {

}