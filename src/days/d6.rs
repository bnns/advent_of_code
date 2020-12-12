use std::collections::HashMap;
use std::fs;

pub fn solve(target: u8) {
    let contents = fs::read_to_string("data/day_6/input.txt").expect("error");
    let lines: Vec<String> = contents.lines().filter_map(|s| s.parse().ok()).collect();
    let mut count = 0;
    let mut people = 0;
    let mut answers = HashMap::new();

    for row in lines.iter() {
        if None == row.chars().nth(0) {
            count += if target == 1 {
                answers.len()
            } else if target == 2 {
                answers
                    .iter()
                    .filter(|(_, &val)| val == people)
                    .collect::<Vec<_>>()
                    .len()
            } else {
                0
            };
            answers = HashMap::new();
            people = 0;
        } else {
            people += 1;
            for c in row.split_terminator("").skip(1) {
                *answers.entry(c).or_insert(1) += 1;
            }
        }
    }

    println!("{}", count);
}
