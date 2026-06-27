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
