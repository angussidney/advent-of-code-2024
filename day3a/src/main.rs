use std::fs;
use regex::Regex;

fn main() {
    const INPUT_FILE: &str = "data/input.txt";

    let file_contents = fs::read_to_string(INPUT_FILE).expect("Should read input file");

    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    // Parse input into a useful format
    let mut total = 0;
    for line in file_contents.lines() {
        let matches = re
            .captures_iter(line);
        
        for m in matches {
            let left = &m[1].parse::<u32>().unwrap();
            let right = &m[2].parse::<u32>().unwrap();
            total += left * right;
        }
    }
    println!("{total}");
}
