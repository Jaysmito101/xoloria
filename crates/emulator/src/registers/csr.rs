use std::sync::atomic::Ordering;

use crate::{
    PrivilageMode,
    instructions::InstructionResult,
    registers::{AtomicRegister, ControlRegisterName, Register},
};

#[derive(Debug)]
pub struct ControlStatusRegisters {
    regs: [AtomicRegister; 4096],
}

impl ControlStatusRegisters {
    pub fn new() -> Self {
        Self {
            regs: [const { AtomicRegister::new(0) }; 4096],
        }
    }

    pub fn with(self, name: ControlRegisterName, value: Register) -> Self {
        self.regs[name as usize].store(value, std::sync::atomic::Ordering::SeqCst);
        self
    }

    pub fn read(
        &self,
        name: ControlRegisterName,
        privilage: PrivilageMode,
    ) -> InstructionResult<Register> {
        self.rmw(name, privilage, |value| value)
    }

    pub fn write(
        &self,
        name: ControlRegisterName,
        value: Register,
        privilage: PrivilageMode,
    ) -> InstructionResult<()> {
        self.rmw(name, privilage, |_| value).map(|_| ())
    }

    fn rmw<Op>(
        &self,
        name: ControlRegisterName,
        _privilage: PrivilageMode,
        op: Op,
    ) -> InstructionResult<Register>
    where
        Op: Fn(Register) -> Register,
    {
        let mut old_value = self.regs[name as usize].load(Ordering::Acquire);
        loop {
            let new_value = op(old_value);
            match self.regs[name as usize].compare_exchange(
                old_value,
                new_value,
                Ordering::AcqRel,
                Ordering::Acquire,
            ) {
                Ok(_) => break,
                Err(current) => old_value = current,
            }
        }
        Ok(old_value)
    }
}
