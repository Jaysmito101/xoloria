use crate::{
    Address, Hart,
    registers::GeneralRegisterName,
    vm::{VmOutput, VmResult},
};

#[inline]
pub fn execute_jal(rd: GeneralRegisterName, imm: i32, hart: &mut Hart) -> VmResult {
    let next_address = hart.registers.pc.wrapping_add(4);
    let jump_target = hart.registers.pc.wrapping_add_signed(imm as i64);
    hart.registers.x[rd as usize] = next_address;
    Ok(VmOutput::Jump(jump_target as Address))
}

#[inline]
pub fn execute_jalr(
    rd: GeneralRegisterName,
    rs1: GeneralRegisterName,
    imm: i32,
    hart: &mut Hart,
) -> VmResult {
    let base_address = hart.registers.x[rs1 as usize];
    let next_address = hart.registers.pc.wrapping_add(4);
    let jump_target = base_address.wrapping_add_signed(imm as i64) & (!1);
    hart.registers.x[rd as usize] = next_address;
    Ok(VmOutput::Jump(jump_target as Address))
}
