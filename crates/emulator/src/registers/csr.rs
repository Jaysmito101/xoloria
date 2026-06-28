use crate::{
    PrivilageMode,
    instructions::InstructionResult,
    registers::{ControlRegisterName, Register},
};

#[derive(Debug)]
pub struct ControlStatusRegisters {
    regs: [Register; 4096],
}

impl ControlStatusRegisters {
    pub fn new() -> Self {
        Self { regs: [0; 4096] }
    }

    pub fn with(mut self, name: ControlRegisterName, value: Register) -> Self {
        self.regs[name as usize] = value;
        self
    }

    pub fn read(
        &self,
        name: ControlRegisterName,
        privilage: PrivilageMode,
    ) -> InstructionResult<Register> {
        Ok(self.regs[name as usize])
    }

    pub fn write(
        &mut self,
        name: ControlRegisterName,
        value: Register,
        privilage: PrivilageMode,
    ) -> InstructionResult<()> {
        self.regs[name as usize] = value;
        Ok(())
    }
}

impl Default for ControlStatusRegisters {
    fn default() -> Self {
        Self::new()
    }
}
