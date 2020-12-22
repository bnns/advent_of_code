use regex::Regex;
use std::fs;

enum Sign {
    Positive = 1,
    Negative = -1,
}

fn forward(angle: i16) -> fn((i16, i16, i16), i16) -> (i16, i16, i16) {
    match angle {
        d if d.abs() == 0 => |(x, y, angle), num| (x + num, y, angle),
        d if d.abs() == 180 => |(x, y, angle), num| (x - num, y, angle),
        d if d.abs() == 90 => |(x, y, angle), num| (x, y + num, angle),
        d if d.abs() == 270 => |(x, y, angle), num| (x, y - num, angle),
        _ => panic!("bad angle."),
    }
}

fn rotate(sign: Sign, angle: i32, x: i32, y: i32) -> (i32, i32) {
    match (angle, sign) {
        (d, _) if d == 180 => (-x, -y),
        (d, Sign::Positive) if d == 90 => (-y, x),
        (d, Sign::Positive) if d == 270 => (y, -x),
        (d, Sign::Negative) if d == 90 => (y, -x),
        (d, Sign::Negative) if d == 270 => (-y, x),
        _ => panic!("bad angle."),
    }
}

fn move_ship((x, y, angle): (i16, i16, i16), (instruction, num): (&str, i16)) -> (i16, i16, i16) {
    // println!("{:?}", (x, y, angle));
    // println!("{:?}", (instruction, num));
    match instruction {
        "F" => forward(angle)((x, y, angle), num),
        "L" => (x, y, (angle + num).rem_euclid(360)),
        "R" => (x, y, (angle - num).rem_euclid(360)),
        "N" => (x, y + num, angle),
        "S" => (x, y - num, angle),
        "E" => (x + num, y, angle),
        "W" => (x - num, y, angle),
        _ => panic!("invalid move."),
    }
}

fn move_ship_waypoint(
    (x, y, (offset_x, offset_y)): (i32, i32, (i32, i32)),
    (instruction, num): (&str, i32),
) -> (i32, i32, (i32, i32)) {
    // println!("{:?}", (x, y, offset_x, offset_y));
    // println!("{:?}", (instruction, num));
    match instruction {
        "F" => (x + num * offset_x, y + num * offset_y, (offset_x, offset_y)),
        "L" => (x, y, rotate(Sign::Positive, num, offset_x, offset_y)),
        "R" => (x, y, rotate(Sign::Negative, num, offset_x, offset_y)),
        "N" => (x, y, (offset_x, offset_y + num)),
        "S" => (x, y, (offset_x, offset_y - num)),
        "E" => (x, y, (offset_x + num, offset_y)),
        "W" => (x, y, (offset_x - num, offset_y)),
        _ => panic!("invalid move."),
    }
}

pub fn solve(target: u8) {
    let contents = fs::read_to_string("data/day_12/input.txt").expect("error");
    let moves: Vec<String> = contents.lines().filter_map(|s| s.parse().ok()).collect();
    let re = Regex::new(r"^(\w)(\d+)").unwrap();
    let mapped = moves
        .iter()
        .map(|m| {
            if let Some(caps) = re.captures(m) {
                let instruction = caps.get(1).unwrap().as_str();
                let num = caps.get(2).unwrap().as_str().parse::<i16>().unwrap();
                (instruction, num)
            } else {
                panic!("bad capture.")
            }
        })
        .collect::<Vec<(&str, i16)>>();

    if target == 1 {
        let (x, y, _) = mapped.iter().fold((0, 0, 0), |acc, n| move_ship(acc, *n));
        println!("{}", (x as i16).abs() + (y as i16).abs())
    } else if target == 2 {
        let (x, y, _) = mapped.iter().fold((0, 0, (10, 1)), |acc, (ins, n)| {
            move_ship_waypoint(acc, (ins, *n as i32))
        });
        println!("{}", (x as i32).abs() + (y as i32).abs())
    };
}
