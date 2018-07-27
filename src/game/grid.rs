pub struct Grid {
    width: usize,
    height: usize,
    data: Vec<bool>,
}

impl Clone for Grid {
    fn clone(&self) -> Self {
        Grid {
            width: self.width,
            height: self.height,
            data: self.data.clone(),
        }
    }
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Self {
        Grid::from_value(width, height, false)
    }

    pub fn from_value(width: usize, height: usize, value: bool) -> Self {
        Grid {
            width,
            height,
            data: vec![value; width * height],
        }
    }

    pub fn get(&self, i: usize, j: usize) -> Option<bool> {
        self.data.get(i + self.width * j).cloned()
    }

    pub fn set(&mut self, i: usize, j: usize, value: bool) {
        if let Some(old_value) = self.data.get_mut(i + self.width * j) {
            *old_value = value;
        }
    }

    pub fn count(&self) -> usize {
        self.data.iter().filter(|&&x| x).count()
    }

    pub fn shape(&self) -> (usize, usize) {
        (self.width, self.height)
    }
}
