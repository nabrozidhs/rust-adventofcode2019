fn _module_mass(mass: i64, with_fuel_weight: bool) -> i64 {
    let mut fuel = ((mass / 3) - 2).max(0);

    if with_fuel_weight {
        let mut next = fuel;
        while next > 0 {
            next = _module_mass(next, false);
            fuel += next;
        }
    }
    fuel
}

fn _day01(input: &Vec<i64>, with_fuel_weight: bool) -> i64 {
    input.iter().map(|v| _module_mass(*v, with_fuel_weight)).sum::<i64>()
}

#[cfg(test)]
mod tests_day01 {
    use std::fs::File;
    use std::io::Read;

    use crate::day01::_day01;

    #[test]
    fn part1_test_input() {
        assert_eq!(2, _day01(&vec![12], false));
        assert_eq!(2, _day01(&vec![14], false));
        assert_eq!(654, _day01(&vec![1969], false));
        assert_eq!(33583, _day01(&vec![100756], false));
    }

    #[test]
    fn part2_test_input() {
        assert_eq!(2, _day01(&vec![12], true));
        assert_eq!(2, _day01(&vec![14], true));
        assert_eq!(966, _day01(&vec![1969], true));
        assert_eq!(50346, _day01(&vec![100756], true));
    }

    #[test]
    fn day01() {
        let mut file = File::open("data/day01.txt").unwrap();
        let mut buffer = String::new();
        file.read_to_string(&mut buffer).unwrap();

        let input: Vec<i64> = buffer.lines()
            .map(|line| line.parse::<i64>().unwrap())
            .collect();

        assert_eq!(3373568, _day01(&input, false));
        assert_eq!(5057481, _day01(&input, true));
    }
}
