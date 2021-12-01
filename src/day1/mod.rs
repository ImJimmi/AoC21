use std::fs;

fn read_input() -> Vec<u32> {
    let contents = fs::read_to_string("./src/day1/input.txt").unwrap();
    let tokens = contents.trim().split("\n");
    return tokens.map(|token| token.parse::<u32>().unwrap()).collect();
}

fn count_increments(measurements: &Vec<u32>) -> u32 {

    if measurements.len() == 2 && measurements[1] > measurements[0] {
        return 1;
    }

    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_input() {
        let output = count_increments(&Vec::new());
        assert_eq!(output, 0);
    }

    #[test]
    fn test_single_input() {
        let input = [123].to_vec();
        let output = count_increments(&input);
        assert_eq!(output, 0);
    }

    #[test]
    fn test_two_inputs_incrementing() {
        let input = [45, 67].to_vec();
        let output = count_increments(&input);
        assert_eq!(output, 1);
    }

    #[test]
    fn test_two_inputs_decrementing() {
        let input = [98, 76].to_vec();
        let output = count_increments(&input);
        assert_eq!(output, 0);
    }
}

pub fn sonar_sweep() {
    let measurements = read_input();
    let num_increments = count_increments(&measurements);
    println!("1.1: {}", num_increments);
}
