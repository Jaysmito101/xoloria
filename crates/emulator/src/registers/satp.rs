use macros::{register, RegisterField};

#[derive(Debug, Clone, Copy, PartialEq, Eq, RegisterField)]
pub enum SatpMode {
    Bare = 0,
    Sv39 = 8,
    Sv48 = 9,
    Sv57 = 10,
    Sv64 = 11,
}

register! {
    pub register Satp {
        pub mode: SatpMode = range(60..=63),
        pub asid: u16 = range(44..=59),
        pub ppn: u64 = range(0..=43),
    }
}
