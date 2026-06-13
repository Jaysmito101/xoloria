use crate::{
    Hart,
    registers::GeneralRegisterName,
    vm::{VmOutput, VmResult},
};

pub fn execute_lui(rd: GeneralRegisterName, imm: i32, hart: &mut Hart) -> VmResult {
    hart.registers.x[rd as usize] = imm as u64;
    Ok(VmOutput::NextInstruction)
}
