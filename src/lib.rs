const PROGRAM_START: usize = 0x3000;
const MAX_MEMORY_SIZE: usize = BusSize::MAX as usize;
const REGISTER_COUNT: usize = 8;

type BusSize = u16;
type MemoryLocationSize = u16;
type RegisterSize = u16;
type InstructionSize = u16;
type Memory = [MemoryLocationSize; MAX_MEMORY_SIZE];

/// OpCodes specify the instruction to be performed. In LC3 they are bits 12 to 15 of the 16 bit
/// instruction. The numbers asssociated with each opcode in the enum correspond with bits 12 to 15 of an LC3 instruction for that opcode. That is, doing 12 right shifts on an instruction will leave
/// the number associated with the opcode below.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u16)]
pub enum OpCode {
    Branch = 0,
    Add = 1,
    Load = 2,
    Store = 3,
    JumpRegister = 4,
    And = 5,
    LoadRegister = 6,
    StoreRegister = 7,
    RTI = 8, // Unused?
    Not = 9,
    LoadIndirect = 10,
    StoreIndirect = 11,
    Jump = 12,
    Reserved = 13, // Unused?
    LoadEffectiveAddress = 14,
    Trap = 15,
}

impl OpCode {
    /// `align_instruction` will shift the bits of the opcode so the number returned will align with
    /// bits 12 to 15 with an instruction that contains that opcode.
    pub fn align_instruction(&self) -> InstructionSize {
        (*self as InstructionSize) << 12
    }

    /// `from_instruction` returns the OpCode for a particular instruction. The OpCode is bits 12 to
    /// 15 for an instruction
    ///
    /// # Panics if the opcode for the instruction is not recognized
    pub fn from_instruction(instruction: InstructionSize) -> Self {
        let opcode = instruction >> 12;
        match opcode {
            1 => OpCode::Add,
            _ => todo!(),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Instruction {
    AddRegister(AddRegister),
    AddImmediate(AddImmediate),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct AddRegister {
    dest: usize,
    src1: usize,
    src2: usize,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct AddImmediate {
    dest: usize,
    src1: usize,
    immediate: u16,
}

impl Instruction {
    pub fn decode(instr: u16) -> Self {
        match OpCode::from_instruction(instr) {
            OpCode::Add => {
                let mode_flag = instr >> 5 & 1;
                if mode_flag == 1 {
                    Instruction::AddImmediate(Instruction::decode_immediate_add(instr))
                } else {
                    Instruction::AddRegister(Instruction::decode_register_add(instr))
                }
            }
            _ => todo!(),
        }
    }

    fn decode_register_add(instr: InstructionSize) -> AddRegister {
        let dest = (instr >> 9 & 0x7) as usize;
        let src1 = (instr >> 6 & 0x7) as usize;
        let src2 = (instr & 0x7) as usize;

        AddRegister { dest, src1, src2 }
    }

    fn decode_immediate_add(instr: InstructionSize) -> AddImmediate {
        let dest = (instr >> 9 & 0x7) as usize;
        let src1 = (instr >> 6 & 0x7) as usize;
        let immediate = instr & 0x1F;
        let sign_extended_immediate = if immediate >> 5 == 1 {
            0xFFE0 | immediate
        } else {
            immediate
        };

        AddImmediate {
            dest,
            src1,
            immediate: sign_extended_immediate,
        }
    }
}

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
        let value = self.registers[instr.src1] + self.registers[instr.src2];
        self.set_register(instr.dest, value)
    }

    pub fn add_immediate(&mut self, instr: AddImmediate) {
        let value = self.registers[instr.src1] + instr.immediate;
        self.set_register(instr.dest, value)
    }

    /// Put `value` in `register` and set the cond register based on `value`
    pub fn set_register(&mut self, register: usize, value: RegisterSize) {
        self.cond = match value {
            0 => Flag::Zero,
            v if v > 0 => Flag::Pos,
            _ => Flag::Neg,
        };

        self.registers[register] = value;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use helpers::*;

    #[test]
    fn add_register() {
        let mut memory = [0; MAX_MEMORY_SIZE];
        let dest = 1;
        let src1 = 2;
        let src2 = 3;

        let instruction = {
            let i: InstructionSize = 0;
            let i = set_opcode(i, OpCode::Add);
            let i = set_destination(i, dest);
            let i = set_source1(i, src1);
            let i = set_source2(i, src2);
            i
        };

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

        let instruction = {
            let i: InstructionSize = 0;
            let i = set_opcode(i, OpCode::Add);
            let i = set_destination(i, dest);
            let i = set_source1(i, src1);
            let i = i | immediate;
            let immediate_mode_flag = 0b100000;
            let i = i | immediate_mode_flag;
            i
        };

        memory[PROGRAM_START] = instruction;

        let mut machine = LC3::new(memory);
        machine.registers[src1 as usize] = 5;
        machine.step();

        assert_eq!(machine.registers[dest as usize], 11);
        assert_eq!(machine.cond, Flag::Pos);
    }

    mod helpers {
        use super::*;

        pub fn set_opcode(instr: InstructionSize, op: OpCode) -> InstructionSize {
            instr | op.align_instruction()
        }

        pub fn set_destination(instr: InstructionSize, register: u16) -> InstructionSize {
            instr | (register << 9)
        }

        pub fn set_source1(instr: InstructionSize, register: u16) -> InstructionSize {
            instr | (register << 6)
        }

        pub fn set_source2(instr: InstructionSize, register: u16) -> InstructionSize {
            instr | register
        }
    }
}
