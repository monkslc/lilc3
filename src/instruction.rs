use super::{CondFlag, InstructionSize, RegisterIndex};

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
    JumpSubRoutine = 4,
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
        let opcode = get_opcode(instruction);
        match opcode {
            0 => OpCode::Branch,
            1 => OpCode::Add,
            2 => OpCode::Load,
            4 => OpCode::JumpSubRoutine,
            5 => OpCode::And,
            10 => OpCode::LoadIndirect,
            12 => OpCode::Jump,
            _ => todo!(),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Instruction {
    AddImmediate(AddImmediate),
    AddRegister(AddRegister),
    AndImmediate(AndImmediate),
    AndRegister(AndRegister),
    Branch(Branch),
    Jump(Jump),
    JumpSubRoutineOffset(JumpSubRoutineOffset),
    JumpSubRoutineRegister(JumpSubRoutineRegister),
    Load(Load),
    LoadIndirect(LoadIndirect),
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
        let instr = set_imm5(instr, self.imm5);

        instr
    }

    pub fn decode(instr: u16) -> Self {
        let dr = get_dr(instr);
        let sr1 = get_sr1(instr);
        let imm5 = get_imm5(instr);

        AddImmediate { dr, sr1, imm5 }
    }
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
pub struct AndImmediate {
    pub dr: RegisterIndex,
    pub sr1: RegisterIndex,
    pub imm5: u16,
}

impl AndImmediate {
    pub fn encode(&self) -> InstructionSize {
        let instr = 0;
        let instr = set_opcode(instr, OpCode::And);
        let instr = set_dr(instr, self.dr);
        let instr = set_sr1(instr, self.sr1);
        let instr = set_imm5(instr, self.imm5);

        instr
    }

    pub fn decode(instr: InstructionSize) -> Self {
        let dr = get_dr(instr);
        let sr1 = get_sr1(instr);

        let imm5 = instr & 0x1F;
        let imm5 = sign_extend_u16(imm5, 5);

        AndImmediate { dr, sr1, imm5 }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct AndRegister {
    pub dr: RegisterIndex,
    pub sr1: RegisterIndex,
    pub sr2: RegisterIndex,
}

impl AndRegister {
    pub fn encode(&self) -> InstructionSize {
        let instr = 0;
        let instr = set_opcode(instr, OpCode::And);
        let instr = set_dr(instr, self.dr);
        let instr = set_sr1(instr, self.sr1);
        let instr = set_sr2(instr, self.sr2);

        instr
    }
    pub fn decode(instr: InstructionSize) -> Self {
        let dr = get_dr(instr);
        let sr1 = get_sr1(instr);
        let sr2 = get_sr2(instr);

        AndRegister { dr, sr1, sr2 }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Branch {
    pub nzp: CondFlag,
    pub pc_offset9: u16,
}

impl Branch {
    pub fn encode(&self) -> InstructionSize {
        let instr = 0;
        let instr = set_opcode(instr, OpCode::Branch);
        let instr = set_nzp(instr, self.nzp);
        let instr = set_pcoffset9(instr, self.pc_offset9);

        instr
    }
    pub fn decode(instr: InstructionSize) -> Self {
        let nzp = get_nzp(instr);
        let pc_offset9 = get_pcoffset9(instr);

        Branch { nzp, pc_offset9 }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Jump {
    pub base_r: u8,
}

impl Jump {
    pub fn encode(&self) -> InstructionSize {
        let instr = 0;
        let instr = set_opcode(instr, OpCode::Jump);
        let instr = set_base_r(instr, self.base_r);

        instr
    }
    pub fn decode(instr: InstructionSize) -> Self {
        let base_r = get_base_r(instr);

        Jump { base_r }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct JumpSubRoutineOffset {
    pub pc_offset11: u16,
}

impl JumpSubRoutineOffset {
    pub fn encode(&self) -> InstructionSize {
        let instr = 0;
        let instr = set_opcode(instr, OpCode::JumpSubRoutine);
        let instr = set_pc_offset11(instr, self.pc_offset11);
        let instr = set_pc_offset_mode(instr);

        instr
    }
    pub fn decode(instr: InstructionSize) -> Self {
        let pc_offset11 = get_pc_offset11(instr);

        JumpSubRoutineOffset { pc_offset11 }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct JumpSubRoutineRegister {
    pub base_r: RegisterIndex,
}

impl JumpSubRoutineRegister {
    pub fn encode(&self) -> InstructionSize {
        let instr = 0;
        let instr = set_opcode(instr, OpCode::JumpSubRoutine);
        let instr = set_base_r(instr, self.base_r);

        instr
    }
    pub fn decode(instr: InstructionSize) -> Self {
        let base_r = get_base_r(instr);

        JumpSubRoutineRegister { base_r }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Load {
    pub dr: RegisterIndex,
    pub pc_offset9: u16,
}

impl Load {
    pub fn encode(&self) -> InstructionSize {
        let instr = 0;
        let instr = set_opcode(instr, OpCode::Load);
        let instr = set_dr(instr, self.dr);
        let instr = set_pcoffset9(instr, self.pc_offset9);
        instr
    }

    pub fn decode(instr: InstructionSize) -> Self {
        let dr = get_dr(instr);
        let pc_offset9 = get_pcoffset9(instr);

        Load { dr, pc_offset9 }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct LoadIndirect {
    pub dr: RegisterIndex,
    pub pc_offset9: u16,
}

impl LoadIndirect {
    pub fn encode(&self) -> InstructionSize {
        let instr = 0;
        let instr = set_opcode(instr, OpCode::LoadIndirect);
        let instr = set_dr(instr, self.dr);
        let instr = set_pcoffset9(instr, self.pc_offset9);
        instr
    }

    pub fn decode(instr: InstructionSize) -> Self {
        let dr = get_dr(instr);
        let pc_offset9 = get_pcoffset9(instr);

        LoadIndirect { dr, pc_offset9 }
    }
}

impl Instruction {
    pub fn decode(instr: InstructionSize) -> Self {
        match OpCode::from_instruction(instr) {
            OpCode::Add => {
                let mode_flag = get_immediate_mode(instr);

                if mode_flag == 1 {
                    Instruction::AddImmediate(AddImmediate::decode(instr))
                } else {
                    Instruction::AddRegister(AddRegister::decode(instr))
                }
            }
            OpCode::And => {
                let mode_flag = get_immediate_mode(instr);

                if mode_flag == 1 {
                    Instruction::AndImmediate(AndImmediate::decode(instr))
                } else {
                    Instruction::AndRegister(AndRegister::decode(instr))
                }
            }
            OpCode::Branch => Instruction::Branch(Branch::decode(instr)),
            OpCode::Jump => Instruction::Jump(Jump::decode(instr)),
            OpCode::JumpSubRoutine => {
                let offset_mode = get_pc_offset_mode(instr);

                if offset_mode == 1 {
                    Instruction::JumpSubRoutineOffset(JumpSubRoutineOffset::decode(instr))
                } else {
                    Instruction::JumpSubRoutineRegister(JumpSubRoutineRegister::decode(instr))
                }
            }
            OpCode::LoadIndirect => Instruction::LoadIndirect(LoadIndirect::decode(instr)),
            OpCode::Load => Instruction::Load(Load::decode(instr)),
            _ => todo!(),
        }
    }

    pub fn encode(&self) -> InstructionSize {
        match self {
            Self::AddImmediate(instr) => instr.encode(),
            Self::AddRegister(instr) => instr.encode(),
            Self::AndImmediate(instr) => instr.encode(),
            Self::AndRegister(instr) => instr.encode(),
            Self::Branch(instr) => instr.encode(),
            Self::Jump(instr) => instr.encode(),
            Self::JumpSubRoutineOffset(instr) => instr.encode(),
            Self::JumpSubRoutineRegister(instr) => instr.encode(),
            Self::Load(instr) => instr.encode(),
            Self::LoadIndirect(instr) => instr.encode(),
        }
    }
}

/// Returns the bits of an instruction from `start` to `end`
///
/// Instruction bits are 0 indexed. `start` is inclusive and `end` is exclusive.
fn get_bit_field(instr: InstructionSize, start: u8, end: u8) -> InstructionSize {
    instr >> start & !(0xFFFF << (end - start))
}

/// Sets the least significant bits of `field` in `instr` starting at `start`.
///
/// Instruction bits are 0 indexed. `start` is inclusive.
fn set_bit_field(instr: InstructionSize, field: u16, start: u8) -> InstructionSize {
    instr | (field << start)
}

fn set_opcode(instr: InstructionSize, op: OpCode) -> InstructionSize {
    set_bit_field(instr, op as u16, 12)
}

fn get_opcode(instr: InstructionSize) -> u16 {
    get_bit_field(instr, 12, 16)
}

fn set_dr(instr: InstructionSize, register: RegisterIndex) -> InstructionSize {
    set_bit_field(instr, register as u16, 9)
}

fn get_dr(instr: InstructionSize) -> RegisterIndex {
    get_bit_field(instr, 9, 12) as u8
}

fn set_sr1(instr: InstructionSize, register: RegisterIndex) -> InstructionSize {
    set_bit_field(instr, register as u16, 6)
}

fn get_sr1(instr: InstructionSize) -> RegisterIndex {
    get_bit_field(instr, 6, 9) as u8
}

fn set_sr2(instr: InstructionSize, register: RegisterIndex) -> InstructionSize {
    set_bit_field(instr, register as u16, 0)
}

fn get_sr2(instr: InstructionSize) -> RegisterIndex {
    get_bit_field(instr, 0, 3) as u8
}

fn set_imm5(instr: InstructionSize, imm5: u16) -> InstructionSize {
    let instr = set_bit_field(instr, imm5, 0);
    let immediate_mode_flag = 0b100000;
    let instr = instr | immediate_mode_flag;
    instr
}

fn get_imm5(instr: InstructionSize) -> u16 {
    let imm5 = get_bit_field(instr, 0, 5);
    let imm5 = sign_extend_u16(imm5, 5);
    imm5
}

fn get_immediate_mode(instr: InstructionSize) -> u16 {
    get_bit_field(instr, 5, 6)
}

fn get_nzp(instr: InstructionSize) -> CondFlag {
    let cond = get_bit_field(instr, 9, 12);
    CondFlag::from_bits(cond as u8).unwrap()
}

fn set_nzp(instr: InstructionSize, cond: CondFlag) -> InstructionSize {
    set_bit_field(instr, cond.bits() as u16, 9)
}

fn get_pcoffset9(instr: InstructionSize) -> u16 {
    let pc_offset9 = instr & 0x1FF;
    sign_extend_u16(pc_offset9, 9)
}

fn set_pcoffset9(instr: InstructionSize, offset: u16) -> InstructionSize {
    set_bit_field(instr, offset, 0)
}

fn get_base_r(instr: InstructionSize) -> RegisterIndex {
    get_bit_field(instr, 6, 8) as u8
}

fn set_base_r(instr: InstructionSize, base_r: RegisterIndex) -> InstructionSize {
    set_bit_field(instr, base_r as u16, 6)
}

fn get_pc_offset_mode(instr: InstructionSize) -> u16 {
    get_bit_field(instr, 11, 12)
}

fn set_pc_offset_mode(instr: InstructionSize) -> u16 {
    set_bit_field(instr, 1, 11)
}

fn get_pc_offset11(instr: InstructionSize) -> u16 {
    get_bit_field(instr, 0, 11)
}

fn set_pc_offset11(instr: InstructionSize, offset: u16) -> u16 {
    set_bit_field(instr, offset, 0)
}

fn sign_extend_u16(val: u16, original_length: u8) -> u16 {
    if (val >> (original_length - 1)) == 1 {
        (0xFFFF << original_length) | val
    } else {
        val
    }
}
