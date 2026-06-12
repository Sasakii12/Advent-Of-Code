use std::fs::read;

use crate::utils::input::read_lines;

fn part1(line: String) -> u32 {
    if line == "" {
        return 0
    }
    let mut num_list: Vec<char> = vec![];

    for num in line.chars() {
        if num.is_numeric() {
            num_list.push(num);
        }
    }

    format!("{}{}", num_list[0], num_list[num_list.len() - 1]).parse().expect("Error parsing numbers")
}

pub fn solve() {
    let test_case = vec![
        "1abc2",
        "pqr3stu8vwx",
        "a1b2c3d4e5f",
        "treb7uchet"
    ];

    let mut sum1: Vec<u32> = vec![];
    let mut sum2: Vec<u32> = vec![];

    if let Ok(lines) = read_lines(1) {
        for line in lines.map_while(Result::ok) {
            sum1.push(part1(line));
        }
    }
    assert_eq!(sum1.iter().sum::<u32>(), 54605);
}
