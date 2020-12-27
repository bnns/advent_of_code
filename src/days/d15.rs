use std::collections::HashMap;
use std::fs;

pub fn solve(target: u8) {
    let contents = fs::read_to_string("data/day_15/input.txt").expect("error");
    let numbers: Vec<u32> = contents
        .lines()
        .filter_map(|s| s.parse::<String>().ok())
        .flat_map(|v| {
            v.split(",")
                .map(|n| n.parse::<u32>().ok().unwrap())
                .collect::<Vec<u32>>()
        })
        .collect();
    // let numbers = vec![1,3,2];
    let final_step = if target == 1 {
        2020
    } else if target == 2 {
        30000000
    } else {
        panic!("bad target.")
    };
    let mut last_heard: u32 = *numbers.last().unwrap();
    let mut recall: HashMap<u32, u32> = numbers
        .iter()
        .enumerate()
        .map(|(idx, n)| (*n, idx as u32 + 1))
        .collect();
    recall.remove(&last_heard);
    (numbers.len()..final_step + 1).for_each(|step| {
        let entry = if let Some(x) = recall.get(&last_heard) {
            step as u32 - *x
        } else {
            0
        };
        // println!("{:?}", (step, entry, last_heard));
        recall.insert(last_heard, step as u32);
        if step < final_step {
            last_heard = entry;
        }
    });

    println!("{}", last_heard);
}
