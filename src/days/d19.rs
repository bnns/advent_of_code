use regex::Regex;
use std::collections::HashMap;
use std::fs;

enum Rule {
    Cryptic(Vec<Vec<String>>),
    Clear(String),
}

fn valid_strings(rule: &Rule, hash: &HashMap<u32, Rule>, valid: &mut Vec<String>) -> Vec<String> {
    if let Rule::Clear(val) = rule {
        *valid = valid
            .iter()
            .map(|v| [&v[..], &val[..]].join(""))
            .collect::<Vec<String>>();
        valid.to_vec()
    } else if let Rule::Cryptic(list) = rule {
        let mut new_valid = vec![];
        for outer in list.iter() {
            let mut copy = valid.clone();
            for inner in outer.iter() {
                let key: u32 = inner.parse().ok().unwrap();
                let inner_rule: &Rule = &hash.get(&key).unwrap();
                copy = valid_strings(inner_rule, hash, &mut copy)
            }
            new_valid.append(&mut copy);
        }
        new_valid
    } else {
        panic!("bad enum.")
    }
}

pub fn solve(target: u8) {
    let contents = fs::read_to_string("data/day_19/input.txt").expect("error");
    let rule_re = Regex::new(r"(\d+): (.*)$").unwrap();
    let string_re = Regex::new(r"^([a-b]+)$").unwrap();
    let base_rule_re = Regex::new(r"([a-b])").unwrap();
    let mut strings: Vec<String> = vec![];
    let dict: HashMap<u32, Rule> = contents
        .lines()
        .filter_map(|s| s.parse::<String>().ok())
        .fold(HashMap::new(), |mut acc, line| {
            if let Some(caps) = rule_re.captures(&line) {
                let rule = caps.get(1).unwrap().as_str().parse().ok().unwrap();
                let body = caps.get(2).unwrap().as_str();
                if let Some(captured_body) = base_rule_re.find(body) {
                    acc.insert(rule, Rule::Clear(captured_body.as_str().to_string()));
                } else {
                    let rule_body: Vec<Vec<String>> = body
                        .split("|")
                        .map(|part| {
                            part.split_terminator(" ")
                                .filter_map(|p| match p {
                                    "" => None,
                                    v => Some(v.to_string()),
                                })
                                .collect::<Vec<String>>()
                        })
                        .collect();
                    acc.insert(rule, Rule::Cryptic(rule_body));
                }
            } else if let Some(caps) = string_re.captures(&line) {
                strings.push(caps.get(1).unwrap().as_str().to_string());
            }
            acc
        });

    if target == 2 {
        let rule_42 = valid_strings(dict.get(&42).unwrap(), &dict, &mut vec!["".to_string()]);
        let rule_31 = valid_strings(dict.get(&31).unwrap(), &dict, &mut vec!["".to_string()]);
        let valid = valid_strings(dict.get(&0).unwrap(), &dict, &mut vec!["".to_string()]);

        println!("rule 42 {}", rule_42.len());

        let matching = strings.iter().filter(|s| {
            if valid.contains(s) {
                return true;
            } else if rule_42.contains(s) && rule_31.contains(s) {
                println!("rule 42 and 31 matches {}", s);
                return true;
            } else if rule_42.contains(s) {
                println!("rule 42 matches {}", s);
                return true;
            }

            false
        }).collect::<Vec<&String>>();
        println!("{}", matching.len());
    } else if target == 1 {
        let valid = valid_strings(dict.get(&0).unwrap(), &dict, &mut vec!["".to_string()]);
        let matching = strings.iter().filter(|s| valid.contains(s)).collect::<Vec<&String>>();
        println!("{}", matching.len());
    }

}
