use game::Direction;

#[derive(Clone, Copy)]
pub struct Configuration {
    x: usize,
    y: usize,
    direction: Direction,
}

impl Configuration {
    pub fn new(x: usize, y: usize, direction: Direction) -> Self {
        Configuration { x, y, direction }
    }

    pub fn gen_successor(&self, dx: isize, dy: isize) -> Self {
        match Direction::from_diff(dx, dy) {
            Direction::Stop => {
                Configuration::new(self.x + dx as usize, self.y + dy as usize, self.direction)
            }
            direction => Configuration::new(
                (self.x as isize + dx) as usize,
                (self.y as isize + dy) as usize,
                direction,
            ),
        }
    }

    pub fn position(&self) -> (usize, usize) {
        (self.x, self.y)
    }

    pub fn direction(&self) -> Direction {
        self.direction
    }
}
