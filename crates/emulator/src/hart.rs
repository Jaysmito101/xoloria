use std::fmt::Display;

use strum::IntoEnumIterator;

use crate::{
    Bus, BusIO, Result,
    instructions::Instruction,
    registers::{ControlRegisterName, GeneralRegisterName, ISAExtensions, Misa, Register},
    vm::{self, VmError, VmOutput},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PrivilageMode {
    Machine = 0,
    Supervisor = 1,
    User = 2,
}

impl std::fmt::Display for PrivilageMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Machine => write!(f, "M"),
            Self::Supervisor => write!(f, "S"),
            Self::User => write!(f, "U"),
        }
    }
}

#[derive(Debug)]
pub struct ControlStatusRegisters {
    regs: [Register; 4096],
}

impl ControlStatusRegisters {
    pub fn new() -> Self {
        Self { regs: [0; 4096] }
    }

    pub fn with(mut self, name: ControlRegisterName, value: Register) -> Self {
        self.regs[name as usize] = value;
        self
    }

    pub fn get(&self, name: ControlRegisterName, privilage: PrivilageMode) -> Register {
        self.regs[name as usize]
    }
}

#[derive(Debug)]
pub struct HartRegisters {
    pub(crate) pc: Register,
    pub(crate) x: [Register; 32],
    pub(crate) csr: ControlStatusRegisters,

    pub(crate) load_reservation_valid: bool,
    pub(crate) load_reservation_address: Register,
    // pub(crate) mhartid: Register, // hardware thread id

    // pub(crate) misa: Register, // machine ISA and extensions

    // pub(crate) mstatus: Register, // machine status (interrupt enables, previous privilage mode)
    // pub(crate) medeleg: Register, // machine exception delegation
    // pub(crate) mideleg: Register, // machine interrupt deligation
    // pub(crate) mie: Register,     // machine interrupt enable mask
    // pub(crate) mtvec: Register,   // machine trap vector base address
    // pub(crate) mscratch: Register, // machine scratch register
    // pub(crate) mepc: Register,    // maching exception program counter
    // pub(crate) mcause: Register,  // machine trap cause
    // pub(crate) mtval: Register,   // machine bad address or instruction
    // pub(crate) mip: Register,     // machine interrupt pending

    // pub(crate) stvec: Register,    // supervisor trap vector base address
    // pub(crate) sscratch: Register, // supervisor scratch register
    // pub(crate) sepc: Register,     // supervisor exception program counter
    // pub(crate) scause: Register,   // supervisor trap cause
    // pub(crate) stval: Register,    // supervisor bad address or instruction
    // pub(crate) satp: Register,     // supervisor address translation and protection
}

impl Display for HartRegisters {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Program Counter: {:#x}", self.pc)?;
        write!(f, "General Registers: {{ ")?;
        for (i, &value) in self.x.iter().enumerate() {
            write!(
                f,
                "{}: {:#x} ({}){}",
                GeneralRegisterName::try_from(i as u8).unwrap_or(GeneralRegisterName::Zero),
                // i,
                value,
                value,
                if i != self.x.len() - 1 { ", " } else { "" }
            )?;
        }

        writeln!(f, "System Registers: {{")?;
        for name in ControlRegisterName::iter() {
            writeln!(
                f,
                "    {}: {:#x} ({})",
                name,
                self.csr.get(name, PrivilageMode::Machine),
                self.csr.get(name, PrivilageMode::Machine)
            )?;
        }
        writeln!(f, "}}")?;

        Ok(())
    }
}

#[derive(Debug)]
pub struct Hart {
    pub(crate) privilage_mode: PrivilageMode,
    pub(crate) registers: HartRegisters,
}

impl HartRegisters {
    pub fn pc(&self) -> Register {
        self.pc
    }
    pub fn x(&self) -> &[Register; 32] {
        &self.x
    }

    pub fn csrs(&self, privilage_mode: PrivilageMode) -> Vec<(ControlRegisterName, Register)> {
        ControlRegisterName::iter()
            .map(|name| (name, self.csr.get(name, privilage_mode)))
            .collect()
    }
}

impl Display for Hart {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Privilage Mode: {:?}", self.privilage_mode)?;
        writeln!(f, "Registers:\n{}", self.registers)?;
        Ok(())
    }
}

impl Hart {
    pub fn registers(&self) -> &HartRegisters {
        &self.registers
    }
    pub fn privilage_mode(&self) -> PrivilageMode {
        self.privilage_mode
    }

    pub fn new(id: u64) -> Result<Self> {
        Ok(Self {
            privilage_mode: PrivilageMode::Machine,

            registers: HartRegisters {
                pc: 0x80000000,
                x: [0; 32],

                load_reservation_valid: false,
                load_reservation_address: 0,

                csr: ControlStatusRegisters::new()
                    .with(ControlRegisterName::Mhartid, id)
                    .with(
                        ControlRegisterName::Misa,
                        Misa::default()
                            .with_xlen(64)
                            .with_extension(ISAExtensions::I)
                            .with_extension(ISAExtensions::M)
                            .with_extension(ISAExtensions::A)
                            .with_extension(ISAExtensions::C)
                            .register(),
                    ),
            },
        })
    }

    pub fn id(&self) -> u64 {
        self.registers
            .csr
            .get(ControlRegisterName::Mhartid, self.privilage_mode)
    }

    pub fn tick(&mut self, bus: &Bus) -> Result<()> {
        let instruction_value: u32 = bus.read(self.registers.pc)?;
        let is_compressed = instruction_value & 0b11 != 0b11;
        let instruction = Instruction::try_from(instruction_value)?;
        // tracing::warn!("[{:#x}] {}", self.registers.pc, instruction);
        self.registers.x[0] = 0; // enforce zero being zero
        let vm_result = match instruction {
            Instruction::Noop => Ok(VmOutput::NextInstruction),
            Instruction::Lui { rd, imm } => vm::load::execute_lui(rd, imm, self),
            Instruction::Auipc { rd, imm } => vm::load::execute_auipc(rd, imm, self),
            Instruction::Jal { rd, imm } => vm::jump::execute_jal(is_compressed, rd, imm, self),
            Instruction::Jalr { rd, rs1, imm } => {
                vm::jump::execute_jalr(is_compressed, rd, rs1, imm, self)
            }
            Instruction::Lb { rd, rs1, imm } => vm::load::execute_lb(rd, rs1, imm, self, bus),
            Instruction::Lh { rd, rs1, imm } => vm::load::execute_lh(rd, rs1, imm, self, bus),
            Instruction::Lw { rd, rs1, imm } => vm::load::execute_lw(rd, rs1, imm, self, bus),
            Instruction::Lbu { rd, rs1, imm } => vm::load::execute_lbu(rd, rs1, imm, self, bus),
            Instruction::Lhu { rd, rs1, imm } => vm::load::execute_lhu(rd, rs1, imm, self, bus),
            Instruction::Ld { rd, rs1, imm } => vm::load::execute_ld(rd, rs1, imm, self, bus),
            Instruction::Lwu { rd, rs1, imm } => vm::load::execute_lwu(rd, rs1, imm, self, bus),
            Instruction::Sb { rs1, rs2, imm } => vm::store::execute_sb(rs1, rs2, imm, self, bus),
            Instruction::Sh { rs1, rs2, imm } => vm::store::execute_sh(rs1, rs2, imm, self, bus),
            Instruction::Sw { rs1, rs2, imm } => vm::store::execute_sw(rs1, rs2, imm, self, bus),
            Instruction::Sd { rs1, rs2, imm } => vm::store::execute_sd(rs1, rs2, imm, self, bus),
            Instruction::Addi { rd, rs1, imm } => vm::opimm::execute_addi(rd, rs1, imm, self),
            Instruction::Slti { rd, rs1, imm } => vm::opimm::execute_slti(rd, rs1, imm, self),
            Instruction::Sltiu { rd, rs1, imm } => vm::opimm::execute_sltiu(rd, rs1, imm, self),
            Instruction::Slli { rd, rs1, imm } => vm::opimm::execute_slli(rd, rs1, imm, self),
            Instruction::Srli { rd, rs1, imm } => vm::opimm::execute_srli(rd, rs1, imm, self),
            Instruction::Srai { rd, rs1, imm } => vm::opimm::execute_srai(rd, rs1, imm, self),
            Instruction::Xori { rd, rs1, imm } => vm::opimm::execute_xori(rd, rs1, imm, self),
            Instruction::Ori { rd, rs1, imm } => vm::opimm::execute_ori(rd, rs1, imm, self),
            Instruction::Andi { rd, rs1, imm } => vm::opimm::execute_andi(rd, rs1, imm, self),
            Instruction::Addiw { rd, rs1, imm } => vm::opimm::execute_addiw(rd, rs1, imm, self),
            Instruction::Slliw { rd, rs1, imm } => vm::opimm::execute_slliw(rd, rs1, imm, self),
            Instruction::Srliw { rd, rs1, imm } => vm::opimm::execute_srliw(rd, rs1, imm, self),
            Instruction::Sraiw { rd, rs1, imm } => vm::opimm::execute_sraiw(rd, rs1, imm, self),
            Instruction::Beq { rs1, rs2, imm } => vm::branch::execute_beq(rs1, rs2, imm, self),
            Instruction::Bne { rs1, rs2, imm } => vm::branch::execute_bne(rs1, rs2, imm, self),
            Instruction::Blt { rs1, rs2, imm } => vm::branch::execute_blt(rs1, rs2, imm, self),
            Instruction::Bge { rs1, rs2, imm } => vm::branch::execute_bge(rs1, rs2, imm, self),
            Instruction::Bltu { rs1, rs2, imm } => vm::branch::execute_bltu(rs1, rs2, imm, self),
            Instruction::Bgeu { rs1, rs2, imm } => vm::branch::execute_bgeu(rs1, rs2, imm, self),
            Instruction::Add { rd, rs1, rs2 } => vm::opreg::execute_add(rd, rs1, rs2, self),
            Instruction::Sub { rd, rs1, rs2 } => vm::opreg::execute_sub(rd, rs1, rs2, self),
            Instruction::Sll { rd, rs1, rs2 } => vm::opreg::execute_sll(rd, rs1, rs2, self),
            Instruction::Srl { rd, rs1, rs2 } => vm::opreg::execute_srl(rd, rs1, rs2, self),
            Instruction::Sra { rd, rs1, rs2 } => vm::opreg::execute_sra(rd, rs1, rs2, self),
            Instruction::Slt { rd, rs1, rs2 } => vm::opreg::execute_slt(rd, rs1, rs2, self),
            Instruction::Sltu { rd, rs1, rs2 } => vm::opreg::execute_sltu(rd, rs1, rs2, self),
            Instruction::Xor { rd, rs1, rs2 } => vm::opreg::execute_xor(rd, rs1, rs2, self),
            Instruction::Or { rd, rs1, rs2 } => vm::opreg::execute_or(rd, rs1, rs2, self),
            Instruction::And { rd, rs1, rs2 } => vm::opreg::execute_and(rd, rs1, rs2, self),
            Instruction::Addw { rd, rs1, rs2 } => vm::opreg::execute_addw(rd, rs1, rs2, self),
            Instruction::Subw { rd, rs1, rs2 } => vm::opreg::execute_subw(rd, rs1, rs2, self),
            Instruction::Sllw { rd, rs1, rs2 } => vm::opreg::execute_sllw(rd, rs1, rs2, self),
            Instruction::Srlw { rd, rs1, rs2 } => vm::opreg::execute_srlw(rd, rs1, rs2, self),
            Instruction::Sraw { rd, rs1, rs2 } => vm::opreg::execute_sraw(rd, rs1, rs2, self),
            Instruction::Mul { rd, rs1, rs2 } => vm::opreg::execute_mul(rd, rs1, rs2, self),
            Instruction::Mulw { rd, rs1, rs2 } => vm::opreg::execute_mulw(rd, rs1, rs2, self),
            Instruction::Mulh { rd, rs1, rs2 } => vm::opreg::execute_mulh(rd, rs1, rs2, self),
            Instruction::Mulhu { rd, rs1, rs2 } => vm::opreg::execute_mulhu(rd, rs1, rs2, self),
            Instruction::Mulhsu { rd, rs1, rs2 } => vm::opreg::execute_mulhsu(rd, rs1, rs2, self),
            Instruction::Div { rd, rs1, rs2 } => vm::opreg::execute_div(rd, rs1, rs2, self),
            Instruction::Divu { rd, rs1, rs2 } => vm::opreg::execute_divu(rd, rs1, rs2, self),
            Instruction::Rem { rd, rs1, rs2 } => vm::opreg::execute_rem(rd, rs1, rs2, self),
            Instruction::Remu { rd, rs1, rs2 } => vm::opreg::execute_remu(rd, rs1, rs2, self),
            Instruction::Divw { rd, rs1, rs2 } => vm::opreg::execute_divw(rd, rs1, rs2, self),
            Instruction::Divuw { rd, rs1, rs2 } => vm::opreg::execute_divuw(rd, rs1, rs2, self),
            Instruction::Remw { rd, rs1, rs2 } => vm::opreg::execute_remw(rd, rs1, rs2, self),
            Instruction::Remuw { rd, rs1, rs2 } => vm::opreg::execute_remuw(rd, rs1, rs2, self),

            Instruction::Lr {
                rd,
                rs1,
                aq,
                rl,
                width,
            } => vm::atomic::execute_lr(rd, rs1, (aq, rl), width, bus, self),
            Instruction::Sc {
                rd,
                rs1,
                rs2,
                aq,
                rl,
                width,
            } => vm::atomic::execute_sc(rd, rs1, rs2, (aq, rl), width, bus, self),
            Instruction::Amoxor {
                rd,
                rs1,
                rs2,
                aq,
                rl,
                width,
            } => vm::atomic::execute_amxor(rd, rs1, rs2, (aq, rl), width, bus, self),
            Instruction::Amoor {
                rd,
                rs1,
                rs2,
                aq,
                rl,
                width,
            } => vm::atomic::execute_amoor(rd, rs1, rs2, (aq, rl), width, bus, self),
            Instruction::Amoand {
                rd,
                rs1,
                rs2,
                aq,
                rl,
                width,
            } => vm::atomic::execute_amoand(rd, rs1, rs2, (aq, rl), width, bus, self),
            Instruction::Amoadd {
                rd,
                rs1,
                rs2,
                aq,
                rl,
                width,
            } => vm::atomic::execute_amoadd(rd, rs1, rs2, (aq, rl), width, bus, self),
            Instruction::Amoswap {
                rd,
                rs1,
                rs2,
                aq,
                rl,
                width,
            } => vm::atomic::execute_amoswap(rd, rs1, rs2, (aq, rl), width, bus, self),
            Instruction::Amomax {
                rd,
                rs1,
                rs2,
                aq,
                rl,
                width,
            } => vm::atomic::execute_amomax(rd, rs1, rs2, (aq, rl), width, bus, self),
            Instruction::Amomaxu {
                rd,
                rs1,
                rs2,
                aq,
                rl,
                width,
            } => vm::atomic::execute_amomaxu(rd, rs1, rs2, (aq, rl), width, bus, self),
            Instruction::Amomin {
                rd,
                rs1,
                rs2,
                aq,
                rl,
                width,
            } => vm::atomic::execute_amomin(rd, rs1, rs2, (aq, rl), width, bus, self),
            Instruction::Amominu {
                rd,
                rs1,
                rs2,
                aq,
                rl,
                width,
            } => vm::atomic::execute_amominu(rd, rs1, rs2, (aq, rl), width, bus, self),
            Instruction::Ecall => {
                tracing::info!("{}", &self);
                Ok(VmOutput::NextInstruction)
            }

            Instruction::Fence => {
                /* its a no op for the vm */
                Ok(VmOutput::NextInstruction)
            }
            Instruction::Fencei => {
                /* its a no op for the vm */
                Ok(VmOutput::NextInstruction)
            }

            Instruction::Ebreak => {
                tracing::warn!("EBREAK at {:#x}", self.registers.pc);
                Ok(VmOutput::NextInstruction)
            }
            Instruction::Sret => todo!(),
            Instruction::Mret => todo!(),
            Instruction::Wfi => todo!(),
            Instruction::Csrrw { rd, rs1, csr } => todo!(),
            Instruction::Csrrs { rd, rs1, csr } => todo!(),
            Instruction::Csrrc { rd, rs1, csr } => todo!(),
            Instruction::Csrrwi { rd, csr, imm } => todo!(),
            Instruction::Csrrsi { rd, csr, imm } => todo!(),
            Instruction::Csrrci { rd, csr, imm } => todo!(),
        };

        match vm_result {
            Ok(output) => match output {
                VmOutput::NextInstruction => self.registers.pc += if is_compressed { 2 } else { 4 },
                VmOutput::Jump(target) => self.registers.pc = target,
            },
            Err(err) => match err {
                VmError::BusError(bus_err) => {
                    tracing::error!("Bus error at address {:?}", bus_err);
                    return crate::err!(bus_err);
                }
            },
        }

        Ok(())
    }
}
