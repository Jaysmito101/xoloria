pub trait FromBits {
    fn from_bits(bits: u64) -> Self;
}

pub trait IntoBits {
    fn into_bits(self) -> u64;
}

impl FromBits for bool {
    #[inline(always)]
    fn from_bits(bits: u64) -> Self {
        bits != 0
    }
}

impl IntoBits for bool {
    #[inline(always)]
    fn into_bits(self) -> u64 {
        if self { 1 } else { 0 }
    }
}

macro_rules! impl_bits_for_int {
    ($($t:ty),*) => {
        $(
            impl FromBits for $t {
                #[inline(always)]
                fn from_bits(bits: u64) -> Self {
                    bits as $t
                }
            }

            impl IntoBits for $t {
                #[inline(always)]
                fn into_bits(self) -> u64 {
                    self as u64
                }
            }
        )*
    };
}

impl_bits_for_int!(u8, u16, u32, u64);
