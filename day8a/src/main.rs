use std::collections::{HashMap, HashSet};
use std::fs;
use std::fmt::Display;

struct Grid<T: Copy + Clone> {
    width: usize,
    height: usize,
    squares: Vec<T>,
}

impl<T: Copy + Clone + Display> Grid<T> {
    fn from_file(input_file: &str) -> Grid<char> {
        // TODO: read the file into a grid here
        let mut grid: Vec<char> = Vec::new();

        let file_str = fs::read_to_string(input_file)
            .expect("File not found");
    
        let lines = file_str.lines();

        let mut width = 0;
        let mut height = 0;
        for line in lines {
            if width == 0 {
                width = line.len();
            } else {
                assert!(width == line.len());
            }

            height += 1;

            grid.extend(line.chars());
        }
    
        Grid {
            width: width,
            height: height,
            squares: grid
        }
    }

    fn from_default_val<U: Copy>(width: usize, height: usize, val: U) -> Grid<U> {
        let mut grid = Vec::new();
        grid.resize(width * height, val);
        Grid {
            width: width,
            height: height,
            squares: grid,
        }
    }

    fn print_grid(&self) {
        for (idx, char) in self.squares.iter().enumerate() {
            print!("{char}");
            if idx % self.width == self.width - 1 {
                println!();
            }
        }
    }

    fn get(&self, x: usize, y: usize) -> Option<T> {
        if x >= self.width || y >= self.height {
            None
        } else {
            Some(self.squares[y * self.width + x])
        }
    }

    fn get_val(&self, x: usize, y: usize) -> T {
        self.get(x, y).expect("get_val(...) was given coords out of bounds")
    }

    fn set_val(&mut self, x: usize, y: usize, val: T) {
        if x >= self.width || y >= self.height {
            panic!("set_val({x},{y},...) is not within grid dimensions {}x{}", self.width, self.height);
        } else {
            self.squares[y * self.width + x] = val;
        }
    }

    fn off(&self, x: usize, y: usize, xoff: isize, yoff: isize) -> Option<(usize, usize)> {
        let newx = x.checked_add_signed(xoff)?;
        let newy = y.checked_add_signed(yoff)?;

        if newx >= self.width || newy >= self.height {
            None
        } else {
            Some((newx, newy))
        }
    }

    fn get_off(&self, x: usize, y: usize, xoff: isize, yoff: isize) -> Option<T> {
        let newx = x.checked_add_signed(xoff);
        let newy = y.checked_add_signed(yoff);
    
        self.get(newx?, newy?)
    }

    fn get_off_val(&self, x: usize, y: usize, xoff: isize, yoff: isize) -> T {
        self.get_off(x, y, xoff, yoff).expect("get_off_val(...) was given coords out of bounds")
    }

    fn set_off_val(&mut self, x: usize, y: usize, xoff: isize, yoff: isize, val: T) {
        let newx = x.checked_add_signed(xoff).expect("X offset overflowed");
        let newy = y.checked_add_signed(yoff).expect("Y offset overflowed");
    
        self.set_val(newx, newy, val);
    }
}

impl<'a, T: Copy + Clone> IntoIterator for &'a Grid<T> {
    type Item = ((usize, usize), T);

    type IntoIter = GridIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        GridIter {
            idx: 0,
            grid: self,
        }
    }
}

struct GridIter<'a, T: Copy + Clone> {
    idx: usize,
    grid: &'a Grid<T>,
}

impl<T: Copy + Clone> Iterator for GridIter<'_, T> {
    type Item = ((usize, usize), T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx >= self.grid.width * self.grid.height {
            return None
        }
        let ret = self.grid.squares[self.idx];
        let x = self.idx % self.grid.width;
        let y = self.idx / self.grid.width;
        self.idx += 1;
        Some(((x, y), ret))
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Dir {
    Up,
    Right,
    Down,
    Left,
}

impl Dir {
    fn off(&self) -> (isize, isize) {
        match self {
            Dir::Up => (0, -1),
            Dir::Right => (1, 0),
            Dir::Down => (0, 1),
            Dir::Left => (-1, 0),
        }
    }

    fn next(&self) -> Dir {
        match self {
            Dir::Up => Dir::Right,
            Dir::Right => Dir::Down,
            Dir::Down => Dir::Left,
            Dir::Left => Dir::Up,
        }
    }
}


const VALID: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";

fn main() {
    const INPUT_FILE: &str = "data/input.txt";

    let mut grid: Grid<char> = Grid::<char>::from_file(INPUT_FILE);
    let mut antinodes: HashSet<(usize, usize)> = HashSet::new();

    let mut letter_coords: HashMap<char, Vec<(usize, usize)>> = HashMap::new();

    for (coord, val) in grid.into_iter() {
        if !VALID.contains(val) {
            continue;
        }
        letter_coords
            .entry(val)
            .or_insert_with(|| Vec::new())
            .push(coord);
    }

    for (letter, coords) in letter_coords.iter() {
        for i in 0..coords.len() {
            for j in (i+1)..coords.len() {
                let (x1, y1) = coords[i];
                let (x2, y2) = coords[j];
                // TODO: Replace with checked_signed_diff once it comes out of nightly
                let xdiff = (x2 as isize) - (x1 as isize);
                let ydiff = (y2 as isize) - (y1 as isize);

                if let Some(coord) = grid.off(x1, y1, -xdiff, -ydiff) {
                    antinodes.insert(coord);
                }
                if let Some(coord) = grid.off(x2, y2, xdiff, ydiff) {
                    antinodes.insert(coord);
                }
            }
        }
    }

    println!("{}", antinodes.len());
}
