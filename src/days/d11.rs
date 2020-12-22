use itertools::Itertools;
use std::fs;

fn update_tolerant(r: usize, c: usize, grid: &Vec<Vec<i8>>, new_grid: &mut Vec<Vec<i8>>) -> bool {
    if grid[r][c] == -1 {
        return false;
    }
    let max_row = grid.len() as i16;
    let max_col = grid[r].len() as i16;
    let directions: Vec<(i16, i16)> = vec![
        (1, 0),
        (0, 1),
        (1, 1),
        (-1, 0),
        (0, -1),
        (-1, -1),
        (-1, 1),
        (1, -1),
    ];
    let neighbors = directions
        .iter()
        .map(|(r_inc, c_inc)| {
            let mut r_ptr = r as i16;
            let mut c_ptr = c as i16;
            let mut seat = None;
            while seat.is_none() {
                r_ptr += r_inc;
                c_ptr += c_inc;
                if r_ptr >= max_row || r_ptr < 0 || c_ptr >= max_col || c_ptr < 0 {
                    break;
                }
                seat = match grid[r_ptr as usize][c_ptr as usize] {
                    0 => Some(0),
                    1 => Some(1),
                    _ => None,
                }
            }
            seat.unwrap_or(0)
        })
        .sum::<u8>();

    match neighbors {
        x if x == 0 => {
            new_grid[r][c] = 1;
        }
        x if x > 4 => {
            new_grid[r][c] = 0;
        }
        _ => (),
    }

    new_grid[r][c] != grid[r][c]
}

fn update(r: usize, c: usize, grid: &Vec<Vec<i8>>, new_grid: &mut Vec<Vec<i8>>) -> bool {
    if grid[r][c] == -1 {
        return false;
    }
    let max_row = grid.len();
    let max_col = grid[r].len();
    let neighbors = vec![
        (c.checked_add(1), Some(r)),
        (Some(c), r.checked_add(1)),
        (c.checked_add(1), r.checked_add(1)),
        (c.checked_sub(1), Some(r)),
        (Some(c), r.checked_sub(1)),
        (c.checked_sub(1), r.checked_sub(1)),
        (c.checked_add(1), r.checked_sub(1)),
        (c.checked_sub(1), r.checked_add(1)),
    ];
    match neighbors.iter().fold(0, |acc, (col, row)| {
        if let (Some(co), Some(ro)) = (col, row) {
            if *ro < max_row && *co < max_col && grid[*ro][*co] == 1 {
                return acc + 1;
            }
        }

        acc
    }) {
        x if x == 0 => {
            new_grid[r][c] = 1;
        }
        x if x > 3 => {
            new_grid[r][c] = 0;
        }
        _ => (),
    }

    new_grid[r][c] != grid[r][c]
}

pub fn solve(target: i32) {
    let contents = fs::read_to_string("data/day_11/input.txt").expect("error");
    let seats: Vec<String> = contents.lines().filter_map(|s| s.parse().ok()).collect();
    let n: usize = seats[0].len();
    let mut state = vec![vec![0i8; n]; seats.len()];
    seats.iter().enumerate().for_each(|(row, s)| {
        s.split("")
            .filter(|c| *c == "L" || *c == "#" || *c == ".")
            .enumerate()
            .for_each(|(col, s)| match s {
                "L" => state[row][col] = 0,
                "#" => state[row][col] = 1,
                "." => state[row][col] = -1,
                _ => panic!("bad character."),
            })
    });
    let mut changed = true;
    let mut count = 0;
    while changed == true {
        changed = false;
        count = 0;
        let mut new_state = state.clone();
        for (row, col) in (0..state.len()).cartesian_product(0..state[0].len()) {
            if target == 1 {
                changed |= update(row, col, &state, &mut new_state);
            } else if target == 2 {
                changed |= update_tolerant(row, col, &state, &mut new_state);
            }
            if new_state[row][col] == 1 {
                count += 1;
            }
        }
        state = new_state;
    }

    println!("{}", count);
}
