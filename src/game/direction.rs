#[derive(Clone, Debug, PartialEq)]
pub enum Direction {
    North,
    South,
    East,
    West,
    Stop,
}

impl Direction {
    pub fn left(&self) -> Self {
        match self {
            Direction::North => Direction::West,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
            Direction::West => Direction::South,
            Direction::Stop => Direction::Stop,
        }
    }

    pub fn right(&self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::South => Direction::West,
            Direction::East => Direction::South,
            Direction::West => Direction::North,
            Direction::Stop => Direction::Stop,
        }
    }

    pub fn reverse(&self) -> Self {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
            Direction::Stop => Direction::Stop,
        }
    }

    pub fn from_diff(dx: isize, dy: isize) -> Self {
        if dy > 0 {
            Direction::North
        } else if dy < 0 {
            Direction::South
        } else if dx < 0 {
            Direction::West
        } else if dx > 0 {
            Direction::East
        } else {
            Direction::Stop
        }
    }

    pub fn to_diff(&self, speed: usize) -> (isize, isize) {
        let speed = speed as isize;
        match self {
            Direction::North => (0, speed),
            Direction::South => (0, -speed),
            Direction::East => (speed, 0),
            Direction::West => (-speed, 0),
            Direction::Stop => (0, 0),
        }
    }

    pub fn values() -> Vec<Self> {
        vec![
            Direction::North,
            Direction::South,
            Direction::East,
            Direction::West,
            Direction::Stop,
        ]
    }
}
