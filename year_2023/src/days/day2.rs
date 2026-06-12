
use std::collections::HashMap;

fn part1(line: &String) -> u32 {
    let split_line = line.split(":").collect::<Vec<&str>>();
    let game_id = split_line[0].split(" ").collect::<Vec<&str>>();

    let dice_game = split_line[1].split(";").collect::<Vec<&str>>();
    let seperated_game = dice_game.iter().map(|x| x.split(",").collect::<Vec<&str>>()).collect::<Vec<Vec<&str>>>();
    println!("{:?}", seperated_game);

    for dice_set in seperated_game {
        for pairing in dice_set {
            let x = pairing.trim().split(" ").collect::<Vec<&str>>();
            let x_num: u32 = x[0].parse().unwrap();
            println!("{:?}", x);
            println!("{}", x[1] == "blue");
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

pub fn solve() {
    let test_case1 = vec![
    "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
    "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
    "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
    "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
    "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
    ];
    println!("Temp");
    println!("{}", part1(&test_case1[0].to_string()));
}
