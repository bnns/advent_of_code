use std::fs;

pub fn solve(target: Vec<(usize, usize)>) {
    let contents = fs::read_to_string("day_3/input.txt").expect("error");
    let lines: Vec<String> = contents.lines().filter_map(|s| s.parse().ok()).collect();
    let mut results = Vec::new();

    for (row_incr, col_incr) in target.iter() {
        let mut row = 0;
        let mut col = 0;
        let mut count = 0;

        while row < lines.len() {
            if let Some(current) = lines[row].chars().nth(col) {
                if current == '#' {
                    count += 1;
                }
                col = (col + col_incr) % lines[row].len();
                row += row_incr;
            } else {
                break;
            }
        }
        results.push(count);
    }

    println!("{}", results.iter().fold(1, |acc, n| acc * n));
}
