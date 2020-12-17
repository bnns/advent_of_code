use std::fs;

fn contiguous(list: &Vec<i32>, target: &i32) -> i32 {
    let mut i = 0;
    let mut j = 1;
    while i < list.len() - 1 && j < list.len() {
        let sum = &list[i..j].iter().sum::<i32>();
        if sum == target {
            let min = list[i..j].iter().min().unwrap();
            let max = list[i..j].iter().max().unwrap();
            return min + max;
        } else if sum > target {
            i += 1;
        } else if sum < target {
            j += 1;
        }
    }
    panic!("bad")
}


fn valid(preamble: &Vec<i32>, target: &i32) -> Option<(i32, i32)> {
    if preamble.len() == 0 {
        return None;
    }
    let mut i = 0;
    let mut j = preamble.len() - 1;
    let mut answer = None;
    while i < j {
        if (target - preamble[i]) == preamble[j] {
            answer = Some((preamble[i], preamble[j]));
        }
        if j == i + 1 {
            j = preamble.len();
            i += 1;
        }
        j -= 1;
    }
    answer
}

pub fn solve(target: u8) {
    let contents = fs::read_to_string("data/day_9/input.txt").expect("error");
    let lines: Vec<i32> = contents
        .lines()
        .filter_map(|s| s.parse::<i32>().ok())
        .collect();
    let mut invalid = 0;
    let mut preamble = lines[0..25].to_vec();
    let mut preamble_sorted: Vec<i32> = preamble.clone();
    preamble_sorted.sort();
    for num in lines[25..lines.len()].iter() {
        let min = preamble.iter().min().unwrap();
        if let None = valid(
            &preamble_sorted
                .into_iter()
                .filter(|i| i <= &(num - min))
                .collect(),
            num,
        ) {
            invalid = *num;
            break;
        }

        preamble.remove(0);
        preamble.push(*num);
        preamble_sorted = preamble.clone();
        preamble_sorted.sort();
    }

    if target == 1 {
        println!("{}", invalid);
    } else if target == 2 {
        println!("{}", contiguous(&lines, &invalid));
    }
}
