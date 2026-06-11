use crate::registers::GeneralRegisterName;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RType {
    rd: GeneralRegisterName,
    funct3: u8,
    rs1: GeneralRegisterName,
    rs2: GeneralRegisterName,
    funct7: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct IType {
    rd: GeneralRegisterName,
    funct3: u8,
    rs1: GeneralRegisterName,
    imm: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SType {
    funct3: u8,
    rs1: GeneralRegisterName,
    rs2: GeneralRegisterName,
    imm: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BType {
    funct3: u8,
    rs1: GeneralRegisterName,
    rs2: GeneralRegisterName,
    imm: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UType {
    rd: GeneralRegisterName,
    imm: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct JType {
    rd: GeneralRegisterName,
    imm: i32,
}
