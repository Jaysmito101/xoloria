use crate::{
    Bus, BusIO, Hart,
    registers::GeneralRegisterName,
    vm::{VmError, VmOutput, VmResult},
};

#[inline]
pub fn execute_lui(rd: GeneralRegisterName, imm: i32, hart: &mut Hart) -> VmResult {
    hart.registers.x[rd as usize] = imm as u64;
    Ok(VmOutput::NextInstruction)
}

#[inline]
pub fn execute_auipc(rd: GeneralRegisterName, imm: i32, hart: &mut Hart) -> VmResult {
    hart.registers.x[rd as usize] = hart.registers.pc.wrapping_add_signed(imm as i64);
    Ok(VmOutput::NextInstruction)
}

#[inline]
pub fn execute_lb(
    rd: GeneralRegisterName,
    rs1: GeneralRegisterName,
    imm: i32,
    hart: &mut Hart,
    bus: &Bus,
) -> VmResult {
    let address = hart.registers.x[rs1 as usize].wrapping_add_signed(imm as i64);
    let value = bus.read_u8(address).map_err(VmError::BusError)?;
    hart.registers.x[rd as usize] = value as i8 as u64;
    Ok(VmOutput::NextInstruction)
}

#[inline]
pub fn execute_lh(
    rd: GeneralRegisterName,
    rs1: GeneralRegisterName,
    imm: i32,
    hart: &mut Hart,
    bus: &Bus,
) -> VmResult {
    let address = hart.registers.x[rs1 as usize].wrapping_add_signed(imm as i64);
    let value = bus.read_u16(address).map_err(VmError::BusError)?;
    hart.registers.x[rd as usize] = value as i16 as u64;
    Ok(VmOutput::NextInstruction)
}

#[inline]
pub fn execute_lw(
    rd: GeneralRegisterName,
    rs1: GeneralRegisterName,
    imm: i32,
    hart: &mut Hart,
    bus: &Bus,
) -> VmResult {
    let address = hart.registers.x[rs1 as usize].wrapping_add_signed(imm as i64);
    let value = bus.read_u32(address).map_err(VmError::BusError)?;
    hart.registers.x[rd as usize] = value as i32 as u64;
    Ok(VmOutput::NextInstruction)
}

#[inline]
pub fn execute_lbu(
    rd: GeneralRegisterName,
    rs1: GeneralRegisterName,
    imm: i32,
    hart: &mut Hart,
    bus: &Bus,
) -> VmResult {
    let address = hart.registers.x[rs1 as usize].wrapping_add_signed(imm as i64);
    let value = bus.read_u8(address).map_err(VmError::BusError)?;
    hart.registers.x[rd as usize] = value as u64;
    Ok(VmOutput::NextInstruction)
}

#[inline]
pub fn execute_lhu(
    rd: GeneralRegisterName,
    rs1: GeneralRegisterName,
    imm: i32,
    hart: &mut Hart,
    bus: &Bus,
) -> VmResult {
    let address = hart.registers.x[rs1 as usize].wrapping_add_signed(imm as i64);
    let value = bus.read_u16(address).map_err(VmError::BusError)?;
    hart.registers.x[rd as usize] = value as u64;
    Ok(VmOutput::NextInstruction)
}

#[inline]
pub fn execute_ld(
    rd: GeneralRegisterName,
    rs1: GeneralRegisterName,
    imm: i32,
    hart: &mut Hart,
    bus: &Bus,
) -> VmResult {
    let address = hart.registers.x[rs1 as usize].wrapping_add_signed(imm as i64);
    let value = bus.read_u64(address).map_err(VmError::BusError)?;
    hart.registers.x[rd as usize] = value;
    Ok(VmOutput::NextInstruction)
}

#[inline]
pub fn execute_lwu(
    rd: GeneralRegisterName,
    rs1: GeneralRegisterName,
    imm: i32,
    hart: &mut Hart,
    bus: &Bus,
) -> VmResult {
    let address = hart.registers.x[rs1 as usize].wrapping_add_signed(imm as i64);
    let value = bus.read_u32(address).map_err(VmError::BusError)?;
    hart.registers.x[rd as usize] = value as u64;
    Ok(VmOutput::NextInstruction)
}
