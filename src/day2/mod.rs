mod part1;
mod position;

use position::Position;
use std::fs;

fn read_input() -> Vec<String> {
    let contents = fs::read_to_string("./src/day2/input.txt").unwrap();
    let tokens = contents.trim().split("\n");
    return tokens.map(|token| token.to_string()).collect();
}

pub fn dive() {
    let commands = read_input();

    let final_position = part1::follow_commands_from(&commands, Position { x: 0, y: 0 });
    println!("2.1: {:?}", final_position.x * final_position.y);
}
