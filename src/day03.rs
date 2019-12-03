use std::collections::HashMap;

use crate::util::Vector2;

#[derive(Debug, Copy, Clone)]
struct Instruction {
    direction: Vector2,
    distance: i64,
}

fn _parse(input: &str) -> (Vec<Instruction>, Vec<Instruction>) {
    let parsed: Vec<Vec<Instruction>> = input.lines()
        .map(|line| line.split(",")
            .map(|w|
                Instruction {
                    direction: match w.chars().next().unwrap() {
                        'D' => Vector2::new(0, -1),
                        'U' => Vector2::new(0, 1),
                        'L' => Vector2::new(-1, 0),
                        'R' => Vector2::new(1, 0),
                        _ => panic!()
                    },
                    distance: w.get(1..).unwrap().parse::<i64>().unwrap(),
                })
            .collect())
        .collect();
    (parsed[0].clone(), parsed[1].clone())
}

fn _day03(input: &str) -> (i64, usize) {
    let (path1, path2) = _parse(input);

    let mut paths: HashMap<Vector2, usize> = HashMap::new();
    let mut current = Vector2::new(0, 0);
    let mut distance = 0;
    for instruction in path1 {
        for _ in 0..instruction.distance {
            current += instruction.direction;
            distance += 1;
            paths.insert(current, distance);
        }
    }

    let mut matches: Vec<(Vector2, usize)> = vec![];
    current = Vector2::new(0, 0);
    distance = 0;
    for instruction in path2 {
        for _ in 0..instruction.distance {
            current += instruction.direction;
            distance += 1;
            if paths.contains_key(&current) {
                matches.push((current, distance));
            }
        }
    }

    (
        matches.iter().map(|x| x.0.manhattan_distance(&Vector2::ZERO)).min().unwrap(),
        matches.iter().map(|x| x.1 + paths[&x.0]).min().unwrap()
    )
}

#[cfg(test)]
mod tests_day03 {
    use std::fs::File;
    use std::io::Read;

    use crate::day03::_day03;

    #[test]
    fn part1_test_input() {
        assert_eq!(6, _day03("R8,U5,L5,D3\nU7,R6,D4,L4").0);
        assert_eq!(
            159,
            _day03("R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83").0
        );
        assert_eq!(
            135,
            _day03("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7").0
        );
    }

    #[test]
    fn part2_test_input() {
        assert_eq!(30, _day03("R8,U5,L5,D3\nU7,R6,D4,L4").1);
        assert_eq!(
            610,
            _day03("R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83").1
        );
        assert_eq!(
            410,
            _day03("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7").1
        );
    }

    #[test]
    fn day03() {
        let mut file = File::open("data/day03.txt").unwrap();
        let mut buffer = String::new();
        file.read_to_string(&mut buffer).unwrap();

        let (part1, part2) = _day03(&buffer);
        assert_eq!(5357, part1);
        assert_eq!(101956, part2);
    }
}
