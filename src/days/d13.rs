use std::fs;

pub fn solve(target: u8) {
    let contents = fs::read_to_string("data/day_13/input.txt").expect("error");
    let lines: Vec<String> = contents.lines().filter_map(|s| s.parse().ok()).collect();
    let (start_time, buses): (u32, Vec<Option<u32>>) = (
        lines[0].parse::<u32>().unwrap(),
        lines[1]
            .split(",")
            .map(|s| s.parse::<u32>().ok())
            .collect(),
    );
    if target == 1 {
        let mut valid_buses: Vec<u32> = buses.iter().filter_map(|b| b.clone()).collect();
        valid_buses.sort_by(|a, b| (a - start_time % a).cmp(&(b - start_time % b)));
        let products: Vec<u32> = valid_buses.iter().map(|id| id * (id - start_time % id)).collect();
        println!("{}", products[0]);
    } else if target == 2 {
        let mut timestamp: u64 = 0;
        let mut factor: u64 = 1;
        let mut last_satisfied = 0;

        loop {
            let satisfied = buses.iter().enumerate().take_while(|(idx, b)| {
                match b {
                    None => true,
                    Some(id) => (timestamp + (*idx as u64)) % u64::from(*id) == 0,
                }
            });
            let count = satisfied.clone().count();

            if count == buses.len() {
                break;
            } else if count != last_satisfied {
                last_satisfied = count;
                factor = satisfied.filter(|(_, n)| n.is_some()).map(|(_, n)| n.unwrap() as u64).product();
            }

            timestamp += factor;
        }
        println!("{}", timestamp);
    }
}
