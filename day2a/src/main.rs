use std::fs;

#[derive(PartialEq)]
#[derive(Clone, Copy)]
enum Dir {
    Pos,
    Neg
}

fn main() {
    const INPUT_FILE: &str = "data/input.txt";

    let file_contents = fs::read_to_string(INPUT_FILE).expect("Should read input file");


    // Parse input into a useful format
    let mut invalid = 0;
    let mut lines = 0;
    'lines: for line in file_contents.lines() {
        lines += 1;
        let vals = line.split_whitespace();
        let mut last = None;
        let mut dir = None;
        for num_str in vals {
            let num: i32 = num_str.parse().unwrap();
            match last {
                Some(lst) => {
                    let diff: i32 = num - lst;
                    match dir {
                        Some(d) => {
                            if (diff > 0) != (d == Dir::Pos) {
                                invalid += 1;
                                println!("Wrong dir");
                                continue 'lines;
                            }
                        },
                        None => {
                            if diff > 0 {
                                dir = Some(Dir::Pos);
                            } else {
                                dir = Some(Dir::Neg);
                            }
                        }
                    }

                    if diff.abs() < 1 || diff.abs() > 3 {
                        invalid += 1;
                        println!("Wrong amount {diff}");
                        continue 'lines;
                    }
                },
                None => {},
            }
            last = Some(num);
        }
    }
    println!("{lines}");
    println!("{}", lines - invalid);
}
