use crate::{Hart, registers::GeneralRegisterName, vm::VmResult};

pub fn execute_lui(rd: GeneralRegisterName, imm: i32, hart: &mut Hart) -> VmResult<()> {
    hart.registers.x[rd as usize] = imm as u64;
    Ok(())
}
