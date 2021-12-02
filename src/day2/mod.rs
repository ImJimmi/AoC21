use std::fs;

fn read_input() -> Vec<String> {
    let contents = fs::read_to_string("./src/day2/input.txt").unwrap();
    let tokens = contents.trim().split("\n");
    return tokens.map(|token| token.to_string()).collect();
}

#[derive(Debug, PartialEq)]
struct Position {
    pub x: i32,
    pub y: i32,
}

fn follow_commands_from(_commands: &Vec<String>, initial_position: Position) -> Position {
    return initial_position;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zero_commands_from_0_0() {
        let output = follow_commands_from(&Vec::new(), Position { x: 0, y: 0 });
        assert_eq!(output, Position { x: 0, y: 0 });
    }

    #[test]
    fn test_zero_commands_from_3_2() {
        let output = follow_commands_from(&Vec::new(), Position { x: 3, y: 2 });
        assert_eq!(output, Position { x: 3, y: 2 });
    }
}

pub fn dive() {
    let commands = read_input();
    let final_position = follow_commands_from(&commands, Position { x: 0, y: 0 });
    println!("2.1: {:?}", final_position);
}
