use crate::intcode::IntCodeMachine;

fn _day09(instructions: &Vec<i64>, input: &Vec<i64>) -> i64 {
    let mut machine = IntCodeMachine::new_with_input(instructions, input);
    machine.run();
    *machine.output.last().unwrap()
}

#[cfg(test)]
mod tests_day09 {
    use std::fs::File;
    use std::io::Read;

    use crate::day09::_day09;

    #[test]
    fn part1_test_input() {
        assert_eq!(99, _day09(&vec![109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99], &vec![]));
        assert_eq!(1219070632396864, _day09(&vec![1102, 34915192, 34915192, 7, 4, 7, 99, 0], &vec![]));
        assert_eq!(1125899906842624, _day09(&vec![104, 1125899906842624, 99], &vec![]));
    }

    #[test]
    fn day09() {
        let mut file = File::open("data/day09.txt").unwrap();
        let mut buffer = String::new();
        file.read_to_string(&mut buffer).unwrap();

        let input: Vec<i64> = buffer.split(",")
            .map(|line| line.parse::<i64>().unwrap())
            .collect();

        assert_eq!(3638931938, _day09(&input, &vec![1]));
        assert_eq!(86025, _day09(&input, &vec![2]));
    }
}
