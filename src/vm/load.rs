use crate::{
    Hart,
    registers::{GeneralRegisterName, Register},
    vm::{VmOutput, VmResult},
};

pub fn execute_lui(rd: GeneralRegisterName, imm: i32, hart: &mut Hart) -> VmResult {
    hart.registers.x[rd as usize] = imm as u64;
    Ok(VmOutput::NextInstruction)
}

pub fn execute_auipc(rd: GeneralRegisterName, imm: i32, hart: &mut Hart) -> VmResult {
    hart.registers.x[rd as usize] = (imm as i64 + hart.registers.pc as i64) as Register;
    Ok(VmOutput::NextInstruction)
}
