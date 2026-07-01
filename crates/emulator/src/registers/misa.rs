use macros::{RegisterField, register};

#[derive(Debug, Clone, Copy, PartialEq, Eq, RegisterField)]
pub enum Xlen {
    Rv32 = 1,
    Rv64 = 2,
    Rv128 = 3,
}

register! {
    pub register Misa {
        /// Atomic extension
        pub a: bool = bit(0),

        /// B extension
        pub b: bool = bit(1),

        /// Compressed extension
        pub c: bool = bit(2),

        /// Double-precision floating-point extension
        pub d: bool = bit(3),

        /// RV32E/64E base ISA
        pub e: bool = bit(4),

        /// Single-precision floating-point extension
        pub f: bool = bit(5),

        /// Hypervisor extension
        pub h: bool = bit(7),

        /// RV32I/64I base ISA
        pub i: bool = bit(8),

        /// Integer Multiply/Divide extension
        pub m: bool = bit(12),

        /// Tentatively reserved for User-Level Interrupts extension
        pub n: bool = bit(13),

        /// Tentatively reserved for Packed-SIMD extension
        pub p: bool = bit(15),

        /// Quad-precision floating-point extension
        pub q: bool = bit(16),

        /// Supervisor mode implemented
        pub s: bool = bit(18),

        /// User mode implemented
        pub u: bool = bit(20),

        /// Vector extension
        pub v: bool = bit(21),

        /// Non-standard extensions present
        pub x: bool = bit(23),

        pub mxlen: u8 = range(26..=61),
        pub xlen: Xlen = range(62..=63),
    }
}
