use std::env;
pub mod days;

fn main() {
    let args: Vec<String> = env::args().collect();
    let day = &args[1];
    match &day[..] {
        "1.1" => days::d1::solve_1(),
        "1.2" => days::d1::solve_2(),
        "2.1" => days::d2::solve(1),
        "2.2" => days::d2::solve(2),
        "3.1" => days::d3::solve([(1, 3)].to_vec()),
        "3.2" => days::d3::solve([(1, 3), (1, 1), (1, 5), (1, 7), (2, 1)].to_vec()),
        "4.1" => days::d4::solve(1),
        "4.2" => days::d4::solve(2),
        "5.1" => days::d5::solve(1),
        "5.2" => days::d5::solve(2),
        "6.1" => days::d6::solve(1),
        "6.2" => days::d6::solve(2),
        _ => println!("bad input")
    }
}
