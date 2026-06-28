use macros::RegisterBits;

use crate::registers::Register;

#[derive(RegisterBits)]
pub struct Mip(Register);

impl Mip {
    pub const SSIP: u8 = 1;
    pub const MSIP: u8 = 3;
    pub const STIP: u8 = 5;
    pub const MTIP: u8 = 7;
    pub const SEIP: u8 = 9;
    pub const MEIP: u8 = 11;
    pub const LCOFIP: u8 = 13;

    #[inline(always)]
    pub fn ssip(&self) -> bool {
        self.bit(Self::SSIP)
    }

    #[inline(always)]
    pub fn msip(&self) -> bool {
        self.bit(Self::MSIP)
    }

    #[inline(always)]
    pub fn stip(&self) -> bool {
        self.bit(Self::STIP)
    }

    #[inline(always)]
    pub fn mtip(&self) -> bool {
        self.bit(Self::MTIP)
    }

    #[inline(always)]
    pub fn seip(&self) -> bool {
        self.bit(Self::SEIP)
    }

    #[inline(always)]
    pub fn meip(&self) -> bool {
        self.bit(Self::MEIP)
    }

    #[inline(always)]
    pub fn lcofip(&self) -> bool {
        self.bit(Self::LCOFIP)
    }
}
