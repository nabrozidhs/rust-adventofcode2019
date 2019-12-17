use std::collections::HashSet;

use crate::intcode::IntCodeMachine;
use crate::util::Vector2;

fn _possible_directions(direction: &Vector2) -> Vec<Vector2> {
    if direction.1 != 0 {
        vec![Vector2::new(1, 0), Vector2::new(-1, 0)]
    } else {
        vec![Vector2::new(0, 1), Vector2::new(0, -1)]
    }
}

fn _day17_part1(instructions: &Vec<i64>) -> u64 {
    let mut machine_code = IntCodeMachine::new(instructions);

    machine_code.run();

    let mut walls: HashSet<Vector2> = HashSet::new();
    let mut pos: Vector2 = Vector2::new(0, 0);
    let mut start_location: Vector2 = Vector2::new(0, 0);

    while let Some(o) = machine_code.consume_output() {
        let c = char::from(o as u8);

        match c {
            '#' => {
                walls.insert(pos.clone());
                pos.0 += 1;
            }
            '.' => pos.0 += 1,
            '\n' => {
                pos.1 += 1;
                pos.0 = 0;
            }
            '^' => {
                start_location = pos.clone();
                walls.insert(pos.clone());
                pos.0 += 1;
            }
            _ => panic!("unexpected {}", c),
        }
    }

    let mut visited: HashSet<Vector2> = HashSet::new();
    let mut current_position = start_location;
    let mut direction = Vector2::new(1, 0);
    let mut part1 = 0;
    loop {
        if visited.contains(&current_position) {
            part1 += current_position.0 * current_position.1;
        }

        visited.insert(current_position);
        let mut next_position = current_position + direction;
        if !walls.contains(&next_position) {
            let directions = _possible_directions(&direction);
            let mut found = false;
            for d in directions {
                next_position = current_position + d;
                if walls.contains(&next_position) {
                    direction = d;
                    found = true;
                    break;
                }
            }
            if !found {
                break;
            }
        }
        current_position = next_position;
    }

    part1 as u64
}

fn _day17_part2(instructions: &Vec<i64>) -> i64 {
    let mut machine_code = IntCodeMachine::new(instructions);
    machine_code.memory.insert(0, 2);

    machine_code.run();

    /* Done this part manually.
    MAIN: A,A,B,C,B,C,B,C,B,A
    A   : R,10,L,12,R,6
    B   : R,6,R,10,R,12,R,6
    C   : R,10,L,12,L,12
    */
    let main = "A,A,B,C,B,C,B,C,B,A";
    let a = "R,10,L,12,R,6";
    let b = "R,6,R,10,R,12,R,6";
    let c = "R,10,L,12,L,12";
    format!("{}\n{}\n{}\n{}\nn\n", main, a, b, c)
        .chars()
        .for_each(|c| machine_code.push_input(c as i64));

    machine_code.run();
    *machine_code.output.last().unwrap()
}

#[cfg(test)]
mod tests_day17 {
    use std::fs::File;
    use std::io::Read;

    use crate::day17::{_day17_part1, _day17_part2};

    #[test]
    fn day17() {
        let mut file = File::open("data/day17.txt").unwrap();
        let mut buffer = String::new();
        file.read_to_string(&mut buffer).unwrap();

        let input: Vec<i64> = buffer.split(",")
            .map(|line| line.parse::<i64>().unwrap())
            .collect();

        assert_eq!(7816, _day17_part1(&input));
        assert_eq!(952010, _day17_part2(&input));
    }
}
