use macros::register;

register! {
    pub register Mie {
        pub ssie: bool = bit(1),
        pub msie: bool = bit(3),
        pub stie: bool = bit(5),
        pub mtie: bool = bit(7),
        pub seie: bool = bit(9),
        pub meie: bool = bit(11),
        pub lcofie: bool = bit(13),
    }
}
