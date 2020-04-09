use std::collections::VecDeque;

pub struct Computer {
    memory: Vec<i64>,
    ip: usize,
    rbp: usize,
    is_running: bool,
    input: VecDeque<i64>,
    output: VecDeque<i64>,
}

pub enum Instruction {
    Add { src1: i64, src2: i64, dst: i64 },
    Mul { src1: i64, src2: i64, dst: i64 },
    Write { dst: i64 },
    Read { src: i64 },
    JumpNotZero { cond: i64, dst: i64 },
    JumpZero { cond: i64, dst: i64 },
    LessThan { src1: i64, src2: i64, dst: i64 },
    Equals { src1: i64, src2: i64, dst: i64 },
    AdjustRbp { src: i64 },
    Halt,
}

#[derive(Debug)]
enum ParameterMode {
    Position,
    Immediate,
    Relative,
}

impl Computer {
    pub fn new(memory: Vec<i64>) -> Computer {
        Computer {
            memory,
            ip: 0,
            rbp: 0,
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

    pub fn is_running(&self) -> bool {
        self.is_running
    }

    pub fn compute(&mut self) {
        while self.is_running {
            self.step();
        }
    }

    pub fn compute_until_read(&mut self) {
        while self.is_running {
            if let Instruction::Read { .. } = self.step() {
                break;
            }
        }
    }

    pub fn compute_until_io(&mut self) {
        while self.is_running {
            match self.step() {
                Instruction::Read { .. } => break,
                Instruction::Write { .. } => break,
                _ => {}
            }
        }
    }

    fn step(&mut self) -> Instruction {
        let instruction = self.fetch_and_decode();

        self.execute(&instruction);

        instruction
    }

    fn fetch_and_resize_memory(&mut self, addr: usize) -> i64 {
        if addr >= self.memory.len() {
            self.memory.resize(addr + 1, 0);
        }
        self.memory[addr]
    }

    fn store_and_resize_memory(&mut self, addr: usize, val: i64) {
        if addr >= self.memory.len() {
            self.memory.resize(addr + 1, 0);
        }
        self.memory[addr] = val;
    }

    fn fetch_dst_address(&self, mode: &ParameterMode, immediate: i64) -> i64 {
        match mode {
            ParameterMode::Position => immediate,
            ParameterMode::Immediate => panic!("dst operand cannot use immediate mode"),
            ParameterMode::Relative => self.rbp as i64 + immediate,
        }
    }

    fn fetch_operand(&mut self, mode: &ParameterMode, immediate: i64) -> i64 {
        match mode {
            ParameterMode::Position => self.fetch_and_resize_memory(immediate as usize),
            ParameterMode::Immediate => immediate,
            ParameterMode::Relative => {
                self.fetch_and_resize_memory((self.rbp as i64 + immediate) as usize)
            }
        }
    }

    fn fetch_and_decode(&mut self) -> Instruction {
        let intcode = self.memory[self.ip];

        let opcode = intcode % 100;

        let modes = (0..3)
            .map(|x| match intcode / (100 * 10i64.pow(x)) % 10 {
                0 => ParameterMode::Position,
                1 => ParameterMode::Immediate,
                2 => ParameterMode::Relative,
                _ => panic!("illegal parameter mode"),
            })
            .collect::<Vec<ParameterMode>>();

        match opcode {
            1 => Instruction::Add {
                src1: self.fetch_operand(&modes[0], self.memory[self.ip + 1]),
                src2: self.fetch_operand(&modes[1], self.memory[self.ip + 2]),
                dst: self.fetch_dst_address(&modes[2], self.memory[self.ip + 3]),
            },
            2 => Instruction::Mul {
                src1: self.fetch_operand(&modes[0], self.memory[self.ip + 1]),
                src2: self.fetch_operand(&modes[1], self.memory[self.ip + 2]),
                dst: self.fetch_dst_address(&modes[2], self.memory[self.ip + 3]),
            },
            3 => Instruction::Write {
                dst: self.fetch_dst_address(&modes[0], self.memory[self.ip + 1]),
            },
            4 => Instruction::Read {
                src: self.fetch_operand(&modes[0], self.memory[self.ip + 1]),
            },
            5 => Instruction::JumpNotZero {
                cond: self.fetch_operand(&modes[0], self.memory[self.ip + 1]),
                dst: self.fetch_operand(&modes[1], self.memory[self.ip + 2]),
            },
            6 => Instruction::JumpZero {
                cond: self.fetch_operand(&modes[0], self.memory[self.ip + 1]),
                dst: self.fetch_operand(&modes[1], self.memory[self.ip + 2]),
            },
            7 => Instruction::LessThan {
                src1: self.fetch_operand(&modes[0], self.memory[self.ip + 1]),
                src2: self.fetch_operand(&modes[1], self.memory[self.ip + 2]),
                dst: self.fetch_dst_address(&modes[2], self.memory[self.ip + 3]),
            },
            8 => Instruction::Equals {
                src1: self.fetch_operand(&modes[0], self.memory[self.ip + 1]),
                src2: self.fetch_operand(&modes[1], self.memory[self.ip + 2]),
                dst: self.fetch_dst_address(&modes[2], self.memory[self.ip + 3]),
            },
            9 => Instruction::AdjustRbp {
                src: self.fetch_operand(&modes[0], self.memory[self.ip + 1]),
            },
            99 => Instruction::Halt,
            _ => panic!("illegal opcode"),
        }
    }

    fn execute(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::Add { src1, src2, dst } => {
                self.store_and_resize_memory(*dst as usize, src1 + src2);
                self.ip += 4;
            }
            Instruction::Mul { src1, src2, dst } => {
                self.store_and_resize_memory(*dst as usize, src1 * src2);
                self.ip += 4;
            }
            Instruction::Write { dst } => {
                let src = self.input.pop_front().unwrap();
                self.store_and_resize_memory(*dst as usize, src);
                self.ip += 2;
            }
            Instruction::Read { src } => {
                self.output.push_back(*src);
                self.ip += 2;
            }
            Instruction::JumpNotZero { cond, dst } => {
                if *cond != 0 {
                    self.ip = *dst as usize;
                } else {
                    self.ip += 3;
                }
            }
            Instruction::JumpZero { cond, dst } => {
                if *cond == 0 {
                    self.ip = *dst as usize;
                } else {
                    self.ip += 3;
                }
            }
            Instruction::LessThan { src1, src2, dst } => {
                self.store_and_resize_memory(*dst as usize, (*src1 < *src2) as i64);
                self.ip += 4;
            }
            Instruction::Equals { src1, src2, dst } => {
                self.store_and_resize_memory(*dst as usize, (*src1 == *src2) as i64);
                self.ip += 4;
            }
            Instruction::AdjustRbp { src } => {
                self.rbp = ((self.rbp as i64) + *src) as usize;
                self.ip += 2;
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
