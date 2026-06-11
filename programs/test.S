.global _start

.text
_start:
    # --- 1. U-Type Instructions ---
    lui t0, 0x12345        # t0 = 0x12345000
    auipc t1, 0            # t1 = Current PC

    lb t2, 5(t0)            # Load Byte from address in t0 (should be 0)
    sd t3, 8(t0)            # Store Doubleword to address in t0 + 8
    slli t4, t2, 3         # Shift left logical: t4 = t2 << 3 (should be 0)
    srai t5, t2, 1         # Shift right arithmetic: t5 = t2 >> 1 (should be 0)
    srli t6, t2, 1         # Shift right logical: t6 = t2 >> 1 (should be 0)

    # --- 2. I-Type and R-Type ALU Instructions ---
    li a0, 15              # addi a0, zero, 15
    addi a1, zero, 25      # a1 = 25
    add a2, a0, a1         # a2 = 40 (0x28)
    sub a3, a1, a0         # a3 = 10 (0x0A)

    # Bitwise operations
    xori a4, a0, 0b1010    # a4 = 15 ^ 10 = 5
    xor a5, a4, a1         # a5 = 5 ^ 25 = 28
    or a6, a4, a5          # a6 = 5 | 28 = 29
    andi a7, a6, 0xF       # a7 = 29 & 15 = 13

    slliw s0, a0, 0          # slliw s0, a0, 0 (should be 15)
    srliw s1, a0, 1          # srliw s1, a0, 1 (should be 7)
    sraiw s2, a0, 1          # sraiw s2, a0, 1 (should be 7)

    addiw s0, a0, 0          # addiw s0, a0, 0 (should be 15)

    addw s1, a0, a1          # addw s1, a0, a1 (should be 40)
    
    # Shifts
    slli s0, a0, 2         # s0 = 15 << 2 = 60
    srl s1, s0, a4         # s1 = 60 >> 5 = 1  (Logical)
    sra s2, s0, a4         # s2 = 60 >> 5 = 1  (Arithmetic)

    srlw s3, a0, zero          # s3 = 15
    sllw s4, a0, t1          # s4 = 15 << 1 = 30
    sraw s5, a0, t1          # s5 = 15 >> 1 = 7 (Arithmetic)
    
    fence 
    fence.i

    # --- 3. Branching Instructions ---
    beq a1, a1, test_bne   # 25 == 25, should branch
    j fail_loop

test_bne:
    bne a0, a1, test_blt   # 15 != 25, should branch
    j fail_loop

test_blt:
    blt a0, a1, test_bge   # 15 < 25, should branch
    j fail_loop

test_bge:
    bge a1, a0, test_load_store # 25 >= 15, should branch
    j fail_loop

test_load_store:
    # --- 4. Load and Store Instructions ---
    la t2, my_data         # Load address of data section
    sw a0, 0(t2)           # Store Word: mem[my_data] = 15
    sw a1, 4(t2)           # Store Word: mem[my_data+4] = 25
    
    lw s3, 0(t2)           # Load Word: s3 = 15
    lb s4, 4(t2)           # Load Byte: s4 = 25
    sb s3, 8(t2)           # Store Byte: mem[my_data+8] = 15
    lh s5, 8(t2)           # Load Half: s5 = 15

    # Verify load/store worked
    bne s3, a0, fail_loop  # if 15 != 15 -> fail

    # --- 5. J-Type Instructions ---
    jal ra, my_function    # Jump And Link to my_function
    
    # Function returns here. Return value in a0 should be modified
    li t3, 100
    bne a0, t3, fail_loop

    # --- 6. M Extension (Multiply / Divide) ---
    li t4, 21
    li t5, 4
    
    # RV32M
    mul t6, t4, t5         # t6 = 84
    mulh s0, t4, t5        
    mulhsu s1, t4, t5
    mulhu s2, t4, t5
    div a1, t4, t5         # a1 = 5
    divu a2, t4, t5
    rem a3, t4, t5         # a3 = 1
    remu a4, t4, t5

    # RV64M Specific
    mulw a5, t4, t5        # a5 = 84 (32-bit sign-extended)
    divw a6, t4, t5        # a6 = 5 (32-bit sign-extended)
    divuw a7, t4, t5
    remw t6, t4, t5        # t6 = 1 (32-bit sign-extended)
    remuw s0, t4, t5

    # --- 7. A Extension (Atomics) ---
    la t2, my_data
    li t3, 15
    sw zero, 0(t2)          # Reset memory to 0
    
    # RV32A
    lr.w t5, (t2)           # t5 = 0, lock memory
    addi t5, t5, 20         # t5 = 20
    sc.w t6, t5, (t2)       # mem[my_data] = 20, t6 = 0 (success)
    amoswap.w t5, t3, (t2)  # mem[my_data] = 15, t5 = old_value (20)
    amoadd.w t4, t3, (t2)   # mem[my_data] = 30, t4 = 15
    amoxor.w t4, t3, (t2)
    amoand.w t4, t3, (t2)
    amoor.w t4, t3, (t2)
    amomin.w t4, t3, (t2)
    amomax.w t4, t3, (t2)
    amominu.w t4, t3, (t2)
    amomaxu.w t4, t3, (t2)

    # RV64A
    lr.d t5, (t2)           
    addi t5, t5, 20         
    sc.d t6, t5, (t2)       
    amoswap.d t5, t3, (t2)  
    amoadd.d t4, t3, (t2)   
    amoxor.d t4, t3, (t2)
    amoand.d t4, t3, (t2)
    amoor.d t4, t3, (t2)
    amomin.d t4, t3, (t2)
    amomax.d t4, t3, (t2)
    amominu.d t4, t3, (t2)
    amomaxu.d t4, t3, (t2)

    # --- 8. System & CSR Instructions ---
    # Exception / Traps
    ecall               # Not called to not exit early
    ebreak              # Not called to not trap early
    mret                # Not called as we are not returning from trap
    sret
    wfi                 # Not called so we don't stall
    
    # CSR operations
    csrr t4, mstatus       # Read CSR (mstatus) - this is a pseudo instruction for csrrs t4, mstatus, zero
    li t5, 0b1000          # MIE bit
    csrrs zero, mstatus, t5 # Set MIE in mstatus
    csrrc zero, mstatus, t5 # Clear MIE in mstatus
    csrrw t4, mstatus, t5  # Swap
    csrrwi zero, mstatus, 0 # Write literal 0 to mstatus
    csrrsi zero, mstatus, 0 # Set literal
    csrrci zero, mstatus, 0 # Clear literal

    # --- Success State ---
success_loop:
    li a7, 93              # ecall exit syscall number
    li a0, 0               # exit code 0
    ebreak                 # Trigger debugger/environment break
    j success_loop

fail_loop:
    li a7, 93
    li a0, 1               # exit code 1
    ebreak
    j fail_loop

# Function block
my_function:
    li a0, 100             # Set return value to 100
    jalr zero, 0(ra)       # Return

.data
my_data:
    .word 0x00000000
    .word 0x00000000
    .word 0x00000000
