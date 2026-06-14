
target/riscv64imac-unknown-none-elf/release/firmware:	file format elf64-littleriscv

Disassembly of section .text:

0000000080000000 <_start>:
80000000: 00008117     	auipc	sp, 0x8
80000004: 00010113     	mv	sp, sp
80000008: 00000097     	auipc	ra, 0x0
8000000c: 00000073     	ecall
80000010: 00000097     	auipc	ra, 0x0
80000014: 00a080e7     	jalr	0xa(ra) <runtime_init>

0000000080000018 <_RNvCsdBezzDwma51_7___rustc17rust_begin_unwind>:
80000018: a001         	j	0x80000018 <_RNvCsdBezzDwma51_7___rustc17rust_begin_unwind>

000000008000001a <runtime_init>:
8000001a: 1141         	addi	sp, sp, -0x10
8000001c: e422         	sd	s0, 0x8(sp)

000000008000001e <.Lpcrel_hi11>:
8000001e: 00004517     	auipc	a0, 0x4
80000022: 39a50513     	addi	a0, a0, 0x39a
80000026: ffc57593     	andi	a1, a0, -0x4
8000002a: 4605         	li	a2, 0x1
8000002c: 44c5a6af     	amoor.w.aq	a3, a2, (a1)
80000030: 0ff6f693     	zext.b	a3, a3
80000034: ca91         	beqz	a3, 0x80000048 <.Lpcrel_hi12>
80000036: 00050683     	lb	a3, 0x0(a0)
8000003a: daed         	beqz	a3, 0x8000002c <.Lpcrel_hi11+0xe>
8000003c: 0100000f     	fence	w, 0
80000040: 00050683     	lb	a3, 0x0(a0)
80000044: fee5         	bnez	a3, 0x8000003c <.Lpcrel_hi11+0x1e>
80000046: b7dd         	j	0x8000002c <.Lpcrel_hi11+0xe>

0000000080000048 <.Lpcrel_hi12>:
80000048: 00000597     	auipc	a1, 0x0
8000004c: 6611         	lui	a2, 0x4
8000004e: 37058593     	addi	a1, a1, 0x370
80000052: 00758793     	addi	a5, a1, 0x7
80000056: 95b2         	add	a1, a1, a2
80000058: 9be1         	andi	a5, a5, -0x8
8000005a: ff85ff93     	andi	t6, a1, -0x8
8000005e: 12ffed63     	bltu	t6, a5, 0x80000198 <.Lpcrel_hi15>
80000062: 4681         	li	a3, 0x0
80000064: 00878593     	addi	a1, a5, 0x8
80000068: 00bffc63     	bgeu	t6, a1, 0x80000080 <.Lpcrel_hi12+0x38>
8000006c: 11853583     	ld	a1, 0x118(a0)
80000070: 95b6         	add	a1, a1, a3
80000072: 10b53c23     	sd	a1, 0x118(a0)
80000076: 0310000f     	fence	rw, w
8000007a: 00050023     	sb	zero, 0x0(a0)
8000007e: a001         	j	0x8000007e <.Lpcrel_hi12+0x36>
80000080: 4805         	li	a6, 0x1

0000000080000082 <.Lpcrel_hi13>:
80000082: 00000597     	auipc	a1, 0x0
80000086: 55555337     	lui	t1, 0x55555
8000008a: 33333737     	lui	a4, 0x33333
8000008e: 0f0f1437     	lui	s0, 0xf0f1
80000092: 01010637     	lui	a2, 0x1010
80000096: 01f81893     	slli	a7, a6, 0x1f
8000009a: 1765b283     	ld	t0, 0x176(a1)
8000009e: 55530313     	addi	t1, t1, 0x555
800000a2: 33370f13     	addi	t5, a4, 0x333
800000a6: f0f40393     	addi	t2, s0, -0xf1
800000aa: 10160e13     	addi	t3, a2, 0x101
800000ae: 02031593     	slli	a1, t1, 0x20
800000b2: 932e         	add	t1, t1, a1
800000b4: 020f1593     	slli	a1, t5, 0x20
800000b8: 9f2e         	add	t5, t5, a1
800000ba: 02039593     	slli	a1, t2, 0x20
800000be: 93ae         	add	t2, t2, a1
800000c0: 020e1593     	slli	a1, t3, 0x20
800000c4: 9e2e         	add	t3, t3, a1

00000000800000c6 <.Lpcrel_hi14>:
800000c6: 00000597     	auipc	a1, 0x0
800000ca: 1ba58e93     	addi	t4, a1, 0x1ba
800000ce: 447d         	li	s0, 0x1f
800000d0: a821         	j	0x800000e8 <.Lpcrel_hi14+0x22>
800000d2: 060e         	slli	a2, a2, 0x3
800000d4: 962a         	add	a2, a2, a0
800000d6: 6618         	ld	a4, 0x8(a2)
800000d8: e398         	sd	a4, 0x0(a5)
800000da: e61c         	sd	a5, 0x8(a2)
800000dc: 97ae         	add	a5, a5, a1
800000de: 00878613     	addi	a2, a5, 0x8
800000e2: 96ae         	add	a3, a3, a1
800000e4: f8cfe4e3     	bltu	t6, a2, 0x8000006c <.Lpcrel_hi12+0x24>
800000e8: 40ff85b3     	sub	a1, t6, a5
800000ec: c5c1         	beqz	a1, 0x80000174 <.Lpcrel_hi14+0xae>
800000ee: 0015d613     	srli	a2, a1, 0x1
800000f2: 8dd1         	or	a1, a1, a2
800000f4: 0025d613     	srli	a2, a1, 0x2
800000f8: 8dd1         	or	a1, a1, a2
800000fa: 0045d613     	srli	a2, a1, 0x4
800000fe: 8dd1         	or	a1, a1, a2
80000100: 0085d613     	srli	a2, a1, 0x8
80000104: 8dd1         	or	a1, a1, a2
80000106: 0105d613     	srli	a2, a1, 0x10
8000010a: 8dd1         	or	a1, a1, a2
8000010c: 0205d613     	srli	a2, a1, 0x20
80000110: 8dd1         	or	a1, a1, a2
80000112: fff5c593     	not	a1, a1
80000116: 0015d613     	srli	a2, a1, 0x1
8000011a: 00667633     	and	a2, a2, t1
8000011e: 8d91         	sub	a1, a1, a2
80000120: 01e5f633     	and	a2, a1, t5
80000124: 8189         	srli	a1, a1, 0x2
80000126: 01e5f5b3     	and	a1, a1, t5
8000012a: 95b2         	add	a1, a1, a2
8000012c: 0045d613     	srli	a2, a1, 0x4
80000130: 95b2         	add	a1, a1, a2
80000132: 0075f5b3     	and	a1, a1, t2
80000136: 03c585b3     	mul	a1, a1, t3
8000013a: 91e1         	srli	a1, a1, 0x38
8000013c: 40f00633     	neg	a2, a5
80000140: fff5c593     	not	a1, a1
80000144: 8e7d         	and	a2, a2, a5
80000146: 00b815b3     	sll	a1, a6, a1
8000014a: 00c5e363     	bltu	a1, a2, 0x80000150 <.Lpcrel_hi14+0x8a>
8000014e: 85b2         	mv	a1, a2
80000150: cd95         	beqz	a1, 0x8000018c <.Lpcrel_hi14+0xc6>
80000152: 40b00633     	neg	a2, a1
80000156: 8e6d         	and	a2, a2, a1
80000158: 02560633     	mul	a2, a2, t0
8000015c: 9269         	srli	a2, a2, 0x3a
8000015e: 9676         	add	a2, a2, t4
80000160: 00064703     	lbu	a4, 0x0(a2)
80000164: 863a         	mv	a2, a4
80000166: 00876363     	bltu	a4, s0, 0x8000016c <.Lpcrel_hi14+0xa6>
8000016a: 467d         	li	a2, 0x1f
8000016c: f6e473e3     	bgeu	s0, a4, 0x800000d2 <.Lpcrel_hi14+0xc>
80000170: 85c6         	mv	a1, a7
80000172: b785         	j	0x800000d2 <.Lpcrel_hi14+0xc>
80000174: 04000593     	li	a1, 0x40
80000178: 40f00633     	neg	a2, a5
8000017c: fff5c593     	not	a1, a1
80000180: 8e7d         	and	a2, a2, a5
80000182: 00b815b3     	sll	a1, a6, a1
80000186: fcc5f4e3     	bgeu	a1, a2, 0x8000014e <.Lpcrel_hi14+0x88>
8000018a: b7d9         	j	0x80000150 <.Lpcrel_hi14+0x8a>
8000018c: 04000713     	li	a4, 0x40
80000190: 863a         	mv	a2, a4
80000192: fc877ce3     	bgeu	a4, s0, 0x8000016a <.Lpcrel_hi14+0xa4>
80000196: bfd9         	j	0x8000016c <.Lpcrel_hi14+0xa6>

0000000080000198 <.Lpcrel_hi15>:
80000198: 00000517     	auipc	a0, 0x0

000000008000019c <.Lpcrel_hi16>:
8000019c: 00000597     	auipc	a1, 0x0
800001a0: 12850513     	addi	a0, a0, 0x128
800001a4: 1ac58613     	addi	a2, a1, 0x1ac
800001a8: 45f9         	li	a1, 0x1e
800001aa: 00000097     	auipc	ra, 0x0
800001ae: 008080e7     	jalr	0x8(ra) <_ZN4core9panicking5panic17h9d6ec1dc790c8d5cE>

00000000800001b2 <_ZN4core9panicking5panic17h9d6ec1dc790c8d5cE>:
800001b2: 1141         	addi	sp, sp, -0x10
800001b4: e406         	sd	ra, 0x8(sp)
800001b6: e022         	sd	s0, 0x0(sp)
800001b8: 0800         	addi	s0, sp, 0x10
800001ba: 0586         	slli	a1, a1, 0x1
800001bc: 0585         	addi	a1, a1, 0x1
800001be: 00000097     	auipc	ra, 0x0
800001c2: 008080e7     	jalr	0x8(ra) <_ZN4core9panicking9panic_fmt17h0765cebb7e8be13cE>

00000000800001c6 <_ZN4core9panicking9panic_fmt17h0765cebb7e8be13cE>:
800001c6: 7139         	addi	sp, sp, -0x40
800001c8: fc06         	sd	ra, 0x38(sp)
800001ca: f822         	sd	s0, 0x30(sp)
800001cc: 0080         	addi	s0, sp, 0x40
800001ce: fca43423     	sd	a0, -0x38(s0)
800001d2: fcb43823     	sd	a1, -0x30(s0)
800001d6: fc840513     	addi	a0, s0, -0x38
800001da: 4585         	li	a1, 0x1
800001dc: fca43c23     	sd	a0, -0x28(s0)
800001e0: fec43023     	sd	a2, -0x20(s0)
800001e4: feb41423     	sh	a1, -0x18(s0)
800001e8: fd840513     	addi	a0, s0, -0x28
800001ec: 00000097     	auipc	ra, 0x0
800001f0: e2c080e7     	jalr	-0x1d4(ra) <_RNvCsdBezzDwma51_7___rustc17rust_begin_unwind>
800001f4: 0000         	unimp
800001f6: 0000         	unimp
800001f8: cd3d5dbf 0218a392    	<unknown>

0000000080000200 <.LCPI5_1>:
80000200: 0100         	addi	s0, sp, 0x80
80000202: 0702         	c.slli64	a4
80000204: 13080d03     	lb	s10, 0x130(a6)
80000208: 1904         	addi	s1, sp, 0xb0
8000020a: 1c0e         	slli	s8, s8, 0x23
8000020c: 2209         	addiw	tp, tp, 0x2
8000020e: 2814         	<unknown>
80000210: 1105         	addi	sp, sp, -0x1f
80000212: 261a         	<unknown>
80000214: 301d2e0f     	<unknown>
80000218: 1f0a         	slli	t5, t5, 0x22
8000021a: 32153623     	sd	ra, 0x32c(a0)
8000021e: 3929         	addiw	s2, s2, -0x16
80000220: 120c063f 27211b18    	<unknown>
80000228: 2510         	<unknown>
8000022a: 2f2d         	addiw	t5, t5, 0xb
8000022c: 351e         	<unknown>
8000022e: 3831         	addiw	a6, a6, -0x14
80000230: 0b3e         	slli	s6, s6, 0xf
80000232: 2c242017     	auipc	zero, 0x2c242
80000236: 3734         	<unknown>
80000238: 163d         	addi	a2, a2, -0x11
8000023a: 2a3c332b     	<unknown>
8000023e: 01003a3b     	<unknown>

0000000080000240 <.LCPI6_1>:
80000240: 0100         	addi	s0, sp, 0x80
80000242: 0702         	c.slli64	a4
80000244: 13080d03     	lb	s10, 0x130(a6)
80000248: 1904         	addi	s1, sp, 0xb0
8000024a: 1c0e         	slli	s8, s8, 0x23
8000024c: 2209         	addiw	tp, tp, 0x2
8000024e: 2814         	<unknown>
80000250: 1105         	addi	sp, sp, -0x1f
80000252: 261a         	<unknown>
80000254: 301d2e0f     	<unknown>
80000258: 1f0a         	slli	t5, t5, 0x22
8000025a: 32153623     	sd	ra, 0x32c(a0)
8000025e: 3929         	addiw	s2, s2, -0x16
80000260: 120c063f 27211b18    	<unknown>
80000268: 2510         	<unknown>
8000026a: 2f2d         	addiw	t5, t5, 0xb
8000026c: 351e         	<unknown>
8000026e: 3831         	addiw	a6, a6, -0x14
80000270: 0b3e         	slli	s6, s6, 0xf
80000272: 2c242017     	auipc	zero, 0x2c242
80000276: 3734         	<unknown>
80000278: 163d         	addi	a2, a2, -0x11
8000027a: 2a3c332b     	<unknown>
8000027e: 01003a3b     	<unknown>

0000000080000280 <.LCPI7_1>:
80000280: 0100         	addi	s0, sp, 0x80
80000282: 0702         	c.slli64	a4
80000284: 13080d03     	lb	s10, 0x130(a6)
80000288: 1904         	addi	s1, sp, 0xb0
8000028a: 1c0e         	slli	s8, s8, 0x23
8000028c: 2209         	addiw	tp, tp, 0x2
8000028e: 2814         	<unknown>
80000290: 1105         	addi	sp, sp, -0x1f
80000292: 261a         	<unknown>
80000294: 301d2e0f     	<unknown>
80000298: 1f0a         	slli	t5, t5, 0x22
8000029a: 32153623     	sd	ra, 0x32c(a0)
8000029e: 3929         	addiw	s2, s2, -0x16
800002a0: 120c063f 27211b18    	<unknown>
800002a8: 2510         	<unknown>
800002aa: 2f2d         	addiw	t5, t5, 0xb
800002ac: 351e         	<unknown>
800002ae: 3831         	addiw	a6, a6, -0x14
800002b0: 0b3e         	slli	s6, s6, 0xf
800002b2: 2c242017     	auipc	zero, 0x2c242
800002b6: 3734         	<unknown>
800002b8: 163d         	addi	a2, a2, -0x11
800002ba: 2a3c332b     	<unknown>
800002be: 73613a3b     	<unknown>

00000000800002c0 <.Lanon.2c284481d9903df12fd6b9dc83f34668.0>:
800002c0: 61 73 73 65 72 74 69 6f         assertio
800002c8: 6e 20 66 61 69 6c 65 64         n failed
800002d0: 3a 20 73 74 61 72 74 20         : start 
800002d8: 3c 3d 20 65 6e 64 43 3a         <= endC:
800002e0: 5c 72 75 73 74 5c 63 61         \rust\ca
800002e8: 72 67 6f 5c 72 65 67 69         rgo\regi
800002f0: 73 74 72 79 5c 73 72 63         stry\src
800002f8: 5c 69 6e 64 65 78 2e 63         \index.c
80000300: 72 61 74 65 73 2e 69 6f         rates.io
80000308: 2d 31 39 34 39 63 66 38         -1949cf8
80000310: 63 36 62 35 62 35 35 37         c6b5b557
80000318: 66 5c 62 75 64 64 79 5f         f\buddy_
80000320: 73 79 73 74 65 6d 5f 61         system_a
80000328: 6c 6c 6f 63 61 74 6f 72         llocator
80000330: 2d 30 2e 31 33 2e 30 5c         -0.13.0\
80000338: 73 72 63 5c 6c 69 62 2e         src\lib.
80000340: 72 73 00 00 00 00 00 00         rs......

0000000080000348 <.Lanon.2c284481d9903df12fd6b9dc83f34668.2>:
80000348: de 02 00 80 00 00 00 00         ........
80000350: 64 00 00 00 00 00 00 00         d.......
80000358: 5e 00 00 00 09 00 00 00         ^.......
