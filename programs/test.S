.global _start
_start:
    li a0, 42        # Load the value 42 into register a0
    li a7, 1         # Load the syscall number for write (1) into register a7
    add a3, a0, a0   # Double the value in a0 and store it in a3
    csrr t0, mstatus  # Read the current mstatus into t0
1:  j 1b             # Jump back to the label 1 (infinite loop)