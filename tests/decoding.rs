use lilc3::{
    instruction::{AddRegister, Instruction},
    CondFlag, LC3,
};

#[test]
fn decoding() {
    let dr = 1;
    let sr1 = 2;
    let sr2 = 3;

    let add = Instruction::AddRegister(AddRegister { dr, sr1, sr2 }).encode();
    let add_instruction_bytes = add.to_be_bytes();

    let origin: u16 = 0xF;
    let origin: [u8; 2] = origin.to_be_bytes();
    let instructions = [origin, add_instruction_bytes].concat();

    let mut machine = LC3::new(&instructions);
    machine.registers[sr1 as usize] = 5;
    machine.registers[sr2 as usize] = 6;
    machine.step();

    assert_eq!(machine.registers[dr as usize], 11);
    assert_eq!(machine.cond, CondFlag::POSITIVE);
}
