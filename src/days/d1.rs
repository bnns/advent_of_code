use std::fs;

pub fn solve_2() {
    let contents = fs::read_to_string("day_1/input.txt").expect("error");
    let mut numbers: Vec<i32> = contents.lines().filter_map(|s| s.parse().ok()).collect();

    numbers.sort();

    let mut start_idx = 0;
    let mut end_idx = numbers.len() - 1;

    while start_idx < end_idx - 2 {
        let start = numbers[start_idx];
        let end = numbers[end_idx];

        if 2020 >= start + end {
            let remainder = 2020 - start - end;
            if numbers[start_idx + 1..end_idx - 1].contains(&remainder) {
                println!("{}", start * remainder * end);
                break;
            }
        }

        if end_idx > start_idx + 2 {
            end_idx -= 1;
        } else {
            start_idx += 1;
            end_idx = numbers.len() - 1;
        }
    }
}

pub fn solve_1() {
    let contents = fs::read_to_string("day_1/input.txt").expect("error");
    let numbers: Vec<i32> = contents.lines().map(|s| s.parse().unwrap()).collect();
    let numbers_clone = numbers.clone();
    for num in numbers_clone {
        if numbers.contains(&(2020 - num)) {
            println!("{}", num * (2020 - num));
            break;
        }
    }
}
