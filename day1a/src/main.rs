use std::fs;

fn main() {
    const INPUT_FILE: &str = "data/input.txt";

    let file_contents = fs::read_to_string(INPUT_FILE).expect("Should read input file");

    // Parse input into a useful format
    let mut line_one = Vec::new();
    let mut line_two = Vec::new();
    for line in file_contents.lines() {
        let mut vals = line.split_whitespace();
        line_one.push(vals.next().unwrap());
        line_two.push(vals.next().unwrap());
    }

    // Sort both lines so that we can pop the smallest elements in O(1) time
    line_one.sort();
    line_two.sort();

    let mut sum = 0;
    while line_one.len() > 0 {
        let a = line_one.pop().unwrap().parse::<i64>().unwrap();
        let b = line_two.pop().unwrap().parse::<i64>().unwrap();

        let diff = (a - b).abs();
        sum += diff;
    }

    println!("{sum}");
}
