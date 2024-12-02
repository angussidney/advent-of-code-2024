use std::fs;

#[derive(PartialEq)]
#[derive(Clone, Copy)]
enum Dir {
    Pos,
    Neg
}


fn test_pair(a: &i32, b: &i32, dir: Dir) -> bool {
    let diff = b - a;
    if (diff > 0) != (dir == Dir::Pos) {
        return false;
    }

    let absdiff = diff.abs();
    if absdiff < 1 || absdiff > 3 {
        return false;
    }

    true
}

fn test_seq_dir(vals: &Vec<i32>, dir: Dir) -> Option<usize> {
    for (i, left) in vals.iter().enumerate() {
        match vals.get(i + 1) {
            Some(right) => {
                if test_pair(left, right, dir) {
                    continue;
                } else {
                    return Some(i + 1);
                }
            }
            _ => {},
        }
    }
    None
}

fn test_seq_variations_dir(vals: &Vec<i32>, dir: Dir) -> bool {
    match test_seq_dir(vals, dir) {
        // Sequence is valid by default
        None => true,
        // Sequence needs some tweaking
        Some(idx) => {
            // TODO: O(n) (but only called a constant number of times)
            let mut vals_deleted = vals.clone();
            vals_deleted.remove(idx);
            if test_seq_dir(&vals_deleted, dir) == None {
                return true;
            }

            let mut del_first = vals.clone();
            del_first.remove(0);

            test_seq_dir(&del_first, dir) == None
        },
    }
}

fn test_sequence(vals: &Vec<i32>) -> bool {
    test_seq_variations_dir(vals, Dir::Pos) || test_seq_variations_dir(vals, Dir::Neg)
}

fn main() {
    const INPUT_FILE: &str = "data/sample.txt";

    let file_contents = fs::read_to_string(INPUT_FILE).expect("Should read input file");


    // Parse input into a useful format
    let mut valid = 0;
    for line in file_contents.lines() {
        let vals = line
            .split_whitespace()
            .map(|val| val.parse::<i32>().unwrap());
        
        if test_sequence(&Vec::from_iter(vals)) {
            valid += 1;
        }
    }
    println!("{valid}");
}
