use macros::RegisterBits;

use crate::registers::Register;

#[derive(RegisterBits)]
pub struct Satp(Register);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SatpMode {
    Bare = 0,
    Sv39 = 8,
    Sv48 = 9,
    Sv57 = 10,
    Sv64 = 11,
}

impl Satp {
    pub const MODE: u8 = 60;
    pub const ASID: u8 = 44;

    #[inline(always)]
    pub fn mode(&self) -> SatpMode {
        match (self.0 >> Satp::MODE) & 0xf {
            0 => SatpMode::Bare,
            8 => SatpMode::Sv39,
            9 => SatpMode::Sv48,
            10 => SatpMode::Sv57,
            11 => SatpMode::Sv64,
            _ => panic!("Invalid SATP mode"),
        }
    }

    #[inline(always)]
    pub fn asid(&self) -> u16 {
        ((self.0 >> Satp::ASID) & 0xffff) as u16
    }

    #[inline(always)]
    pub fn ppn(&self) -> u64 {
        self.0 & 0x0000_ffff_ffff_ffff
    }
}
