use crate::instructions::Instruction;
use std::fmt::Display;

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let fmt_aqrl = |aq: bool, rl: bool| match (aq, rl) {
            (true, true) => ".aqrl",
            (true, false) => ".aq",
            (false, true) => ".rl",
            (false, false) => "",
        };

        match self {
            Instruction::Noop => write!(f, "; nop"),

            Instruction::Lui { rd, imm } => write!(f, "lui {}, {:#x}", rd, imm),
            Instruction::Auipc { rd, imm } => write!(f, "auipc {}, {:#x}", rd, imm),

            Instruction::Jal { rd, imm } => write!(f, "jal {}, {:#x}", rd, imm),
            Instruction::Jalr { rd, rs1, imm } => write!(f, "jalr {}, {}({})", rd, imm, rs1),

            Instruction::Beq { rs1, rs2, imm } => write!(f, "beq {}, {}, {:#x}", rs1, rs2, imm),
            Instruction::Bne { rs1, rs2, imm } => write!(f, "bne {}, {}, {:#x}", rs1, rs2, imm),
            Instruction::Blt { rs1, rs2, imm } => write!(f, "blt {}, {}, {:#x}", rs1, rs2, imm),
            Instruction::Bge { rs1, rs2, imm } => write!(f, "bge {}, {}, {:#x}", rs1, rs2, imm),
            Instruction::Bltu { rs1, rs2, imm } => write!(f, "bltu {}, {}, {:#x}", rs1, rs2, imm),
            Instruction::Bgeu { rs1, rs2, imm } => write!(f, "bgeu {}, {}, {:#x}", rs1, rs2, imm),

            Instruction::Lb { rd, imm, rs1, .. } => write!(f, "lb {}, {}({})", rd, imm, rs1),
            Instruction::Lh { rd, imm, rs1, .. } => write!(f, "lh {}, {}({})", rd, imm, rs1),
            Instruction::Lw { rd, imm, rs1, .. } => write!(f, "lw {}, {}({})", rd, imm, rs1),
            Instruction::Lbu { rd, imm, rs1, .. } => write!(f, "lbu {}, {}({})", rd, imm, rs1),
            Instruction::Lhu { rd, imm, rs1, .. } => write!(f, "lhu {}, {}({})", rd, imm, rs1),

            Instruction::Sb { imm, rs1, rs2 } => write!(f, "sb {}, {}({})", rs2, imm, rs1),
            Instruction::Sh { imm, rs1, rs2 } => write!(f, "sh {}, {}({})", rs2, imm, rs1),
            Instruction::Sw { imm, rs1, rs2 } => write!(f, "sw {}, {}({})", rs2, imm, rs1),

            Instruction::Addi { rd, rs1, imm } => write!(f, "addi {}, {}, {}", rd, rs1, imm),
            Instruction::Slti { rd, rs1, imm } => write!(f, "slti {}, {}, {}", rd, rs1, imm),
            Instruction::Sltiu { rd, rs1, imm } => write!(f, "sltiu {}, {}, {}", rd, rs1, imm),
            Instruction::Slli { rd, rs1, imm } => write!(f, "slli {}, {}, {}", rd, rs1, imm),
            Instruction::Srli { rd, rs1, imm } => write!(f, "srli {}, {}, {}", rd, rs1, imm),
            Instruction::Srai { rd, rs1, imm } => write!(f, "srai {}, {}, {}", rd, rs1, imm),
            Instruction::Xori { rd, rs1, imm } => write!(f, "xori {}, {}, {}", rd, rs1, imm),
            Instruction::Ori { rd, rs1, imm } => write!(f, "ori {}, {}, {}", rd, rs1, imm),
            Instruction::Andi { rd, rs1, imm } => write!(f, "andi {}, {}, {}", rd, rs1, imm),

            Instruction::Add { rd, rs1, rs2 } => write!(f, "add {}, {}, {}", rd, rs1, rs2),
            Instruction::Sub { rd, rs1, rs2 } => write!(f, "sub {}, {}, {}", rd, rs1, rs2),
            Instruction::Slt { rd, rs1, rs2 } => write!(f, "slt {}, {}, {}", rd, rs1, rs2),
            Instruction::Sltu { rd, rs1, rs2 } => write!(f, "sltu {}, {}, {}", rd, rs1, rs2),
            Instruction::Sll { rd, rs1, rs2 } => write!(f, "sll {}, {}, {}", rd, rs1, rs2),
            Instruction::Srl { rd, rs1, rs2 } => write!(f, "srl {}, {}, {}", rd, rs1, rs2),
            Instruction::Sra { rd, rs1, rs2 } => write!(f, "sra {}, {}, {}", rd, rs1, rs2),
            Instruction::Xor { rd, rs1, rs2 } => write!(f, "xor {}, {}, {}", rd, rs1, rs2),
            Instruction::Or { rd, rs1, rs2 } => write!(f, "or {}, {}, {}", rd, rs1, rs2),
            Instruction::And { rd, rs1, rs2 } => write!(f, "and {}, {}, {}", rd, rs1, rs2),

            Instruction::Ld { rd, imm, rs1, .. } => write!(f, "ld {}, {}({})", rd, imm, rs1),
            Instruction::Lwu { rd, imm, rs1, .. } => write!(f, "lwu {}, {}({})", rd, imm, rs1),
            Instruction::Sd { imm, rs1, rs2 } => write!(f, "sd {}, {}({})", rs2, imm, rs1),

            Instruction::Addiw { rd, rs1, imm } => write!(f, "addiw {}, {}, {}", rd, rs1, imm),
            Instruction::Slliw { rd, rs1, imm } => write!(f, "slliw {}, {}, {}", rd, rs1, imm),
            Instruction::Srliw { rd, rs1, imm } => write!(f, "srliw {}, {}, {}", rd, rs1, imm),
            Instruction::Sraiw { rd, rs1, imm } => write!(f, "sraiw {}, {}, {}", rd, rs1, imm),
            Instruction::Addw { rd, rs1, rs2 } => write!(f, "addw {}, {}, {}", rd, rs1, rs2),
            Instruction::Subw { rd, rs1, rs2 } => write!(f, "subw {}, {}, {}", rd, rs1, rs2),
            Instruction::Sllw { rd, rs1, rs2 } => write!(f, "sllw {}, {}, {}", rd, rs1, rs2),
            Instruction::Srlw { rd, rs1, rs2 } => write!(f, "srlw {}, {}, {}", rd, rs1, rs2),
            Instruction::Sraw { rd, rs1, rs2 } => write!(f, "sraw {}, {}, {}", rd, rs1, rs2),

            Instruction::Mul { rd, rs1, rs2 } => write!(f, "mul {}, {}, {}", rd, rs1, rs2),
            Instruction::Mulh { rd, rs1, rs2 } => write!(f, "mulh {}, {}, {}", rd, rs1, rs2),
            Instruction::Mulhsu { rd, rs1, rs2 } => write!(f, "mulhsu {}, {}, {}", rd, rs1, rs2),
            Instruction::Mulhu { rd, rs1, rs2 } => write!(f, "mulhu {}, {}, {}", rd, rs1, rs2),
            Instruction::Div { rd, rs1, rs2 } => write!(f, "div {}, {}, {}", rd, rs1, rs2),
            Instruction::Divu { rd, rs1, rs2 } => write!(f, "divu {}, {}, {}", rd, rs1, rs2),
            Instruction::Rem { rd, rs1, rs2 } => write!(f, "rem {}, {}, {}", rd, rs1, rs2),
            Instruction::Remu { rd, rs1, rs2 } => write!(f, "remu {}, {}, {}", rd, rs1, rs2),
            Instruction::Mulw { rd, rs1, rs2 } => write!(f, "mulw {}, {}, {}", rd, rs1, rs2),
            Instruction::Divw { rd, rs1, rs2 } => write!(f, "divw {}, {}, {}", rd, rs1, rs2),
            Instruction::Divuw { rd, rs1, rs2 } => write!(f, "divuw {}, {}, {}", rd, rs1, rs2),
            Instruction::Remw { rd, rs1, rs2 } => write!(f, "remw {}, {}, {}", rd, rs1, rs2),
            Instruction::Remuw { rd, rs1, rs2 } => write!(f, "remuw {}, {}, {}", rd, rs1, rs2),

            Instruction::Amoadd {
                rd,
                rs1,
                rs2,
                aq,
                rl,
            } => write!(
                f,
                "amoadd.w{} {}, {}, ({})",
                fmt_aqrl(*aq, *rl),
                rd,
                rs2,
                rs1
            ),
            Instruction::Amoswap {
                rd,
                rs1,
                rs2,
                aq,
                rl,
            } => write!(
                f,
                "amoswap.w{} {}, {}, ({})",
                fmt_aqrl(*aq, *rl),
                rd,
                rs2,
                rs1
            ),
            Instruction::Lr { rd, rs1, aq, rl } => {
                write!(f, "lr.w{} {}, ({})", fmt_aqrl(*aq, *rl), rd, rs1)
            }
            Instruction::Sc {
                rd,
                rs1,
                rs2,
                aq,
                rl,
            } => write!(f, "sc.w{} {}, {}, ({})", fmt_aqrl(*aq, *rl), rd, rs2, rs1),
            Instruction::Amoxor {
                rd,
                rs1,
                rs2,
                aq,
                rl,
            } => write!(
                f,
                "amoxor.w{} {}, {}, ({})",
                fmt_aqrl(*aq, *rl),
                rd,
                rs2,
                rs1
            ),
            Instruction::Amoor {
                rd,
                rs1,
                rs2,
                aq,
                rl,
            } => write!(
                f,
                "amoor.w{} {}, {}, ({})",
                fmt_aqrl(*aq, *rl),
                rd,
                rs2,
                rs1
            ),
            Instruction::Amoand {
                rd,
                rs1,
                rs2,
                aq,
                rl,
            } => write!(
                f,
                "amoand.w{} {}, {}, ({})",
                fmt_aqrl(*aq, *rl),
                rd,
                rs2,
                rs1
            ),
            Instruction::Amomin {
                rd,
                rs1,
                rs2,
                aq,
                rl,
            } => write!(
                f,
                "amomin.w{} {}, {}, ({})",
                fmt_aqrl(*aq, *rl),
                rd,
                rs2,
                rs1
            ),
            Instruction::Amomax {
                rd,
                rs1,
                rs2,
                aq,
                rl,
            } => write!(
                f,
                "amomax.w{} {}, {}, ({})",
                fmt_aqrl(*aq, *rl),
                rd,
                rs2,
                rs1
            ),
            Instruction::Amominu {
                rd,
                rs1,
                rs2,
                aq,
                rl,
            } => write!(
                f,
                "amominu.w{} {}, {}, ({})",
                fmt_aqrl(*aq, *rl),
                rd,
                rs2,
                rs1
            ),
            Instruction::Amomaxu {
                rd,
                rs1,
                rs2,
                aq,
                rl,
            } => write!(
                f,
                "amomaxu.w{} {}, {}, ({})",
                fmt_aqrl(*aq, *rl),
                rd,
                rs2,
                rs1
            ),

            Instruction::Fence => write!(f, "fence"),

            Instruction::Fencei => write!(f, "fence.i"),
            Instruction::Ecall => write!(f, "ecall"),
            Instruction::Ebreak => write!(f, "ebreak"),
            Instruction::Sret => write!(f, "sret"),
            Instruction::Mret => write!(f, "mret"),
            Instruction::Wfi => write!(f, "wfi"),

            Instruction::Csrrw { rd, rs1, csr } => write!(f, "csrrw {}, {}, {}", rd, csr, rs1),
            Instruction::Csrrs { rd, rs1, csr } => write!(f, "csrrs {}, {}, {}", rd, csr, rs1),
            Instruction::Csrrc { rd, rs1, csr } => write!(f, "csrrc {}, {}, {}", rd, csr, rs1),
            Instruction::Csrrwi { rd, imm, csr } => write!(f, "csrrwi {}, {}, {}", rd, csr, imm),
            Instruction::Csrrsi { rd, imm, csr } => write!(f, "csrrsi {}, {}, {}", rd, csr, imm),
            Instruction::Csrrci { rd, imm, csr } => write!(f, "csrrci {}, {}, {}", rd, csr, imm),
        }
    }
}
