# kernels.rs: rustc of the PPC64 toolchain (rustc_bootstrap cell)
# Buck target //examples/vmx128bench:kernels_rs
	.section	.rodata.cst4,"aM",@progbits,4
	.p2align	2, 0x0
.Lrs_CPI0_0:
	.long	0x00000000
	.section	.text.rs_k_abs_f32_r,"ax",@progbits
	.globl	rs_k_abs_f32_r
	.p2align	2
	.type	rs_k_abs_f32_r,@function
rs_k_abs_f32_r:
.Lrs_func_begin0:
	cmpwi	5, 1
	bltlr	0
	lis 6, .Lrs_CPI0_0@ha
	lfs 0, .Lrs_CPI0_0@l(6)
	addi 4, 4, -4
	addi 3, 3, -4
	b .Lrs_BB0_3
.Lrs_BB0_2:
	addi 5, 5, -1
	cmplwi	5, 0
	stfsu 1, 4(3)
	beqlr	0
.Lrs_BB0_3:
	lfsu 1, 4(4)
	fcmpu 0, 1, 0
	bge 0, .Lrs_BB0_2
	fneg 1, 1
	b .Lrs_BB0_2
.Lrs_func_end0:
	.size	rs_k_abs_f32_r, .Lrs_func_end0-.Lrs_func_begin0

	.section	.text.rs_k_abs_i32_r,"ax",@progbits
	.globl	rs_k_abs_i32_r
	.p2align	2
	.type	rs_k_abs_i32_r,@function
rs_k_abs_i32_r:
.Lrs_func_begin1:
	cmpwi	5, 1
	bltlr	0
	addi 4, 4, -4
	addi 3, 3, -4
.Lrs_BB1_2:
	lwzu 6, 4(4)
	addi 5, 5, -1
	cmplwi	5, 0
	srawi 7, 6, 31
	xor 6, 6, 7
	sub	6, 6, 7
	stwu 6, 4(3)
	bne	0, .Lrs_BB1_2
	blr
.Lrs_func_end1:
	.size	rs_k_abs_i32_r, .Lrs_func_end1-.Lrs_func_begin1

	.section	.text.rs_k_add16_f32_r,"ax",@progbits
	.globl	rs_k_add16_f32_r
	.p2align	2
	.type	rs_k_add16_f32_r,@function
rs_k_add16_f32_r:
.Lrs_func_begin2:
	lfs 0, 0(4)
	lfs 1, 0(5)
	fadds 0, 0, 1
	stfs 0, 0(3)
	lfs 0, 4(4)
	lfs 1, 4(5)
	fadds 0, 0, 1
	stfs 0, 4(3)
	lfs 0, 8(4)
	lfs 1, 8(5)
	fadds 0, 0, 1
	stfs 0, 8(3)
	lfs 0, 12(4)
	lfs 1, 12(5)
	fadds 0, 0, 1
	stfs 0, 12(3)
	lfs 0, 16(4)
	lfs 1, 16(5)
	fadds 0, 0, 1
	stfs 0, 16(3)
	lfs 0, 20(4)
	lfs 1, 20(5)
	fadds 0, 0, 1
	stfs 0, 20(3)
	lfs 0, 24(4)
	lfs 1, 24(5)
	fadds 0, 0, 1
	stfs 0, 24(3)
	lfs 0, 28(4)
	lfs 1, 28(5)
	fadds 0, 0, 1
	stfs 0, 28(3)
	lfs 0, 32(4)
	lfs 1, 32(5)
	fadds 0, 0, 1
	stfs 0, 32(3)
	lfs 0, 36(4)
	lfs 1, 36(5)
	fadds 0, 0, 1
	stfs 0, 36(3)
	lfs 0, 40(4)
	lfs 1, 40(5)
	fadds 0, 0, 1
	stfs 0, 40(3)
	lfs 0, 44(4)
	lfs 1, 44(5)
	fadds 0, 0, 1
	stfs 0, 44(3)
	lfs 0, 48(4)
	lfs 1, 48(5)
	fadds 0, 0, 1
	stfs 0, 48(3)
	lfs 0, 52(4)
	lfs 1, 52(5)
	fadds 0, 0, 1
	stfs 0, 52(3)
	lfs 0, 56(4)
	lfs 1, 56(5)
	fadds 0, 0, 1
	stfs 0, 56(3)
	lfs 0, 60(4)
	lfs 1, 60(5)
	fadds 0, 0, 1
	stfs 0, 60(3)
	blr
.Lrs_func_end2:
	.size	rs_k_add16_f32_r, .Lrs_func_end2-.Lrs_func_begin2

	.section	.text.rs_k_add_f32_r,"ax",@progbits
	.globl	rs_k_add_f32_r
	.p2align	2
	.type	rs_k_add_f32_r,@function
rs_k_add_f32_r:
.Lrs_func_begin3:
	cmpwi	6, 1
	bltlr	0
	addi 4, 4, -4
	addi 5, 5, -4
	addi 3, 3, -4
.Lrs_BB3_2:
	lfsu 0, 4(4)
	addi 6, 6, -1
	lfsu 1, 4(5)
	cmplwi	6, 0
	fadds 0, 0, 1
	stfsu 0, 4(3)
	bne	0, .Lrs_BB3_2
	blr
.Lrs_func_end3:
	.size	rs_k_add_f32_r, .Lrs_func_end3-.Lrs_func_begin3

	.section	.text.rs_k_add_i16_r,"ax",@progbits
	.globl	rs_k_add_i16_r
	.p2align	2
	.type	rs_k_add_i16_r,@function
rs_k_add_i16_r:
.Lrs_func_begin4:
	cmpwi	6, 1
	bltlr	0
	addi 4, 4, -2
	addi 5, 5, -2
	addi 3, 3, -2
.Lrs_BB4_2:
	lhzu 7, 2(4)
	addi 6, 6, -1
	lhzu 8, 2(5)
	cmplwi	6, 0
	add 7, 8, 7
	sthu 7, 2(3)
	bne	0, .Lrs_BB4_2
	blr
.Lrs_func_end4:
	.size	rs_k_add_i16_r, .Lrs_func_end4-.Lrs_func_begin4

	.section	.text.rs_k_add_i32,"ax",@progbits
	.globl	rs_k_add_i32
	.p2align	2
	.type	rs_k_add_i32,@function
rs_k_add_i32:
.Lrs_func_begin5:
	cmpwi	6, 1
	bltlr	0
	addi 4, 4, -4
	addi 5, 5, -4
	addi 3, 3, -4
.Lrs_BB5_2:
	lwzu 7, 4(4)
	addi 6, 6, -1
	lwzu 8, 4(5)
	cmplwi	6, 0
	add 7, 8, 7
	stwu 7, 4(3)
	bne	0, .Lrs_BB5_2
	blr
.Lrs_func_end5:
	.size	rs_k_add_i32, .Lrs_func_end5-.Lrs_func_begin5

	.section	.text.rs_k_add_i32_r,"ax",@progbits
	.globl	rs_k_add_i32_r
	.p2align	2
	.type	rs_k_add_i32_r,@function
rs_k_add_i32_r:
.Lrs_func_begin6:
	cmpwi	6, 1
	bltlr	0
	addi 4, 4, -4
	addi 5, 5, -4
	addi 3, 3, -4
.Lrs_BB6_2:
	lwzu 7, 4(4)
	addi 6, 6, -1
	lwzu 8, 4(5)
	cmplwi	6, 0
	add 7, 8, 7
	stwu 7, 4(3)
	bne	0, .Lrs_BB6_2
	blr
.Lrs_func_end6:
	.size	rs_k_add_i32_r, .Lrs_func_end6-.Lrs_func_begin6

	.section	.text.rs_k_add_u8_r,"ax",@progbits
	.globl	rs_k_add_u8_r
	.p2align	2
	.type	rs_k_add_u8_r,@function
rs_k_add_u8_r:
.Lrs_func_begin7:
	cmpwi	6, 1
	bltlr	0
	addi 4, 4, -1
	addi 5, 5, -1
	addi 3, 3, -1
	li 7, 0
.Lrs_BB7_2:
	lbzu 8, 1(4)
	addi 7, 7, 1
	lbzu 9, 1(5)
	cmplw	6, 7
	add 8, 9, 8
	stbu 8, 1(3)
	bne	0, .Lrs_BB7_2
	blr
.Lrs_func_end7:
	.size	rs_k_add_u8_r, .Lrs_func_end7-.Lrs_func_begin7

	.section	.text.rs_k_addsat_u8_r,"ax",@progbits
	.globl	rs_k_addsat_u8_r
	.p2align	2
	.type	rs_k_addsat_u8_r,@function
rs_k_addsat_u8_r:
.Lrs_func_begin8:
	cmpwi	6, 1
	bltlr	0
	addi 4, 4, -1
	addi 5, 5, -1
	addi 3, 3, -1
	li 7, 0
	b .Lrs_BB8_3
.Lrs_BB8_2:
	addi 7, 7, 1
	cmplw	6, 7
	stbu 8, 1(3)
	beqlr	0
.Lrs_BB8_3:
	lbzu 8, 1(4)
	lbzu 9, 1(5)
	add 8, 8, 9
	cmplwi	8, 255
	blt	0, .Lrs_BB8_2
	li 8, 255
	b .Lrs_BB8_2
.Lrs_func_end8:
	.size	rs_k_addsat_u8_r, .Lrs_func_end8-.Lrs_func_begin8

	.section	.text.rs_k_and_u32_r,"ax",@progbits
	.globl	rs_k_and_u32_r
	.p2align	2
	.type	rs_k_and_u32_r,@function
rs_k_and_u32_r:
.Lrs_func_begin9:
	cmpwi	6, 1
	bltlr	0
	addi 4, 4, -4
	addi 5, 5, -4
	addi 3, 3, -4
.Lrs_BB9_2:
	lwzu 7, 4(4)
	addi 6, 6, -1
	lwzu 8, 4(5)
	cmplwi	6, 0
	and 7, 8, 7
	stwu 7, 4(3)
	bne	0, .Lrs_BB9_2
	blr
.Lrs_func_end9:
	.size	rs_k_and_u32_r, .Lrs_func_end9-.Lrs_func_begin9

	.section	.text.rs_k_avg_u8_r,"ax",@progbits
	.globl	rs_k_avg_u8_r
	.p2align	2
	.type	rs_k_avg_u8_r,@function
rs_k_avg_u8_r:
.Lrs_func_begin10:
	cmpwi	6, 1
	bltlr	0
	addi 4, 4, -1
	addi 5, 5, -1
	addi 3, 3, -1
	li 7, 0
.Lrs_BB10_2:
	lbzu 8, 1(4)
	addi 7, 7, 1
	lbzu 9, 1(5)
	cmplw	6, 7
	add 8, 8, 9
	addi 8, 8, 1
	srwi 8, 8, 1
	stbu 8, 1(3)
	bne	0, .Lrs_BB10_2
	blr
.Lrs_func_end10:
	.size	rs_k_avg_u8_r, .Lrs_func_end10-.Lrs_func_begin10

	.section	.text.rs_k_clamp_f32_r,"ax",@progbits
	.globl	rs_k_clamp_f32_r
	.p2align	2
	.type	rs_k_clamp_f32_r,@function
rs_k_clamp_f32_r:
.Lrs_func_begin11:
	cmpwi	7, 1
	bltlr	0
	addi 4, 4, -4
	addi 3, 3, -4
	b .Lrs_BB11_3
.Lrs_BB11_2:
	addi 7, 7, -1
	cmplwi	7, 0
	stfsu 0, 4(3)
	beqlr	0
.Lrs_BB11_3:
	lfsu 0, 4(4)
	fmr 3, 1
	fcmpu 0, 0, 1
	bge 0, .Lrs_BB11_5
	fcmpu 0, 3, 2
	fmr 0, 2
	bgt	0, .Lrs_BB11_2
	b .Lrs_BB11_6
.Lrs_BB11_5:
	fmr 3, 0
	fcmpu 0, 3, 2
	fmr 0, 2
	bgt	0, .Lrs_BB11_2
.Lrs_BB11_6:
	fmr 0, 3
	b .Lrs_BB11_2
.Lrs_func_end11:
	.size	rs_k_clamp_f32_r, .Lrs_func_end11-.Lrs_func_begin11

	.section	.text.rs_k_clamp_i16_r,"ax",@progbits
	.globl	rs_k_clamp_i16_r
	.p2align	2
	.type	rs_k_clamp_i16_r,@function
rs_k_clamp_i16_r:
.Lrs_func_begin12:
	cmpwi	5, 1
	bltlr	0
	addi 4, 4, -2
	addi 3, 3, -2
	b .Lrs_BB12_3
.Lrs_BB12_2:
	addi 5, 5, -1
	cmplwi	5, 0
	sthu 6, 2(3)
	beqlr	0
.Lrs_BB12_3:
	lhau 6, 2(4)
	cmpwi	6, -1000
	ble 0, .Lrs_BB12_5
	cmpwi	6, 1000
	blt	0, .Lrs_BB12_2
	b .Lrs_BB12_6
.Lrs_BB12_5:
	li 6, -1000
	cmpwi	6, 1000
	blt	0, .Lrs_BB12_2
.Lrs_BB12_6:
	li 6, 1000
	b .Lrs_BB12_2
.Lrs_func_end12:
	.size	rs_k_clamp_i16_r, .Lrs_func_end12-.Lrs_func_begin12

	.section	.text.rs_k_copy_u32_r,"ax",@progbits
	.globl	rs_k_copy_u32_r
	.p2align	2
	.type	rs_k_copy_u32_r,@function
rs_k_copy_u32_r:
.Lrs_func_begin13:
	cmpwi	5, 1
	bltlr	0
	addi 4, 4, -4
	addi 3, 3, -4
.Lrs_BB13_2:
	lwzu 6, 4(4)
	addi 5, 5, -1
	cmplwi	5, 0
	stwu 6, 4(3)
	bne	0, .Lrs_BB13_2
	blr
.Lrs_func_end13:
	.size	rs_k_copy_u32_r, .Lrs_func_end13-.Lrs_func_begin13

	.section	.text.rs_k_copy_u8,"ax",@progbits
	.globl	rs_k_copy_u8
	.p2align	2
	.type	rs_k_copy_u8,@function
rs_k_copy_u8:
.Lrs_func_begin14:
	cmpwi	5, 1
	bltlr	0
	addi 4, 4, -1
	addi 3, 3, -1
	li 6, 0
.Lrs_BB14_2:
	lbzu 7, 1(4)
	addi 6, 6, 1
	cmplw	5, 6
	stbu 7, 1(3)
	bne	0, .Lrs_BB14_2
	blr
.Lrs_func_end14:
	.size	rs_k_copy_u8, .Lrs_func_end14-.Lrs_func_begin14

	.section	.rodata.cst4,"aM",@progbits,4
	.p2align	2, 0x0
.Lrs_CPI15_0:
	.long	0x00000000
	.section	.text.rs_k_dot_f32,"ax",@progbits
	.globl	rs_k_dot_f32
	.p2align	2
	.type	rs_k_dot_f32,@function
rs_k_dot_f32:
.Lrs_func_begin15:
	srawi 6, 5, 31
	andc 5, 5, 6
	clrlwi.	5, 5, 2
	lis 6, .Lrs_CPI15_0@ha
	beq	0, .Lrs_BB15_4
	lfs 1, .Lrs_CPI15_0@l(6)
	addi 3, 3, -4
	addi 4, 4, -4
.Lrs_BB15_2:
	lfsu 0, 4(4)
	addi 5, 5, -1
	lfsu 2, 4(3)
	cmplwi	5, 0
	fmuls 0, 2, 0
	fadds 1, 1, 0
	bne	0, .Lrs_BB15_2
	blr
.Lrs_BB15_4:
	lfs 1, .Lrs_CPI15_0@l(6)
	blr
.Lrs_func_end15:
	.size	rs_k_dot_f32, .Lrs_func_end15-.Lrs_func_begin15

	.section	.text.rs_k_dot_i16,"ax",@progbits
	.globl	rs_k_dot_i16
	.p2align	2
	.type	rs_k_dot_i16,@function
rs_k_dot_i16:
.Lrs_func_begin16:
	srawi 6, 5, 31
	andc 7, 5, 6
	cmplwi	7, 0
	beq	0, .Lrs_BB16_8
	stwu 1, -48(1)
	li 6, 0
	cmpwi	5, 2
	li 8, 0
	std 29, 16(1)
	std 30, 24(1)
	blt	0, .Lrs_BB16_5
	rlwinm 8, 7, 0, 1, 30
	addi 9, 3, -4
	addi 10, 4, -4
	mr	12, 8
	li 11, 0
.Lrs_BB16_3:
	lhau 0, 4(9)
	addi 12, 12, -2
	lhau 30, 4(10)
	cmplwi	12, 0
	lha 29, 2(9)
	mullw 0, 30, 0
	lha 30, 2(10)
	add 6, 0, 6
	mullw 0, 30, 29
	add 11, 0, 11
	bne	0, .Lrs_BB16_3
	cmplw	5, 8
	add 6, 11, 6
	beq	0, .Lrs_BB16_7
.Lrs_BB16_5:
	sub	5, 7, 8
	slwi 7, 8, 1
	addi 7, 7, -2
	add 3, 3, 7
	add 4, 4, 7
.Lrs_BB16_6:
	lhau 7, 2(4)
	addi 5, 5, -1
	lhau 8, 2(3)
	cmplwi	5, 0
	mullw 7, 7, 8
	add 6, 7, 6
	bne	0, .Lrs_BB16_6
.Lrs_BB16_7:
	ld 30, 24(1)
	ld 29, 16(1)
	addi 1, 1, 48
	mr	3, 6
	blr
.Lrs_BB16_8:
	li 3, 0
	blr
.Lrs_func_end16:
	.size	rs_k_dot_i16, .Lrs_func_end16-.Lrs_func_begin16

	.section	.rodata.cst4,"aM",@progbits,4
	.p2align	2, 0x0
.Lrs_CPI17_0:
	.long	0xcf000000
.Lrs_CPI17_1:
	.long	0x4effffff
	.section	.text.rs_k_f2i_r,"ax",@progbits
	.globl	rs_k_f2i_r
	.p2align	2
	.type	rs_k_f2i_r,@function
rs_k_f2i_r:
.Lrs_func_begin17:
	cmpwi	5, 1
	bltlr	0
	stwu 1, -32(1)
	lis 7, .Lrs_CPI17_0@ha
	lfs 0, .Lrs_CPI17_0@l(7)
	lis 7, .Lrs_CPI17_1@ha
	lfs 1, .Lrs_CPI17_1@l(7)
	lis 7, 32767
	addi 4, 4, -4
	addi 3, 3, -4
	addi 6, 1, 28
	ori 7, 7, 65535
	b .Lrs_BB17_3
.Lrs_BB17_2:
	addi 5, 5, -1
	cmplwi	5, 0
	stwu 8, 4(3)
	beq	0, .Lrs_BB17_9
.Lrs_BB17_3:
	lfsu 2, 4(4)
	lis 8, -32768
	fcmpu 0, 2, 0
	fctiwz 3, 2
	cror 20, 0, 3
	stfiwx 3, 0, 6
	bc 4, 20, .Lrs_BB17_6
	fcmpu 0, 2, 1
	mr	9, 7
	ble 0, .Lrs_BB17_7
.Lrs_BB17_5:
	fcmpu 0, 2, 2
	li 8, 0
	bun 0, .Lrs_BB17_2
	b .Lrs_BB17_8
.Lrs_BB17_6:
	lwz 8, 28(1)
	fcmpu 0, 2, 1
	mr	9, 7
	bgt	0, .Lrs_BB17_5
.Lrs_BB17_7:
	mr	9, 8
	fcmpu 0, 2, 2
	li 8, 0
	bun 0, .Lrs_BB17_2
.Lrs_BB17_8:
	mr	8, 9
	b .Lrs_BB17_2
.Lrs_BB17_9:
	addi 1, 1, 32
	blr
.Lrs_func_end17:
	.size	rs_k_f2i_r, .Lrs_func_end17-.Lrs_func_begin17

	.section	.rodata.cst4,"aM",@progbits,4
	.p2align	2, 0x0
.Lrs_CPI18_0:
	.long	0x3e800000
.Lrs_CPI18_1:
	.long	0x3f000000
	.section	.text.rs_k_fir3_f32_r,"ax",@progbits
	.globl	rs_k_fir3_f32_r
	.p2align	2
	.type	rs_k_fir3_f32_r,@function
rs_k_fir3_f32_r:
.Lrs_func_begin18:
	srawi 6, 5, 31
	andc 6, 5, 6
	clrlwi.	6, 6, 2
	cmpwi 1, 5, 1
	cror 20, 4, 2
	bclr 12, 20, 0
	lis 5, .Lrs_CPI18_0@ha
	lfs 0, .Lrs_CPI18_0@l(5)
	lis 5, .Lrs_CPI18_1@ha
	lfs 1, .Lrs_CPI18_1@l(5)
	addi 4, 4, -4
	addi 3, 3, -4
.Lrs_BB18_2:
	lfsu 2, 4(4)
	addi 6, 6, -1
	cmplwi	6, 0
	lfs 3, 4(4)
	fmuls 2, 2, 0
	lfs 4, 8(4)
	fmuls 3, 3, 1
	fadds 2, 2, 3
	fmuls 3, 4, 0
	fadds 2, 2, 3
	stfsu 2, 4(3)
	bne	0, .Lrs_BB18_2
	blr
.Lrs_func_end18:
	.size	rs_k_fir3_f32_r, .Lrs_func_end18-.Lrs_func_begin18

	.section	.text.rs_k_g_add_f32,"ax",@progbits
	.globl	rs_k_g_add_f32
	.p2align	2
	.type	rs_k_g_add_f32,@function
rs_k_g_add_f32:
.Lrs_func_begin19:
	lis 3, rs_g_c@ha
	lis 4, rs_g_b@ha
	lis 5, rs_g_a@ha
	la 3, rs_g_c@l(3)
	la 4, rs_g_b@l(4)
	la 5, rs_g_a@l(5)
	addi 3, 3, -8
	addi 4, 4, -8
	addi 5, 5, -8
	li 6, 1024
.Lrs_BB19_1:
	lfsu 0, 8(5)
	addi 6, 6, -2
	lfsu 1, 8(4)
	cmplwi	6, 0
	lfs 2, 4(5)
	lfs 3, 4(4)
	fadds 0, 0, 1
	stfsu 0, 8(3)
	fadds 0, 2, 3
	stfs 0, 4(3)
	bne	0, .Lrs_BB19_1
	blr
.Lrs_func_end19:
	.size	rs_k_g_add_f32, .Lrs_func_end19-.Lrs_func_begin19

	.section	.text.rs_k_g_add_i32,"ax",@progbits
	.globl	rs_k_g_add_i32
	.p2align	2
	.type	rs_k_g_add_i32,@function
rs_k_g_add_i32:
.Lrs_func_begin20:
	lis 3, rs_g_ic@ha
	lis 4, rs_g_ib@ha
	lis 5, rs_g_ia@ha
	la 3, rs_g_ic@l(3)
	la 4, rs_g_ib@l(4)
	la 5, rs_g_ia@l(5)
	addi 3, 3, -8
	addi 4, 4, -8
	addi 5, 5, -8
	li 6, 1024
.Lrs_BB20_1:
	lwzu 7, 8(5)
	addi 6, 6, -2
	lwzu 8, 8(4)
	cmplwi	6, 0
	lwz 9, 4(5)
	lwz 10, 4(4)
	add 7, 8, 7
	stwu 7, 8(3)
	add 7, 10, 9
	stw 7, 4(3)
	bne	0, .Lrs_BB20_1
	blr
.Lrs_func_end20:
	.size	rs_k_g_add_i32, .Lrs_func_end20-.Lrs_func_begin20

	.section	.rodata.cst4,"aM",@progbits,4
	.p2align	2, 0x0
.Lrs_CPI21_0:
	.long	0x00000000
	.section	.text.rs_k_g_dot,"ax",@progbits
	.globl	rs_k_g_dot
	.p2align	2
	.type	rs_k_g_dot,@function
rs_k_g_dot:
.Lrs_func_begin21:
	lis 6, .Lrs_CPI21_0@ha
	lfs 1, .Lrs_CPI21_0@l(6)
	lis 4, rs_g_b@ha
	lis 5, rs_g_a@ha
	la 4, rs_g_b@l(4)
	la 5, rs_g_a@l(5)
	li 3, 0
	addi 4, 4, -4
	addi 5, 5, -4
.Lrs_BB21_1:
	lfsu 0, 4(4)
	addi 3, 3, 4
	lfsu 2, 4(5)
	cmplwi	3, 4096
	fmuls 0, 2, 0
	fadds 1, 1, 0
	bne	0, .Lrs_BB21_1
	blr
.Lrs_func_end21:
	.size	rs_k_g_dot, .Lrs_func_end21-.Lrs_func_begin21

	.section	.text.rs_k_g_saxpy,"ax",@progbits
	.globl	rs_k_g_saxpy
	.p2align	2
	.type	rs_k_g_saxpy,@function
rs_k_g_saxpy:
.Lrs_func_begin22:
	lis 3, rs_g_c@ha
	lis 4, rs_g_b@ha
	lis 5, rs_g_a@ha
	la 3, rs_g_c@l(3)
	la 4, rs_g_b@l(4)
	la 5, rs_g_a@l(5)
	addi 3, 3, -8
	addi 4, 4, -8
	addi 5, 5, -8
	li 6, 1024
.Lrs_BB22_1:
	lfsu 0, 8(5)
	addi 6, 6, -2
	lfsu 2, 8(4)
	cmplwi	6, 0
	lfs 3, 4(5)
	fmuls 0, 1, 0
	lfs 4, 4(4)
	fadds 0, 0, 2
	stfsu 0, 8(3)
	fmuls 0, 1, 3
	fadds 0, 0, 4
	stfs 0, 4(3)
	bne	0, .Lrs_BB22_1
	blr
.Lrs_func_end22:
	.size	rs_k_g_saxpy, .Lrs_func_end22-.Lrs_func_begin22

	.section	.text.rs_k_gather_f32_r,"ax",@progbits
	.globl	rs_k_gather_f32_r
	.p2align	2
	.type	rs_k_gather_f32_r,@function
rs_k_gather_f32_r:
.Lrs_func_begin23:
	cmpwi	6, 1
	bltlr	0
	addi 5, 5, -4
	addi 3, 3, -4
.Lrs_BB23_2:
	lwzu 7, 4(5)
	addi 6, 6, -1
	cmplwi	6, 0
	slwi 7, 7, 2
	lwzx 7, 4, 7
	stwu 7, 4(3)
	bne	0, .Lrs_BB23_2
	blr
.Lrs_func_end23:
	.size	rs_k_gather_f32_r, .Lrs_func_end23-.Lrs_func_begin23

	.section	.text.rs_k_i2f_r,"ax",@progbits
	.globl	rs_k_i2f_r
	.p2align	2
	.type	rs_k_i2f_r,@function
rs_k_i2f_r:
.Lrs_func_begin24:
	cmpwi	5, 1
	bltlr	0
	stwu 1, -32(1)
	addi 4, 4, -4
	addi 3, 3, -4
.Lrs_BB24_2:
	lwa 6, 4(4)
	addi 5, 5, -1
	cmplwi	5, 0
	addi 4, 4, 4
	std 6, 24(1)
	lfd 0, 24(1)
	fcfid 0, 0
	frsp 0, 0
	stfsu 0, 4(3)
	bne	0, .Lrs_BB24_2
	addi 1, 1, 32
	blr
.Lrs_func_end24:
	.size	rs_k_i2f_r, .Lrs_func_end24-.Lrs_func_begin24

	.section	.text.rs_k_mat4_vec4_r,"ax",@progbits
	.globl	rs_k_mat4_vec4_r
	.p2align	2
	.type	rs_k_mat4_vec4_r,@function
rs_k_mat4_vec4_r:
.Lrs_func_begin25:
	cmpwi	6, 1
	bltlr	0
	stwu 1, -48(1)
	addi 5, 5, -16
	addi 3, 3, -16
	stfd 29, 16(1)
	stfd 30, 24(1)
	stfd 31, 32(1)
.Lrs_BB25_2:
	lfsu 0, 16(5)
	addi 6, 6, -1
	lfs 1, 0(4)
	cmplwi	6, 0
	lfs 5, 4(4)
	lfs 9, 8(4)
	fmuls 1, 0, 1
	lfs 13, 12(4)
	fmuls 5, 0, 5
	lfs 2, 16(4)
	fmuls 9, 0, 9
	lfs 6, 20(4)
	fmuls 0, 0, 13
	lfs 10, 24(4)
	lfs 31, 28(4)
	lfs 29, 4(5)
	lfs 3, 32(4)
	lfs 7, 36(4)
	fmuls 2, 29, 2
	lfs 11, 40(4)
	fmuls 6, 29, 6
	lfs 30, 44(4)
	fmuls 10, 29, 10
	lfs 13, 8(5)
	fmuls 31, 29, 31
	lfs 4, 48(4)
	fadds 1, 1, 2
	lfs 29, 12(5)
	fmuls 3, 13, 3
	lfs 8, 52(4)
	fmuls 7, 13, 7
	lfs 12, 56(4)
	fmuls 11, 13, 11
	fmuls 13, 13, 30
	lfs 30, 60(4)
	fmuls 4, 29, 4
	fadds 2, 5, 6
	fadds 1, 1, 3
	fmuls 8, 29, 8
	fadds 5, 9, 10
	fadds 0, 0, 31
	fadds 2, 2, 7
	fadds 1, 1, 4
	fmuls 12, 29, 12
	fmuls 30, 29, 30
	fadds 3, 5, 11
	fadds 0, 0, 13
	stfsu 1, 16(3)
	fadds 1, 2, 8
	stfs 1, 4(3)
	fadds 1, 3, 12
	fadds 0, 0, 30
	stfs 1, 8(3)
	stfs 0, 12(3)
	bne	0, .Lrs_BB25_2
	lfd 31, 32(1)
	lfd 30, 24(1)
	lfd 29, 16(1)
	addi 1, 1, 48
	blr
.Lrs_func_end25:
	.size	rs_k_mat4_vec4_r, .Lrs_func_end25-.Lrs_func_begin25

	.section	.text.rs_k_max_f32,"ax",@progbits
	.globl	rs_k_max_f32
	.p2align	2
	.type	rs_k_max_f32,@function
rs_k_max_f32:
.Lrs_func_begin26:
	mflr 0
	stw 0, -8(1)
	stwu 1, -96(1)
	cmpwi	4, 1
	blt-	0, .Lrs_BB26_8
	lfs 0, 0(3)
	slwi 4, 4, 2
	cmplwi	4, 4
	bne	0, .Lrs_BB26_4
	fmr 1, 0
.Lrs_BB26_3:
	addi 1, 1, 96
	lwz 0, -8(1)
	mtlr 0
	blr
.Lrs_BB26_4:
	addi 4, 4, -4
	b .Lrs_BB26_6
.Lrs_BB26_5:
	addi 4, 4, -4
	cmplwi	4, 0
	fmr 0, 1
	beq	0, .Lrs_BB26_3
.Lrs_BB26_6:
	lfsu 1, 4(3)
	fcmpu 0, 1, 0
	bgt	0, .Lrs_BB26_5
	fmr 1, 0
	b .Lrs_BB26_5
.Lrs_BB26_8:
	lis 3, .Lrs__MergedGlobals@ha
	la 5, .Lrs__MergedGlobals@l(3)
	li 3, 0
	li 4, 0
	bl _RNvNtCsedlPyG8v7DD_4core9panicking18panic_bounds_check
.Lrs_func_end26:
	.size	rs_k_max_f32, .Lrs_func_end26-.Lrs_func_begin26

	.section	.text.rs_k_max_i32_r,"ax",@progbits
	.globl	rs_k_max_i32_r
	.p2align	2
	.type	rs_k_max_i32_r,@function
rs_k_max_i32_r:
.Lrs_func_begin27:
	cmpwi	6, 1
	bltlr	0
	addi 4, 4, -4
	addi 5, 5, -4
	addi 3, 3, -4
	b .Lrs_BB27_3
.Lrs_BB27_2:
	addi 6, 6, -1
	cmplwi	6, 0
	stwu 7, 4(3)
	beqlr	0
.Lrs_BB27_3:
	lwzu 7, 4(4)
	lwzu 8, 4(5)
	cmpw	7, 8
	bgt	0, .Lrs_BB27_2
	mr	7, 8
	b .Lrs_BB27_2
.Lrs_func_end27:
	.size	rs_k_max_i32_r, .Lrs_func_end27-.Lrs_func_begin27

	.section	.text.rs_k_min_f32_r,"ax",@progbits
	.globl	rs_k_min_f32_r
	.p2align	2
	.type	rs_k_min_f32_r,@function
rs_k_min_f32_r:
.Lrs_func_begin28:
	cmpwi	6, 1
	bltlr	0
	addi 4, 4, -4
	addi 5, 5, -4
	addi 3, 3, -4
	b .Lrs_BB28_3
.Lrs_BB28_2:
	addi 6, 6, -1
	cmplwi	6, 0
	stfsu 0, 4(3)
	beqlr	0
.Lrs_BB28_3:
	lfsu 0, 4(4)
	lfsu 1, 4(5)
	fcmpu 0, 0, 1
	blt	0, .Lrs_BB28_2
	fmr 0, 1
	b .Lrs_BB28_2
.Lrs_func_end28:
	.size	rs_k_min_f32_r, .Lrs_func_end28-.Lrs_func_begin28

	.section	.text.rs_k_mul_f32_r,"ax",@progbits
	.globl	rs_k_mul_f32_r
	.p2align	2
	.type	rs_k_mul_f32_r,@function
rs_k_mul_f32_r:
.Lrs_func_begin29:
	cmpwi	6, 1
	bltlr	0
	addi 4, 4, -4
	addi 5, 5, -4
	addi 3, 3, -4
.Lrs_BB29_2:
	lfsu 0, 4(4)
	addi 6, 6, -1
	lfsu 1, 4(5)
	cmplwi	6, 0
	fmuls 0, 0, 1
	stfsu 0, 4(3)
	bne	0, .Lrs_BB29_2
	blr
.Lrs_func_end29:
	.size	rs_k_mul_f32_r, .Lrs_func_end29-.Lrs_func_begin29

	.section	.text.rs_k_mul_i32_r,"ax",@progbits
	.globl	rs_k_mul_i32_r
	.p2align	2
	.type	rs_k_mul_i32_r,@function
rs_k_mul_i32_r:
.Lrs_func_begin30:
	cmpwi	6, 1
	bltlr	0
	addi 4, 4, -4
	addi 5, 5, -4
	addi 3, 3, -4
.Lrs_BB30_2:
	lwzu 7, 4(4)
	addi 6, 6, -1
	lwzu 8, 4(5)
	cmplwi	6, 0
	mullw 7, 8, 7
	stwu 7, 4(3)
	bne	0, .Lrs_BB30_2
	blr
.Lrs_func_end30:
	.size	rs_k_mul_i32_r, .Lrs_func_end30-.Lrs_func_begin30

	.section	.text.rs_k_offset1_f32_r,"ax",@progbits
	.globl	rs_k_offset1_f32_r
	.p2align	2
	.type	rs_k_offset1_f32_r,@function
rs_k_offset1_f32_r:
.Lrs_func_begin31:
	srawi 6, 5, 31
	andc 6, 5, 6
	clrlwi.	6, 6, 2
	cmpwi 1, 5, 1
	cror 20, 4, 2
	bclr 12, 20, 0
	addi 4, 4, -4
	addi 3, 3, -4
.Lrs_BB31_2:
	lfsu 0, 4(4)
	addi 6, 6, -1
	cmplwi	6, 0
	lfs 1, 4(4)
	fadds 0, 0, 1
	stfsu 0, 4(3)
	bne	0, .Lrs_BB31_2
	blr
.Lrs_func_end31:
	.size	rs_k_offset1_f32_r, .Lrs_func_end31-.Lrs_func_begin31

	.section	.text.rs_k_prefix_f32,"ax",@progbits
	.globl	rs_k_prefix_f32
	.p2align	2
	.type	rs_k_prefix_f32,@function
rs_k_prefix_f32:
.Lrs_func_begin32:
	cmpwi	4, 2
	bltlr	0
	lfs 0, 0(3)
	addi 4, 4, -1
.Lrs_BB32_2:
	lfsu 1, 4(3)
	addi 4, 4, -1
	cmplwi	4, 0
	fadds 0, 0, 1
	stfs 0, 0(3)
	bne	0, .Lrs_BB32_2
	blr
.Lrs_func_end32:
	.size	rs_k_prefix_f32, .Lrs_func_end32-.Lrs_func_begin32

	.section	.text.rs_k_saxpy,"ax",@progbits
	.globl	rs_k_saxpy
	.p2align	2
	.type	rs_k_saxpy,@function
rs_k_saxpy:
.Lrs_func_begin33:
	cmpwi	6, 1
	bltlr	0
	addi 4, 4, -4
	addi 3, 3, -4
.Lrs_BB33_2:
	lfsu 0, 4(4)
	addi 6, 6, -1
	lfsu 2, 4(3)
	cmplwi	6, 0
	fmuls 0, 1, 0
	fadds 0, 0, 2
	stfs 0, 0(3)
	bne	0, .Lrs_BB33_2
	blr
.Lrs_func_end33:
	.size	rs_k_saxpy, .Lrs_func_end33-.Lrs_func_begin33

	.section	.text.rs_k_saxpy_r,"ax",@progbits
	.globl	rs_k_saxpy_r
	.p2align	2
	.type	rs_k_saxpy_r,@function
rs_k_saxpy_r:
.Lrs_func_begin34:
	cmpwi	6, 1
	bltlr	0
	addi 4, 4, -4
	addi 3, 3, -4
.Lrs_BB34_2:
	lfsu 0, 4(4)
	addi 6, 6, -1
	lfsu 2, 4(3)
	cmplwi	6, 0
	fmuls 0, 1, 0
	fadds 0, 0, 2
	stfs 0, 0(3)
	bne	0, .Lrs_BB34_2
	blr
.Lrs_func_end34:
	.size	rs_k_saxpy_r, .Lrs_func_end34-.Lrs_func_begin34

	.section	.text.rs_k_scale_inplace,"ax",@progbits
	.globl	rs_k_scale_inplace
	.p2align	2
	.type	rs_k_scale_inplace,@function
rs_k_scale_inplace:
.Lrs_func_begin35:
	cmpwi	5, 1
	bltlr	0
	srawi 4, 5, 31
	andc 4, 5, 4
	slwi 4, 4, 2
	addi 6, 4, -4
	cmplwi	6, 0
	mr	5, 3
	beq	0, .Lrs_BB35_5
	srwi 5, 6, 2
	addi 6, 5, 1
	rlwinm 7, 6, 0, 1, 30
	rlwinm 5, 6, 2, 0, 28
	add 5, 3, 5
	addi 8, 3, -8
	mr	9, 7
.Lrs_BB35_3:
	lfsu 0, 8(8)
	addi 9, 9, -2
	cmplwi	9, 0
	lfs 2, 4(8)
	fmuls 0, 1, 0
	stfs 0, 0(8)
	fmuls 0, 1, 2
	stfs 0, 4(8)
	bne	0, .Lrs_BB35_3
	cmplw	6, 7
	beqlr	0
.Lrs_BB35_5:
	add 3, 3, 4
	addi 4, 5, -4
.Lrs_BB35_6:
	lfsu 0, 4(4)
	addi 5, 4, 4
	fmuls 0, 1, 0
	cmplw	5, 3
	stfs 0, 0(4)
	bne	0, .Lrs_BB35_6
	blr
.Lrs_func_end35:
	.size	rs_k_scale_inplace, .Lrs_func_end35-.Lrs_func_begin35

	.section	.text.rs_k_set_f32,"ax",@progbits
	.globl	rs_k_set_f32
	.p2align	2
	.type	rs_k_set_f32,@function
rs_k_set_f32:
.Lrs_func_begin36:
	cmpwi	5, 1
	bltlr	0
	li 4, 0
	ble 0, .Lrs_BB36_5
	rlwinm 4, 5, 0, 1, 30
	addi 6, 3, -8
	mr	7, 4
.Lrs_BB36_3:
	addi 7, 7, -2
	stfsu 1, 8(6)
	cmplwi	7, 0
	stfs 1, 4(6)
	bne	0, .Lrs_BB36_3
	cmplw	5, 4
	beqlr	0
.Lrs_BB36_5:
	slwi 6, 4, 2
	add 3, 6, 3
	sub	4, 5, 4
	addi 3, 3, -4
.Lrs_BB36_6:
	addi 4, 4, -1
	cmplwi	4, 0
	stfsu 1, 4(3)
	bne	0, .Lrs_BB36_6
	blr
.Lrs_func_end36:
	.size	rs_k_set_f32, .Lrs_func_end36-.Lrs_func_begin36

	.section	.text.rs_k_set_u32,"ax",@progbits
	.globl	rs_k_set_u32
	.p2align	2
	.type	rs_k_set_u32,@function
rs_k_set_u32:
.Lrs_func_begin37:
	cmpwi	5, 1
	bltlr	0
	li 6, 0
	ble 0, .Lrs_BB37_5
	rlwinm 6, 5, 0, 1, 30
	addi 7, 3, -8
	mr	8, 6
.Lrs_BB37_3:
	addi 8, 8, -2
	stwu 4, 8(7)
	cmplwi	8, 0
	stw 4, 4(7)
	bne	0, .Lrs_BB37_3
	cmplw	5, 6
	beqlr	0
.Lrs_BB37_5:
	slwi 7, 6, 2
	add 3, 7, 3
	sub	5, 5, 6
	addi 3, 3, -4
.Lrs_BB37_6:
	addi 5, 5, -1
	cmplwi	5, 0
	stwu 4, 4(3)
	bne	0, .Lrs_BB37_6
	blr
.Lrs_func_end37:
	.size	rs_k_set_u32, .Lrs_func_end37-.Lrs_func_begin37

	.section	.text.rs_k_shl_i32_r,"ax",@progbits
	.globl	rs_k_shl_i32_r
	.p2align	2
	.type	rs_k_shl_i32_r,@function
rs_k_shl_i32_r:
.Lrs_func_begin38:
	cmpwi	5, 1
	bltlr	0
	addi 4, 4, -4
	addi 3, 3, -4
.Lrs_BB38_2:
	lwzu 6, 4(4)
	addi 5, 5, -1
	cmplwi	5, 0
	slwi 6, 6, 3
	stwu 6, 4(3)
	bne	0, .Lrs_BB38_2
	blr
.Lrs_func_end38:
	.size	rs_k_shl_i32_r, .Lrs_func_end38-.Lrs_func_begin38

	.section	.text.rs_k_stride2_f32_r,"ax",@progbits
	.globl	rs_k_stride2_f32_r
	.p2align	2
	.type	rs_k_stride2_f32_r,@function
rs_k_stride2_f32_r:
.Lrs_func_begin39:
	srawi 6, 5, 31
	andc 5, 5, 6
	clrlwi.	5, 5, 2
	beqlr	0
	addi 4, 4, -8
	addi 3, 3, -4
.Lrs_BB39_2:
	lfsu 0, 8(4)
	addi 5, 5, -1
	cmplwi	5, 0
	lfs 1, 4(4)
	fadds 0, 0, 1
	stfsu 0, 4(3)
	bne	0, .Lrs_BB39_2
	blr
.Lrs_func_end39:
	.size	rs_k_stride2_f32_r, .Lrs_func_end39-.Lrs_func_begin39

	.section	.rodata.cst4,"aM",@progbits,4
	.p2align	2, 0x0
.Lrs_CPI40_0:
	.long	0x00000000
	.section	.text.rs_k_sum_f32,"ax",@progbits
	.globl	rs_k_sum_f32
	.p2align	2
	.type	rs_k_sum_f32,@function
rs_k_sum_f32:
.Lrs_func_begin40:
	cmpwi	4, 1
	lis 5, .Lrs_CPI40_0@ha
	blt	0, .Lrs_BB40_4
	lfs 1, .Lrs_CPI40_0@l(5)
	srawi 6, 4, 31
	andc 4, 4, 6
	addi 3, 3, -4
	slwi 4, 4, 2
.Lrs_BB40_2:
	lfsu 0, 4(3)
	addi 4, 4, -4
	cmplwi	4, 0
	fadds 1, 1, 0
	bne	0, .Lrs_BB40_2
	blr
.Lrs_BB40_4:
	lfs 1, .Lrs_CPI40_0@l(5)
	blr
.Lrs_func_end40:
	.size	rs_k_sum_f32, .Lrs_func_end40-.Lrs_func_begin40

	.section	.text.rs_k_sum_i32,"ax",@progbits
	.globl	rs_k_sum_i32
	.p2align	2
	.type	rs_k_sum_i32,@function
rs_k_sum_i32:
.Lrs_func_begin41:
	cmpwi	4, 1
	blt	0, .Lrs_BB41_6
	srawi 5, 4, 31
	andc 4, 4, 5
	slwi 5, 4, 2
	addi 4, 5, -4
	cmplwi	4, 0
	beq	0, .Lrs_BB41_7
	srwi 6, 4, 2
	addi 7, 6, 1
	rlwinm 9, 7, 0, 1, 30
	rlwinm 6, 7, 2, 0, 28
	addi 8, 3, -8
	li 4, 0
	add 6, 3, 6
	mr	11, 9
	li 10, 0
.Lrs_BB41_3:
	lwzu 12, 8(8)
	addi 11, 11, -2
	cmplwi	11, 0
	lwz 0, 4(8)
	add 4, 12, 4
	add 10, 0, 10
	bne	0, .Lrs_BB41_3
	cmplw	7, 9
	add 4, 10, 4
	bne	0, .Lrs_BB41_8
.Lrs_BB41_5:
	mr	3, 4
	blr
.Lrs_BB41_6:
	li 4, 0
	mr	3, 4
	blr
.Lrs_BB41_7:
	li 4, 0
	mr	6, 3
.Lrs_BB41_8:
	add 3, 3, 5
	addi 5, 6, -4
.Lrs_BB41_9:
	lwzu 6, 4(5)
	addi 7, 5, 4
	cmplw	7, 3
	add 4, 6, 4
	bne	0, .Lrs_BB41_9
	b .Lrs_BB41_5
.Lrs_func_end41:
	.size	rs_k_sum_i32, .Lrs_func_end41-.Lrs_func_begin41

	.section	.rodata.cst4,"aM",@progbits,4
	.p2align	2, 0x0
.Lrs_CPI42_0:
	.long	0x3b808081
	.section	.text.rs_k_u8_to_f32_r,"ax",@progbits
	.globl	rs_k_u8_to_f32_r
	.p2align	2
	.type	rs_k_u8_to_f32_r,@function
rs_k_u8_to_f32_r:
.Lrs_func_begin42:
	cmpwi	5, 1
	bltlr	0
	stwu 1, -32(1)
	lis 6, .Lrs_CPI42_0@ha
	lfs 0, .Lrs_CPI42_0@l(6)
	addi 4, 4, -1
	addi 3, 3, -4
.Lrs_BB42_2:
	lbzu 6, 1(4)
	addi 5, 5, -1
	cmplwi	5, 0
	std 6, 24(1)
	lfd 1, 24(1)
	fcfid 1, 1
	frsp 1, 1
	fmuls 1, 1, 0
	stfsu 1, 4(3)
	bne	0, .Lrs_BB42_2
	addi 1, 1, 32
	blr
.Lrs_func_end42:
	.size	rs_k_u8_to_f32_r, .Lrs_func_end42-.Lrs_func_begin42

	.section	.text.rs_k_zero_u8,"ax",@progbits
	.globl	rs_k_zero_u8
	.p2align	2
	.type	rs_k_zero_u8,@function
rs_k_zero_u8:
.Lrs_func_begin43:
	cmpwi	4, 1
	bltlr	0
	mflr 0
	stw 0, -8(1)
	stwu 1, -96(1)
	mr	5, 4
	li 4, 0
	bl memset
	addi 1, 1, 96
	lwz 0, -8(1)
	mtlr 0
	blr
.Lrs_func_end43:
	.size	rs_k_zero_u8, .Lrs_func_end43-.Lrs_func_begin43

	.section	.text.rs_sp_bright_s,"ax",@progbits
	.globl	rs_sp_bright_s
	.p2align	2
	.type	rs_sp_bright_s,@function
rs_sp_bright_s:
.Lrs_func_begin44:
	cmpwi	5, 1
	bltlr	0
	addi 4, 4, -1
	addi 3, 3, -1
	li 6, 0
	b .Lrs_BB44_3
.Lrs_BB44_2:
	addi 6, 6, 1
	cmplw	5, 6
	stbu 7, 1(3)
	beqlr	0
.Lrs_BB44_3:
	lbzu 7, 1(4)
	addi 7, 7, 40
	cmplwi	7, 255
	blt	0, .Lrs_BB44_2
	li 7, 255
	b .Lrs_BB44_2
.Lrs_func_end44:
	.size	rs_sp_bright_s, .Lrs_func_end44-.Lrs_func_begin44

	.section	.text.rs_sp_bright_v,"ax",@progbits
	.globl	rs_sp_bright_v
	.p2align	2
	.type	rs_sp_bright_v,@function
rs_sp_bright_v:
.Lrs_func_begin45:
	srawi 6, 5, 31
	andc 5, 5, 6
	rlwinm. 6, 5, 28, 4, 31
	beqlr	0
	stwu 1, -112(1)
	rlwinm 5, 5, 0, 0, 27
	addi 4, 4, -16
	addi 3, 3, -16
	li 6, 0
	std 22, 24(1)
	std 23, 32(1)
	std 24, 40(1)
	std 25, 48(1)
	std 26, 56(1)
	std 27, 64(1)
	std 28, 72(1)
	std 29, 80(1)
	std 30, 88(1)
	b .Lrs_BB45_3
.Lrs_BB45_2:
	addi 6, 6, 16
	cmplw	5, 6
	stb 9, 15(3)
	stb 11, 14(3)
	stb 30, 13(3)
	stb 28, 12(3)
	stb 26, 11(3)
	stb 24, 10(3)
	stb 22, 9(3)
	stb 23, 8(3)
	stb 25, 7(3)
	stb 27, 6(3)
	stb 29, 5(3)
	stb 12, 4(3)
	stb 10, 3(3)
	stb 8, 2(3)
	stb 7, 1(3)
	beq	0, .Lrs_BB45_35
.Lrs_BB45_3:
	lbzu 7, 16(4)
	addi 7, 7, 40
	cmplwi	7, 255
	blt	0, .Lrs_BB45_5
	li 7, 255
.Lrs_BB45_5:
	stbu 7, 16(3)
	lbz 7, 1(4)
	addi 7, 7, 40
	cmplwi	7, 255
	blt	0, .Lrs_BB45_7
	li 7, 255
.Lrs_BB45_7:
	lbz 8, 2(4)
	lbz 9, 15(4)
	lbz 11, 14(4)
	addi 8, 8, 40
	lbz 30, 13(4)
	cmplwi	8, 255
	lbz 28, 12(4)
	lbz 26, 11(4)
	lbz 24, 10(4)
	lbz 22, 9(4)
	lbz 23, 8(4)
	lbz 25, 7(4)
	lbz 27, 6(4)
	lbz 29, 5(4)
	lbz 12, 4(4)
	lbz 10, 3(4)
	bge 0, .Lrs_BB45_21
	addi 10, 10, 40
	cmplwi	10, 255
	bge 0, .Lrs_BB45_22
.Lrs_BB45_9:
	addi 12, 12, 40
	cmplwi	12, 255
	bge 0, .Lrs_BB45_23
.Lrs_BB45_10:
	addi 29, 29, 40
	cmplwi	29, 255
	bge 0, .Lrs_BB45_24
.Lrs_BB45_11:
	addi 27, 27, 40
	cmplwi	27, 255
	bge 0, .Lrs_BB45_25
.Lrs_BB45_12:
	addi 25, 25, 40
	cmplwi	25, 255
	bge 0, .Lrs_BB45_26
.Lrs_BB45_13:
	addi 23, 23, 40
	cmplwi	23, 255
	bge 0, .Lrs_BB45_27
.Lrs_BB45_14:
	addi 22, 22, 40
	cmplwi	22, 255
	bge 0, .Lrs_BB45_28
.Lrs_BB45_15:
	addi 24, 24, 40
	cmplwi	24, 255
	bge 0, .Lrs_BB45_29
.Lrs_BB45_16:
	addi 26, 26, 40
	cmplwi	26, 255
	bge 0, .Lrs_BB45_30
.Lrs_BB45_17:
	addi 28, 28, 40
	cmplwi	28, 255
	bge 0, .Lrs_BB45_31
.Lrs_BB45_18:
	addi 30, 30, 40
	cmplwi	30, 255
	bge 0, .Lrs_BB45_32
.Lrs_BB45_19:
	addi 11, 11, 40
	cmplwi	11, 255
	bge 0, .Lrs_BB45_33
.Lrs_BB45_20:
	addi 9, 9, 40
	cmplwi	9, 255
	blt	0, .Lrs_BB45_2
	b .Lrs_BB45_34
.Lrs_BB45_21:
	li 8, 255
	addi 10, 10, 40
	cmplwi	10, 255
	blt	0, .Lrs_BB45_9
.Lrs_BB45_22:
	li 10, 255
	addi 12, 12, 40
	cmplwi	12, 255
	blt	0, .Lrs_BB45_10
.Lrs_BB45_23:
	li 12, 255
	addi 29, 29, 40
	cmplwi	29, 255
	blt	0, .Lrs_BB45_11
.Lrs_BB45_24:
	li 29, 255
	addi 27, 27, 40
	cmplwi	27, 255
	blt	0, .Lrs_BB45_12
.Lrs_BB45_25:
	li 27, 255
	addi 25, 25, 40
	cmplwi	25, 255
	blt	0, .Lrs_BB45_13
.Lrs_BB45_26:
	li 25, 255
	addi 23, 23, 40
	cmplwi	23, 255
	blt	0, .Lrs_BB45_14
.Lrs_BB45_27:
	li 23, 255
	addi 22, 22, 40
	cmplwi	22, 255
	blt	0, .Lrs_BB45_15
.Lrs_BB45_28:
	li 22, 255
	addi 24, 24, 40
	cmplwi	24, 255
	blt	0, .Lrs_BB45_16
.Lrs_BB45_29:
	li 24, 255
	addi 26, 26, 40
	cmplwi	26, 255
	blt	0, .Lrs_BB45_17
.Lrs_BB45_30:
	li 26, 255
	addi 28, 28, 40
	cmplwi	28, 255
	blt	0, .Lrs_BB45_18
.Lrs_BB45_31:
	li 28, 255
	addi 30, 30, 40
	cmplwi	30, 255
	blt	0, .Lrs_BB45_19
.Lrs_BB45_32:
	li 30, 255
	addi 11, 11, 40
	cmplwi	11, 255
	blt	0, .Lrs_BB45_20
.Lrs_BB45_33:
	li 11, 255
	addi 9, 9, 40
	cmplwi	9, 255
	blt	0, .Lrs_BB45_2
.Lrs_BB45_34:
	li 9, 255
	b .Lrs_BB45_2
.Lrs_BB45_35:
	ld 30, 88(1)
	ld 29, 80(1)
	ld 28, 72(1)
	ld 27, 64(1)
	ld 26, 56(1)
	ld 25, 48(1)
	ld 24, 40(1)
	ld 23, 32(1)
	ld 22, 24(1)
	addi 1, 1, 112
	blr
.Lrs_func_end45:
	.size	rs_sp_bright_v, .Lrs_func_end45-.Lrs_func_begin45

	.section	.text.rs_sp_copy_v,"ax",@progbits
	.globl	rs_sp_copy_v
	.p2align	2
	.type	rs_sp_copy_v,@function
rs_sp_copy_v:
.Lrs_func_begin46:
	srawi 6, 5, 31
	andc 5, 5, 6
	rlwinm. 5, 5, 30, 2, 31
	beqlr	0
	addi 4, 4, -16
	addi 3, 3, -16
.Lrs_BB46_2:
	ldu 6, 16(4)
	addi 5, 5, -1
	cmplwi	5, 0
	stdu 6, 16(3)
	lwz 6, 12(4)
	lwz 7, 8(4)
	stw 6, 12(3)
	stw 7, 8(3)
	bne	0, .Lrs_BB46_2
	blr
.Lrs_func_end46:
	.size	rs_sp_copy_v, .Lrs_func_end46-.Lrs_func_begin46

	.section	.rodata.cst4,"aM",@progbits,4
	.p2align	2, 0x0
.Lrs_CPI47_0:
	.long	0x00000000
	.section	.text.rs_sp_dot_s,"ax",@progbits
	.globl	rs_sp_dot_s
	.p2align	2
	.type	rs_sp_dot_s,@function
rs_sp_dot_s:
.Lrs_func_begin47:
	srawi 6, 5, 31
	andc 5, 5, 6
	clrlwi.	5, 5, 2
	lis 6, .Lrs_CPI47_0@ha
	beq	0, .Lrs_BB47_4
	lfs 1, .Lrs_CPI47_0@l(6)
	addi 3, 3, -4
	addi 4, 4, -4
.Lrs_BB47_2:
	lfsu 0, 4(4)
	addi 5, 5, -1
	lfsu 2, 4(3)
	cmplwi	5, 0
	fmuls 0, 2, 0
	fadds 1, 1, 0
	bne	0, .Lrs_BB47_2
	blr
.Lrs_BB47_4:
	lfs 1, .Lrs_CPI47_0@l(6)
	blr
.Lrs_func_end47:
	.size	rs_sp_dot_s, .Lrs_func_end47-.Lrs_func_begin47

	.section	.rodata.cst4,"aM",@progbits,4
	.p2align	2, 0x0
.Lrs_CPI48_0:
	.long	0x00000000
	.section	.text.rs_sp_dot_v,"ax",@progbits
	.globl	rs_sp_dot_v
	.p2align	2
	.type	rs_sp_dot_v,@function
rs_sp_dot_v:
.Lrs_func_begin48:
	srawi 6, 5, 31
	andc 5, 5, 6
	rlwinm. 5, 5, 28, 4, 31
	lis 6, .Lrs_CPI48_0@ha
	beq	0, .Lrs_BB48_4
	stwu 1, -192(1)
	lfs 1, .Lrs_CPI48_0@l(6)
	addi 3, 3, -64
	stfd 27, 144(1)
	addi 4, 4, -64
	stfd 28, 152(1)
	fmr 28, 1
	stfd 29, 160(1)
	fmr 29, 1
	stfd 30, 168(1)
	fmr 27, 1
	stfd 31, 176(1)
	fmr 5, 1
	fmr 4, 1
	fmr 6, 1
	fmr 7, 1
	fmr 8, 1
	fmr 9, 1
	fmr 10, 1
	fmr 11, 1
	fmr 13, 1
	fmr 12, 1
	fmr 31, 1
	fmr 30, 1
	stfd 14, 40(1)
	stfd 15, 48(1)
	stfd 16, 56(1)
	stfd 17, 64(1)
	stfd 18, 72(1)
	stfd 19, 80(1)
	stfd 20, 88(1)
	stfd 21, 96(1)
	stfd 22, 104(1)
	stfd 23, 112(1)
	stfd 24, 120(1)
	stfd 25, 128(1)
	stfd 26, 136(1)
.Lrs_BB48_2:
	stfs 28, 32(1)
	addi 5, 5, -1
	lfsu 28, 64(4)
	cmplwi	5, 0
	stfs 27, 28(1)
	stfs 29, 36(1)
	lfsu 29, 64(3)
	lfs 27, 28(4)
	lfs 2, 12(3)
	fmuls 29, 29, 28
	stfs 27, 24(1)
	fadds 12, 12, 29
	lfs 27, 44(3)
	lfs 26, 8(3)
	lfs 25, 4(3)
	lfs 24, 12(4)
	lfs 23, 8(4)
	lfs 22, 4(4)
	fmuls 24, 2, 24
	lfs 3, 24(3)
	fmuls 26, 26, 23
	lfs 16, 24(4)
	fmuls 25, 25, 22
	stfs 27, 20(1)
	fadds 30, 30, 24
	lfs 0, 28(3)
	fmuls 16, 3, 16
	lfs 19, 20(3)
	fadds 31, 31, 26
	lfs 18, 16(3)
	fadds 1, 1, 25
	lfs 15, 20(4)
	fadds 11, 11, 16
	lfs 14, 16(4)
	lfs 21, 40(3)
	fmuls 19, 19, 15
	lfs 27, 36(3)
	fmuls 18, 18, 14
	lfs 28, 32(3)
	fadds 10, 10, 19
	lfs 29, 44(4)
	fadds 9, 9, 18
	lfs 22, 40(4)
	lfs 23, 36(4)
	lfs 20, 32(4)
	fmuls 22, 21, 22
	lfs 2, 24(1)
	fmuls 27, 27, 23
	lfs 3, 20(1)
	fmuls 28, 28, 20
	lfs 14, 60(3)
	fmuls 2, 0, 2
	lfs 15, 56(3)
	fmuls 29, 3, 29
	lfs 17, 52(3)
	fadds 8, 8, 29
	lfs 0, 48(3)
	fadds 6, 6, 27
	lfs 20, 56(4)
	fadds 4, 4, 28
	lfs 23, 48(4)
	fadds 13, 13, 2
	lfs 21, 52(4)
	fadds 7, 7, 22
	lfs 3, 60(4)
	fmuls 0, 0, 23
	lfs 29, 36(1)
	fmuls 23, 17, 21
	lfs 27, 28(1)
	fmuls 21, 15, 20
	lfs 28, 32(1)
	fmuls 3, 14, 3
	fadds 5, 5, 3
	fadds 27, 27, 21
	fadds 28, 28, 23
	fadds 29, 29, 0
	bne	0, .Lrs_BB48_2
	fadds 3, 30, 13
	fadds 2, 31, 11
	fadds 7, 7, 27
	fadds 6, 6, 28
	fadds 4, 4, 29
	lfd 31, 176(1)
	fadds 1, 1, 10
	lfd 30, 168(1)
	fadds 0, 12, 9
	lfd 29, 160(1)
	fadds 5, 8, 5
	lfd 28, 152(1)
	fadds 0, 0, 4
	lfd 27, 144(1)
	fadds 1, 1, 6
	lfd 26, 136(1)
	fadds 2, 2, 7
	lfd 25, 128(1)
	fadds 3, 3, 5
	lfd 24, 120(1)
	lfd 23, 112(1)
	lfd 22, 104(1)
	lfd 21, 96(1)
	lfd 20, 88(1)
	lfd 19, 80(1)
	lfd 18, 72(1)
	lfd 17, 64(1)
	lfd 16, 56(1)
	lfd 15, 48(1)
	lfd 14, 40(1)
	addi 1, 1, 192
	fadds 0, 0, 1
	fadds 1, 2, 3
	fadds 1, 0, 1
	blr
.Lrs_BB48_4:
	lfs 0, .Lrs_CPI48_0@l(6)
	fmr 1, 0
	fmr 2, 0
	fmr 3, 0
	fadds 0, 0, 1
	fadds 1, 2, 3
	fadds 1, 0, 1
	blr
.Lrs_func_end48:
	.size	rs_sp_dot_v, .Lrs_func_end48-.Lrs_func_begin48

	.section	.rodata.cst4,"aM",@progbits,4
	.p2align	2, 0x0
.Lrs_CPI49_0:
	.long	0x437f0000
.Lrs_CPI49_1:
	.long	0x3f000000
.Lrs_CPI49_2:
	.long	0x00000000
.Lrs_CPI49_3:
	.long	0x4f7fffff
	.section	.text.rs_sp_f2u8_s,"ax",@progbits
	.globl	rs_sp_f2u8_s
	.p2align	2
	.type	rs_sp_f2u8_s,@function
rs_sp_f2u8_s:
.Lrs_func_begin49:
	cmpwi	5, 1
	bltlr	0
	stwu 1, -32(1)
	lis 6, .Lrs_CPI49_0@ha
	lfs 0, .Lrs_CPI49_0@l(6)
	lis 6, .Lrs_CPI49_1@ha
	lfs 1, .Lrs_CPI49_1@l(6)
	lis 6, .Lrs_CPI49_2@ha
	lfs 2, .Lrs_CPI49_2@l(6)
	lis 6, .Lrs_CPI49_3@ha
	lfs 3, .Lrs_CPI49_3@l(6)
	addi 4, 4, -4
	addi 3, 3, -1
	b .Lrs_BB49_3
.Lrs_BB49_2:
	addi 5, 5, -1
	cmplwi	5, 0
	stbu 7, 1(3)
	beq	0, .Lrs_BB49_11
.Lrs_BB49_3:
	lfsu 4, 4(4)
	fmr 5, 2
	fmuls 4, 4, 0
	fadds 4, 4, 1
	fcmpu 0, 4, 2
	blt	0, .Lrs_BB49_5
	fmr 5, 4
.Lrs_BB49_5:
	fcmpu 0, 5, 0
	fmr 4, 0
	bgt	0, .Lrs_BB49_7
	fmr 4, 5
.Lrs_BB49_7:
	fcmpu 0, 4, 2
	fctidz 5, 4
	cror 20, 0, 3
	li 6, 0
	stfd 5, 24(1)
	bc 4, 20, .Lrs_BB49_9
	fcmpu 0, 4, 3
	li 7, -1
	bgt	0, .Lrs_BB49_2
	b .Lrs_BB49_10
.Lrs_BB49_9:
	lwz 6, 28(1)
	fcmpu 0, 4, 3
	li 7, -1
	bgt	0, .Lrs_BB49_2
.Lrs_BB49_10:
	mr	7, 6
	b .Lrs_BB49_2
.Lrs_BB49_11:
	addi 1, 1, 32
	blr
.Lrs_func_end49:
	.size	rs_sp_f2u8_s, .Lrs_func_end49-.Lrs_func_begin49

	.section	.rodata.cst4,"aM",@progbits,4
	.p2align	2, 0x0
.Lrs_CPI50_0:
	.long	0x437f0000
.Lrs_CPI50_1:
	.long	0x3f000000
.Lrs_CPI50_2:
	.long	0x00000000
.Lrs_CPI50_3:
	.long	0x4f7fffff
	.section	.text.rs_sp_f2u8_v,"ax",@progbits
	.globl	rs_sp_f2u8_v
	.p2align	2
	.type	rs_sp_f2u8_v,@function
rs_sp_f2u8_v:
.Lrs_func_begin50:
	rlwinm. 6, 5, 28, 4, 31
	cmpwi 1, 5, 1
	cror 20, 4, 2
	bclr 12, 20, 0
	stwu 1, -256(1)
	lis 11, .Lrs_CPI50_0@ha
	lfs 0, .Lrs_CPI50_0@l(11)
	lis 11, .Lrs_CPI50_1@ha
	lfs 1, .Lrs_CPI50_1@l(11)
	lis 11, .Lrs_CPI50_2@ha
	lfs 2, .Lrs_CPI50_2@l(11)
	lis 11, .Lrs_CPI50_3@ha
	lfs 3, .Lrs_CPI50_3@l(11)
	addi 5, 1, 64
	addi 7, 1, 80
	addi 8, 1, 96
	li 10, 0
	addi 9, 1, 32
	mr	11, 4
	std 15, 112(1)
	std 16, 120(1)
	std 17, 128(1)
	std 18, 136(1)
	std 19, 144(1)
	std 20, 152(1)
	std 21, 160(1)
	std 22, 168(1)
	std 23, 176(1)
	std 24, 184(1)
	std 25, 192(1)
	std 26, 200(1)
	std 27, 208(1)
	std 28, 216(1)
	std 29, 224(1)
	std 30, 232(1)
	b .Lrs_BB50_3
.Lrs_BB50_2:
	lbz 29, 51(1)
	slwi 16, 10, 4
	lbz 12, 55(1)
	mr	15, 3
	lbz 0, 59(1)
	addi 10, 10, 1
	lbz 30, 63(1)
	cmplw	10, 6
	lbz 28, 3(5)
	addi 11, 11, 64
	lbz 27, 7(5)
	lbz 26, 11(5)
	lbz 25, 15(5)
	lbz 24, 3(7)
	lbz 23, 7(7)
	lbz 22, 11(7)
	lbz 21, 15(7)
	lbz 20, 3(8)
	lbz 19, 7(8)
	lbz 18, 11(8)
	lbz 17, 15(8)
	stbux 29, 15, 16
	stb 17, 15(15)
	stb 18, 14(15)
	stb 19, 13(15)
	stb 20, 12(15)
	stb 21, 11(15)
	stb 22, 10(15)
	stb 23, 9(15)
	stb 24, 8(15)
	stb 25, 7(15)
	stb 26, 6(15)
	stb 27, 5(15)
	stb 28, 4(15)
	stb 30, 3(15)
	stb 0, 2(15)
	stb 12, 1(15)
	beq	0, .Lrs_BB50_37
.Lrs_BB50_3:
	slwi 12, 10, 6
	add 12, 12, 4
	addi 12, 12, -16
	li 30, 0
	mr	29, 9
	b .Lrs_BB50_5
.Lrs_BB50_4:
	addi 30, 30, 16
	cmplwi	30, 64
	stw 27, 12(29)
	stw 25, 8(29)
	stw 26, 4(29)
	beq	0, .Lrs_BB50_2
.Lrs_BB50_5:
	lfsu 4, 16(12)
	fmr 5, 2
	fmuls 4, 4, 0
	fadds 4, 4, 1
	fcmpu 0, 4, 2
	blt	0, .Lrs_BB50_7
	fmr 5, 4
.Lrs_BB50_7:
	fcmpu 0, 5, 0
	fmr 4, 0
	bgt	0, .Lrs_BB50_9
	fmr 4, 5
.Lrs_BB50_9:
	fcmpu 0, 4, 2
	fctidz 5, 4
	cror 20, 0, 3
	li 28, 0
	li 26, 0
	stfd 5, 16(1)
	bc 12, 20, .Lrs_BB50_11
	lwz 26, 20(1)
.Lrs_BB50_11:
	fcmpu 0, 4, 3
	li 27, -1
	li 25, -1
	bgt	0, .Lrs_BB50_13
	mr	25, 26
.Lrs_BB50_13:
	lfs 5, 4(12)
	fmr 6, 2
	lfs 4, 12(12)
	fmuls 5, 5, 0
	fadds 7, 5, 1
	lfs 5, 8(12)
	fcmpu 0, 7, 2
	stwu 25, 16(29)
	blt	0, .Lrs_BB50_15
	fmr 6, 7
.Lrs_BB50_15:
	fmuls 5, 5, 0
	fadds 5, 5, 1
	fcmpu 0, 5, 2
	fmr 7, 2
	blt	0, .Lrs_BB50_17
	fmr 7, 5
.Lrs_BB50_17:
	fmuls 4, 4, 0
	fadds 4, 4, 1
	fcmpu 0, 4, 2
	fmr 5, 2
	bge 0, .Lrs_BB50_28
	fcmpu 0, 5, 0
	fmr 4, 0
	ble 0, .Lrs_BB50_29
.Lrs_BB50_19:
	fcmpu 0, 7, 0
	fmr 5, 0
	ble 0, .Lrs_BB50_30
.Lrs_BB50_20:
	fcmpu 0, 6, 0
	fmr 7, 0
	bgt	0, .Lrs_BB50_22
.Lrs_BB50_21:
	fmr 7, 6
.Lrs_BB50_22:
	fctidz 6, 7
	stfd 6, 24(1)
	fctidz 6, 5
	fcmpu 0, 7, 2
	stfd 6, 40(1)
	fctidz 6, 4
	cror 20, 0, 3
	li 25, 0
	stfd 6, 32(1)
	bc 4, 20, .Lrs_BB50_31
	fcmpu 0, 7, 3
	li 26, -1
	ble 0, .Lrs_BB50_32
.Lrs_BB50_24:
	fcmpu 0, 5, 2
	cror 20, 0, 3
	li 24, 0
	bc 4, 20, .Lrs_BB50_33
.Lrs_BB50_25:
	fcmpu 0, 5, 3
	li 25, -1
	ble 0, .Lrs_BB50_34
.Lrs_BB50_26:
	fcmpu 0, 4, 2
	cror 20, 0, 3
	bc 4, 20, .Lrs_BB50_35
.Lrs_BB50_27:
	fcmpu 0, 4, 3
	bgt	0, .Lrs_BB50_4
	b .Lrs_BB50_36
.Lrs_BB50_28:
	fmr 5, 4
	fcmpu 0, 5, 0
	fmr 4, 0
	bgt	0, .Lrs_BB50_19
.Lrs_BB50_29:
	fmr 4, 5
	fcmpu 0, 7, 0
	fmr 5, 0
	bgt	0, .Lrs_BB50_20
.Lrs_BB50_30:
	fmr 5, 7
	fcmpu 0, 6, 0
	fmr 7, 0
	ble 0, .Lrs_BB50_21
	b .Lrs_BB50_22
.Lrs_BB50_31:
	lwz 25, 28(1)
	fcmpu 0, 7, 3
	li 26, -1
	bgt	0, .Lrs_BB50_24
.Lrs_BB50_32:
	mr	26, 25
	fcmpu 0, 5, 2
	cror 20, 0, 3
	li 24, 0
	bc 12, 20, .Lrs_BB50_25
.Lrs_BB50_33:
	lwz 24, 44(1)
	fcmpu 0, 5, 3
	li 25, -1
	bgt	0, .Lrs_BB50_26
.Lrs_BB50_34:
	mr	25, 24
	fcmpu 0, 4, 2
	cror 20, 0, 3
	bc 12, 20, .Lrs_BB50_27
.Lrs_BB50_35:
	lwz 28, 36(1)
	fcmpu 0, 4, 3
	bgt	0, .Lrs_BB50_4
.Lrs_BB50_36:
	mr	27, 28
	b .Lrs_BB50_4
.Lrs_BB50_37:
	ld 30, 232(1)
	ld 29, 224(1)
	ld 28, 216(1)
	ld 27, 208(1)
	ld 26, 200(1)
	ld 25, 192(1)
	ld 24, 184(1)
	ld 23, 176(1)
	ld 22, 168(1)
	ld 21, 160(1)
	ld 20, 152(1)
	ld 19, 144(1)
	ld 18, 136(1)
	ld 17, 128(1)
	ld 16, 120(1)
	ld 15, 112(1)
	addi 1, 1, 256
	blr
.Lrs_func_end50:
	.size	rs_sp_f2u8_v, .Lrs_func_end50-.Lrs_func_begin50

	.section	.text.rs_sp_mat4_v,"ax",@progbits
	.globl	rs_sp_mat4_v
	.p2align	2
	.type	rs_sp_mat4_v,@function
rs_sp_mat4_v:
.Lrs_func_begin51:
	cmpwi	6, 1
	bltlr	0
	stwu 1, -112(1)
	stfd 30, 88(1)
	addi 3, 3, -16
	stfd 31, 96(1)
	lfs 0, 12(4)
	lfs 1, 8(4)
	lfs 2, 4(4)
	lfs 3, 0(4)
	lfs 4, 28(4)
	lfs 5, 24(4)
	lfs 6, 20(4)
	lfs 7, 16(4)
	lfs 8, 44(4)
	lfs 9, 40(4)
	lfs 10, 36(4)
	lfs 11, 32(4)
	lfs 12, 60(4)
	lfs 13, 56(4)
	lfs 31, 52(4)
	lfs 30, 48(4)
	addi 4, 5, -16
	stfd 22, 24(1)
	stfd 23, 32(1)
	stfd 24, 40(1)
	stfd 25, 48(1)
	stfd 26, 56(1)
	stfd 27, 64(1)
	stfd 28, 72(1)
	stfd 29, 80(1)
.Lrs_BB51_2:
	lfsu 28, 16(4)
	addi 6, 6, -1
	cmplwi	6, 0
	lfs 26, 4(4)
	fmuls 25, 3, 28
	lfs 27, 8(4)
	fmuls 24, 0, 28
	fmuls 22, 7, 26
	lfs 29, 12(4)
	fmuls 23, 1, 28
	fmuls 28, 2, 28
	fadds 25, 25, 22
	fmuls 22, 6, 26
	fadds 28, 28, 22
	fmuls 22, 5, 26
	fadds 23, 23, 22
	fmuls 22, 11, 27
	fmuls 26, 4, 26
	fadds 26, 24, 26
	fadds 25, 22, 25
	fmuls 22, 8, 27
	fmuls 24, 30, 29
	fadds 26, 22, 26
	fmuls 22, 10, 27
	fmuls 27, 9, 27
	fadds 27, 27, 23
	fmuls 23, 12, 29
	fadds 28, 22, 28
	fmuls 22, 13, 29
	fmuls 29, 31, 29
	fadds 25, 24, 25
	stfsu 25, 16(3)
	fadds 26, 23, 26
	fadds 27, 22, 27
	fadds 29, 29, 28
	stfs 26, 12(3)
	stfs 27, 8(3)
	stfs 29, 4(3)
	bne	0, .Lrs_BB51_2
	lfd 31, 96(1)
	lfd 30, 88(1)
	lfd 29, 80(1)
	lfd 28, 72(1)
	lfd 27, 64(1)
	lfd 26, 56(1)
	lfd 25, 48(1)
	lfd 24, 40(1)
	lfd 23, 32(1)
	lfd 22, 24(1)
	addi 1, 1, 112
	blr
.Lrs_func_end51:
	.size	rs_sp_mat4_v, .Lrs_func_end51-.Lrs_func_begin51

	.section	.text.rs_sp_mat4_v16,"ax",@progbits
	.globl	rs_sp_mat4_v16
	.p2align	2
	.type	rs_sp_mat4_v16,@function
rs_sp_mat4_v16:
.Lrs_func_begin52:
	mflr 0
	stw 0, -8(1)
	stwu 1, -1328(1)
	std 30, 1304(1)
	mr	30, 5
	srawi 5, 6, 31
	andc 5, 6, 5
	std 27, 1280(1)
	rlwinm. 27, 5, 28, 4, 31
	std 21, 1232(1)
	std 22, 1240(1)
	std 23, 1248(1)
	std 24, 1256(1)
	std 25, 1264(1)
	std 26, 1272(1)
	std 28, 1288(1)
	std 29, 1296(1)
	stfd 16, 1104(1)
	stfd 17, 1112(1)
	stfd 18, 1120(1)
	stfd 19, 1128(1)
	stfd 20, 1136(1)
	stfd 21, 1144(1)
	stfd 22, 1152(1)
	stfd 23, 1160(1)
	stfd 24, 1168(1)
	stfd 25, 1176(1)
	stfd 26, 1184(1)
	stfd 27, 1192(1)
	stfd 28, 1200(1)
	stfd 29, 1208(1)
	stfd 30, 1216(1)
	stfd 31, 1224(1)
	beq	0, .Lrs_BB52_7
	lfs 31, 12(4)
	mr	29, 3
	lfs 30, 8(4)
	li 26, 0
	lfs 29, 4(4)
	addi 25, 1, 16
	lfs 28, 0(4)
	addi 28, 1, 80
	lfs 27, 28(4)
	mr	24, 3
	lfs 26, 24(4)
	mr	23, 30
	lfs 25, 20(4)
	lfs 24, 16(4)
	lfs 23, 44(4)
	lfs 22, 40(4)
	lfs 21, 36(4)
	lfs 20, 32(4)
	lfs 19, 60(4)
	lfs 18, 56(4)
	lfs 17, 52(4)
	lfs 16, 48(4)
.Lrs_BB52_2:
	slwi 3, 26, 8
	addi 3, 3, -16
	add 22, 29, 3
	add 21, 30, 3
	mr	3, 28
	li 4, 0
	li 5, 1024
	bl memset
	li 3, 0
	mr	4, 25
.Lrs_BB52_3:
	lfsu 0, 16(21)
	addi 3, 3, 64
	cmplwi	3, 1024
	lfs 1, 12(21)
	lfs 2, 8(21)
	lfs 3, 4(21)
	stfsu 0, 64(4)
	stfs 0, 12(4)
	stfs 0, 8(4)
	stfs 0, 4(4)
	stfs 3, 28(4)
	stfs 3, 24(4)
	stfs 3, 20(4)
	stfs 3, 16(4)
	stfs 2, 44(4)
	stfs 2, 40(4)
	stfs 2, 36(4)
	stfs 2, 32(4)
	stfs 1, 60(4)
	stfs 1, 56(4)
	stfs 1, 52(4)
	stfs 1, 48(4)
	bne	0, .Lrs_BB52_3
	li 3, 0
	mr	4, 25
.Lrs_BB52_5:
	lfsu 0, 64(4)
	addi 3, 3, 64
	cmplwi	3, 1024
	lfs 1, 16(4)
	fmuls 0, 28, 0
	lfs 2, 32(4)
	lfs 3, 48(4)
	fmuls 1, 24, 1
	fadds 0, 0, 1
	fmuls 2, 20, 2
	lfs 4, 4(4)
	fadds 0, 0, 2
	lfs 5, 8(4)
	fmuls 3, 16, 3
	lfs 2, 24(4)
	fadds 0, 0, 3
	lfs 1, 20(4)
	fmuls 3, 29, 4
	lfs 6, 12(4)
	fmuls 4, 30, 5
	stfsu 0, 16(22)
	fmuls 2, 26, 2
	lfs 0, 28(4)
	fmuls 1, 25, 1
	fadds 2, 4, 2
	lfs 4, 40(4)
	fadds 1, 3, 1
	lfs 3, 44(4)
	fmuls 5, 31, 6
	fmuls 0, 27, 0
	fadds 0, 5, 0
	lfs 5, 36(4)
	fmuls 4, 22, 4
	fmuls 3, 23, 3
	fadds 2, 2, 4
	lfs 4, 60(4)
	fadds 0, 0, 3
	lfs 3, 56(4)
	fmuls 5, 21, 5
	fadds 1, 1, 5
	lfs 5, 52(4)
	fmuls 4, 19, 4
	fmuls 3, 18, 3
	fadds 0, 0, 4
	fmuls 5, 17, 5
	stfs 0, 12(22)
	fadds 0, 2, 3
	stfs 0, 8(22)
	fadds 0, 1, 5
	stfs 0, 4(22)
	bne	0, .Lrs_BB52_5
	addi 26, 26, 1
	addi 23, 23, 256
	cmplw	26, 27
	addi 24, 24, 256
	bne	0, .Lrs_BB52_2
.Lrs_BB52_7:
	lfd 31, 1224(1)
	ld 30, 1304(1)
	ld 29, 1296(1)
	ld 28, 1288(1)
	ld 27, 1280(1)
	ld 26, 1272(1)
	ld 25, 1264(1)
	ld 24, 1256(1)
	ld 23, 1248(1)
	ld 22, 1240(1)
	ld 21, 1232(1)
	lfd 30, 1216(1)
	lfd 29, 1208(1)
	lfd 28, 1200(1)
	lfd 27, 1192(1)
	lfd 26, 1184(1)
	lfd 25, 1176(1)
	lfd 24, 1168(1)
	lfd 23, 1160(1)
	lfd 22, 1152(1)
	lfd 21, 1144(1)
	lfd 20, 1136(1)
	lfd 19, 1128(1)
	lfd 18, 1120(1)
	lfd 17, 1112(1)
	lfd 16, 1104(1)
	addi 1, 1, 1328
	lwz 0, -8(1)
	mtlr 0
	blr
.Lrs_func_end52:
	.size	rs_sp_mat4_v16, .Lrs_func_end52-.Lrs_func_begin52

	.section	.text.rs_sp_mat4_v8,"ax",@progbits
	.globl	rs_sp_mat4_v8
	.p2align	2
	.type	rs_sp_mat4_v8,@function
rs_sp_mat4_v8:
.Lrs_func_begin53:
	mflr 0
	stw 0, -8(1)
	stwu 1, -816(1)
	std 30, 792(1)
	mr	30, 5
	srawi 5, 6, 31
	andc 5, 6, 5
	std 27, 768(1)
	rlwinm. 27, 5, 29, 3, 31
	std 21, 720(1)
	std 22, 728(1)
	std 23, 736(1)
	std 24, 744(1)
	std 25, 752(1)
	std 26, 760(1)
	std 28, 776(1)
	std 29, 784(1)
	stfd 16, 592(1)
	stfd 17, 600(1)
	stfd 18, 608(1)
	stfd 19, 616(1)
	stfd 20, 624(1)
	stfd 21, 632(1)
	stfd 22, 640(1)
	stfd 23, 648(1)
	stfd 24, 656(1)
	stfd 25, 664(1)
	stfd 26, 672(1)
	stfd 27, 680(1)
	stfd 28, 688(1)
	stfd 29, 696(1)
	stfd 30, 704(1)
	stfd 31, 712(1)
	beq	0, .Lrs_BB53_7
	lfs 31, 12(4)
	mr	29, 3
	lfs 30, 8(4)
	li 26, 0
	lfs 29, 4(4)
	addi 25, 1, 16
	lfs 28, 0(4)
	addi 28, 1, 80
	lfs 27, 28(4)
	mr	24, 3
	lfs 26, 24(4)
	mr	23, 30
	lfs 25, 20(4)
	lfs 24, 16(4)
	lfs 23, 44(4)
	lfs 22, 40(4)
	lfs 21, 36(4)
	lfs 20, 32(4)
	lfs 19, 60(4)
	lfs 18, 56(4)
	lfs 17, 52(4)
	lfs 16, 48(4)
.Lrs_BB53_2:
	slwi 3, 26, 7
	addi 3, 3, -16
	add 22, 29, 3
	add 21, 30, 3
	mr	3, 28
	li 4, 0
	li 5, 512
	bl memset
	li 3, 0
	mr	4, 25
.Lrs_BB53_3:
	lfsu 0, 16(21)
	addi 3, 3, 64
	cmplwi	3, 512
	lfs 1, 12(21)
	lfs 2, 8(21)
	lfs 3, 4(21)
	stfsu 0, 64(4)
	stfs 0, 12(4)
	stfs 0, 8(4)
	stfs 0, 4(4)
	stfs 3, 28(4)
	stfs 3, 24(4)
	stfs 3, 20(4)
	stfs 3, 16(4)
	stfs 2, 44(4)
	stfs 2, 40(4)
	stfs 2, 36(4)
	stfs 2, 32(4)
	stfs 1, 60(4)
	stfs 1, 56(4)
	stfs 1, 52(4)
	stfs 1, 48(4)
	bne	0, .Lrs_BB53_3
	li 3, 0
	mr	4, 25
.Lrs_BB53_5:
	lfsu 0, 64(4)
	addi 3, 3, 64
	cmplwi	3, 512
	lfs 1, 16(4)
	fmuls 0, 28, 0
	lfs 2, 32(4)
	lfs 3, 48(4)
	fmuls 1, 24, 1
	fadds 0, 0, 1
	fmuls 2, 20, 2
	lfs 4, 4(4)
	fadds 0, 0, 2
	lfs 5, 8(4)
	fmuls 3, 16, 3
	lfs 2, 24(4)
	fadds 0, 0, 3
	lfs 1, 20(4)
	fmuls 3, 29, 4
	lfs 6, 12(4)
	fmuls 4, 30, 5
	stfsu 0, 16(22)
	fmuls 2, 26, 2
	lfs 0, 28(4)
	fmuls 1, 25, 1
	fadds 2, 4, 2
	lfs 4, 40(4)
	fadds 1, 3, 1
	lfs 3, 44(4)
	fmuls 5, 31, 6
	fmuls 0, 27, 0
	fadds 0, 5, 0
	lfs 5, 36(4)
	fmuls 4, 22, 4
	fmuls 3, 23, 3
	fadds 2, 2, 4
	lfs 4, 60(4)
	fadds 0, 0, 3
	lfs 3, 56(4)
	fmuls 5, 21, 5
	fadds 1, 1, 5
	lfs 5, 52(4)
	fmuls 4, 19, 4
	fmuls 3, 18, 3
	fadds 0, 0, 4
	fmuls 5, 17, 5
	stfs 0, 12(22)
	fadds 0, 2, 3
	stfs 0, 8(22)
	fadds 0, 1, 5
	stfs 0, 4(22)
	bne	0, .Lrs_BB53_5
	addi 26, 26, 1
	addi 23, 23, 128
	cmplw	26, 27
	addi 24, 24, 128
	bne	0, .Lrs_BB53_2
.Lrs_BB53_7:
	lfd 31, 712(1)
	ld 30, 792(1)
	ld 29, 784(1)
	ld 28, 776(1)
	ld 27, 768(1)
	ld 26, 760(1)
	ld 25, 752(1)
	ld 24, 744(1)
	ld 23, 736(1)
	ld 22, 728(1)
	ld 21, 720(1)
	lfd 30, 704(1)
	lfd 29, 696(1)
	lfd 28, 688(1)
	lfd 27, 680(1)
	lfd 26, 672(1)
	lfd 25, 664(1)
	lfd 24, 656(1)
	lfd 23, 648(1)
	lfd 22, 640(1)
	lfd 21, 632(1)
	lfd 20, 624(1)
	lfd 19, 616(1)
	lfd 18, 608(1)
	lfd 17, 600(1)
	lfd 16, 592(1)
	addi 1, 1, 816
	lwz 0, -8(1)
	mtlr 0
	blr
.Lrs_func_end53:
	.size	rs_sp_mat4_v8, .Lrs_func_end53-.Lrs_func_begin53

	.section	.text.rs_sp_saxpy_v,"ax",@progbits
	.globl	rs_sp_saxpy_v
	.p2align	2
	.type	rs_sp_saxpy_v,@function
rs_sp_saxpy_v:
.Lrs_func_begin54:
	srawi 5, 6, 31
	andc 5, 6, 5
	rlwinm. 5, 5, 30, 2, 31
	beqlr	0
	addi 4, 4, -16
	addi 3, 3, -16
.Lrs_BB54_2:
	lfsu 0, 16(4)
	addi 5, 5, -1
	lfsu 2, 16(3)
	cmplwi	5, 0
	lfs 5, 8(4)
	fmuls 0, 1, 0
	lfs 4, 4(4)
	fadds 0, 0, 2
	lfs 6, 8(3)
	lfs 3, 12(4)
	fmuls 2, 1, 4
	lfs 7, 4(3)
	lfs 8, 12(3)
	fmuls 3, 1, 3
	stfs 0, 0(3)
	fmuls 0, 1, 5
	fadds 0, 0, 6
	stfs 0, 8(3)
	fadds 0, 2, 7
	stfs 0, 4(3)
	fadds 0, 3, 8
	stfs 0, 12(3)
	bne	0, .Lrs_BB54_2
	blr
.Lrs_func_end54:
	.size	rs_sp_saxpy_v, .Lrs_func_end54-.Lrs_func_begin54

	.section	.rodata.cst4,"aM",@progbits,4
	.p2align	2, 0x0
.Lrs_CPI55_0:
	.long	0x3b808081
	.section	.text.rs_sp_u8f_v,"ax",@progbits
	.globl	rs_sp_u8f_v
	.p2align	2
	.type	rs_sp_u8f_v,@function
rs_sp_u8f_v:
.Lrs_func_begin55:
	rlwinm. 6, 5, 28, 4, 31
	cmpwi 1, 5, 1
	cror 20, 4, 2
	bclr 12, 20, 0
	stwu 1, -224(1)
	lis 5, .Lrs_CPI55_0@ha
	lfs 0, .Lrs_CPI55_0@l(5)
	addi 4, 4, -16
	addi 3, 3, -64
	std 23, 144(1)
	std 24, 152(1)
	std 25, 160(1)
	std 26, 168(1)
	std 27, 176(1)
	std 28, 184(1)
	std 29, 192(1)
	std 30, 200(1)
.Lrs_BB55_2:
	lbzu 30, 16(4)
	addi 6, 6, -1
	cmplwi	6, 0
	lbz 5, 12(4)
	lbz 7, 13(4)
	lbz 8, 14(4)
	lbz 9, 15(4)
	lbz 10, 8(4)
	lbz 11, 9(4)
	lbz 12, 10(4)
	lbz 0, 11(4)
	lbz 29, 4(4)
	lbz 28, 5(4)
	lbz 27, 6(4)
	lbz 26, 7(4)
	lbz 25, 1(4)
	lbz 24, 2(4)
	lbz 23, 3(4)
	std 30, 136(1)
	lfd 1, 136(1)
	fcfid 1, 1
	frsp 1, 1
	fmuls 1, 1, 0
	stfsu 1, 64(3)
	std 23, 120(1)
	std 24, 112(1)
	std 25, 128(1)
	lfd 1, 120(1)
	lfd 2, 112(1)
	lfd 3, 128(1)
	fcfid 1, 1
	fcfid 2, 2
	frsp 1, 1
	fcfid 3, 3
	frsp 2, 2
	fmuls 1, 1, 0
	frsp 3, 3
	stfs 1, 12(3)
	fmuls 1, 2, 0
	stfs 1, 8(3)
	fmuls 1, 3, 0
	stfs 1, 4(3)
	std 26, 96(1)
	std 27, 88(1)
	std 28, 104(1)
	std 29, 80(1)
	lfd 1, 96(1)
	lfd 2, 88(1)
	lfd 3, 104(1)
	fcfid 1, 1
	lfd 4, 80(1)
	fcfid 2, 2
	frsp 1, 1
	fcfid 3, 3
	frsp 2, 2
	fmuls 1, 1, 0
	fcfid 4, 4
	frsp 3, 3
	stfs 1, 28(3)
	fmuls 1, 2, 0
	frsp 4, 4
	stfs 1, 24(3)
	fmuls 1, 3, 0
	stfs 1, 20(3)
	fmuls 1, 4, 0
	stfs 1, 16(3)
	std 0, 64(1)
	std 12, 56(1)
	std 11, 72(1)
	std 10, 16(1)
	lfd 1, 64(1)
	lfd 2, 56(1)
	lfd 3, 72(1)
	fcfid 1, 1
	lfd 4, 16(1)
	fcfid 2, 2
	frsp 1, 1
	fcfid 3, 3
	frsp 2, 2
	fmuls 1, 1, 0
	fcfid 4, 4
	frsp 3, 3
	stfs 1, 44(3)
	fmuls 1, 2, 0
	frsp 4, 4
	stfs 1, 40(3)
	fmuls 1, 3, 0
	stfs 1, 36(3)
	fmuls 1, 4, 0
	stfs 1, 32(3)
	std 9, 40(1)
	std 8, 32(1)
	std 7, 48(1)
	std 5, 24(1)
	lfd 1, 40(1)
	lfd 2, 32(1)
	lfd 3, 48(1)
	fcfid 1, 1
	lfd 4, 24(1)
	fcfid 2, 2
	frsp 1, 1
	fcfid 3, 3
	frsp 2, 2
	fmuls 1, 1, 0
	fcfid 4, 4
	frsp 3, 3
	stfs 1, 60(3)
	fmuls 1, 2, 0
	frsp 4, 4
	stfs 1, 56(3)
	fmuls 1, 3, 0
	stfs 1, 52(3)
	fmuls 1, 4, 0
	stfs 1, 48(3)
	bne	0, .Lrs_BB55_2
	ld 30, 200(1)
	ld 29, 192(1)
	ld 28, 184(1)
	ld 27, 176(1)
	ld 26, 168(1)
	ld 25, 160(1)
	ld 24, 152(1)
	ld 23, 144(1)
	addi 1, 1, 224
	blr
.Lrs_func_end55:
	.size	rs_sp_u8f_v, .Lrs_func_end55-.Lrs_func_begin55

	.type	rs_g_a,@object
	.section	.bss.rs_g_a,"aw",@nobits
	.globl	rs_g_a
	.p2align	4, 0x0
rs_g_a:
	.space	4096
	.size	rs_g_a, 4096

	.type	rs_g_b,@object
	.section	.bss.rs_g_b,"aw",@nobits
	.globl	rs_g_b
	.p2align	4, 0x0
rs_g_b:
	.space	4096
	.size	rs_g_b, 4096

	.type	rs_g_c,@object
	.section	.bss.rs_g_c,"aw",@nobits
	.globl	rs_g_c
	.p2align	4, 0x0
rs_g_c:
	.space	4096
	.size	rs_g_c, 4096

	.type	rs_g_ia,@object
	.section	.bss.rs_g_ia,"aw",@nobits
	.globl	rs_g_ia
	.p2align	4, 0x0
rs_g_ia:
	.space	4096
	.size	rs_g_ia, 4096

	.type	rs_g_ib,@object
	.section	.bss.rs_g_ib,"aw",@nobits
	.globl	rs_g_ib
	.p2align	4, 0x0
rs_g_ib:
	.space	4096
	.size	rs_g_ib, 4096

	.type	rs_g_ic,@object
	.section	.bss.rs_g_ic,"aw",@nobits
	.globl	rs_g_ic
	.p2align	4, 0x0
rs_g_ic:
	.space	4096
	.size	rs_g_ic, 4096

	.type	.Lrs__MergedGlobals,@object
	.section	.data.rel.ro..Lrs__MergedGlobals,"aw",@progbits
	.p2align	2, 0x0
.Lrs__MergedGlobals:
	.long	.Lrs__MergedGlobals+16
	.ascii	"\000\000\000\033\000\000\000p\000\000\000\021"
	.asciz	"examples/vmx128bench/lib.rs"
	.size	.Lrs__MergedGlobals, 44

	.globl	rs_sp_mat4_s
	.type	rs_sp_mat4_s,@function
rs_sp_mat4_s = rs_k_mat4_vec4_r
	.globl	rs_sp_u8f_s
	.type	rs_sp_u8f_s,@function
rs_sp_u8f_s = rs_k_u8_to_f32_r
	.globl	rs_sp_saxpy_s
	.type	rs_sp_saxpy_s,@function
rs_sp_saxpy_s = rs_k_saxpy_r
	.globl	rs_sp_copy_s
	.type	rs_sp_copy_s,@function
rs_sp_copy_s = rs_k_copy_u32_r
.Lrs_anon.022d31073c17f2770e6d6b3bd0d67d08.1 = .Lrs__MergedGlobals
	.size	.Lrs_anon.022d31073c17f2770e6d6b3bd0d67d08.1, 16
.Lrs_anon.022d31073c17f2770e6d6b3bd0d67d08.0 = .Lrs__MergedGlobals+16
	.size	.Lrs_anon.022d31073c17f2770e6d6b3bd0d67d08.0, 28
