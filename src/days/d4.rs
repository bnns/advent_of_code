use regex::Regex;
use std::fs;

#[derive(PartialEq)]
enum Metric {
    Centimeter,
    Inch,
}

struct Passport {
    byr: Option<i32>,
    iyr: Option<i32>,
    eyr: Option<i32>,
    hgt: Option<(i32, Metric)>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>
}

impl Passport {
    fn new() -> Self {
        Passport {
            byr: None,
            iyr: None,
            eyr: None,
            hgt: None,
            hcl: None,
            ecl: None,
            pid: None,
        }
    }
    fn is_valid(&self) -> bool {
        self.byr != None
            && self.iyr != None
            && self.eyr != None
            && self.hgt != None
            && self.hcl != None
            && self.ecl != None
            && self.pid != None
    }
    fn convert_byr(s: &str) -> Option<i32> {
        let num = s.parse::<i32>().unwrap();
        if num >= 1920 && num <= 2002 {
            Some(num)
        } else {
            None
        }
    }
    fn convert_iyr(s: &str) -> Option<i32> {
        let num = s.parse::<i32>().unwrap();
        if num >= 2010 && num <= 2020 {
            Some(num)
        } else {
            None
        }
    }
    fn convert_eyr(s: &str) -> Option<i32> {
        let num = s.parse::<i32>().unwrap();
        if num >= 2020 && num <= 2030 {
            Some(num)
        } else {
            None
        }
    }
    fn convert_hgt(s: &str) -> Option<(i32, Metric)> {
        if s.ends_with("cm") {
            match s.strip_suffix("cm").unwrap().parse::<i32>().unwrap() {
                x if x >= 150 && x <= 193 => Some((x, Metric::Centimeter)),
                _ => None,
            }
        } else if s.ends_with("in") {
            match s.strip_suffix("in").unwrap().parse::<i32>().unwrap() {
                x if x >= 59 && x <= 76 => Some((x, Metric::Inch)),
                _ => None,
            }
        } else {
            None
        }
    }

    fn convert_hcl(s: &str) -> Option<String> {
        let re = Regex::new(r"^#[0-9|a-f]{6}$").unwrap();
        if re.is_match(s) {
            Some(String::from(s))
        } else {
            None
        }
    }

    fn convert_ecl(s: &str) -> Option<String> {
        let re = Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
        if re.is_match(s) {
            Some(String::from(s))
        } else {
            None
        }
    }

    fn convert_pid(s: &str) -> Option<String> {
        let re = Regex::new(r"^\d{9}$").unwrap();
        if re.is_match(s) {
            Some(String::from(s))
        } else {
            None
        }
    }
}

impl From<Vec<(&str, &str)>> for Passport {
    fn from(inputs: Vec<(&str, &str)>) -> Self {
        inputs
            .iter()
            .fold(Passport::new(), |acc, (k, v)| match &k[..] {
                "byr" => Passport {
                    byr: Passport::convert_byr(v),
                    ..acc
                },
                "iyr" => Passport {
                    iyr: Passport::convert_iyr(v),
                    ..acc
                },
                "eyr" => Passport {
                    eyr: Passport::convert_eyr(v),
                    ..acc
                },
                "hgt" => Passport {
                    hgt: Passport::convert_hgt(v),
                    ..acc
                },
                "hcl" => Passport {
                    hcl: Passport::convert_hcl(v),
                    ..acc
                },
                "ecl" => Passport {
                    ecl: Passport::convert_ecl(v),
                    ..acc
                },
                "pid" => Passport {
                    pid: Passport::convert_pid(v),
                    ..acc
                },
                _ => Passport { ..acc },
            })
    }
}

pub fn solve(target: i32) {
    let contents = fs::read_to_string("data/day_4/input.txt").expect("error");
    let lines: Vec<String> = contents.lines().filter_map(|s| s.parse().ok()).collect();
    let allowed: Vec<&str> = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"].to_vec();
    let mut count = 0;
    let mut results: Vec<(&str, &str)> = Vec::new();

    for row in lines.iter() {
        if None == row.chars().nth(0) {
            if target == 1
                && allowed.iter().all(|s| {
                    results
                        .iter()
                        .map(|s| s.0)
                        .collect::<Vec<&str>>()
                        .contains(s)
                })
            {
                count += 1;
            } else if target == 2 && Passport::from(results).is_valid() {
                count += 1;
            }
            results = Vec::new();
        } else {
            let mut pairs = row
                .split(|c| c == ' ')
                .map(|s| {
                    let (k, v) = s.split_at(s.find(':').unwrap());
                    (k, v.strip_prefix(":").unwrap())
                })
                .collect::<Vec<(&str, &str)>>();

            results.append(&mut pairs);
        }
    }

    println!("{}", count);
}
