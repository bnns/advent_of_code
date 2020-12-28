use std::collections::HashMap;
use std::fs;
use std::hash::{Hash, Hasher};
use std::ops::Add;

#[derive(Copy, Clone)]
struct Point(i16, i16, i16);

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1 && self.2 == other.2
    }
}

impl Eq for Point {}

impl Hash for Point {
    fn hash<H: Hasher>(&self, state: &mut H) {
        format!("{},{},{}", self.0, self.1, self.2).hash(state)
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

fn populate(cubes: &mut HashMap<Point, bool>, neighbors: &Vec<Point>) {
    for (pos, _) in cubes
        .clone()
        .iter()
        .filter(|(_, active)| **active)
        .collect::<Vec<(&Point, &bool)>>()
    {
        for n in neighbors.iter() {
            let new_cube = *pos + *n;
            cubes.entry(new_cube).or_insert(false);
        }
    }
}

fn cycle(cubes: &mut HashMap<Point, bool>, neighbors: &Vec<Point>) {
    let prev = cubes.clone();
    for (pos, active) in prev.iter() {
        let mut active_neighbors = 0;
        for n in neighbors.iter() {
            let new_cube = *pos + *n;
            if let Some(true) = prev.get(&new_cube) {
                active_neighbors += 1;
            };
        }
        let new_val = match active_neighbors {
            2 if *active => true,
            3 if *active => true,
            3 if !*active => true,
            _ => false,
        };

        cubes.insert(*pos, new_val);
    }
}

fn show(cubes: &HashMap<Point, bool>) {
    for z in -2..2 {
        println!("z = {}", z);
        for y in -10..10 {
            for x in -10..10 {
                let p = match cubes.get(&Point(x, y, z)) {
                    Some(true) => "#",
                    Some(false) => ".",
                    None => " ",
                };
                print!("{}", p);
            }
            print!("\n");
        }
    }
}

pub fn solve(_target: u8) {
    let contents = fs::read_to_string("data/day_17/input.txt").expect("error");
    let data: Vec<String> = contents
        .lines()
        .filter_map(|s| s.parse::<String>().ok())
        .collect();
    // let data = vec![".#.", "..#", "###"];
    let mut cubes: HashMap<Point, bool> =
        data.iter()
            .enumerate()
            .fold(HashMap::new(), |mut acc, (y, line)| {
                let new_cubes = line
                    .split_terminator("")
                    .skip(1)
                    .enumerate()
                    .map(|(x, c)| {
                        if c == "#" {
                            (Point(x as i16, y as i16, 0), true)
                        } else {
                            (Point(x as i16, y as i16, 0), false)
                        }
                    })
                    .collect::<HashMap<Point, bool>>();
                acc.extend(new_cubes);
                acc
            });
    let neighbors: Vec<Point> = [
        (-1, -1, -1),
        (-1, -1, 0),
        (-1, -1, 1),
        (-1, 0, -1),
        (-1, 0, 0),
        (-1, 0, 1),
        (-1, 1, -1),
        (-1, 1, 0),
        (-1, 1, 1),
        (0, -1, -1),
        (0, -1, 0),
        (0, -1, 1),
        (0, 0, -1),
        (0, 0, 1),
        (0, 1, -1),
        (0, 1, 0),
        (0, 1, 1),
        (1, -1, -1),
        (1, -1, 0),
        (1, -1, 1),
        (1, 0, -1),
        (1, 0, 0),
        (1, 0, 1),
        (1, 1, -1),
        (1, 1, 0),
        (1, 1, 1),
    ]
    .iter()
    .map(|(x, y, z)| Point(*x as i16, *y as i16, *z as i16))
    .collect::<Vec<Point>>();

    for step in 0..6 {
        println!("========= {} ==============", step);
        show(&cubes);
        println!("===========================");
        populate(&mut cubes, &neighbors);
        cycle(&mut cubes, &neighbors);
    }
    let answer = cubes.iter().filter(|(_, active)| **active).count();
    println!("{}", answer);
}
