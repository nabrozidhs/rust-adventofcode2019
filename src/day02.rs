use crate::intcode::IntCodeMachine;

fn _day02_part1(input: &Vec<i64>) -> i64 {
    let mut virtual_machine = IntCodeMachine::new(input);

    virtual_machine.run();

    virtual_machine.code[0]
}

fn _day02_part2(input: &Vec<i64>, expected: i64) -> i64 {
    let mut program = input.clone();
    for noun in 0..99 {
        for verb in 0..99 {
            program[1] = noun;
            program[2] = verb;
            if _day02_part1(&program) == expected {
                return 100 * noun + verb;
            }
        }
    }
    panic!()
}

#[cfg(test)]
mod tests_day02 {
    use std::fs::File;
    use std::io::Read;

    use crate::day02::{_day02_part1, _day02_part2};

    #[test]
    fn part1_test_input() {
        assert_eq!(2, _day02_part1(&vec![1, 0, 0, 0, 99]));
        assert_eq!(2, _day02_part1(&vec![2, 3, 0, 3, 99]));
        assert_eq!(2, _day02_part1(&vec![2, 4, 4, 5, 99, 0]));
        assert_eq!(30, _day02_part1(&vec![1, 1, 1, 4, 99, 5, 6, 0, 99]));
    }

    #[test]
    fn day02() {
        let mut file = File::open("data/day02.txt").unwrap();
        let mut buffer = String::new();
        file.read_to_string(&mut buffer).unwrap();

        let mut input: Vec<i64> = buffer.split(",")
            .map(|line| line.parse::<i64>().unwrap())
            .collect();

        input[1] = 12;
        input[2] = 2;
        assert_eq!(3931283, _day02_part1(&input));
        assert_eq!(6979, _day02_part2(&input, 19690720));
    }
}
