use std::collections::HashMap;

use crate::utils::input::read_lines;


fn part1(line: &String) -> u32 {
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

fn part2(line: &String, hash: &HashMap<&str, char>) -> u32 {

    if line == "" {
        return 0
    }

    let mut num_list: Vec<char> = vec![];
    let mut string_stack = String::new();

    for num in line.chars() {
        if num.is_numeric() {
            num_list.push(num);
        } else {
            string_stack.push(num);
            for key in hash.keys() {
                if string_stack.contains(key) {
                    num_list.push(hash[key]);
                    string_stack = String::from(string_stack.chars().last().unwrap());
                }
            }
        }

    }
    format!("{}{}", num_list[0], num_list[num_list.len() - 1]).parse().expect("Error parsing numbers")
}

pub fn solve() {
    let num_map: HashMap<&str, char> = HashMap::from([
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9'),
    ]);

    let mut sum1: Vec<u32> = vec![];
    let mut sum2: Vec<u32> = vec![];

    if let Ok(lines) = read_lines(1) {
        for line in lines.map_while(Result::ok) {
            sum1.push(part1(&line));
            sum2.push(part2(&line, &num_map));
        }
    }

    assert_eq!(sum1.iter().sum::<u32>(), 54605);
    assert_eq!(sum2.iter().sum::<u32>(), 55429);

    println!("{}", sum1.iter().sum::<u32>());
    println!("{}", sum2.iter().sum::<u32>());
}
