const BOARD_SIZE: usize = 10;

pub struct Game<'a> {
    pub state: GameState,
    pub p1_board: Board<'a>,
    pub p2_board: Board<'a>,
}

impl<'a> Game<'a> {
    pub fn new() -> Game<'a> {
        Game {
            state: GameState::Setup,
            p1_board: Board {
                grid: new_empty_grid(),
                fleet: [None, None, None, None, None],
                num_ships: 0,
            },
            p2_board: Board {
                grid: new_empty_grid(),
                fleet: [None, None, None, None, None],
                num_ships: 0,
            },
        }
    }
}

pub enum GameState {
    Setup,
    P1Turn,
    P2Turn,
    P1Win,
    P2Win,
}

pub struct Board<'a> {
    grid: Grid<'a>,
    fleet: [Option<Ship>; 5],
    num_ships: u8,
}

type Grid<'a> = [[Tile<'a>; BOARD_SIZE]; BOARD_SIZE];

fn new_empty_grid<'a>() -> Grid<'a> {
    [[Tile::Empty; BOARD_SIZE]; BOARD_SIZE]
}


impl<'a> Board<'a> {
    pub fn place_ship(&'a mut self, ship_type: ShipType, x: u8, y: u8, orientation: Orientation) -> bool {
        let length = match ship_type {
            ShipType::Carrier => 5,
            ShipType::Battleship => 4,
            ShipType::Destroyer => 3,
            ShipType::Submarine => 3,
            ShipType::Patrol => 2,
        };

        for i in 0 .. self.num_ships {
            if self.fleet[i as usize].as_ref().expect("Ship DNE").ship_type == ship_type {
                return false;
            }
        }

        match orientation {
            Orientation::Horizontal =>
                if x + length > BOARD_SIZE as u8 || y > BOARD_SIZE as u8 { return false }
            Orientation::Vertical =>
                if y + length > BOARD_SIZE as u8 || x > BOARD_SIZE as u8 { return false }
        };
        
        self.fleet[self.num_ships as usize] = Some(Ship {ship_type, health: length});
        for i in 0 .. length {
            match orientation {
                Orientation::Horizontal => {
                    self.grid[(x+i) as usize][y as usize] = Tile::Occupied(self.fleet[self.num_ships as usize].as_mut().expect("New ship doesn't exist somehow"));
                },
                Orientation::Vertical => {
                    self.grid[x as usize][(y+i) as usize] = Tile::Occupied(self.fleet[self.num_ships as usize].as_mut().expect("New ship doesn't exist somehow"));
                },
            }
        }
        true
    }

    pub fn shoot(&mut self, x: u8, y: u8) -> ShotResult {
        if x > 10 || y > 10 {
            ShotResult::Invalid
        } else {
            match &mut self.grid[x as usize][y as usize] {
                Tile::Occupied(ship) => {
                    ship.health -= 1;
                    ShotResult::Hit
                },
                Tile::Empty => {
                    ShotResult::Miss
                },
                _ => {
                    ShotResult::Invalid
                }
            }
        }
    }
}

enum Tile<'a> {
    Occupied(&'a mut Ship),  // A ship is here
    Empty,                   // Just ocean here
    Destroyed(&'a mut Ship), // A shot landed on a ship here
    Splash,                  // A shot landed in the ocean here
}

pub struct Ship {
    ship_type: ShipType,
    health: u8,
}

#[derive(Eq, PartialEq, Copy, Clone)]
pub enum ShipType {
    Carrier,
    Battleship,
    Destroyer,
    Submarine,
    Patrol,
}

pub enum Orientation {
    Horizontal,
    Vertical,
}

#[derive(Eq, PartialEq)]
pub enum ShotResult {
    Hit,
    Miss,
    Invalid,
    Sunk(ShipType),
}