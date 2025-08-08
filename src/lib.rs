use std::collections::HashMap;
use std::fmt;

const GRID_SIZE: usize = 10;

pub struct Board {
    grid: [[Tile; GRID_SIZE]; GRID_SIZE],
    fleet: HashMap<ShipType, Ship>,
}

impl Board {
    pub fn new() -> Board {
        Board {
            grid: [[Tile::Empty; GRID_SIZE]; GRID_SIZE],
            fleet: HashMap::new()
        }
    }
    // Sets the location of a ship, moving it if a ship of this type exists already
    // false if ship could not be placed in the given location
    pub fn place_ship(
        &mut self,
        ship_type: ShipType,
        row: u8,
        col: u8,
        orientation: Orientation,
    ) -> bool {
        let length = ship_type.length();
        if !fits_in_grid(length, row, col, orientation) {
            return false;
        }

        match self.fleet.get(&ship_type) {
            Some(ship) => {
                let hori = ship.orientation == Orientation::Horizontal;
                for i in 0..length {
                    self.grid[(ship.row + (i * !hori as u8)) as usize]
                             [(ship.col + (i * hori as u8)) as usize] 
                        = Tile::Empty
                }
            }
            None => {
                self.fleet.insert(ship_type, Ship::new(ship_type, row, col, orientation));
            }
        }

        let hori = orientation == Orientation::Horizontal;
        for i in 0..length {
            self.grid[(row + (i * !hori as u8)) as usize]
                     [(col + (i * hori as u8)) as usize] 
                = Tile::Occupied(ship_type)
        }
        true
    }

    pub fn shoot(&mut self, row: u8, col: u8) -> ShotResult {
        if row > GRID_SIZE as u8 || col > GRID_SIZE as u8 {
            ShotResult::Invalid
        } else {
            match &mut self.grid[row as usize][col as usize] {
                Tile::Occupied(ship_type) => {
                    match self.fleet.get_mut(ship_type) {
                        Some(ship) => {
                            ship.health -= 1;
                            self.grid[row as usize][col as usize] = Tile::Destroyed(ship_type.clone());
                            if ship.health <= 0 {
                                ShotResult::Sunk
                            } else {
                                ShotResult::Hit
                            }
                        },
                        None => ShotResult::Invalid,
                    }
                }
                Tile::Empty => {
                    self.grid[row as usize][col as usize] = Tile::Splashed;
                    ShotResult::Miss
                },
                _ => ShotResult::Invalid,
            }
        }
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut map = String::new(); 
        for row in self.grid {
            for tile in row {
                match tile {
                    Tile::Occupied(_) => map += "O ",
                    Tile::Destroyed(_) => map += "D ",
                    Tile::Empty => map += "~ ",
                    Tile::Splashed => map += "S ",
                }
            }
            map += "\n";
        }
        let mut fleet = String::new();
        for ship in self.fleet.values() {
            let ship_type = match ship.ship_type {
                ShipType::Carrier => "Carrier",
                ShipType::Battleship => "Battleship",
                ShipType::Destroyer => "Destroyer",
                ShipType::Submarine => "Submarine",
                ShipType::Patrol => "Patrol",
            };
            fleet += &format!("{}: {}", ship_type, ship.health);
        }
        write!(f, "{map}Ships: {fleet}")
    }
}

pub enum ShotResult {
    Hit,
    Miss,
    Sunk,
    Invalid,
}

#[derive(Eq, PartialEq, Copy, Clone)]
enum Tile {
    Occupied(ShipType),
    Destroyed(ShipType),
    Empty,
    Splashed,
}

struct Ship {
    ship_type: ShipType,
    health: u8,
    row: u8, 
    col: u8,
    orientation: Orientation,
}

impl Ship {
    // Creates a new ship at the given location. Does not check if position is valid
    fn new(ship_type: ShipType, row: u8, col: u8, orientation: Orientation) -> Ship {
        Ship {
            ship_type,
            health: ship_type.length(),
            row,
            col,
            orientation,
        }
    }
}

// Checks to see if a ship with the given length and location will fit into the grid
fn fits_in_grid(length: u8, row: u8, col: u8, orientation: Orientation) -> bool {
    let (rowmax, colmax) = match orientation {
        Orientation::Vertical => (row + length - 1, col),
        Orientation::Horizontal => (row, col + length - 1),
    };
    row < GRID_SIZE as u8 && col < GRID_SIZE as u8 && rowmax < GRID_SIZE as u8 && colmax < GRID_SIZE as u8
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

// #[cfg(test)]
// mod tests {
//     use super::*;
// }
