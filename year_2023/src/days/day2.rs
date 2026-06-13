
use std::collections::HashMap;

use crate::utils::input::read_lines;

fn part1(line: &String) -> u32 {
    if line == "" {
        return 0
    }
    let split_line = line.split(":").collect::<Vec<&str>>();
    let game_id = split_line[0].split(" ").collect::<Vec<&str>>();

    let dice_game = split_line[1].split(";").collect::<Vec<&str>>();
    let seperated_game = dice_game.iter().map(|x| x.split(",").collect::<Vec<&str>>()).collect::<Vec<Vec<&str>>>();

    for dice_set in seperated_game {
        for pairing in dice_set {
            let x = pairing.trim().split(" ").collect::<Vec<&str>>();
            let x_num: u32 = x[0].parse().unwrap();
            if x[1] == "blue" && x_num > 14 {
                return 0
            } else if x[1] == "red" && x_num > 12 {
                return 0
            } else if x[1] == "green" && x_num > 13 {
                return 0
            }
        }
    }
    game_id[1].parse().unwrap()
}

fn part2(line: &String) -> u32 {
     if line == "" {
        return 0
    }
    let mut color_hash: HashMap<String, u32> = HashMap::new();
    let split_line = line.split(":").collect::<Vec<&str>>();
    let game_id = split_line[0].split(" ").collect::<Vec<&str>>();

    let dice_game = split_line[1].split(";").collect::<Vec<&str>>();
    let seperated_game = dice_game.iter().map(|x| x.split(",").collect::<Vec<&str>>()).collect::<Vec<Vec<&str>>>();
    let max_game: Vec<u32> = vec![];
    for dice_set in seperated_game {
        for pairing in dice_set {
            let x = pairing.trim().split(" ").collect::<Vec<&str>>();
            let game_num = x[0].parse().unwrap();
            if !color_hash.contains_key(x[1]) {
                color_hash.insert(x[1].to_string(), game_num);
            }

            if color_hash[x[1]] < game_num {
                color_hash.insert(x[1].to_string(), game_num);
            }
            println!("{:?}", color_hash);
        }
    }
    color_hash["red"] * color_hash["blue"] * color_hash["green"]
}

pub fn solve() {
    let test_case1 = vec![
    "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
    "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
    "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
    "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
    "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
    ];
    let mut x = 0;
    let mut x2 = 0;
    if let Ok(lines) = read_lines(2) {
        for line in lines.map_while(Result::ok) {
            x += part1(&line);
            x2 += part2(&line);
        }
    }
    // println!("{}", x);

    println!("{}", x2);
}
