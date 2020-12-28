use std::collections::HashMap;
use std::fs;
use std::hash::{Hash, Hasher};
use std::ops::Add;

#[derive(Copy, Clone, Debug)]
struct Point(i16, i16, i16, i16);

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1 && self.2 == other.2 && self.3 == other.3
    }
}

impl Eq for Point {}

impl Hash for Point {
    fn hash<H: Hasher>(&self, state: &mut H) {
        format!("{},{},{},{}", self.0, self.1, self.2, self.3).hash(state)
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self(
            self.0 + other.0,
            self.1 + other.1,
            self.2 + other.2,
            self.3 + other.3,
        )
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
            let new_pos = *pos + *n;
            if new_pos != *pos {
                cubes.entry(new_pos).or_insert(false);
            }
        }
    }
}

fn cycle(cubes: &mut HashMap<Point, bool>, neighbors: &Vec<Point>) {
    let prev = cubes.clone();
    for (pos, active) in prev.iter() {
        let mut active_neighbors = 0;
        for n in neighbors.iter() {
            let new_pos = *pos + *n;
            if new_pos != *pos {
                if let Some(true) = prev.get(&new_pos) {
                    active_neighbors += 1;
                }
            }
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
    for w in -2..2 {
        println!("w = {}", w);
        for z in -2..2 {
            println!("z = {}", z);
            for y in -10..10 {
                for x in -10..10 {
                    let p = match cubes.get(&Point(x, y, z, w)) {
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
}

fn generate_neighbors(neighbors: &mut Vec<Point>, partial: Vec<i16>, places: usize) {
    if partial.len() == places {
        // could use a macro here
        if places == 3 {
            neighbors.push(Point(partial[0], partial[1], partial[2], 0));
        } else if places == 4 {
            neighbors.push(Point(partial[0], partial[1], partial[2], partial[3]));
        }
        return;
    }

    for p in -1..2 {
        let mut new_partial = partial.clone();
        new_partial.push(p);
        generate_neighbors(neighbors, new_partial, places);
    }
}

pub fn solve(target: u8) {
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
                            (Point(x as i16, y as i16, 0, 0), true)
                        } else {
                            (Point(x as i16, y as i16, 0, 0), false)
                        }
                    })
                    .collect::<HashMap<Point, bool>>();
                acc.extend(new_cubes);
                acc
            });

    let mut neighbors: Vec<Point> = vec![];

    if target == 1 {
        generate_neighbors(&mut neighbors, vec![], 3);
    } else if target == 2 {
        generate_neighbors(&mut neighbors, vec![], 4);
    }

    for _step in 0..6 {
        // println!("========= {} ==============", step);
        // show(&cubes);
        // println!("===========================");
        populate(&mut cubes, &neighbors);
        cycle(&mut cubes, &neighbors);
    }
    let answer = cubes.iter().filter(|(_, active)| **active).count();
    println!("{}", answer);
}
