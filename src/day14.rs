use std::collections::HashMap;

use itertools::Itertools;

fn _split(input: &str) -> (i64, String) {
    let mut s = input.split(" ");
    (
        s.next().unwrap().parse::<i64>().unwrap(),
        s.next().unwrap().to_string()
    )
}

fn _parse(input: &str) -> HashMap<String, Vec<(i64, String)>> {
    let mut map: HashMap<String, Vec<(i64, String)>> = HashMap::new();
    for line in input.lines() {
        let mut s = line.split(" => ");
        let i = s.next().unwrap();
        let o = s.next().unwrap();

        let mut v: Vec<(i64, String)> = i.split(", ").map(|x| _split(x)).collect();
        let output = _split(o);

        v.push(output.clone());

        map.insert(output.1, v);
    }
    map
}

fn _ore_required(mapping: &HashMap<String, Vec<(i64, String)>>, fuel_quantity: i64) -> i64 {
    let mut requires: HashMap<String, i64> = HashMap::new();
    requires.insert("FUEL".to_string(), fuel_quantity);
    let mut extra_materials: HashMap<String, i64> = HashMap::new();

    loop {
        let mut next: HashMap<String, i64> = HashMap::new();
        for (key, amount_required) in requires {
            if &key == "ORE" {
                let ore = *extra_materials.get(&key).unwrap_or(&0);
                extra_materials.insert(key, ore + amount_required);
                continue;
            }

            let v = mapping[&key].clone();
            let materials_in_inventory = *extra_materials.get(&key).unwrap_or(&0);
            if amount_required <= materials_in_inventory {
                extra_materials.insert(key, materials_in_inventory - amount_required);
            } else {
                let required = amount_required - materials_in_inventory;
                if required > 0 {
                    let times = (required as f64 / v.last().unwrap().0 as f64).ceil() as i64;
                    for e in v.iter().dropping_back(1) {
                        let g = *next.get(&e.1).unwrap_or(&0);
                        next.insert(e.1.clone(), g + times * e.0);
                    }
                    extra_materials.insert(key, v.last().unwrap().0 * times - required);
                }
            }
        }

        requires = next;

        if requires.is_empty() {
            return extra_materials["ORE"];
        }
    }
}

fn _day14_part1(input: &str) -> i64 {
    _ore_required(&_parse(input), 1)
}

fn _day14_part2(input: &str) -> i64 {
    let mapping = _parse(input);

    let max_ore = 1_000_000_000_000;

    let mut left = 0;
    let mut right = max_ore;
    while left <= right {
        let c = (left + right) / 2;
        let result = _ore_required(&mapping, c);
        if result < max_ore {
            left = c + 1;
        } else if result > max_ore {
            right = c - 1;
        }
    }
    right
}

#[cfg(test)]
mod tests_day14 {
    use std::fs::File;
    use std::io::Read;

    use crate::day14::{_day14_part1, _day14_part2};

    #[test]
    fn part1_test_input() {
        assert_eq!(
            165,
            _day14_part1(
                "9 ORE => 2 A\n8 ORE => 3 B\n7 ORE => 5 C\n3 A, 4 B => 1 AB\n5 B, 7 C => 1 BC\n4 C, 1 A => 1 CA\n2 AB, 3 BC, 4 CA => 1 FUEL",
            )
        );
        assert_eq!(
            13312,
            _day14_part1(
                "157 ORE => 5 NZVS\n165 ORE => 6 DCFZ\n44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL\n12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ\n179 ORE => 7 PSHF\n177 ORE => 5 HKGWZ\n7 DCFZ, 7 PSHF => 2 XJWVT\n165 ORE => 2 GPVTF\n3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT",
            )
        );
        assert_eq!(
            180697,
            _day14_part1(
            "2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG\n17 NVRVD, 3 JNWZP => 8 VPVL\n53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL\n22 VJHF, 37 MNCFX => 5 FWMGM\n139 ORE => 4 NVRVD\n144 ORE => 7 JNWZP\n5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC\n5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV\n145 ORE => 6 MNCFX\n1 NVRVD => 8 CXFTF\n1 VJHF, 6 MNCFX => 4 RFSQX\n176 ORE => 6 VJHF",
            )
        );
    }

    #[test]
    fn part2_test_input() {
        assert_eq!(
            82892753,
            _day14_part2(
                "157 ORE => 5 NZVS\n165 ORE => 6 DCFZ\n44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL\n12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ\n179 ORE => 7 PSHF\n177 ORE => 5 HKGWZ\n7 DCFZ, 7 PSHF => 2 XJWVT\n165 ORE => 2 GPVTF\n3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT",
            )
        );
        assert_eq!(
            5586022,
            _day14_part2(
                "2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG\n17 NVRVD, 3 JNWZP => 8 VPVL\n53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL\n22 VJHF, 37 MNCFX => 5 FWMGM\n139 ORE => 4 NVRVD\n144 ORE => 7 JNWZP\n5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC\n5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV\n145 ORE => 6 MNCFX\n1 NVRVD => 8 CXFTF\n1 VJHF, 6 MNCFX => 4 RFSQX\n176 ORE => 6 VJHF",
            )
        );

    }

    #[test]
    fn day14() {
        let mut file = File::open("data/day14.txt").unwrap();
        let mut buffer = String::new();
        file.read_to_string(&mut buffer).unwrap();

        assert_eq!(720484, _day14_part1(&buffer));
        assert_eq!(1993284, _day14_part2(&buffer));
    }
}
