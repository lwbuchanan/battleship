use std::collections::HashMap;

const GRID_SIZE: usize = 10;

pub struct Board {
    grid: [[Tile; GRID_SIZE]; GRID_SIZE],
    fleet: HashMap<ShipType, Ship>,
}

impl Board {
    // Sets the location of a ship, moving it if a ship of this type exists already
    // false if ship could not be placed in the given location
    pub fn place_ship(
        &mut self,
        ship_type: ShipType,
        x: u8,
        y: u8,
        orientation: Orientation,
    ) -> bool {
        let length = ship_type.length();
        if !fits_in_grid(length, x, y, orientation) {
            return false;
        }

        match self.fleet.get(&ship_type) {
            Some(ship) => {
                let hori = ship.orientation == Orientation::Horizontal;
                for i in 0..length {
                    self.grid[(ship.x_pos + (i * hori as u8)) as usize]
                             [(ship.y_pos + (i * !hori as u8)) as usize] 
                        = Tile::Empty
                }
            }
            None => {
                self.fleet.insert(ship_type, Ship::new(ship_type, x, y, orientation));
            }
        }

        let hori = orientation == Orientation::Horizontal;
        for i in 0..length {
            self.grid[(x + (i * hori as u8)) as usize]
                     [(y + (i * !hori as u8)) as usize] 
                = Tile::Empty
        }
        true
    }

    pub fn shoot(&mut self, x: u8, y: u8) -> ShotResult {
        if x > GRID_SIZE as u8 || y > GRID_SIZE as u8 {
            ShotResult::Invalid
        } else {
            match &mut self.grid[x as usize][y as usize] {
                Tile::Occupied(ship_type) => {
                    match self.fleet.get_mut(ship_type) {
                        Some(ship) => {
                            ship.health -= 1;
                            ShotResult::Hit
                        },
                        None => ShotResult::Invalid,
                    }
                }
                Tile::Empty => ShotResult::Miss,
                _ => ShotResult::Invalid,
            }
        }
    }
}

pub enum ShotResult {
    Hit,
    Miss,
    Invalid,
}

enum Tile {
    Occupied(ShipType),
    Destroyed(ShipType),
    Empty,
    Splashed,
}

struct Ship {
    ship_type: ShipType,
    health: u8,
    x_pos: u8, 
    y_pos: u8,
    orientation: Orientation,
}

impl Ship {
    // Creates a new ship at the given location. Does not check if position is valid
    fn new(ship_type: ShipType, x: u8, y: u8, orientation: Orientation) -> Ship {
        Ship {
            ship_type,
            health: ship_type.length(),
            x_pos: x,
            y_pos: y,
            orientation,
        }
    }
}

// Checks to see if a ship with the given length and location will fit into the grid
fn fits_in_grid(length: u8, x: u8, y: u8, orientation: Orientation) -> bool {
    let (xmax, ymax) = match orientation {
        Orientation::Horizontal => (x + length - 1, y),
        Orientation::Vertical => (x, y + length - 1),
    };
    x < GRID_SIZE as u8 && y < GRID_SIZE as u8 && xmax < GRID_SIZE as u8 && ymax < GRID_SIZE as u8
}

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
pub enum ShipType {
    Carrier,
    Battleship,
    Destroyer,
    Submarine,
    Patrol,
}

impl ShipType {
    fn length(&self) -> u8 {
        match self {
            ShipType::Carrier => 5,
            ShipType::Battleship => 4,
            ShipType::Destroyer => 3,
            ShipType::Submarine => 3,
            ShipType::Patrol => 2,
        }
    }
}

#[derive(Eq, PartialEq, Copy, Clone)]
pub enum Orientation {
    Horizontal,
    Vertical,
}
