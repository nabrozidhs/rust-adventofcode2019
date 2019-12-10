use crate::util::{Vector2, gcd};
use std::collections::{HashSet, HashMap};

fn _parse(input: &str) -> HashSet<Vector2> {
    let mut asteroids: HashSet<Vector2> = HashSet::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                asteroids.insert(Vector2::new(x as i64, y as i64));
            }
        }
    }
    asteroids
}

fn _in_sight(asteroids: &HashSet<Vector2>, asteroid: &Vector2) -> Vec<Vector2> {
    let mut found: Vec<Vector2> = vec![];
    'found: for other in asteroids.iter() {
        if asteroid == other {
            continue;
        }

        let gcd = gcd(asteroid.0 - other.0, asteroid.1 - other.1);
        let diff = Vector2::new(
            (asteroid.0 - other.0) / gcd,
            (asteroid.1 - other.1) / gcd,
        );
        let mut next_position = *other + diff;
        while *asteroid != next_position {
            if asteroids.contains(&next_position) {
                continue 'found;
            }
            next_position += diff;
        }
        found.push(*other);
    }
    found.sort_by(|a, b| {
        let j = *a - *asteroid;
        let i = *b - *asteroid;
        (i.0 as f64).atan2(i.1 as f64).partial_cmp(&(j.0 as f64).atan2(j.1 as f64)).unwrap()
    });
    found
}

fn _day10_part1(input: &str) -> u64 {
    let asteroids = _parse(input);

    let mut hits: HashMap<Vector2, u64> = HashMap::new();
    for asteroid in asteroids.iter() {
        hits.insert(*asteroid, _in_sight(&asteroids, asteroid).len() as u64);
    }

    *hits.values().max().unwrap()
}

fn _day10_part2(input: &str) -> u64 {
    let mut asteroids = _parse(input);

    let mut hits: HashMap<Vector2, u64> = HashMap::new();
    for asteroid in asteroids.iter() {
        hits.insert(*asteroid, _in_sight(&asteroids, asteroid).len() as u64);
    }

    let station = hits.iter().max_by_key(|x| x.1).unwrap().0;
    let mut removed = 0;
    loop {
        for to_remove in _in_sight(&asteroids, station) {
            asteroids.remove(&to_remove);
            removed +=1;
            if removed == 200 {
                return (to_remove.0 * 100 + to_remove.1) as u64;
            }
        }
    }
}

#[cfg(test)]
mod tests_day10 {
    use std::fs::File;
    use std::io::Read;

    use crate::day10::{_day10_part1, _day10_part2};

    #[test]
    fn part1_test_input() {
        assert_eq!(8, _day10_part1(".#..#\n.....\n#####\n....#\n...##"));
        assert_eq!(
            33,
            _day10_part1("......#.#.\n#..#.#....\n..#######.\n.#.#.###..\n.#..#.....\n..#....#.#\n#..#....#.\n.##.#..###\n##...#..#.\n.#....####")
        );
    }

    #[test]
    fn part2_test_input() {
        assert_eq!(
            802,
            _day10_part2(".#..##.###...#######\n##.############..##.\n.#.######.########.#\n.###.#######.####.#.\n#####.##.#.##.###.##\n..#####..#.#########\n####################\n#.####....###.#.#.##\n##.#################\n#####.##.###..####..\n..######..##.#######\n####.##.####...##..#\n.#####..#.######.###\n##...#.##########...\n#.##########.#######\n.####.#.###.###.#.##\n....##.##.###..#####\n.#.#.###########.###\n#.#.#.#####.####.###\n###.##.####.##.#..##")
        );
    }

    #[test]
    fn day10() {
        let mut file = File::open("data/day10.txt").unwrap();
        let mut buffer = String::new();
        file.read_to_string(&mut buffer).unwrap();

        assert_eq!(269, _day10_part1(&buffer));
        assert_eq!(612, _day10_part2(&buffer));
    }
}
