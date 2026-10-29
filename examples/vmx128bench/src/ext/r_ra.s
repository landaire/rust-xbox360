# kernels.rs: rustc of the PPC64 toolchain (rustc_bootstrap cell)
# Buck target //examples/vmx128bench:kernels_ra
	.section	.rodata.cst4,"aM",@progbits,4
	.p2align	2, 0x0
.Lra_CPI0_0:
	.long	0x00000000
	.section	.text.ra_k_abs_f32_r,"ax",@progbits
	.globl	ra_k_abs_f32_r
	.p2align	2
	.type	ra_k_abs_f32_r,@function
ra_k_abs_f32_r:
.Lra_func_begin0:
	cmpwi	5, 1
	bltlr	0
	lis 6, .Lra_CPI0_0@ha
	lfs 0, .Lra_CPI0_0@l(6)
	addi 4, 4, -4
	addi 3, 3, -4
	b .Lra_BB0_3
.Lra_BB0_2:
	addi 5, 5, -1
	cmplwi	5, 0
	stfsu 1, 4(3)
	beqlr	0
.Lra_BB0_3:
	lfsu 1, 4(4)
	fcmpu 0, 1, 0
	bge 0, .Lra_BB0_2
	fneg 1, 1
	b .Lra_BB0_2
.Lra_func_end0:
	.size	ra_k_abs_f32_r, .Lra_func_end0-.Lra_func_begin0

	.section	.text.ra_k_abs_i32_r,"ax",@progbits
	.globl	ra_k_abs_i32_r
	.p2align	2
	.type	ra_k_abs_i32_r,@function
ra_k_abs_i32_r:
.Lra_func_begin1:
	cmpwi	5, 1
	bltlr	0
	stwu 1, -32(1)
	cmplwi	5, 4
	sub	7, 3, 4
	li 6, 0
	bc 12, 0, .Lra_BB1_6
	cmplwi	7, 16
	bc 12, 0, .Lra_BB1_6
	rlwinm 6, 5, 0, 1, 29
	li 7, 0
	li 8, 15
	vxor 2, 2, 2
	addi 9, 1, 16
	mr	10, 6
.Lra_BB1_4:
	add 11, 4, 7
	lvx 3, 4, 7
	mr	12, 3
	lvx 4, 11, 8
	addi 10, 10, -4
	lvsl 5, 4, 7
	cmplwi	10, 0
	vperm 3, 3, 4, 5
	vsubuwm 4, 2, 3
	vmaxsw 3, 3, 4
	stvx 3, 0, 9
	ld 11, 16(1)
	stdux 11, 12, 7
	ld 11, 24(1)
	addi 7, 7, 16
	std 11, 8(12)
	bne	0, .Lra_BB1_4
	cmplw	5, 6
	beq	0, .Lra_BB1_8
.Lra_BB1_6:
	sub	5, 5, 6
	slwi 6, 6, 2
	addi 6, 6, -4
	add 4, 4, 6
	add 3, 3, 6
.Lra_BB1_7:
	lwzu 6, 4(4)
	addi 5, 5, -1
	cmplwi	5, 0
	srawi 7, 6, 31
	xor 6, 6, 7
	sub	6, 6, 7
	stwu 6, 4(3)
	bne	0, .Lra_BB1_7
.Lra_BB1_8:
	addi 1, 1, 32
	blr
.Lra_func_end1:
	.size	ra_k_abs_i32_r, .Lra_func_end1-.Lra_func_begin1

	.section	.text.ra_k_add16_f32_r,"ax",@progbits
	.globl	ra_k_add16_f32_r
	.p2align	2
	.type	ra_k_add16_f32_r,@function
ra_k_add16_f32_r:
.Lra_func_begin2:
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
.Lra_func_end2:
	.size	ra_k_add16_f32_r, .Lra_func_end2-.Lra_func_begin2

	.section	.text.ra_k_add_f32_r,"ax",@progbits
	.globl	ra_k_add_f32_r
	.p2align	2
	.type	ra_k_add_f32_r,@function
ra_k_add_f32_r:
.Lra_func_begin3:
	cmpwi	6, 1
	bltlr	0
	stwu 1, -64(1)
	cmplwi	6, 8
	li 7, 0
	std 30, 40(1)
	bge 0, .Lra_BB3_5
.Lra_BB3_2:
	sub	6, 6, 7
	slwi 7, 7, 2
	addi 7, 7, -4
	add 4, 4, 7
	add 5, 5, 7
	add 3, 3, 7
.Lra_BB3_3:
	lfsu 0, 4(4)
	addi 6, 6, -1
	lfsu 1, 4(5)
	cmplwi	6, 0
	fadds 0, 0, 1
	stfsu 0, 4(3)
	bne	0, .Lra_BB3_3
.Lra_BB3_4:
	ld 30, 40(1)
	addi 1, 1, 64
	blr
.Lra_BB3_5:
	sub	8, 3, 4
	cmplwi	8, 16
	sub	8, 3, 5
	bc 12, 0, .Lra_BB3_2
	cmplwi	8, 16
	bc 12, 0, .Lra_BB3_2
	rlwinm 7, 6, 0, 1, 29
	li 8, 0
	li 9, 15
	addi 10, 1, 16
	mr	11, 7
.Lra_BB3_8:
	add 12, 4, 8
	lvx 2, 4, 8
	mr	30, 3
	lvx 3, 12, 9
	add 12, 5, 8
	lvx 4, 5, 8
	addi 11, 11, -4
	lvx 5, 12, 9
	cmplwi	11, 0
	lvsl 0, 4, 8
	vperm 2, 2, 3, 0
	lvsl 1, 5, 8
	vperm 3, 4, 5, 1
	vaddfp 2, 2, 3
	stvx 2, 0, 10
	ld 12, 16(1)
	stdux 12, 30, 8
	ld 12, 24(1)
	addi 8, 8, 16
	std 12, 8(30)
	bne	0, .Lra_BB3_8
	cmplw	6, 7
	bne	0, .Lra_BB3_2
	b .Lra_BB3_4
.Lra_func_end3:
	.size	ra_k_add_f32_r, .Lra_func_end3-.Lra_func_begin3

	.section	.text.ra_k_add_i16_r,"ax",@progbits
	.globl	ra_k_add_i16_r
	.p2align	2
	.type	ra_k_add_i16_r,@function
ra_k_add_i16_r:
.Lra_func_begin4:
	cmpwi	6, 1
	bltlr	0
	stwu 1, -64(1)
	cmplwi	6, 16
	li 7, 0
	std 30, 40(1)
	bge 0, .Lra_BB4_5
.Lra_BB4_2:
	sub	6, 6, 7
	slwi 7, 7, 1
	addi 7, 7, -2
	add 4, 4, 7
	add 5, 5, 7
	add 3, 3, 7
.Lra_BB4_3:
	lhzu 7, 2(4)
	addi 6, 6, -1
	lhzu 8, 2(5)
	cmplwi	6, 0
	add 7, 8, 7
	sthu 7, 2(3)
	bne	0, .Lra_BB4_3
.Lra_BB4_4:
	ld 30, 40(1)
	addi 1, 1, 64
	blr
.Lra_BB4_5:
	sub	8, 3, 4
	cmplwi	8, 16
	sub	8, 3, 5
	bc 12, 0, .Lra_BB4_2
	cmplwi	8, 16
	bc 12, 0, .Lra_BB4_2
	rlwinm 7, 6, 0, 1, 28
	li 8, 0
	li 9, 15
	addi 10, 1, 16
	mr	11, 7
.Lra_BB4_8:
	add 12, 4, 8
	lvx 2, 4, 8
	mr	30, 3
	lvx 3, 12, 9
	add 12, 5, 8
	lvx 4, 5, 8
	addi 11, 11, -8
	lvx 5, 12, 9
	cmplwi	11, 0
	lvsl 0, 4, 8
	vperm 2, 2, 3, 0
	lvsl 1, 5, 8
	vperm 3, 4, 5, 1
	vadduhm 2, 3, 2
	stvx 2, 0, 10
	ld 12, 16(1)
	stdux 12, 30, 8
	ld 12, 24(1)
	addi 8, 8, 16
	std 12, 8(30)
	bne	0, .Lra_BB4_8
	cmplw	6, 7
	bne	0, .Lra_BB4_2
	b .Lra_BB4_4
.Lra_func_end4:
	.size	ra_k_add_i16_r, .Lra_func_end4-.Lra_func_begin4

	.section	.text.ra_k_add_i32,"ax",@progbits
	.globl	ra_k_add_i32
	.p2align	2
	.type	ra_k_add_i32,@function
ra_k_add_i32:
.Lra_func_begin5:
	cmpwi	6, 1
	bltlr	0
	stwu 1, -64(1)
	cmplwi	6, 20
	li 7, 0
	std 30, 40(1)
	bge 0, .Lra_BB5_5
.Lra_BB5_2:
	slwi 8, 7, 2
	sub	6, 6, 7
	addi 7, 8, -4
	add 4, 4, 7
	add 5, 5, 7
	add 3, 3, 7
.Lra_BB5_3:
	lwzu 7, 4(4)
	addi 6, 6, -1
	lwzu 8, 4(5)
	cmplwi	6, 0
	add 7, 8, 7
	stwu 7, 4(3)
	bne	0, .Lra_BB5_3
.Lra_BB5_4:
	ld 30, 40(1)
	addi 1, 1, 64
	blr
.Lra_BB5_5:
	sub	8, 3, 4
	cmplwi	8, 16
	sub	8, 3, 5
	bc 12, 0, .Lra_BB5_2
	cmplwi	8, 16
	bc 12, 0, .Lra_BB5_2
	rlwinm 7, 6, 0, 1, 29
	li 8, 0
	li 9, 15
	addi 10, 1, 16
	mr	11, 7
.Lra_BB5_8:
	add 12, 4, 8
	lvx 2, 4, 8
	mr	30, 3
	lvx 3, 12, 9
	add 12, 5, 8
	lvx 4, 5, 8
	addi 11, 11, -4
	lvx 5, 12, 9
	cmplwi	11, 0
	lvsl 0, 4, 8
	vperm 2, 2, 3, 0
	lvsl 1, 5, 8
	vperm 3, 4, 5, 1
	vadduwm 2, 3, 2
	stvx 2, 0, 10
	ld 12, 16(1)
	stdux 12, 30, 8
	ld 12, 24(1)
	addi 8, 8, 16
	std 12, 8(30)
	bne	0, .Lra_BB5_8
	cmplw	6, 7
	bne	0, .Lra_BB5_2
	b .Lra_BB5_4
.Lra_func_end5:
	.size	ra_k_add_i32, .Lra_func_end5-.Lra_func_begin5

	.section	.text.ra_k_add_i32_r,"ax",@progbits
	.globl	ra_k_add_i32_r
	.p2align	2
	.type	ra_k_add_i32_r,@function
ra_k_add_i32_r:
.Lra_func_begin6:
	cmpwi	6, 1
	bltlr	0
	stwu 1, -64(1)
	cmplwi	6, 20
	li 7, 0
	std 30, 40(1)
	bge 0, .Lra_BB6_5
.Lra_BB6_2:
	sub	6, 6, 7
	slwi 7, 7, 2
	addi 7, 7, -4
	add 4, 4, 7
	add 5, 5, 7
	add 3, 3, 7
.Lra_BB6_3:
	lwzu 7, 4(4)
	addi 6, 6, -1
	lwzu 8, 4(5)
	cmplwi	6, 0
	add 7, 8, 7
	stwu 7, 4(3)
	bne	0, .Lra_BB6_3
.Lra_BB6_4:
	ld 30, 40(1)
	addi 1, 1, 64
	blr
.Lra_BB6_5:
	sub	8, 3, 4
	cmplwi	8, 16
	sub	8, 3, 5
	bc 12, 0, .Lra_BB6_2
	cmplwi	8, 16
	bc 12, 0, .Lra_BB6_2
	rlwinm 7, 6, 0, 1, 29
	li 8, 0
	li 9, 15
	addi 10, 1, 16
	mr	11, 7
.Lra_BB6_8:
	add 12, 4, 8
	lvx 2, 4, 8
	mr	30, 3
	lvx 3, 12, 9
	add 12, 5, 8
	lvx 4, 5, 8
	addi 11, 11, -4
	lvx 5, 12, 9
	cmplwi	11, 0
	lvsl 0, 4, 8
	vperm 2, 2, 3, 0
	lvsl 1, 5, 8
	vperm 3, 4, 5, 1
	vadduwm 2, 3, 2
	stvx 2, 0, 10
	ld 12, 16(1)
	stdux 12, 30, 8
	ld 12, 24(1)
	addi 8, 8, 16
	std 12, 8(30)
	bne	0, .Lra_BB6_8
	cmplw	6, 7
	bne	0, .Lra_BB6_2
	b .Lra_BB6_4
.Lra_func_end6:
	.size	ra_k_add_i32_r, .Lra_func_end6-.Lra_func_begin6

	.section	.text.ra_k_add_u8_r,"ax",@progbits
	.globl	ra_k_add_u8_r
	.p2align	2
	.type	ra_k_add_u8_r,@function
ra_k_add_u8_r:
.Lra_func_begin7:
	cmpwi	6, 1
	bltlr	0
	stwu 1, -32(1)
	cmplwi	6, 16
	li 7, 0
	bge 0, .Lra_BB7_5
.Lra_BB7_2:
	addi 8, 7, -1
	add 4, 4, 8
	add 5, 5, 8
	add 3, 3, 8
.Lra_BB7_3:
	lbzu 8, 1(4)
	addi 7, 7, 1
	lbzu 9, 1(5)
	cmplw	6, 7
	add 8, 9, 8
	stbu 8, 1(3)
	bne	0, .Lra_BB7_3
.Lra_BB7_4:
	addi 1, 1, 32
	blr
.Lra_BB7_5:
	sub	8, 3, 4
	cmplwi	8, 16
	sub	8, 3, 5
	bc 12, 0, .Lra_BB7_2
	cmplwi	8, 16
	bc 12, 0, .Lra_BB7_2
	rlwinm 7, 6, 0, 1, 27
	li 8, 0
	li 9, 15
	addi 10, 1, 16
.Lra_BB7_8:
	add 11, 4, 8
	lvx 2, 4, 8
	mr	12, 3
	lvx 3, 11, 9
	add 11, 5, 8
	lvx 4, 5, 8
	lvx 5, 11, 9
	lvsl 0, 4, 8
	vperm 2, 2, 3, 0
	lvsl 1, 5, 8
	vperm 3, 4, 5, 1
	vaddubm 2, 3, 2
	stvx 2, 0, 10
	ld 11, 16(1)
	stdux 11, 12, 8
	ld 11, 24(1)
	addi 8, 8, 16
	cmplw	7, 8
	std 11, 8(12)
	bne	0, .Lra_BB7_8
	cmplw	6, 7
	bne	0, .Lra_BB7_2
	b .Lra_BB7_4
.Lra_func_end7:
	.size	ra_k_add_u8_r, .Lra_func_end7-.Lra_func_begin7

	.section	.text.ra_k_addsat_u8_r,"ax",@progbits
	.globl	ra_k_addsat_u8_r
	.p2align	2
	.type	ra_k_addsat_u8_r,@function
ra_k_addsat_u8_r:
.Lra_func_begin8:
	cmpwi	6, 1
	bltlr	0
	stwu 1, -32(1)
	cmplwi	6, 16
	li 7, 0
	bge 0, .Lra_BB8_6
.Lra_BB8_2:
	addi 8, 7, -1
	add 4, 4, 8
	add 5, 5, 8
	add 3, 3, 8
	b .Lra_BB8_4
.Lra_BB8_3:
	addi 7, 7, 1
	cmplw	6, 7
	stbu 8, 1(3)
	beq	0, .Lra_BB8_11
.Lra_BB8_4:
	lbzu 8, 1(4)
	lbzu 9, 1(5)
	add 8, 8, 9
	cmplwi	8, 255
	blt	0, .Lra_BB8_3
	li 8, 255
	b .Lra_BB8_3
.Lra_BB8_6:
	sub	8, 3, 4
	cmplwi	8, 16
	sub	8, 3, 5
	bc 12, 0, .Lra_BB8_2
	cmplwi	8, 16
	bc 12, 0, .Lra_BB8_2
	rlwinm 7, 6, 0, 1, 27
	li 8, 0
	li 9, 15
	addi 10, 1, 16
.Lra_BB8_9:
	add 11, 4, 8
	lvx 2, 4, 8
	mr	12, 3
	lvx 3, 11, 9
	add 11, 5, 8
	lvx 4, 5, 8
	lvx 5, 11, 9
	lvsl 0, 4, 8
	vperm 2, 2, 3, 0
	lvsl 1, 5, 8
	vperm 3, 4, 5, 1
	vaddubs 2, 2, 3
	stvx 2, 0, 10
	ld 11, 16(1)
	stdux 11, 12, 8
	ld 11, 24(1)
	addi 8, 8, 16
	cmplw	7, 8
	std 11, 8(12)
	bne	0, .Lra_BB8_9
	cmplw	6, 7
	bne	0, .Lra_BB8_2
.Lra_BB8_11:
	addi 1, 1, 32
	blr
.Lra_func_end8:
	.size	ra_k_addsat_u8_r, .Lra_func_end8-.Lra_func_begin8

	.section	.text.ra_k_and_u32_r,"ax",@progbits
	.globl	ra_k_and_u32_r
	.p2align	2
	.type	ra_k_and_u32_r,@function
ra_k_and_u32_r:
.Lra_func_begin9:
	cmpwi	6, 1
	bltlr	0
	stwu 1, -64(1)
	cmplwi	6, 20
	li 7, 0
	std 30, 40(1)
	bge 0, .Lra_BB9_5
.Lra_BB9_2:
	sub	6, 6, 7
	slwi 7, 7, 2
	addi 7, 7, -4
	add 4, 4, 7
	add 5, 5, 7
	add 3, 3, 7
.Lra_BB9_3:
	lwzu 7, 4(4)
	addi 6, 6, -1
	lwzu 8, 4(5)
	cmplwi	6, 0
	and 7, 8, 7
	stwu 7, 4(3)
	bne	0, .Lra_BB9_3
.Lra_BB9_4:
	ld 30, 40(1)
	addi 1, 1, 64
	blr
.Lra_BB9_5:
	sub	8, 3, 4
	cmplwi	8, 16
	sub	8, 3, 5
	bc 12, 0, .Lra_BB9_2
	cmplwi	8, 16
	bc 12, 0, .Lra_BB9_2
	rlwinm 7, 6, 0, 1, 29
	li 8, 0
	li 9, 15
	addi 10, 1, 16
	mr	11, 7
.Lra_BB9_8:
	add 12, 4, 8
	lvx 2, 4, 8
	mr	30, 3
	lvx 3, 12, 9
	add 12, 5, 8
	lvx 4, 5, 8
	addi 11, 11, -4
	lvx 5, 12, 9
	cmplwi	11, 0
	lvsl 0, 4, 8
	vperm 2, 2, 3, 0
	lvsl 1, 5, 8
	vperm 3, 4, 5, 1
	vand 2, 3, 2
	stvx 2, 0, 10
	ld 12, 16(1)
	stdux 12, 30, 8
	ld 12, 24(1)
	addi 8, 8, 16
	std 12, 8(30)
	bne	0, .Lra_BB9_8
	cmplw	6, 7
	bne	0, .Lra_BB9_2
	b .Lra_BB9_4
.Lra_func_end9:
	.size	ra_k_and_u32_r, .Lra_func_end9-.Lra_func_begin9

	.section	.text.ra_k_avg_u8_r,"ax",@progbits
	.globl	ra_k_avg_u8_r
	.p2align	2
	.type	ra_k_avg_u8_r,@function
ra_k_avg_u8_r:
.Lra_func_begin10:
	cmpwi	6, 1
	bltlr	0
	stwu 1, -32(1)
	cmplwi	6, 16
	li 7, 0
	bge 0, .Lra_BB10_5
.Lra_BB10_2:
	addi 8, 7, -1
	add 4, 4, 8
	add 5, 5, 8
	add 3, 3, 8
.Lra_BB10_3:
	lbzu 8, 1(4)
	addi 7, 7, 1
	lbzu 9, 1(5)
	cmplw	6, 7
	add 8, 8, 9
	addi 8, 8, 1
	srwi 8, 8, 1
	stbu 8, 1(3)
	bne	0, .Lra_BB10_3
.Lra_BB10_4:
	addi 1, 1, 32
	blr
.Lra_BB10_5:
	sub	8, 3, 4
	cmplwi	8, 16
	sub	8, 3, 5
	bc 12, 0, .Lra_BB10_2
	cmplwi	8, 16
	bc 12, 0, .Lra_BB10_2
	rlwinm 7, 6, 0, 1, 27
	li 8, 0
	li 9, 15
	vspltisb 2, 1
	addi 10, 1, 16
.Lra_BB10_8:
	add 11, 4, 8
	lvx 3, 4, 8
	mr	12, 3
	lvx 4, 11, 9
	add 11, 5, 8
	lvx 5, 5, 8
	lvx 0, 11, 9
	lvsl 1, 4, 8
	vperm 3, 3, 4, 1
	lvsl 6, 5, 8
	vperm 4, 5, 0, 6
	vxor 5, 3, 4
	vor 3, 3, 4
	vsrb 4, 5, 2
	vsububm 3, 3, 4
	stvx 3, 0, 10
	ld 11, 16(1)
	stdux 11, 12, 8
	ld 11, 24(1)
	addi 8, 8, 16
	cmplw	7, 8
	std 11, 8(12)
	bne	0, .Lra_BB10_8
	cmplw	6, 7
	bne	0, .Lra_BB10_2
	b .Lra_BB10_4
.Lra_func_end10:
	.size	ra_k_avg_u8_r, .Lra_func_end10-.Lra_func_begin10

	.section	.text.ra_k_clamp_f32_r,"ax",@progbits
	.globl	ra_k_clamp_f32_r
	.p2align	2
	.type	ra_k_clamp_f32_r,@function
ra_k_clamp_f32_r:
.Lra_func_begin11:
	cmpwi	7, 1
	bltlr	0
	stwu 1, -64(1)
	cmplwi	7, 4
	sub	6, 3, 4
	li 5, 0
	bc 12, 0, .Lra_BB11_7
	cmplwi	6, 16
	bc 12, 0, .Lra_BB11_7
	stfs 1, 48(1)
	addi 5, 1, 48
	lvx 2, 0, 5
	addi 5, 1, 32
	stfs 2, 32(1)
	li 6, 0
	lvx 3, 0, 5
	rlwinm 5, 7, 0, 1, 29
	li 8, 15
	vspltw 2, 2, 0
	addi 9, 1, 16
	vspltw 3, 3, 0
	mr	10, 5
.Lra_BB11_4:
	add 11, 4, 6
	lvx 4, 4, 6
	mr	12, 3
	lvx 5, 11, 8
	addi 10, 10, -4
	lvsl 0, 4, 6
	cmplwi	10, 0
	vperm 4, 4, 5, 0
	vcmpgtfp 5, 2, 4
	vsel 4, 4, 2, 5
	vcmpgtfp 5, 4, 3
	vsel 4, 4, 3, 5
	stvx 4, 0, 9
	ld 11, 16(1)
	stdux 11, 12, 6
	ld 11, 24(1)
	addi 6, 6, 16
	std 11, 8(12)
	bne	0, .Lra_BB11_4
	cmplw	7, 5
	bne	0, .Lra_BB11_7
.Lra_BB11_6:
	addi 1, 1, 64
	blr
.Lra_BB11_7:
	sub	6, 7, 5
	slwi 5, 5, 2
	addi 5, 5, -4
	add 4, 4, 5
	add 3, 3, 5
	b .Lra_BB11_9
.Lra_BB11_8:
	addi 6, 6, -1
	cmplwi	6, 0
	stfsu 0, 4(3)
	beq	0, .Lra_BB11_6
.Lra_BB11_9:
	lfsu 0, 4(4)
	fmr 3, 1
	fcmpu 0, 0, 1
	bge 0, .Lra_BB11_11
	fcmpu 0, 3, 2
	fmr 0, 2
	bgt	0, .Lra_BB11_8
	b .Lra_BB11_12
.Lra_BB11_11:
	fmr 3, 0
	fcmpu 0, 3, 2
	fmr 0, 2
	bgt	0, .Lra_BB11_8
.Lra_BB11_12:
	fmr 0, 3
	b .Lra_BB11_8
.Lra_func_end11:
	.size	ra_k_clamp_f32_r, .Lra_func_end11-.Lra_func_begin11

	.section	.rodata.cst16,"aM",@progbits,16
	.p2align	4, 0x0
.Lra_CPI12_0:
	.short	64536
	.short	64536
	.short	64536
	.short	64536
	.short	64536
	.short	64536
	.short	64536
	.short	64536
.Lra_CPI12_1:
	.short	1000
	.short	1000
	.short	1000
	.short	1000
	.short	1000
	.short	1000
	.short	1000
	.short	1000
	.section	.text.ra_k_clamp_i16_r,"ax",@progbits
	.globl	ra_k_clamp_i16_r
	.p2align	2
	.type	ra_k_clamp_i16_r,@function
ra_k_clamp_i16_r:
.Lra_func_begin12:
	cmpwi	5, 1
	bltlr	0
	stwu 1, -32(1)
	cmplwi	5, 8
	sub	7, 3, 4
	li 6, 0
	bc 12, 0, .Lra_BB12_7
	cmplwi	7, 16
	bc 12, 0, .Lra_BB12_7
	li 9, .Lra_CPI12_0@l
	lis 10, .Lra_CPI12_0@ha
	lvx 2, 10, 9
	li 9, .Lra_CPI12_1@l
	lis 10, .Lra_CPI12_1@ha
	lvx 3, 10, 9
	rlwinm 6, 5, 0, 1, 28
	li 7, 0
	li 8, 15
	addi 9, 1, 16
	mr	10, 6
.Lra_BB12_4:
	add 11, 4, 7
	lvx 4, 4, 7
	mr	12, 3
	lvx 5, 11, 8
	addi 10, 10, -8
	lvsl 0, 4, 7
	cmplwi	10, 0
	vperm 4, 4, 5, 0
	vmaxsh 4, 4, 2
	vminsh 4, 4, 3
	stvx 4, 0, 9
	ld 11, 16(1)
	stdux 11, 12, 7
	ld 11, 24(1)
	addi 7, 7, 16
	std 11, 8(12)
	bne	0, .Lra_BB12_4
	cmplw	5, 6
	bne	0, .Lra_BB12_7
.Lra_BB12_6:
	addi 1, 1, 32
	blr
.Lra_BB12_7:
	sub	5, 5, 6
	slwi 6, 6, 1
	addi 6, 6, -2
	add 4, 4, 6
	add 3, 3, 6
	b .Lra_BB12_9
.Lra_BB12_8:
	addi 5, 5, -1
	cmplwi	5, 0
	sthu 6, 2(3)
	beq	0, .Lra_BB12_6
.Lra_BB12_9:
	lhau 6, 2(4)
	cmpwi	6, -1000
	ble 0, .Lra_BB12_11
	cmpwi	6, 1000
	blt	0, .Lra_BB12_8
	b .Lra_BB12_12
.Lra_BB12_11:
	li 6, -1000
	cmpwi	6, 1000
	blt	0, .Lra_BB12_8
.Lra_BB12_12:
	li 6, 1000
	b .Lra_BB12_8
.Lra_func_end12:
	.size	ra_k_clamp_i16_r, .Lra_func_end12-.Lra_func_begin12

	.section	.text.ra_k_copy_u32_r,"ax",@progbits
	.globl	ra_k_copy_u32_r
	.p2align	2
	.type	ra_k_copy_u32_r,@function
ra_k_copy_u32_r:
.Lra_func_begin13:
	cmpwi	5, 1
	bltlr	0
	addi 4, 4, -4
	addi 3, 3, -4
.Lra_BB13_2:
	lwzu 6, 4(4)
	addi 5, 5, -1
	cmplwi	5, 0
	stwu 6, 4(3)
	bne	0, .Lra_BB13_2
	blr
.Lra_func_end13:
	.size	ra_k_copy_u32_r, .Lra_func_end13-.Lra_func_begin13

	.section	.text.ra_k_copy_u8,"ax",@progbits
	.globl	ra_k_copy_u8
	.p2align	2
	.type	ra_k_copy_u8,@function
ra_k_copy_u8:
.Lra_func_begin14:
	cmpwi	5, 1
	bltlr	0
	addi 4, 4, -1
	addi 3, 3, -1
	li 6, 0
.Lra_BB14_2:
	lbzu 7, 1(4)
	addi 6, 6, 1
	cmplw	5, 6
	stbu 7, 1(3)
	bne	0, .Lra_BB14_2
	blr
.Lra_func_end14:
	.size	ra_k_copy_u8, .Lra_func_end14-.Lra_func_begin14

	.section	.rodata.cst4,"aM",@progbits,4
	.p2align	2, 0x0
.Lra_CPI15_0:
	.long	0x00000000
	.section	.text.ra_k_dot_f32,"ax",@progbits
	.globl	ra_k_dot_f32
	.p2align	2
	.type	ra_k_dot_f32,@function
ra_k_dot_f32:
.Lra_func_begin15:
	srawi 6, 5, 31
	andc 5, 5, 6
	clrlwi.	5, 5, 2
	lis 6, .Lra_CPI15_0@ha
	beq	0, .Lra_BB15_4
	lfs 1, .Lra_CPI15_0@l(6)
	addi 3, 3, -4
	addi 4, 4, -4
.Lra_BB15_2:
	lfsu 0, 4(4)
	addi 5, 5, -1
	lfsu 2, 4(3)
	cmplwi	5, 0
	fmuls 0, 2, 0
	fadds 1, 1, 0
	bne	0, .Lra_BB15_2
	blr
.Lra_BB15_4:
	lfs 1, .Lra_CPI15_0@l(6)
	blr
.Lra_func_end15:
	.size	ra_k_dot_f32, .Lra_func_end15-.Lra_func_begin15

	.section	.text.ra_k_dot_i16,"ax",@progbits
	.globl	ra_k_dot_i16
	.p2align	2
	.type	ra_k_dot_i16,@function
ra_k_dot_i16:
.Lra_func_begin16:
	srawi 6, 5, 31
	andc 7, 5, 6
	cmplwi	7, 0
	beq	0, .Lra_BB16_8
	stwu 1, -48(1)
	li 6, 0
	cmpwi	5, 2
	li 8, 0
	std 29, 16(1)
	std 30, 24(1)
	blt	0, .Lra_BB16_5
	rlwinm 8, 7, 0, 1, 30
	addi 9, 3, -4
	addi 10, 4, -4
	mr	12, 8
	li 11, 0
.Lra_BB16_3:
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
	bne	0, .Lra_BB16_3
	cmplw	5, 8
	add 6, 11, 6
	beq	0, .Lra_BB16_7
.Lra_BB16_5:
	sub	5, 7, 8
	slwi 7, 8, 1
	addi 7, 7, -2
	add 3, 3, 7
	add 4, 4, 7
.Lra_BB16_6:
	lhau 7, 2(4)
	addi 5, 5, -1
	lhau 8, 2(3)
	cmplwi	5, 0
	mullw 7, 7, 8
	add 6, 7, 6
	bne	0, .Lra_BB16_6
.Lra_BB16_7:
	ld 30, 24(1)
	ld 29, 16(1)
	addi 1, 1, 48
	mr	3, 6
	blr
.Lra_BB16_8:
	li 3, 0
	blr
.Lra_func_end16:
	.size	ra_k_dot_i16, .Lra_func_end16-.Lra_func_begin16

	.section	.rodata.cst4,"aM",@progbits,4
	.p2align	2, 0x0
.Lra_CPI17_0:
	.long	0xcf000000
.Lra_CPI17_1:
	.long	0x4effffff
	.section	.text.ra_k_f2i_r,"ax",@progbits
	.globl	ra_k_f2i_r
	.p2align	2
	.type	ra_k_f2i_r,@function
ra_k_f2i_r:
.Lra_func_begin17:
	cmpwi	5, 1
	bltlr	0
	stwu 1, -32(1)
	lis 7, .Lra_CPI17_0@ha
	lfs 0, .Lra_CPI17_0@l(7)
	lis 7, .Lra_CPI17_1@ha
	lfs 1, .Lra_CPI17_1@l(7)
	lis 7, 32767
	addi 4, 4, -4
	addi 3, 3, -4
	addi 6, 1, 28
	ori 7, 7, 65535
	b .Lra_BB17_3
.Lra_BB17_2:
	addi 5, 5, -1
	cmplwi	5, 0
	stwu 8, 4(3)
	beq	0, .Lra_BB17_9
.Lra_BB17_3:
	lfsu 2, 4(4)
	lis 8, -32768
	fcmpu 0, 2, 0
	fctiwz 3, 2
	cror 20, 0, 3
	stfiwx 3, 0, 6
	bc 4, 20, .Lra_BB17_6
	fcmpu 0, 2, 1
	mr	9, 7
	ble 0, .Lra_BB17_7
.Lra_BB17_5:
	fcmpu 0, 2, 2
	li 8, 0
	bun 0, .Lra_BB17_2
	b .Lra_BB17_8
.Lra_BB17_6:
	lwz 8, 28(1)
	fcmpu 0, 2, 1
	mr	9, 7
	bgt	0, .Lra_BB17_5
.Lra_BB17_7:
	mr	9, 8
	fcmpu 0, 2, 2
	li 8, 0
	bun 0, .Lra_BB17_2
.Lra_BB17_8:
	mr	8, 9
	b .Lra_BB17_2
.Lra_BB17_9:
	addi 1, 1, 32
	blr
.Lra_func_end17:
	.size	ra_k_f2i_r, .Lra_func_end17-.Lra_func_begin17

	.section	.rodata.cst16,"aM",@progbits,16
	.p2align	4, 0x0
.Lra_CPI18_0:
	.long	0x3e800000
	.long	0x3e800000
	.long	0x3e800000
	.long	0x3e800000
.Lra_CPI18_1:
	.long	0x3f000000
	.long	0x3f000000
	.long	0x3f000000
	.long	0x3f000000
	.section	.rodata.cst4,"aM",@progbits,4
	.p2align	2, 0x0
.Lra_CPI18_2:
	.long	0x3e800000
.Lra_CPI18_3:
	.long	0x3f000000
	.section	.text.ra_k_fir3_f32_r,"ax",@progbits
	.globl	ra_k_fir3_f32_r
	.p2align	2
	.type	ra_k_fir3_f32_r,@function
ra_k_fir3_f32_r:
.Lra_func_begin18:
	srawi 6, 5, 31
	andc 7, 5, 6
	clrlwi.	6, 7, 2
	cmpwi 1, 5, 1
	cror 20, 4, 2
	bclr 12, 20, 0
	stwu 1, -64(1)
	cmplwi	6, 4
	li 5, 0
	std 30, 40(1)
	blt	0, .Lra_BB18_7
	slwi 8, 7, 2
	add 9, 4, 8
	addi 9, 9, 8
	cmplw	3, 9
	add 8, 3, 8
	bc 4, 0, .Lra_BB18_4
	cmplw	4, 8
	bc 12, 0, .Lra_BB18_7
.Lra_BB18_4:
	li 11, .Lra_CPI18_0@l
	lis 12, .Lra_CPI18_0@ha
	lvx 2, 12, 11
	li 11, .Lra_CPI18_1@l
	lis 12, .Lra_CPI18_1@ha
	lvx 4, 12, 11
	rlwinm 5, 7, 0, 2, 29
	vspltisw 3, -1
	addi 7, 4, 8
	addi 8, 4, 4
	li 9, 0
	li 10, 15
	vslw 3, 3, 3
	addi 11, 1, 16
	mr	12, 5
.Lra_BB18_5:
	add 30, 4, 9
	lvx 0, 4, 9
	addi 12, 12, -4
	lvx 1, 30, 10
	add 30, 8, 9
	lvx 6, 8, 9
	cmplwi	12, 0
	lvx 7, 30, 10
	add 30, 7, 9
	lvx 8, 7, 9
	lvx 9, 30, 10
	mr	30, 3
	lvsl 5, 4, 9
	vperm 5, 0, 1, 5
	lvsl 10, 8, 9
	vperm 0, 6, 7, 10
	vmaddfp 5, 5, 2, 3
	lvsl 1, 7, 9
	vperm 1, 8, 9, 1
	vmaddfp 0, 0, 4, 3
	vmaddfp 1, 1, 2, 3
	vaddfp 5, 5, 0
	vaddfp 5, 5, 1
	stvx 5, 0, 11
	ld 0, 16(1)
	stdux 0, 30, 9
	ld 0, 24(1)
	addi 9, 9, 16
	std 0, 8(30)
	bne	0, .Lra_BB18_5
	cmplw	6, 5
	beq	0, .Lra_BB18_9
.Lra_BB18_7:
	sub	6, 6, 5
	slwi 5, 5, 2
	addi 5, 5, -4
	lis 7, .Lra_CPI18_2@ha
	add 4, 4, 5
	add 3, 3, 5
	lis 5, .Lra_CPI18_3@ha
	lfs 0, .Lra_CPI18_2@l(7)
	lfs 1, .Lra_CPI18_3@l(5)
.Lra_BB18_8:
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
	bne	0, .Lra_BB18_8
.Lra_BB18_9:
	ld 30, 40(1)
	addi 1, 1, 64
	blr
.Lra_func_end18:
	.size	ra_k_fir3_f32_r, .Lra_func_end18-.Lra_func_begin18

	.section	.text.ra_k_g_add_f32,"ax",@progbits
	.globl	ra_k_g_add_f32
	.p2align	2
	.type	ra_k_g_add_f32,@function
ra_k_g_add_f32:
.Lra_func_begin19:
	lis 4, ra_g_a@ha
	lis 5, ra_g_b@ha
	lis 6, ra_g_c@ha
	li 3, 0
	la 4, ra_g_a@l(4)
	la 5, ra_g_b@l(5)
	la 6, ra_g_c@l(6)
.Lra_BB19_1:
	lvx 2, 4, 3
	lvx 3, 5, 3
	vaddfp 2, 2, 3
	stvx 2, 6, 3
	addi 3, 3, 16
	cmplwi	3, 4096
	bne	0, .Lra_BB19_1
	blr
.Lra_func_end19:
	.size	ra_k_g_add_f32, .Lra_func_end19-.Lra_func_begin19

	.section	.text.ra_k_g_add_i32,"ax",@progbits
	.globl	ra_k_g_add_i32
	.p2align	2
	.type	ra_k_g_add_i32,@function
ra_k_g_add_i32:
.Lra_func_begin20:
	lis 4, ra_g_ia@ha
	lis 5, ra_g_ib@ha
	lis 6, ra_g_ic@ha
	li 3, 0
	la 4, ra_g_ia@l(4)
	la 5, ra_g_ib@l(5)
	la 6, ra_g_ic@l(6)
.Lra_BB20_1:
	lvx 2, 4, 3
	lvx 3, 5, 3
	vadduwm 2, 3, 2
	stvx 2, 6, 3
	addi 3, 3, 16
	cmplwi	3, 4096
	bne	0, .Lra_BB20_1
	blr
.Lra_func_end20:
	.size	ra_k_g_add_i32, .Lra_func_end20-.Lra_func_begin20

	.section	.rodata.cst4,"aM",@progbits,4
	.p2align	2, 0x0
.Lra_CPI21_0:
	.long	0x00000000
	.section	.text.ra_k_g_dot,"ax",@progbits
	.globl	ra_k_g_dot
	.p2align	2
	.type	ra_k_g_dot,@function
ra_k_g_dot:
.Lra_func_begin21:
	lis 6, .Lra_CPI21_0@ha
	lfs 1, .Lra_CPI21_0@l(6)
	lis 4, ra_g_b@ha
	lis 5, ra_g_a@ha
	la 4, ra_g_b@l(4)
	la 5, ra_g_a@l(5)
	li 3, 0
	addi 4, 4, -4
	addi 5, 5, -4
.Lra_BB21_1:
	lfsu 0, 4(4)
	addi 3, 3, 4
	lfsu 2, 4(5)
	cmplwi	3, 4096
	fmuls 0, 2, 0
	fadds 1, 1, 0
	bne	0, .Lra_BB21_1
	blr
.Lra_func_end21:
	.size	ra_k_g_dot, .Lra_func_end21-.Lra_func_begin21

	.section	.text.ra_k_g_saxpy,"ax",@progbits
	.globl	ra_k_g_saxpy
	.p2align	2
	.type	ra_k_g_saxpy,@function
ra_k_g_saxpy:
.Lra_func_begin22:
	stwu 1, -32(1)
	addi 4, 1, 16
	stfs 1, 16(1)
	lis 5, ra_g_a@ha
	lvx 3, 0, 4
	vspltisw 2, -1
	lis 6, ra_g_b@ha
	lis 7, ra_g_c@ha
	li 3, 0
	la 4, ra_g_a@l(5)
	vslw 2, 2, 2
	la 5, ra_g_b@l(6)
	vspltw 3, 3, 0
	la 6, ra_g_c@l(7)
.Lra_BB22_1:
	lvx 4, 4, 3
	lvx 5, 5, 3
	vmaddfp 4, 3, 4, 2
	vaddfp 4, 4, 5
	stvx 4, 6, 3
	addi 3, 3, 16
	cmplwi	3, 4096
	bne	0, .Lra_BB22_1
	addi 1, 1, 32
	blr
.Lra_func_end22:
	.size	ra_k_g_saxpy, .Lra_func_end22-.Lra_func_begin22

	.section	.text.ra_k_gather_f32_r,"ax",@progbits
	.globl	ra_k_gather_f32_r
	.p2align	2
	.type	ra_k_gather_f32_r,@function
ra_k_gather_f32_r:
.Lra_func_begin23:
	cmpwi	6, 1
	bltlr	0
	addi 5, 5, -4
	addi 3, 3, -4
.Lra_BB23_2:
	lwzu 7, 4(5)
	addi 6, 6, -1
	cmplwi	6, 0
	slwi 7, 7, 2
	lwzx 7, 4, 7
	stwu 7, 4(3)
	bne	0, .Lra_BB23_2
	blr
.Lra_func_end23:
	.size	ra_k_gather_f32_r, .Lra_func_end23-.Lra_func_begin23

	.section	.text.ra_k_i2f_r,"ax",@progbits
	.globl	ra_k_i2f_r
	.p2align	2
	.type	ra_k_i2f_r,@function
ra_k_i2f_r:
.Lra_func_begin24:
	cmpwi	5, 1
	bltlr	0
	stwu 1, -32(1)
	addi 4, 4, -4
	addi 3, 3, -4
.Lra_BB24_2:
	lwa 6, 4(4)
	addi 5, 5, -1
	cmplwi	5, 0
	addi 4, 4, 4
	std 6, 24(1)
	lfd 0, 24(1)
	fcfid 0, 0
	frsp 0, 0
	stfsu 0, 4(3)
	bne	0, .Lra_BB24_2
	addi 1, 1, 32
	blr
.Lra_func_end24:
	.size	ra_k_i2f_r, .Lra_func_end24-.Lra_func_begin24

	.section	.rodata.cst16,"aM",@progbits,16
	.p2align	4, 0x0
.Lra_CPI25_0:
	.byte	0
	.byte	0
	.byte	0
	.byte	0
	.byte	0
	.byte	0
	.byte	0
	.byte	0
	.byte	0
	.byte	1
	.byte	2
	.byte	3
	.byte	16
	.byte	17
	.byte	18
	.byte	19
.Lra_CPI25_1:
	.byte	0
	.byte	1
	.byte	2
	.byte	3
	.byte	4
	.byte	5
	.byte	6
	.byte	7
	.byte	24
	.byte	25
	.byte	26
	.byte	27
	.byte	28
	.byte	29
	.byte	30
	.byte	31
.Lra_CPI25_2:
	.byte	4
	.byte	5
	.byte	6
	.byte	7
	.byte	20
	.byte	21
	.byte	22
	.byte	23
	.byte	0
	.byte	0
	.byte	0
	.byte	0
	.byte	0
	.byte	0
	.byte	0
	.byte	0
.Lra_CPI25_3:
	.byte	0
	.byte	0
	.byte	0
	.byte	0
	.byte	0
	.byte	0
	.byte	0
	.byte	0
	.byte	8
	.byte	9
	.byte	10
	.byte	11
	.byte	24
	.byte	25
	.byte	26
	.byte	27
.Lra_CPI25_4:
	.byte	12
	.byte	13
	.byte	14
	.byte	15
	.byte	28
	.byte	29
	.byte	30
	.byte	31
	.byte	0
	.byte	0
	.byte	0
	.byte	0
	.byte	0
	.byte	0
	.byte	0
	.byte	0
	.section	.text.ra_k_mat4_vec4_r,"ax",@progbits
	.globl	ra_k_mat4_vec4_r
	.p2align	2
	.type	ra_k_mat4_vec4_r,@function
ra_k_mat4_vec4_r:
.Lra_func_begin25:
	cmpwi	6, 1
	bltlr	0
	stwu 1, -464(1)
	cmplwi	6, 4
	li 7, 0
	std 25, 400(1)
	std 26, 408(1)
	std 27, 416(1)
	std 28, 424(1)
	std 29, 432(1)
	std 30, 440(1)
	stfd 29, 376(1)
	stfd 30, 384(1)
	stfd 31, 392(1)
	bge 0, .Lra_BB25_5
.Lra_BB25_2:
	sub	6, 6, 7
	slwi 7, 7, 4
	addi 7, 7, -16
	add 5, 5, 7
	add 3, 3, 7
.Lra_BB25_3:
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
	bne	0, .Lra_BB25_3
.Lra_BB25_4:
	lfd 31, 392(1)
	ld 30, 440(1)
	ld 29, 432(1)
	ld 28, 424(1)
	ld 27, 416(1)
	ld 26, 408(1)
	ld 25, 400(1)
	lfd 30, 384(1)
	lfd 29, 376(1)
	addi 1, 1, 464
	blr
.Lra_BB25_5:
	slwi 9, 6, 4
	addi 10, 4, 64
	add 8, 3, 9
	add 9, 5, 9
	cmplw 5, 3, 9
	cmplw	3, 10
	cmplw 1, 4, 8
	bc 4, 20, .Lra_BB25_7
	cmplw 5, 5, 8
	bc 12, 20, .Lra_BB25_2
.Lra_BB25_7:
	crand 20, 0, 4
	bc 12, 20, .Lra_BB25_2
	lwz 7, 60(4)
	li 8, .Lra_CPI25_0@l
	lis 9, .Lra_CPI25_0@ha
	lvx 2, 9, 8
	li 8, .Lra_CPI25_2@l
	stw 7, 112(1)
	lis 9, .Lra_CPI25_2@ha
	lwz 7, 44(4)
	li 10, .Lra_CPI25_1@l
	lvx 4, 9, 8
	addi 8, 1, 112
	lvx 6, 0, 8
	addi 8, 1, 128
	stw 7, 128(1)
	addi 9, 1, 272
	lwz 7, 28(4)
	lis 11, .Lra_CPI25_1@ha
	lvx 7, 0, 8
	addi 8, 1, 144
	stw 7, 144(1)
	addi 12, 1, 320
	lwz 7, 12(4)
	addi 0, 1, 336
	lvx 8, 0, 8
	addi 8, 1, 160
	stw 7, 160(1)
	addi 30, 1, 352
	lwz 7, 56(4)
	lis 27, .Lra_CPI25_3@ha
	lvx 9, 0, 8
	addi 8, 1, 176
	stw 7, 176(1)
	li 26, .Lra_CPI25_4@l
	lwz 7, 40(4)
	vspltisw 1, -1
	vspltw 6, 6, 0
	vspltw 7, 7, 0
	vspltw 8, 8, 0
	vspltw 9, 9, 0
	lvx 10, 0, 8
	addi 8, 1, 192
	stw 7, 192(1)
	vslw 1, 1, 1
	lwz 7, 24(4)
	vspltw 10, 10, 0
	lvx 11, 0, 8
	addi 8, 1, 208
	stw 7, 208(1)
	lwz 7, 8(4)
	vspltw 11, 11, 0
	lvx 12, 0, 8
	addi 8, 1, 224
	stw 7, 224(1)
	lwz 7, 52(4)
	vspltw 12, 12, 0
	lvx 13, 0, 8
	addi 8, 1, 240
	stw 7, 240(1)
	lwz 7, 36(4)
	vspltw 13, 13, 0
	lvx 14, 0, 8
	addi 8, 1, 256
	stw 7, 256(1)
	lwz 7, 20(4)
	vspltw 14, 14, 0
	lvx 15, 0, 8
	stw 7, 272(1)
	rlwinm 7, 6, 0, 1, 29
	lwz 8, 4(4)
	vspltw 15, 15, 0
	lvx 16, 0, 9
	addi 9, 1, 288
	stw 8, 288(1)
	lwz 8, 48(4)
	vspltw 16, 16, 0
	lvx 17, 0, 9
	li 9, 32
	stw 8, 304(1)
	li 8, 0
	lvx 3, 11, 10
	addi 10, 1, 304
	lwz 11, 32(4)
	vspltw 17, 17, 0
	lvx 18, 0, 10
	li 10, 16
	stw 11, 320(1)
	li 11, 63
	lwz 29, 16(4)
	vspltw 18, 18, 0
	lvx 19, 0, 12
	li 12, 48
	stw 29, 336(1)
	li 29, .Lra_CPI25_3@l
	lwz 28, 0(4)
	vspltw 19, 19, 0
	lvx 31, 0, 0
	addi 0, 1, 48
	stw 28, 352(1)
	lis 28, .Lra_CPI25_4@ha
	lvx 30, 0, 30
	addi 30, 1, 96
	lvx 5, 27, 29
	addi 29, 1, 80
	lvx 0, 28, 26
	li 28, 32
	stvx 6, 1, 28
	vspltw 31, 31, 0
	vspltw 30, 30, 0
	addi 28, 1, 64
	mr	27, 7
.Lra_BB25_9:
	add 26, 5, 8
	lvx 29, 26, 9
	mr	25, 3
	lvx 28, 26, 10
	addi 27, 27, -4
	lvx 27, 5, 8
	cmplwi	27, 0
	lvx 26, 26, 11
	lvx 25, 26, 12
	li 26, 32
	lvsl 24, 5, 8
	vperm 23, 28, 29, 24
	vperm 28, 27, 28, 24
	vperm 27, 25, 26, 24
	vperm 29, 29, 25, 24
	vmrghw 25, 28, 23
	vperm 24, 28, 23, 4
	vmrglw 20, 28, 23
	vperm 28, 28, 23, 0
	vperm 26, 29, 27, 2
	vmrghw 22, 29, 27
	vperm 21, 29, 27, 5
	vmrglw 29, 29, 27
	vperm 26, 25, 26, 3
	vperm 25, 24, 22, 3
	vperm 27, 20, 21, 3
	vperm 29, 28, 29, 3
	vmaddfp 24, 26, 30, 1
	vmaddfp 23, 25, 31, 1
	vmaddfp 22, 26, 17, 1
	vmaddfp 21, 25, 16, 1
	vmaddfp 20, 26, 13, 1
	vmaddfp 6, 25, 12, 1
	vmaddfp 26, 26, 9, 1
	vmaddfp 25, 25, 8, 1
	vaddfp 28, 24, 23
	vaddfp 24, 22, 21
	vmaddfp 22, 27, 15, 1
	vmaddfp 23, 27, 19, 1
	vaddfp 6, 20, 6
	vaddfp 26, 26, 25
	vmaddfp 25, 27, 11, 1
	vmaddfp 27, 27, 7, 1
	vaddfp 24, 24, 22
	lvx 22, 1, 26
	vaddfp 28, 28, 23
	vmaddfp 23, 29, 18, 1
	vaddfp 6, 6, 25
	vmaddfp 25, 29, 14, 1
	vaddfp 27, 26, 27
	vmaddfp 26, 29, 10, 1
	vaddfp 28, 28, 23
	vaddfp 25, 24, 25
	vaddfp 6, 6, 26
	vmrghw 26, 28, 25
	vperm 24, 28, 25, 0
	vmaddfp 29, 29, 22, 1
	vaddfp 29, 27, 29
	vperm 27, 6, 29, 2
	vmrglw 23, 6, 29
	vperm 27, 26, 27, 3
	vmrglw 26, 28, 25
	vperm 28, 28, 25, 4
	vperm 24, 24, 23, 3
	stvx 27, 0, 0
	vperm 27, 6, 29, 5
	vmrghw 6, 6, 29
	vperm 29, 26, 27, 3
	vperm 6, 28, 6, 3
	stvx 24, 0, 30
	stvx 29, 0, 29
	stvx 6, 0, 28
	ld 26, 48(1)
	stdux 26, 25, 8
	ld 26, 104(1)
	addi 8, 8, 64
	std 26, 56(25)
	ld 26, 96(1)
	std 26, 48(25)
	ld 26, 88(1)
	std 26, 40(25)
	ld 26, 80(1)
	std 26, 32(25)
	ld 26, 72(1)
	std 26, 24(25)
	ld 26, 64(1)
	std 26, 16(25)
	ld 26, 56(1)
	std 26, 8(25)
	bne	0, .Lra_BB25_9
	cmplw	6, 7
	bne	0, .Lra_BB25_2
	b .Lra_BB25_4
.Lra_func_end25:
	.size	ra_k_mat4_vec4_r, .Lra_func_end25-.Lra_func_begin25

	.section	.text.ra_k_max_f32,"ax",@progbits
	.globl	ra_k_max_f32
	.p2align	2
	.type	ra_k_max_f32,@function
ra_k_max_f32:
.Lra_func_begin26:
	mflr 0
	stw 0, -8(1)
	stwu 1, -96(1)
	cmpwi	4, 1
	blt-	0, .Lra_BB26_8
	lfs 0, 0(3)
	slwi 4, 4, 2
	cmplwi	4, 4
	bne	0, .Lra_BB26_4
	fmr 1, 0
.Lra_BB26_3:
	addi 1, 1, 96
	lwz 0, -8(1)
	mtlr 0
	blr
.Lra_BB26_4:
	addi 4, 4, -4
	b .Lra_BB26_6
.Lra_BB26_5:
	addi 4, 4, -4
	cmplwi	4, 0
	fmr 0, 1
	beq	0, .Lra_BB26_3
.Lra_BB26_6:
	lfsu 1, 4(3)
	fcmpu 0, 1, 0
	bgt	0, .Lra_BB26_5
	fmr 1, 0
	b .Lra_BB26_5
.Lra_BB26_8:
	lis 3, .Lra__MergedGlobals@ha
	la 5, .Lra__MergedGlobals@l(3)
	li 3, 0
	li 4, 0
	bl _RNvNtCsedlPyG8v7DD_4core9panicking18panic_bounds_check
.Lra_func_end26:
	.size	ra_k_max_f32, .Lra_func_end26-.Lra_func_begin26

	.section	.text.ra_k_max_i32_r,"ax",@progbits
	.globl	ra_k_max_i32_r
	.p2align	2
	.type	ra_k_max_i32_r,@function
ra_k_max_i32_r:
.Lra_func_begin27:
	cmpwi	6, 1
	bltlr	0
	stwu 1, -64(1)
	cmplwi	6, 8
	li 7, 0
	std 30, 40(1)
	bge 0, .Lra_BB27_6
.Lra_BB27_2:
	sub	6, 6, 7
	slwi 7, 7, 2
	addi 7, 7, -4
	add 4, 4, 7
	add 5, 5, 7
	add 3, 3, 7
	b .Lra_BB27_4
.Lra_BB27_3:
	addi 6, 6, -1
	cmplwi	6, 0
	stwu 7, 4(3)
	beq	0, .Lra_BB27_11
.Lra_BB27_4:
	lwzu 7, 4(4)
	lwzu 8, 4(5)
	cmpw	7, 8
	bgt	0, .Lra_BB27_3
	mr	7, 8
	b .Lra_BB27_3
.Lra_BB27_6:
	sub	8, 3, 4
	cmplwi	8, 16
	sub	8, 3, 5
	bc 12, 0, .Lra_BB27_2
	cmplwi	8, 16
	bc 12, 0, .Lra_BB27_2
	rlwinm 7, 6, 0, 1, 29
	li 8, 0
	li 9, 15
	addi 10, 1, 16
	mr	11, 7
.Lra_BB27_9:
	add 12, 4, 8
	lvx 2, 4, 8
	mr	30, 3
	lvx 3, 12, 9
	add 12, 5, 8
	lvx 4, 5, 8
	addi 11, 11, -4
	lvx 5, 12, 9
	cmplwi	11, 0
	lvsl 0, 4, 8
	vperm 2, 2, 3, 0
	lvsl 1, 5, 8
	vperm 3, 4, 5, 1
	vmaxsw 2, 2, 3
	stvx 2, 0, 10
	ld 12, 16(1)
	stdux 12, 30, 8
	ld 12, 24(1)
	addi 8, 8, 16
	std 12, 8(30)
	bne	0, .Lra_BB27_9
	cmplw	6, 7
	bne	0, .Lra_BB27_2
.Lra_BB27_11:
	ld 30, 40(1)
	addi 1, 1, 64
	blr
.Lra_func_end27:
	.size	ra_k_max_i32_r, .Lra_func_end27-.Lra_func_begin27

	.section	.text.ra_k_min_f32_r,"ax",@progbits
	.globl	ra_k_min_f32_r
	.p2align	2
	.type	ra_k_min_f32_r,@function
ra_k_min_f32_r:
.Lra_func_begin28:
	cmpwi	6, 1
	bltlr	0
	stwu 1, -64(1)
	cmplwi	6, 8
	li 7, 0
	std 30, 40(1)
	bge 0, .Lra_BB28_6
.Lra_BB28_2:
	sub	6, 6, 7
	slwi 7, 7, 2
	addi 7, 7, -4
	add 4, 4, 7
	add 5, 5, 7
	add 3, 3, 7
	b .Lra_BB28_4
.Lra_BB28_3:
	addi 6, 6, -1
	cmplwi	6, 0
	stfsu 0, 4(3)
	beq	0, .Lra_BB28_11
.Lra_BB28_4:
	lfsu 0, 4(4)
	lfsu 1, 4(5)
	fcmpu 0, 0, 1
	blt	0, .Lra_BB28_3
	fmr 0, 1
	b .Lra_BB28_3
.Lra_BB28_6:
	sub	8, 3, 4
	cmplwi	8, 16
	sub	8, 3, 5
	bc 12, 0, .Lra_BB28_2
	cmplwi	8, 16
	bc 12, 0, .Lra_BB28_2
	rlwinm 7, 6, 0, 1, 29
	li 8, 0
	li 9, 15
	addi 10, 1, 16
	mr	11, 7
.Lra_BB28_9:
	add 12, 4, 8
	lvx 2, 4, 8
	mr	30, 3
	lvx 3, 12, 9
	add 12, 5, 8
	lvx 4, 5, 8
	addi 11, 11, -4
	lvx 5, 12, 9
	cmplwi	11, 0
	lvsl 0, 4, 8
	vperm 2, 2, 3, 0
	lvsl 1, 5, 8
	vperm 3, 4, 5, 1
	vcmpgtfp 4, 3, 2
	vsel 2, 3, 2, 4
	stvx 2, 0, 10
	ld 12, 16(1)
	stdux 12, 30, 8
	ld 12, 24(1)
	addi 8, 8, 16
	std 12, 8(30)
	bne	0, .Lra_BB28_9
	cmplw	6, 7
	bne	0, .Lra_BB28_2
.Lra_BB28_11:
	ld 30, 40(1)
	addi 1, 1, 64
	blr
.Lra_func_end28:
	.size	ra_k_min_f32_r, .Lra_func_end28-.Lra_func_begin28

	.section	.text.ra_k_mul_f32_r,"ax",@progbits
	.globl	ra_k_mul_f32_r
	.p2align	2
	.type	ra_k_mul_f32_r,@function
ra_k_mul_f32_r:
.Lra_func_begin29:
	cmpwi	6, 1
	bltlr	0
	stwu 1, -64(1)
	cmplwi	6, 8
	li 7, 0
	std 30, 40(1)
	bge 0, .Lra_BB29_5
.Lra_BB29_2:
	sub	6, 6, 7
	slwi 7, 7, 2
	addi 7, 7, -4
	add 4, 4, 7
	add 5, 5, 7
	add 3, 3, 7
.Lra_BB29_3:
	lfsu 0, 4(4)
	addi 6, 6, -1
	lfsu 1, 4(5)
	cmplwi	6, 0
	fmuls 0, 0, 1
	stfsu 0, 4(3)
	bne	0, .Lra_BB29_3
.Lra_BB29_4:
	ld 30, 40(1)
	addi 1, 1, 64
	blr
.Lra_BB29_5:
	sub	8, 3, 4
	cmplwi	8, 16
	sub	8, 3, 5
	bc 12, 0, .Lra_BB29_2
	cmplwi	8, 16
	bc 12, 0, .Lra_BB29_2
	rlwinm 7, 6, 0, 1, 29
	vspltisw 2, -1
	li 8, 0
	li 9, 15
	vslw 2, 2, 2
	addi 10, 1, 16
	mr	11, 7
.Lra_BB29_8:
	add 12, 4, 8
	lvx 3, 4, 8
	mr	30, 3
	lvx 4, 12, 9
	add 12, 5, 8
	lvx 5, 5, 8
	addi 11, 11, -4
	lvx 0, 12, 9
	cmplwi	11, 0
	lvsl 1, 4, 8
	vperm 3, 3, 4, 1
	lvsl 6, 5, 8
	vperm 4, 5, 0, 6
	vmaddfp 3, 3, 4, 2
	stvx 3, 0, 10
	ld 12, 16(1)
	stdux 12, 30, 8
	ld 12, 24(1)
	addi 8, 8, 16
	std 12, 8(30)
	bne	0, .Lra_BB29_8
	cmplw	6, 7
	bne	0, .Lra_BB29_2
	b .Lra_BB29_4
.Lra_func_end29:
	.size	ra_k_mul_f32_r, .Lra_func_end29-.Lra_func_begin29

	.section	.text.ra_k_mul_i32_r,"ax",@progbits
	.globl	ra_k_mul_i32_r
	.p2align	2
	.type	ra_k_mul_i32_r,@function
ra_k_mul_i32_r:
.Lra_func_begin30:
	cmpwi	6, 1
	bltlr	0
	addi 4, 4, -4
	addi 5, 5, -4
	addi 3, 3, -4
.Lra_BB30_2:
	lwzu 7, 4(4)
	addi 6, 6, -1
	lwzu 8, 4(5)
	cmplwi	6, 0
	mullw 7, 8, 7
	stwu 7, 4(3)
	bne	0, .Lra_BB30_2
	blr
.Lra_func_end30:
	.size	ra_k_mul_i32_r, .Lra_func_end30-.Lra_func_begin30

	.section	.text.ra_k_offset1_f32_r,"ax",@progbits
	.globl	ra_k_offset1_f32_r
	.p2align	2
	.type	ra_k_offset1_f32_r,@function
ra_k_offset1_f32_r:
.Lra_func_begin31:
	srawi 6, 5, 31
	andc 7, 5, 6
	clrlwi.	6, 7, 2
	cmpwi 1, 5, 1
	cror 20, 4, 2
	bclr 12, 20, 0
	stwu 1, -64(1)
	cmplwi	6, 8
	li 5, 0
	std 30, 40(1)
	blt	0, .Lra_BB31_7
	slwi 8, 7, 2
	add 9, 4, 8
	addi 9, 9, 4
	cmplw	3, 9
	add 8, 3, 8
	bc 4, 0, .Lra_BB31_4
	cmplw	4, 8
	bc 12, 0, .Lra_BB31_7
.Lra_BB31_4:
	rlwinm 5, 7, 0, 2, 29
	addi 7, 4, 4
	li 8, 0
	li 9, 15
	addi 10, 1, 16
	mr	11, 5
.Lra_BB31_5:
	add 12, 4, 8
	lvx 2, 4, 8
	mr	30, 3
	lvx 3, 12, 9
	add 12, 7, 8
	lvx 4, 7, 8
	addi 11, 11, -4
	lvx 5, 12, 9
	cmplwi	11, 0
	lvsl 0, 4, 8
	vperm 2, 2, 3, 0
	lvsl 1, 7, 8
	vperm 3, 4, 5, 1
	vaddfp 2, 2, 3
	stvx 2, 0, 10
	ld 12, 16(1)
	stdux 12, 30, 8
	ld 12, 24(1)
	addi 8, 8, 16
	std 12, 8(30)
	bne	0, .Lra_BB31_5
	cmplw	6, 5
	beq	0, .Lra_BB31_9
.Lra_BB31_7:
	sub	6, 6, 5
	slwi 5, 5, 2
	addi 5, 5, -4
	add 4, 4, 5
	add 3, 3, 5
.Lra_BB31_8:
	lfsu 0, 4(4)
	addi 6, 6, -1
	cmplwi	6, 0
	lfs 1, 4(4)
	fadds 0, 0, 1
	stfsu 0, 4(3)
	bne	0, .Lra_BB31_8
.Lra_BB31_9:
	ld 30, 40(1)
	addi 1, 1, 64
	blr
.Lra_func_end31:
	.size	ra_k_offset1_f32_r, .Lra_func_end31-.Lra_func_begin31

	.section	.text.ra_k_prefix_f32,"ax",@progbits
	.globl	ra_k_prefix_f32
	.p2align	2
	.type	ra_k_prefix_f32,@function
ra_k_prefix_f32:
.Lra_func_begin32:
	cmpwi	4, 2
	bltlr	0
	lfs 0, 0(3)
	addi 4, 4, -1
.Lra_BB32_2:
	lfsu 1, 4(3)
	addi 4, 4, -1
	cmplwi	4, 0
	fadds 0, 0, 1
	stfs 0, 0(3)
	bne	0, .Lra_BB32_2
	blr
.Lra_func_end32:
	.size	ra_k_prefix_f32, .Lra_func_end32-.Lra_func_begin32

	.section	.text.ra_k_saxpy,"ax",@progbits
	.globl	ra_k_saxpy
	.p2align	2
	.type	ra_k_saxpy,@function
ra_k_saxpy:
.Lra_func_begin33:
	cmpwi	6, 1
	bltlr	0
	stwu 1, -48(1)
	cmplwi	6, 8
	li 5, 0
	blt	0, .Lra_BB33_7
	slwi 7, 6, 2
	add 8, 4, 7
	cmplw	3, 8
	add 7, 3, 7
	bc 4, 0, .Lra_BB33_4
	cmplw	4, 7
	bc 12, 0, .Lra_BB33_7
.Lra_BB33_4:
	stfs 1, 32(1)
	addi 5, 1, 32
	lvx 3, 0, 5
	rlwinm 5, 6, 0, 1, 29
	vspltisw 2, -1
	li 7, 0
	li 8, 15
	vslw 2, 2, 2
	vspltw 3, 3, 0
	addi 9, 1, 16
	mr	10, 5
.Lra_BB33_5:
	add 11, 4, 7
	lvx 4, 4, 7
	addi 10, 10, -4
	lvx 5, 11, 8
	add 11, 3, 7
	lvx 0, 3, 7
	cmplwi	10, 0
	lvx 1, 11, 8
	lvsl 6, 4, 7
	vperm 4, 4, 5, 6
	lvsl 5, 3, 7
	vperm 5, 0, 1, 5
	vmaddfp 4, 3, 4, 2
	vaddfp 4, 4, 5
	stvx 4, 0, 9
	ld 12, 16(1)
	stdx 12, 3, 7
	addi 7, 7, 16
	ld 12, 24(1)
	std 12, 8(11)
	bne	0, .Lra_BB33_5
	cmplw	6, 5
	beq	0, .Lra_BB33_9
.Lra_BB33_7:
	slwi 7, 5, 2
	sub	5, 6, 5
	addi 6, 7, -4
	add 4, 4, 6
	add 3, 3, 6
.Lra_BB33_8:
	lfsu 0, 4(4)
	addi 5, 5, -1
	lfsu 2, 4(3)
	cmplwi	5, 0
	fmuls 0, 1, 0
	fadds 0, 0, 2
	stfs 0, 0(3)
	bne	0, .Lra_BB33_8
.Lra_BB33_9:
	addi 1, 1, 48
	blr
.Lra_func_end33:
	.size	ra_k_saxpy, .Lra_func_end33-.Lra_func_begin33

	.section	.text.ra_k_saxpy_r,"ax",@progbits
	.globl	ra_k_saxpy_r
	.p2align	2
	.type	ra_k_saxpy_r,@function
ra_k_saxpy_r:
.Lra_func_begin34:
	cmpwi	6, 1
	bltlr	0
	stwu 1, -48(1)
	cmplwi	6, 8
	li 5, 0
	blt	0, .Lra_BB34_7
	slwi 7, 6, 2
	add 8, 4, 7
	cmplw	3, 8
	add 7, 3, 7
	bc 4, 0, .Lra_BB34_4
	cmplw	4, 7
	bc 12, 0, .Lra_BB34_7
.Lra_BB34_4:
	stfs 1, 32(1)
	addi 5, 1, 32
	lvx 3, 0, 5
	rlwinm 5, 6, 0, 1, 29
	vspltisw 2, -1
	li 7, 0
	li 8, 15
	vslw 2, 2, 2
	vspltw 3, 3, 0
	addi 9, 1, 16
	mr	10, 5
.Lra_BB34_5:
	add 11, 4, 7
	lvx 4, 4, 7
	addi 10, 10, -4
	lvx 5, 11, 8
	add 11, 3, 7
	lvx 0, 3, 7
	cmplwi	10, 0
	lvx 1, 11, 8
	lvsl 6, 4, 7
	vperm 4, 4, 5, 6
	lvsl 5, 3, 7
	vperm 5, 0, 1, 5
	vmaddfp 4, 3, 4, 2
	vaddfp 4, 4, 5
	stvx 4, 0, 9
	ld 12, 16(1)
	stdx 12, 3, 7
	addi 7, 7, 16
	ld 12, 24(1)
	std 12, 8(11)
	bne	0, .Lra_BB34_5
	cmplw	6, 5
	beq	0, .Lra_BB34_9
.Lra_BB34_7:
	sub	6, 6, 5
	slwi 5, 5, 2
	addi 5, 5, -4
	add 4, 4, 5
	add 3, 3, 5
.Lra_BB34_8:
	lfsu 0, 4(4)
	addi 6, 6, -1
	lfsu 2, 4(3)
	cmplwi	6, 0
	fmuls 0, 1, 0
	fadds 0, 0, 2
	stfs 0, 0(3)
	bne	0, .Lra_BB34_8
.Lra_BB34_9:
	addi 1, 1, 48
	blr
.Lra_func_end34:
	.size	ra_k_saxpy_r, .Lra_func_end34-.Lra_func_begin34

	.section	.text.ra_k_scale_inplace,"ax",@progbits
	.globl	ra_k_scale_inplace
	.p2align	2
	.type	ra_k_scale_inplace,@function
ra_k_scale_inplace:
.Lra_func_begin35:
	cmpwi	5, 1
	bltlr	0
	srawi 4, 5, 31
	andc 4, 5, 4
	slwi 4, 4, 2
	addi 6, 4, -4
	cmplwi	6, 0
	mr	5, 3
	beq	0, .Lra_BB35_5
	srwi 5, 6, 2
	addi 6, 5, 1
	rlwinm 7, 6, 0, 1, 30
	rlwinm 5, 6, 2, 0, 28
	add 5, 3, 5
	addi 8, 3, -8
	mr	9, 7
.Lra_BB35_3:
	lfsu 0, 8(8)
	addi 9, 9, -2
	cmplwi	9, 0
	lfs 2, 4(8)
	fmuls 0, 1, 0
	stfs 0, 0(8)
	fmuls 0, 1, 2
	stfs 0, 4(8)
	bne	0, .Lra_BB35_3
	cmplw	6, 7
	beqlr	0
.Lra_BB35_5:
	add 3, 3, 4
	addi 4, 5, -4
.Lra_BB35_6:
	lfsu 0, 4(4)
	addi 5, 4, 4
	fmuls 0, 1, 0
	cmplw	5, 3
	stfs 0, 0(4)
	bne	0, .Lra_BB35_6
	blr
.Lra_func_end35:
	.size	ra_k_scale_inplace, .Lra_func_end35-.Lra_func_begin35

	.section	.text.ra_k_set_f32,"ax",@progbits
	.globl	ra_k_set_f32
	.p2align	2
	.type	ra_k_set_f32,@function
ra_k_set_f32:
.Lra_func_begin36:
	cmpwi	5, 1
	bltlr	0
	li 4, 0
	ble 0, .Lra_BB36_5
	rlwinm 4, 5, 0, 1, 30
	addi 6, 3, -8
	mr	7, 4
.Lra_BB36_3:
	addi 7, 7, -2
	stfsu 1, 8(6)
	cmplwi	7, 0
	stfs 1, 4(6)
	bne	0, .Lra_BB36_3
	cmplw	5, 4
	beqlr	0
.Lra_BB36_5:
	slwi 6, 4, 2
	add 3, 6, 3
	sub	4, 5, 4
	addi 3, 3, -4
.Lra_BB36_6:
	addi 4, 4, -1
	cmplwi	4, 0
	stfsu 1, 4(3)
	bne	0, .Lra_BB36_6
	blr
.Lra_func_end36:
	.size	ra_k_set_f32, .Lra_func_end36-.Lra_func_begin36

	.section	.text.ra_k_set_u32,"ax",@progbits
	.globl	ra_k_set_u32
	.p2align	2
	.type	ra_k_set_u32,@function
ra_k_set_u32:
.Lra_func_begin37:
	cmpwi	5, 1
	bltlr	0
	li 6, 0
	ble 0, .Lra_BB37_5
	rlwinm 6, 5, 0, 1, 30
	addi 7, 3, -8
	mr	8, 6
.Lra_BB37_3:
	addi 8, 8, -2
	stwu 4, 8(7)
	cmplwi	8, 0
	stw 4, 4(7)
	bne	0, .Lra_BB37_3
	cmplw	5, 6
	beqlr	0
.Lra_BB37_5:
	slwi 7, 6, 2
	add 3, 7, 3
	sub	5, 5, 6
	addi 3, 3, -4
.Lra_BB37_6:
	addi 5, 5, -1
	cmplwi	5, 0
	stwu 4, 4(3)
	bne	0, .Lra_BB37_6
	blr
.Lra_func_end37:
	.size	ra_k_set_u32, .Lra_func_end37-.Lra_func_begin37

	.section	.text.ra_k_shl_i32_r,"ax",@progbits
	.globl	ra_k_shl_i32_r
	.p2align	2
	.type	ra_k_shl_i32_r,@function
ra_k_shl_i32_r:
.Lra_func_begin38:
	cmpwi	5, 1
	bltlr	0
	addi 4, 4, -4
	addi 3, 3, -4
.Lra_BB38_2:
	lwzu 6, 4(4)
	addi 5, 5, -1
	cmplwi	5, 0
	slwi 6, 6, 3
	stwu 6, 4(3)
	bne	0, .Lra_BB38_2
	blr
.Lra_func_end38:
	.size	ra_k_shl_i32_r, .Lra_func_end38-.Lra_func_begin38

	.section	.rodata.cst16,"aM",@progbits,16
	.p2align	4, 0x0
.Lra_CPI39_0:
	.byte	0
	.byte	1
	.byte	2
	.byte	3
	.byte	8
	.byte	9
	.byte	10
	.byte	11
	.byte	16
	.byte	17
	.byte	18
	.byte	19
	.byte	24
	.byte	25
	.byte	26
	.byte	27
.Lra_CPI39_1:
	.byte	4
	.byte	5
	.byte	6
	.byte	7
	.byte	12
	.byte	13
	.byte	14
	.byte	15
	.byte	20
	.byte	21
	.byte	22
	.byte	23
	.byte	28
	.byte	29
	.byte	30
	.byte	31
	.section	.text.ra_k_stride2_f32_r,"ax",@progbits
	.globl	ra_k_stride2_f32_r
	.p2align	2
	.type	ra_k_stride2_f32_r,@function
ra_k_stride2_f32_r:
.Lra_func_begin39:
	srawi 6, 5, 31
	andc 7, 5, 6
	clrlwi.	5, 7, 2
	beqlr	0
	stwu 1, -32(1)
	cmplwi	5, 8
	li 6, 0
	blt	0, .Lra_BB39_7
	slwi 9, 7, 3
	slwi 8, 7, 2
	add 9, 4, 9
	cmplw	3, 9
	add 8, 3, 8
	bc 4, 0, .Lra_BB39_4
	cmplw	4, 8
	bc 12, 0, .Lra_BB39_7
.Lra_BB39_4:
	li 9, .Lra_CPI39_0@l
	lis 10, .Lra_CPI39_0@ha
	li 11, .Lra_CPI39_1@l
	lvx 2, 10, 9
	lis 9, .Lra_CPI39_1@ha
	lvx 3, 9, 11
	rlwinm 6, 7, 0, 2, 29
	li 7, 31
	li 8, 16
	addi 9, 1, 16
	mr	10, 3
	mr	11, 4
	mr	12, 6
.Lra_BB39_5:
	lvx 4, 11, 7
	addi 12, 12, -4
	lvx 5, 11, 8
	cmplwi	12, 0
	lvx 0, 0, 11
	lvsl 1, 0, 11
	vperm 4, 5, 4, 1
	addi 11, 11, 32
	vperm 5, 0, 5, 1
	vperm 0, 5, 4, 2
	vperm 4, 5, 4, 3
	vaddfp 4, 0, 4
	stvx 4, 0, 9
	ld 0, 24(1)
	std 0, 8(10)
	ld 0, 16(1)
	std 0, 0(10)
	addi 10, 10, 16
	bne	0, .Lra_BB39_5
	cmplw	5, 6
	beq	0, .Lra_BB39_9
.Lra_BB39_7:
	sub	5, 5, 6
	slwi 7, 6, 2
	slwi 6, 6, 3
	add 4, 6, 4
	add 6, 7, 3
	addi 3, 4, -8
	addi 4, 6, -4
.Lra_BB39_8:
	lfsu 0, 8(3)
	addi 5, 5, -1
	cmplwi	5, 0
	lfs 1, 4(3)
	fadds 0, 0, 1
	stfsu 0, 4(4)
	bne	0, .Lra_BB39_8
.Lra_BB39_9:
	addi 1, 1, 32
	blr
.Lra_func_end39:
	.size	ra_k_stride2_f32_r, .Lra_func_end39-.Lra_func_begin39

	.section	.rodata.cst4,"aM",@progbits,4
	.p2align	2, 0x0
.Lra_CPI40_0:
	.long	0x00000000
	.section	.text.ra_k_sum_f32,"ax",@progbits
	.globl	ra_k_sum_f32
	.p2align	2
	.type	ra_k_sum_f32,@function
ra_k_sum_f32:
.Lra_func_begin40:
	cmpwi	4, 1
	lis 5, .Lra_CPI40_0@ha
	blt	0, .Lra_BB40_4
	lfs 1, .Lra_CPI40_0@l(5)
	srawi 6, 4, 31
	andc 4, 4, 6
	addi 3, 3, -4
	slwi 4, 4, 2
.Lra_BB40_2:
	lfsu 0, 4(3)
	addi 4, 4, -4
	cmplwi	4, 0
	fadds 1, 1, 0
	bne	0, .Lra_BB40_2
	blr
.Lra_BB40_4:
	lfs 1, .Lra_CPI40_0@l(5)
	blr
.Lra_func_end40:
	.size	ra_k_sum_f32, .Lra_func_end40-.Lra_func_begin40

	.section	.text.ra_k_sum_i32,"ax",@progbits
	.globl	ra_k_sum_i32
	.p2align	2
	.type	ra_k_sum_i32,@function
ra_k_sum_i32:
.Lra_func_begin41:
	cmpwi	4, 1
	blt	0, .Lra_BB41_8
	stwu 1, -32(1)
	srawi 5, 4, 31
	andc 4, 4, 5
	slwi 5, 4, 2
	addi 7, 5, -4
	cmplwi	7, 28
	li 4, 0
	mr	6, 3
	blt	0, .Lra_BB41_5
	srwi 4, 7, 2
	addi 4, 4, 1
	rlwinm 7, 4, 0, 1, 28
	rlwinm 6, 4, 2, 0, 26
	addi 8, 3, 16
	vxor 2, 2, 2
	li 9, 0
	add 6, 3, 6
	li 10, 15
	mr	11, 7
	vxor 3, 3, 3
.Lra_BB41_3:
	add 12, 3, 9
	lvx 4, 3, 9
	addi 11, 11, -8
	lvx 5, 12, 10
	add 12, 8, 9
	lvx 0, 8, 9
	cmplwi	11, 0
	lvx 1, 12, 10
	lvsl 6, 3, 9
	vperm 4, 4, 5, 6
	lvsl 5, 8, 9
	vperm 5, 0, 1, 5
	vadduwm 2, 4, 2
	addi 9, 9, 32
	vadduwm 3, 5, 3
	bne	0, .Lra_BB41_3
	vadduwm 2, 3, 2
	addi 8, 1, 16
	vsldoi 3, 2, 2, 8
	cmplw	4, 7
	vadduwm 2, 2, 3
	vspltw 3, 2, 1
	vadduwm 2, 2, 3
	stvx 2, 0, 8
	lwz 4, 16(1)
	beq	0, .Lra_BB41_7
.Lra_BB41_5:
	add 3, 3, 5
	addi 5, 6, -4
.Lra_BB41_6:
	lwzu 6, 4(5)
	addi 7, 5, 4
	cmplw	7, 3
	add 4, 6, 4
	bne	0, .Lra_BB41_6
.Lra_BB41_7:
	addi 1, 1, 32
	mr	3, 4
	blr
.Lra_BB41_8:
	li 3, 0
	blr
.Lra_func_end41:
	.size	ra_k_sum_i32, .Lra_func_end41-.Lra_func_begin41

	.section	.rodata.cst4,"aM",@progbits,4
	.p2align	2, 0x0
.Lra_CPI42_0:
	.long	0x3b808081
	.section	.text.ra_k_u8_to_f32_r,"ax",@progbits
	.globl	ra_k_u8_to_f32_r
	.p2align	2
	.type	ra_k_u8_to_f32_r,@function
ra_k_u8_to_f32_r:
.Lra_func_begin42:
	cmpwi	5, 1
	bltlr	0
	stwu 1, -32(1)
	lis 6, .Lra_CPI42_0@ha
	lfs 0, .Lra_CPI42_0@l(6)
	addi 4, 4, -1
	addi 3, 3, -4
.Lra_BB42_2:
	lbzu 6, 1(4)
	addi 5, 5, -1
	cmplwi	5, 0
	std 6, 24(1)
	lfd 1, 24(1)
	fcfid 1, 1
	frsp 1, 1
	fmuls 1, 1, 0
	stfsu 1, 4(3)
	bne	0, .Lra_BB42_2
	addi 1, 1, 32
	blr
.Lra_func_end42:
	.size	ra_k_u8_to_f32_r, .Lra_func_end42-.Lra_func_begin42

	.section	.text.ra_k_zero_u8,"ax",@progbits
	.globl	ra_k_zero_u8
	.p2align	2
	.type	ra_k_zero_u8,@function
ra_k_zero_u8:
.Lra_func_begin43:
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
.Lra_func_end43:
	.size	ra_k_zero_u8, .Lra_func_end43-.Lra_func_begin43

	.section	.text.ra_sp_bright_s,"ax",@progbits
	.globl	ra_sp_bright_s
	.p2align	2
	.type	ra_sp_bright_s,@function
ra_sp_bright_s:
.Lra_func_begin44:
	cmpwi	5, 1
	bltlr	0
	stwu 1, -32(1)
	cmplwi	5, 16
	sub	7, 3, 4
	li 6, 0
	bc 12, 0, .Lra_BB44_7
	cmplwi	7, 16
	bc 12, 0, .Lra_BB44_7
	vspltisb 2, 10
	rlwinm 6, 5, 0, 1, 27
	li 7, 0
	li 8, 15
	vslb 2, 2, 2
	addi 9, 1, 16
.Lra_BB44_4:
	add 10, 4, 7
	lvx 3, 4, 7
	mr	11, 3
	lvx 4, 10, 8
	lvsl 5, 4, 7
	vperm 3, 3, 4, 5
	vaddubs 3, 3, 2
	stvx 3, 0, 9
	ld 10, 16(1)
	stdux 10, 11, 7
	ld 10, 24(1)
	addi 7, 7, 16
	cmplw	6, 7
	std 10, 8(11)
	bne	0, .Lra_BB44_4
	cmplw	5, 6
	bne	0, .Lra_BB44_7
.Lra_BB44_6:
	addi 1, 1, 32
	blr
.Lra_BB44_7:
	addi 7, 6, -1
	add 4, 4, 7
	add 3, 3, 7
	b .Lra_BB44_9
.Lra_BB44_8:
	addi 6, 6, 1
	cmplw	5, 6
	stbu 7, 1(3)
	beq	0, .Lra_BB44_6
.Lra_BB44_9:
	lbzu 7, 1(4)
	addi 7, 7, 40
	cmplwi	7, 255
	blt	0, .Lra_BB44_8
	li 7, 255
	b .Lra_BB44_8
.Lra_func_end44:
	.size	ra_sp_bright_s, .Lra_func_end44-.Lra_func_begin44

	.section	.text.ra_sp_bright_v,"ax",@progbits
	.globl	ra_sp_bright_v
	.p2align	2
	.type	ra_sp_bright_v,@function
ra_sp_bright_v:
.Lra_func_begin45:
	srawi 6, 5, 31
	andc 5, 5, 6
	rlwinm. 6, 5, 28, 4, 31
	beqlr	0
	vspltisb 2, 10
	rlwinm 5, 5, 0, 0, 27
	li 6, 0
	vslb 2, 2, 2
.Lra_BB45_2:
	lvx 3, 4, 6
	vaddubs 3, 3, 2
	stvx 3, 3, 6
	addi 6, 6, 16
	cmplw	5, 6
	bne	0, .Lra_BB45_2
	blr
.Lra_func_end45:
	.size	ra_sp_bright_v, .Lra_func_end45-.Lra_func_begin45

	.section	.text.ra_sp_copy_v,"ax",@progbits
	.globl	ra_sp_copy_v
	.p2align	2
	.type	ra_sp_copy_v,@function
ra_sp_copy_v:
.Lra_func_begin46:
	srawi 6, 5, 31
	andc 5, 5, 6
	rlwinm. 5, 5, 30, 2, 31
	beqlr	0
	li 6, 0
.Lra_BB46_2:
	lvx 2, 4, 6
	addi 5, 5, -1
	cmplwi	5, 0
	stvx 2, 3, 6
	addi 6, 6, 16
	bne	0, .Lra_BB46_2
	blr
.Lra_func_end46:
	.size	ra_sp_copy_v, .Lra_func_end46-.Lra_func_begin46

	.section	.rodata.cst4,"aM",@progbits,4
	.p2align	2, 0x0
.Lra_CPI47_0:
	.long	0x00000000
	.section	.text.ra_sp_dot_s,"ax",@progbits
	.globl	ra_sp_dot_s
	.p2align	2
	.type	ra_sp_dot_s,@function
ra_sp_dot_s:
.Lra_func_begin47:
	srawi 6, 5, 31
	andc 5, 5, 6
	clrlwi.	5, 5, 2
	lis 6, .Lra_CPI47_0@ha
	beq	0, .Lra_BB47_4
	lfs 1, .Lra_CPI47_0@l(6)
	addi 3, 3, -4
	addi 4, 4, -4
.Lra_BB47_2:
	lfsu 0, 4(4)
	addi 5, 5, -1
	lfsu 2, 4(3)
	cmplwi	5, 0
	fmuls 0, 2, 0
	fadds 1, 1, 0
	bne	0, .Lra_BB47_2
	blr
.Lra_BB47_4:
	lfs 1, .Lra_CPI47_0@l(6)
	blr
.Lra_func_end47:
	.size	ra_sp_dot_s, .Lra_func_end47-.Lra_func_begin47

	.section	.rodata.cst16,"aM",@progbits,16
	.p2align	4, 0x0
.Lra_CPI48_0:
	.byte	0
	.byte	1
	.byte	2
	.byte	3
	.byte	12
	.byte	13
	.byte	14
	.byte	15
	.byte	0
	.byte	0
	.byte	0
	.byte	0
	.byte	0
	.byte	0
	.byte	0
	.byte	0
	.section	.text.ra_sp_dot_v,"ax",@progbits
	.globl	ra_sp_dot_v
	.p2align	2
	.type	ra_sp_dot_v,@function
ra_sp_dot_v:
.Lra_func_begin48:
	srawi 6, 5, 31
	andc 5, 5, 6
	vxor 2, 2, 2
	rlwinm. 5, 5, 28, 4, 31
	beq	0, .Lra_BB48_4
	vspltisw 3, -1
	li 6, 0
	li 7, 16
	li 8, 32
	vslw 3, 3, 3
	li 9, 48
	vxor 4, 4, 4
	vxor 5, 5, 5
	vxor 0, 0, 0
.Lra_BB48_2:
	lvx 1, 3, 6
	add 10, 3, 6
	lvx 6, 4, 6
	add 11, 4, 6
	lvx 7, 10, 7
	addi 5, 5, -1
	lvx 8, 11, 7
	vmaddfp 1, 1, 6, 3
	cmplwi	5, 0
	lvx 9, 10, 8
	vaddfp 0, 0, 1
	addi 6, 6, 64
	lvx 10, 11, 8
	vmaddfp 6, 7, 8, 3
	lvx 11, 10, 9
	vaddfp 5, 5, 6
	lvx 12, 11, 9
	vmaddfp 1, 9, 10, 3
	vaddfp 4, 4, 1
	vmaddfp 7, 11, 12, 3
	vaddfp 2, 2, 7
	bne	0, .Lra_BB48_2
	vaddfp 3, 0, 5
	vaddfp 2, 4, 2
	vaddfp 2, 3, 2
.Lra_BB48_4:
	stwu 1, -32(1)
	li 3, .Lra_CPI48_0@l
	lis 4, .Lra_CPI48_0@ha
	lvx 3, 4, 3
	vsldoi 4, 2, 2, 4
	addi 3, 1, 16
	vperm 2, 2, 2, 3
	vaddfp 2, 4, 2
	vspltw 3, 2, 1
	vaddfp 2, 2, 3
	stvx 2, 0, 3
	lfs 1, 16(1)
	addi 1, 1, 32
	blr
.Lra_func_end48:
	.size	ra_sp_dot_v, .Lra_func_end48-.Lra_func_begin48

	.section	.rodata.cst4,"aM",@progbits,4
	.p2align	2, 0x0
.Lra_CPI49_0:
	.long	0x437f0000
.Lra_CPI49_1:
	.long	0x3f000000
.Lra_CPI49_2:
	.long	0x00000000
.Lra_CPI49_3:
	.long	0x4f7fffff
	.section	.text.ra_sp_f2u8_s,"ax",@progbits
	.globl	ra_sp_f2u8_s
	.p2align	2
	.type	ra_sp_f2u8_s,@function
ra_sp_f2u8_s:
.Lra_func_begin49:
	cmpwi	5, 1
	bltlr	0
	stwu 1, -32(1)
	lis 6, .Lra_CPI49_0@ha
	lfs 0, .Lra_CPI49_0@l(6)
	lis 6, .Lra_CPI49_1@ha
	lfs 1, .Lra_CPI49_1@l(6)
	lis 6, .Lra_CPI49_2@ha
	lfs 2, .Lra_CPI49_2@l(6)
	lis 6, .Lra_CPI49_3@ha
	lfs 3, .Lra_CPI49_3@l(6)
	addi 4, 4, -4
	addi 3, 3, -1
	b .Lra_BB49_3
.Lra_BB49_2:
	addi 5, 5, -1
	cmplwi	5, 0
	stbu 7, 1(3)
	beq	0, .Lra_BB49_11
.Lra_BB49_3:
	lfsu 4, 4(4)
	fmr 5, 2
	fmuls 4, 4, 0
	fadds 4, 4, 1
	fcmpu 0, 4, 2
	blt	0, .Lra_BB49_5
	fmr 5, 4
.Lra_BB49_5:
	fcmpu 0, 5, 0
	fmr 4, 0
	bgt	0, .Lra_BB49_7
	fmr 4, 5
.Lra_BB49_7:
	fcmpu 0, 4, 2
	fctidz 5, 4
	cror 20, 0, 3
	li 6, 0
	stfd 5, 24(1)
	bc 4, 20, .Lra_BB49_9
	fcmpu 0, 4, 3
	li 7, -1
	bgt	0, .Lra_BB49_2
	b .Lra_BB49_10
.Lra_BB49_9:
	lwz 6, 28(1)
	fcmpu 0, 4, 3
	li 7, -1
	bgt	0, .Lra_BB49_2
.Lra_BB49_10:
	mr	7, 6
	b .Lra_BB49_2
.Lra_BB49_11:
	addi 1, 1, 32
	blr
.Lra_func_end49:
	.size	ra_sp_f2u8_s, .Lra_func_end49-.Lra_func_begin49

	.section	.rodata.cst16,"aM",@progbits,16
	.p2align	4, 0x0
.Lra_CPI50_0:
	.long	0x437f0000
	.long	0x437f0000
	.long	0x437f0000
	.long	0x437f0000
.Lra_CPI50_1:
	.long	0x3f000000
	.long	0x3f000000
	.long	0x3f000000
	.long	0x3f000000
.Lra_CPI50_4:
	.byte	3
	.byte	7
	.byte	11
	.byte	15
	.byte	19
	.byte	23
	.byte	27
	.byte	31
	.byte	0
	.byte	0
	.byte	0
	.byte	0
	.byte	0
	.byte	0
	.byte	0
	.byte	0
.Lra_CPI50_5:
	.byte	0
	.byte	1
	.byte	2
	.byte	3
	.byte	4
	.byte	5
	.byte	6
	.byte	7
	.byte	16
	.byte	17
	.byte	18
	.byte	19
	.byte	20
	.byte	21
	.byte	22
	.byte	23
	.section	.rodata.cst4,"aM",@progbits,4
	.p2align	2, 0x0
.Lra_CPI50_2:
	.long	0x00000000
.Lra_CPI50_3:
	.long	0x4f7fffff
	.section	.text.ra_sp_f2u8_v,"ax",@progbits
	.globl	ra_sp_f2u8_v
	.p2align	2
	.type	ra_sp_f2u8_v,@function
ra_sp_f2u8_v:
.Lra_func_begin50:
	rlwinm. 6, 5, 28, 4, 31
	cmpwi 1, 5, 1
	cror 20, 4, 2
	bclr 12, 20, 0
	stwu 1, -208(1)
	std 30, 184(1)
	li 11, .Lra_CPI50_0@l
	lis 12, .Lra_CPI50_0@ha
	li 0, .Lra_CPI50_1@l
	lis 30, .Lra_CPI50_1@ha
	std 28, 168(1)
	std 29, 176(1)
	lvx 3, 12, 11
	lis 29, .Lra_CPI50_2@ha
	lis 28, .Lra_CPI50_3@ha
	lvx 4, 30, 0
	li 11, .Lra_CPI50_4@l
	lis 12, .Lra_CPI50_4@ha
	li 0, .Lra_CPI50_5@l
	lis 30, .Lra_CPI50_5@ha
	lvx 5, 12, 11
	vspltisw 0, -1
	addi 5, 1, 96
	lfs 0, .Lra_CPI50_2@l(29)
	addi 7, 1, 112
	lfs 1, .Lra_CPI50_3@l(28)
	li 10, 0
	lvx 1, 30, 0
	addi 8, 1, 128
	vxor 2, 2, 2
	addi 9, 1, 48
	addi 11, 1, 64
	addi 12, 1, 80
	vslw 0, 0, 0
	std 26, 152(1)
	std 27, 160(1)
	b .Lra_BB50_3
.Lra_BB50_2:
	lvx 6, 0, 12
	slwi 0, 10, 4
	lvx 7, 0, 5
	addi 10, 10, 1
	lvx 8, 0, 7
	cmplw	10, 6
	lvx 9, 0, 8
	vperm 6, 6, 7, 5
	addi 4, 4, 64
	vperm 7, 8, 9, 5
	vperm 6, 6, 7, 1
	stvx 6, 3, 0
	beq	0, .Lra_BB50_21
.Lra_BB50_3:
	li 30, 0
	b .Lra_BB50_5
.Lra_BB50_4:
	stw 28, 64(1)
	lvx 6, 0, 11
	stvx 6, 12, 30
	addi 30, 30, 16
	cmplwi	30, 64
	beq	0, .Lra_BB50_2
.Lra_BB50_5:
	lvx 6, 4, 30
	li 29, 0
	li 27, 0
	vmaddfp 6, 6, 3, 0
	vaddfp 6, 6, 4
	vcmpgtfp 7, 2, 6
	vnot	7, 7
	vand 6, 7, 6
	vcmpgtfp 7, 6, 3
	vsel 6, 6, 3, 7
	stvx 6, 0, 9
	lfs 5, 60(1)
	lfs 4, 56(1)
	lfs 3, 52(1)
	fctidz 2, 5
	stfd 2, 40(1)
	fctidz 2, 4
	stfd 2, 32(1)
	fctidz 2, 3
	stfd 2, 24(1)
	fcmpu 0, 5, 0
	lfs 2, 48(1)
	cror 20, 0, 3
	fctidz 6, 2
	stfd 6, 16(1)
	bc 12, 20, .Lra_BB50_7
	lwz 27, 44(1)
.Lra_BB50_7:
	fcmpu 0, 5, 1
	li 28, -1
	li 26, -1
	bgt	0, .Lra_BB50_9
	mr	26, 27
.Lra_BB50_9:
	fcmpu 0, 4, 0
	cror 20, 0, 3
	li 27, 0
	stw 26, 76(1)
	bc 12, 20, .Lra_BB50_11
	lwz 27, 36(1)
.Lra_BB50_11:
	fcmpu 0, 4, 1
	li 26, -1
	bgt	0, .Lra_BB50_13
	mr	26, 27
.Lra_BB50_13:
	fcmpu 0, 3, 0
	cror 20, 0, 3
	li 27, 0
	stw 26, 72(1)
	bc 4, 20, .Lra_BB50_17
	fcmpu 0, 3, 1
	li 26, -1
	ble 0, .Lra_BB50_18
.Lra_BB50_15:
	fcmpu 0, 2, 0
	cror 20, 0, 3
	stw 26, 68(1)
	bc 4, 20, .Lra_BB50_19
.Lra_BB50_16:
	fcmpu 0, 2, 1
	bgt	0, .Lra_BB50_4
	b .Lra_BB50_20
.Lra_BB50_17:
	lwz 27, 28(1)
	fcmpu 0, 3, 1
	li 26, -1
	bgt	0, .Lra_BB50_15
.Lra_BB50_18:
	mr	26, 27
	fcmpu 0, 2, 0
	cror 20, 0, 3
	stw 26, 68(1)
	bc 12, 20, .Lra_BB50_16
.Lra_BB50_19:
	lwz 29, 20(1)
	fcmpu 0, 2, 1
	bgt	0, .Lra_BB50_4
.Lra_BB50_20:
	mr	28, 29
	b .Lra_BB50_4
.Lra_BB50_21:
	ld 30, 184(1)
	ld 29, 176(1)
	ld 28, 168(1)
	ld 27, 160(1)
	ld 26, 152(1)
	addi 1, 1, 208
	blr
.Lra_func_end50:
	.size	ra_sp_f2u8_v, .Lra_func_end50-.Lra_func_begin50

	.section	.text.ra_sp_mat4_v,"ax",@progbits
	.globl	ra_sp_mat4_v
	.p2align	2
	.type	ra_sp_mat4_v,@function
ra_sp_mat4_v:
.Lra_func_begin51:
	cmpwi	6, 1
	bltlr	0
	li 7, 16
	lvx 3, 4, 7
	li 7, 32
	lvx 4, 4, 7
	li 7, 48
	lvx 2, 0, 4
	vspltisw 0, -1
	lvx 5, 4, 7
	li 4, 0
	vslw 0, 0, 0
.Lra_BB51_2:
	lvx 1, 5, 4
	addi 6, 6, -1
	cmplwi	6, 0
	vspltw 6, 1, 0
	vspltw 7, 1, 1
	vspltw 8, 1, 2
	vspltw 1, 1, 3
	vmaddfp 6, 2, 6, 0
	vmaddfp 7, 3, 7, 0
	vmaddfp 8, 4, 8, 0
	vmaddfp 1, 5, 1, 0
	vaddfp 6, 6, 7
	vaddfp 6, 8, 6
	vaddfp 1, 1, 6
	stvx 1, 3, 4
	addi 4, 4, 16
	bne	0, .Lra_BB51_2
	blr
.Lra_func_end51:
	.size	ra_sp_mat4_v, .Lra_func_end51-.Lra_func_begin51

	.section	.text.ra_sp_mat4_v16,"ax",@progbits
	.globl	ra_sp_mat4_v16
	.p2align	2
	.type	ra_sp_mat4_v16,@function
ra_sp_mat4_v16:
.Lra_func_begin52:
	srawi 7, 6, 31
	andc 6, 6, 7
	rlwinm. 6, 6, 28, 4, 31
	beqlr	0
	stwu 1, -112(1)
	li 7, 16
	li 8, 32
	li 9, 48
	lvx 2, 0, 4
	vspltisw 0, -1
	std 23, 32(1)
	std 24, 40(1)
	std 25, 48(1)
	std 26, 56(1)
	std 27, 64(1)
	std 28, 72(1)
	std 29, 80(1)
	std 30, 88(1)
	li 10, 0
	lvx 3, 4, 7
	li 11, 64
	lvx 4, 4, 8
	li 12, 80
	lvx 5, 4, 9
	li 0, 96
	li 30, 112
	li 29, 128
	li 28, 144
	li 27, 160
	li 26, 176
	li 25, 192
	li 24, 208
	li 23, 224
	li 4, 240
	vslw 0, 0, 0
	std 22, 24(1)
.Lra_BB52_2:
	lvx 8, 5, 10
	add 22, 5, 10
	lvx 19, 22, 7
	addi 6, 6, -1
	lvx 18, 22, 8
	vspltw 6, 8, 1
	vspltw 31, 8, 0
	vspltw 30, 8, 2
	cmplwi	6, 0
	lvx 17, 22, 9
	vspltw 29, 19, 1
	vspltw 28, 19, 0
	vmaddfp 31, 2, 31, 0
	vmaddfp 27, 3, 6, 0
	vmaddfp 30, 4, 30, 0
	vspltw 26, 19, 2
	vspltw 19, 19, 3
	vaddfp 31, 31, 27
	vmaddfp 28, 2, 28, 0
	vmaddfp 29, 3, 29, 0
	lvx 16, 22, 11
	vspltw 27, 8, 3
	vaddfp 31, 31, 30
	vspltw 30, 18, 1
	vaddfp 29, 28, 29
	vspltw 28, 18, 0
	vmaddfp 26, 4, 26, 0
	vmaddfp 19, 5, 19, 0
	vmaddfp 27, 5, 27, 0
	vmaddfp 28, 2, 28, 0
	vmaddfp 30, 3, 30, 0
	lvx 15, 22, 12
	vaddfp 30, 28, 30
	vspltw 28, 18, 2
	vaddfp 31, 31, 27
	vspltw 27, 17, 1
	vaddfp 29, 29, 26
	vspltw 26, 17, 0
	vspltw 18, 18, 3
	lvx 14, 22, 0
	vmaddfp 27, 3, 27, 0
	vaddfp 19, 29, 19
	vspltw 29, 16, 0
	vmaddfp 18, 5, 18, 0
	lvx 13, 22, 30
	vmaddfp 29, 2, 29, 0
	lvx 12, 22, 29
	lvx 11, 22, 28
	lvx 10, 22, 27
	lvx 9, 22, 26
	lvx 7, 22, 25
	lvx 1, 22, 24
	lvx 6, 22, 23
	lvx 8, 22, 4
	add 22, 3, 10
	stvx 31, 3, 10
	vmaddfp 31, 4, 28, 0
	vmaddfp 28, 2, 26, 0
	addi 10, 10, 256
	vaddfp 28, 28, 27
	vspltw 27, 17, 2
	vaddfp 31, 30, 31
	vspltw 30, 16, 1
	stvx 19, 22, 7
	vmaddfp 19, 4, 27, 0
	vmaddfp 30, 3, 30, 0
	vspltw 17, 17, 3
	vaddfp 18, 31, 18
	vspltw 31, 15, 0
	vaddfp 30, 29, 30
	vspltw 29, 16, 2
	vaddfp 19, 28, 19
	vspltw 28, 15, 1
	stvx 18, 22, 8
	vmaddfp 17, 5, 17, 0
	vmaddfp 18, 4, 29, 0
	vmaddfp 31, 2, 31, 0
	vmaddfp 29, 3, 28, 0
	vspltw 16, 16, 3
	vaddfp 31, 31, 29
	vspltw 29, 15, 2
	vaddfp 18, 30, 18
	vspltw 30, 14, 1
	vaddfp 17, 19, 17
	vspltw 19, 14, 0
	vmaddfp 16, 5, 16, 0
	vspltw 15, 15, 3
	stvx 17, 22, 9
	vmaddfp 17, 4, 29, 0
	vmaddfp 19, 2, 19, 0
	vmaddfp 30, 3, 30, 0
	vaddfp 16, 18, 16
	vspltw 18, 13, 0
	vmaddfp 15, 5, 15, 0
	vaddfp 19, 19, 30
	vspltw 30, 14, 2
	vaddfp 17, 31, 17
	vspltw 31, 13, 1
	stvx 16, 22, 11
	vmaddfp 16, 4, 30, 0
	vmaddfp 18, 2, 18, 0
	vmaddfp 31, 3, 31, 0
	vspltw 14, 14, 3
	vaddfp 15, 17, 15
	vspltw 17, 12, 0
	vaddfp 18, 18, 31
	vspltw 31, 13, 2
	vaddfp 16, 19, 16
	vspltw 19, 12, 1
	stvx 15, 22, 12
	vmaddfp 14, 5, 14, 0
	vmaddfp 15, 4, 31, 0
	vmaddfp 17, 2, 17, 0
	vmaddfp 19, 3, 19, 0
	vspltw 13, 13, 3
	vaddfp 17, 17, 19
	vspltw 19, 12, 2
	vaddfp 15, 18, 15
	vspltw 18, 11, 1
	vaddfp 14, 16, 14
	vspltw 16, 11, 0
	vmaddfp 13, 5, 13, 0
	vspltw 12, 12, 3
	stvx 14, 22, 0
	vmaddfp 14, 4, 19, 0
	vmaddfp 16, 2, 16, 0
	vmaddfp 18, 3, 18, 0
	vaddfp 13, 15, 13
	vspltw 15, 10, 0
	vmaddfp 12, 5, 12, 0
	vaddfp 16, 16, 18
	vspltw 18, 11, 2
	vaddfp 14, 17, 14
	vspltw 17, 10, 1
	stvx 13, 22, 30
	vmaddfp 13, 4, 18, 0
	vmaddfp 15, 2, 15, 0
	vmaddfp 17, 3, 17, 0
	vspltw 11, 11, 3
	vaddfp 12, 14, 12
	vspltw 14, 9, 0
	vaddfp 15, 15, 17
	vspltw 17, 10, 2
	vaddfp 13, 16, 13
	vspltw 16, 9, 1
	stvx 12, 22, 29
	vmaddfp 11, 5, 11, 0
	vmaddfp 12, 4, 17, 0
	vmaddfp 14, 2, 14, 0
	vmaddfp 16, 3, 16, 0
	vspltw 10, 10, 3
	vaddfp 14, 14, 16
	vspltw 16, 9, 2
	vaddfp 12, 15, 12
	vspltw 15, 7, 1
	vaddfp 11, 13, 11
	vspltw 13, 7, 0
	vmaddfp 10, 5, 10, 0
	vspltw 9, 9, 3
	stvx 11, 22, 28
	vmaddfp 11, 4, 16, 0
	vmaddfp 13, 2, 13, 0
	vmaddfp 15, 3, 15, 0
	vaddfp 10, 12, 10
	vspltw 12, 1, 0
	vmaddfp 9, 5, 9, 0
	vaddfp 13, 13, 15
	vspltw 15, 7, 2
	vaddfp 11, 14, 11
	vspltw 14, 1, 1
	stvx 10, 22, 27
	vmaddfp 10, 4, 15, 0
	vmaddfp 12, 2, 12, 0
	vmaddfp 14, 3, 14, 0
	vspltw 7, 7, 3
	vaddfp 9, 11, 9
	vspltw 11, 6, 0
	vaddfp 12, 12, 14
	vspltw 14, 1, 2
	vaddfp 10, 13, 10
	vspltw 13, 6, 1
	stvx 9, 22, 26
	vmaddfp 7, 5, 7, 0
	vmaddfp 9, 4, 14, 0
	vmaddfp 11, 2, 11, 0
	vmaddfp 13, 3, 13, 0
	vspltw 1, 1, 3
	vaddfp 11, 11, 13
	vspltw 13, 6, 2
	vaddfp 9, 12, 9
	vaddfp 7, 10, 7
	vmaddfp 1, 5, 1, 0
	vspltw 12, 8, 1
	vspltw 10, 8, 0
	vspltw 6, 6, 3
	stvx 7, 22, 25
	vmaddfp 7, 4, 13, 0
	vaddfp 1, 9, 1
	vmaddfp 10, 2, 10, 0
	vmaddfp 12, 3, 12, 0
	vspltw 9, 8, 2
	vaddfp 7, 11, 7
	stvx 1, 22, 24
	vmaddfp 1, 5, 6, 0
	vaddfp 10, 10, 12
	vmaddfp 6, 4, 9, 0
	vaddfp 1, 7, 1
	vspltw 7, 8, 3
	vaddfp 6, 10, 6
	stvx 1, 22, 23
	vmaddfp 1, 5, 7, 0
	vaddfp 1, 6, 1
	stvx 1, 22, 4
	bne	0, .Lra_BB52_2
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
.Lra_func_end52:
	.size	ra_sp_mat4_v16, .Lra_func_end52-.Lra_func_begin52

	.section	.text.ra_sp_mat4_v8,"ax",@progbits
	.globl	ra_sp_mat4_v8
	.p2align	2
	.type	ra_sp_mat4_v8,@function
ra_sp_mat4_v8:
.Lra_func_begin53:
	srawi 7, 6, 31
	andc 6, 6, 7
	rlwinm. 6, 6, 29, 3, 31
	beqlr	0
	stwu 1, -48(1)
	li 7, 16
	li 8, 32
	li 9, 48
	lvx 2, 0, 4
	vspltisw 0, -1
	li 10, 0
	lvx 3, 4, 7
	li 11, 64
	lvx 4, 4, 8
	li 12, 80
	lvx 5, 4, 9
	li 0, 96
	li 4, 112
	vslw 0, 0, 0
	std 30, 24(1)
.Lra_BB53_2:
	lvx 8, 5, 10
	add 30, 5, 10
	lvx 11, 30, 7
	addi 6, 6, -1
	lvx 10, 30, 8
	vspltw 6, 8, 1
	vspltw 12, 8, 0
	vspltw 13, 8, 2
	cmplwi	6, 0
	lvx 9, 30, 9
	vmaddfp 12, 2, 12, 0
	vmaddfp 16, 3, 6, 0
	vspltw 14, 11, 1
	vspltw 15, 11, 0
	vmaddfp 13, 4, 13, 0
	vspltw 17, 11, 2
	vspltw 11, 11, 3
	vaddfp 12, 12, 16
	lvx 7, 30, 11
	vmaddfp 15, 2, 15, 0
	vmaddfp 14, 3, 14, 0
	vspltw 16, 8, 3
	vspltw 18, 10, 1
	vaddfp 12, 12, 13
	vspltw 13, 10, 0
	vmaddfp 11, 5, 11, 0
	vaddfp 14, 15, 14
	vmaddfp 15, 5, 16, 0
	vmaddfp 16, 4, 17, 0
	vmaddfp 13, 2, 13, 0
	vmaddfp 17, 3, 18, 0
	lvx 1, 30, 12
	vaddfp 13, 13, 17
	vspltw 17, 10, 2
	vaddfp 12, 12, 15
	vspltw 15, 9, 1
	vaddfp 14, 14, 16
	vspltw 16, 9, 0
	vspltw 10, 10, 3
	lvx 6, 30, 0
	vmaddfp 16, 2, 16, 0
	vmaddfp 15, 3, 15, 0
	vaddfp 11, 14, 11
	vspltw 14, 7, 0
	vmaddfp 10, 5, 10, 0
	lvx 8, 30, 4
	add 30, 3, 10
	stvx 12, 3, 10
	vmaddfp 12, 4, 17, 0
	vaddfp 15, 16, 15
	vspltw 16, 9, 2
	vmaddfp 14, 2, 14, 0
	vspltw 9, 9, 3
	addi 10, 10, 128
	vaddfp 12, 13, 12
	vspltw 13, 7, 1
	stvx 11, 30, 7
	vmaddfp 11, 4, 16, 0
	vmaddfp 13, 3, 13, 0
	vaddfp 10, 12, 10
	vspltw 12, 1, 0
	vmaddfp 9, 5, 9, 0
	vaddfp 13, 14, 13
	vspltw 14, 7, 2
	vaddfp 11, 15, 11
	vspltw 15, 1, 1
	stvx 10, 30, 8
	vmaddfp 10, 4, 14, 0
	vmaddfp 12, 2, 12, 0
	vmaddfp 14, 3, 15, 0
	vspltw 7, 7, 3
	vaddfp 9, 11, 9
	vspltw 11, 6, 0
	vaddfp 12, 12, 14
	vspltw 14, 1, 2
	vaddfp 10, 13, 10
	vspltw 13, 6, 1
	stvx 9, 30, 9
	vmaddfp 7, 5, 7, 0
	vmaddfp 9, 4, 14, 0
	vmaddfp 11, 2, 11, 0
	vmaddfp 13, 3, 13, 0
	vspltw 1, 1, 3
	vaddfp 11, 11, 13
	vspltw 13, 6, 2
	vaddfp 9, 12, 9
	vaddfp 7, 10, 7
	vmaddfp 1, 5, 1, 0
	vspltw 12, 8, 1
	vspltw 10, 8, 0
	vspltw 6, 6, 3
	stvx 7, 30, 11
	vmaddfp 7, 4, 13, 0
	vaddfp 1, 9, 1
	vmaddfp 10, 2, 10, 0
	vmaddfp 12, 3, 12, 0
	vspltw 9, 8, 2
	vaddfp 7, 11, 7
	stvx 1, 30, 12
	vmaddfp 1, 5, 6, 0
	vaddfp 10, 10, 12
	vmaddfp 6, 4, 9, 0
	vaddfp 1, 7, 1
	vspltw 7, 8, 3
	vaddfp 6, 10, 6
	stvx 1, 30, 0
	vmaddfp 1, 5, 7, 0
	vaddfp 1, 6, 1
	stvx 1, 30, 4
	bne	0, .Lra_BB53_2
	ld 30, 24(1)
	addi 1, 1, 48
	blr
.Lra_func_end53:
	.size	ra_sp_mat4_v8, .Lra_func_end53-.Lra_func_begin53

	.section	.text.ra_sp_saxpy_v,"ax",@progbits
	.globl	ra_sp_saxpy_v
	.p2align	2
	.type	ra_sp_saxpy_v,@function
ra_sp_saxpy_v:
.Lra_func_begin54:
	stwu 1, -32(1)
	srawi 5, 6, 31
	andc 5, 6, 5
	rlwinm. 5, 5, 30, 2, 31
	stfs 1, 16(1)
	beq	0, .Lra_BB54_3
	addi 6, 1, 16
	lvx 2, 0, 6
	vspltisw 3, -1
	li 6, 0
	vslw 3, 3, 3
	vspltw 2, 2, 0
.Lra_BB54_2:
	lvx 4, 4, 6
	addi 5, 5, -1
	lvx 5, 3, 6
	cmplwi	5, 0
	vmaddfp 4, 2, 4, 3
	vaddfp 4, 4, 5
	stvx 4, 3, 6
	addi 6, 6, 16
	bne	0, .Lra_BB54_2
.Lra_BB54_3:
	addi 1, 1, 32
	blr
.Lra_func_end54:
	.size	ra_sp_saxpy_v, .Lra_func_end54-.Lra_func_begin54

	.section	.rodata.cst16,"aM",@progbits,16
	.p2align	4, 0x0
.Lra_CPI55_0:
	.long	0x3b808081
	.long	0x3b808081
	.long	0x3b808081
	.long	0x3b808081
	.section	.text.ra_sp_u8f_v,"ax",@progbits
	.globl	ra_sp_u8f_v
	.p2align	2
	.type	ra_sp_u8f_v,@function
ra_sp_u8f_v:
.Lra_func_begin55:
	rlwinm. 6, 5, 28, 4, 31
	cmpwi 1, 5, 1
	cror 20, 4, 2
	bclr 12, 20, 0
	stwu 1, -352(1)
	std 30, 328(1)
	li 0, .Lra_CPI55_0@l
	lis 30, .Lra_CPI55_0@ha
	lvx 2, 30, 0
	vspltisw 3, -1
	addi 5, 3, 48
	addi 7, 3, 32
	addi 8, 3, 16
	li 9, 0
	addi 10, 1, 16
	addi 11, 1, 208
	addi 12, 1, 192
	addi 0, 1, 176
	vslw 3, 3, 3
	addi 30, 1, 160
	std 17, 224(1)
	std 18, 232(1)
	std 19, 240(1)
	std 20, 248(1)
	std 21, 256(1)
	std 22, 264(1)
	std 23, 272(1)
	std 24, 280(1)
	std 25, 288(1)
	std 26, 296(1)
	std 27, 304(1)
	std 28, 312(1)
	std 29, 320(1)
.Lra_BB55_2:
	lvx 4, 0, 4
	addi 6, 6, -1
	addi 4, 4, 16
	cmplwi	6, 0
	stvx 4, 0, 10
	lbz 17, 19(1)
	lbz 19, 23(1)
	std 17, 152(1)
	lbz 17, 18(1)
	lfd 0, 152(1)
	std 17, 144(1)
	lbz 17, 17(1)
	fcfid 0, 0
	frsp 0, 0
	stfs 0, 220(1)
	std 17, 136(1)
	lfd 0, 144(1)
	lbz 17, 16(1)
	fcfid 0, 0
	frsp 0, 0
	stfs 0, 216(1)
	std 17, 128(1)
	lfd 0, 136(1)
	lbz 22, 20(1)
	fcfid 0, 0
	frsp 0, 0
	stfs 0, 212(1)
	lfd 0, 128(1)
	lbz 21, 21(1)
	fcfid 0, 0
	frsp 0, 0
	stfs 0, 208(1)
	lvx 4, 0, 11
	lbz 20, 22(1)
	vmaddfp 4, 4, 2, 3
	lbz 25, 24(1)
	lbz 24, 25(1)
	lbz 23, 26(1)
	lbz 18, 27(1)
	lbz 29, 28(1)
	lbz 28, 29(1)
	lbz 27, 30(1)
	lbz 26, 31(1)
	stvx 4, 3, 9
	std 19, 120(1)
	std 20, 112(1)
	std 21, 104(1)
	std 22, 96(1)
	lfd 0, 120(1)
	fcfid 0, 0
	frsp 0, 0
	stfs 0, 204(1)
	lfd 0, 112(1)
	fcfid 0, 0
	frsp 0, 0
	stfs 0, 200(1)
	lfd 0, 104(1)
	fcfid 0, 0
	frsp 0, 0
	stfs 0, 196(1)
	lfd 0, 96(1)
	fcfid 0, 0
	frsp 0, 0
	stfs 0, 192(1)
	lvx 4, 0, 12
	vmaddfp 4, 4, 2, 3
	stvx 4, 8, 9
	std 18, 88(1)
	std 23, 80(1)
	std 24, 72(1)
	std 25, 64(1)
	lfd 0, 88(1)
	fcfid 0, 0
	frsp 0, 0
	stfs 0, 188(1)
	lfd 0, 80(1)
	fcfid 0, 0
	frsp 0, 0
	stfs 0, 184(1)
	lfd 0, 72(1)
	fcfid 0, 0
	frsp 0, 0
	stfs 0, 180(1)
	lfd 0, 64(1)
	fcfid 0, 0
	frsp 0, 0
	stfs 0, 176(1)
	lvx 4, 0, 0
	vmaddfp 4, 4, 2, 3
	stvx 4, 7, 9
	std 26, 56(1)
	std 27, 48(1)
	std 28, 40(1)
	std 29, 32(1)
	lfd 0, 56(1)
	fcfid 0, 0
	frsp 0, 0
	stfs 0, 172(1)
	lfd 0, 48(1)
	fcfid 0, 0
	frsp 0, 0
	stfs 0, 168(1)
	lfd 0, 40(1)
	fcfid 0, 0
	frsp 0, 0
	stfs 0, 164(1)
	lfd 0, 32(1)
	fcfid 0, 0
	frsp 0, 0
	stfs 0, 160(1)
	lvx 4, 0, 30
	vmaddfp 4, 4, 2, 3
	stvx 4, 5, 9
	addi 9, 9, 64
	bne	0, .Lra_BB55_2
	ld 30, 328(1)
	ld 29, 320(1)
	ld 28, 312(1)
	ld 27, 304(1)
	ld 26, 296(1)
	ld 25, 288(1)
	ld 24, 280(1)
	ld 23, 272(1)
	ld 22, 264(1)
	ld 21, 256(1)
	ld 20, 248(1)
	ld 19, 240(1)
	ld 18, 232(1)
	ld 17, 224(1)
	addi 1, 1, 352
	blr
.Lra_func_end55:
	.size	ra_sp_u8f_v, .Lra_func_end55-.Lra_func_begin55

	.type	ra_g_a,@object
	.section	.bss.ra_g_a,"aw",@nobits
	.globl	ra_g_a
	.p2align	4, 0x0
ra_g_a:
	.space	4096
	.size	ra_g_a, 4096

	.type	ra_g_b,@object
	.section	.bss.ra_g_b,"aw",@nobits
	.globl	ra_g_b
	.p2align	4, 0x0
ra_g_b:
	.space	4096
	.size	ra_g_b, 4096

	.type	ra_g_c,@object
	.section	.bss.ra_g_c,"aw",@nobits
	.globl	ra_g_c
	.p2align	4, 0x0
ra_g_c:
	.space	4096
	.size	ra_g_c, 4096

	.type	ra_g_ia,@object
	.section	.bss.ra_g_ia,"aw",@nobits
	.globl	ra_g_ia
	.p2align	4, 0x0
ra_g_ia:
	.space	4096
	.size	ra_g_ia, 4096

	.type	ra_g_ib,@object
	.section	.bss.ra_g_ib,"aw",@nobits
	.globl	ra_g_ib
	.p2align	4, 0x0
ra_g_ib:
	.space	4096
	.size	ra_g_ib, 4096

	.type	ra_g_ic,@object
	.section	.bss.ra_g_ic,"aw",@nobits
	.globl	ra_g_ic
	.p2align	4, 0x0
ra_g_ic:
	.space	4096
	.size	ra_g_ic, 4096

	.type	.Lra__MergedGlobals,@object
	.section	.data.rel.ro..Lra__MergedGlobals,"aw",@progbits
	.p2align	2, 0x0
.Lra__MergedGlobals:
	.long	.Lra__MergedGlobals+16
	.ascii	"\000\000\000\033\000\000\000p\000\000\000\021"
	.asciz	"examples/vmx128bench/lib.rs"
	.size	.Lra__MergedGlobals, 44

	.globl	ra_sp_mat4_s
	.type	ra_sp_mat4_s,@function
ra_sp_mat4_s = ra_k_mat4_vec4_r
	.globl	ra_sp_u8f_s
	.type	ra_sp_u8f_s,@function
ra_sp_u8f_s = ra_k_u8_to_f32_r
	.globl	ra_sp_saxpy_s
	.type	ra_sp_saxpy_s,@function
ra_sp_saxpy_s = ra_k_saxpy_r
	.globl	ra_sp_copy_s
	.type	ra_sp_copy_s,@function
ra_sp_copy_s = ra_k_copy_u32_r
.Lra_anon.c4ad4bf0cb04114197cf469a12acc631.1 = .Lra__MergedGlobals
	.size	.Lra_anon.c4ad4bf0cb04114197cf469a12acc631.1, 16
.Lra_anon.c4ad4bf0cb04114197cf469a12acc631.0 = .Lra__MergedGlobals+16
	.size	.Lra_anon.c4ad4bf0cb04114197cf469a12acc631.0, 28
