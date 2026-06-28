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
        use ControlRegisterName::*;

        if privilage < name.privilage_level() {
            return Err(RegisterError::UnprivilegedAccess(name, privilage));
        }

        match name {
            Mie | Mip | Mideleg | Medeleg | Mepc | Mcause | Mtval | Mtval2 | Mscratch => {
                Ok(self.regs[name as usize])
            }
            _ => Err(RegisterError::InvalidCSRRead(name, privilage)),
        }
    }

    pub fn write(
        &mut self,
        name: ControlRegisterName,
        value: Register,
        privilage: PrivilageMode,
    ) -> RegisterResult<()> {
        use ControlRegisterName::*;

        if privilage < name.privilage_level() {
            return Err(RegisterError::UnprivilegedAccess(name, privilage));
        }

        if name.is_read_only() {
            return Err(RegisterError::InvalidCSRWrite(name, value, privilage));
        }

        match name {
            Mie | Mip | Mideleg | Medeleg | Mepc | Mcause | Mtval | Mtval2 | Mscratch => {
                self.regs[name as usize] = value;
                Ok(())
            }
            _ => Err(RegisterError::InvalidCSRWrite(name, value, privilage)),
        }
    }
}

impl Default for ControlStatusRegisters {
    fn default() -> Self {
        Self::new()
    }
}
