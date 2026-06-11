#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OpcodeName {
    // RV32I Base
    Load = 3,
    OpImm = 19,
    Auipc = 23,
    Store = 35,
    OpReg = 51,
    Lui = 55,
    Branch = 99,
    Jalr = 103,
    Jal = 111,

    // RV64I Extensions
    OpImm64 = 27,
    OpReg64 = 59,

    // A Extension
    Atomic = 47,

    // System, Zicsr, Zifencei Extensions
    Fence = 15,
    System = 115,
}
