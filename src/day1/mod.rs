use std::cmp;
use std::fs;

fn read_input() -> Vec<u32> {
    let contents = fs::read_to_string("./src/day1/input.txt").unwrap();
    let tokens = contents.trim().split("\n");
    return tokens.map(|token| token.parse::<u32>().unwrap()).collect();
}

fn get_nth(measurements: &Vec<u32>, window_size: usize, n: usize) -> u32 {
    let start = n;
    let stop = cmp::min(measurements.len(), n + window_size);
    return measurements[start..stop].iter().sum();
}

fn count_increments(measurements: &Vec<u32>, window_size: usize) -> u32 {
    if measurements.len() > window_size + 1 {
        return count_increments(&measurements[0..2].to_vec(), window_size)
            + count_increments(&measurements[1..measurements.len()].to_vec(), window_size);
    }

    if measurements.len() == window_size + 1 {
        let first = get_nth(measurements, window_size, 0);
        let second = get_nth(measurements, window_size, 1);

        if second > first {
            return 1;
        }
    }

    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_input() {
        let output = count_increments(&Vec::new(), 1);
        assert_eq!(output, 0);
    }

    #[test]
    fn test_single_input() {
        let input = [123].to_vec();
        let output = count_increments(&input, 1);
        assert_eq!(output, 0);
    }

    #[test]
    fn test_two_inputs_incrementing() {
        let input = [45, 67].to_vec();
        let output = count_increments(&input, 1);
        assert_eq!(output, 1);
    }

    #[test]
    fn test_two_inputs_decrementing() {
        let input = [98, 76].to_vec();
        let output = count_increments(&input, 1);
        assert_eq!(output, 0);
    }

    #[test]
    fn test_three_inputs_incrementing() {
        let input = [1, 2, 3].to_vec();
        let output = count_increments(&input, 1);
        assert_eq!(output, 2);
    }

    #[test]
    fn test_empty_input_window_size_3() {
        let input = Vec::new();
        let output = count_increments(&input, 3);
        assert_eq!(output, 0);
    }

    #[test]
    fn test_two_inputs_window_size_3() {
        let input = [100, 300].to_vec();
        let output = count_increments(&input, 3);
        assert_eq!(output, 0);
    }

    #[test]
    fn test_four_inputs_window_size_3() {
        let input = [37, 47, 73, 74].to_vec();
        let output = count_increments(&input, 3);
        assert_eq!(output, 1);
    }
}

pub fn sonar_sweep() {
    let measurements = read_input();

    let num_increments_window_size_1 = count_increments(&measurements, 1);
    println!("1.1: {}", num_increments_window_size_1);

    let num_increments_window_size_3 = count_increments(&measurements, 3);
    println!("1.2: {}", num_increments_window_size_3);
}
