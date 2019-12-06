use std::collections::HashMap;

fn _parse(input: &str) -> HashMap<String, String> {
    let mut graph: HashMap<String, String> = HashMap::new();
    for line in input.lines() {
        let mut split = line.split(")");
        let v = split.next().unwrap().to_string();
        let k = split.next().unwrap().to_string();
        graph.insert(k, v);
    }
    graph
}

fn _find_orbits(graph: &HashMap<String, String>, from_planet: &str) -> Vec<String> {
    let mut orbits: Vec<String> = vec![];
    let mut next: Option<&String> = graph.get(&from_planet.to_string());
    while let Some(n) = next {
        orbits.push(n.clone());
        next = graph.get(n);
    }
    orbits
}

fn _day06_part1(input: &str) -> u64 {
    let graph = _parse(input);

    let mut orbits = 0;
    for value in graph.values() {
        orbits += 1 + _find_orbits(&graph, value).len() as u64;
    }
    orbits
}

fn _day06_part2(input: &str) -> u64 {
    let graph = _parse(input);

    let mut you_path = _find_orbits(&graph, "YOU");
    let mut santa_path = _find_orbits(&graph, "SAN");
    while you_path.last() == santa_path.last() {
        you_path.pop();
        santa_path.pop();
    }

    (you_path.len() + santa_path.len()) as u64
}


#[cfg(test)]
mod tests_day06 {
    use std::fs::File;
    use std::io::Read;

    use crate::day06::{_day06_part1, _day06_part2};

    #[test]
    fn part1_test_input() {
        assert_eq!(42, _day06_part1("COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L"));
    }


    #[test]
    fn part2_test_input() {
        assert_eq!(4, _day06_part2("COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L\nK)YOU\nI)SAN"));
    }

    #[test]
    fn day06() {
        let mut file = File::open("data/day06.txt").unwrap();
        let mut buffer = String::new();
        file.read_to_string(&mut buffer).unwrap();

        assert_eq!(295834, _day06_part1(&buffer));
        assert_eq!(361, _day06_part2(&buffer));
    }
}
