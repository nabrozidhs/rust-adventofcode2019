extern crate itertools;

use std::ops::Range;

use itertools::Itertools;

use crate::intcode::{IntCodeMachine, IntCodeMachineState};

fn _day07(instructions: &Vec<i64>, input: Range<i64>) -> i64 {
    let start: Vec<i64> = input.collect();
    let mut result = i64::min_value();
    for p in start.iter().permutations(start.len()) {
        let mut amplifiers = vec![
            IntCodeMachine::new(instructions),
            IntCodeMachine::new(instructions),
            IntCodeMachine::new(instructions),
            IntCodeMachine::new(instructions),
            IntCodeMachine::new(instructions)
        ];
        for i in 0..p.len() {
            amplifiers[i].push_input(*p[i]);
        }

        let mut signal = 0;
        'outer: loop {
            for i in 0..amplifiers.len() {
                amplifiers[i].push_input(signal);

                match amplifiers[i].run() {
                    IntCodeMachineState::Paused => signal = amplifiers[i].consume_output().unwrap(),
                    IntCodeMachineState::Finished => {
                        signal = amplifiers[i].consume_output().unwrap();
                        if i == amplifiers.len() - 1 {
                            break 'outer;
                        }
                    }
                }
            }
        }
        result = signal.max(result);
    }
    result
}

#[cfg(test)]
mod tests_day07 {
    use std::fs::File;
    use std::io::Read;

    use crate::day07::_day07;

    #[test]
    fn part1_test_input() {
        assert_eq!(
            43210,
            _day07(
                &vec![3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0],
                0..5,
            )
        );
        assert_eq!(
            54321,
            _day07(
                &vec![3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23,
                      101, 5, 23, 23, 1, 24, 23, 23, 4, 23, 99, 0, 0],
                0..5,
            )
        );
        assert_eq!(
            65210,
            _day07(
                &vec![3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33,
                      1002, 33, 7, 33, 1, 33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0],
                0..5,
            )
        );
    }

    #[test]
    fn part2_test_input() {
        assert_eq!(
            139629729,
            _day07(
                &vec![3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26,
                      27, 4, 27, 1001, 28, -1, 28, 1005, 28, 6, 99, 0, 0, 5],
                5..10,
            )
        );
        assert_eq!(
            18216,
            _day07(
                &vec![3, 52, 1001, 52, -5, 52, 3, 53, 1, 52, 56, 54, 1007, 54, 5, 55, 1005, 55, 26,
                      1001, 54, -5, 54, 1105, 1, 12, 1, 53, 54, 53, 1008, 54, 0, 55, 1001, 55, 1,
                      55, 2, 53, 55, 53, 4, 53, 1001, 56, -1, 56, 1005, 56, 6, 99, 0, 0, 0, 0, 10],
                5..10,
            )
        );
    }

    #[test]
    fn day07() {
        let mut file = File::open("data/day07.txt").unwrap();
        let mut buffer = String::new();
        file.read_to_string(&mut buffer).unwrap();

        let input: Vec<i64> = buffer.split(",")
            .map(|line| line.parse::<i64>().unwrap())
            .collect();

        assert_eq!(298586, _day07(&input, 0..5));
        assert_eq!(9246095, _day07(&input, 5..10));
    }
}
