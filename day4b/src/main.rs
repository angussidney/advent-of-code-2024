use std::fs;

#[derive(Clone, Copy)]
struct XY {
    x: i32,
    y: i32,
}

const STAR_OFF: [XY; 4] = [
    XY {x: 1, y: 1}, // RD
    XY {x: 1, y:-1}, // RU
    XY {x:-1, y:-1}, // LU
    XY {x:-1, y: 1}, // LD
];

// check if M count is 2 and S count is 2
const INVALID_STARS: [&str; 2] = [
    "MSMS",
    "SMSM",
];

fn count_letters(letters: &str) -> bool {
    let mut m = 0;
    let mut s = 0;
    for l in letters.chars() {
        if l == 'M' {
            m += 1;
        }
        if l == 'S' {
            s += 1
        }
    }
    m == 2 && s == 2
}

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
            grid.get(y).and_then(|row| row.get(x))
        })
    }).copied()

}

fn get_star(grid: &Vec<Vec<char>>, x: usize, y: usize) -> Option<String> {
    let mut word = String::new();
    for off in STAR_OFF {
        let char = get_square(grid, x, y, off.x, off.y);
        match char {
            Some(c) => word.push(c),
            None => return None
        }
    }
    Some(word)
}

fn main() {
    const INPUT_FILE: &str = "data/input.txt";

    let file_contents = fs::read_to_string(INPUT_FILE).expect("Should read input file");

    // Parse input into a useful format
    let grid: Vec<Vec<char>> = file_contents
        .lines()
        .map(|row| row.chars().collect())
        .collect();

    let mut total = 0;
    for (y, row) in grid.iter().enumerate() {
        'next: for (x, letter) in row.iter().enumerate() {
            if *letter != 'A' {
                continue
            }
            if let Some(word) = get_star(&grid, x, y) {
                if count_letters(&word) {
                    for disallowed in INVALID_STARS {
                        if word.eq(disallowed) {
                            continue 'next;
                        }
                    }
                    total += 1;
                }
            }
        } 
    }

    println!("{total}");
}
