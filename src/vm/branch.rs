use crate::{
    Address, Hart,
    registers::GeneralRegisterName,
    vm::{VmOutput, VmResult},
};

#[inline]
pub fn execute_beq(
    rs1: GeneralRegisterName,
    rs2: GeneralRegisterName,
    imm: i32,
    hart: &mut Hart,
) -> VmResult {
    if hart.registers.x[rs1 as usize] == hart.registers.x[rs2 as usize] {
        let target_address = hart.registers.pc.wrapping_add_signed(imm as i64);
        Ok(VmOutput::Jump(target_address as Address))
    } else {
        Ok(VmOutput::NextInstruction)
    }
}

#[inline]
pub fn execute_bne(
    rs1: GeneralRegisterName,
    rs2: GeneralRegisterName,
    imm: i32,
    hart: &mut Hart,
) -> VmResult {
    if hart.registers.x[rs1 as usize] != hart.registers.x[rs2 as usize] {
        let target_address = hart.registers.pc.wrapping_add_signed(imm as i64);
        Ok(VmOutput::Jump(target_address as Address))
    } else {
        Ok(VmOutput::NextInstruction)
    }
}

#[inline]
pub fn execute_blt(
    rs1: GeneralRegisterName,
    rs2: GeneralRegisterName,
    imm: i32,
    hart: &mut Hart,
) -> VmResult {
    if (hart.registers.x[rs1 as usize] as i64) < (hart.registers.x[rs2 as usize] as i64) {
        let target_address = hart.registers.pc.wrapping_add_signed(imm as i64);
        Ok(VmOutput::Jump(target_address as Address))
    } else {
        Ok(VmOutput::NextInstruction)
    }
}

#[inline]
pub fn execute_bge(
    rs1: GeneralRegisterName,
    rs2: GeneralRegisterName,
    imm: i32,
    hart: &mut Hart,
) -> VmResult {
    if (hart.registers.x[rs1 as usize] as i64) >= (hart.registers.x[rs2 as usize] as i64) {
        let target_address = hart.registers.pc.wrapping_add_signed(imm as i64);
        Ok(VmOutput::Jump(target_address as Address))
    } else {
        Ok(VmOutput::NextInstruction)
    }
}

#[inline]
pub fn execute_bltu(
    rs1: GeneralRegisterName,
    rs2: GeneralRegisterName,
    imm: i32,
    hart: &mut Hart,
) -> VmResult {
    if hart.registers.x[rs1 as usize] < hart.registers.x[rs2 as usize] {
        let target_address = hart.registers.pc.wrapping_add_signed(imm as i64);
        Ok(VmOutput::Jump(target_address as Address))
    } else {
        Ok(VmOutput::NextInstruction)
    }
}

#[inline]
pub fn execute_bgeu(
    rs1: GeneralRegisterName,
    rs2: GeneralRegisterName,
    imm: i32,
    hart: &mut Hart,
) -> VmResult {
    if hart.registers.x[rs1 as usize] >= hart.registers.x[rs2 as usize] {
        let target_address = hart.registers.pc.wrapping_add_signed(imm as i64);
        Ok(VmOutput::Jump(target_address as Address))
    } else {
        Ok(VmOutput::NextInstruction)
    }
}
