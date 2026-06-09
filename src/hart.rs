use crate::Result;

pub type Register = u64;

pub enum PrivilageMode {
    Machine = 0,
    Supervisor = 1,
    User = 2,
}

pub enum GeneralRegisterNames {
    Zero = 0,
    Ra = 1,
    Sp = 2,
    Gp = 3,
    Tp = 4,
    T0 = 5,
    T1 = 6,
    T2 = 7,
    S0 = 8,
    S1 = 9,
    A0 = 10,
    A1 = 11,
    A2 = 12,
    A3 = 13,
    A4 = 14,
    A5 = 15,
    A6 = 16,
    A7 = 17,
    S2 = 18,
    S3 = 19,
    S4 = 20,
    S5 = 21,
    S6 = 22,
    S7 = 23,
    S8 = 24,
    S9 = 25,
    S10 = 26,
    S11 = 27,
    T3 = 28,
    T4 = 29,
    T5 = 30,
    T6 = 31,
}

pub enum ControlRegisterNames {
    Mhartid = 0xF14,

    Mstatus = 0x300,
    Misa = 0x301,
    Medeleg = 0x302,
    Mideleg = 0x303,
    Mie = 0x304,
    Mtvec = 0x305,
    Mscratch = 0x340,
    Mepc = 0x341,
    Mcause = 0x342,
    Mtval = 0x343,
    Mip = 0x344,

    Stvec = 0x105,
    Sscratch = 0x140,
    Sepc = 0x141,
    Scause = 0x142,
    Stval = 0x143,
    Satp = 0x180,
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

    misa: Register,     // machine ISA and extensions
    mstatus: Register,  // machine status (interrupt enables, previous privilage mode)
    medeleg: Register,  // machine exception delegation
    mideleg: Register,  // machine interrupt deligation
    mie: Register,      // machine interrupt enable mask
    mtvec: Register,    // machine trap vector base address
    mscratch: Register, // machine scratch register
    mepc: Register,     // maching exception program counter
    mcause: Register,   // machine trap cause
    mtval: Register,    // machine bad address or instruction
    mip: Register,      // machine interrupt pending

    stvec: Register,    // supervisor trap vector base address
    sscratch: Register, // supervisor scratch register
    sepc: Register,     // supervisor exception program counter
    scause: Register,   // supervisor trap cause
    stval: Register,    // supervisor bad address or instruction
    satp: Register,     // supervisor address translation and protection
}

impl Hart {
    pub fn new(id: u64) -> Result<Self> {
        unimplemented!()
    }
}
