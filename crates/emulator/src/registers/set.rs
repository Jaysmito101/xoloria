use std::fmt::Display;

use strum::IntoEnumIterator;

use crate::{
    PrivilageMode,
    registers::{ControlRegisterName, ControlStatusRegisters, GeneralRegisterName, Register},
};

#[derive(Debug)]
pub struct RegisterSet {
    pub(crate) pc: Register,
    pub(crate) x: [Register; 32],
    pub(crate) csr: ControlStatusRegisters,

    pub(crate) _load_reservation_valid: bool,
    pub(crate) _load_reservation_address: Register,
}

impl Display for RegisterSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Program Counter: {:#x}", self.pc)?;
        write!(f, "General Registers: {{ ")?;
        for (i, &value) in self.x.iter().enumerate() {
            write!(
                f,
                "{}: {:#x} ({}){}",
                GeneralRegisterName::try_from(i as u8).unwrap_or(GeneralRegisterName::Zero),
                // i,
                value,
                value,
                if i != self.x.len() - 1 { ", " } else { "" }
            )?;
        }

        writeln!(f, "System Registers: {{")?;
        for name in ControlRegisterName::iter() {
            writeln!(
                f,
                "    {}: {:#x} ({})",
                name,
                self.csr
                    .read(name, PrivilageMode::Machine)
                    .expect("Machine mode should always be able to read CSRs"),
                self.csr
                    .read(name, PrivilageMode::Machine)
                    .expect("Machine mode should always be able to read CSRs")
            )?;
        }
        writeln!(f, "}}")?;

        Ok(())
    }
}

impl RegisterSet {
    pub fn pc(&self) -> Register {
        self.pc
    }

    pub fn x(&self) -> &[Register; 32] {
        &self.x
    }

    pub fn csr(&self) -> &ControlStatusRegisters {
        &self.csr
    }
}
