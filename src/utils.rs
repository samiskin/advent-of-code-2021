use std::fmt;
use std::collections::HashSet;

pub struct Grid<T> {
    grid: Vec<Vec<T>>,
}

impl<T> Grid<T> {
    pub fn new(grid: Vec<Vec<T>>) -> Grid<T> {
        Grid { grid }
    }
    pub fn get(&self, x: usize, y: usize) -> &T {
        &self.grid[y][x]
    }
    pub fn set(&mut self, x: usize, y: usize, val: T) {
        self.grid[y][x] = val;
    }
    pub fn width(&self) -> usize {
        self.grid[0].len()
    }
    pub fn height(&self) -> usize {
        self.grid.len()
    }
    pub fn size(&self) -> usize {
        self.width() * self.height()
    }
    pub fn neighbors(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut neighbors = vec![];
        if x > 0 {
            neighbors.push((x - 1, y))
        }
        if y > 0 {
            neighbors.push((x, y - 1))
        }
        if x < self.width() - 1 {
            neighbors.push((x + 1, y))
        }
        if y < self.height() - 1 {
            neighbors.push((x, y + 1))
        }
        return neighbors;
    }
    pub fn neighbors_diag(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut neighbors = self.neighbors(x, y);
        if x > 0 && y > 0 {
            neighbors.push((x - 1, y - 1))
        }
        if x < self.width() - 1 && y > 0 {
            neighbors.push((x + 1, y - 1))
        }
        if x < self.width() - 1 && y < self.height() - 1 {
            neighbors.push((x + 1, y + 1))
        }
        if x > 0 && y < self.height() - 1 {
            neighbors.push((x - 1, y + 1))
        }
        return neighbors;
    }
    pub fn iter(&self) -> GridIter<'_, T> {
        GridIter { grid: self, pos: 0 }
    }

    pub fn bfs_iter(&self, pos: (usize, usize), validator: fn((usize, usize), &T) -> bool) -> GridBfsIter<'_, T> {
        GridBfsIter { grid: self, to_visit: vec!(pos), seen: HashSet::new(), validator }
    }
}

impl<T> fmt::Debug for Grid<T> where T: fmt::Debug, {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.grid {
            write!(f, "\n{:?}", row).unwrap();
        }
        Ok(())
    }
}

pub struct GridIter<'a, T> {
    grid: &'a Grid<T>,
    pos: usize,
}

impl<'a, T> Iterator for GridIter<'a, T> {
    type Item = ((usize, usize), &'a T);
    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.grid.width() * self.grid.height() {
            return None;
        };
        let coord = (self.pos % self.grid.width(), self.pos / self.grid.width());
        self.pos += 1;
        return Some((coord, self.grid.get(coord.0, coord.1)));
    }
}

pub struct GridBfsIter<'a, T> {
    grid: &'a Grid<T>,
    validator: fn((usize, usize), &T) -> bool,
    to_visit: Vec<(usize, usize)>,
    seen: HashSet<(usize, usize)>,
}

impl<'a, T> Iterator for GridBfsIter<'a, T> where T: fmt::Debug, {
    type Item = ((usize, usize), &'a T);
    fn next(&mut self) -> Option<Self::Item> {
        if self.to_visit.is_empty() {
            return None;
        }
        let pos = self.to_visit.pop().unwrap();
        self.seen.insert(pos);
        for (nx, ny) in self.grid.neighbors(pos.0, pos.1) {
            if !self.seen.contains(&(nx, ny)) && (self.validator)(pos, self.grid.get(nx, ny)) {
                self.to_visit.insert(0, (nx, ny));
                self.seen.insert((nx, ny));
            }
        }

        return Some((pos, self.grid.get(pos.0, pos.1)));
    }
}
