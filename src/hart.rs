use crate::{
    Result,
    registers::{ISAExtensions, Misa, Register},
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

pub struct Hart {
    privilage_mode: PrivilageMode,

    pc: Register,
    x: [Register; 32],

    load_reservation_valid: bool,
    load_reservation_address: Register,

    mhartid: Register, // hardware thread id

    misa: Register, // machine ISA and extensions

    mstatus: Register, // machine status (interrupt enables, previous privilage mode)
    medeleg: Register, // machine exception delegation
    mideleg: Register, // machine interrupt deligation
    mie: Register,     // machine interrupt enable mask
    mtvec: Register,   // machine trap vector base address
    mscratch: Register, // machine scratch register
    mepc: Register,    // maching exception program counter
    mcause: Register,  // machine trap cause
    mtval: Register,   // machine bad address or instruction
    mip: Register,     // machine interrupt pending

    stvec: Register,    // supervisor trap vector base address
    sscratch: Register, // supervisor scratch register
    sepc: Register,     // supervisor exception program counter
    scause: Register,   // supervisor trap cause
    stval: Register,    // supervisor bad address or instruction
    satp: Register,     // supervisor address translation and protection
}

impl Hart {
    pub fn new(id: u64) -> Result<Self> {
        Ok(Self {
            privilage_mode: PrivilageMode::Machine,

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
        })
    }

    pub fn id(&self) -> u64 {
        self.mhartid
    }

    pub fn set_pc(&mut self, pc: Register) {
        self.pc = pc;
    }
}
