.global _start

.text
_start:
    # --- 1. U-Type Instructions ---
    lui t0, 0x12345        # t0 = 0x12345000
    auipc t1, 0            # t1 = Current PC

    lb t2, 5(t0)            # Load Byte from address in t0 (should be 0)
    sd t3, 8(t0)            # Store Doubleword to address in t0 + 8

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
    
    # Shifts
    slli s0, a0, 2         # s0 = 15 << 2 = 60
    srl s1, s0, a4         # s1 = 60 >> 5 = 1  (Logical)
    sra s2, s0, a4         # s2 = 60 >> 5 = 1  (Arithmetic)

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

    # --- 6. System & CSR Instructions ---
    csrr t4, mstatus       # Read CSR

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
