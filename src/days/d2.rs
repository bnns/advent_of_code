use std::convert::TryFrom;
use std::convert::TryInto;
use std::fs;

fn check_pw_pos(password: &str, pattern: &str, pos_1: i32, pos_2: i32) -> bool {
    let (_, first) = password.split_at((pos_1 - 1).try_into().unwrap());
    let (_, second) = password.split_at((pos_2 - 1).try_into().unwrap());
    return first.starts_with(pattern) ^ second.starts_with(pattern);
}

fn check_pw_range(password: &str, pattern: &str, min: i32, max: i32) -> bool {
    let count_raw = password.matches(pattern).count();
    let count = i32::try_from(count_raw).unwrap();

    return count >= min && count <= max;
}

pub fn solve(target: i32) {
    let contents = fs::read_to_string("day_2/input.txt").expect("error");
    let passwords: Vec<String> = contents.lines().filter_map(|s| s.parse().ok()).collect();

    let total = passwords.iter().fold(0, |acc, x| {
        let each: Vec<String> = x
            .split(|c| c == ' ' || c == '-' || c == ':')
            .map(|s| s.to_string())
            .collect();

        if let [min_raw, max_raw, pattern, _, password] = each.as_slice() {
            let min: i32 = min_raw.parse().unwrap();
            let max: i32 = max_raw.parse().unwrap();
            if target == 2 && check_pw_pos(password, pattern, min, max) {
                acc + 1
            } else if target == 1 && check_pw_range(password, pattern, min, max) {
                acc + 1
            } else {
                acc
            }
        } else {
            acc
        }
    });

    println!("{}/{}", total, passwords.len());
}
