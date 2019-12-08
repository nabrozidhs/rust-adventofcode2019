use crate::intcode::IntCodeMachine;

fn _day05(program: &Vec<i64>, input: i64) -> Option<i64> {
    let mut virtual_machine = IntCodeMachine::new_with_input(program, &vec![input]);

    virtual_machine.run();

    virtual_machine.output.last().cloned()
}

#[cfg(test)]
mod tests_day05 {
    use std::fs::File;
    use std::io::Read;

    use crate::day05::_day05;

    #[test]
    fn part1_test_input() {
        assert_eq!(None, _day05(&vec![1101, 100, -1, 4, 0], 1));
    }

    #[test]
    fn part2_test_input() {
        let input = vec![3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31,
                         1106, 0, 36, 98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104,
                         999, 1105, 1, 46, 1101, 1000, 1, 20, 4, 20, 1105, 1, 46, 98, 99];
        assert_eq!(Some(999), _day05(&input, 7));
        assert_eq!(Some(1000), _day05(&input, 8));
        assert_eq!(Some(1001), _day05(&input, 9));
    }

    #[test]
    fn day05() {
        let mut file = File::open("data/day05.txt").unwrap();
        let mut buffer = String::new();
        file.read_to_string(&mut buffer).unwrap();

        let input: Vec<i64> = buffer.split(",")
            .map(|line| line.parse::<i64>().unwrap())
            .collect();

        assert_eq!(Some(6069343), _day05(&input.clone(), 1));
        assert_eq!(Some(3188550), _day05(&input.clone(), 5));
    }
}
