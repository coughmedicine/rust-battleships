fn points_dist(a: i32, b: i32) -> u32 {
    ((a as i64) - (b as i64)).abs() as u32
}

// Clone - allows .clone()
// Copy - allows implicitly copying without asking, requires Clone
// Debug - allows printing like {:?} (almost always)
// PartialEq - allows comparison like x == y
// Eq - just do it if you do PartialEq, requires PartialEq
// PartialOrd - allows comparison like x < y, x <= y, x > y, x >= y
// Ord - stronger PartialOrd, requires PartialOrd
// Hash - allows hashing, so you can use it as a dictionary key
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Deserialize, serde::Serialize,
)]
pub struct Location {
    pub x: i32,
    pub y: i32,
}

#[derive(
    Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Deserialize, serde::Serialize,
)]
pub enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    pub fn from_input(input: &str) -> Option<Dir> {
        match &*input.to_uppercase() {
            "DOWN" | "D" => Some(Self::Down),
            "UP" | "U" => Some(Self::Up),
            "LEFT" | "L" => Some(Self::Left),
            "RIGHT" | "R" => Some(Self::Right),
            _ => None,
        }
    }
}

impl Location {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
    pub fn move_dir(&mut self, dir: Dir) {
        match dir {
            Dir::Up => {
                self.y += 1;
            }
            Dir::Down => {
                self.y -= 1;
            }
            Dir::Left => {
                self.x -= 1;
            }
            Dir::Right => {
                self.x += 1;
            }
        }
    }

    pub fn get_distance(&self, other: Location) -> f32 {
        let xdist = points_dist(self.x, other.x);
        let ydist = points_dist(self.y, other.y);
        let temp = (xdist.pow(2) + ydist.pow(2)) as f32;
        temp.sqrt()
    }

    #[allow(dead_code)]
    pub fn check_neighbour(&self, other: Location) -> bool {
        !(points_dist(self.x, other.x) > 1 && points_dist(self.y, other.y) > 1)
    }
}

impl std::fmt::Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl std::ops::Sub for Location {
    type Output = Location;

    fn sub(self, rhs: Self) -> Self::Output {
        let x = self.x - rhs.x;
        let y = self.y - rhs.y;
        Location::new(x, y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_points_dist() {
        assert_eq!(points_dist(0, 1), 1);
        assert_eq!(points_dist(1, 0), 1);
        assert_eq!(points_dist(-5, 3), 8);
        assert_eq!(points_dist(-6, -8), 2);
        assert_eq!(points_dist(i32::MAX, i32::MIN), 4294967295);
    }

    #[test]
    fn test_dir_from_input() {
        assert_eq!(Dir::from_input("UP"), Some(Dir::Up));
        assert_eq!(Dir::from_input("U"), Some(Dir::Up));
        assert_eq!(Dir::from_input("Up"), Some(Dir::Up));
        assert_eq!(Dir::from_input("up"), Some(Dir::Up));

        assert_eq!(Dir::from_input("DOWN"), Some(Dir::Down));
        assert_eq!(Dir::from_input("D"), Some(Dir::Down));
        assert_eq!(Dir::from_input("Down"), Some(Dir::Down));
        assert_eq!(Dir::from_input("down"), Some(Dir::Down));

        assert_eq!(Dir::from_input("LEFT"), Some(Dir::Left));
        assert_eq!(Dir::from_input("L"), Some(Dir::Left));
        assert_eq!(Dir::from_input("Left"), Some(Dir::Left));
        assert_eq!(Dir::from_input("left"), Some(Dir::Left));

        assert_eq!(Dir::from_input("RIGHT"), Some(Dir::Right));
        assert_eq!(Dir::from_input("R"), Some(Dir::Right));
        assert_eq!(Dir::from_input("Right"), Some(Dir::Right));
        assert_eq!(Dir::from_input("right"), Some(Dir::Right));

        assert_eq!(Dir::from_input("a string"), None);
    }

    #[test]
    fn test_location_new() {
        assert_eq!(Location::new(0, 0), Location { x: 0, y: 0 });
        assert_eq!(Location::new(1, 3), Location { x: 1, y: 3 });
        assert_eq!(Location::new(-1, 1), Location { x: -1, y: 1 });
    }

    #[test]
    fn test_location_move_dir() {
        {
            let mut loc = Location { x: 2, y: 5 };
            loc.move_dir(Dir::Up);
            assert_eq!(loc, Location { x: 2, y: 6 });
        }
        {
            let mut loc = Location { x: 2, y: -5 };
            loc.move_dir(Dir::Up);
            assert_eq!(loc, Location { x: 2, y: -4 });
        }
        {
            let mut loc = Location { x: 2, y: 5 };
            loc.move_dir(Dir::Down);
            assert_eq!(loc, Location { x: 2, y: 4 });
        }
        {
            let mut loc = Location { x: 2, y: -5 };
            loc.move_dir(Dir::Down);
            assert_eq!(loc, Location { x: 2, y: -6 });
        }
        {
            let mut loc = Location { x: 2, y: 5 };
            loc.move_dir(Dir::Left);
            assert_eq!(loc, Location { x: 1, y: 5 });
        }
        {
            let mut loc = Location { x: -2, y: 5 };
            loc.move_dir(Dir::Left);
            assert_eq!(loc, Location { x: -3, y: 5 });
        }
        {
            let mut loc = Location { x: 2, y: 5 };
            loc.move_dir(Dir::Right);
            assert_eq!(loc, Location { x: 3, y: 5 });
        }
        {
            let mut loc = Location { x: -2, y: 5 };
            loc.move_dir(Dir::Right);
            assert_eq!(loc, Location { x: -1, y: 5 });
        }
    }

    #[test]
    fn test_location_get_dist() {
        {
            let loc1 = Location { x: 0, y: 0 };
            let loc2 = Location { x: 0, y: 0 };
            assert_eq!(loc1.get_distance(loc2), 0.0);
        }
        {
            let loc1 = Location { x: 1, y: 2 };
            let loc2 = Location { x: 3, y: 4 };
            assert_eq!(loc1.get_distance(loc2), 2.82842712475);
        }
        {
            let loc1 = Location { x: -1, y: 2 };
            let loc2 = Location { x: 3, y: -4 };
            assert_eq!(loc1.get_distance(loc2), 7.21110255093);
        }
    }

    #[test]
    fn test_location_check_neighbour() {
        {
            let loc1 = Location { x: 1, y: 1 };
            let loc2 = Location { x: 1, y: 2 };
            assert!(loc1.check_neighbour(loc2));
        }
        {
            let loc1 = Location { x: 1, y: 1 };
            let loc2 = Location { x: 2, y: 2 };
            assert!(loc1.check_neighbour(loc2));
        }
        {
            let loc1 = Location { x: 1, y: 1 };
            let loc2 = Location { x: 5, y: 5 };
            assert!(!(loc1.check_neighbour(loc2)));
        }
    }

    #[test]
    fn test_location_display() {
        assert_eq!(format!("{}", Location { x: 2, y: 5 }), "(2, 5)");
        assert_eq!(format!("{}", Location { x: -2, y: 5 }), "(-2, 5)");
        assert_eq!(format!("{}", Location { x: 8, y: -1 }), "(8, -1)");
        assert_eq!(format!("{}", Location { x: -120, y: -800 }), "(-120, -800)");
    }
}
