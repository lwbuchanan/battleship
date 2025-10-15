> [!NOTE]
> This is a work in progress

# effnine
Effnine (pronounced like f9) is a rust implementation of the classic strategy game Battleship. It uses a client / authoritative server model to prevent cheating. The server uses a REST API for stats and matchmaking and websockets for running the matches.

## Todo
### Development
- Implement common game model (game rules / board representation)
    - The client and server will both use this for handling game state
 
- Implement matchmaking api
    - Request random match
    - Create private match
    - Join private match
 
- Implement gameplay protocol
    - Connect both player websockets
    - Handle all game actions
    - Report winning player
 
- Implement TUI client
    - Show board state
    - Menu for choosing match
    - Support for anonymous sessions

### Deploy
- Host an instance of the game server

### Ideas
- In game chat
    - Chat / banter with opponent which playing
 
- Match spectators
  
- Remote clients
    - Users can ssh into the game server and spawn a client rather than downloading the client binary

- Returning user identification
    - Set nickname
    - Store lifetime gameplay stats
 
- AI opponents
    - Play against different models to see which one is the best
