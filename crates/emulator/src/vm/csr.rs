use crate::{
    Hart,
    registers::{ControlRegisterName, GeneralRegisterName},
    vm::{VmError, VmOutput, VmResult},
};

pub fn execute_csrrw(
    rd: GeneralRegisterName,
    rs1: GeneralRegisterName,
    csr: ControlRegisterName,
    hart: &mut Hart,
) -> VmResult {
    let old = match rd {
        GeneralRegisterName::Zero => 0,
        _ => hart
            .registers
            .csr
            .read(csr, hart.privilage_mode)
            .map_err(|e| VmError::RegisterError(e))?,
    };
    hart.registers
        .csr
        .write(csr, hart.registers.x[rs1 as usize], hart.privilage_mode)
        .map_err(|e| VmError::RegisterError(e))?;
    hart.registers.x[rd as usize] = old;
    Ok(VmOutput::NextInstruction)
}

pub fn execute_csrrs(
    rd: GeneralRegisterName,
    rs1: GeneralRegisterName,
    csr: ControlRegisterName,
    hart: &mut Hart,
) -> VmResult {
    let old = hart
        .registers
        .csr
        .read(csr, hart.privilage_mode)
        .map_err(|e| VmError::RegisterError(e))?;
    let new = old | hart.registers.x[rs1 as usize];
    hart.registers.x[rd as usize] = old;
    if rs1 != GeneralRegisterName::Zero {
        hart.registers
            .csr
            .write(csr, new, hart.privilage_mode)
            .map_err(|e| VmError::RegisterError(e))?;
    }
    Ok(VmOutput::NextInstruction)
}

pub fn execute_csrrc(
    rd: GeneralRegisterName,
    rs1: GeneralRegisterName,
    csr: ControlRegisterName,
    hart: &mut Hart,
) -> VmResult {
    let old = hart
        .registers
        .csr
        .read(csr, hart.privilage_mode)
        .map_err(|e| VmError::RegisterError(e))?;
    let new = old & !hart.registers.x[rs1 as usize];
    hart.registers.x[rd as usize] = old;
    if rs1 != GeneralRegisterName::Zero {
        hart.registers
            .csr
            .write(csr, new, hart.privilage_mode)
            .map_err(|e| VmError::RegisterError(e))?;
    }
    Ok(VmOutput::NextInstruction)
}

pub fn execute_csrrwi(
    rd: GeneralRegisterName,
    csr: ControlRegisterName,
    imm: u8,
    hart: &mut Hart,
) -> VmResult {
    let old = match rd {
        GeneralRegisterName::Zero => 0,
        _ => hart
            .registers
            .csr
            .read(csr, hart.privilage_mode)
            .map_err(|e| VmError::RegisterError(e))?,
    };
    hart.registers
        .csr
        .write(csr, imm as u64, hart.privilage_mode)
        .map_err(|e| VmError::RegisterError(e))?;
    hart.registers.x[rd as usize] = old;
    Ok(VmOutput::NextInstruction)
}

pub fn execute_csrrsi(
    rd: GeneralRegisterName,
    csr: ControlRegisterName,
    imm: u8,
    hart: &mut Hart,
) -> VmResult {
    let old = hart
        .registers
        .csr
        .read(csr, hart.privilage_mode)
        .map_err(|e| VmError::RegisterError(e))?;
    let new = old | (imm as u64);
    hart.registers.x[rd as usize] = old;
    if imm != 0 {
        hart.registers
            .csr
            .write(csr, new, hart.privilage_mode)
            .map_err(|e| VmError::RegisterError(e))?;
    }
    Ok(VmOutput::NextInstruction)
}

pub fn execute_csrrci(
    rd: GeneralRegisterName,
    csr: ControlRegisterName,
    imm: u8,
    hart: &mut Hart,
) -> VmResult {
    let old = hart
        .registers
        .csr
        .read(csr, hart.privilage_mode)
        .map_err(|e| VmError::RegisterError(e))?;
    let new = old & !imm as u64;
    hart.registers.x[rd as usize] = old;
    if imm != 0 {
        hart.registers
            .csr
            .write(csr, new, hart.privilage_mode)
            .map_err(|e| VmError::RegisterError(e))?;
    }
    Ok(VmOutput::NextInstruction)
}
