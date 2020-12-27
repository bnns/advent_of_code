use regex::Regex;
use std::fs;

fn assign_fields<'a>(
    fields: &mut Vec<(&'a String, Vec<usize>)>,
    assigned: &mut Vec<(&'a String, usize)>,
) -> Vec<(&'a String, usize)> {
    if fields.len() == 0 {
        return assigned.to_vec();
    }
    if let Some(field) = fields.iter().find(|f| f.1.len() == 1) {
        let only_one = *field.1.last().unwrap();
        assigned.push((field.0, only_one));
        assign_fields(
            &mut fields
                .iter()
                .filter(|(name, _)| *name != field.0)
                .map(|(name, f)| {
                    (
                        *name,
                        f.clone()
                            .iter()
                            .filter(|ln| **ln != only_one)
                            .map(|ln| *ln)
                            .collect(),
                    )
                })
                .collect(),
            assigned,
        )
    } else {
        panic!("stalemate.")
    }
}

pub fn solve(target: u8) {
    let contents = fs::read_to_string("data/day_16/input.txt").expect("error");
    let info: Vec<String> = contents
        .lines()
        .filter_map(|s| s.parse::<String>().ok())
        .collect();
    let rules_re = Regex::new(r"^(.*): (\d+)-(\d+) or (\d+)-(\d+)").unwrap();
    let ticket_re = Regex::new(r"^your ticket").unwrap();
    let nearby_re = Regex::new(r"^nearby tickets").unwrap();
    let mut rules: Vec<(String, (u16, u16), (u16, u16))> = vec![];
    let mut my_ticket: Vec<u16> = vec![];
    let mut nearby_tickets: Vec<Vec<u16>> = vec![];
    let mut nearby_only: bool = false;
    for (idx, l) in info.iter().enumerate() {
        if nearby_only {
            nearby_tickets.push(
                l.split(",")
                    .map(|n| n.parse().unwrap())
                    .collect::<Vec<u16>>(),
            );
            continue;
        }
        if let Some(rule_matches) = rules_re.captures(l) {
            let name = rule_matches.get(1).unwrap().as_str();
            let s1 = rule_matches
                .get(2)
                .unwrap()
                .as_str()
                .parse::<u16>()
                .unwrap();
            let t1 = rule_matches
                .get(3)
                .unwrap()
                .as_str()
                .parse::<u16>()
                .unwrap();
            let s2 = rule_matches
                .get(4)
                .unwrap()
                .as_str()
                .parse::<u16>()
                .unwrap();
            let t2 = rule_matches
                .get(5)
                .unwrap()
                .as_str()
                .parse::<u16>()
                .unwrap();
            rules.push((name.to_string(), (s1, t1), (s2, t2)));
        }
        if ticket_re.is_match(l) {
            my_ticket = info[idx + 1]
                .split(",")
                .map(|n| n.parse().unwrap())
                .collect();
        }
        if nearby_re.is_match(l) {
            nearby_only = true;
        }
    }
    if target == 1 {
        let answer: u16 = nearby_tickets
            .iter()
            .flatten()
            .filter(|n| {
                rules
                    .iter()
                    .all(|(_, (s1, t1), (s2, t2))| (*n < s1 || *n > t1) && (*n < s2 || *n > t2))
            })
            .sum();
        println!("{}", answer);
    } else if target == 2 {
        let valid: Vec<Vec<u16>> = nearby_tickets
            .iter()
            .filter(|t| {
                t.iter().all(|n| {
                    rules
                        .iter()
                        .any(|(_, (s1, t1), (s2, t2))| (n >= s1 && n <= t1) || (n >= s2 && n <= t2))
                })
            })
            .map(|t| t.clone())
            .collect();
        let mut columns: Vec<Vec<u16>> = vec![];
        for idx in 0..my_ticket.len() {
            columns.push(valid.iter().map(|t| t[idx]).collect());
        }
        let mut all: Vec<(&String, Vec<usize>)> = rules
            .iter()
            .map(|(name, (s1, t1), (s2, t2))| {
                let possible: Vec<usize> = columns
                    .iter()
                    .enumerate()
                    .filter(|(_idx, col)| {
                        col.iter()
                            .all(|n| (n >= s1 && n <= t1) || (n >= s2 && n <= t2))
                    })
                    .map(|(idx, _)| idx)
                    .collect();
                (name, possible)
            })
            .collect();
        let answer: u64 = assign_fields(&mut all, &mut vec![])
            .iter()
            .filter(|(name, _)| name.contains("departure"))
            .map(|(_, idx)| my_ticket[*idx] as u64)
            .product();
        println!("{}", answer);
    }
}
