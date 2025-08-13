use uuid::Uuid;
use warp::{
    http::StatusCode, 
    ws::Ws, 
    Filter, 
    Rejection, 
    Reply,
    reply, 
};

use crate::connection::{
    self, with_clients, Client, Clients
};

//
// Router
//

// Get all routes for the api with CORS support
pub fn routes(clients: Clients) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    health_route()
        .or(register_routes(clients.clone()))
        .or(play_route(clients.clone()))
        .with(warp::cors().allow_any_origin())
}


//
// GET /health
//

fn health_route() -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    warp::path!("health")
        .and_then(handle_health)
}

async fn handle_health() -> Result<impl Reply, Rejection> {
    Ok(StatusCode::OK)
}


//
// POST /register
// DELETE /register/{client_id}
//

fn register_routes(clients: Clients) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    let register = warp::path!("register")
        .and(warp::post())
        .and(warp::body::json())
        .and(with_clients(clients.clone()))
        .and_then(handle_register);

    let unregister = warp::path!("register" / String)
        .and(warp::delete())
        .and(with_clients(clients.clone()))
        .and_then(handle_unregister);

    register.or(unregister)
}

async fn handle_register(body: RegisterRequest, clients: Clients) -> Result<impl Reply, Rejection> {
    let user_id = body.user_id;
    let client_uuid = Uuid::new_v4().simple().to_string();

    clients.write().await.insert(
        client_uuid.clone(),
        Client {
            user_id,
            sender: None,
        }
    );

    Ok(reply::json(&RegisterResponse {
        uuid: client_uuid
    }))
}

async fn handle_unregister(client_uuid: String, clients: Clients) -> Result<impl Reply, Rejection> {
    clients.write().await.remove(&client_uuid);
    Ok(StatusCode::OK)
}

#[derive(serde::Deserialize, serde::Serialize)]
struct RegisterRequest {
    user_id: usize,
}

#[derive(serde::Deserialize, serde::Serialize)]
struct RegisterResponse {
    uuid: String,
}


//
// GET /play
//

fn play_route(clients: Clients) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    warp::path!("play" / String)
        .and(warp::ws())
        .and(with_clients(clients.clone()))
        .and_then(handle_play)
}

async fn handle_play(client_uuid: String, ws: Ws, clients: Clients) -> Result<impl Reply, Rejection> {
    let client = clients.write().await.get(&client_uuid).cloned();
    match client {
        Some(c) => Ok(ws.on_upgrade(move |socket| connection::client_connection(socket, clients, c, client_uuid))),
        None => Err(warp::reject::not_found()),
    }
}