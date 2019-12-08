use crate::intcode::IntCodeMachineState::{Finished, Paused};

#[derive(Eq, PartialEq)]
pub enum IntCodeMachineState {
    Paused,
    Finished,
}

pub struct IntCodeMachine {
    pub code: Vec<i64>,
    idx: usize,
    state: IntCodeMachineState,
    input: Vec<i64>,
    pub output: Vec<i64>,
}

impl IntCodeMachine {
    pub fn new_with_input(code: &Vec<i64>, input: &Vec<i64>) -> IntCodeMachine {
        IntCodeMachine {
            code: code.clone(),
            idx: 0,
            state: Paused,
            input: input.clone(),
            output: vec![],
        }
    }

    pub fn new(code: &Vec<i64>) -> IntCodeMachine {
        IntCodeMachine::new_with_input(code, &vec![])
    }

    fn _get_value_from_instruction(&self, argument_position: usize) -> i64 {
        if self.code[self.idx] / (10 * 10_i64.pow(argument_position as u32)) % 10 == 1 {
            self.code[self.idx + argument_position]
        } else {
            self.code[self.code[self.idx + argument_position] as usize]
        }
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

        while self.code[self.idx] != 99 {
            match self.code[self.idx] % 10 {
                1 => {
                    let output_idx = self.code[self.idx + 3] as usize;
                    let a = self._get_value_from_instruction(1);
                    let b = self._get_value_from_instruction(2);
                    self.code[output_idx] = a + b;
                    self.idx += 4;
                }
                2 => {
                    let output_idx = self.code[self.idx + 3] as usize;
                    let a = self._get_value_from_instruction(1);
                    let b = self._get_value_from_instruction(2);
                    self.code[output_idx] = a * b;
                    self.idx += 4;
                }
                3 => {
                    if self.input.len() == 0 {
                        return Paused;
                    }
                    let output_idx = self.code[self.idx + 1] as usize;
                    self.code[output_idx] = self.input.remove(0);
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
                    let output_idx = self.code[self.idx + 3] as usize;
                    self.code[output_idx] = if a < b { 1 } else { 0 };
                    self.idx += 4;
                }
                8 => {
                    let a = self._get_value_from_instruction(1);
                    let b = self._get_value_from_instruction(2);
                    let output_idx = self.code[self.idx + 3] as usize;
                    self.code[output_idx] = if a == b { 1 } else { 0 };
                    self.idx += 4;
                }
                _ => panic!()
            }
        }
        Finished
    }
}
