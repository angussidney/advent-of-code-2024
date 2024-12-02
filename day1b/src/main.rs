use std::fs;
use std::collections::HashMap;

fn main() {
    const INPUT_FILE: &str = "data/sample.txt";

    let file_contents = fs::read_to_string(INPUT_FILE).expect("Should read input file");

    // Parse input into a useful format
    let mut left: HashMap<i32, i32> = HashMap::new();
    let mut right: HashMap<i32, i32> = HashMap::new();
    for line in file_contents.lines() {
        let mut vals = line.split_whitespace();
        let a = vals.next().unwrap().parse::<i32>().unwrap();
        let b = vals.next().unwrap().parse::<i32>().unwrap();
        left.insert(a, match left.get(&a) {
            None => 1,
            Some(tot) => tot + 1
        });
        right.insert(b, match right.get(&b) {
            None => 1,
            Some(tot) => tot + 1
        });
    }

    let mut sum = 0;
    for (num, ltimes) in left.into_iter() {
        sum += match right.get(&num) {
            None => 0,
            Some(rtimes) => num * ltimes * rtimes
        }
    }

    println!("{sum}");
}
