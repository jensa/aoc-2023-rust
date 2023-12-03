use std::{i32, collections::HashMap};

use crate::util;

pub fn solve() {
    let vals = util::input_lines_as_strings();
    let first_part = false;
    let max_vals: HashMap<&str, i32> = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);

    let mut total = 0;
    for line in vals {
        let game: Game = to_game(line);
        if first_part {
            if game.max_blue <= max_vals["blue"] && game.max_green <= max_vals["green"] && game.max_red <= max_vals["red"] {
                total += game.id;
            }
        } else {
            let power = game.max_blue * game.max_green * game.max_red;
            total += power;
        }

    }
    println!("{total}")

}

fn to_game(line:String) -> Game {
    let game_parts: Vec<String> = line.split(":").map(|f| f.to_string()).collect();

    let id = game_parts[0].split_whitespace().next_back().unwrap().to_i32();
    let draws: Vec<String> = game_parts[1].split(";").map(str::to_string).collect();

    let mut max_red = 0;
    let mut max_green = 0;
    let mut max_blue = 0;
    for draw in draws {
        let colors = draw.split(",");
        for color in colors {
            let amount = color.split_whitespace().next().unwrap().to_i32();
            if color.ends_with("red"){
                max_red = if amount > max_red { amount } else { max_red };
            }
            if color.ends_with("green"){
                max_green = if amount > max_green { amount } else { max_green };
            }
            if color.ends_with("blue"){
                max_blue = if amount > max_blue { amount } else { max_blue };
            }
        }
    }

    return Game {
        max_red,
        max_green,
        max_blue,
        id
    }
}

#[derive(Debug)]
struct Game {
    max_red: i32,
    max_green: i32,
    max_blue: i32,
    id:i32
}