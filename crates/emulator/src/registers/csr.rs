use crate::{
    PrivilageMode,
    registers::{ControlRegisterName, Register, RegisterError, RegisterResult},
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
    ) -> RegisterResult<Register> {
        match name {
            _ => Err(RegisterError::InvalidCSRRead(name, privilage)),
        }
        // Ok(self.regs[name as usize])
    }

    pub fn write(
        &mut self,
        name: ControlRegisterName,
        value: Register,
        privilage: PrivilageMode,
    ) -> RegisterResult<()> {
        match name {
            _ => Err(RegisterError::InvalidCSRWrite(name, value, privilage)),
        }
        // self.regs[name as usize] = value;
    }
}

impl Default for ControlStatusRegisters {
    fn default() -> Self {
        Self::new()
    }
}
