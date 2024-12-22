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

const DIRS: [(isize, isize); 4] = [
    ( 0,-1),
    ( 1, 0),
    ( 0, 1),
    (-1, 0),
];

const START: char = '^';
const OBSTACLE: char = '#';

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

    let mut xoff = 0;
    let mut yoff = 0;
    let mut curr_dir = 0;
    let mut visited = 0;
    loop {
        if grid.get_off_val(start_x, start_y, xoff, yoff) != 'X' {
            visited += 1;
        }
        grid.set_off_val(start_x, start_y, xoff, yoff, 'X');

        // grid.print_grid();
        // println!();

        let next_x = DIRS[curr_dir % 4].0;
        let next_y = DIRS[curr_dir % 4].1;
        if let Some(next) = grid.get_off(start_x, start_y, xoff + next_x, yoff + next_y) {
            if next == OBSTACLE {
                curr_dir += 1;
                continue; // Need to recheck the next dir
            } else {
                xoff = xoff + next_x;
                yoff = yoff + next_y;
            }
        } else {
            // This is the last square we will walk
            break;
        }
    }

    println!("{visited}");
}
