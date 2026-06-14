use crate::{
    Hart,
    registers::GeneralRegisterName,
    vm::{VmOutput, VmResult},
};

#[inline]
pub fn execute_add(
    rd: GeneralRegisterName,
    rs1: GeneralRegisterName,
    rs2: GeneralRegisterName,
    hart: &mut Hart,
) -> VmResult {
    let result =
        (hart.registers.x[rs1 as usize] as i64).wrapping_add(hart.registers.x[rs2 as usize] as i64);
    hart.registers.x[rd as usize] = result as u64;
    Ok(VmOutput::NextInstruction)
}

#[inline]
pub fn execute_sub(
    rd: GeneralRegisterName,
    rs1: GeneralRegisterName,
    rs2: GeneralRegisterName,
    hart: &mut Hart,
) -> VmResult {
    let result =
        (hart.registers.x[rs1 as usize] as i64).wrapping_sub(hart.registers.x[rs2 as usize] as i64);
    hart.registers.x[rd as usize] = result as u64;
    Ok(VmOutput::NextInstruction)
}

#[inline]
pub fn execute_slt(
    rd: GeneralRegisterName,
    rs1: GeneralRegisterName,
    rs2: GeneralRegisterName,
    hart: &mut Hart,
) -> VmResult {
    let value1 = hart.registers.x[rs1 as usize] as i64;
    let value2 = hart.registers.x[rs2 as usize] as i64;
    hart.registers.x[rd as usize] = if value1 < value2 { 1 } else { 0 };
    Ok(VmOutput::NextInstruction)
}

#[inline]
pub fn execute_sltu(
    rd: GeneralRegisterName,
    rs1: GeneralRegisterName,
    rs2: GeneralRegisterName,
    hart: &mut Hart,
) -> VmResult {
    let value1 = hart.registers.x[rs1 as usize];
    let value2 = hart.registers.x[rs2 as usize];
    hart.registers.x[rd as usize] = if value1 < value2 { 1 } else { 0 };
    Ok(VmOutput::NextInstruction)
}

#[inline]
pub fn execute_sll(
    rd: GeneralRegisterName,
    rs1: GeneralRegisterName,
    rs2: GeneralRegisterName,
    hart: &mut Hart,
) -> VmResult {
    let shift_amount = (hart.registers.x[rs2 as usize] & 0x3F) as u8;
    hart.registers.x[rd as usize] = hart.registers.x[rs1 as usize] << shift_amount;
    Ok(VmOutput::NextInstruction)
}

#[inline]
pub fn execute_srl(
    rd: GeneralRegisterName,
    rs1: GeneralRegisterName,
    rs2: GeneralRegisterName,
    hart: &mut Hart,
) -> VmResult {
    let shift_amount = (hart.registers.x[rs2 as usize] & 0x3F) as u8;
    hart.registers.x[rd as usize] = hart.registers.x[rs1 as usize] >> shift_amount;
    Ok(VmOutput::NextInstruction)
}

#[inline]
pub fn execute_sra(
    rd: GeneralRegisterName,
    rs1: GeneralRegisterName,
    rs2: GeneralRegisterName,
    hart: &mut Hart,
) -> VmResult {
    let shift_amount = (hart.registers.x[rs2 as usize] & 0x3F) as u8;
    let value = hart.registers.x[rs1 as usize] as i64;
    hart.registers.x[rd as usize] = (value >> shift_amount) as u64;
    Ok(VmOutput::NextInstruction)
}

#[inline]
pub fn execute_xor(
    rd: GeneralRegisterName,
    rs1: GeneralRegisterName,
    rs2: GeneralRegisterName,
    hart: &mut Hart,
) -> VmResult {
    hart.registers.x[rd as usize] = hart.registers.x[rs1 as usize] ^ hart.registers.x[rs2 as usize];
    Ok(VmOutput::NextInstruction)
}

#[inline]
pub fn execute_or(
    rd: GeneralRegisterName,
    rs1: GeneralRegisterName,
    rs2: GeneralRegisterName,
    hart: &mut Hart,
) -> VmResult {
    hart.registers.x[rd as usize] = hart.registers.x[rs1 as usize] | hart.registers.x[rs2 as usize];
    Ok(VmOutput::NextInstruction)
}

#[inline]
pub fn execute_and(
    rd: GeneralRegisterName,
    rs1: GeneralRegisterName,
    rs2: GeneralRegisterName,
    hart: &mut Hart,
) -> VmResult {
    hart.registers.x[rd as usize] = hart.registers.x[rs1 as usize] & hart.registers.x[rs2 as usize];
    Ok(VmOutput::NextInstruction)
}

#[inline]
pub fn execute_addw(
    rd: GeneralRegisterName,
    rs1: GeneralRegisterName,
    rs2: GeneralRegisterName,
    hart: &mut Hart,
) -> VmResult {
    let result = (hart.registers.x[rs1 as usize] as i64 as i32)
        .wrapping_add(hart.registers.x[rs2 as usize] as i64 as i32);
    hart.registers.x[rd as usize] = result as i64 as u64;
    Ok(VmOutput::NextInstruction)
}

#[inline]
pub fn execute_subw(
    rd: GeneralRegisterName,
    rs1: GeneralRegisterName,
    rs2: GeneralRegisterName,
    hart: &mut Hart,
) -> VmResult {
    let result = (hart.registers.x[rs1 as usize] as i64 as i32)
        .wrapping_sub(hart.registers.x[rs2 as usize] as i64 as i32);
    hart.registers.x[rd as usize] = result as i64 as u64;
    Ok(VmOutput::NextInstruction)
}

#[inline]
pub fn execute_sllw(
    rd: GeneralRegisterName,
    rs1: GeneralRegisterName,
    rs2: GeneralRegisterName,
    hart: &mut Hart,
) -> VmResult {
    let shift_amount = (hart.registers.x[rs2 as usize] & 0x1F) as u8;
    hart.registers.x[rd as usize] =
        (hart.registers.x[rs1 as usize] as i64 as i32 as u64) << shift_amount;
    Ok(VmOutput::NextInstruction)
}

#[inline]
pub fn execute_srlw(
    rd: GeneralRegisterName,
    rs1: GeneralRegisterName,
    rs2: GeneralRegisterName,
    hart: &mut Hart,
) -> VmResult {
    let shift_amount = (hart.registers.x[rs2 as usize] & 0x1F) as u8;
    hart.registers.x[rd as usize] =
        (hart.registers.x[rs1 as usize] as i64 as i32 as u64) >> shift_amount;
    Ok(VmOutput::NextInstruction)
}

#[inline]
pub fn execute_sraw(
    rd: GeneralRegisterName,
    rs1: GeneralRegisterName,
    rs2: GeneralRegisterName,
    hart: &mut Hart,
) -> VmResult {
    let shift_amount = (hart.registers.x[rs2 as usize] & 0x1F) as u8;
    let value = (hart.registers.x[rs1 as usize] as i64 as i32) as i64;
    hart.registers.x[rd as usize] = (value >> shift_amount) as u64;
    Ok(VmOutput::NextInstruction)
}

#[inline]
pub fn execute_mul(
    rd: GeneralRegisterName,
    rs1: GeneralRegisterName,
    rs2: GeneralRegisterName,
    hart: &mut Hart,
) -> VmResult {
    let result =
        (hart.registers.x[rs1 as usize] as i64).wrapping_mul(hart.registers.x[rs2 as usize] as i64);
    hart.registers.x[rd as usize] = result as u64;
    Ok(VmOutput::NextInstruction)
}

#[inline]
pub fn execute_mulw(
    rd: GeneralRegisterName,
    rs1: GeneralRegisterName,
    rs2: GeneralRegisterName,
    hart: &mut Hart,
) -> VmResult {
    let result = (hart.registers.x[rs1 as usize] as i64 as i32)
        .wrapping_mul(hart.registers.x[rs2 as usize] as i64 as i32);
    hart.registers.x[rd as usize] = result as i64 as u64;
    Ok(VmOutput::NextInstruction)
}

#[inline]
pub fn execute_mulh(
    rd: GeneralRegisterName,
    rs1: GeneralRegisterName,
    rs2: GeneralRegisterName,
    hart: &mut Hart,
) -> VmResult {
    let value1 = hart.registers.x[rs1 as usize] as i64;
    let value2 = hart.registers.x[rs2 as usize] as i64;
    let result = ((value1 as i128 * value2 as i128) >> 64) as i64;
    hart.registers.x[rd as usize] = result as u64;
    Ok(VmOutput::NextInstruction)
}

#[inline]
pub fn execute_mulhu(
    rd: GeneralRegisterName,
    rs1: GeneralRegisterName,
    rs2: GeneralRegisterName,
    hart: &mut Hart,
) -> VmResult {
    let value1 = hart.registers.x[rs1 as usize];
    let value2 = hart.registers.x[rs2 as usize];
    let result = ((value1 as u128 * value2 as u128) >> 64) as u64;
    hart.registers.x[rd as usize] = result;
    Ok(VmOutput::NextInstruction)
}

#[inline]
pub fn execute_mulhsu(
    rd: GeneralRegisterName,
    rs1: GeneralRegisterName,
    rs2: GeneralRegisterName,
    hart: &mut Hart,
) -> VmResult {
    let value1 = hart.registers.x[rs1 as usize] as i64;
    let value2 = hart.registers.x[rs2 as usize];
    let result = ((value1 as i128 * value2 as i128) >> 64) as i64;
    hart.registers.x[rd as usize] = result as u64;
    Ok(VmOutput::NextInstruction)
}

#[inline]
pub fn execute_div(
    rd: GeneralRegisterName,
    rs1: GeneralRegisterName,
    rs2: GeneralRegisterName,
    hart: &mut Hart,
) -> VmResult {
    let value1 = hart.registers.x[rs1 as usize] as i64;
    let value2 = hart.registers.x[rs2 as usize] as i64;

    let result = if value2 == 0 {
        -1i64
    } else if value1 == i64::MIN && value2 == -1 {
        i64::MIN
    } else {
        value1.wrapping_div(value2)
    };

    hart.registers.x[rd as usize] = result as u64;
    Ok(VmOutput::NextInstruction)
}

#[inline]
pub fn execute_divu(
    rd: GeneralRegisterName,
    rs1: GeneralRegisterName,
    rs2: GeneralRegisterName,
    hart: &mut Hart,
) -> VmResult {
    let value1 = hart.registers.x[rs1 as usize];
    let value2 = hart.registers.x[rs2 as usize];

    let result = if value2 == 0 {
        u64::MAX
    } else {
        value1.wrapping_div(value2)
    };

    hart.registers.x[rd as usize] = result;
    Ok(VmOutput::NextInstruction)
}

#[inline]
pub fn execute_divw(
    rd: GeneralRegisterName,
    rs1: GeneralRegisterName,
    rs2: GeneralRegisterName,
    hart: &mut Hart,
) -> VmResult {
    let value1 = hart.registers.x[rs1 as usize] as i64 as i32;
    let value2 = hart.registers.x[rs2 as usize] as i64 as i32;

    let result = if value2 == 0 {
        -1i32
    } else if value1 == i32::MIN && value2 == -1 {
        i32::MIN
    } else {
        value1.wrapping_div(value2)
    };

    hart.registers.x[rd as usize] = result as i64 as u64;
    Ok(VmOutput::NextInstruction)
}

#[inline]
pub fn execute_divuw(
    rd: GeneralRegisterName,
    rs1: GeneralRegisterName,
    rs2: GeneralRegisterName,
    hart: &mut Hart,
) -> VmResult {
    let value1 = hart.registers.x[rs1 as usize] as u32;
    let value2 = hart.registers.x[rs2 as usize] as u32;

    let result = if value2 == 0 {
        u32::MAX
    } else {
        value1.wrapping_div(value2)
    };

    hart.registers.x[rd as usize] = result as i32 as i64 as u64;
    Ok(VmOutput::NextInstruction)
}

#[inline]
pub fn execute_rem(
    rd: GeneralRegisterName,
    rs1: GeneralRegisterName,
    rs2: GeneralRegisterName,
    hart: &mut Hart,
) -> VmResult {
    let value1 = hart.registers.x[rs1 as usize] as i64;
    let value2 = hart.registers.x[rs2 as usize] as i64;

    let result = if value2 == 0 {
        value1
    } else if value1 == i64::MIN && value2 == -1 {
        0
    } else {
        value1.wrapping_rem(value2)
    };

    hart.registers.x[rd as usize] = result as u64;
    Ok(VmOutput::NextInstruction)
}

#[inline]
pub fn execute_remu(
    rd: GeneralRegisterName,
    rs1: GeneralRegisterName,
    rs2: GeneralRegisterName,
    hart: &mut Hart,
) -> VmResult {
    let value1 = hart.registers.x[rs1 as usize];
    let value2 = hart.registers.x[rs2 as usize];

    let result = if value2 == 0 {
        value1
    } else {
        value1.wrapping_rem(value2)
    };

    hart.registers.x[rd as usize] = result;
    Ok(VmOutput::NextInstruction)
}

#[inline]
pub fn execute_remw(
    rd: GeneralRegisterName,
    rs1: GeneralRegisterName,
    rs2: GeneralRegisterName,
    hart: &mut Hart,
) -> VmResult {
    let value1 = hart.registers.x[rs1 as usize] as i64 as i32;
    let value2 = hart.registers.x[rs2 as usize] as i64 as i32;

    let result = if value2 == 0 {
        value1
    } else if value1 == i32::MIN && value2 == -1 {
        0
    } else {
        value1.wrapping_rem(value2)
    };

    hart.registers.x[rd as usize] = result as i64 as u64;
    Ok(VmOutput::NextInstruction)
}

#[inline]
pub fn execute_remuw(
    rd: GeneralRegisterName,
    rs1: GeneralRegisterName,
    rs2: GeneralRegisterName,
    hart: &mut Hart,
) -> VmResult {
    let value1 = hart.registers.x[rs1 as usize] as u32;
    let value2 = hart.registers.x[rs2 as usize] as u32;

    let result = if value2 == 0 {
        value1
    } else {
        value1.wrapping_rem(value2)
    };

    hart.registers.x[rd as usize] = result as i32 as i64 as u64;
    Ok(VmOutput::NextInstruction)
}
