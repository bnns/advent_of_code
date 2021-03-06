use std::env;
pub mod days;

#[macro_use]
extern crate lazy_static;

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
        "7.1" => days::d7::solve(1),
        "7.2" => days::d7::solve(2),
        "8.1" => days::d8::solve(1),
        "8.2" => days::d8::solve(2),
        "9.1" => days::d9::solve(1),
        "9.2" => days::d9::solve(2),
        "10.1" => days::d10::solve(1),
        "10.2" => days::d10::solve(2),
        "11.1" => days::d11::solve(1),
        "11.2" => days::d11::solve(2),
        "12.1" => days::d12::solve(1),
        "12.2" => days::d12::solve(2),
        "13.1" => days::d13::solve(1),
        "13.2" => days::d13::solve(2),
        "14.1" => days::d14::solve(1),
        "14.2" => days::d14::solve(2),
        "15.1" => days::d15::solve(1),
        "15.2" => days::d15::solve(2),
        "16.1" => days::d16::solve(1),
        "16.2" => days::d16::solve(2),
        "17.1" => days::d17::solve(1),
        "17.2" => days::d17::solve(2),
        "18.1" => days::d18::solve(1),
        "18.2" => days::d18::solve(2),
        "19.1" => days::d19::solve(1),
        "19.2" => days::d19::solve(2),
        _ => println!("bad input")
    }
}
