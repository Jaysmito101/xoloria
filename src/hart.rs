use std::fmt::Display;

use crate::{
    Bus, BusIO, Result,
    instructions::Instruction,
    registers::{GeneralRegisterName, ISAExtensions, Misa, Register},
    vm::{self, VmOutput},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PrivilageMode {
    Machine = 0,
    Supervisor = 1,
    User = 2,
}

#[derive(Debug)]
pub struct HartRegisters {
    pub(crate) pc: Register,
    pub(crate) x: [Register; 32],

    pub(crate) load_reservation_valid: bool,
    pub(crate) load_reservation_address: Register,

    pub(crate) mhartid: Register, // hardware thread id

    pub(crate) misa: Register, // machine ISA and extensions

    pub(crate) mstatus: Register, // machine status (interrupt enables, previous privilage mode)
    pub(crate) medeleg: Register, // machine exception delegation
    pub(crate) mideleg: Register, // machine interrupt deligation
    pub(crate) mie: Register,     // machine interrupt enable mask
    pub(crate) mtvec: Register,   // machine trap vector base address
    pub(crate) mscratch: Register, // machine scratch register
    pub(crate) mepc: Register,    // maching exception program counter
    pub(crate) mcause: Register,  // machine trap cause
    pub(crate) mtval: Register,   // machine bad address or instruction
    pub(crate) mip: Register,     // machine interrupt pending

    pub(crate) stvec: Register,    // supervisor trap vector base address
    pub(crate) sscratch: Register, // supervisor scratch register
    pub(crate) sepc: Register,     // supervisor exception program counter
    pub(crate) scause: Register,   // supervisor trap cause
    pub(crate) stval: Register,    // supervisor bad address or instruction
    pub(crate) satp: Register,     // supervisor address translation and protection
}

impl Display for HartRegisters {
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
        writeln!(f, " }}")?;
        write!(
            f,
            "System Registers: {{ mhartid: {:#x} ({}), ",
            self.mhartid, self.mhartid
        )?;
        write!(f, "misa: {:#x} ({}), ", self.misa, self.misa)?;
        write!(f, "mstatus: {:#x} ({}), ", self.mstatus, self.mstatus)?;
        write!(f, "medeleg: {:#x} ({}), ", self.medeleg, self.medeleg)?;
        write!(f, "mideleg: {:#x} ({}), ", self.mideleg, self.mideleg)?;
        write!(f, "mie: {:#x} ({}), ", self.mie, self.mie)?;
        write!(f, "mtvec: {:#x} ({}), ", self.mtvec, self.mtvec)?;
        write!(f, "mscratch: {:#x} ({}), ", self.mscratch, self.mscratch)?;
        write!(f, "mepc: {:#x} ({}), ", self.mepc, self.mepc)?;
        write!(f, "mcause: {:#x} ({}), ", self.mcause, self.mcause)?;
        write!(f, "mtval: {:#x} ({}), ", self.mtval, self.mtval)?;
        write!(f, "mip: {:#x} ({}), ", self.mip, self.mip)?;
        write!(f, "stvec: {:#x} ({}), ", self.stvec, self.stvec)?;
        write!(f, "sscratch: {:#x} ({}), ", self.sscratch, self.sscratch)?;
        write!(f, "sepc: {:#x} ({}), ", self.sepc, self.sepc)?;
        write!(f, "scause: {:#x} ({}), ", self.scause, self.scause)?;
        write!(f, "stval: {:#x} ({}), ", self.stval, self.stval)?;
        write!(f, "satp: {:#x} ({}) }}", self.satp, self.satp)?;
        write!(f, "}}")?;

        Ok(())
    }
}

#[derive(Debug)]
pub struct Hart {
    pub(crate) privilage_mode: PrivilageMode,
    pub(crate) registers: HartRegisters,
}

impl Display for Hart {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Privilage Mode: {:?}", self.privilage_mode)?;
        writeln!(f, "Registers:\n{}", self.registers)?;
        Ok(())
    }
}

impl Hart {
    pub fn new(id: u64) -> Result<Self> {
        Ok(Self {
            privilage_mode: PrivilageMode::Machine,

            registers: HartRegisters {
                pc: 0x80000000,
                x: [0; 32],

                load_reservation_valid: false,
                load_reservation_address: 0,

                mhartid: id,

                misa: Misa::default()
                    .with_xlen(64)
                    .with_extension(ISAExtensions::I)
                    .with_extension(ISAExtensions::M)
                    .with_extension(ISAExtensions::A)
                    .register(),

                mstatus: 0,
                medeleg: 0,
                mideleg: 0,
                mie: 0,
                mtvec: 0,
                mscratch: 0,
                mepc: 0,
                mcause: 0,
                mtval: 0,
                mip: 0,

                stvec: 0,
                sscratch: 0,
                sepc: 0,
                scause: 0,
                stval: 0,
                satp: 0,
            },
        })
    }

    pub fn id(&self) -> u64 {
        self.registers.mhartid
    }

    pub fn tick(&mut self, bus: &Bus) -> Result<()> {
        let instruction_value = bus.read_u32(self.registers.pc)?;
        let instruction = Instruction::try_from(instruction_value)?;
        tracing::warn!("[{:#x}] {}", self.registers.pc, instruction);
        self.registers.x[0] = 0; // enforce zero being zero
        let vm_result = match instruction {
            Instruction::Noop => Ok(VmOutput::NextInstruction),
            Instruction::Lui { rd, imm } => vm::load::execute_lui(rd, imm, self),
            Instruction::Auipc { rd, imm } => vm::load::execute_auipc(rd, imm, self),
            Instruction::Jal { rd, imm } => vm::jump::execute_jal(rd, imm, self),
            Instruction::Jalr { rd, rs1, imm } => vm::jump::execute_jalr(rd, rs1, imm, self),

            // a way to debug register state with this for now
            Instruction::Ecall => {
                tracing::info!("{}", &self);
                Ok(VmOutput::NextInstruction)
            }
            _ => unimplemented!("Instruction {:?} not implemented", instruction),
        };

        match vm_result {
            Ok(output) => match output {
                VmOutput::NextInstruction => self.registers.pc += 4,
                VmOutput::Jump(target) => self.registers.pc = target,
            },
            Err(err) => match err {},
        }

        Ok(())
    }
}
