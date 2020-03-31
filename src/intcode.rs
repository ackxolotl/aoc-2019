pub struct Computer {
    memory: Vec<usize>,
    ip: usize,
    is_running: bool,
}

pub enum Instruction {
    Add {
        src1: usize,
        src2: usize,
        dst: usize,
    },
    Mul {
        src1: usize,
        src2: usize,
        dst: usize,
    },
    Halt,
}

#[derive(Debug)]
enum ParameterMode {
    Position,
    Immediate,
}

impl Computer {
    pub fn new(memory: Vec<usize>) -> Computer {
        Computer {
            memory,
            ip: 0,
            is_running: true,
        }
    }

    pub fn get_output(&self) -> usize {
        self.memory[0]
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

    fn fetch_operand(&mut self, mode: &ParameterMode, immediate: usize) -> usize {
        match mode {
            ParameterMode::Position => self.memory[immediate],
            ParameterMode::Immediate => immediate,
        }
    }

    fn fetch_and_decode(&mut self) -> Instruction {
        let intcode = self.memory[self.ip];

        let opcode = intcode % 100;

        let modes = (0..3)
            .map(|x| match intcode / (100 * 10u32.pow(x) as usize) % 10 {
                0 => ParameterMode::Position,
                1 => ParameterMode::Immediate,
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
            Instruction::Halt => {
                self.is_running = false;
            }
        }
    }
}
