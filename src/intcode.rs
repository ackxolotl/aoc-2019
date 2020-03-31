use std::collections::VecDeque;

pub struct Computer {
    memory: Vec<i64>,
    ip: usize,
    is_running: bool,
    input: VecDeque<i64>,
    output: VecDeque<i64>,
}

pub enum Instruction {
    Add { src1: i64, src2: i64, dst: usize },
    Mul { src1: i64, src2: i64, dst: usize },
    Write { dst: usize },
    Read { src: i64 },
    JumpNotZero { cond: i64, dst: usize },
    JumpZero { cond: i64, dst: usize },
    LessThan { src1: i64, src2: i64, dst: usize },
    Equals { src1: i64, src2: i64, dst: usize },
    Halt,
}

#[derive(Debug)]
enum ParameterMode {
    Position,
    Immediate,
}

impl Computer {
    pub fn new(memory: Vec<i64>) -> Computer {
        Computer {
            memory,
            ip: 0,
            is_running: true,
            input: VecDeque::new(),
            output: VecDeque::new(),
        }
    }

    pub fn push_input(&mut self, input: i64) {
        self.input.push_back(input);
    }

    pub fn pop_output(&mut self) -> Option<i64> {
        self.output.pop_front()
    }

    pub fn get_from_memory(&mut self, index: usize) -> Option<i64> {
        self.memory.get(index).copied()
    }

    pub fn compute(&mut self) {
        while self.is_running {
            self.step();
        }
    }

    fn step(&mut self) -> Instruction {
        let instruction = self.fetch_and_decode();

        self.execute(&instruction);

        instruction
    }

    fn fetch_dst_address(&self, mode: &ParameterMode, immediate: usize) -> usize {
        match mode {
            ParameterMode::Position => immediate,
            ParameterMode::Immediate => panic!("dst operand cannot use immediate mode"),
        }
    }

    fn fetch_operand(&mut self, mode: &ParameterMode, immediate: i64) -> i64 {
        match mode {
            ParameterMode::Position => self.memory[immediate as usize],
            ParameterMode::Immediate => immediate,
        }
    }

    fn fetch_and_decode(&mut self) -> Instruction {
        let intcode = self.memory[self.ip];

        let opcode = intcode % 100;

        let modes = (0..3)
            .map(|x| match intcode / (100 * 10i64.pow(x)) % 10 {
                0 => ParameterMode::Position,
                1 => ParameterMode::Immediate,
                _ => panic!("illegal parameter mode"),
            })
            .collect::<Vec<ParameterMode>>();

        match opcode {
            1 => Instruction::Add {
                src1: self.fetch_operand(&modes[0], self.memory[self.ip + 1]),
                src2: self.fetch_operand(&modes[1], self.memory[self.ip + 2]),
                dst: self.fetch_dst_address(&modes[2], self.memory[self.ip + 3] as usize),
            },
            2 => Instruction::Mul {
                src1: self.fetch_operand(&modes[0], self.memory[self.ip + 1]),
                src2: self.fetch_operand(&modes[1], self.memory[self.ip + 2]),
                dst: self.fetch_dst_address(&modes[2], self.memory[self.ip + 3] as usize),
            },
            3 => Instruction::Write {
                dst: self.fetch_dst_address(&modes[0], self.memory[self.ip + 1] as usize),
            },
            4 => Instruction::Read {
                src: self.fetch_operand(&modes[0], self.memory[self.ip + 1]),
            },
            5 => Instruction::JumpNotZero {
                cond: self.fetch_operand(&modes[0], self.memory[self.ip + 1]),
                dst: self.fetch_operand(&modes[1], self.memory[self.ip + 2]) as usize,
            },
            6 => Instruction::JumpZero {
                cond: self.fetch_operand(&modes[0], self.memory[self.ip + 1]),
                dst: self.fetch_operand(&modes[1], self.memory[self.ip + 2]) as usize,
            },
            7 => Instruction::LessThan {
                src1: self.fetch_operand(&modes[0], self.memory[self.ip + 1]),
                src2: self.fetch_operand(&modes[1], self.memory[self.ip + 2]),
                dst: self.fetch_dst_address(&modes[2], self.memory[self.ip + 3] as usize),
            },
            8 => Instruction::Equals {
                src1: self.fetch_operand(&modes[0], self.memory[self.ip + 1]),
                src2: self.fetch_operand(&modes[1], self.memory[self.ip + 2]),
                dst: self.fetch_dst_address(&modes[2], self.memory[self.ip + 3] as usize),
            },
            _ => Instruction::Halt,
        }
    }

    fn execute(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::Add { src1, src2, dst } => {
                self.memory[*dst] = src1 + src2;
                self.ip += 4;
            }
            Instruction::Mul { src1, src2, dst } => {
                self.memory[*dst] = src1 * src2;
                self.ip += 4;
            }
            Instruction::Write { dst } => {
                let src = self.input.pop_front().unwrap();
                self.memory[*dst] = src;
                self.ip += 2;
            }
            Instruction::Read { src } => {
                self.output.push_back(*src);
                self.ip += 2;
            }
            Instruction::JumpNotZero { cond, dst } => {
                if *cond != 0 {
                    self.ip = *dst;
                } else {
                    self.ip += 3;
                }
            }
            Instruction::JumpZero { cond, dst } => {
                if *cond == 0 {
                    self.ip = *dst;
                } else {
                    self.ip += 3;
                }
            }
            Instruction::LessThan { src1, src2, dst } => {
                self.memory[*dst] = (*src1 < *src2) as i64;
                self.ip += 4;
            }
            Instruction::Equals { src1, src2, dst } => {
                self.memory[*dst] = (*src1 == *src2) as i64;
                self.ip += 4;
            }
            Instruction::Halt => {
                self.is_running = false;
            }
        }
    }
}

#[test]
fn test_position_equals() {
    let mut computer = Computer::new(vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8]);
    computer.push_input(8);
    computer.compute();
    assert_eq!(computer.pop_output(), Some(1));
    assert_eq!(computer.pop_output(), None);

    let mut computer = Computer::new(vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8]);
    computer.push_input(9);
    computer.compute();
    assert_eq!(computer.pop_output(), Some(0));
    assert_eq!(computer.pop_output(), None);
}

#[test]
fn test_position_less_than() {
    let mut computer = Computer::new(vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8]);
    computer.push_input(7);
    computer.compute();
    assert_eq!(computer.pop_output(), Some(1));
    assert_eq!(computer.pop_output(), None);

    let mut computer = Computer::new(vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8]);
    computer.push_input(8);
    computer.compute();
    assert_eq!(computer.pop_output(), Some(0));
    assert_eq!(computer.pop_output(), None);
}

#[test]
fn test_immediate_equals() {
    let mut computer = Computer::new(vec![3, 3, 1108, -1, 8, 3, 4, 3, 99]);
    computer.push_input(8);
    computer.compute();
    assert_eq!(computer.pop_output(), Some(1));
    assert_eq!(computer.pop_output(), None);

    let mut computer = Computer::new(vec![3, 3, 1108, -1, 8, 3, 4, 3, 99]);
    computer.push_input(9);
    computer.compute();
    assert_eq!(computer.pop_output(), Some(0));
    assert_eq!(computer.pop_output(), None);
}

#[test]
fn test_immediate_less_than() {
    let mut computer = Computer::new(vec![3, 3, 1107, -1, 8, 3, 4, 3, 99]);
    computer.push_input(7);
    computer.compute();
    assert_eq!(computer.pop_output(), Some(1));
    assert_eq!(computer.pop_output(), None);

    let mut computer = Computer::new(vec![3, 3, 1107, -1, 8, 3, 4, 3, 99]);
    computer.push_input(8);
    computer.compute();
    assert_eq!(computer.pop_output(), Some(0));
    assert_eq!(computer.pop_output(), None);
}
