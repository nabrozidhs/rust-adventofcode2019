use crate::intcode::IntCodeMachine;
use crate::intcode::IntCodeMachineState::Finished;
use crate::util::Vector2;
use std::collections::HashMap;

fn _day11(instructions: &Vec<i64>, input: &Vec<i64>, print: bool) -> u64 {
    let mut machine = IntCodeMachine::new_with_input(instructions, input);
    let mut painted: HashMap<Vector2, i64> = HashMap::new();
    let mut pos = Vector2::new(0, 0);
    let mut direction = Vector2::new(0, 1);
    let mut bottom_left = Vector2::new(0, 0);
    let mut top_right = Vector2::new(0, 0);
    while machine.run() != Finished {
        painted.insert(pos, machine.consume_output().unwrap());
        let next = if machine.consume_output().unwrap() == 0 { -1 } else { 1 };
        direction = Vector2::new(direction.1 * next, -direction.0 * next);
        pos += direction;
        machine.push_input(*painted.get(&pos).unwrap_or(&0));

        bottom_left.0 = pos.0.min(bottom_left.0);
        bottom_left.1 = pos.1.min(bottom_left.1);
        top_right.0 = pos.0.max(top_right.0);
        top_right.1 = pos.1.max(top_right.1);
    }

    if print {
        for y in (bottom_left.1..=top_right.1).rev() {
            for x in bottom_left.0..=top_right.0 {
                if *painted.get(&Vector2::new(x, y)).unwrap_or(&0) == 1 {
                    print!("#");
                } else {
                    print!(" ");
                }
            }
            println!();
        }
    }

    painted.len() as u64
}

#[cfg(test)]
mod tests_day11 {
    use std::fs::File;
    use std::io::Read;

    use crate::day11::_day11;

    #[test]
    fn day11() {
        let mut file = File::open("data/day11.txt").unwrap();
        let mut buffer = String::new();
        file.read_to_string(&mut buffer).unwrap();

        let input: Vec<i64> = buffer.split(",")
            .map(|line| line.parse::<i64>().unwrap())
            .collect();

        assert_eq!(2469, _day11(&input, &vec![0], false));
        _day11(&input, &vec![1], true);
    }
}
