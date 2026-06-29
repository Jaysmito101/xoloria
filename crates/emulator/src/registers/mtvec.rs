use macros::RegisterBits;

use crate::{Address, registers::Register};

#[derive(RegisterBits)]
pub struct Mtvec(Register);

pub enum MtvecMode {
    Direct = 0,
    Vectored = 1,
    Reserved = 2,
}

impl Mtvec {
    pub const MODE: u8 = 0;
    pub const BASE: u8 = 2;

    #[inline(always)]
    pub fn mode(&self) -> MtvecMode {
        match self.0 & 0b11 {
            0 => MtvecMode::Direct,
            1 => MtvecMode::Vectored,
            _ => MtvecMode::Reserved,
        }
    }

    #[inline(always)]
    pub fn base(&self) -> Address {
        self.0 & !0b11
    }
}
