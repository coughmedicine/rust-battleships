use std::collections::{HashMap, HashSet};

use thiserror::Error;

use crate::location::Location;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Ship {
    coords: Vec<Location>,
    found: HashMap<Location, bool>,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub enum ShipDir {
    Horz,
    Vert,
}

impl Ship {
    pub fn new(start: Location, dir: ShipDir, len: i32) -> Self {
        let mut coords = vec![];
        for o in 0..len {
            let new = match dir {
                ShipDir::Horz => Location::new(start.x + o, start.y),
                ShipDir::Vert => Location::new(start.x, start.y + o),
            };
            coords.push(new);
        }

        let mut found = HashMap::new();
        for &c in &coords {
            found.insert(c, false);
        }

        Self { coords, found }
    }

    pub fn remove(&mut self, coord: Location) {
        //found this online, dont understand it but seems to work?
        let index = self.coords.iter().position(|&r| r == coord).unwrap();
        self.coords.remove(index);
    }

    pub fn guess(&mut self, coord: Location) -> bool {
        if self.coords.contains(&coord) {
            self.found.insert(coord, true);
            true
        } else {
            false
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
pub enum AddShipError {
    #[error("ship is out of bounds")]
    ShipOutOfBounds,
    #[error("ship overlaps an existing ship")]
    ShipOverlap,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Grid {
    pub ships: Vec<Ship>,
    pub size: i32,
}
impl Grid {
    pub fn new(size: i32) -> Self {
        Self {
            ships: vec![],
            size,
        }
    }

    pub fn add_ship(&mut self, ship: Ship) -> Result<(), AddShipError> {
        let coord_oob = |coord: Location| {
            coord.x >= self.size || coord.x < 0 || coord.y >= self.size || coord.y < 0
        };

        let existing_coords: HashSet<Location> = self
            .ships
            .iter()
            .flat_map(|s| s.coords.iter())
            .copied()
            .collect();

        let new_coords: HashSet<Location> = ship.coords.iter().copied().collect();

        if !existing_coords.is_disjoint(&new_coords) {
            return Err(AddShipError::ShipOverlap);
        }

        for c in &ship.coords {
            if coord_oob(*c) {
                return Err(AddShipError::ShipOutOfBounds);
            }
        }

        self.ships.push(ship);
        Ok(())
    }

    pub fn get_all(&self) -> Vec<Location> {
        let mut x = vec![];
        for s in &self.ships {
            for &c in &s.coords {
                x.push(c);
            }
        }
        x
    }

    pub fn get_all_found(&self) -> Vec<Location> {
        let mut x = vec![];
        for s in &self.ships {
            for (&l, &b) in &s.found {
                if b {
                    x.push(l);
                }
            }
        }
        x
    }

    pub fn check_loss(&self) -> bool {
        self.get_all().len() == self.get_all_found().len()
    }

    pub fn guess_grid(&mut self, coords: Location) -> bool {
        for i in 0..self.ships.len() {
            if self.ships[i].guess(coords) {
                return true;
            }
        }
        false
    }

    pub fn get_display(&self, see_unfound: bool) -> GridDisplay {
        GridDisplay {
            grid: self,
            see_unfound,
        }
    }
}

pub struct GridDisplay<'g> {
    grid: &'g Grid,
    see_unfound: bool,
}

impl<'g> std::fmt::Display for GridDisplay<'g> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let all = self.grid.get_all();
        let found = self.grid.get_all_found();
        write!(f, "  ")?;
        for x in 1..self.grid.size + 1 {
            write!(f, "{x} ")?;
        }
        writeln!(f)?;

        for y in 0..self.grid.size {
            write!(f, "{} ", char::from_u32('A' as u32 + y as u32).unwrap())?;
            for x in 0..self.grid.size {
                let cur = Location::new(x, y);
                if found.contains(&cur) {
                    write!(f, "x ")?;
                } else if self.see_unfound && all.contains(&cur) {
                    write!(f, "o ")?;
                } else {
                    write!(f, ". ")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use maplit::*;

    fn ship_horz_3() -> Ship {
        Ship {
            coords: vec![
                Location { x: 0, y: 0 },
                Location { x: 1, y: 0 },
                Location { x: 2, y: 0 },
            ],
            found: hashmap! {
                Location { x: 0, y: 0 } => false,
                Location { x: 1, y: 0 } => false,
                Location { x: 2, y: 0 } => false,
            },
        }
    }
    fn ship_vert_4() -> Ship {
        Ship {
            coords: vec![
                Location { x: 1, y: 1 },
                Location { x: 1, y: 2 },
                Location { x: 1, y: 3 },
                Location { x: 1, y: 4 },
            ],
            found: hashmap! {
                Location { x: 1, y: 1 } => false,
                Location { x: 1, y: 2 } => false,
                Location { x: 1, y: 3 } => false,
                Location { x: 1, y: 4 } => false,
            },
        }
    }

    #[test]
    fn test_ship_new() {
        assert_eq!(
            Ship::new(Location { x: 0, y: 0 }, ShipDir::Horz, 3),
            ship_horz_3()
        );
        assert_eq!(
            Ship::new(Location { x: 1, y: 1 }, ShipDir::Vert, 4),
            ship_vert_4()
        );
    }

    #[test]
    fn test_ship_guess() {
        {
            let mut s = ship_horz_3();
            assert!(s.guess(Location { x: 0, y: 0 }));
            assert_eq!(
                s,
                Ship {
                    coords: vec![
                        Location { x: 0, y: 0 },
                        Location { x: 1, y: 0 },
                        Location { x: 2, y: 0 },
                    ],
                    found: hashmap! {
                        Location { x: 0, y: 0 } => true,
                        Location { x: 1, y: 0 } => false,
                        Location { x: 2, y: 0 } => false,
                    },
                }
            );
        }
        {
            let mut s = ship_horz_3();
            assert!(!s.guess(Location { x: 10, y: 10 }));
            assert_eq!(s, ship_horz_3());
        }
    }
}
