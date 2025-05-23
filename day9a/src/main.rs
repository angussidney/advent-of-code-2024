use std::fs;


// doubly linked list that's a freelist
// read from last node, insert in free nodes

fn main() {
    const INPUT_FILE: &str = "data/sample.txt";

    let file_contents = fs::read_to_string(INPUT_FILE).expect("Should read input file");

    let mut total = 0;
    for line in file_contents.lines() {
        let (sum, nums_str) = line.split_once(": ").unwrap();
        let nums: Vec<usize> = nums_str
            .split_ascii_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        let mut valid = false;
        accumulate_equation(0, 0, &nums, sum.parse::<usize>().unwrap(), &mut valid);
        if valid {
            total += sum.parse::<usize>().unwrap();
        }
    }

    println!("{total}");
}
