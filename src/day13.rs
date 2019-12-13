use std::collections::HashSet;

use crate::intcode::IntCodeMachine;
use crate::intcode::IntCodeMachineState::Finished;
use crate::util::Vector2;

struct _Game {
    machine: IntCodeMachine,
    blocks: HashSet<Vector2>,
    score: i64,
    paddle: Vector2,
    ball: Vector2,
}

impl _Game {
    fn _new(instructions: &Vec<i64>, play_mode: bool) -> _Game {
        let mut machine = IntCodeMachine::new(instructions);
        if play_mode {
            machine.memory.insert(0, 2);
        }
        _Game {
            machine,
            blocks: HashSet::new(),
            score: 0,
            paddle: Vector2::new(0, 0),
            ball: Vector2::new(0, 0),
        }
    }

    fn _update_state(&mut self) {
        while !self.machine.output.is_empty() {
            let x = self.machine.output.remove(0);
            let y = self.machine.output.remove(0);
            let id = self.machine.output.remove(0);
            if x == -1 && y == 0 {
                self.score = id;
            } else if id == 0 {
                self.blocks.remove(&Vector2::new(x, y));
            } else if id == 2 {
                self.blocks.insert(Vector2::new(x, y));
            } else if id == 3 {
                self.paddle = Vector2::new(x, y);
            } else if id == 4 {
                self.ball = Vector2::new(x, y);
            }
        }
    }

    fn _init(&mut self) {
        self.machine.run();

        self._update_state();
    }

    fn _play(&mut self) -> bool {
        self.machine.push_input((self.ball.0 - self.paddle.0).signum());

        let is_finished = self.machine.run() == Finished;

        self._update_state();

        self.blocks.is_empty() || is_finished
    }
}

fn _day13(instructions: &Vec<i64>, play_mode: bool) -> u64 {
    let mut game = _Game::_new(instructions, play_mode);
    game._init();

    if !play_mode {
        return game.blocks.len() as u64;
    }

    while !game._play() {}

    game.score as u64
}

#[cfg(test)]
mod tests_day13 {
    use std::fs::File;
    use std::io::Read;

    use crate::day13::_day13;

    #[test]
    fn day13() {
        let mut file = File::open("data/day13.txt").unwrap();
        let mut buffer = String::new();
        file.read_to_string(&mut buffer).unwrap();

        let input: Vec<i64> = buffer.split(",")
            .map(|line| line.parse::<i64>().unwrap())
            .collect();

        assert_eq!(255, _day13(&input, false));
        assert_eq!(12338, _day13(&input, true));
    }
}
