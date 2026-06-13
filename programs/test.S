.global _start
_start:
        lui a0, 1
        ecall
        j       _start # debug infinite loop
        addi    sp,sp,-32
        sd      s0,16(sp)
        sd      s1,8(sp)
        sd      ra,24(sp)
        li      s0,0
        li      s1,10
.L62:
        mv      a0,s0
        call    fact@plt
        addiw   s0,s0,1
        call    fib@plt
        bne     s0,s1,.L62
.L63:
        j       .L63
        
fib:
        addi    sp,sp,-272
        sd      s6,208(sp)
        sd      ra,264(sp)
        li      s6,1
        ble     a0,s6,.L37
        addiw   a5,a0,-1
        andi    a4,a5,-2
        sd      s1,248(sp)
        sd      s3,232(sp)
        sd      s11,168(sp)
        subw    s1,a0,a4
        li      s3,0
        mv      s11,a5
        beq     a0,s1,.L47
.L3:
        sd      s4,224(sp)
        addiw   s4,a0,-2
        andi    a5,s4,-2
        sd      s8,192(sp)
        sd      s9,184(sp)
        sd      s10,176(sp)
        sd      s1,64(sp)
        sd      s0,256(sp)
        mv      s1,s3
        sd      s2,240(sp)
        sd      s5,216(sp)
        sd      s7,200(sp)
        subw    s9,s11,a5
        li      s8,0
        mv      s10,s11
        mv      s3,s4
.L9:
        addiw   a5,s10,-1
        beq     s10,s9,.L48
        addiw   s10,s10,-2
        andi    a4,s10,-2
        subw    s11,a5,a4
        li      s2,0
        sd      s8,72(sp)
        mv      s5,s3
.L13:
        addiw   a3,a5,-1
        beq     a5,s11,.L49
        addiw   a5,a5,-2
        andi    a4,a5,-2
        sd      s2,80(sp)
        li      s7,0
        sd      s9,88(sp)
        sd      s11,96(sp)
        subw    s2,a3,a4
        sd      a5,104(sp)
.L17:
        addiw   a2,a3,-1
        beq     a3,s2,.L50
        addiw   a3,a3,-2
        andi    s3,a3,-2
        subw    s3,a2,s3
        sd      s1,112(sp)
        sd      s7,120(sp)
        li      s11,0
        sd      s2,128(sp)
        sd      s3,56(sp)
        sd      s5,136(sp)
        sd      s10,144(sp)
        sd      a3,152(sp)
.L21:
        ld      a5,56(sp)
        addiw   a6,a2,-1
        beq     a2,a5,.L51
        addiw   s9,a2,-2
        andi    s2,s9,-2
        subw    s2,a6,s2
        sd      s11,40(sp)
        li      a3,0
        addiw   s11,a2,-4
        sd      s2,32(sp)
        sd      s9,48(sp)
.L25:
        ld      a5,32(sp)
        beq     a6,a5,.L52
        addiw   a2,a6,-2
        andi    a5,a2,-2
        addiw   s3,a6,-3
        addiw   s2,a6,-5
        subw    s3,s3,a5
        andi    a5,s11,-2
        subw    s7,s2,a5
        sd      a3,16(sp)
        mv      s2,s11
        sd      a2,24(sp)
        li      s4,0
        addiw   a5,s2,1
        beq     s3,s2,.L53
.L26:
        mv      s9,s2
        addiw   s8,s2,-2
        sd      s2,8(sp)
        mv      s5,s3
        sd      s4,0(sp)
        li      s1,0
        mv      s2,s8
        mv      s3,s9
.L33:
        beq     s3,s6,.L44
        addiw   s4,a5,-2
        andi    a0,s2,-2
        addiw   a5,a5,-4
        subw    s10,a5,a0
        mv      s9,s4
        li      s0,0
.L35:
        mv      a0,s9
        call    fib.localalias
        addiw   s9,s9,-2
        add     s0,a0,s0
        bne     s10,s9,.L35
        srliw   a5,s2,1
        negw    a5,a5
        addiw   s3,s3,-2
        slli    a5,a5,1
        add     a5,a5,s3
        add     a5,a5,s0
        add     s1,a5,s1
        addi    s2,s2,-2
        mv      a5,s4
        bne     s4,s6,.L33
.L44:
        ld      s4,0(sp)
        addiw   a2,s1,1
        ld      s2,8(sp)
        mv      s3,s5
        add     s4,s4,a2
        beq     s7,s8,.L28
        mv      s2,s8
        addiw   a5,s2,1
        bne     s3,s2,.L26
.L53:
        ld      a3,16(sp)
        ld      a2,24(sp)
        addw    a4,a5,s4
.L27:
        add     a3,a3,a4
        addiw   s11,s11,-2
        mv      a6,a2
        bne     a2,s6,.L25
        ld      s11,40(sp)
        ld      s9,48(sp)
        addiw   a4,a3,1
        j       .L23
.L52:
        ld      s11,40(sp)
        ld      s9,48(sp)
        addi    a4,a6,-1
        add     a4,a4,a3
.L23:
        add     s11,s11,a4
        mv      a2,s9
        bne     s9,s6,.L21
        ld      s1,112(sp)
        ld      s7,120(sp)
        ld      s2,128(sp)
        ld      s5,136(sp)
        ld      s10,144(sp)
        ld      a3,152(sp)
        addiw   a6,s11,1
.L19:
        add     s7,s7,a6
        bne     a3,s6,.L17
        ld      s2,80(sp)
        ld      s9,88(sp)
        ld      s11,96(sp)
        ld      a5,104(sp)
        addiw   a2,s7,1
        j       .L15
.L51:
        ld      s1,112(sp)
        ld      s7,120(sp)
        ld      s2,128(sp)
        ld      s5,136(sp)
        ld      s10,144(sp)
        ld      a3,152(sp)
        addw    a6,a6,s11
        j       .L19
.L50:
        ld      s2,80(sp)
        ld      s9,88(sp)
        ld      s11,96(sp)
        ld      a5,104(sp)
        addw    a2,a2,s7
.L15:
        add     s2,s2,a2
        bne     a5,s6,.L13
        ld      s8,72(sp)
        mv      s3,s5
        addiw   a3,s2,1
        j       .L11
.L49:
        ld      s8,72(sp)
        mv      s3,s5
        addw    a3,a3,s2
.L11:
        add     s8,s8,a3
        bne     s10,s6,.L9
        mv      s4,s3
        addiw   s8,s8,1
        mv      s3,s1
        mv      a0,s4
        ld      s1,64(sp)
        add     s3,s3,s8
        bne     s4,s6,.L54
.L38:
        ld      s0,256(sp)
        ld      ra,264(sp)
        ld      s1,248(sp)
        ld      s2,240(sp)
        ld      s4,224(sp)
        ld      s5,216(sp)
        ld      s7,200(sp)
        ld      s8,192(sp)
        ld      s9,184(sp)
        ld      s10,176(sp)
        ld      s11,168(sp)
        ld      s6,208(sp)
        addiw   a0,s3,1
        ld      s3,232(sp)
        addi    sp,sp,272
        jr      ra
.L48:
        mv      s4,s3
        addw    s8,a5,s8
        mv      s3,s1
        mv      a0,s4
        ld      s1,64(sp)
        add     s3,s3,s8
        beq     s4,s6,.L38
.L54:
        addiw   a5,s4,-1
        ld      s0,256(sp)
        ld      s2,240(sp)
        ld      s4,224(sp)
        ld      s5,216(sp)
        ld      s7,200(sp)
        ld      s8,192(sp)
        ld      s9,184(sp)
        ld      s10,176(sp)
        mv      s11,a5
        bne     a0,s1,.L3
.L47:
        addw    a0,a5,s3
        ld      s1,248(sp)
        ld      s3,232(sp)
        ld      s11,168(sp)
.L37:
        ld      ra,264(sp)
        ld      s6,208(sp)
        addi    sp,sp,272
        jr      ra
.L28:
        ld      a3,16(sp)
        ld      a2,24(sp)
        addw    a4,s2,s4
        j       .L27
        .set    fib.localalias,fib
fact:
        mv      a5,a0
        li      a0,1
        mv      a3,a0
        beq     a5,a0,.L59
.L56:
        mulw    a0,a5,a0
        addiw   a5,a5,-1
        bne     a5,a3,.L56
.L59:
        ret
