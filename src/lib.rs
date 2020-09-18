pub mod instruction;

use instruction::{AddImmediate, AddRegister, Instruction};

pub type BusSize = u16;
pub type InstructionSize = u16;
pub type Memory = [MemoryLocationSize; MAX_MEMORY_SIZE];
pub type MemoryLocationSize = u16;
pub type RegisterIndex = u8;
pub type RegisterSize = u16;

const PROGRAM_START: usize = 0x3000;
const MAX_MEMORY_SIZE: usize = BusSize::MAX as usize;
const REGISTER_COUNT: usize = 8;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Flag {
    Pos = 1,
    Zero = 2,
    Neg = 4,
}

pub struct LC3 {
    memory: [MemoryLocationSize; MAX_MEMORY_SIZE],
    registers: [RegisterSize; REGISTER_COUNT],
    pc: usize,
    cond: Flag,
}

impl LC3 {
    pub fn new(memory: Memory) -> Self {
        LC3 {
            memory,
            registers: [0; REGISTER_COUNT],
            pc: PROGRAM_START,
            cond: Flag::Zero,
        }
    }

    pub fn step(&mut self) {
        let raw_instr = self.memory[self.pc];
        let instr = Instruction::decode(raw_instr);

        match instr {
            Instruction::AddRegister(instr) => self.add_register(instr),
            Instruction::AddImmediate(instr) => self.add_immediate(instr),
        }
    }

    pub fn add_register(&mut self, instr: AddRegister) {
        // u32s are added to prevent overflow
        let value: u32 =
            self.registers[instr.src1 as usize] as u32 + self.registers[instr.src2 as usize] as u32;
        self.set_register(instr.dest, value as u16)
    }

    pub fn add_immediate(&mut self, instr: AddImmediate) {
        let value = self.registers[instr.src1 as usize] + instr.immediate;
        self.set_register(instr.dest, value)
    }

    /// Put `value` in `register` and set the cond register based on `value`
    pub fn set_register(&mut self, register: RegisterIndex, value: RegisterSize) {
        self.cond = match value {
            0 => Flag::Zero,
            v if v >> 15 == 1 => Flag::Neg,
            _ => Flag::Pos,
        };

        self.registers[register as usize] = value;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_register() {
        let mut memory = [0; MAX_MEMORY_SIZE];
        let dest = 1;
        let src1 = 2;
        let src2 = 3;

        let instruction = Instruction::AddRegister(AddRegister { dest, src1, src2 }).encode();

        memory[PROGRAM_START] = instruction;

        let mut machine = LC3::new(memory);
        machine.registers[src1 as usize] = 5;
        machine.registers[src2 as usize] = 6;
        machine.step();

        assert_eq!(machine.registers[dest as usize], 11);
        assert_eq!(machine.cond, Flag::Pos);
    }

    #[test]
    fn add_immediate() {
        let mut memory = [0; MAX_MEMORY_SIZE];
        let dest = 1;
        let src1 = 2;
        let immediate = 6;

        let instruction = Instruction::AddImmediate(AddImmediate {
            dest,
            src1,
            immediate,
        })
        .encode();

        memory[PROGRAM_START] = instruction;

        let mut machine = LC3::new(memory);
        machine.registers[src1 as usize] = 5;
        machine.step();

        assert_eq!(machine.registers[dest as usize], 11);
        assert_eq!(machine.cond, Flag::Pos);
    }

    #[test]
    fn add_cond_flag_negative() {
        let mut memory = [0; MAX_MEMORY_SIZE];
        let dest = 1;
        let src1 = 2;
        let src2 = 3;

        let instruction = Instruction::AddRegister(AddRegister { dest, src1, src2 }).encode();

        memory[PROGRAM_START] = instruction;

        let negative_one: u16 = 0xFFFF;
        let negative_two = 0xFFFE;

        let mut machine = LC3::new(memory);
        machine.registers[src1 as usize] = negative_one;
        machine.registers[src2 as usize] = negative_one;
        machine.step();

        assert_eq!(machine.registers[dest as usize], negative_two);
        assert_eq!(machine.cond, Flag::Neg);
    }

    #[test]
    fn add_cond_flag_zero() {
        let mut memory = [0; MAX_MEMORY_SIZE];
        let dest = 1;
        let src1 = 2;
        let src2 = 3;

        let instruction = Instruction::AddRegister(AddRegister { dest, src1, src2 }).encode();

        memory[PROGRAM_START] = instruction;

        let negative_one: u16 = 0xFFFF;

        let mut machine = LC3::new(memory);
        machine.registers[src1 as usize] = 1;
        machine.registers[src2 as usize] = negative_one;
        machine.step();

        assert_eq!(machine.registers[dest as usize], 0);
        assert_eq!(machine.cond, Flag::Zero);
    }
}
