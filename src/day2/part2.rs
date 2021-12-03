use super::position::Position;

pub fn follow_commands_from(_commands: &Vec<String>, initial_position: Position) -> Position {
    return Position { x: 0, y: 0 };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_input() {
        let output = follow_commands_from(&Vec::new(), Position { x: 0, y: 0 });
        assert_eq!(output, Position { x: 0, y: 0 });
    }
}
