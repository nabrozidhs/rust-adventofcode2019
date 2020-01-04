use std::fs::File;
use std::io::Read;

use adventofcode2019::intcode::IntCodeMachine;
use adventofcode2019::intcode::IntCodeMachineState::Finished;

fn _day25(instructions: &Vec<i64>) {
    let mut machine_code = IntCodeMachine::new(instructions);

    while machine_code.run() != Finished {
        while let Some(o) = machine_code.consume_output() {
            print!("{}", char::from(o as u8));
        }

        let mut line = String::new();
        let b1 = std::io::stdin().read_line(&mut line).unwrap();

        line.chars().for_each(|c| machine_code.push_input(c as i64));
    }

    while let Some(o) = machine_code.consume_output() {
        print!("{}", char::from(o as u8));
    }
}

// Played by hand, answer is 295944
fn main() {
    let mut file = File::open("data/day25.txt").unwrap();
    let mut buffer = String::new();
    file.read_to_string(&mut buffer).unwrap();

    let input: Vec<i64> = buffer.split(",")
        .map(|line| line.parse::<i64>().unwrap())
        .collect();

    _day25(&input);
}

