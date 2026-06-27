use std::fmt::Display;

use strum::{EnumIter, IntoEnumIterator};

#[derive(EnumIter, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GeneralRegisterName {
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

#[derive(EnumIter, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ControlRegisterName {
    // unprivileged floating point CSRs
    Fflags = 0x001, // floating-point accured exceptions
    Frm = 0x002,    // floating-point dynamic rounding mode
    Fcsr = 0x003,   // floating point control and status register (frm + fflags)

    // unprivileged vector CSRs
    Vstart = 0x008, // vector start position
    Vxsat = 0x009,  // fixed-point accured saturation flag
    Vxrm = 0x00A,   // fixed-point rounding mode
    Vcsr = 0x00F,   // vector control and status register (vxsat + vxrm)
    Vl = 0xC20,     // vector length
    Vtype = 0xC21,  // vector data type
    Vlenb = 0xC22,  // vector register length in bytes

    // unprivileged zicfiss extension CSR
    // Ssp = 0x011, // shadow stack pointer

    // unprevileged entry source extension CSR
    // Seed = 0x015, // seed for cryptographic instructions

    // Unprivileged Zcmt extension CSR
    // Jvt = 0x017, // table jump base vector and control register

    // unprivileged counters/timers
    Cycle = 0xC00,       // cycle counter for RDCYCLE instruction
    Time = 0xC01,        // timer for RDTIME instruction
    Instret = 0xC02,     // instructions-retired counter for RDINSTRET
    Hpmcounter3 = 0xC03, // performance-monitoring counter
    Hpmcounter4 = 0xC04,
    Hpmcounter5 = 0xC05,
    Hpmcounter6 = 0xC06,
    Hpmcounter7 = 0xC07,
    Hpmcounter8 = 0xC08,
    Hpmcounter9 = 0xC09,
    Hpmcounter10 = 0xC0A,
    Hpmcounter11 = 0xC0B,
    Hpmcounter12 = 0xC0C,
    Hpmcounter13 = 0xC0D,
    Hpmcounter14 = 0xC0E,
    Hpmcounter15 = 0xC0F,
    Hpmcounter16 = 0xC10,
    Hpmcounter17 = 0xC11,
    Hpmcounter18 = 0xC12,
    Hpmcounter19 = 0xC13,
    Hpmcounter20 = 0xC14,
    Hpmcounter21 = 0xC15,
    Hpmcounter22 = 0xC16,
    Hpmcounter23 = 0xC17,
    Hpmcounter24 = 0xC18,
    Hpmcounter25 = 0xC19,
    Hpmcounter26 = 0xC1A,
    Hpmcounter27 = 0xC1B,
    Hpmcounter28 = 0xC1C,
    Hpmcounter29 = 0xC1D,
    Hpmcounter30 = 0xC1E,
    Hpmcounter31 = 0xC1F,

    // supervisor level csr

    // supervisor trap setup
    Sstatus = 0x100,    // supervisor status register
    Sie = 0x104,        // supervisor interrupt-enable register
    Stvec = 0x105,      // supervisor trap handler base address
    Scounteren = 0x106, // supervisor counter enable

    // supervisor configuration
    Sencfg = 0x10A, // supervisor environment configuration

    // supervisor counter setup
    Scountinhibit = 0x120, // supervisor counter-inhibit register

    // supervisor trap handling
    Sscratch = 0x140,  // supervisor scratch register
    Sepc = 0x141,      // supervisor exception program counter
    Scause = 0x142,    // supervisor trap cause
    Stval = 0x143,     // supervisor bad address or instruction
    Sip = 0x144,       // supervisor interrupt pending
    Scountovf = 0xDA0, // supervisor counter overflow

    // supervisor indirect
    Siselect = 0x150, // supervisor indiect register select
    Sireg = 0x151,    // supervisor indirect register alias
    Sireg2 = 0x152,   // supervisor indirect register alias 2
    Sireg3 = 0x153,   // supervisor indirect register alias 3
    Sireg4 = 0x154,   // supervisor indirect register alias 4
    Sireg5 = 0x155,   // supervisor indirect register alias 5\
    Sireg6 = 0x156,   // supervisor indirect register alias 6

    // sipervisor protection and translation
    Satp = 0x180, // supervisor adddress translation and protection

    // supervisor timer compare
    Stimecmp = 0x14D, // supervisor timer compare

    // debug/trace registers
    Scontext = 0x5A8, // supervisor context register

    // supervixsor resource management configuration
    Srmcfg = 0x181, // supervisor resource management configuration

    // supervisor state enable registers
    Sstateen0 = 0x10C, // supervisor state enable register 0
    Sstateen1 = 0x10D, // supervisor state enable register 1
    Sstateen2 = 0x10E, // supervisor state enable register 2
    Sstateen3 = 0x10F, // supervisor state enable register 3

    // supervisor control transfer records configuration
    Sctrctl = 0x14E,    // supervisor control transfer records control register
    Sctrstatus = 0x14F, // supervisor control transfer records status register
    Sctrdepth = 0x15F,  // supervisor control transfer records depth register

    // machine level csr

    // machine information registers
    Mvendorid = 0xF11,  // vendor ID
    Marchid = 0xF12,    // architecture ID
    Mimpid = 0xF13,     // implementation ID
    Mhartid = 0xF14,    // hardware thread ID
    Mconfigptr = 0xF15, // pointer to configuration pointer data structure

    // machine trap setup
    Mstatus = 0x300,    // machine status register
    Misa = 0x301,       // machine ISA and extensions
    Medeleg = 0x302,    // machine exception delegation register
    Mideleg = 0x303,    // machine interrupt delegation register
    Mie = 0x304,        // machine interrupt-enable register
    Mtvec = 0x305,      // machine trap-handler base address
    Mcounteren = 0x306, // machine counter enable register

    // machine trap handling
    Mscratch = 0x340, // machine scratch register
    Mepc = 0x341,     // machine exception program counter
    Mcause = 0x342,   // machine trap cause
    Mtval = 0x343,    // machine trap value
    Mip = 0x344,      // machine interrupt pending
    Mtinst = 0x34A,   // machine trap instruction
    Mtval2 = 0x34B,   // machine second trap value

    // machine indirect
    Miselect = 0x350, // machine indirect register select
    Mireg = 0x351,    // machine indirect register alias
    Mireg2 = 0x352,   // machine indirect register alias 2
    Mireg3 = 0x353,   // machine indirect register alias 3
    Mireg4 = 0x355,   // machine indirect register alias 4
    Mireg5 = 0x356,   // machine indirect register alias 5
    Mireg6 = 0x357,   // machine indirect register alias 6

    // machine configuration
    Menvcfg = 0x30A, // machine environment configuration
    Mseccfg = 0x747, // machine security configuration

    // machine state enable registers
    Mstateen0 = 0x30C, // machine state enable register 0
    Mstateen1 = 0x30D, // machine state enable register 1
    Mstateen2 = 0x30E, // machine state enable register 2
    Mstateen3 = 0x30F, // machine state enable register 3

    // machine non maskable interrupt handling
    Mnscratch = 0x740, // machine non-maskable interrupt scratch register
    Mnepc = 0x741,     // machine non-maskable interrupt exception program counter
    Mncause = 0x742,   // machine non-maskable interrupt cause
    Mnstatus = 0x744,  // machine non-maskable interrupt status register
}

impl Display for GeneralRegisterName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{:?}", self).to_lowercase())
    }
}

impl Display for ControlRegisterName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{:?}", self).to_lowercase())
    }
}

impl TryFrom<u8> for GeneralRegisterName {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        for name in GeneralRegisterName::iter() {
            if name as u8 == value {
                return Ok(name);
            }
        }
        Err(())
    }
}

impl TryFrom<u16> for ControlRegisterName {
    type Error = ();

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        if value > 4096 {
            return Err(());
        }
        for name in ControlRegisterName::iter() {
            if name as u16 == value {
                return Ok(name);
            }
        }
        Err(())
    }
}
