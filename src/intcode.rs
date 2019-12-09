use crate::intcode::IntCodeMachineState::{Finished, Paused};
use std::collections::HashMap;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum IntCodeMachineState {
    Paused,
    Finished,
}

pub struct IntCodeMachine {
    pub memory: HashMap<usize, i64>,
    idx: usize,
    state: IntCodeMachineState,
    input: Vec<i64>,
    relative_base: i64,
    pub output: Vec<i64>,
}

impl IntCodeMachine {
    pub fn new_with_input(code: &Vec<i64>, input: &Vec<i64>) -> IntCodeMachine {
        let mut memory: HashMap<usize, i64> = HashMap::new();
        for i in 0..code.len() {
            memory.insert(i, code[i]);
        }
        IntCodeMachine {
            memory,
            idx: 0,
            state: Paused,
            input: input.clone(),
            relative_base: 0,
            output: vec![],
        }
    }

    pub fn new(code: &Vec<i64>) -> IntCodeMachine {
        IntCodeMachine::new_with_input(code, &vec![])
    }

    fn _get_write_pointer(&self, argument_position: usize) -> usize {
        let mode = self.memory[&self.idx] / (10 * 10_i64.pow(argument_position as u32)) % 10;
        match mode {
            0 => self.memory[&(self.idx + argument_position)] as usize,
            1 => panic!("cannot write in this mode"),
            2 => (self.relative_base + self.memory[&(self.idx + argument_position)]) as usize,
            _ => panic!()
        }
    }

    fn _get_value_from_instruction(&self, argument_position: usize) -> i64 {
        let mode = self.memory[&self.idx] / (10 * 10_i64.pow(argument_position as u32)) % 10;
        let pointer = match mode {
            0 => self.memory[&(self.idx + argument_position)] as usize,
            1 => self.idx + argument_position,
            2 => (self.relative_base + self.memory[&(self.idx + argument_position)]) as usize,
            _ => panic!()
        };
        *self.memory.get(&pointer).unwrap_or(&0)
    }

    pub fn push_input(&mut self, input: i64) { self.input.push(input); }

    pub fn consume_output(&mut self) -> Option<i64> {
        if self.output.len() > 0 {
            Some(self.output.remove(0))
        } else {
            None
        }
    }

    pub fn run(&mut self) -> IntCodeMachineState {
        if self.state == Finished {
            return Finished;
        }

        'outer: loop {
            match self.memory[&self.idx] % 100 {
                1 => {
                    let output_idx = self._get_write_pointer(3) as usize;
                    let a = self._get_value_from_instruction(1);
                    let b = self._get_value_from_instruction(2);
                    self.memory.insert(output_idx, a + b);
                    self.idx += 4;
                }
                2 => {
                    let output_idx = self._get_write_pointer(3);
                    let a = self._get_value_from_instruction(1);
                    let b = self._get_value_from_instruction(2);
                    self.memory.insert(output_idx, a * b);
                    self.idx += 4;
                }
                3 => {
                    if self.input.len() == 0 {
                        self.state = Paused;
                        break 'outer;
                    }
                    let output_idx = self._get_write_pointer(1);
                    self.memory.insert(output_idx, self.input.remove(0));
                    self.idx += 2;
                }
                4 => {
                    self.output.push(self._get_value_from_instruction(1));
                    self.idx += 2;
                }
                5 => {
                    let a = self._get_value_from_instruction(1);
                    self.idx = if a != 0 {
                        self._get_value_from_instruction(2) as usize
                    } else {
                        self.idx + 3
                    }
                }
                6 => {
                    let a = self._get_value_from_instruction(1);
                    self.idx = if a == 0 {
                        self._get_value_from_instruction(2) as usize
                    } else {
                        self.idx + 3
                    }
                }
                7 => {
                    let a = self._get_value_from_instruction(1);
                    let b = self._get_value_from_instruction(2);
                    let output_idx = self._get_write_pointer(3);
                    self.memory.insert(output_idx, if a < b { 1 } else { 0 });
                    self.idx += 4;
                }
                8 => {
                    let a = self._get_value_from_instruction(1);
                    let b = self._get_value_from_instruction(2);
                    let output_idx = self._get_write_pointer(3);
                    self.memory.insert(output_idx, if a == b { 1 } else { 0 });
                    self.idx += 4;
                }
                9 => {
                    self.relative_base += self._get_value_from_instruction(1);
                    self.idx += 2;
                }
                99 => {
                    self.state = Finished;
                    break 'outer;
                }
                _ => panic!()
            }
        }
        self.state
    }
}
