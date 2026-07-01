use macros::register;

use crate::Address;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MtvecMode {
    Direct = 0,
    Vectored = 1,
    Reserved = 2,
}

impl crate::registers::FromBits for MtvecMode {
    #[inline(always)]
    fn from_bits(bits: u64) -> Self {
        match bits {
            0 => MtvecMode::Direct,
            1 => MtvecMode::Vectored,
            _ => MtvecMode::Reserved,
        }
    }
}

impl crate::registers::IntoBits for MtvecMode {
    #[inline(always)]
    fn into_bits(self) -> u64 {
        self as u64
    }
}

register! {
    pub register Mtvec {
        pub mode: MtvecMode = range(0..=1),
        pub base: Address = range(2..=63),
    }
}
