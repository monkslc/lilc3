use bitflags::bitflags;

pub mod instruction;

use instruction::{
    AddImmediate, AddRegister, AndImmediate, AndRegister, Branch, Instruction, Jump,
    JumpSubRoutineOffset, JumpSubRoutineRegister, Load, LoadBaseOffset, LoadEffectiveAddress,
    LoadIndirect,
};

pub type BusSize = u16;
pub type InstructionSize = u16;
pub type Memory = [MemoryLocationSize; MAX_MEMORY_SIZE];
pub type MemoryLocationSize = u16;
pub type RegisterIndex = u8;
pub type RegisterSize = u16;

const PROGRAM_START: MemoryLocationSize = 0x3000;
const MAX_MEMORY_SIZE: usize = BusSize::MAX as usize;
const REGISTER_COUNT: usize = 8;

bitflags! {
    pub struct CondFlag: u8 {
        const POSITIVE = 0b1;
        const NEGATIVE = 0b10;
        const ZERO = 0b100;
    }
}

pub struct LC3 {
    memory: [MemoryLocationSize; MAX_MEMORY_SIZE],
    registers: [RegisterSize; REGISTER_COUNT],
    pc: u16,
    cond: CondFlag,
}

impl LC3 {
    pub fn new(memory: Memory) -> Self {
        LC3 {
            memory,
            registers: [0; REGISTER_COUNT],
            pc: PROGRAM_START,
            cond: CondFlag::ZERO,
        }
    }

    pub fn step(&mut self) {
        let raw_instr = self.memory[self.pc as usize];
        self.pc += 1;
        let instr = Instruction::decode(raw_instr);

        match instr {
            Instruction::AddImmediate(instr) => self.add_immediate(instr),
            Instruction::AddRegister(instr) => self.add_register(instr),
            Instruction::AndImmediate(instr) => self.and_immediate(instr),
            Instruction::AndRegister(instr) => self.and_register(instr),
            Instruction::Branch(instr) => self.branch(instr),
            Instruction::Jump(instr) => self.jump(instr),
            Instruction::JumpSubRoutineOffset(instr) => self.jump_subroutine_offset(instr),
            Instruction::JumpSubRoutineRegister(instr) => self.jump_subroutine_register(instr),
            Instruction::Load(instr) => self.load(instr),
            Instruction::LoadBaseOffset(instr) => self.load_base_offset(instr),
            Instruction::LoadEffectiveAddress(instr) => self.load_effective_address(instr),
            Instruction::LoadIndirect(instr) => self.load_indirect(instr),
        }
    }

    pub fn add_immediate(&mut self, instr: AddImmediate) {
        // u32s are added to prevent overflow
        let value: u32 = self.registers[instr.sr1 as usize] as u32 + (instr.imm5 as u16) as u32;
        self.set_register(instr.dr, value as u16)
    }

    pub fn add_register(&mut self, instr: AddRegister) {
        // u32s are added to prevent overflow
        let value: u32 =
            self.registers[instr.sr1 as usize] as u32 + self.registers[instr.sr2 as usize] as u32;
        self.set_register(instr.dr, value as u16)
    }

    pub fn and_immediate(&mut self, instr: AndImmediate) {
        let value = self.registers[instr.sr1 as usize] & (instr.imm5 as u16);
        self.set_register(instr.dr, value as u16)
    }

    pub fn and_register(&mut self, instr: AndRegister) {
        let value = self.registers[instr.sr1 as usize] & self.registers[instr.sr2 as usize];
        self.set_register(instr.dr, value)
    }

    pub fn branch(&mut self, instr: Branch) {
        if (instr.nzp & self.cond).bits() > 0 {
            self.pc += instr.pc_offset9;
        }
    }

    pub fn jump(&mut self, instr: Jump) {
        self.pc = self.registers[instr.base_r as usize];
    }

    pub fn jump_subroutine_offset(&mut self, instr: JumpSubRoutineOffset) {
        self.registers[7] = self.pc;
        self.pc += instr.pc_offset11;
    }

    pub fn jump_subroutine_register(&mut self, instr: JumpSubRoutineRegister) {
        self.registers[7] = self.pc;
        self.pc = self.registers[instr.base_r as usize];
    }

    pub fn load(&mut self, instr: Load) {
        let address = self.pc + instr.pc_offset9;
        self.set_register(instr.dr, self.memory[address as usize]);
    }

    pub fn load_base_offset(&mut self, instr: LoadBaseOffset) {
        let address = self.registers[instr.base_r as usize] + instr.pc_offset6 as u16;
        self.set_register(instr.dr, self.memory[address as usize]);
    }

    pub fn load_effective_address(&mut self, instr: LoadEffectiveAddress) {
        let address = self.pc + instr.pc_offset9;
        self.set_register(instr.dr, address)
    }

    pub fn load_indirect(&mut self, instr: LoadIndirect) {
        let address = self.memory[(self.pc + instr.pc_offset9) as usize];
        self.set_register(instr.dr, self.memory[address as usize]);
    }

    /// Put `value` in `register` and set the cond register based on `value`
    pub fn set_register(&mut self, register: RegisterIndex, value: RegisterSize) {
        self.cond = match value {
            0 => CondFlag::ZERO,
            v if v >> 15 == 1 => CondFlag::NEGATIVE,
            _ => CondFlag::POSITIVE,
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
        let dr = 1;
        let sr1 = 2;
        let sr2 = 3;

        let instruction = Instruction::AddRegister(AddRegister { dr, sr1, sr2 }).encode();

        memory[PROGRAM_START as usize] = instruction;

        let mut machine = LC3::new(memory);
        machine.registers[sr1 as usize] = 5;
        machine.registers[sr2 as usize] = 6;
        machine.step();

        assert_eq!(machine.registers[dr as usize], 11);
        assert_eq!(machine.cond, CondFlag::POSITIVE);
    }

    #[test]
    fn add_immediate() {
        let mut memory = [0; MAX_MEMORY_SIZE];
        let dr = 1;
        let sr1 = 2;
        let imm5 = 6;

        let instruction = Instruction::AddImmediate(AddImmediate { dr, sr1, imm5 }).encode();

        memory[PROGRAM_START as usize] = instruction;

        let mut machine = LC3::new(memory);
        machine.registers[sr1 as usize] = 5;
        machine.step();

        assert_eq!(machine.registers[dr as usize], 11);
        assert_eq!(machine.cond, CondFlag::POSITIVE);
    }

    #[test]
    fn add_cond_flag_negative() {
        let mut memory = [0; MAX_MEMORY_SIZE];
        let dr = 1;
        let sr1 = 2;
        let sr2 = 3;

        let instruction = Instruction::AddRegister(AddRegister { dr, sr1, sr2 }).encode();

        memory[PROGRAM_START as usize] = instruction;

        let negative_one: u16 = 0xFFFF;
        let negative_two = 0xFFFE;

        let mut machine = LC3::new(memory);
        machine.registers[sr1 as usize] = negative_one;
        machine.registers[sr2 as usize] = negative_one;
        machine.step();

        assert_eq!(machine.registers[dr as usize], negative_two);
        assert_eq!(machine.cond, CondFlag::NEGATIVE);
    }

    #[test]
    fn add_cond_flag_zero() {
        let mut memory = [0; MAX_MEMORY_SIZE];
        let dr = 1;
        let sr1 = 2;
        let sr2 = 3;

        let instruction = Instruction::AddRegister(AddRegister { dr, sr1, sr2 }).encode();

        memory[PROGRAM_START as usize] = instruction;

        let negative_one: u16 = 0xFFFF;

        let mut machine = LC3::new(memory);
        machine.registers[sr1 as usize] = 1;
        machine.registers[sr2 as usize] = negative_one;
        machine.step();

        assert_eq!(machine.registers[dr as usize], 0);
        assert_eq!(machine.cond, CondFlag::ZERO);
    }

    #[test]
    fn sign_extension_add_immediate() {
        let mut memory = [0; MAX_MEMORY_SIZE];
        let dr = 1;
        let sr1 = 2;
        let imm5 = 0x1F; // negative one as 5 bits

        let instruction = Instruction::AddImmediate(AddImmediate { dr, sr1, imm5 }).encode();
        memory[PROGRAM_START as usize] = instruction;

        let mut machine = LC3::new(memory);
        machine.registers[sr1 as usize] = 1;
        machine.step();

        assert_eq!(machine.registers[dr as usize], 0);
        assert_eq!(machine.cond, CondFlag::ZERO);
    }

    #[test]
    fn load_indirect() {
        let mut memory = [0; MAX_MEMORY_SIZE];
        let dr = 1;
        let pc_offset9 = 10;

        let instruction = Instruction::LoadIndirect(LoadIndirect { dr, pc_offset9 }).encode();
        memory[PROGRAM_START as usize] = instruction;
        memory[PROGRAM_START as usize + 1 + 10] = 0xFFFE;
        memory[0xFFFE] = 17;

        let mut machine = LC3::new(memory);
        machine.step();

        assert_eq!(machine.registers[dr as usize], 17);
        assert_eq!(machine.cond, CondFlag::POSITIVE);
    }

    #[test]
    fn and_register() {
        let mut memory = [0; MAX_MEMORY_SIZE];
        let dr = 1;
        let sr1 = 2;
        let sr2 = 3;

        let instruction = Instruction::AndRegister(AndRegister { dr, sr1, sr2 }).encode();
        memory[PROGRAM_START as usize] = instruction;

        let mut machine = LC3::new(memory);
        machine.registers[sr1 as usize] = 0b0101;
        machine.registers[sr2 as usize] = 0b1110;
        machine.step();

        let expected = 0b0100;
        assert_eq!(machine.registers[dr as usize], expected);
        assert_eq!(machine.cond, CondFlag::POSITIVE);
    }

    #[test]
    fn and_immediate() {
        let mut memory = [0; MAX_MEMORY_SIZE];
        let dr = 1;
        let sr1 = 2;
        let imm5 = 0b10001;

        let instruction = Instruction::AndImmediate(AndImmediate { dr, sr1, imm5 }).encode();
        memory[PROGRAM_START as usize] = instruction;

        let mut machine = LC3::new(memory);
        machine.registers[sr1 as usize] = 0xFFF3;
        machine.step();

        let expected = 0xFFF1;
        assert_eq!(machine.registers[dr as usize], expected);
        assert_eq!(machine.cond, CondFlag::NEGATIVE);
    }

    #[test]
    fn branch() {
        let mut memory = [0; MAX_MEMORY_SIZE];
        let nzp = CondFlag::POSITIVE;
        let pc_offset9 = 10;

        let instruction = Instruction::Branch(Branch { nzp, pc_offset9 }).encode();
        memory[PROGRAM_START as usize] = instruction;

        let mut machine = LC3::new(memory);
        machine.cond = CondFlag::POSITIVE;
        machine.step();

        assert_eq!(machine.pc, PROGRAM_START + 11);
    }

    #[test]
    fn dont_branch() {
        let mut memory = [0; MAX_MEMORY_SIZE];
        let nzp = CondFlag::POSITIVE;
        let pc_offset9 = 10;

        let instruction = Instruction::Branch(Branch { nzp, pc_offset9 }).encode();
        memory[PROGRAM_START as usize] = instruction;

        let mut machine = LC3::new(memory);
        machine.cond = CondFlag::NEGATIVE;
        machine.step();

        assert_eq!(machine.pc, PROGRAM_START + 1);
    }

    #[test]
    fn jump() {
        let mut memory = [0; MAX_MEMORY_SIZE];
        let base_r = 1;

        let instruction = Instruction::Jump(Jump { base_r }).encode();
        memory[PROGRAM_START as usize] = instruction;

        let mut machine = LC3::new(memory);
        machine.registers[base_r as usize] = 0xFFFF;
        machine.step();

        assert_eq!(machine.pc, 0xFFFF);
    }

    #[test]
    fn jump_subroutine_offset() {
        let mut memory = [0; MAX_MEMORY_SIZE];
        let pc_offset11 = 10;

        let instruction =
            Instruction::JumpSubRoutineOffset(JumpSubRoutineOffset { pc_offset11 }).encode();
        memory[PROGRAM_START as usize] = instruction;

        let mut machine = LC3::new(memory);
        machine.step();

        assert_eq!(machine.pc, PROGRAM_START + 11);
        assert_eq!(machine.registers[7], PROGRAM_START + 1);
    }

    #[test]
    fn jump_subroutine_register() {
        let mut memory = [0; MAX_MEMORY_SIZE];
        let base_r = 1;

        let instruction =
            Instruction::JumpSubRoutineRegister(JumpSubRoutineRegister { base_r }).encode();
        memory[PROGRAM_START as usize] = instruction;

        let jump_to = 0xFFFF;
        let mut machine = LC3::new(memory);
        machine.registers[base_r as usize] = jump_to;
        machine.step();

        assert_eq!(machine.pc, 0xFFFF);
        assert_eq!(machine.registers[7], PROGRAM_START + 1);
    }

    #[test]
    fn load() {
        let mut memory = [0; MAX_MEMORY_SIZE];
        let dr = 1;
        let pc_offset9 = 10;

        let instruction = Instruction::Load(Load { dr, pc_offset9 }).encode();
        memory[PROGRAM_START as usize] = instruction;
        memory[PROGRAM_START as usize + 1 + 10] = 17;

        let mut machine = LC3::new(memory);
        machine.step();

        assert_eq!(machine.registers[dr as usize], 17);
        assert_eq!(machine.cond, CondFlag::POSITIVE);
    }

    #[test]
    fn load_base_offset() {
        let mut memory = [0; MAX_MEMORY_SIZE];
        let dr = 1;
        let base_r = 2;
        let pc_offset6 = 3;

        let instruction = Instruction::LoadBaseOffset(LoadBaseOffset {
            dr,
            base_r,
            pc_offset6,
        })
        .encode();
        memory[PROGRAM_START as usize] = instruction;
        memory[10] = 17;

        let mut machine = LC3::new(memory);
        machine.registers[base_r as usize] = 7;
        machine.step();

        assert_eq!(machine.registers[dr as usize], 17);
    }

    #[test]
    fn load_effective_address() {
        let mut memory = [0; MAX_MEMORY_SIZE];
        let dr = 1;
        let pc_offset9 = 10;

        let instruction =
            Instruction::LoadEffectiveAddress(LoadEffectiveAddress { dr, pc_offset9 }).encode();
        memory[PROGRAM_START as usize] = instruction;

        let mut machine = LC3::new(memory);
        machine.step();

        assert_eq!(machine.registers[dr as usize], PROGRAM_START + 11);
    }
}
