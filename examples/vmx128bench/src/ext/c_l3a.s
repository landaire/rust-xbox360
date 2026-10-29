# kernels.c: clang version 22.1.8 (git@github.com:landaire/llvm-project.git 5f59e9fff8d5dc5b1fa65127b3579d4b1a71f950)
# flags: --target=powerpc64-unknown-xbox360 -ffreestanding -fno-builtin -fms-extensions -ffunction-sections -fdata-sections -fno-ident -fno-addrsig -fno-asynchronous-unwind-tables -g0 -O3 -mno-vmx128
	.section	.text.l3a_k_saxpy,"ax",@progbits
	.globl	l3a_k_saxpy                     # -- Begin function l3a_k_saxpy
	.p2align	2
	.type	l3a_k_saxpy,@function
l3a_k_saxpy:                            # @l3a_k_saxpy
.Ll3a_func_begin0:
# %bb.0:                                # %entry
	cmpwi	6, 1
	bltlr	0
# %bb.1:                                # %for.body.preheader
	stwu 1, -48(1)
	cmplwi	6, 16
	li 5, 0
	blt	0, .Ll3a_BB0_7
# %bb.2:                                # %vector.memcheck
	slwi 7, 6, 2
	add 8, 4, 7
	cmplw	3, 8
	add 7, 3, 7
	bc 4, 0, .Ll3a_BB0_4
# %bb.3:                                # %vector.memcheck
	cmplw	4, 7
	bc 12, 0, .Ll3a_BB0_7
.Ll3a_BB0_4:                                # %vector.ph
	stfs 1, 32(1)
	addi 5, 1, 32
	li 7, 0
	li 8, 15
	addi 9, 1, 16
	lvx 2, 0, 5
	rlwinm 5, 6, 0, 1, 29
	mr	10, 5
	vspltw 2, 2, 0
.Ll3a_BB0_5:                                # %vector.body
                                        # =>This Inner Loop Header: Depth=1
	add 11, 4, 7
	lvx 3, 4, 7
	addi 10, 10, -4
	cmplwi	10, 0
	lvx 4, 11, 8
	add 11, 3, 7
	lvsl 5, 4, 7
	vperm 3, 3, 4, 5
	lvx 4, 3, 7
	lvx 5, 11, 8
	lvsl 0, 3, 7
	vperm 4, 4, 5, 0
	vmaddfp 3, 2, 3, 4
	stvx 3, 0, 9
	ld 12, 16(1)
	stdx 12, 3, 7
	addi 7, 7, 16
	ld 12, 24(1)
	std 12, 8(11)
	bne	0, .Ll3a_BB0_5
# %bb.6:                                # %middle.block
	cmplw	6, 5
	beq	0, .Ll3a_BB0_9
.Ll3a_BB0_7:                                # %for.body.preheader12
	slwi 7, 5, 2
	sub	5, 6, 5
	addi 6, 7, -4
	add 4, 4, 6
	add 3, 3, 6
.Ll3a_BB0_8:                                # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lfsu 0, 4(3)
	addi 5, 5, -1
	cmplwi	5, 0
	lfsu 2, 4(4)
	fmadds 0, 1, 2, 0
	stfs 0, 0(3)
	bne	0, .Ll3a_BB0_8
.Ll3a_BB0_9:
	addi 1, 1, 48
	blr
.Ll3a_func_end0:
	.size	l3a_k_saxpy, .Ll3a_func_end0-.Ll3a_func_begin0
                                        # -- End function
	.section	.text.l3a_k_saxpy_r,"ax",@progbits
	.globl	l3a_k_saxpy_r                   # -- Begin function l3a_k_saxpy_r
	.p2align	2
	.type	l3a_k_saxpy_r,@function
l3a_k_saxpy_r:                          # @l3a_k_saxpy_r
.Ll3a_func_begin1:
# %bb.0:                                # %entry
	cmpwi	6, 1
	bltlr	0
# %bb.1:                                # %for.body.preheader
	stwu 1, -48(1)
	cmplwi	6, 4
	li 5, 0
	blt	0, .Ll3a_BB1_5
# %bb.2:                                # %vector.ph
	stfs 1, 32(1)
	addi 5, 1, 32
	li 7, 0
	li 8, 15
	addi 9, 1, 16
	lvx 2, 0, 5
	rlwinm 5, 6, 0, 1, 29
	mr	10, 5
	vspltw 2, 2, 0
.Ll3a_BB1_3:                                # %vector.body
                                        # =>This Inner Loop Header: Depth=1
	add 11, 4, 7
	lvx 3, 4, 7
	addi 10, 10, -4
	cmplwi	10, 0
	lvx 4, 11, 8
	add 11, 3, 7
	lvsl 5, 4, 7
	vperm 3, 3, 4, 5
	lvx 4, 3, 7
	lvx 5, 11, 8
	lvsl 0, 3, 7
	vperm 4, 4, 5, 0
	vmaddfp 3, 2, 3, 4
	stvx 3, 0, 9
	ld 12, 16(1)
	stdx 12, 3, 7
	addi 7, 7, 16
	ld 12, 24(1)
	std 12, 8(11)
	bne	0, .Ll3a_BB1_3
# %bb.4:                                # %middle.block
	cmplw	6, 5
	beq	0, .Ll3a_BB1_7
.Ll3a_BB1_5:                                # %for.body.preheader11
	slwi 7, 5, 2
	sub	5, 6, 5
	addi 6, 7, -4
	add 4, 4, 6
	add 3, 3, 6
.Ll3a_BB1_6:                                # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lfsu 0, 4(3)
	addi 5, 5, -1
	cmplwi	5, 0
	lfsu 2, 4(4)
	fmadds 0, 1, 2, 0
	stfs 0, 0(3)
	bne	0, .Ll3a_BB1_6
.Ll3a_BB1_7:
	addi 1, 1, 48
	blr
.Ll3a_func_end1:
	.size	l3a_k_saxpy_r, .Ll3a_func_end1-.Ll3a_func_begin1
                                        # -- End function
	.section	.text.l3a_k_scale_inplace,"ax",@progbits
	.globl	l3a_k_scale_inplace             # -- Begin function l3a_k_scale_inplace
	.p2align	2
	.type	l3a_k_scale_inplace,@function
l3a_k_scale_inplace:                    # @l3a_k_scale_inplace
.Ll3a_func_begin2:
# %bb.0:                                # %entry
	cmpwi	5, 1
	bltlr	0
# %bb.1:                                # %for.body.preheader
	stwu 1, -48(1)
	cmplwi	5, 4
	li 4, 0
	blt	0, .Ll3a_BB2_5
# %bb.2:                                # %vector.ph
	stfs 1, 32(1)
	addi 4, 1, 32
	vspltisw 3, -1
	li 6, 15
	addi 7, 1, 16
	mr	8, 3
	lvx 2, 0, 4
	rlwinm 4, 5, 0, 1, 29
	mr	9, 4
	vspltw 2, 2, 0
	vslw 3, 3, 3
.Ll3a_BB2_3:                                # %vector.body
                                        # =>This Inner Loop Header: Depth=1
	lvx 4, 8, 6
	addi 9, 9, -4
	cmplwi	9, 0
	lvx 5, 0, 8
	lvsl 0, 0, 8
	vperm 4, 5, 4, 0
	vmaddfp 4, 2, 4, 3
	stvx 4, 0, 7
	ld 10, 24(1)
	std 10, 8(8)
	ld 10, 16(1)
	std 10, 0(8)
	addi 8, 8, 16
	bne	0, .Ll3a_BB2_3
# %bb.4:                                # %middle.block
	cmplw	5, 4
	beq	0, .Ll3a_BB2_7
.Ll3a_BB2_5:                                # %for.body.preheader5
	slwi 6, 4, 2
	add 6, 6, 3
	sub	3, 5, 4
	addi 4, 6, -4
.Ll3a_BB2_6:                                # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lfsu 0, 4(4)
	addi 3, 3, -1
	cmplwi	3, 0
	fmuls 0, 1, 0
	stfs 0, 0(4)
	bne	0, .Ll3a_BB2_6
.Ll3a_BB2_7:
	addi 1, 1, 48
	blr
.Ll3a_func_end2:
	.size	l3a_k_scale_inplace, .Ll3a_func_end2-.Ll3a_func_begin2
                                        # -- End function
	.section	.text.l3a_k_add_f32_r,"ax",@progbits
	.globl	l3a_k_add_f32_r                 # -- Begin function l3a_k_add_f32_r
	.p2align	2
	.type	l3a_k_add_f32_r,@function
l3a_k_add_f32_r:                        # @l3a_k_add_f32_r
.Ll3a_func_begin3:
# %bb.0:                                # %entry
	cmpwi	6, 1
	bltlr	0
# %bb.1:                                # %for.body.preheader
	stwu 1, -64(1)
	cmplwi	6, 4
	li 7, 0
	std 30, 40(1)                           # 8-byte Folded Spill
	blt	0, .Ll3a_BB3_5
# %bb.2:                                # %vector.ph
	rlwinm 7, 6, 0, 1, 29
	li 8, 0
	li 9, 15
	addi 10, 1, 16
	mr	11, 7
.Ll3a_BB3_3:                                # %vector.body
                                        # =>This Inner Loop Header: Depth=1
	add 12, 4, 8
	lvx 2, 4, 8
	mr	30, 3
	addi 11, 11, -4
	lvx 3, 12, 9
	add 12, 5, 8
	cmplwi	11, 0
	lvsl 4, 4, 8
	vperm 2, 2, 3, 4
	lvx 3, 5, 8
	lvx 4, 12, 9
	lvsl 5, 5, 8
	vperm 3, 3, 4, 5
	vaddfp 2, 2, 3
	stvx 2, 0, 10
	ld 12, 16(1)
	stdux 12, 30, 8
	addi 8, 8, 16
	ld 12, 24(1)
	std 12, 8(30)
	bne	0, .Ll3a_BB3_3
# %bb.4:                                # %middle.block
	cmplw	6, 7
	beq	0, .Ll3a_BB3_7
.Ll3a_BB3_5:                                # %for.body.preheader10
	slwi 8, 7, 2
	sub	6, 6, 7
	addi 8, 8, -4
	add 4, 4, 8
	add 5, 5, 8
	add 3, 3, 8
.Ll3a_BB3_6:                                # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lfsu 0, 4(4)
	addi 6, 6, -1
	cmplwi	6, 0
	lfsu 1, 4(5)
	fadds 0, 0, 1
	stfsu 0, 4(3)
	bne	0, .Ll3a_BB3_6
.Ll3a_BB3_7:
	ld 30, 40(1)                            # 8-byte Folded Reload
	addi 1, 1, 64
	blr
.Ll3a_func_end3:
	.size	l3a_k_add_f32_r, .Ll3a_func_end3-.Ll3a_func_begin3
                                        # -- End function
	.section	.text.l3a_k_mul_f32_r,"ax",@progbits
	.globl	l3a_k_mul_f32_r                 # -- Begin function l3a_k_mul_f32_r
	.p2align	2
	.type	l3a_k_mul_f32_r,@function
l3a_k_mul_f32_r:                        # @l3a_k_mul_f32_r
.Ll3a_func_begin4:
# %bb.0:                                # %entry
	cmpwi	6, 1
	bltlr	0
# %bb.1:                                # %for.body.preheader
	stwu 1, -64(1)
	cmplwi	6, 4
	li 7, 0
	std 30, 40(1)                           # 8-byte Folded Spill
	blt	0, .Ll3a_BB4_5
# %bb.2:                                # %vector.ph
	vspltisw 2, -1
	rlwinm 7, 6, 0, 1, 29
	li 8, 0
	li 9, 15
	addi 10, 1, 16
	mr	11, 7
	vslw 2, 2, 2
.Ll3a_BB4_3:                                # %vector.body
                                        # =>This Inner Loop Header: Depth=1
	add 12, 4, 8
	lvx 3, 4, 8
	mr	30, 3
	addi 11, 11, -4
	lvx 4, 12, 9
	add 12, 5, 8
	cmplwi	11, 0
	lvsl 5, 4, 8
	vperm 3, 3, 4, 5
	lvx 4, 5, 8
	lvx 5, 12, 9
	lvsl 0, 5, 8
	vperm 4, 4, 5, 0
	vmaddfp 3, 3, 4, 2
	stvx 3, 0, 10
	ld 12, 16(1)
	stdux 12, 30, 8
	addi 8, 8, 16
	ld 12, 24(1)
	std 12, 8(30)
	bne	0, .Ll3a_BB4_3
# %bb.4:                                # %middle.block
	cmplw	6, 7
	beq	0, .Ll3a_BB4_7
.Ll3a_BB4_5:                                # %for.body.preheader10
	slwi 8, 7, 2
	sub	6, 6, 7
	addi 8, 8, -4
	add 4, 4, 8
	add 5, 5, 8
	add 3, 3, 8
.Ll3a_BB4_6:                                # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lfsu 0, 4(4)
	addi 6, 6, -1
	cmplwi	6, 0
	lfsu 1, 4(5)
	fmuls 0, 0, 1
	stfsu 0, 4(3)
	bne	0, .Ll3a_BB4_6
.Ll3a_BB4_7:
	ld 30, 40(1)                            # 8-byte Folded Reload
	addi 1, 1, 64
	blr
.Ll3a_func_end4:
	.size	l3a_k_mul_f32_r, .Ll3a_func_end4-.Ll3a_func_begin4
                                        # -- End function
	.section	.rodata.cst4,"aM",@progbits,4
	.p2align	2, 0x0                          # -- Begin function l3a_k_sum_f32
.Ll3a_CPI5_0:
	.long	0x00000000                      # float 0
	.section	.text.l3a_k_sum_f32,"ax",@progbits
	.globl	l3a_k_sum_f32
	.p2align	2
	.type	l3a_k_sum_f32,@function
l3a_k_sum_f32:                          # @l3a_k_sum_f32
.Ll3a_func_begin5:
# %bb.0:                                # %entry
	cmpwi	4, 1
	lis 5, .Ll3a_CPI5_0@ha
	blt	0, .Ll3a_BB5_4
# %bb.1:                                # %for.body.preheader
	lfs 1, .Ll3a_CPI5_0@l(5)
	addi 3, 3, -4
.Ll3a_BB5_2:                                # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lfsu 0, 4(3)
	addi 4, 4, -1
	cmplwi	4, 0
	fadds 1, 1, 0
	bne	0, .Ll3a_BB5_2
# %bb.3:                                # %for.end
	blr
.Ll3a_BB5_4:
	lfs 1, .Ll3a_CPI5_0@l(5)
	blr
.Ll3a_func_end5:
	.size	l3a_k_sum_f32, .Ll3a_func_end5-.Ll3a_func_begin5
                                        # -- End function
	.section	.rodata.cst4,"aM",@progbits,4
	.p2align	2, 0x0                          # -- Begin function l3a_k_dot_f32
.Ll3a_CPI6_0:
	.long	0x00000000                      # float 0
	.section	.text.l3a_k_dot_f32,"ax",@progbits
	.globl	l3a_k_dot_f32
	.p2align	2
	.type	l3a_k_dot_f32,@function
l3a_k_dot_f32:                          # @l3a_k_dot_f32
.Ll3a_func_begin6:
# %bb.0:                                # %entry
	cmpwi	5, 1
	lis 6, .Ll3a_CPI6_0@ha
	blt	0, .Ll3a_BB6_4
# %bb.1:                                # %for.body.preheader
	lfs 1, .Ll3a_CPI6_0@l(6)
	addi 3, 3, -4
	addi 4, 4, -4
.Ll3a_BB6_2:                                # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lfsu 0, 4(4)
	addi 5, 5, -1
	cmplwi	5, 0
	lfsu 2, 4(3)
	fmadds 1, 2, 0, 1
	bne	0, .Ll3a_BB6_2
# %bb.3:                                # %for.end
	blr
.Ll3a_BB6_4:
	lfs 1, .Ll3a_CPI6_0@l(6)
	blr
.Ll3a_func_end6:
	.size	l3a_k_dot_f32, .Ll3a_func_end6-.Ll3a_func_begin6
                                        # -- End function
	.section	.text.l3a_k_sum_i32,"ax",@progbits
	.globl	l3a_k_sum_i32                   # -- Begin function l3a_k_sum_i32
	.p2align	2
	.type	l3a_k_sum_i32,@function
l3a_k_sum_i32:                          # @l3a_k_sum_i32
.Ll3a_func_begin7:
# %bb.0:                                # %entry
	cmpwi	4, 1
	blt	0, .Ll3a_BB7_8
# %bb.1:                                # %for.body.preheader
	stwu 1, -32(1)
	cmplwi	4, 8
	li 5, 0
	li 6, 0
	blt	0, .Ll3a_BB7_5
# %bb.2:                                # %vector.ph
	rlwinm 6, 4, 0, 1, 28
	vxor 2, 2, 2
	addi 5, 3, 16
	li 7, 0
	li 8, 15
	mr	9, 6
	vxor 3, 3, 3
.Ll3a_BB7_3:                                # %vector.body
                                        # =>This Inner Loop Header: Depth=1
	add 10, 3, 7
	lvx 4, 3, 7
	addi 9, 9, -8
	cmplwi	9, 0
	lvx 5, 10, 8
	add 10, 5, 7
	lvsl 0, 3, 7
	vperm 4, 4, 5, 0
	vadduwm 2, 4, 2
	lvx 5, 5, 7
	lvx 0, 10, 8
	lvsl 1, 5, 7
	addi 7, 7, 32
	vperm 4, 5, 0, 1
	vadduwm 3, 4, 3
	bne	0, .Ll3a_BB7_3
# %bb.4:                                # %middle.block
	vadduwm 2, 3, 2
	addi 5, 1, 16
	cmplw	4, 6
	vsldoi 3, 2, 2, 8
	vadduwm 2, 2, 3
	vspltw 3, 2, 1
	vadduwm 2, 2, 3
	stvx 2, 0, 5
	lwz 5, 16(1)
	beq	0, .Ll3a_BB7_7
.Ll3a_BB7_5:                                # %for.body.preheader9
	slwi 7, 6, 2
	add 7, 7, 3
	sub	3, 4, 6
	addi 4, 7, -4
.Ll3a_BB7_6:                                # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lwzu 6, 4(4)
	addi 3, 3, -1
	cmplwi	3, 0
	add 5, 6, 5
	bne	0, .Ll3a_BB7_6
.Ll3a_BB7_7:
	addi 1, 1, 32
	mr	3, 5
	blr
.Ll3a_BB7_8:
	li 3, 0
	blr
.Ll3a_func_end7:
	.size	l3a_k_sum_i32, .Ll3a_func_end7-.Ll3a_func_begin7
                                        # -- End function
	.section	.text.l3a_k_dot_i16,"ax",@progbits
	.globl	l3a_k_dot_i16                   # -- Begin function l3a_k_dot_i16
	.p2align	2
	.type	l3a_k_dot_i16,@function
l3a_k_dot_i16:                          # @l3a_k_dot_i16
.Ll3a_func_begin8:
# %bb.0:                                # %entry
	cmpwi	5, 1
	blt	0, .Ll3a_BB8_8
# %bb.1:                                # %for.body.preheader
	stwu 1, -48(1)
	li 6, 0
	li 7, 0
	std 29, 16(1)                           # 8-byte Folded Spill
	std 30, 24(1)                           # 8-byte Folded Spill
	beq	0, .Ll3a_BB8_5
# %bb.2:                                # %vector.ph
	rlwinm 7, 5, 0, 1, 30
	addi 8, 3, -4
	addi 9, 4, -4
	mr	11, 7
	li 10, 0
.Ll3a_BB8_3:                                # %vector.body
                                        # =>This Inner Loop Header: Depth=1
	lhau 12, 4(9)
	addi 11, 11, -2
	cmplwi	11, 0
	lhau 0, 4(8)
	mullw 12, 12, 0
	add 6, 12, 6
	lha 30, 2(8)
	lha 29, 2(9)
	mullw 0, 29, 30
	add 10, 0, 10
	bne	0, .Ll3a_BB8_3
# %bb.4:                                # %middle.block
	cmplw	5, 7
	add 6, 10, 6
	beq	0, .Ll3a_BB8_7
.Ll3a_BB8_5:                                # %for.body.preheader11
	slwi 8, 7, 1
	sub	5, 5, 7
	addi 7, 8, -2
	add 3, 3, 7
	add 4, 4, 7
.Ll3a_BB8_6:                                # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lhau 7, 2(4)
	addi 5, 5, -1
	cmplwi	5, 0
	lhau 8, 2(3)
	mullw 7, 7, 8
	add 6, 7, 6
	bne	0, .Ll3a_BB8_6
.Ll3a_BB8_7:
	ld 30, 24(1)                            # 8-byte Folded Reload
	ld 29, 16(1)                            # 8-byte Folded Reload
	addi 1, 1, 48
	mr	3, 6
	blr
.Ll3a_BB8_8:
	li 3, 0
	blr
.Ll3a_func_end8:
	.size	l3a_k_dot_i16, .Ll3a_func_end8-.Ll3a_func_begin8
                                        # -- End function
	.section	.text.l3a_k_max_f32,"ax",@progbits
	.globl	l3a_k_max_f32                   # -- Begin function l3a_k_max_f32
	.p2align	2
	.type	l3a_k_max_f32,@function
l3a_k_max_f32:                          # @l3a_k_max_f32
.Ll3a_func_begin9:
# %bb.0:                                # %entry
	lfs 0, 0(3)
	cmpwi	4, 2
	blt	0, .Ll3a_BB9_5
# %bb.1:                                # %for.body.preheader
	addi 4, 4, -1
	b .Ll3a_BB9_3
.Ll3a_BB9_2:                                # %for.body
                                        #   in Loop: Header=BB9_3 Depth=1
	addi 4, 4, -1
	fmr 0, 1
	cmplwi	4, 0
	beqlr	0
.Ll3a_BB9_3:                                # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lfsu 1, 4(3)
	fcmpu 0, 1, 0
	bgt	0, .Ll3a_BB9_2
# %bb.4:                                # %for.body
                                        #   in Loop: Header=BB9_3 Depth=1
	fmr 1, 0
	b .Ll3a_BB9_2
.Ll3a_BB9_5:
	fmr 1, 0
	blr
.Ll3a_func_end9:
	.size	l3a_k_max_f32, .Ll3a_func_end9-.Ll3a_func_begin9
                                        # -- End function
	.section	.text.l3a_k_add_i32,"ax",@progbits
	.globl	l3a_k_add_i32                   # -- Begin function l3a_k_add_i32
	.p2align	2
	.type	l3a_k_add_i32,@function
l3a_k_add_i32:                          # @l3a_k_add_i32
.Ll3a_func_begin10:
# %bb.0:                                # %entry
	cmpwi	6, 1
	bltlr	0
# %bb.1:                                # %for.body.preheader
	stwu 1, -64(1)
	cmplwi	6, 20
	li 7, 0
	std 30, 40(1)                           # 8-byte Folded Spill
	bge 0, .Ll3a_BB10_5
.Ll3a_BB10_2:                               # %for.body.preheader14
	slwi 8, 7, 2
	sub	6, 6, 7
	addi 8, 8, -4
	add 4, 4, 8
	add 5, 5, 8
	add 3, 3, 8
.Ll3a_BB10_3:                               # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lwzu 7, 4(4)
	addi 6, 6, -1
	cmplwi	6, 0
	lwzu 8, 4(5)
	add 7, 8, 7
	stwu 7, 4(3)
	bne	0, .Ll3a_BB10_3
.Ll3a_BB10_4:
	ld 30, 40(1)                            # 8-byte Folded Reload
	addi 1, 1, 64
	blr
.Ll3a_BB10_5:                               # %vector.memcheck
	sub	8, 3, 4
	cmplwi	8, 16
	sub	8, 3, 5
	bc 12, 0, .Ll3a_BB10_2
# %bb.6:                                # %vector.memcheck
	cmplwi	8, 16
	bc 12, 0, .Ll3a_BB10_2
# %bb.7:                                # %vector.ph
	rlwinm 7, 6, 0, 1, 29
	li 8, 0
	li 9, 15
	addi 10, 1, 16
	mr	11, 7
.Ll3a_BB10_8:                               # %vector.body
                                        # =>This Inner Loop Header: Depth=1
	add 12, 4, 8
	lvx 2, 4, 8
	mr	30, 3
	addi 11, 11, -4
	lvx 3, 12, 9
	add 12, 5, 8
	cmplwi	11, 0
	lvsl 4, 4, 8
	vperm 2, 2, 3, 4
	lvx 3, 5, 8
	lvx 4, 12, 9
	lvsl 5, 5, 8
	vperm 3, 3, 4, 5
	vadduwm 2, 3, 2
	stvx 2, 0, 10
	ld 12, 16(1)
	stdux 12, 30, 8
	addi 8, 8, 16
	ld 12, 24(1)
	std 12, 8(30)
	bne	0, .Ll3a_BB10_8
# %bb.9:                                # %middle.block
	cmplw	6, 7
	bne	0, .Ll3a_BB10_2
	b .Ll3a_BB10_4
.Ll3a_func_end10:
	.size	l3a_k_add_i32, .Ll3a_func_end10-.Ll3a_func_begin10
                                        # -- End function
	.section	.text.l3a_k_add_i32_r,"ax",@progbits
	.globl	l3a_k_add_i32_r                 # -- Begin function l3a_k_add_i32_r
	.p2align	2
	.type	l3a_k_add_i32_r,@function
l3a_k_add_i32_r:                        # @l3a_k_add_i32_r
.Ll3a_func_begin11:
# %bb.0:                                # %entry
	cmpwi	6, 1
	bltlr	0
# %bb.1:                                # %for.body.preheader
	stwu 1, -64(1)
	cmplwi	6, 4
	li 7, 0
	std 30, 40(1)                           # 8-byte Folded Spill
	blt	0, .Ll3a_BB11_5
# %bb.2:                                # %vector.ph
	rlwinm 7, 6, 0, 1, 29
	li 8, 0
	li 9, 15
	addi 10, 1, 16
	mr	11, 7
.Ll3a_BB11_3:                               # %vector.body
                                        # =>This Inner Loop Header: Depth=1
	add 12, 4, 8
	lvx 2, 4, 8
	mr	30, 3
	addi 11, 11, -4
	lvx 3, 12, 9
	add 12, 5, 8
	cmplwi	11, 0
	lvsl 4, 4, 8
	vperm 2, 2, 3, 4
	lvx 3, 5, 8
	lvx 4, 12, 9
	lvsl 5, 5, 8
	vperm 3, 3, 4, 5
	vadduwm 2, 3, 2
	stvx 2, 0, 10
	ld 12, 16(1)
	stdux 12, 30, 8
	addi 8, 8, 16
	ld 12, 24(1)
	std 12, 8(30)
	bne	0, .Ll3a_BB11_3
# %bb.4:                                # %middle.block
	cmplw	6, 7
	beq	0, .Ll3a_BB11_7
.Ll3a_BB11_5:                               # %for.body.preheader10
	slwi 8, 7, 2
	sub	6, 6, 7
	addi 8, 8, -4
	add 4, 4, 8
	add 5, 5, 8
	add 3, 3, 8
.Ll3a_BB11_6:                               # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lwzu 7, 4(4)
	addi 6, 6, -1
	cmplwi	6, 0
	lwzu 8, 4(5)
	add 7, 8, 7
	stwu 7, 4(3)
	bne	0, .Ll3a_BB11_6
.Ll3a_BB11_7:
	ld 30, 40(1)                            # 8-byte Folded Reload
	addi 1, 1, 64
	blr
.Ll3a_func_end11:
	.size	l3a_k_add_i32_r, .Ll3a_func_end11-.Ll3a_func_begin11
                                        # -- End function
	.section	.text.l3a_k_add_i16_r,"ax",@progbits
	.globl	l3a_k_add_i16_r                 # -- Begin function l3a_k_add_i16_r
	.p2align	2
	.type	l3a_k_add_i16_r,@function
l3a_k_add_i16_r:                        # @l3a_k_add_i16_r
.Ll3a_func_begin12:
# %bb.0:                                # %entry
	cmpwi	6, 1
	bltlr	0
# %bb.1:                                # %for.body.preheader
	stwu 1, -64(1)
	cmplwi	6, 8
	li 7, 0
	std 30, 40(1)                           # 8-byte Folded Spill
	blt	0, .Ll3a_BB12_5
# %bb.2:                                # %vector.ph
	rlwinm 7, 6, 0, 1, 28
	li 8, 0
	li 9, 15
	addi 10, 1, 16
	mr	11, 7
.Ll3a_BB12_3:                               # %vector.body
                                        # =>This Inner Loop Header: Depth=1
	add 12, 4, 8
	lvx 2, 4, 8
	mr	30, 3
	addi 11, 11, -8
	lvx 3, 12, 9
	add 12, 5, 8
	cmplwi	11, 0
	lvsl 4, 4, 8
	vperm 2, 2, 3, 4
	lvx 3, 5, 8
	lvx 4, 12, 9
	lvsl 5, 5, 8
	vperm 3, 3, 4, 5
	vadduhm 2, 3, 2
	stvx 2, 0, 10
	ld 12, 16(1)
	stdux 12, 30, 8
	addi 8, 8, 16
	ld 12, 24(1)
	std 12, 8(30)
	bne	0, .Ll3a_BB12_3
# %bb.4:                                # %middle.block
	cmplw	6, 7
	beq	0, .Ll3a_BB12_7
.Ll3a_BB12_5:                               # %for.body.preheader12
	slwi 8, 7, 1
	sub	6, 6, 7
	addi 8, 8, -2
	add 4, 4, 8
	add 5, 5, 8
	add 3, 3, 8
.Ll3a_BB12_6:                               # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lhzu 7, 2(4)
	addi 6, 6, -1
	cmplwi	6, 0
	lhzu 8, 2(5)
	add 7, 8, 7
	sthu 7, 2(3)
	bne	0, .Ll3a_BB12_6
.Ll3a_BB12_7:
	ld 30, 40(1)                            # 8-byte Folded Reload
	addi 1, 1, 64
	blr
.Ll3a_func_end12:
	.size	l3a_k_add_i16_r, .Ll3a_func_end12-.Ll3a_func_begin12
                                        # -- End function
	.section	.text.l3a_k_add_u8_r,"ax",@progbits
	.globl	l3a_k_add_u8_r                  # -- Begin function l3a_k_add_u8_r
	.p2align	2
	.type	l3a_k_add_u8_r,@function
l3a_k_add_u8_r:                         # @l3a_k_add_u8_r
.Ll3a_func_begin13:
# %bb.0:                                # %entry
	cmpwi	6, 1
	bltlr	0
# %bb.1:                                # %for.body.preheader
	stwu 1, -32(1)
	cmplwi	6, 16
	li 7, 0
	blt	0, .Ll3a_BB13_5
# %bb.2:                                # %vector.ph
	rlwinm 7, 6, 0, 1, 27
	li 8, 0
	li 9, 15
	addi 10, 1, 16
.Ll3a_BB13_3:                               # %vector.body
                                        # =>This Inner Loop Header: Depth=1
	add 11, 4, 8
	lvx 2, 4, 8
	mr	12, 3
	lvx 3, 11, 9
	add 11, 5, 8
	lvsl 4, 4, 8
	vperm 2, 2, 3, 4
	lvx 3, 5, 8
	lvx 4, 11, 9
	lvsl 5, 5, 8
	vperm 3, 3, 4, 5
	vaddubm 2, 3, 2
	stvx 2, 0, 10
	ld 11, 16(1)
	stdux 11, 12, 8
	addi 8, 8, 16
	cmplw	7, 8
	ld 11, 24(1)
	std 11, 8(12)
	bne	0, .Ll3a_BB13_3
# %bb.4:                                # %middle.block
	cmplw	6, 7
	beq	0, .Ll3a_BB13_7
.Ll3a_BB13_5:                               # %for.body.preheader12
	addi 8, 7, -1
	add 4, 4, 8
	add 5, 5, 8
	add 3, 3, 8
.Ll3a_BB13_6:                               # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lbzu 8, 1(4)
	addi 7, 7, 1
	cmplw	6, 7
	lbzu 9, 1(5)
	add 8, 9, 8
	stbu 8, 1(3)
	bne	0, .Ll3a_BB13_6
.Ll3a_BB13_7:
	addi 1, 1, 32
	blr
.Ll3a_func_end13:
	.size	l3a_k_add_u8_r, .Ll3a_func_end13-.Ll3a_func_begin13
                                        # -- End function
	.section	.text.l3a_k_avg_u8_r,"ax",@progbits
	.globl	l3a_k_avg_u8_r                  # -- Begin function l3a_k_avg_u8_r
	.p2align	2
	.type	l3a_k_avg_u8_r,@function
l3a_k_avg_u8_r:                         # @l3a_k_avg_u8_r
.Ll3a_func_begin14:
# %bb.0:                                # %entry
	cmpwi	6, 1
	bltlr	0
# %bb.1:                                # %for.body.preheader
	stwu 1, -32(1)
	cmplwi	6, 16
	li 7, 0
	blt	0, .Ll3a_BB14_5
# %bb.2:                                # %vector.ph
	rlwinm 7, 6, 0, 1, 27
	li 8, 0
	vspltisb 2, 1
	li 9, 15
	addi 10, 1, 16
.Ll3a_BB14_3:                               # %vector.body
                                        # =>This Inner Loop Header: Depth=1
	add 11, 4, 8
	lvx 3, 4, 8
	mr	12, 3
	lvx 4, 11, 9
	add 11, 5, 8
	lvsl 5, 4, 8
	vperm 3, 3, 4, 5
	lvx 4, 5, 8
	lvx 5, 11, 9
	lvsl 0, 5, 8
	vperm 4, 4, 5, 0
	vor 5, 3, 4
	vxor 3, 3, 4
	vsrb 3, 3, 2
	vsububm 3, 5, 3
	stvx 3, 0, 10
	ld 11, 16(1)
	stdux 11, 12, 8
	addi 8, 8, 16
	cmplw	7, 8
	ld 11, 24(1)
	std 11, 8(12)
	bne	0, .Ll3a_BB14_3
# %bb.4:                                # %middle.block
	cmplw	6, 7
	beq	0, .Ll3a_BB14_7
.Ll3a_BB14_5:                               # %for.body.preheader13
	addi 8, 7, -1
	add 4, 4, 8
	add 5, 5, 8
	add 3, 3, 8
.Ll3a_BB14_6:                               # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lbzu 8, 1(4)
	addi 7, 7, 1
	cmplw	6, 7
	lbzu 9, 1(5)
	add 8, 8, 9
	addi 8, 8, 1
	srwi 8, 8, 1
	stbu 8, 1(3)
	bne	0, .Ll3a_BB14_6
.Ll3a_BB14_7:
	addi 1, 1, 32
	blr
.Ll3a_func_end14:
	.size	l3a_k_avg_u8_r, .Ll3a_func_end14-.Ll3a_func_begin14
                                        # -- End function
	.section	.rodata.cst16,"aM",@progbits,16
	.p2align	4, 0x0                          # -- Begin function l3a_k_addsat_u8_r
.Ll3a_CPI15_0:
	.byte	0                               # 0x0
	.byte	1                               # 0x1
	.byte	2                               # 0x2
	.byte	16                              # 0x10
	.byte	4                               # 0x4
	.byte	5                               # 0x5
	.byte	6                               # 0x6
	.byte	17                              # 0x11
	.byte	8                               # 0x8
	.byte	9                               # 0x9
	.byte	10                              # 0xa
	.byte	18                              # 0x12
	.byte	12                              # 0xc
	.byte	13                              # 0xd
	.byte	14                              # 0xe
	.byte	19                              # 0x13
	.section	.text.l3a_k_addsat_u8_r,"ax",@progbits
	.globl	l3a_k_addsat_u8_r
	.p2align	2
	.type	l3a_k_addsat_u8_r,@function
l3a_k_addsat_u8_r:                      # @l3a_k_addsat_u8_r
.Ll3a_func_begin15:
# %bb.0:                                # %entry
	cmpwi	6, 1
	bltlr	0
# %bb.1:                                # %for.body.preheader
	stwu 1, -32(1)
	cmplwi	6, 16
	li 7, 0
	blt	0, .Ll3a_BB15_6
# %bb.2:                                # %vector.ph
	li 10, .Ll3a_CPI15_0@l
	lis 11, .Ll3a_CPI15_0@ha
	vspltisw 4, -8
	vxor 2, 2, 2
	lvx 3, 11, 10
	rlwinm 7, 6, 0, 1, 27
	li 8, 0
	li 9, 15
	addi 10, 1, 16
	vsrw 4, 4, 4
.Ll3a_BB15_3:                               # %vector.body
                                        # =>This Inner Loop Header: Depth=1
	add 11, 4, 8
	lvx 5, 4, 8
	mr	12, 3
	lvx 0, 11, 9
	add 11, 5, 8
	lvsl 1, 4, 8
	vperm 5, 5, 0, 1
	vspltw 7, 5, 1
	lvx 0, 5, 8
	vspltw 8, 5, 3
	lvx 1, 11, 9
	lvsl 6, 5, 8
	vperm 7, 2, 7, 3
	vperm 0, 0, 1, 6
	vspltw 9, 0, 1
	vspltw 10, 0, 3
	vperm 1, 2, 5, 3
	vspltw 5, 5, 2
	vperm 6, 2, 0, 3
	vspltw 0, 0, 2
	vadduwm 1, 6, 1
	vperm 8, 2, 8, 3
	vperm 5, 2, 5, 3
	vperm 9, 2, 9, 3
	vadduwm 6, 9, 7
	vperm 10, 2, 10, 3
	vadduwm 7, 10, 8
	vperm 0, 2, 0, 3
	vadduwm 5, 0, 5
	vminuw 1, 1, 4
	vminuw 6, 6, 4
	vminuw 7, 7, 4
	vminuw 5, 5, 4
	vpkuwum 5, 5, 7
	vpkuwum 0, 1, 6
	vpkuhum 5, 0, 5
	stvx 5, 0, 10
	ld 11, 16(1)
	stdux 11, 12, 8
	addi 8, 8, 16
	cmplw	7, 8
	ld 11, 24(1)
	std 11, 8(12)
	bne	0, .Ll3a_BB15_3
# %bb.4:                                # %middle.block
	cmplw	6, 7
	bne	0, .Ll3a_BB15_6
.Ll3a_BB15_5:
	addi 1, 1, 32
	blr
.Ll3a_BB15_6:                               # %for.body.preheader15
	addi 8, 7, -1
	add 4, 4, 8
	add 5, 5, 8
	add 3, 3, 8
	b .Ll3a_BB15_8
.Ll3a_BB15_7:                               # %for.body
                                        #   in Loop: Header=BB15_8 Depth=1
	addi 7, 7, 1
	stbu 8, 1(3)
	cmplw	6, 7
	beq	0, .Ll3a_BB15_5
.Ll3a_BB15_8:                               # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lbzu 8, 1(4)
	lbzu 9, 1(5)
	add 8, 9, 8
	cmplwi	8, 255
	blt	0, .Ll3a_BB15_7
# %bb.9:                                # %for.body
                                        #   in Loop: Header=BB15_8 Depth=1
	li 8, 255
	b .Ll3a_BB15_7
.Ll3a_func_end15:
	.size	l3a_k_addsat_u8_r, .Ll3a_func_end15-.Ll3a_func_begin15
                                        # -- End function
	.section	.text.l3a_k_and_u32_r,"ax",@progbits
	.globl	l3a_k_and_u32_r                 # -- Begin function l3a_k_and_u32_r
	.p2align	2
	.type	l3a_k_and_u32_r,@function
l3a_k_and_u32_r:                        # @l3a_k_and_u32_r
.Ll3a_func_begin16:
# %bb.0:                                # %entry
	cmpwi	6, 1
	bltlr	0
# %bb.1:                                # %for.body.preheader
	stwu 1, -64(1)
	cmplwi	6, 4
	li 7, 0
	std 30, 40(1)                           # 8-byte Folded Spill
	blt	0, .Ll3a_BB16_5
# %bb.2:                                # %vector.ph
	rlwinm 7, 6, 0, 1, 29
	li 8, 0
	li 9, 15
	addi 10, 1, 16
	mr	11, 7
.Ll3a_BB16_3:                               # %vector.body
                                        # =>This Inner Loop Header: Depth=1
	add 12, 4, 8
	lvx 2, 4, 8
	mr	30, 3
	addi 11, 11, -4
	lvx 3, 12, 9
	add 12, 5, 8
	cmplwi	11, 0
	lvsl 4, 4, 8
	vperm 2, 2, 3, 4
	lvx 3, 5, 8
	lvx 4, 12, 9
	lvsl 5, 5, 8
	vperm 3, 3, 4, 5
	vand 2, 3, 2
	stvx 2, 0, 10
	ld 12, 16(1)
	stdux 12, 30, 8
	addi 8, 8, 16
	ld 12, 24(1)
	std 12, 8(30)
	bne	0, .Ll3a_BB16_3
# %bb.4:                                # %middle.block
	cmplw	6, 7
	beq	0, .Ll3a_BB16_7
.Ll3a_BB16_5:                               # %for.body.preheader10
	slwi 8, 7, 2
	sub	6, 6, 7
	addi 8, 8, -4
	add 4, 4, 8
	add 5, 5, 8
	add 3, 3, 8
.Ll3a_BB16_6:                               # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lwzu 7, 4(4)
	addi 6, 6, -1
	cmplwi	6, 0
	lwzu 8, 4(5)
	and 7, 8, 7
	stwu 7, 4(3)
	bne	0, .Ll3a_BB16_6
.Ll3a_BB16_7:
	ld 30, 40(1)                            # 8-byte Folded Reload
	addi 1, 1, 64
	blr
.Ll3a_func_end16:
	.size	l3a_k_and_u32_r, .Ll3a_func_end16-.Ll3a_func_begin16
                                        # -- End function
	.section	.text.l3a_k_shl_i32_r,"ax",@progbits
	.globl	l3a_k_shl_i32_r                 # -- Begin function l3a_k_shl_i32_r
	.p2align	2
	.type	l3a_k_shl_i32_r,@function
l3a_k_shl_i32_r:                        # @l3a_k_shl_i32_r
.Ll3a_func_begin17:
# %bb.0:                                # %entry
	cmpwi	5, 1
	bltlr	0
# %bb.1:                                # %for.body.preheader
	li 6, 0
	beq	0, .Ll3a_BB17_5
# %bb.2:                                # %vector.ph
	rlwinm 6, 5, 0, 1, 30
	addi 7, 4, -8
	addi 8, 3, -8
	mr	9, 6
.Ll3a_BB17_3:                               # %vector.body
                                        # =>This Inner Loop Header: Depth=1
	lwzu 10, 8(7)
	addi 9, 9, -2
	cmplwi	9, 0
	lwz 11, 4(7)
	slwi 10, 10, 3
	stwu 10, 8(8)
	slwi 10, 11, 3
	stw 10, 4(8)
	bne	0, .Ll3a_BB17_3
# %bb.4:                                # %middle.block
	cmplw	5, 6
	beqlr	0
.Ll3a_BB17_5:                               # %for.body.preheader7
	slwi 7, 6, 2
	sub	5, 5, 6
	addi 6, 7, -4
	add 4, 4, 6
	add 3, 3, 6
.Ll3a_BB17_6:                               # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lwzu 6, 4(4)
	addi 5, 5, -1
	cmplwi	5, 0
	slwi 6, 6, 3
	stwu 6, 4(3)
	bne	0, .Ll3a_BB17_6
	blr
.Ll3a_func_end17:
	.size	l3a_k_shl_i32_r, .Ll3a_func_end17-.Ll3a_func_begin17
                                        # -- End function
	.section	.text.l3a_k_mul_i32_r,"ax",@progbits
	.globl	l3a_k_mul_i32_r                 # -- Begin function l3a_k_mul_i32_r
	.p2align	2
	.type	l3a_k_mul_i32_r,@function
l3a_k_mul_i32_r:                        # @l3a_k_mul_i32_r
.Ll3a_func_begin18:
# %bb.0:                                # %entry
	cmpwi	6, 1
	bltlr	0
# %bb.1:                                # %for.body.preheader
	stwu 1, -48(1)
	li 7, 0
	std 30, 24(1)                           # 8-byte Folded Spill
	beq	0, .Ll3a_BB18_5
# %bb.2:                                # %vector.ph
	rlwinm 7, 6, 0, 1, 30
	addi 8, 4, -8
	addi 9, 5, -8
	addi 10, 3, -8
	mr	11, 7
.Ll3a_BB18_3:                               # %vector.body
                                        # =>This Inner Loop Header: Depth=1
	lwzu 12, 8(8)
	addi 11, 11, -2
	cmplwi	11, 0
	lwzu 0, 8(9)
	lwz 30, 4(8)
	mullw 12, 0, 12
	lwz 0, 4(9)
	stwu 12, 8(10)
	mullw 12, 0, 30
	stw 12, 4(10)
	bne	0, .Ll3a_BB18_3
# %bb.4:                                # %middle.block
	cmplw	6, 7
	beq	0, .Ll3a_BB18_7
.Ll3a_BB18_5:                               # %for.body.preheader9
	slwi 8, 7, 2
	sub	6, 6, 7
	addi 8, 8, -4
	add 4, 4, 8
	add 5, 5, 8
	add 3, 3, 8
.Ll3a_BB18_6:                               # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lwzu 7, 4(4)
	addi 6, 6, -1
	cmplwi	6, 0
	lwzu 8, 4(5)
	mullw 7, 8, 7
	stwu 7, 4(3)
	bne	0, .Ll3a_BB18_6
.Ll3a_BB18_7:
	ld 30, 24(1)                            # 8-byte Folded Reload
	addi 1, 1, 48
	blr
.Ll3a_func_end18:
	.size	l3a_k_mul_i32_r, .Ll3a_func_end18-.Ll3a_func_begin18
                                        # -- End function
	.section	.text.l3a_k_min_f32_r,"ax",@progbits
	.globl	l3a_k_min_f32_r                 # -- Begin function l3a_k_min_f32_r
	.p2align	2
	.type	l3a_k_min_f32_r,@function
l3a_k_min_f32_r:                        # @l3a_k_min_f32_r
.Ll3a_func_begin19:
# %bb.0:                                # %entry
	cmpwi	6, 1
	bltlr	0
# %bb.1:                                # %for.body.preheader
	stwu 1, -64(1)
	cmplwi	6, 4
	li 7, 0
	std 30, 40(1)                           # 8-byte Folded Spill
	blt	0, .Ll3a_BB19_6
# %bb.2:                                # %vector.ph
	rlwinm 7, 6, 0, 1, 29
	li 8, 0
	li 9, 15
	addi 10, 1, 16
	mr	11, 7
.Ll3a_BB19_3:                               # %vector.body
                                        # =>This Inner Loop Header: Depth=1
	add 12, 4, 8
	lvx 2, 4, 8
	mr	30, 3
	addi 11, 11, -4
	lvx 3, 12, 9
	add 12, 5, 8
	cmplwi	11, 0
	lvsl 4, 4, 8
	vperm 2, 2, 3, 4
	lvx 3, 5, 8
	lvx 4, 12, 9
	lvsl 5, 5, 8
	vperm 3, 3, 4, 5
	vcmpgtfp 4, 3, 2
	vsel 2, 3, 2, 4
	stvx 2, 0, 10
	ld 12, 16(1)
	stdux 12, 30, 8
	addi 8, 8, 16
	ld 12, 24(1)
	std 12, 8(30)
	bne	0, .Ll3a_BB19_3
# %bb.4:                                # %middle.block
	cmplw	6, 7
	bne	0, .Ll3a_BB19_6
.Ll3a_BB19_5:
	ld 30, 40(1)                            # 8-byte Folded Reload
	addi 1, 1, 64
	blr
.Ll3a_BB19_6:                               # %for.body.preheader18
	slwi 8, 7, 2
	sub	6, 6, 7
	addi 8, 8, -4
	add 4, 4, 8
	add 5, 5, 8
	add 3, 3, 8
	b .Ll3a_BB19_8
.Ll3a_BB19_7:                               # %for.body
                                        #   in Loop: Header=BB19_8 Depth=1
	addi 6, 6, -1
	stfsu 0, 4(3)
	cmplwi	6, 0
	beq	0, .Ll3a_BB19_5
.Ll3a_BB19_8:                               # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lfsu 0, 4(4)
	lfsu 1, 4(5)
	fcmpu 0, 0, 1
	blt	0, .Ll3a_BB19_7
# %bb.9:                                # %for.body
                                        #   in Loop: Header=BB19_8 Depth=1
	fmr 0, 1
	b .Ll3a_BB19_7
.Ll3a_func_end19:
	.size	l3a_k_min_f32_r, .Ll3a_func_end19-.Ll3a_func_begin19
                                        # -- End function
	.section	.text.l3a_k_max_i32_r,"ax",@progbits
	.globl	l3a_k_max_i32_r                 # -- Begin function l3a_k_max_i32_r
	.p2align	2
	.type	l3a_k_max_i32_r,@function
l3a_k_max_i32_r:                        # @l3a_k_max_i32_r
.Ll3a_func_begin20:
# %bb.0:                                # %entry
	cmpwi	6, 1
	bltlr	0
# %bb.1:                                # %for.body.preheader
	stwu 1, -64(1)
	cmplwi	6, 4
	li 7, 0
	std 30, 40(1)                           # 8-byte Folded Spill
	blt	0, .Ll3a_BB20_6
# %bb.2:                                # %vector.ph
	rlwinm 7, 6, 0, 1, 29
	li 8, 0
	li 9, 15
	addi 10, 1, 16
	mr	11, 7
.Ll3a_BB20_3:                               # %vector.body
                                        # =>This Inner Loop Header: Depth=1
	add 12, 4, 8
	lvx 2, 4, 8
	mr	30, 3
	addi 11, 11, -4
	lvx 3, 12, 9
	add 12, 5, 8
	cmplwi	11, 0
	lvsl 4, 4, 8
	vperm 2, 2, 3, 4
	lvx 3, 5, 8
	lvx 4, 12, 9
	lvsl 5, 5, 8
	vperm 3, 3, 4, 5
	vmaxsw 2, 2, 3
	stvx 2, 0, 10
	ld 12, 16(1)
	stdux 12, 30, 8
	addi 8, 8, 16
	ld 12, 24(1)
	std 12, 8(30)
	bne	0, .Ll3a_BB20_3
# %bb.4:                                # %middle.block
	cmplw	6, 7
	bne	0, .Ll3a_BB20_6
.Ll3a_BB20_5:
	ld 30, 40(1)                            # 8-byte Folded Reload
	addi 1, 1, 64
	blr
.Ll3a_BB20_6:                               # %for.body.preheader18
	slwi 8, 7, 2
	sub	6, 6, 7
	addi 8, 8, -4
	add 4, 4, 8
	add 5, 5, 8
	add 3, 3, 8
	b .Ll3a_BB20_8
.Ll3a_BB20_7:                               # %for.body
                                        #   in Loop: Header=BB20_8 Depth=1
	addi 6, 6, -1
	stwu 7, 4(3)
	cmplwi	6, 0
	beq	0, .Ll3a_BB20_5
.Ll3a_BB20_8:                               # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lwzu 7, 4(4)
	lwzu 8, 4(5)
	cmpw	7, 8
	bgt	0, .Ll3a_BB20_7
# %bb.9:                                # %for.body
                                        #   in Loop: Header=BB20_8 Depth=1
	mr	7, 8
	b .Ll3a_BB20_7
.Ll3a_func_end20:
	.size	l3a_k_max_i32_r, .Ll3a_func_end20-.Ll3a_func_begin20
                                        # -- End function
	.section	.text.l3a_k_clamp_f32_r,"ax",@progbits
	.globl	l3a_k_clamp_f32_r               # -- Begin function l3a_k_clamp_f32_r
	.p2align	2
	.type	l3a_k_clamp_f32_r,@function
l3a_k_clamp_f32_r:                      # @l3a_k_clamp_f32_r
.Ll3a_func_begin21:
# %bb.0:                                # %entry
	cmpwi	7, 1
	bltlr	0
# %bb.1:                                # %for.body.preheader
	stwu 1, -64(1)
	cmplwi	7, 4
	li 5, 0
	blt	0, .Ll3a_BB21_6
# %bb.2:                                # %vector.ph
	stfs 1, 48(1)
	addi 5, 1, 48
	li 6, 0
	li 8, 15
	addi 9, 1, 16
	lvx 2, 0, 5
	addi 5, 1, 32
	stfs 2, 32(1)
	vspltw 2, 2, 0
	lvx 3, 0, 5
	rlwinm 5, 7, 0, 1, 29
	mr	10, 5
	vspltw 3, 3, 0
.Ll3a_BB21_3:                               # %vector.body
                                        # =>This Inner Loop Header: Depth=1
	add 11, 4, 6
	lvx 4, 4, 6
	mr	12, 3
	addi 10, 10, -4
	lvx 5, 11, 8
	cmplwi	10, 0
	lvsl 0, 4, 6
	vperm 4, 4, 5, 0
	vcmpgtfp 5, 2, 4
	vsel 4, 4, 2, 5
	vcmpgtfp 5, 4, 3
	vsel 4, 4, 3, 5
	stvx 4, 0, 9
	ld 11, 16(1)
	stdux 11, 12, 6
	addi 6, 6, 16
	ld 11, 24(1)
	std 11, 8(12)
	bne	0, .Ll3a_BB21_3
# %bb.4:                                # %middle.block
	cmplw	7, 5
	bne	0, .Ll3a_BB21_6
.Ll3a_BB21_5:
	addi 1, 1, 64
	blr
.Ll3a_BB21_6:                               # %for.body.preheader21
	slwi 6, 5, 2
	sub	5, 7, 5
	addi 6, 6, -4
	add 4, 4, 6
	add 3, 3, 6
	b .Ll3a_BB21_8
.Ll3a_BB21_7:                               # %for.body
                                        #   in Loop: Header=BB21_8 Depth=1
	addi 5, 5, -1
	stfsu 0, 4(3)
	cmplwi	5, 0
	beq	0, .Ll3a_BB21_5
.Ll3a_BB21_8:                               # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lfsu 0, 4(4)
	fmr 3, 1
	fcmpu 0, 0, 1
	bge 0, .Ll3a_BB21_10
# %bb.9:                                # %for.body
                                        #   in Loop: Header=BB21_8 Depth=1
	fcmpu 0, 3, 2
	fmr 0, 2
	bgt	0, .Ll3a_BB21_7
	b .Ll3a_BB21_11
.Ll3a_BB21_10:                              # %for.body
                                        #   in Loop: Header=BB21_8 Depth=1
	fmr 3, 0
	fcmpu 0, 3, 2
	fmr 0, 2
	bgt	0, .Ll3a_BB21_7
.Ll3a_BB21_11:                              # %for.body
                                        #   in Loop: Header=BB21_8 Depth=1
	fmr 0, 3
	b .Ll3a_BB21_7
.Ll3a_func_end21:
	.size	l3a_k_clamp_f32_r, .Ll3a_func_end21-.Ll3a_func_begin21
                                        # -- End function
	.section	.rodata.cst16,"aM",@progbits,16
	.p2align	4, 0x0                          # -- Begin function l3a_k_clamp_i16_r
.Ll3a_CPI22_0:
	.short	64536                           # 0xfc18
	.short	64536                           # 0xfc18
	.short	64536                           # 0xfc18
	.short	64536                           # 0xfc18
	.short	64536                           # 0xfc18
	.short	64536                           # 0xfc18
	.short	64536                           # 0xfc18
	.short	64536                           # 0xfc18
.Ll3a_CPI22_1:
	.short	1000                            # 0x3e8
	.short	1000                            # 0x3e8
	.short	1000                            # 0x3e8
	.short	1000                            # 0x3e8
	.short	1000                            # 0x3e8
	.short	1000                            # 0x3e8
	.short	1000                            # 0x3e8
	.short	1000                            # 0x3e8
	.section	.text.l3a_k_clamp_i16_r,"ax",@progbits
	.globl	l3a_k_clamp_i16_r
	.p2align	2
	.type	l3a_k_clamp_i16_r,@function
l3a_k_clamp_i16_r:                      # @l3a_k_clamp_i16_r
.Ll3a_func_begin22:
# %bb.0:                                # %entry
	cmpwi	5, 1
	bltlr	0
# %bb.1:                                # %for.body.preheader
	stwu 1, -32(1)
	cmplwi	5, 8
	li 6, 0
	blt	0, .Ll3a_BB22_6
# %bb.2:                                # %vector.ph
	li 8, .Ll3a_CPI22_0@l
	lis 9, .Ll3a_CPI22_0@ha
	lvx 2, 9, 8
	li 9, .Ll3a_CPI22_1@l
	lis 10, .Ll3a_CPI22_1@ha
	lvx 3, 10, 9
	rlwinm 6, 5, 0, 1, 28
	li 7, 0
	li 8, 15
	addi 9, 1, 16
	mr	10, 6
.Ll3a_BB22_3:                               # %vector.body
                                        # =>This Inner Loop Header: Depth=1
	add 11, 4, 7
	lvx 4, 4, 7
	mr	12, 3
	addi 10, 10, -8
	lvx 5, 11, 8
	cmplwi	10, 0
	lvsl 0, 4, 7
	vperm 4, 4, 5, 0
	vmaxsh 4, 4, 2
	vminsh 4, 4, 3
	stvx 4, 0, 9
	ld 11, 16(1)
	stdux 11, 12, 7
	addi 7, 7, 16
	ld 11, 24(1)
	std 11, 8(12)
	bne	0, .Ll3a_BB22_3
# %bb.4:                                # %middle.block
	cmplw	5, 6
	bne	0, .Ll3a_BB22_6
.Ll3a_BB22_5:
	addi 1, 1, 32
	blr
.Ll3a_BB22_6:                               # %for.body.preheader20
	slwi 7, 6, 1
	sub	5, 5, 6
	addi 6, 7, -2
	add 4, 4, 6
	add 3, 3, 6
	b .Ll3a_BB22_8
.Ll3a_BB22_7:                               # %for.body
                                        #   in Loop: Header=BB22_8 Depth=1
	addi 5, 5, -1
	sthu 6, 2(3)
	cmplwi	5, 0
	beq	0, .Ll3a_BB22_5
.Ll3a_BB22_8:                               # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lhau 6, 2(4)
	cmpwi	6, -1000
	ble 0, .Ll3a_BB22_10
# %bb.9:                                # %for.body
                                        #   in Loop: Header=BB22_8 Depth=1
	cmpwi	6, 1000
	blt	0, .Ll3a_BB22_7
	b .Ll3a_BB22_11
.Ll3a_BB22_10:                              # %for.body
                                        #   in Loop: Header=BB22_8 Depth=1
	li 6, -1000
	cmpwi	6, 1000
	blt	0, .Ll3a_BB22_7
.Ll3a_BB22_11:                              # %for.body
                                        #   in Loop: Header=BB22_8 Depth=1
	li 6, 1000
	b .Ll3a_BB22_7
.Ll3a_func_end22:
	.size	l3a_k_clamp_i16_r, .Ll3a_func_end22-.Ll3a_func_begin22
                                        # -- End function
	.section	.rodata.cst4,"aM",@progbits,4
	.p2align	2, 0x0                          # -- Begin function l3a_k_abs_f32_r
.Ll3a_CPI23_0:
	.long	0x00000000                      # float 0
	.section	.text.l3a_k_abs_f32_r,"ax",@progbits
	.globl	l3a_k_abs_f32_r
	.p2align	2
	.type	l3a_k_abs_f32_r,@function
l3a_k_abs_f32_r:                        # @l3a_k_abs_f32_r
.Ll3a_func_begin23:
# %bb.0:                                # %entry
	cmpwi	5, 1
	bltlr	0
# %bb.1:                                # %for.body.preheader
	li 6, 0
	beq	0, .Ll3a_BB23_9
# %bb.2:                                # %vector.ph
	lis 9, .Ll3a_CPI23_0@ha
	rlwinm 6, 5, 0, 1, 30
	lfs 0, .Ll3a_CPI23_0@l(9)
	addi 7, 4, -8
	addi 8, 3, -8
	mr	9, 6
	b .Ll3a_BB23_4
.Ll3a_BB23_3:                               # %vector.body
                                        #   in Loop: Header=BB23_4 Depth=1
	addi 9, 9, -2
	stfs 1, 4(8)
	cmplwi	9, 0
	beq	0, .Ll3a_BB23_8
.Ll3a_BB23_4:                               # %vector.body
                                        # =>This Inner Loop Header: Depth=1
	lfsu 2, 8(7)
	fcmpu 0, 2, 0
	blt	0, .Ll3a_BB23_6
# %bb.5:                                # %vector.body
                                        #   in Loop: Header=BB23_4 Depth=1
	lfs 1, 4(7)
	fcmpu 0, 1, 0
	stfsu 2, 8(8)
	bge 0, .Ll3a_BB23_3
	b .Ll3a_BB23_7
.Ll3a_BB23_6:                               #   in Loop: Header=BB23_4 Depth=1
	fneg 2, 2
	lfs 1, 4(7)
	fcmpu 0, 1, 0
	stfsu 2, 8(8)
	bge 0, .Ll3a_BB23_3
.Ll3a_BB23_7:                               #   in Loop: Header=BB23_4 Depth=1
	fneg 1, 1
	b .Ll3a_BB23_3
.Ll3a_BB23_8:                               # %middle.block
	cmplw	5, 6
	beqlr	0
.Ll3a_BB23_9:                               # %for.body.preheader10
	lis 7, .Ll3a_CPI23_0@ha
	sub	5, 5, 6
	lfs 0, .Ll3a_CPI23_0@l(7)
	slwi 6, 6, 2
	addi 6, 6, -4
	add 4, 4, 6
	add 3, 3, 6
	b .Ll3a_BB23_11
.Ll3a_BB23_10:                              # %for.body
                                        #   in Loop: Header=BB23_11 Depth=1
	addi 5, 5, -1
	stfsu 1, 4(3)
	cmplwi	5, 0
	beqlr	0
.Ll3a_BB23_11:                              # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lfsu 1, 4(4)
	fcmpu 0, 1, 0
	bge 0, .Ll3a_BB23_10
# %bb.12:                               #   in Loop: Header=BB23_11 Depth=1
	fneg 1, 1
	b .Ll3a_BB23_10
.Ll3a_func_end23:
	.size	l3a_k_abs_f32_r, .Ll3a_func_end23-.Ll3a_func_begin23
                                        # -- End function
	.section	.text.l3a_k_abs_i32_r,"ax",@progbits
	.globl	l3a_k_abs_i32_r                 # -- Begin function l3a_k_abs_i32_r
	.p2align	2
	.type	l3a_k_abs_i32_r,@function
l3a_k_abs_i32_r:                        # @l3a_k_abs_i32_r
.Ll3a_func_begin24:
# %bb.0:                                # %entry
	cmpwi	5, 1
	bltlr	0
# %bb.1:                                # %for.body.preheader
	stwu 1, -32(1)
	cmplwi	5, 4
	li 6, 0
	blt	0, .Ll3a_BB24_5
# %bb.2:                                # %vector.ph
	rlwinm 6, 5, 0, 1, 29
	li 7, 0
	vxor 2, 2, 2
	li 8, 15
	addi 9, 1, 16
	mr	10, 6
.Ll3a_BB24_3:                               # %vector.body
                                        # =>This Inner Loop Header: Depth=1
	add 11, 4, 7
	lvx 3, 4, 7
	mr	12, 3
	addi 10, 10, -4
	lvx 4, 11, 8
	cmplwi	10, 0
	lvsl 5, 4, 7
	vperm 3, 3, 4, 5
	vsubuwm 4, 2, 3
	vmaxsw 3, 3, 4
	stvx 3, 0, 9
	ld 11, 16(1)
	stdux 11, 12, 7
	addi 7, 7, 16
	ld 11, 24(1)
	std 11, 8(12)
	bne	0, .Ll3a_BB24_3
# %bb.4:                                # %middle.block
	cmplw	5, 6
	beq	0, .Ll3a_BB24_7
.Ll3a_BB24_5:                               # %for.body.preheader10
	slwi 7, 6, 2
	sub	5, 5, 6
	addi 6, 7, -4
	add 4, 4, 6
	add 3, 3, 6
.Ll3a_BB24_6:                               # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lwzu 6, 4(4)
	addi 5, 5, -1
	cmplwi	5, 0
	srawi 7, 6, 31
	xor 6, 6, 7
	sub	6, 6, 7
	stwu 6, 4(3)
	bne	0, .Ll3a_BB24_6
.Ll3a_BB24_7:
	addi 1, 1, 32
	blr
.Ll3a_func_end24:
	.size	l3a_k_abs_i32_r, .Ll3a_func_end24-.Ll3a_func_begin24
                                        # -- End function
	.section	.text.l3a_k_f2i_r,"ax",@progbits
	.globl	l3a_k_f2i_r                     # -- Begin function l3a_k_f2i_r
	.p2align	2
	.type	l3a_k_f2i_r,@function
l3a_k_f2i_r:                            # @l3a_k_f2i_r
.Ll3a_func_begin25:
# %bb.0:                                # %entry
	cmpwi	5, 1
	bltlr	0
# %bb.1:                                # %for.body.preheader
	stwu 1, -32(1)
	li 6, 0
	beq	0, .Ll3a_BB25_5
# %bb.2:                                # %vector.ph
	rlwinm 6, 5, 0, 1, 30
	addi 7, 4, -8
	addi 8, 3, -8
	addi 9, 1, 28
	addi 10, 1, 24
	mr	11, 6
.Ll3a_BB25_3:                               # %vector.body
                                        # =>This Inner Loop Header: Depth=1
	lfsu 0, 8(7)
	addi 11, 11, -2
	cmplwi	11, 0
	fctiwz 0, 0
	stfiwx 0, 0, 9
	lfs 0, 4(7)
	lwz 12, 28(1)
	fctiwz 0, 0
	stwu 12, 8(8)
	stfiwx 0, 0, 10
	lwz 12, 24(1)
	stw 12, 4(8)
	bne	0, .Ll3a_BB25_3
# %bb.4:                                # %middle.block
	cmplw	5, 6
	beq	0, .Ll3a_BB25_7
.Ll3a_BB25_5:                               # %for.body.preheader7
	slwi 7, 6, 2
	sub	5, 5, 6
	addi 7, 7, -4
	addi 6, 1, 20
	add 4, 4, 7
	add 3, 3, 7
.Ll3a_BB25_6:                               # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lfsu 0, 4(4)
	addi 5, 5, -1
	cmplwi	5, 0
	fctiwz 0, 0
	stfiwx 0, 0, 6
	lwz 7, 20(1)
	stwu 7, 4(3)
	bne	0, .Ll3a_BB25_6
.Ll3a_BB25_7:
	addi 1, 1, 32
	blr
.Ll3a_func_end25:
	.size	l3a_k_f2i_r, .Ll3a_func_end25-.Ll3a_func_begin25
                                        # -- End function
	.section	.text.l3a_k_i2f_r,"ax",@progbits
	.globl	l3a_k_i2f_r                     # -- Begin function l3a_k_i2f_r
	.p2align	2
	.type	l3a_k_i2f_r,@function
l3a_k_i2f_r:                            # @l3a_k_i2f_r
.Ll3a_func_begin26:
# %bb.0:                                # %entry
	cmpwi	5, 1
	bltlr	0
# %bb.1:                                # %for.body.preheader
	stwu 1, -48(1)
	li 6, 0
	beq	0, .Ll3a_BB26_5
# %bb.2:                                # %vector.ph
	rlwinm 6, 5, 0, 1, 30
	addi 9, 4, -8
	addi 7, 3, -8
	mr	8, 6
.Ll3a_BB26_3:                               # %vector.body
                                        # =>This Inner Loop Header: Depth=1
	lwa 10, 8(9)
	addi 8, 8, -2
	cmplwi	8, 0
	std 10, 40(1)
	lfd 0, 40(1)
	lwa 10, 12(9)
	fcfid 0, 0
	addi 9, 9, 8
	frsp 0, 0
	stfsu 0, 8(7)
	std 10, 32(1)
	lfd 0, 32(1)
	fcfid 0, 0
	frsp 0, 0
	stfs 0, 4(7)
	bne	0, .Ll3a_BB26_3
# %bb.4:                                # %middle.block
	cmplw	5, 6
	beq	0, .Ll3a_BB26_7
.Ll3a_BB26_5:                               # %for.body.preheader7
	slwi 7, 6, 2
	sub	5, 5, 6
	addi 6, 7, -4
	add 4, 4, 6
	add 3, 3, 6
.Ll3a_BB26_6:                               # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lwa 6, 4(4)
	addi 5, 5, -1
	addi 4, 4, 4
	cmplwi	5, 0
	std 6, 24(1)
	lfd 0, 24(1)
	fcfid 0, 0
	frsp 0, 0
	stfsu 0, 4(3)
	bne	0, .Ll3a_BB26_6
.Ll3a_BB26_7:
	addi 1, 1, 48
	blr
.Ll3a_func_end26:
	.size	l3a_k_i2f_r, .Ll3a_func_end26-.Ll3a_func_begin26
                                        # -- End function
	.section	.rodata.cst4,"aM",@progbits,4
	.p2align	2, 0x0                          # -- Begin function l3a_k_u8_to_f32_r
.Ll3a_CPI27_0:
	.long	0x3b808081                      # float 0.00392156886
	.section	.text.l3a_k_u8_to_f32_r,"ax",@progbits
	.globl	l3a_k_u8_to_f32_r
	.p2align	2
	.type	l3a_k_u8_to_f32_r,@function
l3a_k_u8_to_f32_r:                      # @l3a_k_u8_to_f32_r
.Ll3a_func_begin27:
# %bb.0:                                # %entry
	cmpwi	5, 1
	bltlr	0
# %bb.1:                                # %for.body.preheader
	stwu 1, -48(1)
	li 6, 0
	beq	0, .Ll3a_BB27_5
# %bb.2:                                # %vector.ph
	lis 9, .Ll3a_CPI27_0@ha
	rlwinm 6, 5, 0, 1, 30
	lfs 0, .Ll3a_CPI27_0@l(9)
	addi 7, 4, -2
	addi 8, 3, -8
	mr	9, 6
.Ll3a_BB27_3:                               # %vector.body
                                        # =>This Inner Loop Header: Depth=1
	lbzu 10, 2(7)
	addi 9, 9, -2
	cmplwi	9, 0
	std 10, 40(1)
	lfd 1, 40(1)
	fcfid 1, 1
	lbz 10, 1(7)
	frsp 1, 1
	fmuls 1, 1, 0
	stfsu 1, 8(8)
	std 10, 32(1)
	lfd 1, 32(1)
	fcfid 1, 1
	frsp 1, 1
	fmuls 1, 1, 0
	stfs 1, 4(8)
	bne	0, .Ll3a_BB27_3
# %bb.4:                                # %middle.block
	cmplw	5, 6
	beq	0, .Ll3a_BB27_7
.Ll3a_BB27_5:                               # %for.body.preheader7
	sub	5, 5, 6
	slwi 7, 6, 2
	add 4, 6, 4
	lis 6, .Ll3a_CPI27_0@ha
	lfs 0, .Ll3a_CPI27_0@l(6)
	add 7, 7, 3
	addi 3, 4, -1
	addi 4, 7, -4
.Ll3a_BB27_6:                               # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lbzu 6, 1(3)
	addi 5, 5, -1
	cmplwi	5, 0
	std 6, 24(1)
	lfd 1, 24(1)
	fcfid 1, 1
	frsp 1, 1
	fmuls 1, 1, 0
	stfsu 1, 4(4)
	bne	0, .Ll3a_BB27_6
.Ll3a_BB27_7:
	addi 1, 1, 48
	blr
.Ll3a_func_end27:
	.size	l3a_k_u8_to_f32_r, .Ll3a_func_end27-.Ll3a_func_begin27
                                        # -- End function
	.section	.rodata.cst16,"aM",@progbits,16
	.p2align	4, 0x0                          # -- Begin function l3a_k_mat4_vec4_r
.Ll3a_CPI28_0:
	.byte	0                               # 0x0
	.byte	0                               # 0x0
	.byte	0                               # 0x0
	.byte	0                               # 0x0
	.byte	0                               # 0x0
	.byte	0                               # 0x0
	.byte	0                               # 0x0
	.byte	0                               # 0x0
	.byte	0                               # 0x0
	.byte	1                               # 0x1
	.byte	2                               # 0x2
	.byte	3                               # 0x3
	.byte	16                              # 0x10
	.byte	17                              # 0x11
	.byte	18                              # 0x12
	.byte	19                              # 0x13
.Ll3a_CPI28_1:
	.byte	0                               # 0x0
	.byte	1                               # 0x1
	.byte	2                               # 0x2
	.byte	3                               # 0x3
	.byte	4                               # 0x4
	.byte	5                               # 0x5
	.byte	6                               # 0x6
	.byte	7                               # 0x7
	.byte	24                              # 0x18
	.byte	25                              # 0x19
	.byte	26                              # 0x1a
	.byte	27                              # 0x1b
	.byte	28                              # 0x1c
	.byte	29                              # 0x1d
	.byte	30                              # 0x1e
	.byte	31                              # 0x1f
.Ll3a_CPI28_2:
	.byte	4                               # 0x4
	.byte	5                               # 0x5
	.byte	6                               # 0x6
	.byte	7                               # 0x7
	.byte	20                              # 0x14
	.byte	21                              # 0x15
	.byte	22                              # 0x16
	.byte	23                              # 0x17
	.byte	0                               # 0x0
	.byte	0                               # 0x0
	.byte	0                               # 0x0
	.byte	0                               # 0x0
	.byte	0                               # 0x0
	.byte	0                               # 0x0
	.byte	0                               # 0x0
	.byte	0                               # 0x0
.Ll3a_CPI28_3:
	.byte	0                               # 0x0
	.byte	0                               # 0x0
	.byte	0                               # 0x0
	.byte	0                               # 0x0
	.byte	0                               # 0x0
	.byte	0                               # 0x0
	.byte	0                               # 0x0
	.byte	0                               # 0x0
	.byte	8                               # 0x8
	.byte	9                               # 0x9
	.byte	10                              # 0xa
	.byte	11                              # 0xb
	.byte	24                              # 0x18
	.byte	25                              # 0x19
	.byte	26                              # 0x1a
	.byte	27                              # 0x1b
.Ll3a_CPI28_4:
	.byte	12                              # 0xc
	.byte	13                              # 0xd
	.byte	14                              # 0xe
	.byte	15                              # 0xf
	.byte	28                              # 0x1c
	.byte	29                              # 0x1d
	.byte	30                              # 0x1e
	.byte	31                              # 0x1f
	.byte	0                               # 0x0
	.byte	0                               # 0x0
	.byte	0                               # 0x0
	.byte	0                               # 0x0
	.byte	0                               # 0x0
	.byte	0                               # 0x0
	.byte	0                               # 0x0
	.byte	0                               # 0x0
	.section	.text.l3a_k_mat4_vec4_r,"ax",@progbits
	.globl	l3a_k_mat4_vec4_r
	.p2align	2
	.type	l3a_k_mat4_vec4_r,@function
l3a_k_mat4_vec4_r:                      # @l3a_k_mat4_vec4_r
.Ll3a_func_begin28:
# %bb.0:                                # %entry
	cmpwi	6, 1
	bltlr	0
# %bb.1:                                # %for.body.lr.ph
	stwu 1, -288(1)
	std 26, 232(1)                          # 8-byte Folded Spill
	li 7, 16
	li 8, 32
	li 9, 48
	li 10, 63
	std 27, 240(1)                          # 8-byte Folded Spill
	cmplwi	6, 4
	std 28, 248(1)                          # 8-byte Folded Spill
	std 29, 256(1)                          # 8-byte Folded Spill
	std 30, 264(1)                          # 8-byte Folded Spill
	lvx 2, 0, 4
	lvx 3, 4, 7
	lvx 4, 4, 8
	lvx 0, 4, 9
	lvx 1, 4, 10
	lvsl 5, 0, 4
	li 4, 0
	vperm 2, 2, 3, 5
	vperm 3, 3, 4, 5
	vperm 4, 4, 0, 5
	vperm 5, 0, 1, 5
	blt	0, .Ll3a_BB28_5
# %bb.2:                                # %vector.ph
	li 12, .Ll3a_CPI28_0@l
	lis 30, .Ll3a_CPI28_0@ha
	vspltw 0, 2, 0
	vspltisw 26, -1
	lvx 16, 30, 12
	li 12, .Ll3a_CPI28_1@l
	lis 30, .Ll3a_CPI28_1@ha
	vspltw 7, 5, 0
	li 4, 64
	lvx 18, 30, 12
	li 12, .Ll3a_CPI28_2@l
	lis 30, .Ll3a_CPI28_2@ha
	li 0, .Ll3a_CPI28_3@l
	vspltw 8, 2, 1
	stvx 0, 1, 4                            # 16-byte Folded Spill
	li 4, 48
	lis 29, .Ll3a_CPI28_3@ha
	vslw 26, 26, 26
	vspltw 0, 3, 0
	lvx 31, 30, 12
	li 30, .Ll3a_CPI28_4@l
	lis 28, .Ll3a_CPI28_4@ha
	vspltw 9, 3, 1
	li 11, 0
	addi 12, 1, 160
	stvx 0, 1, 4                            # 16-byte Folded Spill
	li 4, 32
	vspltw 0, 4, 0
	lvx 29, 29, 0
	addi 0, 1, 208
	addi 29, 1, 176
	vspltw 10, 4, 1
	lvx 27, 28, 30
	addi 30, 1, 192
	vspltw 11, 5, 1
	stvx 0, 1, 4                            # 16-byte Folded Spill
	rlwinm 4, 6, 0, 1, 29
	mr	28, 4
	vspltw 12, 2, 2
	vspltw 13, 3, 2
	vspltw 14, 4, 2
	vspltw 15, 5, 2
	vspltw 17, 2, 3
	vspltw 19, 3, 3
	vspltw 30, 4, 3
	vspltw 28, 5, 3
.Ll3a_BB28_3:                               # %vector.body
                                        # =>This Inner Loop Header: Depth=1
	add 27, 5, 11
	lvsl 23, 5, 11
	mr	26, 3
	addi 28, 28, -4
	lvx 25, 27, 8
	cmplwi	28, 0
	lvx 24, 27, 7
	lvx 21, 5, 11
	vperm 22, 24, 25, 23
	vperm 24, 21, 24, 23
	lvx 21, 27, 10
	lvx 20, 27, 9
	li 27, 48
	vperm 21, 20, 21, 23
	vperm 25, 25, 20, 23
	vperm 23, 25, 21, 16
	vmrghw 20, 24, 22
	vperm 0, 24, 22, 31
	vmrghw 1, 25, 21
	vmrglw 6, 24, 22
	vperm 24, 24, 22, 27
	vperm 22, 25, 21, 29
	vmrglw 25, 25, 21
	vperm 0, 0, 1, 18
	vperm 1, 6, 22, 18
	vperm 6, 24, 25, 18
	lvx 25, 1, 27                           # 16-byte Folded Reload
	li 27, 64
	lvx 21, 1, 27                           # 16-byte Folded Reload
	li 27, 32
	vperm 23, 20, 23, 18
	vmaddfp 25, 0, 25, 26
	vmaddfp 24, 0, 9, 26
	vmaddfp 22, 0, 13, 26
	vmaddfp 0, 0, 19, 26
	vmaddfp 25, 21, 23, 25
	vmaddfp 24, 8, 23, 24
	vmaddfp 22, 12, 23, 22
	vmaddfp 0, 17, 23, 0
	lvx 23, 1, 27                           # 16-byte Folded Reload
	vmaddfp 25, 23, 1, 25
	vmaddfp 24, 10, 1, 24
	vmaddfp 23, 14, 1, 22
	vmaddfp 0, 30, 1, 0
	vmaddfp 1, 7, 6, 25
	vmaddfp 25, 11, 6, 24
	vmaddfp 24, 15, 6, 23
	vmaddfp 0, 28, 6, 0
	vperm 6, 1, 25, 31
	vmrghw 23, 24, 0
	vperm 22, 24, 0, 29
	vmrglw 21, 1, 25
	vmrglw 20, 24, 0
	vperm 0, 24, 0, 16
	vperm 24, 1, 25, 27
	vmrghw 1, 1, 25
	vperm 0, 1, 0, 18
	stvx 0, 0, 12
	vperm 25, 21, 22, 18
	vperm 1, 24, 20, 18
	stvx 1, 0, 0
	stvx 25, 0, 30
	vperm 0, 6, 23, 18
	stvx 0, 0, 29
	ld 27, 160(1)
	stdux 27, 26, 11
	addi 11, 11, 64
	ld 27, 216(1)
	std 27, 56(26)
	ld 27, 208(1)
	std 27, 48(26)
	ld 27, 200(1)
	std 27, 40(26)
	ld 27, 192(1)
	std 27, 32(26)
	ld 27, 184(1)
	std 27, 24(26)
	ld 27, 176(1)
	std 27, 16(26)
	ld 27, 168(1)
	std 27, 8(26)
	bne	0, .Ll3a_BB28_3
# %bb.4:                                # %middle.block
	cmplw	6, 4
	beq	0, .Ll3a_BB28_7
.Ll3a_BB28_5:                               # %for.body.preheader
	slwi 9, 4, 4
	vspltisw 0, -1
	sub	6, 6, 4
	add 5, 9, 5
	addi 4, 1, 112
	addi 7, 1, 96
	addi 8, 1, 128
	add 3, 3, 9
	addi 5, 5, -16
	vslw 0, 0, 0
	addi 9, 1, 144
	addi 10, 1, 80
.Ll3a_BB28_6:                               # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lwzu 11, 16(5)
	addi 6, 6, -1
	cmplwi	6, 0
	lwz 12, 4(5)
	lwz 0, 8(5)
	lwz 30, 12(5)
	stw 12, 112(1)
	lvx 1, 0, 4
	stw 11, 96(1)
	vspltw 1, 1, 0
	vmaddfp 1, 1, 3, 0
	lvx 6, 0, 7
	vspltw 6, 6, 0
	stw 0, 128(1)
	vmaddfp 1, 2, 6, 1
	lvx 6, 0, 8
	vspltw 6, 6, 0
	stw 30, 144(1)
	vmaddfp 1, 4, 6, 1
	lvx 6, 0, 9
	vspltw 6, 6, 0
	vmaddfp 1, 5, 6, 1
	stvx 1, 0, 10
	ld 11, 88(1)
	std 11, 8(3)
	ld 11, 80(1)
	std 11, 0(3)
	addi 3, 3, 16
	bne	0, .Ll3a_BB28_6
.Ll3a_BB28_7:
	ld 30, 264(1)                           # 8-byte Folded Reload
	ld 29, 256(1)                           # 8-byte Folded Reload
	ld 28, 248(1)                           # 8-byte Folded Reload
	ld 27, 240(1)                           # 8-byte Folded Reload
	ld 26, 232(1)                           # 8-byte Folded Reload
	addi 1, 1, 288
	blr
.Ll3a_func_end28:
	.size	l3a_k_mat4_vec4_r, .Ll3a_func_end28-.Ll3a_func_begin28
                                        # -- End function
	.section	.text.l3a_k_copy_u32_r,"ax",@progbits
	.globl	l3a_k_copy_u32_r                # -- Begin function l3a_k_copy_u32_r
	.p2align	2
	.type	l3a_k_copy_u32_r,@function
l3a_k_copy_u32_r:                       # @l3a_k_copy_u32_r
.Ll3a_func_begin29:
# %bb.0:                                # %entry
	cmpwi	5, 1
	bltlr	0
# %bb.1:                                # %for.body.preheader
	li 6, 0
	beq	0, .Ll3a_BB29_5
# %bb.2:                                # %vector.ph
	rlwinm 6, 5, 0, 1, 30
	addi 7, 4, -8
	addi 8, 3, -8
	mr	9, 6
.Ll3a_BB29_3:                               # %vector.body
                                        # =>This Inner Loop Header: Depth=1
	lwzu 10, 8(7)
	addi 9, 9, -2
	cmplwi	9, 0
	stwu 10, 8(8)
	lwz 10, 4(7)
	stw 10, 4(8)
	bne	0, .Ll3a_BB29_3
# %bb.4:                                # %middle.block
	cmplw	5, 6
	beqlr	0
.Ll3a_BB29_5:                               # %for.body.preheader7
	slwi 7, 6, 2
	sub	5, 5, 6
	addi 6, 7, -4
	add 4, 4, 6
	add 3, 3, 6
.Ll3a_BB29_6:                               # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lwzu 6, 4(4)
	addi 5, 5, -1
	cmplwi	5, 0
	stwu 6, 4(3)
	bne	0, .Ll3a_BB29_6
	blr
.Ll3a_func_end29:
	.size	l3a_k_copy_u32_r, .Ll3a_func_end29-.Ll3a_func_begin29
                                        # -- End function
	.section	.text.l3a_k_copy_u8,"ax",@progbits
	.globl	l3a_k_copy_u8                   # -- Begin function l3a_k_copy_u8
	.p2align	2
	.type	l3a_k_copy_u8,@function
l3a_k_copy_u8:                          # @l3a_k_copy_u8
.Ll3a_func_begin30:
# %bb.0:                                # %entry
	cmpwi	5, 1
	bltlr	0
# %bb.1:                                # %for.body.preheader
	addi 4, 4, -1
	addi 3, 3, -1
	li 6, 0
.Ll3a_BB30_2:                               # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lbzu 7, 1(4)
	addi 6, 6, 1
	cmplw	5, 6
	stbu 7, 1(3)
	bne	0, .Ll3a_BB30_2
# %bb.3:                                # %for.end
	blr
.Ll3a_func_end30:
	.size	l3a_k_copy_u8, .Ll3a_func_end30-.Ll3a_func_begin30
                                        # -- End function
	.section	.text.l3a_k_set_u32,"ax",@progbits
	.globl	l3a_k_set_u32                   # -- Begin function l3a_k_set_u32
	.p2align	2
	.type	l3a_k_set_u32,@function
l3a_k_set_u32:                          # @l3a_k_set_u32
.Ll3a_func_begin31:
# %bb.0:                                # %entry
	cmpwi	5, 1
	bltlr	0
# %bb.1:                                # %for.body.preheader
	li 6, 0
	beq	0, .Ll3a_BB31_5
# %bb.2:                                # %vector.ph
	rlwinm 6, 5, 0, 1, 30
	addi 7, 3, -8
	mr	8, 6
.Ll3a_BB31_3:                               # %vector.body
                                        # =>This Inner Loop Header: Depth=1
	addi 8, 8, -2
	stwu 4, 8(7)
	cmplwi	8, 0
	stw 4, 4(7)
	bne	0, .Ll3a_BB31_3
# %bb.4:                                # %middle.block
	cmplw	5, 6
	beqlr	0
.Ll3a_BB31_5:                               # %for.body.preheader5
	slwi 7, 6, 2
	add 7, 7, 3
	sub	3, 5, 6
	addi 5, 7, -4
.Ll3a_BB31_6:                               # %for.body
                                        # =>This Inner Loop Header: Depth=1
	addi 3, 3, -1
	stwu 4, 4(5)
	cmplwi	3, 0
	bne	0, .Ll3a_BB31_6
	blr
.Ll3a_func_end31:
	.size	l3a_k_set_u32, .Ll3a_func_end31-.Ll3a_func_begin31
                                        # -- End function
	.section	.text.l3a_k_set_f32,"ax",@progbits
	.globl	l3a_k_set_f32                   # -- Begin function l3a_k_set_f32
	.p2align	2
	.type	l3a_k_set_f32,@function
l3a_k_set_f32:                          # @l3a_k_set_f32
.Ll3a_func_begin32:
# %bb.0:                                # %entry
	cmpwi	5, 1
	bltlr	0
# %bb.1:                                # %for.body.preheader
	li 4, 0
	beq	0, .Ll3a_BB32_5
# %bb.2:                                # %vector.ph
	rlwinm 4, 5, 0, 1, 30
	addi 6, 3, -8
	mr	7, 4
.Ll3a_BB32_3:                               # %vector.body
                                        # =>This Inner Loop Header: Depth=1
	addi 7, 7, -2
	stfsu 1, 8(6)
	cmplwi	7, 0
	stfs 1, 4(6)
	bne	0, .Ll3a_BB32_3
# %bb.4:                                # %middle.block
	cmplw	5, 4
	beqlr	0
.Ll3a_BB32_5:                               # %for.body.preheader5
	slwi 6, 4, 2
	add 6, 6, 3
	sub	3, 5, 4
	addi 4, 6, -4
.Ll3a_BB32_6:                               # %for.body
                                        # =>This Inner Loop Header: Depth=1
	addi 3, 3, -1
	stfsu 1, 4(4)
	cmplwi	3, 0
	bne	0, .Ll3a_BB32_6
	blr
.Ll3a_func_end32:
	.size	l3a_k_set_f32, .Ll3a_func_end32-.Ll3a_func_begin32
                                        # -- End function
	.section	.text.l3a_k_zero_u8,"ax",@progbits
	.globl	l3a_k_zero_u8                   # -- Begin function l3a_k_zero_u8
	.p2align	2
	.type	l3a_k_zero_u8,@function
l3a_k_zero_u8:                          # @l3a_k_zero_u8
.Ll3a_func_begin33:
# %bb.0:                                # %entry
	cmpwi	4, 1
	bltlr	0
# %bb.1:                                # %for.body.preheader
	li 5, 0
	beq	0, .Ll3a_BB33_5
# %bb.2:                                # %vector.ph
	rlwinm 5, 4, 0, 1, 30
	addi 6, 3, -2
	li 7, 0
	mr	8, 5
.Ll3a_BB33_3:                               # %vector.body
                                        # =>This Inner Loop Header: Depth=1
	addi 8, 8, -2
	stbu 7, 2(6)
	cmplwi	8, 0
	stb 7, 1(6)
	bne	0, .Ll3a_BB33_3
# %bb.4:                                # %middle.block
	cmplw	4, 5
	beqlr	0
.Ll3a_BB33_5:                               # %for.body.preheader5
	add 3, 5, 3
	sub	4, 4, 5
	addi 3, 3, -1
	li 5, 0
.Ll3a_BB33_6:                               # %for.body
                                        # =>This Inner Loop Header: Depth=1
	addi 4, 4, -1
	stbu 5, 1(3)
	cmplwi	4, 0
	bne	0, .Ll3a_BB33_6
	blr
.Ll3a_func_end33:
	.size	l3a_k_zero_u8, .Ll3a_func_end33-.Ll3a_func_begin33
                                        # -- End function
	.section	.rodata.cst16,"aM",@progbits,16
	.p2align	4, 0x0                          # -- Begin function l3a_k_stride2_f32_r
.Ll3a_CPI34_0:
	.byte	0                               # 0x0
	.byte	1                               # 0x1
	.byte	2                               # 0x2
	.byte	3                               # 0x3
	.byte	8                               # 0x8
	.byte	9                               # 0x9
	.byte	10                              # 0xa
	.byte	11                              # 0xb
	.byte	16                              # 0x10
	.byte	17                              # 0x11
	.byte	18                              # 0x12
	.byte	19                              # 0x13
	.byte	24                              # 0x18
	.byte	25                              # 0x19
	.byte	26                              # 0x1a
	.byte	27                              # 0x1b
.Ll3a_CPI34_1:
	.byte	4                               # 0x4
	.byte	5                               # 0x5
	.byte	6                               # 0x6
	.byte	7                               # 0x7
	.byte	12                              # 0xc
	.byte	13                              # 0xd
	.byte	14                              # 0xe
	.byte	15                              # 0xf
	.byte	20                              # 0x14
	.byte	21                              # 0x15
	.byte	22                              # 0x16
	.byte	23                              # 0x17
	.byte	28                              # 0x1c
	.byte	29                              # 0x1d
	.byte	30                              # 0x1e
	.byte	31                              # 0x1f
	.section	.text.l3a_k_stride2_f32_r,"ax",@progbits
	.globl	l3a_k_stride2_f32_r
	.p2align	2
	.type	l3a_k_stride2_f32_r,@function
l3a_k_stride2_f32_r:                    # @l3a_k_stride2_f32_r
.Ll3a_func_begin34:
# %bb.0:                                # %entry
	cmpwi	5, 1
	bltlr	0
# %bb.1:                                # %for.body.preheader
	stwu 1, -32(1)
	cmplwi	5, 4
	li 6, 0
	blt	0, .Ll3a_BB34_5
# %bb.2:                                # %vector.ph
	li 8, .Ll3a_CPI34_0@l
	lis 9, .Ll3a_CPI34_0@ha
	lvx 2, 9, 8
	li 9, .Ll3a_CPI34_1@l
	lis 10, .Ll3a_CPI34_1@ha
	lvx 3, 10, 9
	rlwinm 6, 5, 0, 1, 29
	li 7, 31
	li 8, 16
	addi 9, 1, 16
	mr	10, 4
	mr	11, 3
	mr	12, 6
.Ll3a_BB34_3:                               # %vector.body
                                        # =>This Inner Loop Header: Depth=1
	lvx 4, 10, 7
	addi 12, 12, -4
	cmplwi	12, 0
	lvx 5, 10, 8
	lvx 1, 0, 10
	lvsl 0, 0, 10
	addi 10, 10, 32
	vperm 4, 5, 4, 0
	vperm 5, 1, 5, 0
	vperm 0, 5, 4, 2
	vperm 4, 5, 4, 3
	vaddfp 4, 0, 4
	stvx 4, 0, 9
	ld 0, 24(1)
	std 0, 8(11)
	ld 0, 16(1)
	std 0, 0(11)
	addi 11, 11, 16
	bne	0, .Ll3a_BB34_3
# %bb.4:                                # %middle.block
	cmplw	5, 6
	beq	0, .Ll3a_BB34_7
.Ll3a_BB34_5:                               # %for.body.preheader13
	sub	5, 5, 6
	slwi 7, 6, 2
	slwi 6, 6, 3
	add 4, 6, 4
	add 6, 7, 3
	addi 3, 4, -8
	addi 4, 6, -4
.Ll3a_BB34_6:                               # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lfsu 0, 8(3)
	addi 5, 5, -1
	cmplwi	5, 0
	lfs 1, 4(3)
	fadds 0, 0, 1
	stfsu 0, 4(4)
	bne	0, .Ll3a_BB34_6
.Ll3a_BB34_7:
	addi 1, 1, 32
	blr
.Ll3a_func_end34:
	.size	l3a_k_stride2_f32_r, .Ll3a_func_end34-.Ll3a_func_begin34
                                        # -- End function
	.section	.text.l3a_k_gather_f32_r,"ax",@progbits
	.globl	l3a_k_gather_f32_r              # -- Begin function l3a_k_gather_f32_r
	.p2align	2
	.type	l3a_k_gather_f32_r,@function
l3a_k_gather_f32_r:                     # @l3a_k_gather_f32_r
.Ll3a_func_begin35:
# %bb.0:                                # %entry
	cmpwi	6, 1
	bltlr	0
# %bb.1:                                # %for.body.preheader
	li 7, 0
	beq	0, .Ll3a_BB35_5
# %bb.2:                                # %vector.ph
	rlwinm 7, 6, 0, 1, 30
	addi 8, 5, -8
	addi 9, 3, -8
	mr	10, 7
.Ll3a_BB35_3:                               # %vector.body
                                        # =>This Inner Loop Header: Depth=1
	lwzu 11, 8(8)
	addi 10, 10, -2
	cmplwi	10, 0
	slwi 11, 11, 2
	lwz 12, 4(8)
	lwzx 11, 4, 11
	stwu 11, 8(9)
	slwi 11, 12, 2
	lwzx 11, 4, 11
	stw 11, 4(9)
	bne	0, .Ll3a_BB35_3
# %bb.4:                                # %middle.block
	cmplw	6, 7
	beqlr	0
.Ll3a_BB35_5:                               # %for.body.preheader8
	slwi 8, 7, 2
	sub	6, 6, 7
	addi 7, 8, -4
	add 5, 5, 7
	add 3, 3, 7
.Ll3a_BB35_6:                               # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lwzu 7, 4(5)
	addi 6, 6, -1
	cmplwi	6, 0
	slwi 7, 7, 2
	lwzx 7, 4, 7
	stwu 7, 4(3)
	bne	0, .Ll3a_BB35_6
	blr
.Ll3a_func_end35:
	.size	l3a_k_gather_f32_r, .Ll3a_func_end35-.Ll3a_func_begin35
                                        # -- End function
	.section	.rodata.cst16,"aM",@progbits,16
	.p2align	4, 0x0                          # -- Begin function l3a_k_offset1_f32_r
.Ll3a_CPI36_0:
	.byte	0                               # 0x0
	.byte	0                               # 0x0
	.byte	0                               # 0x0
	.byte	0                               # 0x0
	.byte	0                               # 0x0
	.byte	0                               # 0x0
	.byte	0                               # 0x0
	.byte	0                               # 0x0
	.byte	0                               # 0x0
	.byte	0                               # 0x0
	.byte	0                               # 0x0
	.byte	0                               # 0x0
	.byte	0                               # 0x0
	.byte	1                               # 0x1
	.byte	2                               # 0x2
	.byte	3                               # 0x3
	.section	.text.l3a_k_offset1_f32_r,"ax",@progbits
	.globl	l3a_k_offset1_f32_r
	.p2align	2
	.type	l3a_k_offset1_f32_r,@function
l3a_k_offset1_f32_r:                    # @l3a_k_offset1_f32_r
.Ll3a_func_begin36:
# %bb.0:                                # %entry
	cmpwi	5, 1
	bltlr	0
# %bb.1:                                # %for.body.preheader
	stwu 1, -96(1)
	lfs 0, 0(4)
	cmplwi	5, 12
	li 6, 0
	std 30, 72(1)                           # 8-byte Folded Spill
	blt	0, .Ll3a_BB36_6
# %bb.2:                                # %vector.ph
	li 6, .Ll3a_CPI36_0@l
	lis 7, .Ll3a_CPI36_0@ha
	stfs 0, 48(1)
	li 8, 0
	li 9, 15
	lvx 2, 7, 6
	addi 7, 1, 48
	rlwinm 6, 5, 0, 1, 29
	addi 10, 1, 32
	mr	11, 6
	lvx 3, 0, 7
	addi 7, 4, 4
	vperm 2, 3, 3, 2
.Ll3a_BB36_3:                               # %vector.body
                                        # =>This Inner Loop Header: Depth=1
	add 12, 7, 8
	lvx 3, 7, 8
	vmr	0, 2
	mr	30, 3
	addi 11, 11, -4
	lvx 4, 12, 9
	cmplwi	11, 0
	lvsl 5, 7, 8
	vperm 2, 3, 4, 5
	vsldoi 3, 0, 2, 12
	vaddfp 3, 3, 2
	stvx 3, 0, 10
	ld 12, 32(1)
	stdux 12, 30, 8
	addi 8, 8, 16
	ld 12, 40(1)
	std 12, 8(30)
	bne	0, .Ll3a_BB36_3
# %bb.4:                                # %middle.block
	addi 7, 1, 16
	cmplw	5, 6
	stvx 2, 0, 7
	beq	0, .Ll3a_BB36_8
# %bb.5:
	lfs 0, 28(1)
.Ll3a_BB36_6:                               # %for.body.preheader11
	slwi 7, 6, 2
	sub	5, 5, 6
	add 6, 7, 3
	add 3, 4, 7
	addi 4, 6, -4
.Ll3a_BB36_7:                               # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lfsu 1, 4(3)
	addi 5, 5, -1
	cmplwi	5, 0
	fadds 0, 0, 1
	stfsu 0, 4(4)
	fmr 0, 1
	bne	0, .Ll3a_BB36_7
.Ll3a_BB36_8:
	ld 30, 72(1)                            # 8-byte Folded Reload
	addi 1, 1, 96
	blr
.Ll3a_func_end36:
	.size	l3a_k_offset1_f32_r, .Ll3a_func_end36-.Ll3a_func_begin36
                                        # -- End function
	.section	.rodata.cst16,"aM",@progbits,16
	.p2align	4, 0x0                          # -- Begin function l3a_k_fir3_f32_r
.Ll3a_CPI37_0:
	.byte	0                               # 0x0
	.byte	0                               # 0x0
	.byte	0                               # 0x0
	.byte	0                               # 0x0
	.byte	0                               # 0x0
	.byte	0                               # 0x0
	.byte	0                               # 0x0
	.byte	0                               # 0x0
	.byte	0                               # 0x0
	.byte	0                               # 0x0
	.byte	0                               # 0x0
	.byte	0                               # 0x0
	.byte	0                               # 0x0
	.byte	1                               # 0x1
	.byte	2                               # 0x2
	.byte	3                               # 0x3
.Ll3a_CPI37_1:
	.long	0x3f000000                      # float 0.5
	.long	0x3f000000                      # float 0.5
	.long	0x3f000000                      # float 0.5
	.long	0x3f000000                      # float 0.5
.Ll3a_CPI37_2:
	.long	0x3e800000                      # float 0.25
	.long	0x3e800000                      # float 0.25
	.long	0x3e800000                      # float 0.25
	.long	0x3e800000                      # float 0.25
	.section	.rodata.cst4,"aM",@progbits,4
	.p2align	2, 0x0
.Ll3a_CPI37_3:
	.long	0x3f000000                      # float 0.5
.Ll3a_CPI37_4:
	.long	0x3e800000                      # float 0.25
	.section	.text.l3a_k_fir3_f32_r,"ax",@progbits
	.globl	l3a_k_fir3_f32_r
	.p2align	2
	.type	l3a_k_fir3_f32_r,@function
l3a_k_fir3_f32_r:                       # @l3a_k_fir3_f32_r
.Ll3a_func_begin37:
# %bb.0:                                # %entry
	cmpwi	5, 1
	bltlr	0
# %bb.1:                                # %for.body.preheader
	stwu 1, -96(1)
	lfs 0, 0(4)
	cmplwi	5, 4
	li 6, 0
	std 30, 72(1)                           # 8-byte Folded Spill
	blt	0, .Ll3a_BB37_6
# %bb.2:                                # %vector.ph
	li 6, .Ll3a_CPI37_0@l
	lis 7, .Ll3a_CPI37_0@ha
	stfs 0, 48(1)
	vspltisw 4, -1
	li 10, .Ll3a_CPI37_1@l
	lis 11, .Ll3a_CPI37_1@ha
	lvx 2, 7, 6
	addi 6, 1, 48
	lis 12, .Ll3a_CPI37_2@ha
	addi 7, 4, 4
	addi 8, 4, 8
	lvx 3, 0, 6
	rlwinm 6, 5, 0, 1, 29
	li 9, 0
	vslw 4, 4, 4
	vperm 2, 3, 3, 2
	lvx 3, 11, 10
	li 11, .Ll3a_CPI37_2@l
	li 10, 15
	lvx 5, 12, 11
	addi 11, 1, 32
	mr	12, 6
.Ll3a_BB37_3:                               # %vector.body
                                        # =>This Inner Loop Header: Depth=1
	add 30, 7, 9
	lvx 0, 7, 9
	vmr	7, 2
	addi 12, 12, -4
	cmplwi	12, 0
	lvx 1, 30, 10
	add 30, 8, 9
	lvsl 6, 7, 9
	vperm 2, 0, 1, 6
	lvx 0, 8, 9
	lvx 1, 30, 10
	mr	30, 3
	lvsl 6, 8, 9
	vperm 0, 0, 1, 6
	vsldoi 1, 7, 2, 12
	vmaddfp 6, 2, 3, 4
	vmaddfp 1, 1, 5, 6
	vmaddfp 0, 0, 5, 1
	stvx 0, 0, 11
	ld 0, 32(1)
	stdux 0, 30, 9
	addi 9, 9, 16
	ld 0, 40(1)
	std 0, 8(30)
	bne	0, .Ll3a_BB37_3
# %bb.4:                                # %middle.block
	addi 7, 1, 16
	cmplw	5, 6
	stvx 2, 0, 7
	beq	0, .Ll3a_BB37_8
# %bb.5:
	lfs 0, 28(1)
.Ll3a_BB37_6:                               # %for.body.preheader16
	slwi 7, 6, 2
	sub	5, 5, 6
	lis 6, .Ll3a_CPI37_3@ha
	add 8, 7, 3
	lfs 1, .Ll3a_CPI37_3@l(6)
	lis 6, .Ll3a_CPI37_4@ha
	add 3, 4, 7
	lfs 2, .Ll3a_CPI37_4@l(6)
	addi 4, 8, -4
.Ll3a_BB37_7:                               # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lfsu 3, 4(3)
	addi 5, 5, -1
	cmplwi	5, 0
	fmuls 4, 3, 1
	fmadds 0, 0, 2, 4
	lfs 5, 4(3)
	fmadds 0, 5, 2, 0
	stfsu 0, 4(4)
	fmr 0, 3
	bne	0, .Ll3a_BB37_7
.Ll3a_BB37_8:
	ld 30, 72(1)                            # 8-byte Folded Reload
	addi 1, 1, 96
	blr
.Ll3a_func_end37:
	.size	l3a_k_fir3_f32_r, .Ll3a_func_end37-.Ll3a_func_begin37
                                        # -- End function
	.section	.text.l3a_k_prefix_f32,"ax",@progbits
	.globl	l3a_k_prefix_f32                # -- Begin function l3a_k_prefix_f32
	.p2align	2
	.type	l3a_k_prefix_f32,@function
l3a_k_prefix_f32:                       # @l3a_k_prefix_f32
.Ll3a_func_begin38:
# %bb.0:                                # %entry
	cmpwi	4, 2
	bltlr	0
# %bb.1:                                # %for.body.preheader
	lfs 0, 0(3)
	addi 4, 4, -1
.Ll3a_BB38_2:                               # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lfsu 1, 4(3)
	addi 4, 4, -1
	cmplwi	4, 0
	fadds 0, 0, 1
	stfs 0, 0(3)
	bne	0, .Ll3a_BB38_2
# %bb.3:                                # %for.end
	blr
.Ll3a_func_end38:
	.size	l3a_k_prefix_f32, .Ll3a_func_end38-.Ll3a_func_begin38
                                        # -- End function
	.section	.text.l3a_k_g_add_f32,"ax",@progbits
	.globl	l3a_k_g_add_f32                 # -- Begin function l3a_k_g_add_f32
	.p2align	2
	.type	l3a_k_g_add_f32,@function
l3a_k_g_add_f32:                        # @l3a_k_g_add_f32
.Ll3a_func_begin39:
# %bb.0:                                # %entry
	lis 4, l3a_g_a@ha
	lis 5, l3a_g_b@ha
	lis 6, l3a_g_c@ha
	li 3, 0
	la 4, l3a_g_a@l(4)
	la 5, l3a_g_b@l(5)
	la 6, l3a_g_c@l(6)
.Ll3a_BB39_1:                               # %vector.body
                                        # =>This Inner Loop Header: Depth=1
	lvx 2, 4, 3
	lvx 3, 5, 3
	vaddfp 2, 2, 3
	stvx 2, 6, 3
	addi 3, 3, 16
	cmplwi	3, 4096
	bne	0, .Ll3a_BB39_1
# %bb.2:                                # %for.end
	blr
.Ll3a_func_end39:
	.size	l3a_k_g_add_f32, .Ll3a_func_end39-.Ll3a_func_begin39
                                        # -- End function
	.section	.text.l3a_k_g_saxpy,"ax",@progbits
	.globl	l3a_k_g_saxpy                   # -- Begin function l3a_k_g_saxpy
	.p2align	2
	.type	l3a_k_g_saxpy,@function
l3a_k_g_saxpy:                          # @l3a_k_g_saxpy
.Ll3a_func_begin40:
# %bb.0:                                # %entry
	stwu 1, -32(1)
	stfs 1, 16(1)
	addi 5, 1, 16
	lis 4, l3a_g_a@ha
	lis 6, l3a_g_c@ha
	li 3, 0
	lvx 2, 0, 5
	lis 5, l3a_g_b@ha
	la 4, l3a_g_a@l(4)
	la 5, l3a_g_b@l(5)
	la 6, l3a_g_c@l(6)
	vspltw 2, 2, 0
.Ll3a_BB40_1:                               # %vector.body
                                        # =>This Inner Loop Header: Depth=1
	lvx 3, 4, 3
	lvx 4, 5, 3
	vmaddfp 3, 2, 3, 4
	stvx 3, 6, 3
	addi 3, 3, 16
	cmplwi	3, 4096
	bne	0, .Ll3a_BB40_1
# %bb.2:                                # %for.end
	addi 1, 1, 32
	blr
.Ll3a_func_end40:
	.size	l3a_k_g_saxpy, .Ll3a_func_end40-.Ll3a_func_begin40
                                        # -- End function
	.section	.text.l3a_k_g_add_i32,"ax",@progbits
	.globl	l3a_k_g_add_i32                 # -- Begin function l3a_k_g_add_i32
	.p2align	2
	.type	l3a_k_g_add_i32,@function
l3a_k_g_add_i32:                        # @l3a_k_g_add_i32
.Ll3a_func_begin41:
# %bb.0:                                # %entry
	lis 4, l3a_g_ia@ha
	lis 5, l3a_g_ib@ha
	lis 6, l3a_g_ic@ha
	li 3, 0
	la 4, l3a_g_ia@l(4)
	la 5, l3a_g_ib@l(5)
	la 6, l3a_g_ic@l(6)
.Ll3a_BB41_1:                               # %vector.body
                                        # =>This Inner Loop Header: Depth=1
	lvx 2, 4, 3
	lvx 3, 5, 3
	vadduwm 2, 3, 2
	stvx 2, 6, 3
	addi 3, 3, 16
	cmplwi	3, 4096
	bne	0, .Ll3a_BB41_1
# %bb.2:                                # %for.end
	blr
.Ll3a_func_end41:
	.size	l3a_k_g_add_i32, .Ll3a_func_end41-.Ll3a_func_begin41
                                        # -- End function
	.section	.rodata.cst4,"aM",@progbits,4
	.p2align	2, 0x0                          # -- Begin function l3a_k_g_dot
.Ll3a_CPI42_0:
	.long	0x00000000                      # float 0
	.section	.text.l3a_k_g_dot,"ax",@progbits
	.globl	l3a_k_g_dot
	.p2align	2
	.type	l3a_k_g_dot,@function
l3a_k_g_dot:                            # @l3a_k_g_dot
.Ll3a_func_begin42:
# %bb.0:                                # %entry
	lis 6, .Ll3a_CPI42_0@ha
	lis 3, l3a_g_b@ha
	lfs 1, .Ll3a_CPI42_0@l(6)
	lis 4, l3a_g_a@ha
	la 3, l3a_g_b@l(3)
	la 4, l3a_g_a@l(4)
	addi 3, 3, -4
	addi 4, 4, -4
	li 5, 0
.Ll3a_BB42_1:                               # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lfsu 0, 4(3)
	addi 5, 5, 4
	cmplwi	5, 4096
	lfsu 2, 4(4)
	fmadds 1, 2, 0, 1
	bne	0, .Ll3a_BB42_1
# %bb.2:                                # %for.end
	blr
.Ll3a_func_end42:
	.size	l3a_k_g_dot, .Ll3a_func_end42-.Ll3a_func_begin42
                                        # -- End function
	.section	.text.l3a_k_add16_f32_r,"ax",@progbits
	.globl	l3a_k_add16_f32_r               # -- Begin function l3a_k_add16_f32_r
	.p2align	2
	.type	l3a_k_add16_f32_r,@function
l3a_k_add16_f32_r:                      # @l3a_k_add16_f32_r
.Ll3a_func_begin43:
# %bb.0:                                # %entry
	lfs 0, 0(4)
	lfs 1, 0(5)
	fadds 0, 0, 1
	lfs 1, 4(5)
	stfs 0, 0(3)
	lfs 0, 4(4)
	fadds 0, 0, 1
	lfs 1, 8(5)
	stfs 0, 4(3)
	lfs 0, 8(4)
	fadds 0, 0, 1
	lfs 1, 12(5)
	stfs 0, 8(3)
	lfs 0, 12(4)
	fadds 0, 0, 1
	lfs 1, 16(5)
	stfs 0, 12(3)
	lfs 0, 16(4)
	fadds 0, 0, 1
	lfs 1, 20(5)
	stfs 0, 16(3)
	lfs 0, 20(4)
	fadds 0, 0, 1
	lfs 1, 24(5)
	stfs 0, 20(3)
	lfs 0, 24(4)
	fadds 0, 0, 1
	lfs 1, 28(5)
	stfs 0, 24(3)
	lfs 0, 28(4)
	fadds 0, 0, 1
	lfs 1, 32(5)
	stfs 0, 28(3)
	lfs 0, 32(4)
	fadds 0, 0, 1
	lfs 1, 36(5)
	stfs 0, 32(3)
	lfs 0, 36(4)
	fadds 0, 0, 1
	lfs 1, 40(5)
	stfs 0, 36(3)
	lfs 0, 40(4)
	fadds 0, 0, 1
	lfs 1, 44(5)
	stfs 0, 40(3)
	lfs 0, 44(4)
	fadds 0, 0, 1
	lfs 1, 48(5)
	stfs 0, 44(3)
	lfs 0, 48(4)
	fadds 0, 0, 1
	lfs 1, 52(5)
	stfs 0, 48(3)
	lfs 0, 52(4)
	fadds 0, 0, 1
	lfs 1, 56(5)
	stfs 0, 52(3)
	lfs 0, 56(4)
	fadds 0, 0, 1
	lfs 1, 60(5)
	stfs 0, 56(3)
	lfs 0, 60(4)
	fadds 0, 0, 1
	stfs 0, 60(3)
	blr
.Ll3a_func_end43:
	.size	l3a_k_add16_f32_r, .Ll3a_func_end43-.Ll3a_func_begin43
                                        # -- End function
	.type	l3a_g_a,@object                 # @l3a_g_a
	.section	.bss.l3a_g_a,"aw",@nobits
	.globl	l3a_g_a
	.p2align	4, 0x0
l3a_g_a:
	.space	4096
	.size	l3a_g_a, 4096

	.type	l3a_g_b,@object                 # @l3a_g_b
	.section	.bss.l3a_g_b,"aw",@nobits
	.globl	l3a_g_b
	.p2align	4, 0x0
l3a_g_b:
	.space	4096
	.size	l3a_g_b, 4096

	.type	l3a_g_c,@object                 # @l3a_g_c
	.section	.bss.l3a_g_c,"aw",@nobits
	.globl	l3a_g_c
	.p2align	4, 0x0
l3a_g_c:
	.space	4096
	.size	l3a_g_c, 4096

	.type	l3a_g_ia,@object                # @l3a_g_ia
	.section	.bss.l3a_g_ia,"aw",@nobits
	.globl	l3a_g_ia
	.p2align	4, 0x0
l3a_g_ia:
	.space	4096
	.size	l3a_g_ia, 4096

	.type	l3a_g_ib,@object                # @l3a_g_ib
	.section	.bss.l3a_g_ib,"aw",@nobits
	.globl	l3a_g_ib
	.p2align	4, 0x0
l3a_g_ib:
	.space	4096
	.size	l3a_g_ib, 4096

	.type	l3a_g_ic,@object                # @l3a_g_ic
	.section	.bss.l3a_g_ic,"aw",@nobits
	.globl	l3a_g_ic
	.p2align	4, 0x0
l3a_g_ic:
	.space	4096
	.size	l3a_g_ic, 4096

