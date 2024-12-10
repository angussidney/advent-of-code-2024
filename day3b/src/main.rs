use std::fs;
use regex::Regex;

fn main() {
    const INPUT_FILE: &str = "data/input.txt";

    let file_contents = fs::read_to_string(INPUT_FILE).expect("Should read input file");

    let re = Regex::new(r"(?:(do)\(\)|(don't)\(\)|(mul)\((\d{1,3}),(\d{1,3})\))").unwrap();

    // Parse input into a useful format
    let mut total = 0;
    let mut enabled = true;
    for line in file_contents.lines() {
        let matches = re
            .captures_iter(line);
        
        for m in matches {
            if m.get(1).is_some() {
                println!("do");
                enabled = true;
            } else if m.get(2).is_some() {
                println!("don't");
                enabled = false;
            } else if m.get(3).is_some() {
                if enabled {
                    println!("mul enable");
                    let left = &m[4].parse::<u32>().unwrap();
                    let right = &m[5].parse::<u32>().unwrap();
                    total += left * right;
                } else {
                    println!("mul not enabled");
                }
            } else {
                println!("wat");
            }
        }
    }
    println!("{total}");
}
