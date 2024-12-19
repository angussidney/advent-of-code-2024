use std::{cmp::Ordering, collections::HashMap, fs};

enum ReadMode {
    COMPARISONS,
    LISTS
}

fn main() {
    const INPUT_FILE: &str = "data/input.txt";

    let file_contents = fs::read_to_string(INPUT_FILE).expect("Should read input file");

    let mut comparison: HashMap<(i64, i64), bool> = HashMap::new();

    let mut total = 0;

    let mut mode = ReadMode::COMPARISONS;
    for line in file_contents.lines() {
        if line.is_empty() {
            mode = ReadMode::LISTS;
            continue;
        }
        match mode {
            ReadMode::COMPARISONS => {
                let mut nums = line.split("|");
                let before: i64 = nums.next().unwrap().parse().unwrap();
                let after: i64 = nums.next().unwrap().parse().unwrap();

                comparison.insert((before, after), true);
                comparison.insert((after, before), false);
            },
            ReadMode::LISTS => {
                let nums: Vec<i64> = line
                    .split(",")
                    .map(|x|x.parse::<i64>().unwrap())
                    .collect();

                let mut sorted = nums.to_vec();
                sorted.sort_by(|l, r| {
                    comparison.get(&(*l, *r))
                        .map(|smaller| if *smaller {Ordering::Less} else {Ordering::Greater})
                        .unwrap_or(Ordering::Equal)
                });

                if !nums.eq(&sorted) {
                    total += sorted[sorted.len() / 2];
                }
            },
        }
    }

    println!("{total}");
}
