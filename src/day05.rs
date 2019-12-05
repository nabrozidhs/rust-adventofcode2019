fn _get_value_from_instruction(program: &Vec<i64>, idx: usize, argument_position: usize) -> i64 {
    if program[idx] / (10 * 10_i64.pow(argument_position as u32)) % 10 == 1 {
        program[idx + argument_position]
    } else {
        program[program[idx + argument_position] as usize]
    }
}

fn _day05(instructions: &Vec<i64>, input: i64) -> i64 {
    let mut program = instructions.clone();
    let mut idx = 0;
    let mut output = 0;
    while program[idx] != 99 {
        match program[idx] % 10 {
            1 => {
                let output_idx = program[idx + 3] as usize;
                let a = _get_value_from_instruction(&program, idx, 1);
                let b = _get_value_from_instruction(&program, idx, 2);
                program[output_idx] = a + b;
                idx += 4;
            }
            2 => {
                let output_idx = program[idx + 3] as usize;
                let a = _get_value_from_instruction(&program, idx, 1);
                let b = _get_value_from_instruction(&program, idx, 2);
                program[output_idx] = a * b;
                idx += 4;
            }
            3 => {
                let output_idx = program[idx + 1] as usize;
                program[output_idx] = input;
                idx += 2;
            }
            4 => {
                output = _get_value_from_instruction(&program, idx, 1);
                idx += 2;
            }
            5 => {
                let a = _get_value_from_instruction(&program, idx, 1);
                idx = if a != 0 {
                    _get_value_from_instruction(&program, idx, 2) as usize
                } else {
                    idx + 3
                }
            }
            6 => {
                let a = _get_value_from_instruction(&program, idx, 1);
                idx = if a == 0 {
                    _get_value_from_instruction(&program, idx, 2) as usize
                } else {
                    idx + 3
                }
            }
            7 => {
                let a = _get_value_from_instruction(&program, idx, 1);
                let b = _get_value_from_instruction(&program, idx, 2);
                let output_idx = program[idx + 3] as usize;
                program[output_idx] = if a < b { 1 } else { 0 };
                idx += 4;
            }
            8 => {
                let a = _get_value_from_instruction(&program, idx, 1);
                let b = _get_value_from_instruction(&program, idx, 2);
                let output_idx = program[idx + 3] as usize;
                program[output_idx] = if a == b { 1 } else { 0 };
                idx += 4;
            }
            _ => panic!()
        }
    }
    output
}


#[cfg(test)]
mod tests_day05 {
    use std::fs::File;
    use std::io::Read;

    use crate::day05::_day05;

    #[test]
    fn part1_test_input() {
        assert_eq!(0, _day05(&vec![1101, 100, -1, 4, 0], 1));
    }

    #[test]
    fn part2_test_input() {
        let input = vec![3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31,
                         1106, 0, 36, 98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104,
                         999, 1105, 1, 46, 1101, 1000, 1, 20, 4, 20, 1105, 1, 46, 98, 99];
        assert_eq!(999, _day05(&input, 7));
        assert_eq!(1000, _day05(&input, 8));
        assert_eq!(1001, _day05(&input, 9));
    }

    #[test]
    fn day05() {
        let mut file = File::open("data/day05.txt").unwrap();
        let mut buffer = String::new();
        file.read_to_string(&mut buffer).unwrap();

        let input: Vec<i64> = buffer.split(",")
            .map(|line| line.parse::<i64>().unwrap())
            .collect();

        assert_eq!(6069343, _day05(&input, 1));
        assert_eq!(3188550, _day05(&input, 5));
    }
}
