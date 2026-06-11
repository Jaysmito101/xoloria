use crate::registers::{ControlRegisterName, GeneralRegisterName};

// NOTE: most of the docs here are taken from:
// https://docs.riscv.org/reference/isa/unpriv/rv32.html
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Instruction {
    // Custom emulator specific noop
    Noop,

    // RV32I base integer isa
    /// Load upper immediate, this is used to build 32-bit constants
    /// and uses the U-type format. It places the 32-bit U-immedaite
    /// in the destination register, with the lower 12 bits set to zero.
    Lui {
        rd: GeneralRegisterName,
        imm: i32,
    },

    /// Add upper immediate to pc, this is used to build pc-relative
    /// addresses and uses the U-type format. This forms a 32-bit offset
    /// from the U-immediate, filling the lowest 12 bits with zero, and
    /// then adds this offset to the address of the auipc ionstruction,
    /// then places the result in register rd.
    Auipc {
        rd: GeneralRegisterName,
        imm: i32,
    },

    /// The Jump and link instruction uses the J-type format, where the
    /// J-immeidate encodes a signed offset in multiple of 2 bytes. The
    /// offset is sign-extendted and added to the address of the jump
    /// instruction to form the jump targer address. Jumps can therefore
    /// target a +- 1MiB range. JAL storees the address of the instruction
    /// following the jump ('pc' + 4) into register rd.
    ///
    /// The standard software calling convention uses 'x1' as the return
    /// address and 'x5' as an alternate link register.
    Jal {
        rd: GeneralRegisterName,
        imm: i32,
    },

    /// The indirect jump instruction JALR (jump and link register) uses
    /// the I-type encoding. The target address is obtained by adding the
    /// sign-extendted 12-bit I-immediate to the register rs1, then setting
    /// the least-significant bit of the result to zero. The address
    /// following the jump is written to register rd.
    ///
    /// Register x0 (zero) can be used used as the denstination if the result
    /// is not required
    Jalr {
        rd: GeneralRegisterName,
        imm: i32,
        rs1: GeneralRegisterName,
    },

    /// All branch instructions use the B-type instruction format.
    /// The 12-bit B-immediate encodes signed offsets in multiples of 2 bytes.
    /// The offset is sign-extended and added to the address of the
    /// branch instruction to give the target address. The conditional
    /// branch range is ±4 KiB.
    ///
    /// Beq tiake thje branch if registers rs1 and rs2 are equal
    Beq {
        rs1: GeneralRegisterName,
        rs2: GeneralRegisterName,
        imm: i32,
    },

    /// Bne tiake thje branch if registers rs1 and rs2 are unequal
    Bne {
        rs1: GeneralRegisterName,
        rs2: GeneralRegisterName,
        imm: i32,
    },

    /// Blt tiake thje branch if register rs1 is less than register rs2 (signed)
    Blt {
        rs1: GeneralRegisterName,
        rs2: GeneralRegisterName,
        imm: i32,
    },

    /// Bge tiake thje branch if register rs1 is greater than or equal to register rs2 (signed)
    Bge {
        rs1: GeneralRegisterName,
        rs2: GeneralRegisterName,
        imm: i32,
    },

    /// Bltu tiake thje branch if register rs1 is less than register rs2 (unsigned)
    Bltu {
        rs1: GeneralRegisterName,
        rs2: GeneralRegisterName,
        imm: i32,
    },

    /// Bgeu tiake thje branch if register rs1 is greater than or equal to register rs2 (unsigned)
    Bgeu {
        rs1: GeneralRegisterName,
        rs2: GeneralRegisterName,
        imm: i32,
    },

    /// Loads copy a value from memory to register rd. Loads are encoded in the I-type format.
    /// The effective address is obtained by adding register rs1 to the sign-extended 12-bit offset.
    ///
    /// The LB instruction loads an 8-bit value from memory, then sign-extends to 32-bits before storing in rd
    Lb {
        rd: GeneralRegisterName,
        imm: i32,
        rs1: GeneralRegisterName,
    },

    /// LH loads a 16-bit value from memory, then sign-extends to 32-bits before storing in rd
    Lh {
        rd: GeneralRegisterName,
        imm: i32,
        rs1: GeneralRegisterName,
    },

    /// The LW instruction loads a 32-bit value from memory into rd
    Lw {
        rd: GeneralRegisterName,
        imm: i32,
        rs1: GeneralRegisterName,
    },

    // Unsigned loads zero-extend the loaded value instead of sign-extending it.
    /// The effective address is calculated the same way as for signed loads.
    Lbu {
        rd: GeneralRegisterName,
        imm: i32,
        rs1: GeneralRegisterName,
    },

    /// Lhu loads a 16-bit value from memory, then zero-extends to 32-bits before storing in rd
    /// The effective address is calculated the same way as for signed loads.
    Lhu {
        rd: GeneralRegisterName,
        imm: i32,
        rs1: GeneralRegisterName,
    },

    /// Stores 8 bit values from the low bits of register rs2 to memory.
    Sb {
        imm: i32,
        rs1: GeneralRegisterName,
        rs2: GeneralRegisterName,
    },

    /// Stores 16 bit values from the low bits of register rs2 to memory.
    Sh {
        imm: i32,
        rs1: GeneralRegisterName,
        rs2: GeneralRegisterName,
    },

    /// Stores 32 bit values from register rs2 to memory.
    Sw {
        imm: i32,
        rs1: GeneralRegisterName,
        rs2: GeneralRegisterName,
    },

    /// ADDI adds the sign-extended 12-bit immediate to register rs1.
    /// Arithmetic overflow is ignored and the result is simply the low
    /// XLEN bits of the result.
    ///
    /// `ADDI rd, rs1, 0` is used to implement the `MV rd, rs1` assembler pseudoinstruction.
    Addi {
        rd: GeneralRegisterName,
        imm: i32,
        rs1: GeneralRegisterName,
    },

    /// SLTI (set less than immediate) places the value 1 in register rd if register
    /// rs1 is less than the sign-extended immediate when both are treated as signed numbers,
    /// else 0 is written to rd.
    Slti {
        rd: GeneralRegisterName,
        imm: i32,
        rs1: GeneralRegisterName,
    },

    /// SLTIU is similar but compares the values as unsigned numbers
    /// (i.e., the immediate is first sign-extended to XLEN bits then treated as
    /// an unsigned number). Note, SLTIU rd, rs1, 1 sets rd to 1 if rs1 equals zero,
    /// otherwise sets rd to 0 (assembler pseudoinstruction `SEQZ rd, rs`).
    Sltiu {
        rd: GeneralRegisterName,
        imm: i32,
        rs1: GeneralRegisterName,
    },

    /// Shifts by a constant are encoded as a specialization of the I-type format.
    /// The operand to be shifted is in rs1, and the shift amount is encoded in
    /// the lower 5 bits of the I-immediate field. The right-shift type is encoded in bit 30.
    /// SLLI is a logical left shift (zeros are shifted into the lower bits).
    Slli {
        rd: GeneralRegisterName,
        imm: i32,
        rs1: GeneralRegisterName,
    },

    /// Shifts by a constant are encoded as a specialization of the I-type format.
    /// The operand to be shifted is in rs1, and the shift amount is encoded in
    /// the lower 5 bits of the I-immediate field. The right-shift type is encoded in bit 30.
    /// SRLI is a logical right shift (zeros are shifted into the upper bits).
    Srli {
        rd: GeneralRegisterName,
        imm: i32,
        rs1: GeneralRegisterName,
    },

    /// Shifts by a constant are encoded as a specialization of the I-type format.
    /// The operand to be shifted is in rs1, and the shift amount is encoded in
    /// the lower 5 bits of the I-immediate field. The right-shift type is encoded in bit 30.
    /// SRAI is an arithmetic right shift (the original sign bit is copied into the vacated upper bits).
    Srai {
        rd: GeneralRegisterName,
        imm: i32,
        rs1: GeneralRegisterName,
    },

    /// ANDI, ORI, XORI are logical operations that perform bitwise AND, OR, and XOR on register rs1
    /// and the sign-extended 12-bit immediate and place the result in rd. Note, XORI rd, rs1, -1
    /// performs a bitwise logical inversion of register rs1 (assembler pseudoinstruction NOT rd, rs).
    Xori {
        rd: GeneralRegisterName,
        imm: i32,
        rs1: GeneralRegisterName,
    },

    /// ANDI, ORI, XORI are logical operations that perform bitwise AND, OR, and XOR on register rs1
    /// and the sign-extended 12-bit immediate and place the result in rd. Note, XORI rd, rs1, -1
    /// performs a bitwise logical inversion of register rs1 (assembler pseudoinstruction NOT rd, rs).
    Ori {
        rd: GeneralRegisterName,
        imm: i32,
        rs1: GeneralRegisterName,
    },

    /// ANDI, ORI, XORI are logical operations that perform bitwise AND, OR, and XOR on register rs1
    /// and the sign-extended 12-bit immediate and place the result in rd. Note, XORI rd, rs1, -1
    /// performs a bitwise logical inversion of register rs1 (assembler pseudoinstruction NOT rd, rs).
    Andi {
        rd: GeneralRegisterName,
        imm: i32,
        rs1: GeneralRegisterName,
    },

    /// RV32I defines several arithmetic R-type operations. All operations read the rs1 and rs2
    /// registers as source operands and write the result into register rd.
    ///
    /// ADD performs the addition of rs1 and rs2
    /// Overflows are ignored and the low XLEN bits of results are written to the destination rd
    Add {
        rd: GeneralRegisterName,
        rs1: GeneralRegisterName,
        rs2: GeneralRegisterName,
    },

    /// RV32I defines several arithmetic R-type operations. All operations read the rs1 and rs2
    /// registers as source operands and write the result into register rd.
    ///
    /// SUB performs the subtraction of rs2 from rs1.
    /// Overflows are ignored and the low XLEN bits of results are written to the destination rd
    Sub {
        rd: GeneralRegisterName,
        rs1: GeneralRegisterName,
        rs2: GeneralRegisterName,
    },

    /// SLT perform signed compares, writing 1 to rd if rs1 < rs2, 0 otherwise.
    Slt {
        rd: GeneralRegisterName,
        rs1: GeneralRegisterName,
        rs2: GeneralRegisterName,
    },

    /// SLTU perform unsigned compares respectively, writing 1 to rd if rs1 < rs2, 0 otherwise.
    /// Note, `SLTU rd, x0, rs2` sets rd to 1 if rs2 is not equal to zero, otherwise sets rd to
    /// zero (assembler pseudoinstruction `SNEZ rd, rs`).
    Sltu {
        rd: GeneralRegisterName,
        rs1: GeneralRegisterName,
        rs2: GeneralRegisterName,
    },

    /// SLL perform logical left on the value in register rs1 by the shift amount held in the lower 5 bits of register rs2.
    Sll {
        rd: GeneralRegisterName,
        rs1: GeneralRegisterName,
        rs2: GeneralRegisterName,
    },

    /// SRL perform logical right on the value in register rs1 by the shift amount held in the lower 5 bits of register rs2.
    Srl {
        rd: GeneralRegisterName,
        rs1: GeneralRegisterName,
        rs2: GeneralRegisterName,
    },

    /// SRA perform arithmetic right on the value in register rs1 by the shift amount held in the lower 5 bits of register rs2.
    Sra {
        rd: GeneralRegisterName,
        rs1: GeneralRegisterName,
        rs2: GeneralRegisterName,
    },

    /// Performns biwise XOR on the value in register rs1 and register rs2, writing the result to rd.
    Xor {
        rd: GeneralRegisterName,
        rs1: GeneralRegisterName,
        rs2: GeneralRegisterName,
    },

    /// Performns biwise OR on the value in register rs1 and register rs2, writing the result to rd.
    Or {
        rd: GeneralRegisterName,
        rs1: GeneralRegisterName,
        rs2: GeneralRegisterName,
    },

    /// Performns biwise AND on the value in register rs1 and register rs2, writing the result to rd.
    And {
        rd: GeneralRegisterName,
        rs1: GeneralRegisterName,
        rs2: GeneralRegisterName,
    },

    // RV64I extensions
    /// The LD instruction loads a 64-bit value from memory into register rd for RV64I.
    Ld {
        rd: GeneralRegisterName,
        imm: i32,
        rs1: GeneralRegisterName,
    },

    /// Unsigned loads zero-extend 32-bit values from memory to 64-bit registers instead
    /// of sign-extending them.
    Lwu {
        rd: GeneralRegisterName,
        imm: i32,
        rs1: GeneralRegisterName,
    },

    /// Stores 64 bit values from register rs2 to memory for RV64I.
    Sd {
        imm: i32,
        rs1: GeneralRegisterName,
        rs2: GeneralRegisterName,
    },

    /// ADDIW is an RV64I instruction that adds the sign-extended 12-bit immediate to
    /// register rs1 and produces the proper sign extension of a 32-bit result in rd.
    /// Overflows are ignored and the result is the low 32 bits of the result sign-extended
    /// to 64 bits. Note, `ADDIW rd, rs1, 0` writes the sign extension of the lower 32 bits
    /// of register rs1 into register rd (assembler pseudoinstruction `SEXT.W`).
    Addiw {
        rd: GeneralRegisterName,
        rs1: GeneralRegisterName,
        imm: i32,
    },

    /// SLLIW, SRLIW, and SRAIW are RV64I-only instructions that are analogously defined but
    /// operate on 32-bit values and sign-extend their 32-bit results to 64 bits.
    /// SLLIW, SRLIW, and SRAIW encodings with imm[5] ≠ 0 are reserved.
    Slliw {
        rd: GeneralRegisterName,
        rs1: GeneralRegisterName,
        imm: i32,
    },

    /// SLLIW, SRLIW, and SRAIW are RV64I-only instructions that are analogously defined but
    /// operate on 32-bit values and sign-extend their 32-bit results to 64 bits.
    /// SLLIW, SRLIW, and SRAIW encodings with imm[5] ≠ 0 are reserved.
    Srliw {
        rd: GeneralRegisterName,
        rs1: GeneralRegisterName,
        imm: i32,
    },

    /// SLLIW, SRLIW, and SRAIW are RV64I-only instructions that are analogously defined but
    /// operate on 32-bit values and sign-extend their 32-bit results to 64 bits.
    /// SLLIW, SRLIW, and SRAIW encodings with imm[5] ≠ 0 are reserved.
    Sraiw {
        rd: GeneralRegisterName,
        rs1: GeneralRegisterName,
        imm: i32,
    },

    /// ADDW and SUBW are RV64I-only instructions that are defined analogously to ADD and SUB
    /// but operate on 32-bit values and produce signed 32-bit results. Overflows are ignored,
    /// and the low 32-bits of the result is sign-extended to 64-bits and written to
    /// the destination register.
    Addw {
        rd: GeneralRegisterName,
        rs1: GeneralRegisterName,
        rs2: GeneralRegisterName,
    },

    /// ADDW and SUBW are RV64I-only instructions that are defined analogously to ADD and SUB
    /// but operate on 32-bit values and produce signed 32-bit results. Overflows are ignored,
    /// and the low 32-bits of the result is sign-extended to 64-bits and written to
    /// the destination register.
    Subw {
        rd: GeneralRegisterName,
        rs1: GeneralRegisterName,
        rs2: GeneralRegisterName,
    },

    /// Shifts by a constant are encoded as a specialization of the I-type format using the same
    /// instruction opcode as RV32I. The operand to be shifted is in rs1, and the shift
    /// amount is encoded in the lower 6 bits of the I-immediate field for RV64I.
    /// The right-shift type is encoded in bit 30.
    ///
    /// SLLIW is a logical left shift (zeros are shifted into the lower bits).
    Sllw {
        rd: GeneralRegisterName,
        rs1: GeneralRegisterName,
        imm: i32,
    },

    /// Shifts by a constant are encoded as a specialization of the I-type format using the same
    /// instruction opcode as RV32I. The operand to be shifted is in rs1, and the shift
    /// amount is encoded in the lower 6 bits of the I-immediate field for RV64I.
    /// The right-shift type is encoded in bit 30.
    ///
    /// SRLIW is a logical right shift (zeros are shifted into the upper bits).
    Srlw {
        rd: GeneralRegisterName,
        rs1: GeneralRegisterName,
        imm: i32,
    },

    /// Shifts by a constant are encoded as a specialization of the I-type format using the same
    /// instruction opcode as RV32I. The operand to be shifted is in rs1, and the shift
    /// amount is encoded in the lower 6 bits of the I-immediate field for RV64I.
    /// The right-shift type is encoded in bit 30.
    ///
    /// SLLIW is an arithmetic right shift (the original sign bit is copied into the vacated upper bits).
    Sraw {
        rd: GeneralRegisterName,
        rs1: GeneralRegisterName,
        imm: i32,
    },

    // M extension
    /// MUL performs an XLEN-bit×XLEN-bit multiplication of rs1 by rs2 and places
    /// the lower XLEN bits in the destination register.
    Mul {
        rd: GeneralRegisterName,
        rs1: GeneralRegisterName,
        rs2: GeneralRegisterName,
    },

    /// MULH, MULHU, and MULHSU perform the same multiplication but return the
    /// upper XLEN bits of the full 2×XLEN-bit product, for signed×signed,
    /// unsigned×unsigned, and rs1×unsigned rs2 multiplication.
    /// If both the high and low bits of the same product are required,
    /// then the recommended code sequence is:
    /// ```
    /// MULH[[S]U] rdh, rs1, rs2
    /// MUL rdl, rs1, rs2
    /// ```
    /// (source register specifiers must be in same order and rdh cannot be the same as rs1 or rs2).
    /// Microarchitectures can then fuse these into a single multiply operation
    /// instead of performing two separate multiplies.
    Mulh {
        rd: GeneralRegisterName,
        rs1: GeneralRegisterName,
        rs2: GeneralRegisterName,
    },

    /// MULH, MULHU, and MULHSU perform the same multiplication but return the
    /// upper XLEN bits of the full 2×XLEN-bit product, for signed×signed,
    /// unsigned×unsigned, and rs1×unsigned rs2 multiplication.
    Mulhsu {
        rd: GeneralRegisterName,
        rs1: GeneralRegisterName,
        rs2: GeneralRegisterName,
    },

    /// MULH, MULHU, and MULHSU perform the same multiplication but return the
    /// upper XLEN bits of the full 2×XLEN-bit product, for signed×signed,
    /// unsigned×unsigned, and rs1×unsigned rs2 multiplication.
    Mulhu {
        rd: GeneralRegisterName,
        rs1: GeneralRegisterName,
        rs2: GeneralRegisterName,
    },

    /// DIV performs an XLEN bits by XLEN bits signed integer division of rs1 by rs2,
    /// rounding towards zero.
    Div {
        rd: GeneralRegisterName,
        rs1: GeneralRegisterName,
        rs2: GeneralRegisterName,
    },

    /// DIVU performs an XLEN bits by XLEN bits unsigned integer division of rs1 by rs2,
    /// rounding towards zero.
    Divu {
        rd: GeneralRegisterName,
        rs1: GeneralRegisterName,
        rs2: GeneralRegisterName,
    },

    /// REM provides the remainder of the corresponding division operation.
    /// For REM, the sign of a nonzero result equals the sign of the dividend.
    Rem {
        rd: GeneralRegisterName,
        rs1: GeneralRegisterName,
        rs2: GeneralRegisterName,
    },

    /// REMU provides the remainder of the corresponding unsigned division operation.
    Remu {
        rd: GeneralRegisterName,
        rs1: GeneralRegisterName,
        rs2: GeneralRegisterName,
    },

    /// MULW is an RV64 instruction that multiplies the lower 32 bits of the source
    /// registers, placing the sign extension of the lower 32 bits of the
    /// result into the destination register.
    Mulw {
        rd: GeneralRegisterName,
        rs1: GeneralRegisterName,
        rs2: GeneralRegisterName,
    },

    /// DIVW is an RV64 instruction that divides the lower 32 bits of rs1 by the
    /// lower 32 bits of rs2, treating them as signed integers, placing the
    /// 32-bit quotient in rd, sign-extended to 64 bits.
    Divw {
        rd: GeneralRegisterName,
        rs1: GeneralRegisterName,
        rs2: GeneralRegisterName,
    },

    /// DIVUW is an RV64 instruction that divides the lower 32 bits of rs1 by the
    /// lower 32 bits of rs2, treating them as unsigned integers, placing the
    /// 32-bit quotient in rd, sign-extended to 64 bits.
    Divuw {
        rd: GeneralRegisterName,
        rs1: GeneralRegisterName,
        rs2: GeneralRegisterName,
    },

    /// REMW is an RV64 instruction that provides the remainder of the corresponding signed division operation.
    Remw {
        rd: GeneralRegisterName,
        rs1: GeneralRegisterName,
        rs2: GeneralRegisterName,
    },

    /// REMUW is an RV64 instruction that provides the remainder of the corresponding unsigned division operation.
    Remuw {
        rd: GeneralRegisterName,
        rs1: GeneralRegisterName,
        rs2: GeneralRegisterName,
    },

    // A extension
    /// AMOADD performs an atomic add operation
    Amoadd {
        rd: GeneralRegisterName,
        rs1: GeneralRegisterName,
        rs2: GeneralRegisterName,
        aq: bool,
        rl: bool,
    },

    /// AMOSWAP performs an atomic swap operation
    Amoswap {
        rd: GeneralRegisterName,
        rs1: GeneralRegisterName,
        rs2: GeneralRegisterName,
        aq: bool,
        rl: bool,
    },

    /// LR (Load-Reserved) loads a value from memory and registers a reservation set
    Lr {
        rd: GeneralRegisterName,
        rs1: GeneralRegisterName,
        aq: bool,
        rl: bool,
    },

    /// SC (Store-Conditional) writes a value to memory if the reservation is still valid
    Sc {
        rd: GeneralRegisterName,
        rs1: GeneralRegisterName,
        rs2: GeneralRegisterName,
        aq: bool,
        rl: bool,
    },

    /// AMOXOR performs an atomic bitwise XOR operation
    Amoxor {
        rd: GeneralRegisterName,
        rs1: GeneralRegisterName,
        rs2: GeneralRegisterName,
        aq: bool,
        rl: bool,
    },

    /// AMOOR performs an atomic bitwise OR operation
    Amoor {
        rd: GeneralRegisterName,
        rs1: GeneralRegisterName,
        rs2: GeneralRegisterName,
        aq: bool,
        rl: bool,
    },

    /// AMOAND performs an atomic bitwise AND operation
    Amoand {
        rd: GeneralRegisterName,
        rs1: GeneralRegisterName,
        rs2: GeneralRegisterName,
        aq: bool,
        rl: bool,
    },

    /// AMOMIN performs an atomic minimum operation (signed)
    Amomin {
        rd: GeneralRegisterName,
        rs1: GeneralRegisterName,
        rs2: GeneralRegisterName,
        aq: bool,
        rl: bool,
    },

    /// AMOMAX performs an atomic maximum operation (signed)
    Amomax {
        rd: GeneralRegisterName,
        rs1: GeneralRegisterName,
        rs2: GeneralRegisterName,
        aq: bool,
        rl: bool,
    },

    /// AMOMINU performs an atomic minimum operation (unsigned)
    Amominu {
        rd: GeneralRegisterName,
        rs1: GeneralRegisterName,
        rs2: GeneralRegisterName,
        aq: bool,
        rl: bool,
    },

    /// AMOMAXU performs an atomic maximum operation (unsigned)
    Amomaxu {
        rd: GeneralRegisterName,
        rs1: GeneralRegisterName,
        rs2: GeneralRegisterName,
        aq: bool,
        rl: bool,
    },

    // System, Zicsr, Zifencei extensions
    /// FENCE instruction ensures that all memory accesses from instructions preceding the fence
    /// in program order (the "predecessor set") appear earlier in the global memory order than
    /// memory accesses from instructions appearing after the fence in program order (the "successor set").
    /// However, fences can optionally further restrict the predecessor set and/or the successor
    /// set to a smaller set of memory accesses in order to provide some speedup. Specifically,
    /// fences have PR, PW, SR, and SW bits which restrict the predecessor and/or successor sets.
    /// The predecessor set includes loads (resp.stores) if and only if PR (resp.PW) is set.
    /// Similarly, the successor set includes loads (resp.stores) if and only if SR (resp.SW) is set.
    Fence {
        predecessor_load: bool,
        predecessor_store: bool,
        successor_load: bool,
        successor_store: bool,
    },

    /// The FENCE.I instruction is used to synchronize the instruction and data streams. RISC-V does not
    /// guarantee that stores to instruction memory will be made visible to instruction fetches on a RISC-V
    /// hart until that hart executes a FENCE.I instruction. A FENCE.I instruction ensures that a subsequent
    /// instruction fetch on a RISC-V hart will see any previous data stores already visible to the same RISC-V hart.
    /// FENCE.I does not ensure that other RISC-V harts' instruction fetches will observe the
    /// local hart’s stores in a multiprocessor system. To make a store to instruction memory visible to
    /// all RISC-V harts, the writing hart also has to execute a data FENCE before requesting that all
    /// remote RISC-V harts execute a FENCE.I.
    Fencei,

    /// The ECALL instruction is used to make a request to the supporting execution environment.
    /// When executed in U-mode, S-mode, or M-mode, it generates an environment-call-from-U-mode exception,
    /// environment-call-from-S-mode exception, or environment-call-from-M-mode exception, respectively,
    /// and performs no other operation.
    Ecall,

    /// The EBREAK instruction is used by debuggers to cause control to be transferred back to a debugging environment.
    /// Unless overridden by an external debug environment, EBREAK raises a breakpoint exception and
    /// performs no other operation.
    Ebreak,

    /// To return after handling a trap, there are separate trap return instructions per privilege level, MRET and SRET.
    /// SRET must be provided if supervisor mode is supported, and should raise an illegal-instruction exception otherwise.
    /// SRET should also raise an illegal-instruction exception when TSR=1 in mstatus
    Sret,

    /// To return after handling a trap, there are separate trap return instructions per privilege level, MRET and SRET.
    /// MRET is always provided.
    Mret,

    /// The Wait for Interrupt instruction (WFI) informs the implementation that the current hart can be
    /// stalled until an interrupt might need servicing. Execution of the WFI instruction can also be
    /// used to inform the hardware platform that suitable interrupts should preferentially be routed to this hart.
    /// WFI is available in all privileged modes, and optionally available to U-mode.
    /// This instruction may raise an illegal-instruction exception when TW=1 in mstatus.
    Wfi,

    /// The CSRRW (Atomic Read/Write CSR) instruction atomically swaps values in the CSRs and integer registers.
    /// CSRRW reads the old value of the CSR, zero-extends the value to XLEN bits, then writes it to integer register rd.
    /// The initial value in rs1 is written to the CSR.
    /// If rd=x0, then the instruction shall not read the CSR and shall not cause any of the side effects
    /// that might occur on a CSR read.
    Csrrw {
        rd: GeneralRegisterName,
        rs1: GeneralRegisterName,
        csr: ControlRegisterName,
    },

    /// The CSRRS (Atomic Read and Set Bits in CSR) instruction reads the value of the CSR, zero-extends
    /// the value to XLEN bits, and writes it to integer register rd. The initial value in integer register
    /// rs1 is treated as a bit mask that specifies bit positions to be set in the CSR.
    /// Any bit that is high in rs1 will cause the corresponding bit to be set in the CSR,
    /// if that CSR bit is writable.
    Csrrs {
        rd: GeneralRegisterName,
        rs1: GeneralRegisterName,
        csr: ControlRegisterName,
    },

    /// The CSRRC (Atomic Read and Clear Bits in CSR) instruction reads the value of the CSR, zero-extends
    /// the value to XLEN bits, and writes it to integer register rd. The initial value in integer
    /// register rs1 is treated as a bit mask that specifies bit positions to be cleared in the CSR.
    /// Any bit that is high in rs1 will cause the corresponding bit to be cleared in the CSR,
    /// if that CSR bit is writable.
    Csrrc {
        rd: GeneralRegisterName,
        rs1: GeneralRegisterName,
        csr: ControlRegisterName,
    },

    /// The CSRRWI, CSRRSI, and CSRRCI variants are similar to CSRRW, CSRRS, and CSRRC respectively,
    /// except they update the CSR using an XLEN-bit value obtained by zero-extending a 5-bit unsigned
    /// immediate (uimm[4:0]) field encoded in the rs1 field instead of a value from an integer register.
    /// For CSRRWI, if rd=x0, then the instruction shall not read the CSR and shall not cause
    /// any of the side effects that might occur on a CSR read. Both CSRRSI and CSRRCI will always
    /// read the CSR and cause any read side effects regardless of rd and rs1 fields.
    Csrrwi {
        rd: GeneralRegisterName,
        rs1: GeneralRegisterName,
        csr: ControlRegisterName,
    },

    /// The CSRRWI, CSRRSI, and CSRRCI variants are similar to CSRRW, CSRRS, and CSRRC respectively,
    /// except they update the CSR using an XLEN-bit value obtained by zero-extending a 5-bit unsigned
    /// immediate (uimm[4:0]) field encoded in the rs1 field instead of a value from an integer register.
    /// For CSRRSI and CSRRCI, if the uimm[4:0] field is zero, then these instructions will not write
    /// to the CSR, and shall not cause any of the side effects that might otherwise occur on a
    /// CSR write, nor raise illegal-instruction exceptions on accesses to read-only CSRs.
    Csrrsi {
        rd: GeneralRegisterName,
        rs1: GeneralRegisterName,
        csr: ControlRegisterName,
    },

    /// The CSRRWI, CSRRSI, and CSRRCI variants are similar to CSRRW, CSRRS, and CSRRC respectively,
    /// except they update the CSR using an XLEN-bit value obtained by zero-extending a 5-bit unsigned
    /// immediate (uimm[4:0]) field encoded in the rs1 field instead of a value from an integer register.
    /// For CSRRSI and CSRRCI, if the uimm[4:0] field is zero, then these instructions will not write
    /// to the CSR, and shall not cause any of the side effects that might otherwise occur on a
    /// CSR write, nor raise illegal-instruction exceptions on accesses to read-only CSRs.
    Csrrci {
        rd: GeneralRegisterName,
        rs1: GeneralRegisterName,
        csr: ControlRegisterName,
    },
}
