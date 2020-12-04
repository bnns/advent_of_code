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
        _ => println!("bad input")
    }
}
