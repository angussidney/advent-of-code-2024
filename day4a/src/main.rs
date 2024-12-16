use std::fs;

#[derive(Clone, Copy)]
struct XY {
    x: i32,
    y: i32,
}

// TODO: play around with the idea of a direction enum
const OFFSETS: [[XY; 4]; 8] = [
    [XY {x: 0, y: 0}, XY {x: 1, y: 0}, XY {x: 2, y: 0}, XY {x: 3, y: 0}], // Right
    [XY {x: 0, y: 0}, XY {x:-1, y: 0}, XY {x:-2, y: 0}, XY {x:-3, y: 0}], // Left
    [XY {x: 0, y: 0}, XY {x: 0, y: 1}, XY {x: 0, y: 2}, XY {x: 0, y: 3}], // Down
    [XY {x: 0, y: 0}, XY {x: 0, y:-1}, XY {x: 0, y:-2}, XY {x: 0, y:-3}], // Up
    [XY {x: 0, y: 0}, XY {x: 1, y: 1}, XY {x: 2, y: 2}, XY {x: 3, y: 3}], // RD
    [XY {x: 0, y: 0}, XY {x:-1, y:-1}, XY {x:-2, y:-2}, XY {x:-3, y:-3}], // LU
    [XY {x: 0, y: 0}, XY {x: 1, y:-1}, XY {x: 2, y:-2}, XY {x: 3, y:-3}], // RU
    [XY {x: 0, y: 0}, XY {x:-1, y: 1}, XY {x:-2, y: 2}, XY {x:-3, y: 3}], // LD
];

fn get_square(grid: &Vec<Vec<char>>, x: usize, y: usize, xoff: i32, yoff: i32) -> Option<char> {
    let newx;
    if xoff < 0 {
        newx = x.checked_sub(xoff.abs() as usize);
    } else {
        newx = x.checked_add(xoff as usize);
    }

    let newy;
    if yoff < 0 {
        newy = y.checked_sub(yoff.abs() as usize);
    } else {
        newy = y.checked_add(yoff as usize);
    }

    newx.and_then(|x| {
        newy.and_then(|y| {
            grid.get(x).and_then(|row| row.get(y))
        })
    }).copied()

}

fn get_word(grid: &Vec<Vec<char>>, x: usize, y: usize, dir: usize) -> Option<String> {
    let mut word = String::new();
    for offs in OFFSETS[dir] {
        let char = get_square(grid, x, y, offs.x, offs.y);
        match char {
            Some(c) => word.push(c),
            None => return None
        }
    }
    Some(word)
}

fn main() {
    const INPUT_FILE: &str = "data/sample.txt";

    let file_contents = fs::read_to_string(INPUT_FILE).expect("Should read input file");

    // Parse input into a useful format
    let grid: Vec<Vec<char>> = file_contents
        .lines()
        .map(|row| row.chars().collect())
        .collect();

    let mut total = 0;
    for (y, row) in grid.iter().enumerate() {
        for x in 0..row.len() {
            for dir in 0..OFFSETS.len() {
                if let Some(word) = get_word(&grid, x, y, dir) {
                    if word.eq("XMAS") {
                        total += 1;
                    }
                }
            }
        } 
    }

    println!("{total}");
}
