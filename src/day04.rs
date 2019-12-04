use std::ops::Range;

fn _parse(input: &str) -> Range<i64> {
    let mut splitted = input.split("-");
    Range {
        start: splitted.next().unwrap().parse::<i64>().unwrap(),
        end: splitted.next().unwrap().parse::<i64>().unwrap() + 1,
    }
}

fn _day04(input: &str, larger_group: bool) -> i64 {
    let range = _parse(input);

    let mut count = 0;
    'outer: for n in range {
        // It is a six-digit number
        if !(1..=9).contains(&(n / 100_000)) { continue; }

        // Two adjacent digits are the same
        let mut d = n;
        let mut same_found = false;
        while d >= 10 {
            let mut counter = 1;
            let mut next = d / 10;
            while next > 0 && d % 10 == next % 10 {
                counter += 1;
                next /= 10;
            }
            if (larger_group && counter == 2) || (!larger_group && counter > 1) {
                same_found = true;
                break;
            }
            d = next;
        }
        if !same_found { continue; }

        // Going from left to right, the digits never decrease
        d = n;
        while d >= 10 {
            let next = d / 10;
            if d % 10 < next % 10 {
                continue 'outer;
            }
            d = next;
        }

        count += 1;
    }
    count
}

#[cfg(test)]
mod tests_day04 {
    use crate::day04::_day04;

    #[test]
    fn part1_test_input() {
        assert_eq!(1, _day04("111111-111111", false));
        assert_eq!(0, _day04("223450-223450", false));
        assert_eq!(0, _day04("123789-123789", false));
    }

    #[test]
    fn part2_test_input() {
        assert_eq!(1, _day04("112233-112233", true));
        assert_eq!(0, _day04("123444-123444", true));
        assert_eq!(1, _day04("111122-111122", true));
    }

    #[test]
    fn day04() {
        assert_eq!(1665, _day04("158126-624574", false));
        assert_eq!(1131, _day04("158126-624574", true));
    }
}
