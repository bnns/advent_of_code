use regex::Regex;
use std::collections::HashSet;
use std::convert::TryFrom;
use std::fs;

fn find_broken(
    program: &Vec<(&str, i16)>,
    line: usize,
    count: i16,
    changed: Option<usize>,
    mut visited: HashSet<usize>,
) -> Option<i16> {
    if line == program.len() {
        return Some(count);
    } else if visited.get(&line).is_some() || line >= program.len() {
        return None;
    }

    visited.insert(line);
    let v_clone = visited.clone();

    match program[line] {
        ("acc", num) => find_broken(program, line + 1, count + num, changed, v_clone),
        ("nop", _) if changed.is_some() => find_broken(program, line + 1, count, changed, v_clone),
        ("nop", num) if changed == None => {
            let v2_clone = v_clone.clone();
            if let Some(count) = find_broken(program, line + 1, count, None, v_clone) {
                Some(count)
            } else if let Some(count) = find_broken(
                program,
                ((line as i16) + num) as usize,
                count,
                Some(line),
                v2_clone,
            ) {
                Some(count)
            } else {
                None
            }
        }
        ("jmp", num) if changed.is_some() => find_broken(
            program,
            ((line as i16) + num) as usize,
            count,
            changed,
            v_clone,
        ),
        ("jmp", num) if changed == None => {
            let v2_clone = v_clone.clone();
            if let Some(count) = find_broken(program, line + 1, count, Some(line), v_clone) {
                Some(count)
            } else if let Some(count) = find_broken(
                program,
                ((line as i16) + num) as usize,
                count,
                None,
                v2_clone,
            ) {
                Some(count)
            } else {
                None
            }
        }
        _ => panic!("bad instruction."),
    }
}

pub fn solve(target: u8) {
    let contents = fs::read_to_string("data/day_8/input.txt").expect("error");
    let lines: Vec<String> = contents.lines().filter_map(|s| s.parse().ok()).collect();
    let re = Regex::new(r"(\w+) (\+?\-?\d+)").unwrap();
    let program: Vec<(&str, i16)> = lines
        .iter()
        .map(|child| {
            if let Some(caps) = re.captures(child) {
                let instruction = caps.get(1).unwrap().as_str();
                let num = caps.get(2).unwrap().as_str().parse::<i16>().unwrap();
                (instruction, num)
            } else {
                panic!("bad capture.")
            }
        })
        .collect();

    let mut next = 0;

    if target == 1 {
        let mut visited_lines = HashSet::new();
        let mut count = 0;

        while visited_lines.get(&next).is_none() {
            visited_lines.insert(next);
            match program[next] {
                ("acc", num) => {
                    count += num;
                    next += 1
                }
                ("nop", _) => next += 1,
                ("jmp", num) => next = usize::try_from((next as i16) + num).unwrap(),
                _ => panic!("bad instruction."),
            }
        }

        println!("{}", count);
    } else if target == 2 {
        println!(
            "{}",
            find_broken(&program, next, 0, None, HashSet::new()).unwrap()
        );
    }
}
