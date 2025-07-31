# Battleship
Battleship over tcp


## Protocol
#### Game details
- _coord_ - The coordinates of a location on the map
    - [A-I][1-9]
- _pos_ - The position and orientation of a ship 
    - [A-I][1-9]{H, V}
    - The ship extends down or right based on orientation
- _ship_ - The type of a ship
    - {CAR, BAT, DES, SUB, PAT}

#### Client commands
These are commands that the client can send to the server
- PLACE._ship_._pos_ - Place a given ship at a given position
- SHOOT._coord_ - Fires a shot at coordinate location on the opponents map
- FORFEIT - Client looses game instantly, oppononet wins
- WHOSTURN - Replies with current players turn
- WHOAMI - Replies with the role of the player
- CHAT.(message) - Sends a message to the other player

#### Server replys
Theses are commands that server can reply to the client with
- HIT._coord_ - A shot has landed on occupied location
- MISS._coord_ - A shot has landed on vacant location
- SUNK._ship_ - A ship of the given type has sunk
-  - The role of the client player
- GAMESTART - The game has started
- GAMEEND.{WIN, LOSS} - The game has ended
- BADPOS - A client command contains a location that is invalid for that particular action
- BADACTION - If a client command represents an action that cannot occur at this time, i.e. it is not the players turn, a game is not running, etc
- BADCOMMAND - If a client command can not be interpreted