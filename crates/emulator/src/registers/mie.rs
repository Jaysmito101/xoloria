use macros::RegisterBits;

use crate::registers::Register;

#[derive(RegisterBits)]
pub struct Mie(Register);

impl Mie {
    pub const SSIE: u8 = 1;
    pub const MSIE: u8 = 3;
    pub const STIE: u8 = 5;
    pub const MTIE: u8 = 7;
    pub const SEIE: u8 = 9;
    pub const MEIE: u8 = 11;
    pub const LCOFIE: u8 = 13;

    #[inline(always)]
    pub fn ssie(&self) -> bool {
        self.bit(Self::SSIE)
    }

    #[inline(always)]
    pub fn msie(&self) -> bool {
        self.bit(Self::MSIE)
    }

    #[inline(always)]
    pub fn stie(&self) -> bool {
        self.bit(Self::STIE)
    }

    #[inline(always)]
    pub fn mtie(&self) -> bool {
        self.bit(Self::MTIE)
    }

    #[inline(always)]
    pub fn seie(&self) -> bool {
        self.bit(Self::SEIE)
    }

    #[inline(always)]
    pub fn meie(&self) -> bool {
        self.bit(Self::MEIE)
    }

    #[inline(always)]
    pub fn lcofie(&self) -> bool {
        self.bit(Self::LCOFIE)
    }
}
