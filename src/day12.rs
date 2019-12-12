use regex::Regex;

use crate::util::{Vector3, lcm};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Moon {
    position: Vector3,
    velocity: Vector3,
}

fn _update_position_and_velocity(moons: &mut Vec<Moon>) {
    for i in 0..moons.len() {
        let x: i64 = moons.iter().map(|m| (m.position.0 - moons[i].position.0).signum()).sum();
        let y: i64 = moons.iter().map(|m| (m.position.1 - moons[i].position.1).signum()).sum();
        let z: i64 = moons.iter().map(|m| (m.position.2 - moons[i].position.2).signum()).sum();
        moons.get_mut(i).unwrap().velocity += Vector3(x, y, z);
    }

    for moon in moons.iter_mut() {
        moon.position += moon.velocity;
    }
}

fn _parse(input: &str) -> Vec<Moon> {
    let re = Regex::new(r"<x=(-?\d+), y=(-?\d+), z=(-?\d+)>").unwrap();
    let mut moons = vec![];
    for line in input.lines() {
        let cap = re.captures(line).unwrap();
        moons.push(
            Moon {
                position: Vector3(
                    cap[1].parse().unwrap(),
                    cap[2].parse().unwrap(),
                    cap[3].parse().unwrap(),
                ),
                velocity: Vector3(0, 0, 0),
            }
        );
    }
    moons
}

fn _day12_part1(input: &str, steps: usize) -> i64 {
    let mut moons = _parse(input);

    for _ in 0..steps {
        _update_position_and_velocity(&mut moons);
    }

    moons.iter().map(|moon| {
        let pot = moon.position.0.abs() + moon.position.1.abs() + moon.position.2.abs();
        let kin = moon.velocity.0.abs() + moon.velocity.1.abs() + moon.velocity.2.abs();
        pot * kin
    }).sum()
}

fn _day12_part2(input: &str) -> i64 {
    let start_moons = _parse(input);
    let mut moons = start_moons.clone();

    let mut x = 0;
    let mut y = 0;
    let mut z = 0;

    let mut step = 1;
    loop {
        _update_position_and_velocity(&mut moons);

        if x == 0 && moons.iter().enumerate().all(|(i, x)| {
            x.velocity.0 == 0 && x.position.0 == start_moons[i].position.0
        }) {
            x = step;
        }
        if y == 0 && moons.iter().enumerate().all(|(i, x)| {
            x.velocity.1 == 0 && x.position.1 == start_moons[i].position.1
        }) {
            y = step;
        }
        if z == 0 && moons.iter().enumerate().all(|(i, x)| {
            x.velocity.2 == 0 && x.position.2 == start_moons[i].position.2
        }) {
            z = step;
        }
        if x != 0 && y != 0 && z != 0 {
            return lcm(x, lcm(y, z));
        }

        step += 1;
    }
}

#[cfg(test)]
mod tests_day12 {
    use std::fs::File;
    use std::io::Read;

    use crate::day12::{_day12_part1, _day12_part2};

    #[test]
    fn part1_test_input() {
        assert_eq!(
            179,
            _day12_part1(
                "<x=-1, y=0, z=2>\n<x=2, y=-10, z=-7>\n<x=4, y=-8, z=8>\n<x=3, y=5, z=-1>",
                10,
            )
        );
        assert_eq!(
            1940,
            _day12_part1(
                "<x=-8, y=-10, z=0>\n<x=5, y=5, z=10>\n<x=2, y=-7, z=3>\n<x=9, y=-8, z=-3>",
                100,
            )
        );
    }

    #[test]
    fn part2_test_input() {
        assert_eq!(
            2772,
            _day12_part2(
                "<x=-1, y=0, z=2>\n<x=2, y=-10, z=-7>\n<x=4, y=-8, z=8>\n<x=3, y=5, z=-1>"
            )
        );
        assert_eq!(
            4686774924,
            _day12_part2(
                "<x=-8, y=-10, z=0>\n<x=5, y=5, z=10>\n<x=2, y=-7, z=3>\n<x=9, y=-8, z=-3>"
            )
        );
    }

    #[test]
    fn day12() {
        let mut file = File::open("data/day12.txt").unwrap();
        let mut buffer = String::new();
        file.read_to_string(&mut buffer).unwrap();

        assert_eq!(13399, _day12_part1(&buffer, 1000));
        assert_eq!(312992287193064, _day12_part2(&buffer));
    }
}
