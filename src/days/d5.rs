use std::fs;

fn missing(list: &mut Vec<u32>) -> u32 {
    list.sort();
    let mut missed:u32 = 0;
    for ( idx, el ) in list.iter().enumerate() {
        if idx > 0 && idx < list.len() - 1 {
            if list[idx - 1] + 1 != *el {
                missed = *el - 1;
            }
        }
    }
    missed
}

pub fn solve(target: u8) {
    let contents = fs::read_to_string("data/day_5/input.txt").expect("error");
    let lines: Vec<String> = contents.lines().filter_map(|s| s.parse().ok()).collect();
    let mut count = lines.iter().map(|l| {
        let (row, seat) = l.split("")
            .enumerate()
            .fold((0, 0), |acc, (idx, letter)| match &letter[..] {
                "F" => acc,
                "B" => (acc.0 + 2_u32.pow(7u32 - idx as u32), acc.1),
                "L" => acc,
                "R" => (acc.0, acc.1 + 2_u32.pow(10u32 - idx as u32)),
                _ => acc,
            });
            row * 8 + seat
    }).collect::<Vec<u32>>();

    if target == 1 {
        println!("{}", count.iter().max().unwrap());
    } else if target == 2 {
        println!("{}", missing(&mut count));
    }
}
