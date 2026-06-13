use crate::{
    Address, Hart,
    registers::GeneralRegisterName,
    vm::{VmOutput, VmResult},
};

pub fn execute_jal(rd: GeneralRegisterName, imm: i32, hart: &mut Hart) -> VmResult {
    let next_address = hart.registers.pc + 4;
    hart.registers.x[rd as usize] = next_address;
    let jump_target = hart.registers.pc as i64 + imm as i64;
    Ok(VmOutput::Jump(jump_target as Address))
}
