use std::fs;

fn read_input() -> String {
    return fs::read_to_string("./src/day1/input.txt").unwrap();
}

fn count_increments(_measurments: &str) -> i32 {
    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_input() {
        let output = count_increments(&String::new());
        assert_eq!(output, 0);
    }
}

pub fn sonar_sweep() {
    let measurments = read_input();
    let num_increments = count_increments(&measurments);
    println!("1.1: {}", num_increments);
}
