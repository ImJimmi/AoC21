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

fn get_displacement_for_command(command: &String) -> Position {
    let delta = command[command.len() - 1..].parse::<i32>().unwrap();

    match &command[0..1] {
        "f" => return Position { x: delta, y: 0 },
        "d" => return Position { x: 0, y: delta },
        "u" => return Position { x: 0, y: -delta },
        _ => return Position { x: 0, y: 0 },
    }
}

fn follow_commands_from(commands: &Vec<String>, initial_position: Position) -> Position {
    if commands.len() == 0 {
        return initial_position;
    }

    let displacement = get_displacement_for_command(&commands[0]);
    let new_position = Position {
        x: initial_position.x + displacement.x,
        y: initial_position.y + displacement.y,
    };

    if commands.len() == 1 {
        return new_position;
    }

    return follow_commands_from(&commands[1..commands.len()].to_vec(), new_position);
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

    #[test]
    fn test_single_forward_command() {
        let input = ["forward 5".to_string()].to_vec();
        let output = follow_commands_from(&input, Position { x: 0, y: 0 });
        assert_eq!(output, Position { x: 5, y: 0 });
    }

    #[test]
    fn test_single_down_command() {
        let input = ["down 3".to_string()].to_vec();
        let output = follow_commands_from(&input, Position { x: 0, y: 0 });
        assert_eq!(output, Position { x: 0, y: 3 });
    }

    #[test]
    fn test_single_up_command() {
        let input = ["up 9".to_string()].to_vec();
        let output = follow_commands_from(&input, Position { x: 0, y: 13 });
        assert_eq!(output, Position { x: 0, y: 4 });
    }

    #[test]
    fn test_two_forward_commands() {
        let input = ["forward 3".to_string(), "forward 6".to_string()].to_vec();
        let output = follow_commands_from(&input, Position { x: 0, y: 0 });
        assert_eq!(output, Position { x: 9, y: 0 });
    }
}

pub fn dive() {
    let commands = read_input();
    let final_position = follow_commands_from(&commands, Position { x: 0, y: 0 });
    println!("2.1: {:?}", final_position);
}
