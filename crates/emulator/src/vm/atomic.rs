use crate::{
    Bus, BusIO, Hart,
    registers::GeneralRegisterName,
    vm::{VmError, VmResult},
};

#[inline]
pub fn execute_lr(
    rd: GeneralRegisterName,
    rs1: GeneralRegisterName,
    aq_rel: (bool, bool),
    width: bool,
    bus: &Bus,
    hart: &mut Hart,
) -> VmResult {
    unimplemented!();
}

#[inline]
pub fn execute_sc(
    rd: GeneralRegisterName,
    rs1: GeneralRegisterName,
    rs2: GeneralRegisterName,
    aq_rel: (bool, bool),
    width: bool,
    bus: &Bus,
    hart: &mut Hart,
) -> VmResult {
    unimplemented!();
}

#[inline]
pub fn execute_amoadd(
    rd: GeneralRegisterName,
    rs1: GeneralRegisterName,
    rs2: GeneralRegisterName,
    aq_rel: (bool, bool),
    width: bool,
    bus: &Bus,
    hart: &mut Hart,
) -> VmResult {
    unimplemented!();
}

#[inline]
pub fn execute_amoswap(
    rd: GeneralRegisterName,
    rs1: GeneralRegisterName,
    rs2: GeneralRegisterName,
    aq_rel: (bool, bool),
    width: bool,
    bus: &Bus,
    hart: &mut Hart,
) -> VmResult {
    unimplemented!();
}

#[inline]
pub fn execute_amxor(
    rd: GeneralRegisterName,
    rs1: GeneralRegisterName,
    rs2: GeneralRegisterName,
    aq_rel: (bool, bool),
    width: bool,
    bus: &Bus,
    hart: &mut Hart,
) -> VmResult {
    unimplemented!();
}

#[inline]
pub fn execute_amoor(
    rd: GeneralRegisterName,
    rs1: GeneralRegisterName,
    rs2: GeneralRegisterName,
    aq_rel: (bool, bool),
    width: bool,
    bus: &Bus,
    hart: &mut Hart,
) -> VmResult {
    unimplemented!();
}

#[inline]
pub fn execute_amoand(
    rd: GeneralRegisterName,
    rs1: GeneralRegisterName,
    rs2: GeneralRegisterName,
    aq_rel: (bool, bool),
    width: bool,
    bus: &Bus,
    hart: &mut Hart,
) -> VmResult {
    unimplemented!();
}

#[inline]
pub fn execute_amomin(
    rd: GeneralRegisterName,
    rs1: GeneralRegisterName,
    rs2: GeneralRegisterName,
    aq_rel: (bool, bool),
    width: bool,
    bus: &Bus,
    hart: &mut Hart,
) -> VmResult {
    unimplemented!();
}

#[inline]
pub fn execute_amomax(
    rd: GeneralRegisterName,
    rs1: GeneralRegisterName,
    rs2: GeneralRegisterName,
    aq_rel: (bool, bool),
    width: bool,
    bus: &Bus,
    hart: &mut Hart,
) -> VmResult {
    unimplemented!();
}

#[inline]
pub fn execute_amominu(
    rd: GeneralRegisterName,
    rs1: GeneralRegisterName,
    rs2: GeneralRegisterName,
    aq_rel: (bool, bool),
    width: bool,
    bus: &Bus,
    hart: &mut Hart,
) -> VmResult {
    unimplemented!();
}

#[inline]
pub fn execute_amomaxu(
    rd: GeneralRegisterName,
    rs1: GeneralRegisterName,
    rs2: GeneralRegisterName,
    aq_rel: (bool, bool),
    width: bool,
    bus: &Bus,
    hart: &mut Hart,
) -> VmResult {
    unimplemented!();
}
