use crate::{
    Bus, BusIO, Hart,
    registers::GeneralRegisterName,
    vm::{VmError, VmOutput, VmResult},
};

#[inline]
pub fn execute_sb(
    rs1: GeneralRegisterName,
    rs2: GeneralRegisterName,
    imm: i32,
    hart: &mut Hart,
    bus: &Bus,
) -> VmResult {
    let address = hart.registers.x[rs1 as usize].wrapping_add_signed(imm as i64);
    let value = hart.registers.x[rs2 as usize];
    bus.write_u8(address, value as u8)
        .map_err(VmError::BusError)?;
    Ok(VmOutput::NextInstruction)
}

#[inline]
pub fn execute_sh(
    rs1: GeneralRegisterName,
    rs2: GeneralRegisterName,
    imm: i32,
    hart: &mut Hart,
    bus: &Bus,
) -> VmResult {
    let address = hart.registers.x[rs1 as usize].wrapping_add_signed(imm as i64);
    let value = hart.registers.x[rs2 as usize];
    bus.write_u16(address, value as u16)
        .map_err(VmError::BusError)?;
    Ok(VmOutput::NextInstruction)
}

#[inline]
pub fn execute_sw(
    rs1: GeneralRegisterName,
    rs2: GeneralRegisterName,
    imm: i32,
    hart: &mut Hart,
    bus: &Bus,
) -> VmResult {
    let address = hart.registers.x[rs1 as usize].wrapping_add_signed(imm as i64);
    let value = hart.registers.x[rs2 as usize];
    bus.write_u32(address, value as u32)
        .map_err(VmError::BusError)?;
    Ok(VmOutput::NextInstruction)
}

#[inline]
pub fn execute_sd(
    rs1: GeneralRegisterName,
    rs2: GeneralRegisterName,
    imm: i32,
    hart: &mut Hart,
    bus: &Bus,
) -> VmResult {
    let address = hart.registers.x[rs1 as usize].wrapping_add_signed(imm as i64);
    let value = hart.registers.x[rs2 as usize];
    bus.write_u64(address, value).map_err(VmError::BusError)?;
    Ok(VmOutput::NextInstruction)
}
