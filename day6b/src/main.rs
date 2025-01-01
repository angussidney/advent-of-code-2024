use std::collections::HashSet;
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

const START: char = '^';
const OBSTACLE: char = '#';
const BLANK: char = '.';


fn search_for_loop(grid: &Grid<char>, x: usize, y: usize, dir: Dir, path: &mut Vec<(usize, usize, Dir)>) -> bool {
    let mut curr_xoff = 0;
    let mut curr_yoff = 0;
    let mut curr_dir = dir;

    loop {
        // Do something about the current location before we start thinking about the next square
        // TODO: add to visited list
        let curr_x = x.checked_add_signed(curr_xoff).unwrap();
        let curr_y = y.checked_add_signed(curr_yoff).unwrap();
        let current_state = (curr_x, curr_y, curr_dir);
        if path.contains(&current_state) {
            // Found a loop
            return true;
        } else {
            path.push(current_state);
        }

        let next_xoff = curr_xoff + curr_dir.off().0;
        let next_yoff = curr_yoff + curr_dir.off().1;
        if let Some(next) = grid.get_off(x, y, next_xoff, next_yoff) {
            if next == OBSTACLE {
                curr_dir = curr_dir.next();
                continue; // Need to recheck the next dir
            } else {
                // Otherwise continue as normal
                curr_xoff = next_xoff;
                curr_yoff = next_yoff;
            }
        } else {
            // This is the last square we will walk
            return false;
        }
    }
}

fn main() {
    const INPUT_FILE: &str = "data/input.txt";

    let mut grid: Grid<char> = Grid::<char>::from_file(INPUT_FILE);

    let mut start = None;
    for ((x, y), val) in &grid {
        if val == START {
            start = Some((x, y));
        }
    }

    let start_x = start.expect("Start not found").0;
    let start_y = start.expect("Start not found").1;

    let mut path = Vec::new();
    search_for_loop(&grid, start_x, start_y, Dir::Up, &mut path);

    let mut loops = Vec::new();
    for (x, y, dir) in path {
        let xoff = dir.off().0;
        let yoff = dir.off().1;
        if let Some(next_off) = grid.get_off(x, y, xoff, yoff) {
            if next_off != START {
                grid.set_off_val(x, y, xoff, yoff, OBSTACLE);
                if search_for_loop(&grid, start_x, start_y, Dir::Up, &mut Vec::new()) {
                    let (a, b) = grid.off(x, y, xoff, yoff).unwrap();
                    loops.push((a, b));
                }
                grid.set_off_val(x, y, xoff, yoff, next_off);
            }
        }
    }

    println!("{}", loops.into_iter().collect::<HashSet<(usize, usize)>>().len());
}


// 181 is too low