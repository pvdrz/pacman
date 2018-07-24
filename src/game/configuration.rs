use super::Direction;

pub struct Configuration {
    x: usize,
    y: usize,
    direction: Direction,
}

impl Configuration {
    pub fn new(x: usize, y: usize, direction: Direction) -> Self {
        Configuration { x, y, direction }
    }

    pub fn generate_successor(&self, dx: isize, dy: isize) -> Self {
        match Direction::from_diff(dx, dy) {
            Direction::Stop => {
                Configuration::new(self.x + dx as usize, self.y + dy as usize, self.direction)
            }
            direction => Configuration::new(self.x + dx as usize, self.y + dy as usize, direction),
        }
    }
}
