use crate::intcode::IntCodeMachine;
use crate::util::Vector2;

fn _opposite_direction(direction: i64) -> i64 {
    match direction {
        1 => 2,
        2 => 1,
        3 => 4,
        4 => 3,
        _ => panic!(),
    }
}

fn _backtrack(input: &Vec<i64>) -> Vec<i64> {
    let mut output = vec![];

    for i in (0..input.len()).rev() {
        output.push(_opposite_direction(input[i]));
    }

    output
}

fn _position_for_input(input: &Vec<i64>) -> Vector2 {
    let mut v = Vector2::new(0, 0);

    for i in input {
        match i {
            1 => v.1 -= 1,
            2 => v.1 += 1,
            3 => v.0 -= 1,
            4 => v.0 += 1,
            _ => panic!(),
        }
    }

    v
}

fn _move_to(machine_code: &mut IntCodeMachine, position: i64) -> i64 {
    machine_code.push_input(position);
    machine_code.run();
    machine_code.consume_output().unwrap()
}

fn _should_skip(next: &Vec<i64>, i: i64) -> bool {
    let last = *next.last().unwrap_or(&-1);
    (last == 1 && i == 2) || (last == 2 && i == 1) || (last == 4 && i == 3) || (last == 3 && i == 4)
}

fn _part1(mut machine_code: &mut IntCodeMachine) -> u64 {
    let mut queue: Vec<Vec<i64>> = vec![vec![]];

    loop {
        let mut next = queue.remove(0);
        for n in next.iter() {
            _move_to(machine_code, *n);
        }

        for i in 1..=4 {
            if _should_skip(&next, i) {
                continue;
            }

            next.push(i);
            match _move_to(&mut machine_code, i) {
                1 => {
                    queue.push(next.clone());
                    _move_to(&mut machine_code, _opposite_direction(i));
                }
                2 => return next.len() as u64,
                0 => {}
                _ => panic!(),
            }
            next.pop();
        }

        for n in _backtrack(&next) {
            _move_to(&mut machine_code, n);
        }
    }
}

fn _day15(instructions: &Vec<i64>) -> (u64, u64) {
    let mut machine_code = IntCodeMachine::new(instructions);

    let part1 = _part1(&mut machine_code);

    let mut queue: Vec<Vec<i64>> = vec![vec![]];
    let mut distance = 0;
    while !queue.is_empty() {
        let mut next = queue.remove(0);
        for n in next.iter() {
            _move_to(&mut machine_code, *n);
        }

        for i in 1..=4 {
            if _should_skip(&next, i) {
                continue;
            }
            next.push(i);
            match _move_to(&mut machine_code, i) {
                1 | 2 => {
                    distance = next.len() as u64;
                    queue.push(next.clone());
                    _move_to(&mut machine_code, _opposite_direction(i));
                }
                0 => {}
                _ => panic!(),
            }
            next.pop();
        }

        for n in _backtrack(&next) {
            _move_to(&mut machine_code, n);
        }
    }

    return (part1, distance);
}

#[cfg(test)]
mod tests_day15 {
    use std::fs::File;
    use std::io::Read;

    use crate::day15::_day15;

    #[test]
    fn day15() {
        let mut file = File::open("data/day15.txt").unwrap();
        let mut buffer = String::new();
        file.read_to_string(&mut buffer).unwrap();

        let input: Vec<i64> = buffer.split(",")
            .map(|line| line.parse::<i64>().unwrap())
            .collect();

        let (part1, part2) = _day15(&input);
        assert_eq!(296, part1);
        assert_eq!(302, part2);
    }
}
