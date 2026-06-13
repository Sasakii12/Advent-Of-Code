
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

    let dice_game = split_line[1].split(";").collect::<Vec<&str>>();
    let seperated_game = dice_game.iter().map(|x| x.split(",").collect::<Vec<&str>>()).collect::<Vec<Vec<&str>>>();
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
        }
    }
    color_hash["red"] * color_hash["blue"] * color_hash["green"]
}

pub fn solve() {
    let mut x = 0;
    let mut x2 = 0;
    if let Ok(lines) = read_lines(2) {
        for line in lines.map_while(Result::ok) {
            x += part1(&line);
            x2 += part2(&line);
        }
    }
    
    println!("{}", x);
    println!("{}", x2);
    assert_eq!(x, 2176);
    assert_eq!(x2, 63700);
}
