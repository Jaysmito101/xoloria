use macros::RegisterBits;

use super::Register;

#[derive(RegisterBits, Default)]
pub struct Misa(Register);

pub enum ISAExtensions {
    A = 0,  // Atomic extension
    B = 1,  // B extension
    C = 2,  // Compressed extension
    D = 3,  // Double-precision floating-point extension
    E = 4,  // RV32E/64E base ISA
    F = 5,  // Single-precision floating-point extension
    H = 7,  // Hypervisor extension
    I = 8,  // RV32I/64I base ISA
    M = 12, // Integer Multiply/Divide extension
    N = 13, // Tentatively reserved for User-Level Interrupts extension
    P = 15, // Tentatively reserved for Packed-SIMD extension
    Q = 16, // Quad-precision floating-point extension
    S = 18, // Supervisor mode implemented
    U = 20, // User mode implemented
    V = 21, // Vector extension
    X = 23, // Non-standard extensions present
}

impl Misa {
    pub fn with_xlen(mut self, xlen: u8) -> Self {
        self.0 &= !(0b11 << 30);
        match xlen {
            32 => self.0 |= 1 << 30,
            64 => self.0 |= 2 << 30,
            128 => self.0 |= 3 << 30,
            _ => panic!("Invalid XLEN value: {}", xlen),
        }
        self
    }

    pub fn with_extension(mut self, ext: ISAExtensions) -> Self {
        self.bitset(ext as u8);
        self
    }
}
