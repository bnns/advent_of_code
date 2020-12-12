use regex::Regex;
use std::collections::HashSet;
use std::fmt;
use std::fs;
use std::hash::{Hash, Hasher};
use std::iter::FromIterator;

#[derive(Debug)]
struct Bag {
    adjective: String,
    children: Vec<(u16, Bag)>,
}

impl Bag {
    fn contains(&self, bag: &Self) -> bool {
        self.children.iter().any(|(_, b)| b == bag)
    }
}

impl fmt::Display for Bag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}, {:?}", self.adjective, self.children)
    }
}

impl PartialEq for Bag {
    fn eq(&self, other: &Self) -> bool {
        self.adjective == other.adjective
    }
}

impl Eq for Bag {}

impl Hash for Bag {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.adjective.hash(state)
    }
}

fn count_all(hash: &HashSet<&Bag>, target: &Bag) -> u32 {
    if let Some(bag) = hash.get(target) {
        bag.children
            .iter()
            .map(|(n, c)| (*n as u32) * (count_all(hash, c) + 1))
            .sum::<u32>()
    } else {
        panic!("cannot find.")
    }
}

pub fn solve(target: u8) {
    let contents = fs::read_to_string("data/day_7/input.txt").expect("error");
    let lines: Vec<String> = contents.lines().filter_map(|s| s.parse().ok()).collect();
    let re = Regex::new(r"(\d) (.*) bags?\.?").unwrap();
    let rules: Vec<Bag> = lines
        .iter()
        .map(|row| {
            let slice = row.split(" bags contain ").collect::<Vec<&str>>();
            let children = slice[1]
                .split(", ")
                .filter_map(|child| {
                    if let Some(caps) = re.captures(child) {
                        let num = caps.get(1).unwrap().as_str().parse::<u16>().unwrap();
                        let child_adj = caps.get(2).unwrap().as_str();
                        Some((
                            num,
                            Bag {
                                adjective: child_adj.to_string(),
                                children: vec![],
                            },
                        ))
                    } else {
                        None
                    }
                })
                .collect::<Vec<(u16, Bag)>>();

            Bag {
                adjective: slice[0].to_string(),
                children: children,
            }
        })
        .collect();

    let bag = rules.iter().find(|b| b.adjective == "shiny gold").unwrap();

    if target == 2 {
        let rules_hash = HashSet::from_iter(rules.iter());
        println!("{}", count_all(&rules_hash, bag));
    } else if target == 1 {
        let mut valid_bags = HashSet::new();
        let mut check_next = vec![bag];
        let mut already_checked = HashSet::new();

        while check_next.len() > 0 {
            let next = check_next.pop().unwrap();
            rules
                .iter()
                .filter(|b| b.contains(&next) && !already_checked.contains(*b))
                .for_each(|b| {
                    check_next.push(b);
                    valid_bags.insert(b);
                });

            already_checked.insert(next);
        }

        println!("{}", valid_bags.len());
    }
}
