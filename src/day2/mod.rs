use std::fs;

fn read_input() -> Vec<String> {
    let contents = fs::read_to_string("./src/day2/input.txt").unwrap();
    let tokens = contents.trim().split("\n");
    return tokens.map(|token| token.to_string()).collect();
}

pub fn dive() {
    let _commands = read_input();
    println!("2.1: {}", 0);
}
