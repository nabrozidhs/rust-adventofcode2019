#[derive(Debug, Eq, PartialEq)]
struct Layer {
    width: u64,
    height: u64,
    pixels: Vec<u8>,
}

impl Layer {
    fn _print(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let position = x + y * self.width;
                if self.pixels[position as usize] == 1 {
                    print!("#")
                } else {
                    print!(" ")
                }
            }
            println!()
        }
    }
}

struct _Image {
    width: u64,
    height: u64,
    layers: Vec<Layer>,
}

impl _Image {
    fn _merged_layer(&self) -> Layer {
        let mut layers = self.layers.iter().rev();
        let mut merged_layer: Vec<u8> = layers.next().unwrap().pixels.clone();
        for layer in layers {
            for y in 0..layer.height {
                for x in 0..layer.width {
                    let pos = (x + y * layer.width) as usize;
                    match layer.pixels[pos] {
                        0 => merged_layer[pos] = 0,
                        1 => merged_layer[pos] = 1,
                        _ => {}
                    }
                }
            }
        }
        Layer { height: self.height, width: self.width, pixels: merged_layer }
    }
}

fn _parse(input: &str, width: u64, height: u64) -> _Image {
    let mut layers: Vec<Layer> = vec![];

    let mut next_layer: Vec<u8> = vec![];
    for c in input.chars() {
        let p = c.to_digit(10).unwrap() as u8;
        next_layer.push(p);
        if next_layer.len() == (width * height) as usize {
            layers.push(Layer { width, height, pixels: next_layer });
            next_layer = vec![];
        }
    }

    _Image { width, height, layers }
}

fn _day08(input: &str, width: u64, height: u64) -> (u64, Layer) {
    let image = _parse(input, width, height);

    let min_layer = image.layers.iter()
        .min_by_key(|layer| layer.pixels.iter().filter(|p| **p == 0).count())
        .unwrap();

    (
        (min_layer.pixels.iter().filter(|p| **p == 1).count() *
            min_layer.pixels.iter().filter(|p| **p == 2).count()) as u64,
        image._merged_layer()
    )
}

#[cfg(test)]
mod tests_day08 {
    use std::fs::File;
    use std::io::Read;

    use crate::day08::{_day08, Layer};

    #[test]
    fn part1_test_input() {
        assert_eq!(1, _day08("123456789012", 3, 2).0);
    }

    #[test]
    fn part2_test_input() {
        assert_eq!(
            Layer { width: 2, height: 2, pixels: vec![0, 1, 1, 0] },
            _day08("0222112222120000", 2, 2).1
        );
    }

    #[test]
    fn day08() {
        let mut file = File::open("data/day08.txt").unwrap();
        let mut buffer = String::new();
        file.read_to_string(&mut buffer).unwrap();

        assert_eq!(1072, _day08(&buffer, 25, 6).0);
        _day08(&buffer, 25, 6).1._print();
    }
}
