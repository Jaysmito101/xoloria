use std::fmt::Display;

use strum::EnumIter;

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
        match value {
            0 => Ok(Self::Zero),
            1 => Ok(Self::Ra),
            2 => Ok(Self::Sp),
            3 => Ok(Self::Gp),
            4 => Ok(Self::Tp),
            5 => Ok(Self::T0),
            6 => Ok(Self::T1),
            7 => Ok(Self::T2),
            8 => Ok(Self::S0),
            9 => Ok(Self::S1),
            10 => Ok(Self::A0),
            11 => Ok(Self::A1),
            12 => Ok(Self::A2),
            13 => Ok(Self::A3),
            14 => Ok(Self::A4),
            15 => Ok(Self::A5),
            16 => Ok(Self::A6),
            17 => Ok(Self::A7),
            18 => Ok(Self::S2),
            19 => Ok(Self::S3),
            20 => Ok(Self::S4),
            21 => Ok(Self::S5),
            22 => Ok(Self::S6),
            23 => Ok(Self::S7),
            24 => Ok(Self::S8),
            25 => Ok(Self::S9),
            26 => Ok(Self::S10),
            27 => Ok(Self::S11),
            28 => Ok(Self::T3),
            29 => Ok(Self::T4),
            30 => Ok(Self::T5),
            31 => Ok(Self::T6),
            _ => Err(()),
        }
    }
}

impl TryFrom<u16> for ControlRegisterName {
    type Error = ();

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        if value > 4096 {
            return Err(());
        }
        match value {
            0xF14 => Ok(Self::Mhartid),
            0x300 => Ok(Self::Mstatus),
            0x301 => Ok(Self::Misa),
            0x302 => Ok(Self::Medeleg),
            0x303 => Ok(Self::Mideleg),
            0x304 => Ok(Self::Mie),
            0x305 => Ok(Self::Mtvec),
            0x340 => Ok(Self::Mscratch),
            0x341 => Ok(Self::Mepc),
            0x342 => Ok(Self::Mcause),
            0x343 => Ok(Self::Mtval),
            0x344 => Ok(Self::Mip),
            0x105 => Ok(Self::Stvec),
            0x140 => Ok(Self::Sscratch),
            0x141 => Ok(Self::Sepc),
            0x142 => Ok(Self::Scause),
            0x143 => Ok(Self::Stval),
            0x180 => Ok(Self::Satp),
            _ => Err(()),
        }
    }
}
