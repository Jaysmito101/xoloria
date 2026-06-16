use crate::{
    Bus, BusIO, Hart,
    registers::GeneralRegisterName,
    vm::{VmError, VmOutput, VmResult},
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
    _aq_rel: (bool, bool),
    width: bool,
    bus: &Bus,
    hart: &mut Hart,
) -> VmResult {
    let mask = if width {
        0xFFFFFFFFFFFFFFFF
    } else {
        0xFFFFFFFF
    };
    let value = (hart.registers.x[rs2 as usize] & mask) as i64;
    let address = hart.registers.x[rs1 as usize];
    let old_value = if width {
        bus.rmw::<u64, _>(address, |old| (old as i64).wrapping_add(value) as u64)
    } else {
        bus.rmw::<u32, _>(address, |old| {
            (old as i32).wrapping_add(value as i32) as u32
        })
        .map(|v| v as u64)
    };
    hart.registers.x[rd as usize] = old_value.map_err(VmError::BusError)?;
    Ok(VmOutput::NextInstruction)
}

#[inline]
pub fn execute_amoswap(
    rd: GeneralRegisterName,
    rs1: GeneralRegisterName,
    rs2: GeneralRegisterName,
    _aq_rel: (bool, bool),
    width: bool,
    bus: &Bus,
    hart: &mut Hart,
) -> VmResult {
    let mask = if width {
        0xFFFFFFFFFFFFFFFF
    } else {
        0xFFFFFFFF
    };
    let value = hart.registers.x[rs2 as usize] & mask;
    let address = hart.registers.x[rs1 as usize];
    let old_value = if width {
        bus.rmw::<u64, _>(address, |_| value)
    } else {
        bus.rmw::<u32, _>(address, |_| value as u32)
            .map(|v| v as u64)
    };
    hart.registers.x[rd as usize] = old_value.map_err(VmError::BusError)?;
    Ok(VmOutput::NextInstruction)
}

#[inline]
pub fn execute_amxor(
    rd: GeneralRegisterName,
    rs1: GeneralRegisterName,
    rs2: GeneralRegisterName,
    _aq_rel: (bool, bool),
    width: bool,
    bus: &Bus,
    hart: &mut Hart,
) -> VmResult {
    let mask = if width {
        0xFFFFFFFFFFFFFFFF
    } else {
        0xFFFFFFFF
    };
    let value = hart.registers.x[rs2 as usize] & mask;
    let address = hart.registers.x[rs1 as usize];
    let old_value = if width {
        bus.rmw::<u64, _>(address, |old| old ^ value)
    } else {
        bus.rmw::<u32, _>(address, |old| old ^ (value as u32))
            .map(|v| v as u64)
    };
    hart.registers.x[rd as usize] = old_value.map_err(VmError::BusError)?;
    Ok(VmOutput::NextInstruction)
}

#[inline]
pub fn execute_amoor(
    rd: GeneralRegisterName,
    rs1: GeneralRegisterName,
    rs2: GeneralRegisterName,
    _aq_rel: (bool, bool),
    width: bool,
    bus: &Bus,
    hart: &mut Hart,
) -> VmResult {
    let mask = if width {
        0xFFFFFFFFFFFFFFFF
    } else {
        0xFFFFFFFF
    };
    let value = hart.registers.x[rs2 as usize] & mask;
    let address = hart.registers.x[rs1 as usize];
    let old_value = if width {
        bus.rmw::<u64, _>(address, |old| old | value)
    } else {
        bus.rmw::<u32, _>(address, |old| old | (value as u32))
            .map(|v| v as u64)
    };
    hart.registers.x[rd as usize] = old_value.map_err(VmError::BusError)?;
    Ok(VmOutput::NextInstruction)
}

#[inline]
pub fn execute_amoand(
    rd: GeneralRegisterName,
    rs1: GeneralRegisterName,
    rs2: GeneralRegisterName,
    _aq_rel: (bool, bool),
    width: bool,
    bus: &Bus,
    hart: &mut Hart,
) -> VmResult {
    let mask = if width {
        0xFFFFFFFFFFFFFFFF
    } else {
        0xFFFFFFFF
    };
    let value = hart.registers.x[rs2 as usize] & mask;
    let address = hart.registers.x[rs1 as usize];
    let old_value = if width {
        bus.rmw::<u64, _>(address, |old| old & value)
    } else {
        bus.rmw::<u32, _>(address, |old| old & (value as u32))
            .map(|v| v as u64)
    };
    hart.registers.x[rd as usize] = old_value.map_err(VmError::BusError)?;
    Ok(VmOutput::NextInstruction)
}

#[inline]
pub fn execute_amomin(
    rd: GeneralRegisterName,
    rs1: GeneralRegisterName,
    rs2: GeneralRegisterName,
    _aq_rel: (bool, bool),
    width: bool,
    bus: &Bus,
    hart: &mut Hart,
) -> VmResult {
    let mask = if width {
        0xFFFFFFFFFFFFFFFF
    } else {
        0xFFFFFFFF
    };
    let value = (hart.registers.x[rs2 as usize] & mask) as i64;
    let address = hart.registers.x[rs1 as usize];
    let old_value = if width {
        bus.rmw::<u64, _>(address, |old| (old as i64).min(value) as u64)
    } else {
        bus.rmw::<u32, _>(address, |old| (old as i32).min(value as i32) as u32)
            .map(|v| v as u64)
    };
    hart.registers.x[rd as usize] = old_value.map_err(VmError::BusError)?;
    Ok(VmOutput::NextInstruction)
}

#[inline]
pub fn execute_amomax(
    rd: GeneralRegisterName,
    rs1: GeneralRegisterName,
    rs2: GeneralRegisterName,
    _aq_rel: (bool, bool),
    width: bool,
    bus: &Bus,
    hart: &mut Hart,
) -> VmResult {
    let mask = if width {
        0xFFFFFFFFFFFFFFFF
    } else {
        0xFFFFFFFF
    };
    let value = (hart.registers.x[rs2 as usize] & mask) as i64;
    let address = hart.registers.x[rs1 as usize];
    let old_value = if width {
        bus.rmw::<u64, _>(address, |old| (old as i64).max(value) as u64)
    } else {
        bus.rmw::<u32, _>(address, |old| (old as i32).max(value as i32) as u32)
            .map(|v| v as u64)
    };
    hart.registers.x[rd as usize] = old_value.map_err(VmError::BusError)?;
    Ok(VmOutput::NextInstruction)
}

#[inline]
pub fn execute_amominu(
    rd: GeneralRegisterName,
    rs1: GeneralRegisterName,
    rs2: GeneralRegisterName,
    _aq_rel: (bool, bool),
    width: bool,
    bus: &Bus,
    hart: &mut Hart,
) -> VmResult {
    let mask = if width {
        0xFFFFFFFFFFFFFFFF
    } else {
        0xFFFFFFFF
    };
    let value = hart.registers.x[rs2 as usize] & mask;
    let address = hart.registers.x[rs1 as usize];
    let old_value = if width {
        bus.rmw::<u64, _>(address, |old| old.min(value))
    } else {
        bus.rmw::<u32, _>(address, |old| old.min(value as u32))
            .map(|v| v as u64)
    };
    hart.registers.x[rd as usize] = old_value.map_err(VmError::BusError)?;
    Ok(VmOutput::NextInstruction)
}

#[inline]
pub fn execute_amomaxu(
    rd: GeneralRegisterName,
    rs1: GeneralRegisterName,
    rs2: GeneralRegisterName,
    _aq_rel: (bool, bool),
    width: bool,
    bus: &Bus,
    hart: &mut Hart,
) -> VmResult {
    let mask = if width {
        0xFFFFFFFFFFFFFFFF
    } else {
        0xFFFFFFFF
    };
    let value = hart.registers.x[rs2 as usize] & mask;
    let address = hart.registers.x[rs1 as usize];
    let old_value = if width {
        bus.rmw::<u64, _>(address, |old| old.max(value))
    } else {
        bus.rmw::<u32, _>(address, |old| old.max(value as u32))
            .map(|v| v as u64)
    };
    hart.registers.x[rd as usize] = old_value.map_err(VmError::BusError)?;
    Ok(VmOutput::NextInstruction)
}
