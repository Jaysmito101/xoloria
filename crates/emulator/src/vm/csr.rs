use crate::{
    Hart,
    registers::{ControlRegisterName, GeneralRegisterName, Register},
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
            .map_err(VmError::RegisterError)?,
    };
    hart.registers
        .csr
        .write(csr, hart.registers.x[rs1 as usize], hart.privilage_mode)
        .map_err(VmError::RegisterError)?;
    hart.registers.x[rd as usize] = match csr {
        ControlRegisterName::Mip => {
            // https://docs.riscv.org/reference/isa/v20260120/priv/machine.html#3-1-1-9-machine-interrupt-mip-and-mie-registers
            // set the bit 9 (seip) of mip to be (old[9] | hart.supervisor_ext_interrupt_pending())
            old | ((hart.supervisor_ext_interrupt_pending() as Register) << 9)
        }
        _ => old,
    };
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
        .map_err(VmError::RegisterError)?;
    let new = old | hart.registers.x[rs1 as usize];
    if rs1 != GeneralRegisterName::Zero {
        hart.registers
            .csr
            .write(csr, new, hart.privilage_mode)
            .map_err(VmError::RegisterError)?;
    }
    hart.registers.x[rd as usize] = match csr {
        ControlRegisterName::Mip => {
            // https://docs.riscv.org/reference/isa/v20260120/priv/machine.html#3-1-1-9-machine-interrupt-mip-and-mie-registers
            // set the bit 9 (seip) of mip to be (old[9] | hart.supervisor_ext_interrupt_pending())
            old | ((hart.supervisor_ext_interrupt_pending() as Register) << 9)
        }
        _ => old,
    };
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
        .map_err(VmError::RegisterError)?;
    let new = old & !hart.registers.x[rs1 as usize];
    if rs1 != GeneralRegisterName::Zero {
        hart.registers
            .csr
            .write(csr, new, hart.privilage_mode)
            .map_err(VmError::RegisterError)?;
    }
    hart.registers.x[rd as usize] = match csr {
        ControlRegisterName::Mip => {
            // https://docs.riscv.org/reference/isa/v20260120/priv/machine.html#3-1-1-9-machine-interrupt-mip-and-mie-registers
            // set the bit 9 (seip) of mip to be (old[9] | hart.supervisor_ext_interrupt_pending())
            old | ((hart.supervisor_ext_interrupt_pending() as Register) << 9)
        }
        _ => old,
    };
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
            .map_err(VmError::RegisterError)?,
    };
    hart.registers
        .csr
        .write(csr, imm as u64, hart.privilage_mode)
        .map_err(VmError::RegisterError)?;
    hart.registers.x[rd as usize] = match csr {
        ControlRegisterName::Mip => {
            // https://docs.riscv.org/reference/isa/v20260120/priv/machine.html#3-1-1-9-machine-interrupt-mip-and-mie-registers
            // set the bit 9 (seip) of mip to be (old[9] | hart.supervisor_ext_interrupt_pending())
            old | ((hart.supervisor_ext_interrupt_pending() as Register) << 9)
        }
        _ => old,
    };
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
        .map_err(VmError::RegisterError)?;
    let new = old | (imm as u64);
    if imm != 0 {
        hart.registers
            .csr
            .write(csr, new, hart.privilage_mode)
            .map_err(VmError::RegisterError)?;
    }
    hart.registers.x[rd as usize] = match csr {
        ControlRegisterName::Mip => {
            // https://docs.riscv.org/reference/isa/v20260120/priv/machine.html#3-1-1-9-machine-interrupt-mip-and-mie-registers
            // set the bit 9 (seip) of mip to be (old[9] | hart.supervisor_ext_interrupt_pending())
            old | ((hart.supervisor_ext_interrupt_pending() as Register) << 9)
        }
        _ => old,
    };
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
        .map_err(VmError::RegisterError)?;
    let new = old & !imm as u64;
    if imm != 0 {
        hart.registers
            .csr
            .write(csr, new, hart.privilage_mode)
            .map_err(VmError::RegisterError)?;
    }
    hart.registers.x[rd as usize] = match csr {
        ControlRegisterName::Mip => {
            // https://docs.riscv.org/reference/isa/v20260120/priv/machine.html#3-1-1-9-machine-interrupt-mip-and-mie-registers
            // set the bit 9 (seip) of mip to be (old[9] | hart.supervisor_ext_interrupt_pending())
            old | ((hart.supervisor_ext_interrupt_pending() as Register) << 9)
        }
        _ => old,
    };
    Ok(VmOutput::NextInstruction)
}
