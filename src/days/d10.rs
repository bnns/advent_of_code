use std::fs;

// (0) 1 2 3 5 6 (9)
// 1 2 4 6 10 10
fn find_arrangements(adapters: &Vec<u32>) -> u64 {
    let mut totals: Vec<u64> = vec![0; adapters.len()];
    totals[0] = 1;
    for (idx, adapter) in adapters[1..].iter().enumerate() {
        let mut j = idx;
        let mut new: u64 = if *adapter < 4 { 1 } else { 0 };
        // println!("{}, {}, {:?}", j, adapter, totals);
        while (*adapter - adapters[j]) < 4 {
            new += totals[j] as u64;
            if j == 0 {
                break;
            } else {
                j -= 1;
            }
        }
        totals[idx + 1] = new;
    }

    totals[totals.len() - 1]
}

pub fn solve(target: i32) {
    let contents = fs::read_to_string("data/day_10/input.txt").expect("error");
    let mut adapters: Vec<u32> = contents.lines().filter_map(|s| s.parse().ok()).collect();
    let max: u32 = adapters.iter().max().unwrap() + 3;
    adapters.sort();
    adapters.push(max);
    let (v1, _, v2, _) =
        adapters
            .iter()
            .fold((0, 0, 0, 0), |(v1, v2, v3, last), n| match n - last {
                1 => (v1 + 1, v2, v3, *n),
                2 => (v1, v2 + 1, v3, *n),
                3 => (v1, v2, v3 + 1, *n),
                _ => panic!("jolt bad."),
            });
    if target == 1 {
        println!("{}", v1 * v2);
    } else if target == 2 {
        println!("{}", find_arrangements(&adapters));
    }
}
