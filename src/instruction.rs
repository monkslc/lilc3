use super::{InstructionSize, RegisterIndex};

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
    Unused = 8,
    Not = 9,
    LoadIndirect = 10,
    StoreIndirect = 11,
    Jump = 12,
    Reserved = 13,
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
    pub dr: RegisterIndex,
    pub sr1: RegisterIndex,
    pub sr2: RegisterIndex,
}

impl AddRegister {
    pub fn encode(&self) -> u16 {
        let instr = 0;
        let instr = set_opcode(instr, OpCode::Add);
        let instr = set_dr(instr, self.dr);
        let instr = set_sr1(instr, self.sr1);
        let instr = set_sr2(instr, self.sr2);

        instr
    }
    pub fn decode(instr: u16) -> Self {
        let dr = get_dr(instr);
        let sr1 = get_sr1(instr);
        let sr2 = get_sr2(instr);

        AddRegister { dr, sr1, sr2 }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct AddImmediate {
    pub dr: RegisterIndex,
    pub sr1: RegisterIndex,
    pub imm5: u16,
}

impl AddImmediate {
    pub fn encode(&self) -> u16 {
        let instr = 0;
        let instr = set_opcode(instr, OpCode::Add);
        let instr = set_dr(instr, self.dr);
        let instr = set_sr1(instr, self.sr1);

        let instr = instr | (self.imm5 as u16);
        let immediate_mode_flag = 0b100000;
        let instr = instr | immediate_mode_flag;

        instr
    }

    pub fn decode(instr: u16) -> Self {
        let dr = get_dr(instr);
        let sr1 = get_sr1(instr);

        let imm5 = instr & 0x1F;
        let imm5 = sign_extend_u16(imm5, 5);

        AddImmediate { dr, sr1, imm5 }
    }
}

impl Instruction {
    pub fn decode(instr: u16) -> Self {
        match OpCode::from_instruction(instr) {
            OpCode::Add => {
                let mode_flag = instr >> 5 & 1;
                if mode_flag == 1 {
                    Instruction::AddImmediate(AddImmediate::decode(instr))
                } else {
                    Instruction::AddRegister(AddRegister::decode(instr))
                }
            }
            _ => todo!(),
        }
    }

    pub fn encode(&self) -> u16 {
        match self {
            Self::AddRegister(instr) => instr.encode(),
            Self::AddImmediate(instr) => instr.encode(),
        }
    }
}

fn set_opcode(instr: InstructionSize, op: OpCode) -> InstructionSize {
    instr | op.align_instruction()
}

fn set_dr(instr: InstructionSize, register: RegisterIndex) -> InstructionSize {
    instr | ((register as u16) << 9)
}

fn set_sr1(instr: InstructionSize, register: RegisterIndex) -> InstructionSize {
    instr | ((register as u16) << 6)
}

fn set_sr2(instr: InstructionSize, register: RegisterIndex) -> InstructionSize {
    instr | (register as u16)
}

fn get_dr(instr: InstructionSize) -> RegisterIndex {
    ((instr >> 9) as u8) & 0x7
}

fn get_sr1(instr: InstructionSize) -> RegisterIndex {
    ((instr >> 6) as u8) & 0x7
}

fn get_sr2(instr: InstructionSize) -> RegisterIndex {
    (instr & 0x7) as u8
}

fn sign_extend_u16(val: u16, original_length: u8) -> u16 {
    if (val >> (original_length - 1)) == 1 {
        (0xFFFF << original_length) | val
    } else {
        val
    }
}
