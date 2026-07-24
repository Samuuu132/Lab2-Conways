pub struct GameOfLife {
    pub width: usize,
    pub height: usize,
    cells: Vec<bool>,
}

impl GameOfLife {
    pub fn new(width: usize, height: usize) -> Self {
        GameOfLife {
            width,
            height,
            cells: vec![false; width * height],
        }
    }

    fn index(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    pub fn is_alive(&self, x: usize, y: usize) -> bool {
        if x >= self.width || y >= self.height {
            return false;
        }
        self.cells[self.index(x, y)]
    }

    pub fn set_alive(&mut self, x: usize, y: usize, alive: bool) {
        if x < self.width && y < self.height {
            let idx = self.index(x, y);
            self.cells[idx] = alive;
        }
    }

    pub fn set_cells(&mut self, origin_x: usize, origin_y: usize, cells: &[(usize, usize)]) {
        for (dx, dy) in cells {
            self.set_alive(origin_x + dx, origin_y + dy, true);
        }
    }

    fn count_alive_neighbors(&self, x: usize, y: usize) -> u8 {
        let mut count = 0;

        for dy in -1i32..=1 {
            for dx in -1i32..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }

                let nx = x as i32 + dx;
                let ny = y as i32 + dy;

                if nx < 0 || ny < 0 {
                    continue;
                }

                let nx = nx as usize;
                let ny = ny as usize;

                if self.is_alive(nx, ny) {
                    count += 1;
                }
            }
        }

        count
    }

    pub fn step(&mut self) {
        let mut new_cells = self.cells.clone();

        for y in 0..self.height {
            for x in 0..self.width {
                let alive = self.is_alive(x, y);
                let neighbors = self.count_alive_neighbors(x, y);

                let will_be_alive = match (alive, neighbors) {
                    (true, n) if n < 2 => false,
                    (true, 2) | (true, 3) => true,
                    (true, n) if n > 3 => false,
                    (false, 3) => true,
                    (alive, _) => alive,
                };

                new_cells[self.index(x, y)] = will_be_alive;
            }
        }

        self.cells = new_cells;
    }
}