use std::fs;

fn accumulate_equation(total: usize, cursor: usize, nums: &Vec<usize>, target: usize, valid: &mut bool) {
    if let Some(num) = nums.get(cursor) {
        if total != 0 {
            // In the case of the initial step where the total is zero, we must start with the first number
            accumulate_equation(total * num, cursor + 1, nums, target, valid);
        }
        accumulate_equation(total + num, cursor + 1, nums, target, valid);
        let concat = format!("{total}{num}");
        accumulate_equation(concat.parse::<usize>().unwrap(), cursor + 1, nums, target, valid);
    } else {
        // End of the list
        if total == target {
            *valid = true;
        }
    }
}

fn main() {
    const INPUT_FILE: &str = "data/input.txt";

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
