use crate::{
    Bus, BusIO, Result,
    instructions::Instruction,
    registers::{ISAExtensions, Misa, Register},
    vm::{self, VmOutput},
};

pub enum PrivilageMode {
    Machine = 0,
    Supervisor = 1,
    User = 2,
}

pub struct RegisterSet {
    pub(crate) pc: Register,
    pub(crate) x: [Register; 32],
    // the csr
    pub(crate) csr: [Register; 4096],
}

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

pub struct Hart {
    pub(crate) privilage_mode: PrivilageMode,
    pub(crate) registers: HartRegisters,
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
        let vm_result = match instruction {
            Instruction::Noop => Ok(VmOutput::NextInstruction),
            Instruction::Lui { rd, imm } => vm::load::execute_lui(rd, imm, self),
            Instruction::Jal { rd, imm } => vm::jump::execute_jal(rd, imm, self),
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
