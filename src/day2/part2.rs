use super::position::Position;

fn get_displacement_for_command(command: &String, aim: i32) -> Position {
    let delta = command[command.len() - 1..].parse::<i32>().unwrap();

    match &command[0..1] {
        "f" => {
            return Position {
                x: delta,
                y: aim * delta,
            }
        }
        _ => return Position { x: 0, y: 0 },
    }
}

fn get_delta_aim_for_command(command: &String) -> i32 {
    let delta = command[command.len() - 1..].parse::<i32>().unwrap();

    match &command[0..1] {
        "d" => return delta,
        "u" => return -delta,
        _ => return 0,
    }
}

pub fn follow_commands_from(
    commands: &Vec<String>,
    initial_position: Position,
    aim: i32,
) -> Position {
    if commands.len() == 0 {
        return initial_position;
    }

    let new_aim = aim + get_delta_aim_for_command(&commands[0]);
    let displacement = get_displacement_for_command(&commands[0], new_aim);

    let new_position = Position {
        x: initial_position.x + displacement.x,
        y: initial_position.y + displacement.y,
    };

    if commands.len() == 1 {
        return new_position;
    }

    return follow_commands_from(&commands[1..commands.len()].to_vec(), new_position, new_aim);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_input() {
        let output = follow_commands_from(&Vec::new(), Position { x: 0, y: 0 }, 0);
        assert_eq!(output, Position { x: 0, y: 0 });
    }

    #[test]
    fn test_single_forward_input() {
        let input = ["forward 3".to_string()].to_vec();
        let output = follow_commands_from(&input, Position { x: 0, y: 0 }, 0);
        assert_eq!(output, Position { x: 3, y: 0 });
    }

    #[test]
    fn test_single_down_input() {
        let input = ["down 6".to_string()].to_vec();
        let output = follow_commands_from(&input, Position { x: 13, y: 0 }, 0);
        assert_eq!(output, Position { x: 13, y: 0 });
    }

    #[test]
    fn test_single_up_input() {
        let input = ["up 4".to_string()].to_vec();
        let output = follow_commands_from(&input, Position { x: 0, y: 10 }, 0);
        assert_eq!(output, Position { x: 0, y: 10 });
    }

    #[test]
    fn test_down_then_forward() {
        let input = ["down 3".to_string(), "forward 5".to_string()].to_vec();
        let output = follow_commands_from(&input, Position { x: 0, y: 0 }, 0);
        assert_eq!(output, Position { x: 5, y: 15 });
    }
}
