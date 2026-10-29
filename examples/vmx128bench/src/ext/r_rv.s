# kernels.rs: rustc of the PPC64 toolchain (rustc_bootstrap cell)
# Buck target //examples/vmx128bench:kernels_rv
	.section	.rodata.cst4,"aM",@progbits,4
	.p2align	2, 0x0
.Lrv_CPI0_0:
	.long	0x00000000
	.section	.text.rv_k_abs_f32_r,"ax",@progbits
	.globl	rv_k_abs_f32_r
	.p2align	2
	.type	rv_k_abs_f32_r,@function
rv_k_abs_f32_r:
.Lrv_func_begin0:
	cmpwi	5, 1
	bltlr	0
	lis 6, .Lrv_CPI0_0@ha
	lfs 0, .Lrv_CPI0_0@l(6)
	addi 4, 4, -4
	addi 3, 3, -4
	b .Lrv_BB0_3
.Lrv_BB0_2:
	addi 5, 5, -1
	cmplwi	5, 0
	stfsu 1, 4(3)
	beqlr	0
.Lrv_BB0_3:
	lfsu 1, 4(4)
	fcmpu 0, 1, 0
	bge 0, .Lrv_BB0_2
	fneg 1, 1
	b .Lrv_BB0_2
.Lrv_func_end0:
	.size	rv_k_abs_f32_r, .Lrv_func_end0-.Lrv_func_begin0

	.section	.text.rv_k_abs_i32_r,"ax",@progbits
	.globl	rv_k_abs_i32_r
	.p2align	2
	.type	rv_k_abs_i32_r,@function
rv_k_abs_i32_r:
.Lrv_func_begin1:
	cmpwi	5, 1
	bltlr	0
	stwu 1, -32(1)
	cmplwi	5, 4
	sub	7, 3, 4
	li 6, 0
	bc 12, 0, .Lrv_BB1_6
	cmplwi	7, 16
	bc 12, 0, .Lrv_BB1_6
	rlwinm 6, 5, 0, 1, 29
	li 7, 0
	li 8, 15
	vspltisw128 2, 0
	addi 9, 1, 16
	mr	10, 6
.Lrv_BB1_4:
	add 11, 4, 7
	lvx128 3, 4, 7
	mr	12, 3
	lvx128 4, 11, 8
	addi 10, 10, -4
	lvsl 5, 4, 7
	cmplwi	10, 0
	vperm128 3, 3, 4, 5
	vsubuwm 4, 2, 3
	vmaxsw 3, 3, 4
	stvx128 3, 0, 9
	ld 11, 16(1)
	stdux 11, 12, 7
	ld 11, 24(1)
	addi 7, 7, 16
	std 11, 8(12)
	bne	0, .Lrv_BB1_4
	cmplw	5, 6
	beq	0, .Lrv_BB1_8
.Lrv_BB1_6:
	sub	5, 5, 6
	slwi 6, 6, 2
	addi 6, 6, -4
	add 4, 4, 6
	add 3, 3, 6
.Lrv_BB1_7:
	lwzu 6, 4(4)
	addi 5, 5, -1
	cmplwi	5, 0
	srawi 7, 6, 31
	xor 6, 6, 7
	sub	6, 6, 7
	stwu 6, 4(3)
	bne	0, .Lrv_BB1_7
.Lrv_BB1_8:
	addi 1, 1, 32
	blr
.Lrv_func_end1:
	.size	rv_k_abs_i32_r, .Lrv_func_end1-.Lrv_func_begin1

	.section	.text.rv_k_add16_f32_r,"ax",@progbits
	.globl	rv_k_add16_f32_r
	.p2align	2
	.type	rv_k_add16_f32_r,@function
rv_k_add16_f32_r:
.Lrv_func_begin2:
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
.Lrv_func_end2:
	.size	rv_k_add16_f32_r, .Lrv_func_end2-.Lrv_func_begin2

	.section	.text.rv_k_add_f32_r,"ax",@progbits
	.globl	rv_k_add_f32_r
	.p2align	2
	.type	rv_k_add_f32_r,@function
rv_k_add_f32_r:
.Lrv_func_begin3:
	cmpwi	6, 1
	bltlr	0
	stwu 1, -64(1)
	cmplwi	6, 8
	li 7, 0
	std 30, 40(1)
	bge 0, .Lrv_BB3_5
.Lrv_BB3_2:
	sub	6, 6, 7
	slwi 7, 7, 2
	addi 7, 7, -4
	add 4, 4, 7
	add 5, 5, 7
	add 3, 3, 7
.Lrv_BB3_3:
	lfsu 0, 4(4)
	addi 6, 6, -1
	lfsu 1, 4(5)
	cmplwi	6, 0
	fadds 0, 0, 1
	stfsu 0, 4(3)
	bne	0, .Lrv_BB3_3
.Lrv_BB3_4:
	ld 30, 40(1)
	addi 1, 1, 64
	blr
.Lrv_BB3_5:
	sub	8, 3, 4
	cmplwi	8, 16
	sub	8, 3, 5
	bc 12, 0, .Lrv_BB3_2
	cmplwi	8, 16
	bc 12, 0, .Lrv_BB3_2
	rlwinm 7, 6, 0, 1, 29
	li 8, 0
	li 9, 15
	addi 10, 1, 16
	mr	11, 7
.Lrv_BB3_8:
	add 12, 4, 8
	lvx128 2, 4, 8
	mr	30, 3
	lvx128 3, 12, 9
	add 12, 5, 8
	lvx128 4, 5, 8
	addi 11, 11, -4
	lvx128 5, 12, 9
	cmplwi	11, 0
	lvsl 0, 4, 8
	vperm128 2, 2, 3, 0
	lvsl 1, 5, 8
	vperm128 3, 4, 5, 1
	vaddfp128 2, 2, 3
	stvx128 2, 0, 10
	ld 12, 16(1)
	stdux 12, 30, 8
	ld 12, 24(1)
	addi 8, 8, 16
	std 12, 8(30)
	bne	0, .Lrv_BB3_8
	cmplw	6, 7
	bne	0, .Lrv_BB3_2
	b .Lrv_BB3_4
.Lrv_func_end3:
	.size	rv_k_add_f32_r, .Lrv_func_end3-.Lrv_func_begin3

	.section	.text.rv_k_add_i16_r,"ax",@progbits
	.globl	rv_k_add_i16_r
	.p2align	2
	.type	rv_k_add_i16_r,@function
rv_k_add_i16_r:
.Lrv_func_begin4:
	cmpwi	6, 1
	bltlr	0
	stwu 1, -64(1)
	cmplwi	6, 16
	li 7, 0
	std 30, 40(1)
	bge 0, .Lrv_BB4_5
.Lrv_BB4_2:
	sub	6, 6, 7
	slwi 7, 7, 1
	addi 7, 7, -2
	add 4, 4, 7
	add 5, 5, 7
	add 3, 3, 7
.Lrv_BB4_3:
	lhzu 7, 2(4)
	addi 6, 6, -1
	lhzu 8, 2(5)
	cmplwi	6, 0
	add 7, 8, 7
	sthu 7, 2(3)
	bne	0, .Lrv_BB4_3
.Lrv_BB4_4:
	ld 30, 40(1)
	addi 1, 1, 64
	blr
.Lrv_BB4_5:
	sub	8, 3, 4
	cmplwi	8, 16
	sub	8, 3, 5
	bc 12, 0, .Lrv_BB4_2
	cmplwi	8, 16
	bc 12, 0, .Lrv_BB4_2
	rlwinm 7, 6, 0, 1, 28
	li 8, 0
	li 9, 15
	addi 10, 1, 16
	mr	11, 7
.Lrv_BB4_8:
	add 12, 4, 8
	lvx128 2, 4, 8
	mr	30, 3
	lvx128 3, 12, 9
	add 12, 5, 8
	lvx128 4, 5, 8
	addi 11, 11, -8
	lvx128 5, 12, 9
	cmplwi	11, 0
	lvsl 0, 4, 8
	vperm128 2, 2, 3, 0
	lvsl 1, 5, 8
	vperm128 3, 4, 5, 1
	vadduhm 2, 3, 2
	stvx128 2, 0, 10
	ld 12, 16(1)
	stdux 12, 30, 8
	ld 12, 24(1)
	addi 8, 8, 16
	std 12, 8(30)
	bne	0, .Lrv_BB4_8
	cmplw	6, 7
	bne	0, .Lrv_BB4_2
	b .Lrv_BB4_4
.Lrv_func_end4:
	.size	rv_k_add_i16_r, .Lrv_func_end4-.Lrv_func_begin4

	.section	.text.rv_k_add_i32,"ax",@progbits
	.globl	rv_k_add_i32
	.p2align	2
	.type	rv_k_add_i32,@function
rv_k_add_i32:
.Lrv_func_begin5:
	cmpwi	6, 1
	bltlr	0
	stwu 1, -64(1)
	cmplwi	6, 20
	li 7, 0
	std 30, 40(1)
	bge 0, .Lrv_BB5_5
.Lrv_BB5_2:
	slwi 8, 7, 2
	sub	6, 6, 7
	addi 7, 8, -4
	add 4, 4, 7
	add 5, 5, 7
	add 3, 3, 7
.Lrv_BB5_3:
	lwzu 7, 4(4)
	addi 6, 6, -1
	lwzu 8, 4(5)
	cmplwi	6, 0
	add 7, 8, 7
	stwu 7, 4(3)
	bne	0, .Lrv_BB5_3
.Lrv_BB5_4:
	ld 30, 40(1)
	addi 1, 1, 64
	blr
.Lrv_BB5_5:
	sub	8, 3, 4
	cmplwi	8, 16
	sub	8, 3, 5
	bc 12, 0, .Lrv_BB5_2
	cmplwi	8, 16
	bc 12, 0, .Lrv_BB5_2
	rlwinm 7, 6, 0, 1, 29
	li 8, 0
	li 9, 15
	addi 10, 1, 16
	mr	11, 7
.Lrv_BB5_8:
	add 12, 4, 8
	lvx128 2, 4, 8
	mr	30, 3
	lvx128 3, 12, 9
	add 12, 5, 8
	lvx128 4, 5, 8
	addi 11, 11, -4
	lvx128 5, 12, 9
	cmplwi	11, 0
	lvsl 0, 4, 8
	vperm128 2, 2, 3, 0
	lvsl 1, 5, 8
	vperm128 3, 4, 5, 1
	vadduwm 2, 3, 2
	stvx128 2, 0, 10
	ld 12, 16(1)
	stdux 12, 30, 8
	ld 12, 24(1)
	addi 8, 8, 16
	std 12, 8(30)
	bne	0, .Lrv_BB5_8
	cmplw	6, 7
	bne	0, .Lrv_BB5_2
	b .Lrv_BB5_4
.Lrv_func_end5:
	.size	rv_k_add_i32, .Lrv_func_end5-.Lrv_func_begin5

	.section	.text.rv_k_add_i32_r,"ax",@progbits
	.globl	rv_k_add_i32_r
	.p2align	2
	.type	rv_k_add_i32_r,@function
rv_k_add_i32_r:
.Lrv_func_begin6:
	cmpwi	6, 1
	bltlr	0
	stwu 1, -64(1)
	cmplwi	6, 20
	li 7, 0
	std 30, 40(1)
	bge 0, .Lrv_BB6_5
.Lrv_BB6_2:
	sub	6, 6, 7
	slwi 7, 7, 2
	addi 7, 7, -4
	add 4, 4, 7
	add 5, 5, 7
	add 3, 3, 7
.Lrv_BB6_3:
	lwzu 7, 4(4)
	addi 6, 6, -1
	lwzu 8, 4(5)
	cmplwi	6, 0
	add 7, 8, 7
	stwu 7, 4(3)
	bne	0, .Lrv_BB6_3
.Lrv_BB6_4:
	ld 30, 40(1)
	addi 1, 1, 64
	blr
.Lrv_BB6_5:
	sub	8, 3, 4
	cmplwi	8, 16
	sub	8, 3, 5
	bc 12, 0, .Lrv_BB6_2
	cmplwi	8, 16
	bc 12, 0, .Lrv_BB6_2
	rlwinm 7, 6, 0, 1, 29
	li 8, 0
	li 9, 15
	addi 10, 1, 16
	mr	11, 7
.Lrv_BB6_8:
	add 12, 4, 8
	lvx128 2, 4, 8
	mr	30, 3
	lvx128 3, 12, 9
	add 12, 5, 8
	lvx128 4, 5, 8
	addi 11, 11, -4
	lvx128 5, 12, 9
	cmplwi	11, 0
	lvsl 0, 4, 8
	vperm128 2, 2, 3, 0
	lvsl 1, 5, 8
	vperm128 3, 4, 5, 1
	vadduwm 2, 3, 2
	stvx128 2, 0, 10
	ld 12, 16(1)
	stdux 12, 30, 8
	ld 12, 24(1)
	addi 8, 8, 16
	std 12, 8(30)
	bne	0, .Lrv_BB6_8
	cmplw	6, 7
	bne	0, .Lrv_BB6_2
	b .Lrv_BB6_4
.Lrv_func_end6:
	.size	rv_k_add_i32_r, .Lrv_func_end6-.Lrv_func_begin6

	.section	.text.rv_k_add_u8_r,"ax",@progbits
	.globl	rv_k_add_u8_r
	.p2align	2
	.type	rv_k_add_u8_r,@function
rv_k_add_u8_r:
.Lrv_func_begin7:
	cmpwi	6, 1
	bltlr	0
	stwu 1, -32(1)
	cmplwi	6, 16
	li 7, 0
	bge 0, .Lrv_BB7_5
.Lrv_BB7_2:
	addi 8, 7, -1
	add 4, 4, 8
	add 5, 5, 8
	add 3, 3, 8
.Lrv_BB7_3:
	lbzu 8, 1(4)
	addi 7, 7, 1
	lbzu 9, 1(5)
	cmplw	6, 7
	add 8, 9, 8
	stbu 8, 1(3)
	bne	0, .Lrv_BB7_3
.Lrv_BB7_4:
	addi 1, 1, 32
	blr
.Lrv_BB7_5:
	sub	8, 3, 4
	cmplwi	8, 16
	sub	8, 3, 5
	bc 12, 0, .Lrv_BB7_2
	cmplwi	8, 16
	bc 12, 0, .Lrv_BB7_2
	rlwinm 7, 6, 0, 1, 27
	li 8, 0
	li 9, 15
	addi 10, 1, 16
.Lrv_BB7_8:
	add 11, 4, 8
	lvx128 2, 4, 8
	mr	12, 3
	lvx128 3, 11, 9
	add 11, 5, 8
	lvx128 4, 5, 8
	lvx128 5, 11, 9
	lvsl 0, 4, 8
	vperm128 2, 2, 3, 0
	lvsl 1, 5, 8
	vperm128 3, 4, 5, 1
	vaddubm 2, 3, 2
	stvx128 2, 0, 10
	ld 11, 16(1)
	stdux 11, 12, 8
	ld 11, 24(1)
	addi 8, 8, 16
	cmplw	7, 8
	std 11, 8(12)
	bne	0, .Lrv_BB7_8
	cmplw	6, 7
	bne	0, .Lrv_BB7_2
	b .Lrv_BB7_4
.Lrv_func_end7:
	.size	rv_k_add_u8_r, .Lrv_func_end7-.Lrv_func_begin7

	.section	.text.rv_k_addsat_u8_r,"ax",@progbits
	.globl	rv_k_addsat_u8_r
	.p2align	2
	.type	rv_k_addsat_u8_r,@function
rv_k_addsat_u8_r:
.Lrv_func_begin8:
	cmpwi	6, 1
	bltlr	0
	stwu 1, -32(1)
	cmplwi	6, 16
	li 7, 0
	bge 0, .Lrv_BB8_6
.Lrv_BB8_2:
	addi 8, 7, -1
	add 4, 4, 8
	add 5, 5, 8
	add 3, 3, 8
	b .Lrv_BB8_4
.Lrv_BB8_3:
	addi 7, 7, 1
	cmplw	6, 7
	stbu 8, 1(3)
	beq	0, .Lrv_BB8_11
.Lrv_BB8_4:
	lbzu 8, 1(4)
	lbzu 9, 1(5)
	add 8, 8, 9
	cmplwi	8, 255
	blt	0, .Lrv_BB8_3
	li 8, 255
	b .Lrv_BB8_3
.Lrv_BB8_6:
	sub	8, 3, 4
	cmplwi	8, 16
	sub	8, 3, 5
	bc 12, 0, .Lrv_BB8_2
	cmplwi	8, 16
	bc 12, 0, .Lrv_BB8_2
	rlwinm 7, 6, 0, 1, 27
	li 8, 0
	li 9, 15
	addi 10, 1, 16
.Lrv_BB8_9:
	add 11, 4, 8
	lvx128 2, 4, 8
	mr	12, 3
	lvx128 3, 11, 9
	add 11, 5, 8
	lvx128 4, 5, 8
	lvx128 5, 11, 9
	lvsl 0, 4, 8
	vperm128 2, 2, 3, 0
	lvsl 1, 5, 8
	vperm128 3, 4, 5, 1
	vaddubs 2, 2, 3
	stvx128 2, 0, 10
	ld 11, 16(1)
	stdux 11, 12, 8
	ld 11, 24(1)
	addi 8, 8, 16
	cmplw	7, 8
	std 11, 8(12)
	bne	0, .Lrv_BB8_9
	cmplw	6, 7
	bne	0, .Lrv_BB8_2
.Lrv_BB8_11:
	addi 1, 1, 32
	blr
.Lrv_func_end8:
	.size	rv_k_addsat_u8_r, .Lrv_func_end8-.Lrv_func_begin8

	.section	.text.rv_k_and_u32_r,"ax",@progbits
	.globl	rv_k_and_u32_r
	.p2align	2
	.type	rv_k_and_u32_r,@function
rv_k_and_u32_r:
.Lrv_func_begin9:
	cmpwi	6, 1
	bltlr	0
	stwu 1, -64(1)
	cmplwi	6, 20
	li 7, 0
	std 30, 40(1)
	bge 0, .Lrv_BB9_5
.Lrv_BB9_2:
	sub	6, 6, 7
	slwi 7, 7, 2
	addi 7, 7, -4
	add 4, 4, 7
	add 5, 5, 7
	add 3, 3, 7
.Lrv_BB9_3:
	lwzu 7, 4(4)
	addi 6, 6, -1
	lwzu 8, 4(5)
	cmplwi	6, 0
	and 7, 8, 7
	stwu 7, 4(3)
	bne	0, .Lrv_BB9_3
.Lrv_BB9_4:
	ld 30, 40(1)
	addi 1, 1, 64
	blr
.Lrv_BB9_5:
	sub	8, 3, 4
	cmplwi	8, 16
	sub	8, 3, 5
	bc 12, 0, .Lrv_BB9_2
	cmplwi	8, 16
	bc 12, 0, .Lrv_BB9_2
	rlwinm 7, 6, 0, 1, 29
	li 8, 0
	li 9, 15
	addi 10, 1, 16
	mr	11, 7
.Lrv_BB9_8:
	add 12, 4, 8
	lvx128 2, 4, 8
	mr	30, 3
	lvx128 3, 12, 9
	add 12, 5, 8
	lvx128 4, 5, 8
	addi 11, 11, -4
	lvx128 5, 12, 9
	cmplwi	11, 0
	lvsl 0, 4, 8
	vperm128 2, 2, 3, 0
	lvsl 1, 5, 8
	vperm128 3, 4, 5, 1
	vand128 2, 3, 2
	stvx128 2, 0, 10
	ld 12, 16(1)
	stdux 12, 30, 8
	ld 12, 24(1)
	addi 8, 8, 16
	std 12, 8(30)
	bne	0, .Lrv_BB9_8
	cmplw	6, 7
	bne	0, .Lrv_BB9_2
	b .Lrv_BB9_4
.Lrv_func_end9:
	.size	rv_k_and_u32_r, .Lrv_func_end9-.Lrv_func_begin9

	.section	.text.rv_k_avg_u8_r,"ax",@progbits
	.globl	rv_k_avg_u8_r
	.p2align	2
	.type	rv_k_avg_u8_r,@function
rv_k_avg_u8_r:
.Lrv_func_begin10:
	cmpwi	6, 1
	bltlr	0
	stwu 1, -32(1)
	cmplwi	6, 16
	li 7, 0
	bge 0, .Lrv_BB10_5
.Lrv_BB10_2:
	addi 8, 7, -1
	add 4, 4, 8
	add 5, 5, 8
	add 3, 3, 8
.Lrv_BB10_3:
	lbzu 8, 1(4)
	addi 7, 7, 1
	lbzu 9, 1(5)
	cmplw	6, 7
	add 8, 8, 9
	addi 8, 8, 1
	srwi 8, 8, 1
	stbu 8, 1(3)
	bne	0, .Lrv_BB10_3
.Lrv_BB10_4:
	addi 1, 1, 32
	blr
.Lrv_BB10_5:
	sub	8, 3, 4
	cmplwi	8, 16
	sub	8, 3, 5
	bc 12, 0, .Lrv_BB10_2
	cmplwi	8, 16
	bc 12, 0, .Lrv_BB10_2
	rlwinm 7, 6, 0, 1, 27
	li 8, 0
	li 9, 15
	vspltisb 2, 1
	addi 10, 1, 16
.Lrv_BB10_8:
	add 11, 4, 8
	lvx128 3, 4, 8
	mr	12, 3
	lvx128 4, 11, 9
	add 11, 5, 8
	lvx128 5, 5, 8
	lvx128 0, 11, 9
	lvsl 1, 4, 8
	vperm128 3, 3, 4, 1
	lvsl 6, 5, 8
	vperm128 4, 5, 0, 6
	vxor128 5, 3, 4
	vor128 3, 3, 4
	vsrb 4, 5, 2
	vsububm 3, 3, 4
	stvx128 3, 0, 10
	ld 11, 16(1)
	stdux 11, 12, 8
	ld 11, 24(1)
	addi 8, 8, 16
	cmplw	7, 8
	std 11, 8(12)
	bne	0, .Lrv_BB10_8
	cmplw	6, 7
	bne	0, .Lrv_BB10_2
	b .Lrv_BB10_4
.Lrv_func_end10:
	.size	rv_k_avg_u8_r, .Lrv_func_end10-.Lrv_func_begin10

	.section	.text.rv_k_clamp_f32_r,"ax",@progbits
	.globl	rv_k_clamp_f32_r
	.p2align	2
	.type	rv_k_clamp_f32_r,@function
rv_k_clamp_f32_r:
.Lrv_func_begin11:
	cmpwi	7, 1
	bltlr	0
	stwu 1, -64(1)
	cmplwi	7, 4
	sub	6, 3, 4
	li 5, 0
	bc 12, 0, .Lrv_BB11_7
	cmplwi	6, 16
	bc 12, 0, .Lrv_BB11_7
	stfs 1, 48(1)
	addi 5, 1, 48
	lvx128 2, 0, 5
	addi 5, 1, 32
	stfs 2, 32(1)
	li 6, 0
	lvx128 3, 0, 5
	rlwinm 5, 7, 0, 1, 29
	li 8, 15
	vspltw128 2, 2, 0
	addi 9, 1, 16
	vspltw128 3, 3, 0
	mr	10, 5
.Lrv_BB11_4:
	add 11, 4, 6
	lvx128 4, 4, 6
	mr	12, 3
	lvx128 5, 11, 8
	addi 10, 10, -4
	lvsl 0, 4, 6
	cmplwi	10, 0
	vperm128 4, 4, 5, 0
	vcmpgtfp128 5, 2, 4
	vsel128 5, 4, 2, 5
	vcmpgtfp128 4, 5, 3
	vsel128 4, 5, 3, 4
	stvx128 4, 0, 9
	ld 11, 16(1)
	stdux 11, 12, 6
	ld 11, 24(1)
	addi 6, 6, 16
	std 11, 8(12)
	bne	0, .Lrv_BB11_4
	cmplw	7, 5
	bne	0, .Lrv_BB11_7
.Lrv_BB11_6:
	addi 1, 1, 64
	blr
.Lrv_BB11_7:
	sub	6, 7, 5
	slwi 5, 5, 2
	addi 5, 5, -4
	add 4, 4, 5
	add 3, 3, 5
	b .Lrv_BB11_9
.Lrv_BB11_8:
	addi 6, 6, -1
	cmplwi	6, 0
	stfsu 0, 4(3)
	beq	0, .Lrv_BB11_6
.Lrv_BB11_9:
	lfsu 0, 4(4)
	fmr 3, 1
	fcmpu 0, 0, 1
	bge 0, .Lrv_BB11_11
	fcmpu 0, 3, 2
	fmr 0, 2
	bgt	0, .Lrv_BB11_8
	b .Lrv_BB11_12
.Lrv_BB11_11:
	fmr 3, 0
	fcmpu 0, 3, 2
	fmr 0, 2
	bgt	0, .Lrv_BB11_8
.Lrv_BB11_12:
	fmr 0, 3
	b .Lrv_BB11_8
.Lrv_func_end11:
	.size	rv_k_clamp_f32_r, .Lrv_func_end11-.Lrv_func_begin11

	.section	.rodata.cst16,"aM",@progbits,16
	.p2align	4, 0x0
.Lrv_CPI12_0:
	.short	64536
	.short	64536
	.short	64536
	.short	64536
	.short	64536
	.short	64536
	.short	64536
	.short	64536
.Lrv_CPI12_1:
	.short	1000
	.short	1000
	.short	1000
	.short	1000
	.short	1000
	.short	1000
	.short	1000
	.short	1000
	.section	.text.rv_k_clamp_i16_r,"ax",@progbits
	.globl	rv_k_clamp_i16_r
	.p2align	2
	.type	rv_k_clamp_i16_r,@function
rv_k_clamp_i16_r:
.Lrv_func_begin12:
	cmpwi	5, 1
	bltlr	0
	stwu 1, -32(1)
	cmplwi	5, 8
	sub	7, 3, 4
	li 6, 0
	bc 12, 0, .Lrv_BB12_7
	cmplwi	7, 16
	bc 12, 0, .Lrv_BB12_7
	li 9, .Lrv_CPI12_0@l
	lis 10, .Lrv_CPI12_0@ha
	lvx128 2, 10, 9
	li 9, .Lrv_CPI12_1@l
	lis 10, .Lrv_CPI12_1@ha
	lvx128 3, 10, 9
	rlwinm 6, 5, 0, 1, 28
	li 7, 0
	li 8, 15
	addi 9, 1, 16
	mr	10, 6
.Lrv_BB12_4:
	add 11, 4, 7
	lvx128 4, 4, 7
	mr	12, 3
	lvx128 5, 11, 8
	addi 10, 10, -8
	lvsl 0, 4, 7
	cmplwi	10, 0
	vperm128 4, 4, 5, 0
	vmaxsh 4, 4, 2
	vminsh 4, 4, 3
	stvx128 4, 0, 9
	ld 11, 16(1)
	stdux 11, 12, 7
	ld 11, 24(1)
	addi 7, 7, 16
	std 11, 8(12)
	bne	0, .Lrv_BB12_4
	cmplw	5, 6
	bne	0, .Lrv_BB12_7
.Lrv_BB12_6:
	addi 1, 1, 32
	blr
.Lrv_BB12_7:
	sub	5, 5, 6
	slwi 6, 6, 1
	addi 6, 6, -2
	add 4, 4, 6
	add 3, 3, 6
	b .Lrv_BB12_9
.Lrv_BB12_8:
	addi 5, 5, -1
	cmplwi	5, 0
	sthu 6, 2(3)
	beq	0, .Lrv_BB12_6
.Lrv_BB12_9:
	lhau 6, 2(4)
	cmpwi	6, -1000
	ble 0, .Lrv_BB12_11
	cmpwi	6, 1000
	blt	0, .Lrv_BB12_8
	b .Lrv_BB12_12
.Lrv_BB12_11:
	li 6, -1000
	cmpwi	6, 1000
	blt	0, .Lrv_BB12_8
.Lrv_BB12_12:
	li 6, 1000
	b .Lrv_BB12_8
.Lrv_func_end12:
	.size	rv_k_clamp_i16_r, .Lrv_func_end12-.Lrv_func_begin12

	.section	.text.rv_k_copy_u32_r,"ax",@progbits
	.globl	rv_k_copy_u32_r
	.p2align	2
	.type	rv_k_copy_u32_r,@function
rv_k_copy_u32_r:
.Lrv_func_begin13:
	cmpwi	5, 1
	bltlr	0
	addi 4, 4, -4
	addi 3, 3, -4
.Lrv_BB13_2:
	lwzu 6, 4(4)
	addi 5, 5, -1
	cmplwi	5, 0
	stwu 6, 4(3)
	bne	0, .Lrv_BB13_2
	blr
.Lrv_func_end13:
	.size	rv_k_copy_u32_r, .Lrv_func_end13-.Lrv_func_begin13

	.section	.text.rv_k_copy_u8,"ax",@progbits
	.globl	rv_k_copy_u8
	.p2align	2
	.type	rv_k_copy_u8,@function
rv_k_copy_u8:
.Lrv_func_begin14:
	cmpwi	5, 1
	bltlr	0
	addi 4, 4, -1
	addi 3, 3, -1
	li 6, 0
.Lrv_BB14_2:
	lbzu 7, 1(4)
	addi 6, 6, 1
	cmplw	5, 6
	stbu 7, 1(3)
	bne	0, .Lrv_BB14_2
	blr
.Lrv_func_end14:
	.size	rv_k_copy_u8, .Lrv_func_end14-.Lrv_func_begin14

	.section	.rodata.cst4,"aM",@progbits,4
	.p2align	2, 0x0
.Lrv_CPI15_0:
	.long	0x00000000
	.section	.text.rv_k_dot_f32,"ax",@progbits
	.globl	rv_k_dot_f32
	.p2align	2
	.type	rv_k_dot_f32,@function
rv_k_dot_f32:
.Lrv_func_begin15:
	srawi 6, 5, 31
	andc 5, 5, 6
	clrlwi.	5, 5, 2
	lis 6, .Lrv_CPI15_0@ha
	beq	0, .Lrv_BB15_4
	lfs 1, .Lrv_CPI15_0@l(6)
	addi 3, 3, -4
	addi 4, 4, -4
.Lrv_BB15_2:
	lfsu 0, 4(4)
	addi 5, 5, -1
	lfsu 2, 4(3)
	cmplwi	5, 0
	fmuls 0, 2, 0
	fadds 1, 1, 0
	bne	0, .Lrv_BB15_2
	blr
.Lrv_BB15_4:
	lfs 1, .Lrv_CPI15_0@l(6)
	blr
.Lrv_func_end15:
	.size	rv_k_dot_f32, .Lrv_func_end15-.Lrv_func_begin15

	.section	.text.rv_k_dot_i16,"ax",@progbits
	.globl	rv_k_dot_i16
	.p2align	2
	.type	rv_k_dot_i16,@function
rv_k_dot_i16:
.Lrv_func_begin16:
	srawi 6, 5, 31
	andc 7, 5, 6
	cmplwi	7, 0
	beq	0, .Lrv_BB16_8
	stwu 1, -48(1)
	li 6, 0
	cmpwi	5, 2
	li 8, 0
	std 29, 16(1)
	std 30, 24(1)
	blt	0, .Lrv_BB16_5
	rlwinm 8, 7, 0, 1, 30
	addi 9, 3, -4
	addi 10, 4, -4
	mr	12, 8
	li 11, 0
.Lrv_BB16_3:
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
	bne	0, .Lrv_BB16_3
	cmplw	5, 8
	add 6, 11, 6
	beq	0, .Lrv_BB16_7
.Lrv_BB16_5:
	sub	5, 7, 8
	slwi 7, 8, 1
	addi 7, 7, -2
	add 3, 3, 7
	add 4, 4, 7
.Lrv_BB16_6:
	lhau 7, 2(4)
	addi 5, 5, -1
	lhau 8, 2(3)
	cmplwi	5, 0
	mullw 7, 7, 8
	add 6, 7, 6
	bne	0, .Lrv_BB16_6
.Lrv_BB16_7:
	ld 30, 24(1)
	ld 29, 16(1)
	addi 1, 1, 48
	mr	3, 6
	blr
.Lrv_BB16_8:
	li 3, 0
	blr
.Lrv_func_end16:
	.size	rv_k_dot_i16, .Lrv_func_end16-.Lrv_func_begin16

	.section	.rodata.cst4,"aM",@progbits,4
	.p2align	2, 0x0
.Lrv_CPI17_0:
	.long	0xcf000000
.Lrv_CPI17_1:
	.long	0x4effffff
	.section	.text.rv_k_f2i_r,"ax",@progbits
	.globl	rv_k_f2i_r
	.p2align	2
	.type	rv_k_f2i_r,@function
rv_k_f2i_r:
.Lrv_func_begin17:
	cmpwi	5, 1
	bltlr	0
	stwu 1, -32(1)
	lis 7, .Lrv_CPI17_0@ha
	lfs 0, .Lrv_CPI17_0@l(7)
	lis 7, .Lrv_CPI17_1@ha
	lfs 1, .Lrv_CPI17_1@l(7)
	lis 7, 32767
	addi 4, 4, -4
	addi 3, 3, -4
	addi 6, 1, 28
	ori 7, 7, 65535
	b .Lrv_BB17_3
.Lrv_BB17_2:
	addi 5, 5, -1
	cmplwi	5, 0
	stwu 8, 4(3)
	beq	0, .Lrv_BB17_9
.Lrv_BB17_3:
	lfsu 2, 4(4)
	lis 8, -32768
	fcmpu 0, 2, 0
	fctiwz 3, 2
	cror 20, 0, 3
	stfiwx 3, 0, 6
	bc 4, 20, .Lrv_BB17_6
	fcmpu 0, 2, 1
	mr	9, 7
	ble 0, .Lrv_BB17_7
.Lrv_BB17_5:
	fcmpu 0, 2, 2
	li 8, 0
	bun 0, .Lrv_BB17_2
	b .Lrv_BB17_8
.Lrv_BB17_6:
	lwz 8, 28(1)
	fcmpu 0, 2, 1
	mr	9, 7
	bgt	0, .Lrv_BB17_5
.Lrv_BB17_7:
	mr	9, 8
	fcmpu 0, 2, 2
	li 8, 0
	bun 0, .Lrv_BB17_2
.Lrv_BB17_8:
	mr	8, 9
	b .Lrv_BB17_2
.Lrv_BB17_9:
	addi 1, 1, 32
	blr
.Lrv_func_end17:
	.size	rv_k_f2i_r, .Lrv_func_end17-.Lrv_func_begin17

	.section	.rodata.cst16,"aM",@progbits,16
	.p2align	4, 0x0
.Lrv_CPI18_0:
	.long	0x3e800000
	.long	0x3e800000
	.long	0x3e800000
	.long	0x3e800000
.Lrv_CPI18_1:
	.long	0x3f000000
	.long	0x3f000000
	.long	0x3f000000
	.long	0x3f000000
	.section	.rodata.cst4,"aM",@progbits,4
	.p2align	2, 0x0
.Lrv_CPI18_2:
	.long	0x3e800000
.Lrv_CPI18_3:
	.long	0x3f000000
	.section	.text.rv_k_fir3_f32_r,"ax",@progbits
	.globl	rv_k_fir3_f32_r
	.p2align	2
	.type	rv_k_fir3_f32_r,@function
rv_k_fir3_f32_r:
.Lrv_func_begin18:
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
	blt	0, .Lrv_BB18_7
	slwi 8, 7, 2
	add 9, 4, 8
	addi 9, 9, 8
	cmplw	3, 9
	add 8, 3, 8
	bc 4, 0, .Lrv_BB18_4
	cmplw	4, 8
	bc 12, 0, .Lrv_BB18_7
.Lrv_BB18_4:
	li 11, .Lrv_CPI18_0@l
	lis 12, .Lrv_CPI18_0@ha
	lvx128 2, 12, 11
	li 11, .Lrv_CPI18_1@l
	lis 12, .Lrv_CPI18_1@ha
	lvx128 9, 12, 11
	rlwinm 5, 7, 0, 2, 29
	addi 7, 4, 8
	addi 8, 4, 4
	li 9, 0
	li 10, 15
	addi 11, 1, 16
	mr	12, 5
.Lrv_BB18_5:
	add 30, 4, 9
	lvx128 5, 4, 9
	addi 12, 12, -4
	lvx128 0, 30, 10
	add 30, 8, 9
	lvx128 1, 8, 9
	cmplwi	12, 0
	lvx128 6, 30, 10
	add 30, 7, 9
	lvx128 7, 7, 9
	lvx128 8, 30, 10
	mr	30, 3
	lvsl 4, 4, 9
	vperm128 4, 5, 0, 4
	lvsl 3, 8, 9
	vperm128 3, 1, 6, 3
	vmulfp128 4, 4, 2
	lvsl 5, 7, 9
	vperm128 5, 7, 8, 5
	vmulfp128 3, 3, 9
	vmulfp128 5, 5, 2
	vaddfp128 3, 4, 3
	vaddfp128 3, 3, 5
	stvx128 3, 0, 11
	ld 0, 16(1)
	stdux 0, 30, 9
	ld 0, 24(1)
	addi 9, 9, 16
	std 0, 8(30)
	bne	0, .Lrv_BB18_5
	cmplw	6, 5
	beq	0, .Lrv_BB18_9
.Lrv_BB18_7:
	sub	6, 6, 5
	slwi 5, 5, 2
	addi 5, 5, -4
	lis 7, .Lrv_CPI18_2@ha
	add 4, 4, 5
	add 3, 3, 5
	lis 5, .Lrv_CPI18_3@ha
	lfs 0, .Lrv_CPI18_2@l(7)
	lfs 1, .Lrv_CPI18_3@l(5)
.Lrv_BB18_8:
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
	bne	0, .Lrv_BB18_8
.Lrv_BB18_9:
	ld 30, 40(1)
	addi 1, 1, 64
	blr
.Lrv_func_end18:
	.size	rv_k_fir3_f32_r, .Lrv_func_end18-.Lrv_func_begin18

	.section	.text.rv_k_g_add_f32,"ax",@progbits
	.globl	rv_k_g_add_f32
	.p2align	2
	.type	rv_k_g_add_f32,@function
rv_k_g_add_f32:
.Lrv_func_begin19:
	lis 4, rv_g_a@ha
	lis 5, rv_g_b@ha
	lis 6, rv_g_c@ha
	li 3, 0
	la 4, rv_g_a@l(4)
	la 5, rv_g_b@l(5)
	la 6, rv_g_c@l(6)
.Lrv_BB19_1:
	lvx128 2, 4, 3
	lvx128 3, 5, 3
	vaddfp128 2, 2, 3
	stvx128 2, 6, 3
	addi 3, 3, 16
	cmplwi	3, 4096
	bne	0, .Lrv_BB19_1
	blr
.Lrv_func_end19:
	.size	rv_k_g_add_f32, .Lrv_func_end19-.Lrv_func_begin19

	.section	.text.rv_k_g_add_i32,"ax",@progbits
	.globl	rv_k_g_add_i32
	.p2align	2
	.type	rv_k_g_add_i32,@function
rv_k_g_add_i32:
.Lrv_func_begin20:
	lis 4, rv_g_ia@ha
	lis 5, rv_g_ib@ha
	lis 6, rv_g_ic@ha
	li 3, 0
	la 4, rv_g_ia@l(4)
	la 5, rv_g_ib@l(5)
	la 6, rv_g_ic@l(6)
.Lrv_BB20_1:
	lvx128 2, 4, 3
	lvx128 3, 5, 3
	vadduwm 2, 3, 2
	stvx128 2, 6, 3
	addi 3, 3, 16
	cmplwi	3, 4096
	bne	0, .Lrv_BB20_1
	blr
.Lrv_func_end20:
	.size	rv_k_g_add_i32, .Lrv_func_end20-.Lrv_func_begin20

	.section	.rodata.cst4,"aM",@progbits,4
	.p2align	2, 0x0
.Lrv_CPI21_0:
	.long	0x00000000
	.section	.text.rv_k_g_dot,"ax",@progbits
	.globl	rv_k_g_dot
	.p2align	2
	.type	rv_k_g_dot,@function
rv_k_g_dot:
.Lrv_func_begin21:
	lis 6, .Lrv_CPI21_0@ha
	lfs 1, .Lrv_CPI21_0@l(6)
	lis 4, rv_g_b@ha
	lis 5, rv_g_a@ha
	la 4, rv_g_b@l(4)
	la 5, rv_g_a@l(5)
	li 3, 0
	addi 4, 4, -4
	addi 5, 5, -4
.Lrv_BB21_1:
	lfsu 0, 4(4)
	addi 3, 3, 4
	lfsu 2, 4(5)
	cmplwi	3, 4096
	fmuls 0, 2, 0
	fadds 1, 1, 0
	bne	0, .Lrv_BB21_1
	blr
.Lrv_func_end21:
	.size	rv_k_g_dot, .Lrv_func_end21-.Lrv_func_begin21

	.section	.text.rv_k_g_saxpy,"ax",@progbits
	.globl	rv_k_g_saxpy
	.p2align	2
	.type	rv_k_g_saxpy,@function
rv_k_g_saxpy:
.Lrv_func_begin22:
	stwu 1, -32(1)
	addi 4, 1, 16
	stfs 1, 16(1)
	lis 5, rv_g_a@ha
	lvx128 2, 0, 4
	lis 6, rv_g_b@ha
	lis 7, rv_g_c@ha
	li 3, 0
	la 4, rv_g_a@l(5)
	la 5, rv_g_b@l(6)
	vspltw128 2, 2, 0
	la 6, rv_g_c@l(7)
.Lrv_BB22_1:
	lvx128 3, 4, 3
	lvx128 4, 5, 3
	vmulfp128 3, 2, 3
	vaddfp128 3, 3, 4
	stvx128 3, 6, 3
	addi 3, 3, 16
	cmplwi	3, 4096
	bne	0, .Lrv_BB22_1
	addi 1, 1, 32
	blr
.Lrv_func_end22:
	.size	rv_k_g_saxpy, .Lrv_func_end22-.Lrv_func_begin22

	.section	.text.rv_k_gather_f32_r,"ax",@progbits
	.globl	rv_k_gather_f32_r
	.p2align	2
	.type	rv_k_gather_f32_r,@function
rv_k_gather_f32_r:
.Lrv_func_begin23:
	cmpwi	6, 1
	bltlr	0
	addi 5, 5, -4
	addi 3, 3, -4
.Lrv_BB23_2:
	lwzu 7, 4(5)
	addi 6, 6, -1
	cmplwi	6, 0
	slwi 7, 7, 2
	lwzx 7, 4, 7
	stwu 7, 4(3)
	bne	0, .Lrv_BB23_2
	blr
.Lrv_func_end23:
	.size	rv_k_gather_f32_r, .Lrv_func_end23-.Lrv_func_begin23

	.section	.text.rv_k_i2f_r,"ax",@progbits
	.globl	rv_k_i2f_r
	.p2align	2
	.type	rv_k_i2f_r,@function
rv_k_i2f_r:
.Lrv_func_begin24:
	cmpwi	5, 1
	bltlr	0
	stwu 1, -32(1)
	addi 4, 4, -4
	addi 3, 3, -4
.Lrv_BB24_2:
	lwa 6, 4(4)
	addi 5, 5, -1
	cmplwi	5, 0
	addi 4, 4, 4
	std 6, 24(1)
	lfd 0, 24(1)
	fcfid 0, 0
	frsp 0, 0
	stfsu 0, 4(3)
	bne	0, .Lrv_BB24_2
	addi 1, 1, 32
	blr
.Lrv_func_end24:
	.size	rv_k_i2f_r, .Lrv_func_end24-.Lrv_func_begin24

	.section	.rodata.cst16,"aM",@progbits,16
	.p2align	4, 0x0
.Lrv_CPI25_0:
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
.Lrv_CPI25_1:
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
.Lrv_CPI25_2:
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
.Lrv_CPI25_3:
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
.Lrv_CPI25_4:
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
	.section	.text.rv_k_mat4_vec4_r,"ax",@progbits
	.globl	rv_k_mat4_vec4_r
	.p2align	2
	.type	rv_k_mat4_vec4_r,@function
rv_k_mat4_vec4_r:
.Lrv_func_begin25:
	cmpwi	6, 1
	bltlr	0
	stwu 1, -432(1)
	cmplwi	6, 4
	li 7, 0
	std 25, 368(1)
	std 26, 376(1)
	std 27, 384(1)
	std 28, 392(1)
	std 29, 400(1)
	std 30, 408(1)
	stfd 29, 344(1)
	stfd 30, 352(1)
	stfd 31, 360(1)
	bge 0, .Lrv_BB25_5
.Lrv_BB25_2:
	sub	6, 6, 7
	slwi 7, 7, 4
	addi 7, 7, -16
	add 5, 5, 7
	add 3, 3, 7
.Lrv_BB25_3:
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
	bne	0, .Lrv_BB25_3
.Lrv_BB25_4:
	lfd 31, 360(1)
	ld 30, 408(1)
	ld 29, 400(1)
	ld 28, 392(1)
	ld 27, 384(1)
	ld 26, 376(1)
	ld 25, 368(1)
	lfd 30, 352(1)
	lfd 29, 344(1)
	addi 1, 1, 432
	blr
.Lrv_BB25_5:
	slwi 9, 6, 4
	addi 10, 4, 64
	add 8, 3, 9
	add 9, 5, 9
	cmplw 5, 3, 9
	cmplw	3, 10
	cmplw 1, 4, 8
	bc 4, 20, .Lrv_BB25_7
	cmplw 5, 5, 8
	bc 12, 20, .Lrv_BB25_2
.Lrv_BB25_7:
	crand 20, 0, 4
	bc 12, 20, .Lrv_BB25_2
	lwz 7, 60(4)
	li 8, .Lrv_CPI25_0@l
	lis 9, .Lrv_CPI25_0@ha
	lvx128 2, 9, 8
	li 8, .Lrv_CPI25_2@l
	stw 7, 80(1)
	lis 9, .Lrv_CPI25_2@ha
	lwz 7, 44(4)
	li 10, .Lrv_CPI25_1@l
	lvx128 4, 9, 8
	addi 8, 1, 80
	lvx128 1, 0, 8
	addi 8, 1, 96
	stw 7, 96(1)
	addi 9, 1, 240
	lwz 7, 28(4)
	lis 11, .Lrv_CPI25_1@ha
	lvx128 6, 0, 8
	addi 8, 1, 112
	stw 7, 112(1)
	addi 12, 1, 288
	lwz 7, 12(4)
	addi 0, 1, 304
	lvx128 7, 0, 8
	addi 8, 1, 128
	stw 7, 128(1)
	addi 30, 1, 320
	lwz 7, 56(4)
	lis 27, .Lrv_CPI25_3@ha
	lvx128 8, 0, 8
	addi 8, 1, 144
	stw 7, 144(1)
	li 26, .Lrv_CPI25_4@l
	lwz 7, 40(4)
	vspltw128 25, 1, 0
	vspltw128 6, 6, 0
	vspltw128 7, 7, 0
	vspltw128 8, 8, 0
	lvx128 9, 0, 8
	addi 8, 1, 160
	stw 7, 160(1)
	lwz 7, 24(4)
	vspltw128 9, 9, 0
	lvx128 10, 0, 8
	addi 8, 1, 176
	stw 7, 176(1)
	lwz 7, 8(4)
	vspltw128 10, 10, 0
	lvx128 11, 0, 8
	addi 8, 1, 192
	stw 7, 192(1)
	lwz 7, 52(4)
	vspltw128 11, 11, 0
	lvx128 12, 0, 8
	addi 8, 1, 208
	stw 7, 208(1)
	lwz 7, 36(4)
	vspltw128 12, 12, 0
	lvx128 13, 0, 8
	addi 8, 1, 224
	stw 7, 224(1)
	lwz 7, 20(4)
	vspltw128 13, 13, 0
	lvx128 14, 0, 8
	stw 7, 240(1)
	rlwinm 7, 6, 0, 1, 29
	lwz 8, 4(4)
	vspltw128 14, 14, 0
	lvx128 15, 0, 9
	addi 9, 1, 256
	stw 8, 256(1)
	lwz 8, 48(4)
	vspltw128 15, 15, 0
	lvx128 16, 0, 9
	li 9, 32
	stw 8, 272(1)
	li 8, 0
	lvx128 3, 11, 10
	addi 10, 1, 272
	lwz 11, 32(4)
	vspltw128 16, 16, 0
	lvx128 17, 0, 10
	li 10, 16
	stw 11, 288(1)
	li 11, 63
	lwz 29, 16(4)
	vspltw128 17, 17, 0
	lvx128 18, 0, 12
	li 12, 48
	stw 29, 304(1)
	li 29, .Lrv_CPI25_3@l
	lwz 28, 0(4)
	vspltw128 18, 18, 0
	lvx128 19, 0, 0
	addi 0, 1, 16
	stw 28, 320(1)
	lis 28, .Lrv_CPI25_4@ha
	lvx128 31, 0, 30
	addi 30, 1, 64
	lvx128 5, 27, 29
	addi 29, 1, 48
	lvx128 0, 28, 26
	vspltw128 19, 19, 0
	vspltw128 31, 31, 0
	addi 28, 1, 32
	mr	27, 7
.Lrv_BB25_9:
	add 26, 5, 8
	lvx128 30, 26, 9
	mr	25, 3
	lvx128 29, 26, 10
	addi 27, 27, -4
	lvx128 28, 5, 8
	cmplwi	27, 0
	lvx128 27, 26, 11
	lvx128 26, 26, 12
	lvsl 1, 5, 8
	vperm128 24, 29, 30, 1
	vperm128 29, 28, 29, 1
	vperm128 28, 26, 27, 1
	vperm128 1, 30, 26, 1
	vmrghw128 27, 29, 24
	vperm128 26, 29, 24, 4
	vmrglw128 21, 29, 24
	vperm128 29, 29, 24, 0
	vperm128 30, 1, 28, 2
	vmrghw128 23, 1, 28
	vperm128 22, 1, 28, 5
	vmrglw128 1, 1, 28
	vperm128 30, 27, 30, 3
	vperm128 27, 26, 23, 3
	vperm128 28, 21, 22, 3
	vperm128 1, 29, 1, 3
	vmulfp128 26, 30, 31
	vmulfp128 24, 27, 19
	vmulfp128 23, 30, 16
	vmulfp128 22, 27, 15
	vmulfp128 21, 30, 12
	vmulfp128 20, 27, 11
	vmulfp128 30, 30, 8
	vmulfp128 27, 27, 7
	vaddfp128 29, 26, 24
	vaddfp128 26, 23, 22
	vmulfp128 24, 28, 18
	vaddfp128 23, 21, 20
	vmulfp128 22, 28, 14
	vaddfp128 30, 30, 27
	vmulfp128 27, 28, 10
	vmulfp128 28, 28, 6
	vaddfp128 29, 29, 24
	vaddfp128 26, 26, 22
	vmulfp128 24, 1, 17
	vaddfp128 27, 23, 27
	vmulfp128 23, 1, 13
	vaddfp128 30, 30, 28
	vmulfp128 28, 1, 9
	vmulfp128 1, 1, 25
	vaddfp128 29, 29, 24
	vaddfp128 26, 26, 23
	vaddfp128 28, 27, 28
	vaddfp128 1, 30, 1
	vperm128 30, 28, 1, 2
	vmrghw128 27, 29, 26
	vperm128 24, 29, 26, 0
	vmrglw128 23, 28, 1
	vperm128 30, 27, 30, 3
	vmrglw128 27, 29, 26
	vperm128 29, 29, 26, 4
	vperm128 24, 24, 23, 3
	stvx128 30, 0, 0
	vperm128 30, 28, 1, 5
	vmrghw128 1, 28, 1
	vperm128 30, 27, 30, 3
	vperm128 1, 29, 1, 3
	stvx128 24, 0, 30
	stvx128 30, 0, 29
	stvx128 1, 0, 28
	ld 26, 16(1)
	stdux 26, 25, 8
	ld 26, 72(1)
	addi 8, 8, 64
	std 26, 56(25)
	ld 26, 64(1)
	std 26, 48(25)
	ld 26, 56(1)
	std 26, 40(25)
	ld 26, 48(1)
	std 26, 32(25)
	ld 26, 40(1)
	std 26, 24(25)
	ld 26, 32(1)
	std 26, 16(25)
	ld 26, 24(1)
	std 26, 8(25)
	bne	0, .Lrv_BB25_9
	cmplw	6, 7
	bne	0, .Lrv_BB25_2
	b .Lrv_BB25_4
.Lrv_func_end25:
	.size	rv_k_mat4_vec4_r, .Lrv_func_end25-.Lrv_func_begin25

	.section	.text.rv_k_max_f32,"ax",@progbits
	.globl	rv_k_max_f32
	.p2align	2
	.type	rv_k_max_f32,@function
rv_k_max_f32:
.Lrv_func_begin26:
	mflr 0
	stw 0, -8(1)
	stwu 1, -96(1)
	cmpwi	4, 1
	blt-	0, .Lrv_BB26_8
	lfs 0, 0(3)
	slwi 4, 4, 2
	cmplwi	4, 4
	bne	0, .Lrv_BB26_4
	fmr 1, 0
.Lrv_BB26_3:
	addi 1, 1, 96
	lwz 0, -8(1)
	mtlr 0
	blr
.Lrv_BB26_4:
	addi 4, 4, -4
	b .Lrv_BB26_6
.Lrv_BB26_5:
	addi 4, 4, -4
	cmplwi	4, 0
	fmr 0, 1
	beq	0, .Lrv_BB26_3
.Lrv_BB26_6:
	lfsu 1, 4(3)
	fcmpu 0, 1, 0
	bgt	0, .Lrv_BB26_5
	fmr 1, 0
	b .Lrv_BB26_5
.Lrv_BB26_8:
	lis 3, .Lrv__MergedGlobals@ha
	la 5, .Lrv__MergedGlobals@l(3)
	li 3, 0
	li 4, 0
	bl _RNvNtCsedlPyG8v7DD_4core9panicking18panic_bounds_check
.Lrv_func_end26:
	.size	rv_k_max_f32, .Lrv_func_end26-.Lrv_func_begin26

	.section	.text.rv_k_max_i32_r,"ax",@progbits
	.globl	rv_k_max_i32_r
	.p2align	2
	.type	rv_k_max_i32_r,@function
rv_k_max_i32_r:
.Lrv_func_begin27:
	cmpwi	6, 1
	bltlr	0
	stwu 1, -64(1)
	cmplwi	6, 8
	li 7, 0
	std 30, 40(1)
	bge 0, .Lrv_BB27_6
.Lrv_BB27_2:
	sub	6, 6, 7
	slwi 7, 7, 2
	addi 7, 7, -4
	add 4, 4, 7
	add 5, 5, 7
	add 3, 3, 7
	b .Lrv_BB27_4
.Lrv_BB27_3:
	addi 6, 6, -1
	cmplwi	6, 0
	stwu 7, 4(3)
	beq	0, .Lrv_BB27_11
.Lrv_BB27_4:
	lwzu 7, 4(4)
	lwzu 8, 4(5)
	cmpw	7, 8
	bgt	0, .Lrv_BB27_3
	mr	7, 8
	b .Lrv_BB27_3
.Lrv_BB27_6:
	sub	8, 3, 4
	cmplwi	8, 16
	sub	8, 3, 5
	bc 12, 0, .Lrv_BB27_2
	cmplwi	8, 16
	bc 12, 0, .Lrv_BB27_2
	rlwinm 7, 6, 0, 1, 29
	li 8, 0
	li 9, 15
	addi 10, 1, 16
	mr	11, 7
.Lrv_BB27_9:
	add 12, 4, 8
	lvx128 2, 4, 8
	mr	30, 3
	lvx128 3, 12, 9
	add 12, 5, 8
	lvx128 4, 5, 8
	addi 11, 11, -4
	lvx128 5, 12, 9
	cmplwi	11, 0
	lvsl 0, 4, 8
	vperm128 2, 2, 3, 0
	lvsl 1, 5, 8
	vperm128 3, 4, 5, 1
	vmaxsw 2, 2, 3
	stvx128 2, 0, 10
	ld 12, 16(1)
	stdux 12, 30, 8
	ld 12, 24(1)
	addi 8, 8, 16
	std 12, 8(30)
	bne	0, .Lrv_BB27_9
	cmplw	6, 7
	bne	0, .Lrv_BB27_2
.Lrv_BB27_11:
	ld 30, 40(1)
	addi 1, 1, 64
	blr
.Lrv_func_end27:
	.size	rv_k_max_i32_r, .Lrv_func_end27-.Lrv_func_begin27

	.section	.text.rv_k_min_f32_r,"ax",@progbits
	.globl	rv_k_min_f32_r
	.p2align	2
	.type	rv_k_min_f32_r,@function
rv_k_min_f32_r:
.Lrv_func_begin28:
	cmpwi	6, 1
	bltlr	0
	stwu 1, -64(1)
	cmplwi	6, 8
	li 7, 0
	std 30, 40(1)
	bge 0, .Lrv_BB28_6
.Lrv_BB28_2:
	sub	6, 6, 7
	slwi 7, 7, 2
	addi 7, 7, -4
	add 4, 4, 7
	add 5, 5, 7
	add 3, 3, 7
	b .Lrv_BB28_4
.Lrv_BB28_3:
	addi 6, 6, -1
	cmplwi	6, 0
	stfsu 0, 4(3)
	beq	0, .Lrv_BB28_11
.Lrv_BB28_4:
	lfsu 0, 4(4)
	lfsu 1, 4(5)
	fcmpu 0, 0, 1
	blt	0, .Lrv_BB28_3
	fmr 0, 1
	b .Lrv_BB28_3
.Lrv_BB28_6:
	sub	8, 3, 4
	cmplwi	8, 16
	sub	8, 3, 5
	bc 12, 0, .Lrv_BB28_2
	cmplwi	8, 16
	bc 12, 0, .Lrv_BB28_2
	rlwinm 7, 6, 0, 1, 29
	li 8, 0
	li 9, 15
	addi 10, 1, 16
	mr	11, 7
.Lrv_BB28_9:
	add 12, 4, 8
	lvx128 2, 4, 8
	mr	30, 3
	lvx128 3, 12, 9
	add 12, 5, 8
	lvx128 4, 5, 8
	addi 11, 11, -4
	lvx128 5, 12, 9
	cmplwi	11, 0
	lvsl 0, 4, 8
	vperm128 2, 2, 3, 0
	lvsl 1, 5, 8
	vperm128 3, 4, 5, 1
	vcmpgtfp128 4, 3, 2
	vsel128 4, 3, 2, 4
	stvx128 4, 0, 10
	ld 12, 16(1)
	stdux 12, 30, 8
	ld 12, 24(1)
	addi 8, 8, 16
	std 12, 8(30)
	bne	0, .Lrv_BB28_9
	cmplw	6, 7
	bne	0, .Lrv_BB28_2
.Lrv_BB28_11:
	ld 30, 40(1)
	addi 1, 1, 64
	blr
.Lrv_func_end28:
	.size	rv_k_min_f32_r, .Lrv_func_end28-.Lrv_func_begin28

	.section	.text.rv_k_mul_f32_r,"ax",@progbits
	.globl	rv_k_mul_f32_r
	.p2align	2
	.type	rv_k_mul_f32_r,@function
rv_k_mul_f32_r:
.Lrv_func_begin29:
	cmpwi	6, 1
	bltlr	0
	stwu 1, -64(1)
	cmplwi	6, 8
	li 7, 0
	std 30, 40(1)
	bge 0, .Lrv_BB29_5
.Lrv_BB29_2:
	sub	6, 6, 7
	slwi 7, 7, 2
	addi 7, 7, -4
	add 4, 4, 7
	add 5, 5, 7
	add 3, 3, 7
.Lrv_BB29_3:
	lfsu 0, 4(4)
	addi 6, 6, -1
	lfsu 1, 4(5)
	cmplwi	6, 0
	fmuls 0, 0, 1
	stfsu 0, 4(3)
	bne	0, .Lrv_BB29_3
.Lrv_BB29_4:
	ld 30, 40(1)
	addi 1, 1, 64
	blr
.Lrv_BB29_5:
	sub	8, 3, 4
	cmplwi	8, 16
	sub	8, 3, 5
	bc 12, 0, .Lrv_BB29_2
	cmplwi	8, 16
	bc 12, 0, .Lrv_BB29_2
	rlwinm 7, 6, 0, 1, 29
	li 8, 0
	li 9, 15
	addi 10, 1, 16
	mr	11, 7
.Lrv_BB29_8:
	add 12, 4, 8
	lvx128 2, 4, 8
	mr	30, 3
	lvx128 3, 12, 9
	add 12, 5, 8
	lvx128 4, 5, 8
	addi 11, 11, -4
	lvx128 5, 12, 9
	cmplwi	11, 0
	lvsl 0, 4, 8
	vperm128 2, 2, 3, 0
	lvsl 1, 5, 8
	vperm128 3, 4, 5, 1
	vmulfp128 2, 2, 3
	stvx128 2, 0, 10
	ld 12, 16(1)
	stdux 12, 30, 8
	ld 12, 24(1)
	addi 8, 8, 16
	std 12, 8(30)
	bne	0, .Lrv_BB29_8
	cmplw	6, 7
	bne	0, .Lrv_BB29_2
	b .Lrv_BB29_4
.Lrv_func_end29:
	.size	rv_k_mul_f32_r, .Lrv_func_end29-.Lrv_func_begin29

	.section	.text.rv_k_mul_i32_r,"ax",@progbits
	.globl	rv_k_mul_i32_r
	.p2align	2
	.type	rv_k_mul_i32_r,@function
rv_k_mul_i32_r:
.Lrv_func_begin30:
	cmpwi	6, 1
	bltlr	0
	addi 4, 4, -4
	addi 5, 5, -4
	addi 3, 3, -4
.Lrv_BB30_2:
	lwzu 7, 4(4)
	addi 6, 6, -1
	lwzu 8, 4(5)
	cmplwi	6, 0
	mullw 7, 8, 7
	stwu 7, 4(3)
	bne	0, .Lrv_BB30_2
	blr
.Lrv_func_end30:
	.size	rv_k_mul_i32_r, .Lrv_func_end30-.Lrv_func_begin30

	.section	.text.rv_k_offset1_f32_r,"ax",@progbits
	.globl	rv_k_offset1_f32_r
	.p2align	2
	.type	rv_k_offset1_f32_r,@function
rv_k_offset1_f32_r:
.Lrv_func_begin31:
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
	blt	0, .Lrv_BB31_7
	slwi 8, 7, 2
	add 9, 4, 8
	addi 9, 9, 4
	cmplw	3, 9
	add 8, 3, 8
	bc 4, 0, .Lrv_BB31_4
	cmplw	4, 8
	bc 12, 0, .Lrv_BB31_7
.Lrv_BB31_4:
	rlwinm 5, 7, 0, 2, 29
	addi 7, 4, 4
	li 8, 0
	li 9, 15
	addi 10, 1, 16
	mr	11, 5
.Lrv_BB31_5:
	add 12, 4, 8
	lvx128 2, 4, 8
	mr	30, 3
	lvx128 3, 12, 9
	add 12, 7, 8
	lvx128 4, 7, 8
	addi 11, 11, -4
	lvx128 5, 12, 9
	cmplwi	11, 0
	lvsl 0, 4, 8
	vperm128 2, 2, 3, 0
	lvsl 1, 7, 8
	vperm128 3, 4, 5, 1
	vaddfp128 2, 2, 3
	stvx128 2, 0, 10
	ld 12, 16(1)
	stdux 12, 30, 8
	ld 12, 24(1)
	addi 8, 8, 16
	std 12, 8(30)
	bne	0, .Lrv_BB31_5
	cmplw	6, 5
	beq	0, .Lrv_BB31_9
.Lrv_BB31_7:
	sub	6, 6, 5
	slwi 5, 5, 2
	addi 5, 5, -4
	add 4, 4, 5
	add 3, 3, 5
.Lrv_BB31_8:
	lfsu 0, 4(4)
	addi 6, 6, -1
	cmplwi	6, 0
	lfs 1, 4(4)
	fadds 0, 0, 1
	stfsu 0, 4(3)
	bne	0, .Lrv_BB31_8
.Lrv_BB31_9:
	ld 30, 40(1)
	addi 1, 1, 64
	blr
.Lrv_func_end31:
	.size	rv_k_offset1_f32_r, .Lrv_func_end31-.Lrv_func_begin31

	.section	.text.rv_k_prefix_f32,"ax",@progbits
	.globl	rv_k_prefix_f32
	.p2align	2
	.type	rv_k_prefix_f32,@function
rv_k_prefix_f32:
.Lrv_func_begin32:
	cmpwi	4, 2
	bltlr	0
	lfs 0, 0(3)
	addi 4, 4, -1
.Lrv_BB32_2:
	lfsu 1, 4(3)
	addi 4, 4, -1
	cmplwi	4, 0
	fadds 0, 0, 1
	stfs 0, 0(3)
	bne	0, .Lrv_BB32_2
	blr
.Lrv_func_end32:
	.size	rv_k_prefix_f32, .Lrv_func_end32-.Lrv_func_begin32

	.section	.text.rv_k_saxpy,"ax",@progbits
	.globl	rv_k_saxpy
	.p2align	2
	.type	rv_k_saxpy,@function
rv_k_saxpy:
.Lrv_func_begin33:
	cmpwi	6, 1
	bltlr	0
	stwu 1, -48(1)
	cmplwi	6, 8
	li 5, 0
	blt	0, .Lrv_BB33_7
	slwi 7, 6, 2
	add 8, 4, 7
	cmplw	3, 8
	add 7, 3, 7
	bc 4, 0, .Lrv_BB33_4
	cmplw	4, 7
	bc 12, 0, .Lrv_BB33_7
.Lrv_BB33_4:
	stfs 1, 32(1)
	addi 5, 1, 32
	lvx128 2, 0, 5
	rlwinm 5, 6, 0, 1, 29
	li 7, 0
	li 8, 15
	vspltw128 2, 2, 0
	addi 9, 1, 16
	mr	10, 5
.Lrv_BB33_5:
	add 11, 4, 7
	lvx128 3, 4, 7
	addi 10, 10, -4
	lvx128 4, 11, 8
	add 11, 3, 7
	lvx128 5, 3, 7
	cmplwi	10, 0
	lvx128 0, 11, 8
	lvsl 1, 4, 7
	vperm128 3, 3, 4, 1
	lvsl 4, 3, 7
	vperm128 4, 5, 0, 4
	vmulfp128 3, 2, 3
	vaddfp128 3, 3, 4
	stvx128 3, 0, 9
	ld 12, 16(1)
	stdx 12, 3, 7
	addi 7, 7, 16
	ld 12, 24(1)
	std 12, 8(11)
	bne	0, .Lrv_BB33_5
	cmplw	6, 5
	beq	0, .Lrv_BB33_9
.Lrv_BB33_7:
	slwi 7, 5, 2
	sub	5, 6, 5
	addi 6, 7, -4
	add 4, 4, 6
	add 3, 3, 6
.Lrv_BB33_8:
	lfsu 0, 4(4)
	addi 5, 5, -1
	lfsu 2, 4(3)
	cmplwi	5, 0
	fmuls 0, 1, 0
	fadds 0, 0, 2
	stfs 0, 0(3)
	bne	0, .Lrv_BB33_8
.Lrv_BB33_9:
	addi 1, 1, 48
	blr
.Lrv_func_end33:
	.size	rv_k_saxpy, .Lrv_func_end33-.Lrv_func_begin33

	.section	.text.rv_k_saxpy_r,"ax",@progbits
	.globl	rv_k_saxpy_r
	.p2align	2
	.type	rv_k_saxpy_r,@function
rv_k_saxpy_r:
.Lrv_func_begin34:
	cmpwi	6, 1
	bltlr	0
	stwu 1, -48(1)
	cmplwi	6, 8
	li 5, 0
	blt	0, .Lrv_BB34_7
	slwi 7, 6, 2
	add 8, 4, 7
	cmplw	3, 8
	add 7, 3, 7
	bc 4, 0, .Lrv_BB34_4
	cmplw	4, 7
	bc 12, 0, .Lrv_BB34_7
.Lrv_BB34_4:
	stfs 1, 32(1)
	addi 5, 1, 32
	lvx128 2, 0, 5
	rlwinm 5, 6, 0, 1, 29
	li 7, 0
	li 8, 15
	vspltw128 2, 2, 0
	addi 9, 1, 16
	mr	10, 5
.Lrv_BB34_5:
	add 11, 4, 7
	lvx128 3, 4, 7
	addi 10, 10, -4
	lvx128 4, 11, 8
	add 11, 3, 7
	lvx128 5, 3, 7
	cmplwi	10, 0
	lvx128 0, 11, 8
	lvsl 1, 4, 7
	vperm128 3, 3, 4, 1
	lvsl 4, 3, 7
	vperm128 4, 5, 0, 4
	vmulfp128 3, 2, 3
	vaddfp128 3, 3, 4
	stvx128 3, 0, 9
	ld 12, 16(1)
	stdx 12, 3, 7
	addi 7, 7, 16
	ld 12, 24(1)
	std 12, 8(11)
	bne	0, .Lrv_BB34_5
	cmplw	6, 5
	beq	0, .Lrv_BB34_9
.Lrv_BB34_7:
	sub	6, 6, 5
	slwi 5, 5, 2
	addi 5, 5, -4
	add 4, 4, 5
	add 3, 3, 5
.Lrv_BB34_8:
	lfsu 0, 4(4)
	addi 6, 6, -1
	lfsu 2, 4(3)
	cmplwi	6, 0
	fmuls 0, 1, 0
	fadds 0, 0, 2
	stfs 0, 0(3)
	bne	0, .Lrv_BB34_8
.Lrv_BB34_9:
	addi 1, 1, 48
	blr
.Lrv_func_end34:
	.size	rv_k_saxpy_r, .Lrv_func_end34-.Lrv_func_begin34

	.section	.text.rv_k_scale_inplace,"ax",@progbits
	.globl	rv_k_scale_inplace
	.p2align	2
	.type	rv_k_scale_inplace,@function
rv_k_scale_inplace:
.Lrv_func_begin35:
	cmpwi	5, 1
	bltlr	0
	srawi 4, 5, 31
	andc 4, 5, 4
	slwi 4, 4, 2
	addi 6, 4, -4
	cmplwi	6, 0
	mr	5, 3
	beq	0, .Lrv_BB35_5
	srwi 5, 6, 2
	addi 6, 5, 1
	rlwinm 7, 6, 0, 1, 30
	rlwinm 5, 6, 2, 0, 28
	add 5, 3, 5
	addi 8, 3, -8
	mr	9, 7
.Lrv_BB35_3:
	lfsu 0, 8(8)
	addi 9, 9, -2
	cmplwi	9, 0
	lfs 2, 4(8)
	fmuls 0, 1, 0
	stfs 0, 0(8)
	fmuls 0, 1, 2
	stfs 0, 4(8)
	bne	0, .Lrv_BB35_3
	cmplw	6, 7
	beqlr	0
.Lrv_BB35_5:
	add 3, 3, 4
	addi 4, 5, -4
.Lrv_BB35_6:
	lfsu 0, 4(4)
	addi 5, 4, 4
	fmuls 0, 1, 0
	cmplw	5, 3
	stfs 0, 0(4)
	bne	0, .Lrv_BB35_6
	blr
.Lrv_func_end35:
	.size	rv_k_scale_inplace, .Lrv_func_end35-.Lrv_func_begin35

	.section	.text.rv_k_set_f32,"ax",@progbits
	.globl	rv_k_set_f32
	.p2align	2
	.type	rv_k_set_f32,@function
rv_k_set_f32:
.Lrv_func_begin36:
	cmpwi	5, 1
	bltlr	0
	li 4, 0
	ble 0, .Lrv_BB36_5
	rlwinm 4, 5, 0, 1, 30
	addi 6, 3, -8
	mr	7, 4
.Lrv_BB36_3:
	addi 7, 7, -2
	stfsu 1, 8(6)
	cmplwi	7, 0
	stfs 1, 4(6)
	bne	0, .Lrv_BB36_3
	cmplw	5, 4
	beqlr	0
.Lrv_BB36_5:
	slwi 6, 4, 2
	add 3, 6, 3
	sub	4, 5, 4
	addi 3, 3, -4
.Lrv_BB36_6:
	addi 4, 4, -1
	cmplwi	4, 0
	stfsu 1, 4(3)
	bne	0, .Lrv_BB36_6
	blr
.Lrv_func_end36:
	.size	rv_k_set_f32, .Lrv_func_end36-.Lrv_func_begin36

	.section	.text.rv_k_set_u32,"ax",@progbits
	.globl	rv_k_set_u32
	.p2align	2
	.type	rv_k_set_u32,@function
rv_k_set_u32:
.Lrv_func_begin37:
	cmpwi	5, 1
	bltlr	0
	li 6, 0
	ble 0, .Lrv_BB37_5
	rlwinm 6, 5, 0, 1, 30
	addi 7, 3, -8
	mr	8, 6
.Lrv_BB37_3:
	addi 8, 8, -2
	stwu 4, 8(7)
	cmplwi	8, 0
	stw 4, 4(7)
	bne	0, .Lrv_BB37_3
	cmplw	5, 6
	beqlr	0
.Lrv_BB37_5:
	slwi 7, 6, 2
	add 3, 7, 3
	sub	5, 5, 6
	addi 3, 3, -4
.Lrv_BB37_6:
	addi 5, 5, -1
	cmplwi	5, 0
	stwu 4, 4(3)
	bne	0, .Lrv_BB37_6
	blr
.Lrv_func_end37:
	.size	rv_k_set_u32, .Lrv_func_end37-.Lrv_func_begin37

	.section	.text.rv_k_shl_i32_r,"ax",@progbits
	.globl	rv_k_shl_i32_r
	.p2align	2
	.type	rv_k_shl_i32_r,@function
rv_k_shl_i32_r:
.Lrv_func_begin38:
	cmpwi	5, 1
	bltlr	0
	addi 4, 4, -4
	addi 3, 3, -4
.Lrv_BB38_2:
	lwzu 6, 4(4)
	addi 5, 5, -1
	cmplwi	5, 0
	slwi 6, 6, 3
	stwu 6, 4(3)
	bne	0, .Lrv_BB38_2
	blr
.Lrv_func_end38:
	.size	rv_k_shl_i32_r, .Lrv_func_end38-.Lrv_func_begin38

	.section	.rodata.cst16,"aM",@progbits,16
	.p2align	4, 0x0
.Lrv_CPI39_0:
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
.Lrv_CPI39_1:
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
	.section	.text.rv_k_stride2_f32_r,"ax",@progbits
	.globl	rv_k_stride2_f32_r
	.p2align	2
	.type	rv_k_stride2_f32_r,@function
rv_k_stride2_f32_r:
.Lrv_func_begin39:
	srawi 6, 5, 31
	andc 7, 5, 6
	clrlwi.	5, 7, 2
	beqlr	0
	stwu 1, -32(1)
	cmplwi	5, 8
	li 6, 0
	blt	0, .Lrv_BB39_7
	slwi 9, 7, 3
	slwi 8, 7, 2
	add 9, 4, 9
	cmplw	3, 9
	add 8, 3, 8
	bc 4, 0, .Lrv_BB39_4
	cmplw	4, 8
	bc 12, 0, .Lrv_BB39_7
.Lrv_BB39_4:
	li 9, .Lrv_CPI39_0@l
	lis 10, .Lrv_CPI39_0@ha
	li 11, .Lrv_CPI39_1@l
	lvx128 2, 10, 9
	lis 9, .Lrv_CPI39_1@ha
	lvx128 3, 9, 11
	rlwinm 6, 7, 0, 2, 29
	li 7, 31
	li 8, 16
	addi 9, 1, 16
	mr	10, 3
	mr	11, 4
	mr	12, 6
.Lrv_BB39_5:
	lvx128 4, 11, 7
	addi 12, 12, -4
	lvx128 5, 11, 8
	cmplwi	12, 0
	lvx128 0, 0, 11
	lvsl 1, 0, 11
	vperm128 4, 5, 4, 1
	addi 11, 11, 32
	vperm128 5, 0, 5, 1
	vperm128 0, 5, 4, 2
	vperm128 4, 5, 4, 3
	vaddfp128 4, 0, 4
	stvx128 4, 0, 9
	ld 0, 24(1)
	std 0, 8(10)
	ld 0, 16(1)
	std 0, 0(10)
	addi 10, 10, 16
	bne	0, .Lrv_BB39_5
	cmplw	5, 6
	beq	0, .Lrv_BB39_9
.Lrv_BB39_7:
	sub	5, 5, 6
	slwi 7, 6, 2
	slwi 6, 6, 3
	add 4, 6, 4
	add 6, 7, 3
	addi 3, 4, -8
	addi 4, 6, -4
.Lrv_BB39_8:
	lfsu 0, 8(3)
	addi 5, 5, -1
	cmplwi	5, 0
	lfs 1, 4(3)
	fadds 0, 0, 1
	stfsu 0, 4(4)
	bne	0, .Lrv_BB39_8
.Lrv_BB39_9:
	addi 1, 1, 32
	blr
.Lrv_func_end39:
	.size	rv_k_stride2_f32_r, .Lrv_func_end39-.Lrv_func_begin39

	.section	.rodata.cst4,"aM",@progbits,4
	.p2align	2, 0x0
.Lrv_CPI40_0:
	.long	0x00000000
	.section	.text.rv_k_sum_f32,"ax",@progbits
	.globl	rv_k_sum_f32
	.p2align	2
	.type	rv_k_sum_f32,@function
rv_k_sum_f32:
.Lrv_func_begin40:
	cmpwi	4, 1
	lis 5, .Lrv_CPI40_0@ha
	blt	0, .Lrv_BB40_4
	lfs 1, .Lrv_CPI40_0@l(5)
	srawi 6, 4, 31
	andc 4, 4, 6
	addi 3, 3, -4
	slwi 4, 4, 2
.Lrv_BB40_2:
	lfsu 0, 4(3)
	addi 4, 4, -4
	cmplwi	4, 0
	fadds 1, 1, 0
	bne	0, .Lrv_BB40_2
	blr
.Lrv_BB40_4:
	lfs 1, .Lrv_CPI40_0@l(5)
	blr
.Lrv_func_end40:
	.size	rv_k_sum_f32, .Lrv_func_end40-.Lrv_func_begin40

	.section	.text.rv_k_sum_i32,"ax",@progbits
	.globl	rv_k_sum_i32
	.p2align	2
	.type	rv_k_sum_i32,@function
rv_k_sum_i32:
.Lrv_func_begin41:
	cmpwi	4, 1
	blt	0, .Lrv_BB41_8
	stwu 1, -32(1)
	srawi 5, 4, 31
	andc 4, 4, 5
	slwi 5, 4, 2
	addi 7, 5, -4
	cmplwi	7, 28
	li 4, 0
	mr	6, 3
	blt	0, .Lrv_BB41_5
	srwi 4, 7, 2
	addi 4, 4, 1
	rlwinm 7, 4, 0, 1, 28
	rlwinm 6, 4, 2, 0, 26
	addi 8, 3, 16
	vspltisw128 2, 0
	li 9, 0
	add 6, 3, 6
	li 10, 15
	mr	11, 7
	vspltisw128 3, 0
.Lrv_BB41_3:
	add 12, 3, 9
	lvx128 4, 3, 9
	addi 11, 11, -8
	lvx128 5, 12, 10
	add 12, 8, 9
	lvx128 0, 8, 9
	cmplwi	11, 0
	lvx128 1, 12, 10
	lvsl 6, 3, 9
	vperm128 4, 4, 5, 6
	lvsl 5, 8, 9
	vperm128 5, 0, 1, 5
	vadduwm 2, 4, 2
	addi 9, 9, 32
	vadduwm 3, 5, 3
	bne	0, .Lrv_BB41_3
	vadduwm 2, 3, 2
	addi 8, 1, 16
	vsldoi128 3, 2, 2, 8
	cmplw	4, 7
	vadduwm 2, 2, 3
	vsldoi128 3, 2, 2, 4
	vadduwm 2, 2, 3
	stvx128 2, 0, 8
	lwz 4, 16(1)
	beq	0, .Lrv_BB41_7
.Lrv_BB41_5:
	add 3, 3, 5
	addi 5, 6, -4
.Lrv_BB41_6:
	lwzu 6, 4(5)
	addi 7, 5, 4
	cmplw	7, 3
	add 4, 6, 4
	bne	0, .Lrv_BB41_6
.Lrv_BB41_7:
	addi 1, 1, 32
	mr	3, 4
	blr
.Lrv_BB41_8:
	li 3, 0
	blr
.Lrv_func_end41:
	.size	rv_k_sum_i32, .Lrv_func_end41-.Lrv_func_begin41

	.section	.rodata.cst4,"aM",@progbits,4
	.p2align	2, 0x0
.Lrv_CPI42_0:
	.long	0x3b808081
	.section	.text.rv_k_u8_to_f32_r,"ax",@progbits
	.globl	rv_k_u8_to_f32_r
	.p2align	2
	.type	rv_k_u8_to_f32_r,@function
rv_k_u8_to_f32_r:
.Lrv_func_begin42:
	cmpwi	5, 1
	bltlr	0
	stwu 1, -32(1)
	lis 6, .Lrv_CPI42_0@ha
	lfs 0, .Lrv_CPI42_0@l(6)
	addi 4, 4, -1
	addi 3, 3, -4
.Lrv_BB42_2:
	lbzu 6, 1(4)
	addi 5, 5, -1
	cmplwi	5, 0
	std 6, 24(1)
	lfd 1, 24(1)
	fcfid 1, 1
	frsp 1, 1
	fmuls 1, 1, 0
	stfsu 1, 4(3)
	bne	0, .Lrv_BB42_2
	addi 1, 1, 32
	blr
.Lrv_func_end42:
	.size	rv_k_u8_to_f32_r, .Lrv_func_end42-.Lrv_func_begin42

	.section	.text.rv_k_zero_u8,"ax",@progbits
	.globl	rv_k_zero_u8
	.p2align	2
	.type	rv_k_zero_u8,@function
rv_k_zero_u8:
.Lrv_func_begin43:
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
.Lrv_func_end43:
	.size	rv_k_zero_u8, .Lrv_func_end43-.Lrv_func_begin43

	.section	.text.rv_sp_bright_s,"ax",@progbits
	.globl	rv_sp_bright_s
	.p2align	2
	.type	rv_sp_bright_s,@function
rv_sp_bright_s:
.Lrv_func_begin44:
	cmpwi	5, 1
	bltlr	0
	stwu 1, -32(1)
	cmplwi	5, 16
	sub	7, 3, 4
	li 6, 0
	bc 12, 0, .Lrv_BB44_7
	cmplwi	7, 16
	bc 12, 0, .Lrv_BB44_7
	vspltisb 2, 10
	rlwinm 6, 5, 0, 1, 27
	li 7, 0
	li 8, 15
	vslb 2, 2, 2
	addi 9, 1, 16
.Lrv_BB44_4:
	add 10, 4, 7
	lvx128 3, 4, 7
	mr	11, 3
	lvx128 4, 10, 8
	lvsl 5, 4, 7
	vperm128 3, 3, 4, 5
	vaddubs 3, 3, 2
	stvx128 3, 0, 9
	ld 10, 16(1)
	stdux 10, 11, 7
	ld 10, 24(1)
	addi 7, 7, 16
	cmplw	6, 7
	std 10, 8(11)
	bne	0, .Lrv_BB44_4
	cmplw	5, 6
	bne	0, .Lrv_BB44_7
.Lrv_BB44_6:
	addi 1, 1, 32
	blr
.Lrv_BB44_7:
	addi 7, 6, -1
	add 4, 4, 7
	add 3, 3, 7
	b .Lrv_BB44_9
.Lrv_BB44_8:
	addi 6, 6, 1
	cmplw	5, 6
	stbu 7, 1(3)
	beq	0, .Lrv_BB44_6
.Lrv_BB44_9:
	lbzu 7, 1(4)
	addi 7, 7, 40
	cmplwi	7, 255
	blt	0, .Lrv_BB44_8
	li 7, 255
	b .Lrv_BB44_8
.Lrv_func_end44:
	.size	rv_sp_bright_s, .Lrv_func_end44-.Lrv_func_begin44

	.section	.text.rv_sp_bright_v,"ax",@progbits
	.globl	rv_sp_bright_v
	.p2align	2
	.type	rv_sp_bright_v,@function
rv_sp_bright_v:
.Lrv_func_begin45:
	srawi 6, 5, 31
	andc 5, 5, 6
	rlwinm. 6, 5, 28, 4, 31
	beqlr	0
	vspltisb 2, 10
	rlwinm 5, 5, 0, 0, 27
	li 6, 0
	vslb 2, 2, 2
.Lrv_BB45_2:
	lvx128 3, 4, 6
	vaddubs 3, 3, 2
	stvx128 3, 3, 6
	addi 6, 6, 16
	cmplw	5, 6
	bne	0, .Lrv_BB45_2
	blr
.Lrv_func_end45:
	.size	rv_sp_bright_v, .Lrv_func_end45-.Lrv_func_begin45

	.section	.text.rv_sp_copy_v,"ax",@progbits
	.globl	rv_sp_copy_v
	.p2align	2
	.type	rv_sp_copy_v,@function
rv_sp_copy_v:
.Lrv_func_begin46:
	srawi 6, 5, 31
	andc 5, 5, 6
	rlwinm. 5, 5, 30, 2, 31
	beqlr	0
	li 6, 0
.Lrv_BB46_2:
	lvx128 2, 4, 6
	addi 5, 5, -1
	cmplwi	5, 0
	stvx128 2, 3, 6
	addi 6, 6, 16
	bne	0, .Lrv_BB46_2
	blr
.Lrv_func_end46:
	.size	rv_sp_copy_v, .Lrv_func_end46-.Lrv_func_begin46

	.section	.rodata.cst4,"aM",@progbits,4
	.p2align	2, 0x0
.Lrv_CPI47_0:
	.long	0x00000000
	.section	.text.rv_sp_dot_s,"ax",@progbits
	.globl	rv_sp_dot_s
	.p2align	2
	.type	rv_sp_dot_s,@function
rv_sp_dot_s:
.Lrv_func_begin47:
	srawi 6, 5, 31
	andc 5, 5, 6
	clrlwi.	5, 5, 2
	lis 6, .Lrv_CPI47_0@ha
	beq	0, .Lrv_BB47_4
	lfs 1, .Lrv_CPI47_0@l(6)
	addi 3, 3, -4
	addi 4, 4, -4
.Lrv_BB47_2:
	lfsu 0, 4(4)
	addi 5, 5, -1
	lfsu 2, 4(3)
	cmplwi	5, 0
	fmuls 0, 2, 0
	fadds 1, 1, 0
	bne	0, .Lrv_BB47_2
	blr
.Lrv_BB47_4:
	lfs 1, .Lrv_CPI47_0@l(6)
	blr
.Lrv_func_end47:
	.size	rv_sp_dot_s, .Lrv_func_end47-.Lrv_func_begin47

	.section	.text.rv_sp_dot_v,"ax",@progbits
	.globl	rv_sp_dot_v
	.p2align	2
	.type	rv_sp_dot_v,@function
rv_sp_dot_v:
.Lrv_func_begin48:
	srawi 6, 5, 31
	andc 5, 5, 6
	vspltisw128 2, 0
	rlwinm. 5, 5, 28, 4, 31
	beq	0, .Lrv_BB48_4
	li 6, 0
	li 7, 16
	li 8, 32
	li 9, 48
	vspltisw128 3, 0
	vspltisw128 4, 0
	vspltisw128 5, 0
.Lrv_BB48_2:
	lvx128 0, 3, 6
	add 10, 3, 6
	lvx128 1, 4, 6
	add 11, 4, 6
	lvx128 6, 10, 7
	addi 5, 5, -1
	lvx128 7, 11, 7
	vmulfp128 0, 0, 1
	cmplwi	5, 0
	lvx128 8, 10, 8
	vaddfp128 5, 5, 0
	addi 6, 6, 64
	lvx128 9, 11, 8
	vmulfp128 1, 6, 7
	lvx128 10, 10, 9
	vaddfp128 4, 4, 1
	lvx128 11, 11, 9
	vmulfp128 0, 8, 9
	vaddfp128 3, 3, 0
	vmulfp128 6, 10, 11
	vaddfp128 2, 2, 6
	bne	0, .Lrv_BB48_2
	vaddfp128 4, 5, 4
	vaddfp128 2, 3, 2
	vaddfp128 2, 4, 2
.Lrv_BB48_4:
	stwu 1, -32(1)
	vsldoi128 3, 2, 2, 4
	vpermwi128 2, 2, 59
	addi 3, 1, 16
	vaddfp128 2, 3, 2
	vsldoi128 3, 2, 2, 4
	vaddfp128 2, 2, 3
	stvx128 2, 0, 3
	lfs 1, 16(1)
	addi 1, 1, 32
	blr
.Lrv_func_end48:
	.size	rv_sp_dot_v, .Lrv_func_end48-.Lrv_func_begin48

	.section	.rodata.cst4,"aM",@progbits,4
	.p2align	2, 0x0
.Lrv_CPI49_0:
	.long	0x437f0000
.Lrv_CPI49_1:
	.long	0x3f000000
.Lrv_CPI49_2:
	.long	0x00000000
.Lrv_CPI49_3:
	.long	0x4f7fffff
	.section	.text.rv_sp_f2u8_s,"ax",@progbits
	.globl	rv_sp_f2u8_s
	.p2align	2
	.type	rv_sp_f2u8_s,@function
rv_sp_f2u8_s:
.Lrv_func_begin49:
	cmpwi	5, 1
	bltlr	0
	stwu 1, -32(1)
	lis 6, .Lrv_CPI49_0@ha
	lfs 0, .Lrv_CPI49_0@l(6)
	lis 6, .Lrv_CPI49_1@ha
	lfs 1, .Lrv_CPI49_1@l(6)
	lis 6, .Lrv_CPI49_2@ha
	lfs 2, .Lrv_CPI49_2@l(6)
	lis 6, .Lrv_CPI49_3@ha
	lfs 3, .Lrv_CPI49_3@l(6)
	addi 4, 4, -4
	addi 3, 3, -1
	b .Lrv_BB49_3
.Lrv_BB49_2:
	addi 5, 5, -1
	cmplwi	5, 0
	stbu 7, 1(3)
	beq	0, .Lrv_BB49_11
.Lrv_BB49_3:
	lfsu 4, 4(4)
	fmr 5, 2
	fmuls 4, 4, 0
	fadds 4, 4, 1
	fcmpu 0, 4, 2
	blt	0, .Lrv_BB49_5
	fmr 5, 4
.Lrv_BB49_5:
	fcmpu 0, 5, 0
	fmr 4, 0
	bgt	0, .Lrv_BB49_7
	fmr 4, 5
.Lrv_BB49_7:
	fcmpu 0, 4, 2
	fctidz 5, 4
	cror 20, 0, 3
	li 6, 0
	stfd 5, 24(1)
	bc 4, 20, .Lrv_BB49_9
	fcmpu 0, 4, 3
	li 7, -1
	bgt	0, .Lrv_BB49_2
	b .Lrv_BB49_10
.Lrv_BB49_9:
	lwz 6, 28(1)
	fcmpu 0, 4, 3
	li 7, -1
	bgt	0, .Lrv_BB49_2
.Lrv_BB49_10:
	mr	7, 6
	b .Lrv_BB49_2
.Lrv_BB49_11:
	addi 1, 1, 32
	blr
.Lrv_func_end49:
	.size	rv_sp_f2u8_s, .Lrv_func_end49-.Lrv_func_begin49

	.section	.rodata.cst16,"aM",@progbits,16
	.p2align	4, 0x0
.Lrv_CPI50_0:
	.long	0x437f0000
	.long	0x437f0000
	.long	0x437f0000
	.long	0x437f0000
.Lrv_CPI50_1:
	.long	0x3f000000
	.long	0x3f000000
	.long	0x3f000000
	.long	0x3f000000
.Lrv_CPI50_4:
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
.Lrv_CPI50_5:
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
.Lrv_CPI50_2:
	.long	0x00000000
.Lrv_CPI50_3:
	.long	0x4f7fffff
	.section	.text.rv_sp_f2u8_v,"ax",@progbits
	.globl	rv_sp_f2u8_v
	.p2align	2
	.type	rv_sp_f2u8_v,@function
rv_sp_f2u8_v:
.Lrv_func_begin50:
	rlwinm. 6, 5, 28, 4, 31
	cmpwi 1, 5, 1
	cror 20, 4, 2
	bclr 12, 20, 0
	stwu 1, -208(1)
	std 30, 184(1)
	li 11, .Lrv_CPI50_0@l
	lis 12, .Lrv_CPI50_0@ha
	li 0, .Lrv_CPI50_1@l
	lis 30, .Lrv_CPI50_1@ha
	std 28, 168(1)
	std 29, 176(1)
	lvx128 3, 12, 11
	lis 29, .Lrv_CPI50_2@ha
	lis 28, .Lrv_CPI50_3@ha
	lvx128 4, 30, 0
	li 11, .Lrv_CPI50_4@l
	lis 12, .Lrv_CPI50_4@ha
	li 0, .Lrv_CPI50_5@l
	lis 30, .Lrv_CPI50_5@ha
	lvx128 5, 12, 11
	addi 5, 1, 96
	lfs 0, .Lrv_CPI50_2@l(29)
	addi 7, 1, 112
	lfs 1, .Lrv_CPI50_3@l(28)
	li 10, 0
	lvx128 0, 30, 0
	addi 8, 1, 128
	vspltisw128 2, 0
	addi 9, 1, 48
	addi 11, 1, 64
	addi 12, 1, 80
	std 26, 152(1)
	std 27, 160(1)
	b .Lrv_BB50_3
.Lrv_BB50_2:
	lvx128 1, 0, 12
	slwi 0, 10, 4
	lvx128 6, 0, 5
	addi 10, 10, 1
	lvx128 7, 0, 7
	cmplw	10, 6
	lvx128 8, 0, 8
	vperm128 1, 1, 6, 5
	addi 4, 4, 64
	vperm128 6, 7, 8, 5
	vperm128 1, 1, 6, 0
	stvx128 1, 3, 0
	beq	0, .Lrv_BB50_21
.Lrv_BB50_3:
	li 30, 0
	b .Lrv_BB50_5
.Lrv_BB50_4:
	stw 28, 64(1)
	lvx128 1, 0, 11
	stvx128 1, 12, 30
	addi 30, 30, 16
	cmplwi	30, 64
	beq	0, .Lrv_BB50_2
.Lrv_BB50_5:
	lvx128 1, 4, 30
	li 29, 0
	li 27, 0
	vmulfp128 1, 1, 3
	vaddfp128 1, 1, 4
	vcmpgtfp128 6, 2, 1
	vnor128 6, 6, 6
	vand128 1, 6, 1
	vcmpgtfp128 6, 1, 3
	vsel128 6, 1, 3, 6
	stvx128 6, 0, 9
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
	bc 12, 20, .Lrv_BB50_7
	lwz 27, 44(1)
.Lrv_BB50_7:
	fcmpu 0, 5, 1
	li 28, -1
	li 26, -1
	bgt	0, .Lrv_BB50_9
	mr	26, 27
.Lrv_BB50_9:
	fcmpu 0, 4, 0
	cror 20, 0, 3
	li 27, 0
	stw 26, 76(1)
	bc 12, 20, .Lrv_BB50_11
	lwz 27, 36(1)
.Lrv_BB50_11:
	fcmpu 0, 4, 1
	li 26, -1
	bgt	0, .Lrv_BB50_13
	mr	26, 27
.Lrv_BB50_13:
	fcmpu 0, 3, 0
	cror 20, 0, 3
	li 27, 0
	stw 26, 72(1)
	bc 4, 20, .Lrv_BB50_17
	fcmpu 0, 3, 1
	li 26, -1
	ble 0, .Lrv_BB50_18
.Lrv_BB50_15:
	fcmpu 0, 2, 0
	cror 20, 0, 3
	stw 26, 68(1)
	bc 4, 20, .Lrv_BB50_19
.Lrv_BB50_16:
	fcmpu 0, 2, 1
	bgt	0, .Lrv_BB50_4
	b .Lrv_BB50_20
.Lrv_BB50_17:
	lwz 27, 28(1)
	fcmpu 0, 3, 1
	li 26, -1
	bgt	0, .Lrv_BB50_15
.Lrv_BB50_18:
	mr	26, 27
	fcmpu 0, 2, 0
	cror 20, 0, 3
	stw 26, 68(1)
	bc 12, 20, .Lrv_BB50_16
.Lrv_BB50_19:
	lwz 29, 20(1)
	fcmpu 0, 2, 1
	bgt	0, .Lrv_BB50_4
.Lrv_BB50_20:
	mr	28, 29
	b .Lrv_BB50_4
.Lrv_BB50_21:
	ld 30, 184(1)
	ld 29, 176(1)
	ld 28, 168(1)
	ld 27, 160(1)
	ld 26, 152(1)
	addi 1, 1, 208
	blr
.Lrv_func_end50:
	.size	rv_sp_f2u8_v, .Lrv_func_end50-.Lrv_func_begin50

	.section	.text.rv_sp_mat4_v,"ax",@progbits
	.globl	rv_sp_mat4_v
	.p2align	2
	.type	rv_sp_mat4_v,@function
rv_sp_mat4_v:
.Lrv_func_begin51:
	cmpwi	6, 1
	bltlr	0
	li 7, 16
	lvx128 3, 4, 7
	li 7, 32
	lvx128 4, 4, 7
	li 7, 48
	lvx128 2, 0, 4
	lvx128 5, 4, 7
	li 4, 0
.Lrv_BB51_2:
	lvx128 0, 5, 4
	addi 6, 6, -1
	cmplwi	6, 0
	vspltw128 1, 0, 0
	vspltw128 6, 0, 1
	vspltw128 7, 0, 2
	vspltw128 0, 0, 3
	vmulfp128 1, 2, 1
	vmulfp128 6, 3, 6
	vmulfp128 7, 4, 7
	vmulfp128 0, 5, 0
	vaddfp128 1, 1, 6
	vaddfp128 1, 7, 1
	vaddfp128 0, 0, 1
	stvx128 0, 3, 4
	addi 4, 4, 16
	bne	0, .Lrv_BB51_2
	blr
.Lrv_func_end51:
	.size	rv_sp_mat4_v, .Lrv_func_end51-.Lrv_func_begin51

	.section	.text.rv_sp_mat4_v16,"ax",@progbits
	.globl	rv_sp_mat4_v16
	.p2align	2
	.type	rv_sp_mat4_v16,@function
rv_sp_mat4_v16:
.Lrv_func_begin52:
	srawi 7, 6, 31
	andc 6, 6, 7
	rlwinm. 6, 6, 28, 4, 31
	beqlr	0
	stwu 1, -112(1)
	li 7, 16
	li 8, 32
	li 9, 48
	lvx128 2, 0, 4
	std 23, 32(1)
	std 24, 40(1)
	std 25, 48(1)
	std 26, 56(1)
	std 27, 64(1)
	std 28, 72(1)
	std 29, 80(1)
	std 30, 88(1)
	li 10, 0
	lvx128 3, 4, 7
	li 11, 64
	lvx128 4, 4, 8
	li 12, 80
	lvx128 5, 4, 9
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
	std 22, 24(1)
.Lrv_BB52_2:
	lvx128 0, 5, 10
	add 22, 5, 10
	lvx128 1, 22, 7
	addi 6, 6, -1
	lvx128 6, 22, 8
	vspltw128 18, 0, 1
	vspltw128 19, 0, 0
	vspltw128 30, 0, 2
	vspltw128 0, 0, 3
	cmplwi	6, 0
	lvx128 7, 22, 9
	vspltw128 29, 1, 1
	vspltw128 28, 1, 0
	vmulfp128 19, 2, 19
	vmulfp128 18, 3, 18
	vspltw128 27, 1, 2
	vmulfp128 30, 4, 30
	vspltw128 1, 1, 3
	vmulfp128 0, 5, 0
	lvx128 8, 22, 11
	vspltw128 26, 6, 1
	vspltw128 25, 6, 0
	vaddfp128 18, 19, 18
	vmulfp128 19, 2, 28
	vmulfp128 29, 3, 29
	vspltw128 24, 6, 2
	vmulfp128 27, 4, 27
	vspltw128 6, 6, 3
	vmulfp128 1, 5, 1
	lvx128 9, 22, 12
	vspltw128 23, 7, 1
	vspltw128 22, 7, 0
	vmulfp128 25, 2, 25
	vmulfp128 26, 3, 26
	vaddfp128 18, 18, 30
	vaddfp128 19, 19, 29
	vspltw128 30, 7, 2
	vmulfp128 24, 4, 24
	vmulfp128 6, 5, 6
	lvx128 10, 22, 0
	vspltw128 29, 8, 1
	vspltw128 21, 8, 0
	vmulfp128 22, 2, 22
	vmulfp128 23, 3, 23
	vaddfp128 26, 25, 26
	vaddfp128 0, 18, 0
	vaddfp128 18, 19, 27
	vmulfp128 30, 4, 30
	lvx128 11, 22, 30
	vspltw128 19, 9, 1
	vspltw128 27, 9, 0
	vmulfp128 25, 2, 21
	vmulfp128 29, 3, 29
	vaddfp128 23, 22, 23
	vaddfp128 26, 26, 24
	vaddfp128 1, 18, 1
	lvx128 12, 22, 29
	vspltw128 18, 10, 1
	vspltw128 24, 10, 0
	vmulfp128 27, 2, 27
	vmulfp128 19, 3, 19
	vaddfp128 29, 25, 29
	vaddfp128 30, 23, 30
	vaddfp128 6, 26, 6
	lvx128 13, 22, 28
	vspltw128 26, 11, 1
	vspltw128 25, 11, 0
	vmulfp128 24, 2, 24
	vmulfp128 18, 3, 18
	vaddfp128 19, 27, 19
	lvx128 14, 22, 27
	vmulfp128 27, 2, 25
	vmulfp128 26, 3, 26
	vaddfp128 18, 24, 18
	lvx128 15, 22, 26
	vaddfp128 27, 27, 26
	lvx128 16, 22, 25
	lvx128 17, 22, 24
	lvx128 31, 22, 23
	lvx128 28, 22, 4
	add 22, 3, 10
	stvx128 0, 3, 10
	vspltw128 0, 7, 3
	vspltw128 7, 8, 2
	addi 10, 10, 256
	stvx128 1, 22, 7
	vspltw128 1, 8, 3
	vspltw128 8, 9, 2
	vmulfp128 0, 5, 0
	vmulfp128 7, 4, 7
	stvx128 6, 22, 8
	vspltw128 6, 9, 3
	vspltw128 9, 10, 2
	vmulfp128 1, 5, 1
	vmulfp128 8, 4, 8
	vaddfp128 7, 29, 7
	vaddfp128 0, 30, 0
	vspltw128 30, 12, 1
	vspltw128 29, 12, 0
	stvx128 0, 22, 9
	vspltw128 0, 10, 3
	vspltw128 10, 11, 2
	vmulfp128 6, 5, 6
	vmulfp128 9, 4, 9
	vaddfp128 8, 19, 8
	vaddfp128 1, 7, 1
	vspltw128 7, 12, 2
	vspltw128 19, 13, 0
	vmulfp128 29, 2, 29
	vmulfp128 30, 3, 30
	stvx128 1, 22, 11
	vspltw128 1, 11, 3
	vspltw128 11, 13, 1
	vmulfp128 0, 5, 0
	vmulfp128 10, 4, 10
	vaddfp128 9, 18, 9
	vaddfp128 6, 8, 6
	vspltw128 8, 13, 2
	vspltw128 18, 14, 0
	vmulfp128 7, 4, 7
	vmulfp128 19, 2, 19
	vaddfp128 30, 29, 30
	vspltw128 29, 15, 0
	stvx128 6, 22, 12
	vspltw128 6, 12, 3
	vspltw128 12, 14, 1
	vmulfp128 1, 5, 1
	vmulfp128 11, 3, 11
	vaddfp128 10, 27, 10
	vaddfp128 0, 9, 0
	vspltw128 9, 14, 2
	vmulfp128 8, 4, 8
	vmulfp128 18, 2, 18
	vaddfp128 7, 30, 7
	vmulfp128 30, 2, 29
	stvx128 0, 22, 0
	vspltw128 0, 13, 3
	vmulfp128 6, 5, 6
	vmulfp128 12, 3, 12
	vaddfp128 11, 19, 11
	vaddfp128 1, 10, 1
	vspltw128 13, 15, 1
	vspltw128 19, 16, 0
	vmulfp128 9, 4, 9
	vspltw128 10, 15, 2
	stvx128 1, 22, 30
	vspltw128 1, 14, 3
	vspltw128 14, 16, 1
	vmulfp128 0, 5, 0
	vaddfp128 12, 18, 12
	vaddfp128 8, 11, 8
	vaddfp128 6, 7, 6
	vmulfp128 13, 3, 13
	vspltw128 7, 16, 2
	vspltw128 11, 17, 1
	vmulfp128 18, 2, 19
	vmulfp128 10, 4, 10
	stvx128 6, 22, 29
	vspltw128 6, 15, 3
	vspltw128 15, 17, 0
	vmulfp128 1, 5, 1
	vmulfp128 14, 3, 14
	vaddfp128 9, 12, 9
	vaddfp128 0, 8, 0
	vaddfp128 13, 30, 13
	vspltw128 8, 17, 2
	vmulfp128 7, 4, 7
	vmulfp128 11, 3, 11
	vspltw128 12, 31, 1
	stvx128 0, 22, 28
	vspltw128 0, 16, 3
	vmulfp128 15, 2, 15
	vaddfp128 14, 18, 14
	vaddfp128 1, 9, 1
	vspltw128 16, 31, 0
	vmulfp128 6, 5, 6
	vaddfp128 10, 13, 10
	vspltw128 13, 28, 1
	vmulfp128 8, 4, 8
	vspltw128 9, 31, 2
	vmulfp128 12, 3, 12
	stvx128 1, 22, 27
	vspltw128 1, 17, 3
	vspltw128 17, 28, 0
	vmulfp128 0, 5, 0
	vaddfp128 11, 15, 11
	vaddfp128 7, 14, 7
	vmulfp128 16, 2, 16
	vaddfp128 6, 10, 6
	vspltw128 10, 28, 2
	vmulfp128 13, 3, 13
	vmulfp128 9, 4, 9
	vmulfp128 1, 5, 1
	vmulfp128 14, 2, 17
	vaddfp128 8, 11, 8
	vaddfp128 0, 7, 0
	stvx128 6, 22, 26
	vspltw128 6, 31, 3
	vaddfp128 12, 16, 12
	vmulfp128 7, 4, 10
	vaddfp128 10, 14, 13
	vaddfp128 1, 8, 1
	stvx128 0, 22, 25
	vspltw128 0, 28, 3
	vmulfp128 6, 5, 6
	vaddfp128 9, 12, 9
	stvx128 1, 22, 24
	vmulfp128 0, 5, 0
	vaddfp128 1, 10, 7
	vaddfp128 6, 9, 6
	vaddfp128 0, 1, 0
	stvx128 6, 22, 23
	stvx128 0, 22, 4
	bne	0, .Lrv_BB52_2
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
.Lrv_func_end52:
	.size	rv_sp_mat4_v16, .Lrv_func_end52-.Lrv_func_begin52

	.section	.text.rv_sp_mat4_v8,"ax",@progbits
	.globl	rv_sp_mat4_v8
	.p2align	2
	.type	rv_sp_mat4_v8,@function
rv_sp_mat4_v8:
.Lrv_func_begin53:
	srawi 7, 6, 31
	andc 6, 6, 7
	rlwinm. 6, 6, 29, 3, 31
	beqlr	0
	stwu 1, -48(1)
	li 7, 16
	li 8, 32
	li 9, 48
	lvx128 2, 0, 4
	li 10, 0
	lvx128 3, 4, 7
	li 11, 64
	lvx128 4, 4, 8
	li 12, 80
	lvx128 5, 4, 9
	li 0, 96
	li 4, 112
	std 30, 24(1)
.Lrv_BB53_2:
	lvx128 0, 5, 10
	add 30, 5, 10
	lvx128 1, 30, 7
	addi 6, 6, -1
	lvx128 6, 30, 8
	vspltw128 10, 0, 1
	vspltw128 11, 0, 0
	vspltw128 13, 0, 2
	vspltw128 0, 0, 3
	cmplwi	6, 0
	lvx128 7, 30, 9
	vspltw128 14, 1, 1
	vspltw128 15, 1, 0
	vmulfp128 11, 2, 11
	vmulfp128 10, 3, 10
	vspltw128 16, 1, 2
	vmulfp128 13, 4, 13
	vspltw128 1, 1, 3
	vmulfp128 0, 5, 0
	lvx128 8, 30, 11
	vspltw128 17, 6, 1
	vspltw128 18, 6, 0
	vaddfp128 10, 11, 10
	vmulfp128 11, 2, 15
	vmulfp128 14, 3, 14
	vspltw128 19, 6, 2
	vmulfp128 16, 4, 16
	vspltw128 6, 6, 3
	vmulfp128 1, 5, 1
	lvx128 9, 30, 12
	vspltw128 31, 7, 1
	vspltw128 30, 7, 0
	vmulfp128 18, 2, 18
	vmulfp128 17, 3, 17
	vaddfp128 10, 10, 13
	vaddfp128 11, 11, 14
	vspltw128 13, 7, 2
	vmulfp128 19, 4, 19
	vmulfp128 6, 5, 6
	lvx128 12, 30, 0
	vspltw128 14, 8, 1
	vspltw128 29, 8, 0
	vmulfp128 30, 2, 30
	vmulfp128 31, 3, 31
	vaddfp128 17, 18, 17
	vaddfp128 0, 10, 0
	vaddfp128 10, 11, 16
	vmulfp128 13, 4, 13
	lvx128 15, 30, 4
	add 30, 3, 10
	stvx128 0, 3, 10
	vspltw128 0, 7, 3
	vspltw128 7, 8, 2
	vspltw128 11, 9, 1
	vspltw128 16, 9, 0
	vmulfp128 18, 2, 29
	vmulfp128 14, 3, 14
	vaddfp128 31, 30, 31
	vaddfp128 17, 17, 19
	vaddfp128 1, 10, 1
	vspltw128 10, 12, 1
	vspltw128 19, 12, 0
	addi 10, 10, 128
	stvx128 1, 30, 7
	vspltw128 1, 8, 3
	vspltw128 8, 9, 2
	vmulfp128 0, 5, 0
	vmulfp128 7, 4, 7
	vmulfp128 16, 2, 16
	vmulfp128 11, 3, 11
	vaddfp128 14, 18, 14
	vaddfp128 13, 31, 13
	vaddfp128 6, 17, 6
	vspltw128 17, 15, 1
	vspltw128 18, 15, 0
	vmulfp128 19, 2, 19
	vmulfp128 10, 3, 10
	stvx128 6, 30, 8
	vspltw128 6, 9, 3
	vspltw128 9, 12, 2
	vmulfp128 1, 5, 1
	vmulfp128 8, 4, 8
	vaddfp128 11, 16, 11
	vaddfp128 7, 14, 7
	vaddfp128 0, 13, 0
	vmulfp128 13, 2, 18
	vmulfp128 14, 3, 17
	vaddfp128 10, 19, 10
	stvx128 0, 30, 9
	vspltw128 0, 12, 3
	vspltw128 12, 15, 2
	vmulfp128 6, 5, 6
	vmulfp128 9, 4, 9
	vaddfp128 8, 11, 8
	vaddfp128 1, 7, 1
	vaddfp128 11, 13, 14
	stvx128 1, 30, 11
	vspltw128 1, 15, 3
	vmulfp128 0, 5, 0
	vmulfp128 7, 4, 12
	vaddfp128 9, 10, 9
	vaddfp128 6, 8, 6
	stvx128 6, 30, 12
	vmulfp128 1, 5, 1
	vaddfp128 6, 11, 7
	vaddfp128 0, 9, 0
	stvx128 0, 30, 0
	vaddfp128 0, 6, 1
	stvx128 0, 30, 4
	bne	0, .Lrv_BB53_2
	ld 30, 24(1)
	addi 1, 1, 48
	blr
.Lrv_func_end53:
	.size	rv_sp_mat4_v8, .Lrv_func_end53-.Lrv_func_begin53

	.section	.text.rv_sp_mat4_x,"ax",@progbits
	.globl	rv_sp_mat4_x
	.p2align	2
	.type	rv_sp_mat4_x,@function
rv_sp_mat4_x:
.Lrv_func_begin54:
	cmpwi	6, 1
	bltlr	0
	li 7, 16
	lvx128 3, 4, 7
	li 7, 32
	lvx128 4, 4, 7
	li 7, 48
	lvx128 2, 0, 4
	lvx128 5, 4, 7
	li 4, 0
	vmrghw128 0, 2, 4
	vmrglw128 6, 2, 4
	vmrghw128 1, 3, 5
	vmrglw128 5, 3, 5
	vmrghw128 2, 0, 1
	vmrglw128 3, 0, 1
	vmrghw128 4, 6, 5
	vmrglw128 5, 6, 5
.Lrv_BB54_2:
	lvx128 0, 5, 4
	addi 6, 6, -1
	cmplwi	6, 0
	vmsum4fp128 1, 2, 0
	vmsum4fp128 6, 3, 0
	vrlimi128 1, 6, 4, 0
	vmsum4fp128 6, 4, 0
	vmsum4fp128 0, 5, 0
	vrlimi128 1, 6, 2, 0
	vrlimi128 1, 0, 1, 0
	stvx128 1, 3, 4
	addi 4, 4, 16
	bne	0, .Lrv_BB54_2
	blr
.Lrv_func_end54:
	.size	rv_sp_mat4_x, .Lrv_func_end54-.Lrv_func_begin54

	.section	.text.rv_sp_saxpy_v,"ax",@progbits
	.globl	rv_sp_saxpy_v
	.p2align	2
	.type	rv_sp_saxpy_v,@function
rv_sp_saxpy_v:
.Lrv_func_begin55:
	stwu 1, -32(1)
	srawi 5, 6, 31
	andc 5, 6, 5
	rlwinm. 5, 5, 30, 2, 31
	stfs 1, 16(1)
	beq	0, .Lrv_BB55_3
	addi 6, 1, 16
	lvx128 2, 0, 6
	li 6, 0
	vspltw128 2, 2, 0
.Lrv_BB55_2:
	lvx128 3, 4, 6
	addi 5, 5, -1
	lvx128 4, 3, 6
	cmplwi	5, 0
	vmulfp128 3, 2, 3
	vaddfp128 3, 3, 4
	stvx128 3, 3, 6
	addi 6, 6, 16
	bne	0, .Lrv_BB55_2
.Lrv_BB55_3:
	addi 1, 1, 32
	blr
.Lrv_func_end55:
	.size	rv_sp_saxpy_v, .Lrv_func_end55-.Lrv_func_begin55

	.section	.rodata.cst16,"aM",@progbits,16
	.p2align	4, 0x0
.Lrv_CPI56_0:
	.long	0x3b808081
	.long	0x3b808081
	.long	0x3b808081
	.long	0x3b808081
	.section	.text.rv_sp_u8f_v,"ax",@progbits
	.globl	rv_sp_u8f_v
	.p2align	2
	.type	rv_sp_u8f_v,@function
rv_sp_u8f_v:
.Lrv_func_begin56:
	rlwinm. 6, 5, 28, 4, 31
	cmpwi 1, 5, 1
	cror 20, 4, 2
	bclr 12, 20, 0
	stwu 1, -352(1)
	std 30, 328(1)
	li 0, .Lrv_CPI56_0@l
	lis 30, .Lrv_CPI56_0@ha
	lvx128 2, 30, 0
	addi 5, 3, 48
	addi 7, 3, 32
	addi 8, 3, 16
	li 9, 0
	addi 10, 1, 16
	addi 11, 1, 208
	addi 12, 1, 192
	addi 0, 1, 176
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
.Lrv_BB56_2:
	lvx128 3, 0, 4
	addi 6, 6, -1
	addi 4, 4, 16
	cmplwi	6, 0
	stvx128 3, 0, 10
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
	lvx128 3, 0, 11
	lbz 20, 22(1)
	vmulfp128 3, 3, 2
	lbz 25, 24(1)
	lbz 24, 25(1)
	lbz 23, 26(1)
	lbz 18, 27(1)
	lbz 29, 28(1)
	lbz 28, 29(1)
	lbz 27, 30(1)
	lbz 26, 31(1)
	stvx128 3, 3, 9
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
	lvx128 3, 0, 12
	vmulfp128 3, 3, 2
	stvx128 3, 8, 9
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
	lvx128 3, 0, 0
	vmulfp128 3, 3, 2
	stvx128 3, 7, 9
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
	lvx128 3, 0, 30
	vmulfp128 3, 3, 2
	stvx128 3, 5, 9
	addi 9, 9, 64
	bne	0, .Lrv_BB56_2
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
.Lrv_func_end56:
	.size	rv_sp_u8f_v, .Lrv_func_end56-.Lrv_func_begin56

	.type	rv_g_a,@object
	.section	.bss.rv_g_a,"aw",@nobits
	.globl	rv_g_a
	.p2align	4, 0x0
rv_g_a:
	.space	4096
	.size	rv_g_a, 4096

	.type	rv_g_b,@object
	.section	.bss.rv_g_b,"aw",@nobits
	.globl	rv_g_b
	.p2align	4, 0x0
rv_g_b:
	.space	4096
	.size	rv_g_b, 4096

	.type	rv_g_c,@object
	.section	.bss.rv_g_c,"aw",@nobits
	.globl	rv_g_c
	.p2align	4, 0x0
rv_g_c:
	.space	4096
	.size	rv_g_c, 4096

	.type	rv_g_ia,@object
	.section	.bss.rv_g_ia,"aw",@nobits
	.globl	rv_g_ia
	.p2align	4, 0x0
rv_g_ia:
	.space	4096
	.size	rv_g_ia, 4096

	.type	rv_g_ib,@object
	.section	.bss.rv_g_ib,"aw",@nobits
	.globl	rv_g_ib
	.p2align	4, 0x0
rv_g_ib:
	.space	4096
	.size	rv_g_ib, 4096

	.type	rv_g_ic,@object
	.section	.bss.rv_g_ic,"aw",@nobits
	.globl	rv_g_ic
	.p2align	4, 0x0
rv_g_ic:
	.space	4096
	.size	rv_g_ic, 4096

	.type	.Lrv__MergedGlobals,@object
	.section	.data.rel.ro..Lrv__MergedGlobals,"aw",@progbits
	.p2align	2, 0x0
.Lrv__MergedGlobals:
	.long	.Lrv__MergedGlobals+16
	.ascii	"\000\000\000\033\000\000\000p\000\000\000\021"
	.asciz	"examples/vmx128bench/lib.rs"
	.size	.Lrv__MergedGlobals, 44

	.globl	rv_sp_mat4_s
	.type	rv_sp_mat4_s,@function
rv_sp_mat4_s = rv_k_mat4_vec4_r
	.globl	rv_sp_u8f_s
	.type	rv_sp_u8f_s,@function
rv_sp_u8f_s = rv_k_u8_to_f32_r
	.globl	rv_sp_saxpy_s
	.type	rv_sp_saxpy_s,@function
rv_sp_saxpy_s = rv_k_saxpy_r
	.globl	rv_sp_copy_s
	.type	rv_sp_copy_s,@function
rv_sp_copy_s = rv_k_copy_u32_r
.Lrv_anon.9f5168636304abd2b30fcfe364c41a8c.1 = .Lrv__MergedGlobals
	.size	.Lrv_anon.9f5168636304abd2b30fcfe364c41a8c.1, 16
.Lrv_anon.9f5168636304abd2b30fcfe364c41a8c.0 = .Lrv__MergedGlobals+16
	.size	.Lrv_anon.9f5168636304abd2b30fcfe364c41a8c.0, 28
