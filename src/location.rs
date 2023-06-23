fn points_dist(a: i32, b: i32) -> i32 {
    (a - b).abs()
}

// Clone - allows .clone()
// Copy - allows implicitly copying without asking, requires Clone
// Debug - allows printing like {:?} (almost always)
// PartialEq - allows comparison like x == y
// Eq - just do it if you do PartialEq, requires PartialEq
// PartialOrd - allows comparison like x < y, x <= y, x > y, x >= y
// Ord - stronger PartialOrd, requires PartialOrd
// Hash - allows hashing, so you can use it as a dictionary key
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Location {
    pub x: i32,
    pub y: i32,
}

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
