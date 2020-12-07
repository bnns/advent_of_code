use std::fs;

// byr (Birth Year)
// iyr (Issue Year)
// eyr (Expiration Year)
// hgt (Height)
// hcl (Hair Color)
// ecl (Eye Color)
// pid (Passport ID)
// cid (Country ID)

pub fn solve() {
    let contents = fs::read_to_string("day_4/input.txt").expect("error");
    let lines: Vec<String> = contents.lines().filter_map(|s| s.parse().ok()).collect();
    let allowed: Vec<&str> = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"].to_vec();
    let mut row = 0;
    let mut count = 0;
    let mut results: Vec<&str> = Vec::new();

    while row < lines.len() {
        if None == lines[row].chars().nth(0) {
            if allowed.iter().all(|s| results.contains(s)) {
                count += 1;
            }
            results = Vec::new();
        } else {
            let mut pairs = lines[row]
                .split(|c| c == ' ' || c == ':').collect();

            results.append(&mut pairs);
        }
        row += 1;
    }

    println!("{}", count);
}
