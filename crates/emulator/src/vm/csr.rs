use crate::{
    Hart,
    registers::{ControlRegisterName, GeneralRegisterName},
    vm::{VmOutput, VmResult},
};

pub fn execute_csrrw(
    rd: GeneralRegisterName,
    rs1: GeneralRegisterName,
    csr: ControlRegisterName,
    hart: &mut Hart,
) -> VmResult {
    Ok(VmOutput::NextInstruction)
}

pub fn execute_csrrs(
    rd: GeneralRegisterName,
    rs1: GeneralRegisterName,
    csr: ControlRegisterName,
    hart: &mut Hart,
) -> VmResult {
    Ok(VmOutput::NextInstruction)
}

pub fn execute_csrrc(
    rd: GeneralRegisterName,
    rs1: GeneralRegisterName,
    csr: ControlRegisterName,
    hart: &mut Hart,
) -> VmResult {
    Ok(VmOutput::NextInstruction)
}

pub fn execute_csrrwi(
    rd: GeneralRegisterName,
    csr: ControlRegisterName,
    imm: u8,
    hart: &mut Hart,
) -> VmResult {
    Ok(VmOutput::NextInstruction)
}

pub fn execute_csrrsi(
    rd: GeneralRegisterName,
    csr: ControlRegisterName,
    imm: u8,
    hart: &mut Hart,
) -> VmResult {
    Ok(VmOutput::NextInstruction)
}

pub fn execute_csrrci(
    rd: GeneralRegisterName,
    csr: ControlRegisterName,
    imm: u8,
    hart: &mut Hart,
) -> VmResult {
    Ok(VmOutput::NextInstruction)
}
