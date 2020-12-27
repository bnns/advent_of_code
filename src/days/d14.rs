use regex::Regex;
use std::collections::HashMap;
use std::fs;

fn generate(s: &str) -> (u64, u64) {
    s.split_terminator("")
        .skip(1)
        .enumerate()
        .fold((0, 0), |(p, r), (idx, letter)| {
            if letter != "X" {
                let digit: u64 = letter.parse().ok().unwrap();
                let position = s.len() - idx - 1;
                (p | (1 << position), r | (digit << position))
            } else {
                (p, r)
            }
        })
}

fn generate_all(s: &mut Vec<&str>, masks: &mut Vec<(u64, u64)>) -> Vec<(u64, u64)> {
    if s.len() == 0 {
        return masks.to_vec();
    }
    let pos = s.len() - 1;
    if s[0] == "X" {
        let copy = masks
            .clone()
            .iter()
            .map(|(p, r)| (p | (1 << pos as u64), r | (1 << pos as u64)))
            .collect::<Vec<(u64, u64)>>();
        generate_all(
            &mut s.drain(1..).collect(),
            &mut masks
                .iter()
                .map(|(p, r)| (p | (1 << pos as u64), r | (0 << pos as u64)))
                .chain(copy)
                .collect::<Vec<(u64, u64)>>(),
        )
    } else {
        let digit: u64 = s[0].parse().ok().unwrap();
        generate_all(
            &mut s.drain(1..).collect(),
            &mut masks
                .iter()
                .map(|(p, r)| {
                    if digit == 0 {
                        return (*p, *r);
                    }
                    (p | (1 << pos as u64), r | (1 << pos as u64))
                })
                .collect(),
        )
    }
}

pub fn solve(target: u8) {
    let contents = fs::read_to_string("data/day_14/input.txt").expect("error");
    let lines: Vec<String> = contents.lines().filter_map(|s| s.parse().ok()).collect();
    let re_mask = Regex::new(r"^mask = ([01X]+)").unwrap();
    let re_mem = Regex::new(r"^mem\[(\d+)\] = (\d+)").unwrap();
    let mut memory: HashMap<String, u64> = HashMap::new();
    let (mut p_mask, mut r_mask): (u64, u64) = (0, 0);
    let mut all_masks: Vec<(u64, u64)> = vec![(0, 0)];
    for l in lines.iter() {
        if let Some(mask_cap) = re_mask.captures(l) {
            let mask = mask_cap.get(1).unwrap().as_str();
            // destructuring assignments would be nice
            if target == 1 {
                let mask_tup = generate(mask);
                p_mask = mask_tup.0;
                r_mask = mask_tup.1;
            } else if target == 2 {
                all_masks = generate_all(
                    &mut mask.split_terminator("").skip(1).collect(),
                    &mut vec![(0, 0)],
                );
            }
        } else if let Some(mem_cap) = re_mem.captures(l) {
            let location = mem_cap.get(1).unwrap().as_str();
            let value = mem_cap.get(2).unwrap().as_str().parse::<u64>().unwrap();

            if target == 1 {
                memory.insert(location.to_string(), (value & !p_mask) | r_mask);
            } else if target == 2 {
                all_masks.iter().for_each(|(p_mask, r_mask)| {
                    memory.insert(
                        (((location.parse::<u64>().ok().unwrap()) & !p_mask) | r_mask).to_string(),
                        value,
                    );
                })
            }
        }
    }
    println!("{}", memory.values().sum::<u64>());
}
