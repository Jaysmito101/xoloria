use crate::{
    Hart,
    registers::GeneralRegisterName,
    vm::{VmOutput, VmResult},
};

#[inline]
pub fn execute_addi(
    rd: GeneralRegisterName,
    rs1: GeneralRegisterName,
    imm: i32,
    hart: &mut Hart,
) -> VmResult {
    let result = (hart.registers.x[rs1 as usize] as i64).wrapping_add(imm as i64);
    hart.registers.x[rd as usize] = result as u64;
    Ok(VmOutput::NextInstruction)
}

#[inline]
pub fn execute_slti(
    rd: GeneralRegisterName,
    rs1: GeneralRegisterName,
    imm: i32,
    hart: &mut Hart,
) -> VmResult {
    let value = hart.registers.x[rs1 as usize] as i64;
    hart.registers.x[rd as usize] = if value < imm as i64 { 1 } else { 0 };
    Ok(VmOutput::NextInstruction)
}

#[inline]
pub fn execute_sltiu(
    rd: GeneralRegisterName,
    rs1: GeneralRegisterName,
    imm: i32,
    hart: &mut Hart,
) -> VmResult {
    let value = hart.registers.x[rs1 as usize];
    hart.registers.x[rd as usize] = if value < imm as i64 as u64 { 1 } else { 0 };
    Ok(VmOutput::NextInstruction)
}

#[inline]
pub fn execute_slli(
    rd: GeneralRegisterName,
    rs1: GeneralRegisterName,
    imm: u8,
    hart: &mut Hart,
) -> VmResult {
    let shift_amount = imm & 0x3F;
    hart.registers.x[rd as usize] = hart.registers.x[rs1 as usize] << shift_amount;
    Ok(VmOutput::NextInstruction)
}

#[inline]
pub fn execute_srli(
    rd: GeneralRegisterName,
    rs1: GeneralRegisterName,
    imm: u8,
    hart: &mut Hart,
) -> VmResult {
    let shift_amount = imm & 0x3F;
    hart.registers.x[rd as usize] = hart.registers.x[rs1 as usize] >> shift_amount;
    Ok(VmOutput::NextInstruction)
}

#[inline]
pub fn execute_srai(
    rd: GeneralRegisterName,
    rs1: GeneralRegisterName,
    imm: u8,
    hart: &mut Hart,
) -> VmResult {
    let shift_amount = imm & 0x3F;
    let value = hart.registers.x[rs1 as usize] as i64;
    hart.registers.x[rd as usize] = (value >> shift_amount) as u64;
    Ok(VmOutput::NextInstruction)
}

#[inline]
pub fn execute_xori(
    rd: GeneralRegisterName,
    rs1: GeneralRegisterName,
    imm: i32,
    hart: &mut Hart,
) -> VmResult {
    hart.registers.x[rd as usize] = hart.registers.x[rs1 as usize] ^ (imm as u64);
    Ok(VmOutput::NextInstruction)
}

#[inline]
pub fn execute_ori(
    rd: GeneralRegisterName,
    rs1: GeneralRegisterName,
    imm: i32,
    hart: &mut Hart,
) -> VmResult {
    hart.registers.x[rd as usize] = hart.registers.x[rs1 as usize] | (imm as u64);
    Ok(VmOutput::NextInstruction)
}

#[inline]
pub fn execute_andi(
    rd: GeneralRegisterName,
    rs1: GeneralRegisterName,
    imm: i32,
    hart: &mut Hart,
) -> VmResult {
    hart.registers.x[rd as usize] = hart.registers.x[rs1 as usize] & (imm as u64);
    Ok(VmOutput::NextInstruction)
}

#[inline]
pub fn execute_addiw(
    rd: GeneralRegisterName,
    rs1: GeneralRegisterName,
    imm: i32,
    hart: &mut Hart,
) -> VmResult {
    let result = (hart.registers.x[rs1 as usize] as i32).wrapping_add(imm);
    hart.registers.x[rd as usize] = result as i64 as u64;
    Ok(VmOutput::NextInstruction)
}

#[inline]
pub fn execute_slliw(
    rd: GeneralRegisterName,
    rs1: GeneralRegisterName,
    imm: u8,
    hart: &mut Hart,
) -> VmResult {
    let shift_amount = imm & 0x1F;
    let value = hart.registers.x[rs1 as usize] as i32 as u32;
    hart.registers.x[rd as usize] = (value << shift_amount) as i64 as u64;
    Ok(VmOutput::NextInstruction)
}

#[inline]
pub fn execute_srliw(
    rd: GeneralRegisterName,
    rs1: GeneralRegisterName,
    imm: u8,
    hart: &mut Hart,
) -> VmResult {
    let shift_amount = imm & 0x1F;
    let value = hart.registers.x[rs1 as usize] as i32 as u32;
    hart.registers.x[rd as usize] = (value >> shift_amount) as i64 as u64;
    Ok(VmOutput::NextInstruction)
}

#[inline]
pub fn execute_sraiw(
    rd: GeneralRegisterName,
    rs1: GeneralRegisterName,
    imm: u8,
    hart: &mut Hart,
) -> VmResult {
    let shift_amount = imm & 0x1F;
    let value = hart.registers.x[rs1 as usize] as i32;
    hart.registers.x[rd as usize] = (value >> shift_amount) as i64 as u64;
    Ok(VmOutput::NextInstruction)
}
