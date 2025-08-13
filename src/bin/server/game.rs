use std::collections::HashMap;
use std::convert::Infallible;
use std::sync::Arc;

use uuid::Uuid;
use futures::{FutureExt, StreamExt};
use tokio::sync::{mpsc, RwLock};
use tokio_stream::wrappers::UnboundedReceiverStream;
use warp::{
    Filter, 
    ws::{WebSocket, Message}
};

use effnine::{
    self,
    Board,
};
use crate::connection::{
    self,
    Client,
};

pub struct Game {
    player1: PlayerContext,
    player2: Option<PlayerContext>,   
    state: GameState
}

pub type Games = Arc<RwLock<HashMap<String, Game>>>;

struct PlayerContext {
    client: Client,
    board: Board,
}
impl PlayerContext {
    fn new(client: Client) -> PlayerContext {
        PlayerContext {
            client,
            board: Board::new(),
        }
    }
}

enum GameState {
    Pending,
    Setup,
    P1Turn,
    P2Turn,
    GameOver,
}

// A client has requested to join a game
// If one is unavailible, start one and wait for an opponent
pub async fn handle_join_game(pending_games: Games, client: Client) {

    for game in pending_games.write().await.values_mut() {
        game.player2 = Some(PlayerContext::new(client));
        game.state = GameState::Pending;

        return
    }

    let game_id = Uuid::new_v4().simple().to_string();
    pending_games.write().await.insert(
        game_id,
        Game {
            player1: PlayerContext::new(client),
            player2: None,
            state: GameState::Pending,
        }
    );

}

async fn run_game() {
    todo!()
}