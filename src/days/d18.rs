use regex::Regex;
use std::fs;

fn eval(expr: &str) -> u64 {
    lazy_static! {
        static ref OP_RE: Regex = Regex::new(r"([*+])").unwrap();
        static ref TERM_RE: Regex = Regex::new(r"(\d+)").unwrap();
    }

    let all_terms: Vec<u64> = TERM_RE
        .find_iter(expr)
        .filter_map(|term| term.as_str().parse().ok())
        .collect();
    let mut last_term: u64 = all_terms[0];
    for (idx, captured_op) in OP_RE.find_iter(expr).enumerate() {
        let op = captured_op.as_str();
        let term: u64 = all_terms[idx + 1];
        last_term = match op {
            "+" => last_term + term,
            "*" => last_term * term,
            _ => panic!("bad op."),
        }
    }
    last_term
}

fn replace(expr: &str, target: u8) -> String {
    lazy_static! {
        static ref ADD_RE: Regex = Regex::new(r"(\d+ \+ \d+)").unwrap();
        static ref PARENS_RE: Regex = Regex::new(r"\((\d+[^\(\)]*)\)").unwrap();
    }

    // println!("replace {}", expr);

    if target == 2 {
        if let Some(add_expr) = ADD_RE.captures(expr) {
            let replacement = eval(add_expr.get(1).unwrap().as_str());
            return replace(
                &ADD_RE
                    .replace(expr, &replacement.to_string()[..])
                    .to_string(),
                target,
            );
        }
    }

    match PARENS_RE.captures(expr) {
        None => eval(expr).to_string(),
        Some(cap) => {
            let replacement = replace(cap.get(1).unwrap().as_str(), target).to_string();
            replace(
                &PARENS_RE.replace(expr, &replacement[..]).to_string(),
                target,
            )
        }
    }
}

pub fn solve(target: u8) {
    let contents = fs::read_to_string("data/day_18/input.txt").expect("error");
    let answer: u64 = contents
        .lines()
        .map(|s| s.parse::<String>().ok().unwrap())
        .fold(0, |acc, line| {
            let numba = replace(&line, target).parse::<u64>().ok().unwrap();
            acc + numba
        });
    println!("{}", answer);
}
