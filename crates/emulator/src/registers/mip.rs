use macros::register;

register! {
    pub register Mip {
        pub ssip: bool = bit(1),
        pub msip: bool = bit(3),
        pub stip: bool = bit(5),
        pub mtip: bool = bit(7),
        pub seip: bool = bit(9),
        pub meip: bool = bit(11),
        pub lcofip: bool = bit(13),
    }
}
