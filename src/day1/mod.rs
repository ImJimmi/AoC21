use std::fs;

fn read_input() -> String {
    return fs::read_to_string("./src/day1/input.txt").unwrap();
}

fn count_increments(input: &str) -> i32 {
    let measurements = input.split("\n").collect::<Vec<&str>>();

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
        let output = count_increments(&String::new());
        assert_eq!(output, 0);
    }

    #[test]
    fn test_single_input() {
        let output = count_increments("123");
        assert_eq!(output, 0);
    }

    #[test]
    fn test_two_inputs_incrementing() {
        let output = count_increments("45\n67");
        assert_eq!(output, 1);
    }

    #[test]
    fn test_two_inputs_decrementing() {
        let output = count_increments("98\n76");
        assert_eq!(output, 0);
    }
}

pub fn sonar_sweep() {
    let measurements = read_input();
    let num_increments = count_increments(&measurements);
    println!("1.1: {}", num_increments);
}
