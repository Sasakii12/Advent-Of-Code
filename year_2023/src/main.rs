mod days;
mod utils;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let day = if args.len() > 1 {
        args[1].parse::<u8>().expect("Please provide a valid day number")
    } else {
        println!("Usage: cargo run -- <day>");
        return;
    };

    match day {
        1 => days::day1::solve(),
        _ => println!("Day not implemented"),
    }
}
