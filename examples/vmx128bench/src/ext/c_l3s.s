# kernels.c: clang version 22.1.8 (git@github.com:landaire/llvm-project.git 5f59e9fff8d5dc5b1fa65127b3579d4b1a71f950)
# flags: --target=powerpc64-unknown-xbox360 -ffreestanding -fno-builtin -fms-extensions -ffunction-sections -fdata-sections -fno-ident -fno-addrsig -fno-asynchronous-unwind-tables -g0 -O3 -mno-altivec
	.section	.text.l3s_k_saxpy,"ax",@progbits
	.globl	l3s_k_saxpy                     # -- Begin function l3s_k_saxpy
	.p2align	2
	.type	l3s_k_saxpy,@function
l3s_k_saxpy:                            # @l3s_k_saxpy
.Ll3s_func_begin0:
# %bb.0:                                # %entry
	cmpwi	6, 1
	bltlr	0
# %bb.1:                                # %for.body.preheader
	addi 4, 4, -4
	addi 3, 3, -4
.Ll3s_BB0_2:                                # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lfsu 0, 4(3)
	addi 6, 6, -1
	cmplwi	6, 0
	lfsu 2, 4(4)
	fmadds 0, 1, 2, 0
	stfs 0, 0(3)
	bne	0, .Ll3s_BB0_2
# %bb.3:                                # %for.end
	blr
.Ll3s_func_end0:
	.size	l3s_k_saxpy, .Ll3s_func_end0-.Ll3s_func_begin0
                                        # -- End function
	.section	.text.l3s_k_saxpy_r,"ax",@progbits
	.globl	l3s_k_saxpy_r                   # -- Begin function l3s_k_saxpy_r
	.p2align	2
	.type	l3s_k_saxpy_r,@function
l3s_k_saxpy_r:                          # @l3s_k_saxpy_r
.Ll3s_func_begin1:
# %bb.0:                                # %entry
	cmpwi	6, 1
	bltlr	0
# %bb.1:                                # %for.body.preheader
	li 5, 0
	beq	0, .Ll3s_BB1_5
# %bb.2:                                # %vector.ph
	rlwinm 5, 6, 0, 1, 30
	addi 7, 4, -8
	addi 8, 3, -8
	mr	9, 5
.Ll3s_BB1_3:                                # %vector.body
                                        # =>This Inner Loop Header: Depth=1
	lfsu 0, 8(8)
	addi 9, 9, -2
	cmplwi	9, 0
	lfsu 2, 8(7)
	fmadds 0, 1, 2, 0
	lfs 3, 4(7)
	lfs 4, 4(8)
	stfs 0, 0(8)
	fmadds 0, 1, 3, 4
	stfs 0, 4(8)
	bne	0, .Ll3s_BB1_3
# %bb.4:                                # %middle.block
	cmplw	6, 5
	beqlr	0
.Ll3s_BB1_5:                                # %for.body.preheader10
	slwi 7, 5, 2
	sub	5, 6, 5
	addi 6, 7, -4
	add 4, 4, 6
	add 3, 3, 6
.Ll3s_BB1_6:                                # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lfsu 0, 4(3)
	addi 5, 5, -1
	cmplwi	5, 0
	lfsu 2, 4(4)
	fmadds 0, 1, 2, 0
	stfs 0, 0(3)
	bne	0, .Ll3s_BB1_6
	blr
.Ll3s_func_end1:
	.size	l3s_k_saxpy_r, .Ll3s_func_end1-.Ll3s_func_begin1
                                        # -- End function
	.section	.text.l3s_k_scale_inplace,"ax",@progbits
	.globl	l3s_k_scale_inplace             # -- Begin function l3s_k_scale_inplace
	.p2align	2
	.type	l3s_k_scale_inplace,@function
l3s_k_scale_inplace:                    # @l3s_k_scale_inplace
.Ll3s_func_begin2:
# %bb.0:                                # %entry
	cmpwi	5, 1
	bltlr	0
# %bb.1:                                # %for.body.preheader
	li 4, 0
	beq	0, .Ll3s_BB2_5
# %bb.2:                                # %vector.ph
	rlwinm 4, 5, 0, 1, 30
	addi 6, 3, -8
	mr	7, 4
.Ll3s_BB2_3:                                # %vector.body
                                        # =>This Inner Loop Header: Depth=1
	lfsu 0, 8(6)
	addi 7, 7, -2
	cmplwi	7, 0
	fmuls 0, 1, 0
	lfs 2, 4(6)
	stfs 0, 0(6)
	fmuls 0, 1, 2
	stfs 0, 4(6)
	bne	0, .Ll3s_BB2_3
# %bb.4:                                # %middle.block
	cmplw	5, 4
	beqlr	0
.Ll3s_BB2_5:                                # %for.body.preheader5
	slwi 6, 4, 2
	add 6, 6, 3
	sub	3, 5, 4
	addi 4, 6, -4
.Ll3s_BB2_6:                                # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lfsu 0, 4(4)
	addi 3, 3, -1
	cmplwi	3, 0
	fmuls 0, 1, 0
	stfs 0, 0(4)
	bne	0, .Ll3s_BB2_6
	blr
.Ll3s_func_end2:
	.size	l3s_k_scale_inplace, .Ll3s_func_end2-.Ll3s_func_begin2
                                        # -- End function
	.section	.text.l3s_k_add_f32_r,"ax",@progbits
	.globl	l3s_k_add_f32_r                 # -- Begin function l3s_k_add_f32_r
	.p2align	2
	.type	l3s_k_add_f32_r,@function
l3s_k_add_f32_r:                        # @l3s_k_add_f32_r
.Ll3s_func_begin3:
# %bb.0:                                # %entry
	cmpwi	6, 1
	bltlr	0
# %bb.1:                                # %for.body.preheader
	li 7, 0
	beq	0, .Ll3s_BB3_5
# %bb.2:                                # %vector.ph
	rlwinm 7, 6, 0, 1, 30
	addi 8, 4, -8
	addi 9, 5, -8
	addi 10, 3, -8
	mr	11, 7
.Ll3s_BB3_3:                                # %vector.body
                                        # =>This Inner Loop Header: Depth=1
	lfsu 0, 8(8)
	addi 11, 11, -2
	cmplwi	11, 0
	lfsu 1, 8(9)
	fadds 0, 0, 1
	stfsu 0, 8(10)
	lfs 0, 4(8)
	lfs 1, 4(9)
	fadds 0, 0, 1
	stfs 0, 4(10)
	bne	0, .Ll3s_BB3_3
# %bb.4:                                # %middle.block
	cmplw	6, 7
	beqlr	0
.Ll3s_BB3_5:                                # %for.body.preheader9
	slwi 8, 7, 2
	sub	6, 6, 7
	addi 8, 8, -4
	add 4, 4, 8
	add 5, 5, 8
	add 3, 3, 8
.Ll3s_BB3_6:                                # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lfsu 0, 4(4)
	addi 6, 6, -1
	cmplwi	6, 0
	lfsu 1, 4(5)
	fadds 0, 0, 1
	stfsu 0, 4(3)
	bne	0, .Ll3s_BB3_6
	blr
.Ll3s_func_end3:
	.size	l3s_k_add_f32_r, .Ll3s_func_end3-.Ll3s_func_begin3
                                        # -- End function
	.section	.text.l3s_k_mul_f32_r,"ax",@progbits
	.globl	l3s_k_mul_f32_r                 # -- Begin function l3s_k_mul_f32_r
	.p2align	2
	.type	l3s_k_mul_f32_r,@function
l3s_k_mul_f32_r:                        # @l3s_k_mul_f32_r
.Ll3s_func_begin4:
# %bb.0:                                # %entry
	cmpwi	6, 1
	bltlr	0
# %bb.1:                                # %for.body.preheader
	li 7, 0
	beq	0, .Ll3s_BB4_5
# %bb.2:                                # %vector.ph
	rlwinm 7, 6, 0, 1, 30
	addi 8, 4, -8
	addi 9, 5, -8
	addi 10, 3, -8
	mr	11, 7
.Ll3s_BB4_3:                                # %vector.body
                                        # =>This Inner Loop Header: Depth=1
	lfsu 0, 8(8)
	addi 11, 11, -2
	cmplwi	11, 0
	lfsu 1, 8(9)
	fmuls 0, 0, 1
	stfsu 0, 8(10)
	lfs 0, 4(8)
	lfs 1, 4(9)
	fmuls 0, 0, 1
	stfs 0, 4(10)
	bne	0, .Ll3s_BB4_3
# %bb.4:                                # %middle.block
	cmplw	6, 7
	beqlr	0
.Ll3s_BB4_5:                                # %for.body.preheader9
	slwi 8, 7, 2
	sub	6, 6, 7
	addi 8, 8, -4
	add 4, 4, 8
	add 5, 5, 8
	add 3, 3, 8
.Ll3s_BB4_6:                                # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lfsu 0, 4(4)
	addi 6, 6, -1
	cmplwi	6, 0
	lfsu 1, 4(5)
	fmuls 0, 0, 1
	stfsu 0, 4(3)
	bne	0, .Ll3s_BB4_6
	blr
.Ll3s_func_end4:
	.size	l3s_k_mul_f32_r, .Ll3s_func_end4-.Ll3s_func_begin4
                                        # -- End function
	.section	.rodata.cst4,"aM",@progbits,4
	.p2align	2, 0x0                          # -- Begin function l3s_k_sum_f32
.Ll3s_CPI5_0:
	.long	0x00000000                      # float 0
	.section	.text.l3s_k_sum_f32,"ax",@progbits
	.globl	l3s_k_sum_f32
	.p2align	2
	.type	l3s_k_sum_f32,@function
l3s_k_sum_f32:                          # @l3s_k_sum_f32
.Ll3s_func_begin5:
# %bb.0:                                # %entry
	cmpwi	4, 1
	lis 5, .Ll3s_CPI5_0@ha
	blt	0, .Ll3s_BB5_4
# %bb.1:                                # %for.body.preheader
	lfs 1, .Ll3s_CPI5_0@l(5)
	addi 3, 3, -4
.Ll3s_BB5_2:                                # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lfsu 0, 4(3)
	addi 4, 4, -1
	cmplwi	4, 0
	fadds 1, 1, 0
	bne	0, .Ll3s_BB5_2
# %bb.3:                                # %for.end
	blr
.Ll3s_BB5_4:
	lfs 1, .Ll3s_CPI5_0@l(5)
	blr
.Ll3s_func_end5:
	.size	l3s_k_sum_f32, .Ll3s_func_end5-.Ll3s_func_begin5
                                        # -- End function
	.section	.rodata.cst4,"aM",@progbits,4
	.p2align	2, 0x0                          # -- Begin function l3s_k_dot_f32
.Ll3s_CPI6_0:
	.long	0x00000000                      # float 0
	.section	.text.l3s_k_dot_f32,"ax",@progbits
	.globl	l3s_k_dot_f32
	.p2align	2
	.type	l3s_k_dot_f32,@function
l3s_k_dot_f32:                          # @l3s_k_dot_f32
.Ll3s_func_begin6:
# %bb.0:                                # %entry
	cmpwi	5, 1
	lis 6, .Ll3s_CPI6_0@ha
	blt	0, .Ll3s_BB6_4
# %bb.1:                                # %for.body.preheader
	lfs 1, .Ll3s_CPI6_0@l(6)
	addi 3, 3, -4
	addi 4, 4, -4
.Ll3s_BB6_2:                                # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lfsu 0, 4(4)
	addi 5, 5, -1
	cmplwi	5, 0
	lfsu 2, 4(3)
	fmadds 1, 2, 0, 1
	bne	0, .Ll3s_BB6_2
# %bb.3:                                # %for.end
	blr
.Ll3s_BB6_4:
	lfs 1, .Ll3s_CPI6_0@l(6)
	blr
.Ll3s_func_end6:
	.size	l3s_k_dot_f32, .Ll3s_func_end6-.Ll3s_func_begin6
                                        # -- End function
	.section	.text.l3s_k_sum_i32,"ax",@progbits
	.globl	l3s_k_sum_i32                   # -- Begin function l3s_k_sum_i32
	.p2align	2
	.type	l3s_k_sum_i32,@function
l3s_k_sum_i32:                          # @l3s_k_sum_i32
.Ll3s_func_begin7:
# %bb.0:                                # %entry
	li 5, 0
	cmpwi	4, 1
	blt	0, .Ll3s_BB7_7
# %bb.1:                                # %for.body.preheader
	li 6, 0
	beq	0, .Ll3s_BB7_5
# %bb.2:                                # %vector.ph
	rlwinm 6, 4, 0, 1, 30
	addi 7, 3, -8
	mr	9, 6
	li 8, 0
.Ll3s_BB7_3:                                # %vector.body
                                        # =>This Inner Loop Header: Depth=1
	lwzu 10, 8(7)
	addi 9, 9, -2
	cmplwi	9, 0
	lwz 11, 4(7)
	add 5, 10, 5
	add 8, 11, 8
	bne	0, .Ll3s_BB7_3
# %bb.4:                                # %middle.block
	cmplw	4, 6
	add 5, 8, 5
	beq	0, .Ll3s_BB7_7
.Ll3s_BB7_5:                                # %for.body.preheader8
	slwi 7, 6, 2
	add 7, 7, 3
	sub	3, 4, 6
	addi 4, 7, -4
.Ll3s_BB7_6:                                # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lwzu 6, 4(4)
	addi 3, 3, -1
	cmplwi	3, 0
	add 5, 6, 5
	bne	0, .Ll3s_BB7_6
.Ll3s_BB7_7:                                # %for.end
	mr	3, 5
	blr
.Ll3s_func_end7:
	.size	l3s_k_sum_i32, .Ll3s_func_end7-.Ll3s_func_begin7
                                        # -- End function
	.section	.text.l3s_k_dot_i16,"ax",@progbits
	.globl	l3s_k_dot_i16                   # -- Begin function l3s_k_dot_i16
	.p2align	2
	.type	l3s_k_dot_i16,@function
l3s_k_dot_i16:                          # @l3s_k_dot_i16
.Ll3s_func_begin8:
# %bb.0:                                # %entry
	cmpwi	5, 1
	blt	0, .Ll3s_BB8_8
# %bb.1:                                # %for.body.preheader
	stwu 1, -48(1)
	li 6, 0
	li 7, 0
	std 29, 16(1)                           # 8-byte Folded Spill
	std 30, 24(1)                           # 8-byte Folded Spill
	beq	0, .Ll3s_BB8_5
# %bb.2:                                # %vector.ph
	rlwinm 7, 5, 0, 1, 30
	addi 8, 3, -4
	addi 9, 4, -4
	mr	11, 7
	li 10, 0
.Ll3s_BB8_3:                                # %vector.body
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
	bne	0, .Ll3s_BB8_3
# %bb.4:                                # %middle.block
	cmplw	5, 7
	add 6, 10, 6
	beq	0, .Ll3s_BB8_7
.Ll3s_BB8_5:                                # %for.body.preheader11
	slwi 8, 7, 1
	sub	5, 5, 7
	addi 7, 8, -2
	add 3, 3, 7
	add 4, 4, 7
.Ll3s_BB8_6:                                # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lhau 7, 2(4)
	addi 5, 5, -1
	cmplwi	5, 0
	lhau 8, 2(3)
	mullw 7, 7, 8
	add 6, 7, 6
	bne	0, .Ll3s_BB8_6
.Ll3s_BB8_7:
	ld 30, 24(1)                            # 8-byte Folded Reload
	ld 29, 16(1)                            # 8-byte Folded Reload
	addi 1, 1, 48
	mr	3, 6
	blr
.Ll3s_BB8_8:
	li 3, 0
	blr
.Ll3s_func_end8:
	.size	l3s_k_dot_i16, .Ll3s_func_end8-.Ll3s_func_begin8
                                        # -- End function
	.section	.text.l3s_k_max_f32,"ax",@progbits
	.globl	l3s_k_max_f32                   # -- Begin function l3s_k_max_f32
	.p2align	2
	.type	l3s_k_max_f32,@function
l3s_k_max_f32:                          # @l3s_k_max_f32
.Ll3s_func_begin9:
# %bb.0:                                # %entry
	lfs 0, 0(3)
	cmpwi	4, 2
	blt	0, .Ll3s_BB9_5
# %bb.1:                                # %for.body.preheader
	addi 4, 4, -1
	b .Ll3s_BB9_3
.Ll3s_BB9_2:                                # %for.body
                                        #   in Loop: Header=BB9_3 Depth=1
	addi 4, 4, -1
	fmr 0, 1
	cmplwi	4, 0
	beqlr	0
.Ll3s_BB9_3:                                # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lfsu 1, 4(3)
	fcmpu 0, 1, 0
	bgt	0, .Ll3s_BB9_2
# %bb.4:                                # %for.body
                                        #   in Loop: Header=BB9_3 Depth=1
	fmr 1, 0
	b .Ll3s_BB9_2
.Ll3s_BB9_5:
	fmr 1, 0
	blr
.Ll3s_func_end9:
	.size	l3s_k_max_f32, .Ll3s_func_end9-.Ll3s_func_begin9
                                        # -- End function
	.section	.text.l3s_k_add_i32,"ax",@progbits
	.globl	l3s_k_add_i32                   # -- Begin function l3s_k_add_i32
	.p2align	2
	.type	l3s_k_add_i32,@function
l3s_k_add_i32:                          # @l3s_k_add_i32
.Ll3s_func_begin10:
# %bb.0:                                # %entry
	cmpwi	6, 1
	bltlr	0
# %bb.1:                                # %for.body.preheader
	addi 4, 4, -4
	addi 5, 5, -4
	addi 3, 3, -4
.Ll3s_BB10_2:                               # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lwzu 7, 4(4)
	addi 6, 6, -1
	cmplwi	6, 0
	lwzu 8, 4(5)
	add 7, 8, 7
	stwu 7, 4(3)
	bne	0, .Ll3s_BB10_2
# %bb.3:                                # %for.end
	blr
.Ll3s_func_end10:
	.size	l3s_k_add_i32, .Ll3s_func_end10-.Ll3s_func_begin10
                                        # -- End function
	.section	.text.l3s_k_add_i32_r,"ax",@progbits
	.globl	l3s_k_add_i32_r                 # -- Begin function l3s_k_add_i32_r
	.p2align	2
	.type	l3s_k_add_i32_r,@function
l3s_k_add_i32_r:                        # @l3s_k_add_i32_r
.Ll3s_func_begin11:
# %bb.0:                                # %entry
	cmpwi	6, 1
	bltlr	0
# %bb.1:                                # %for.body.preheader
	stwu 1, -48(1)
	li 7, 0
	std 29, 16(1)                           # 8-byte Folded Spill
	std 30, 24(1)                           # 8-byte Folded Spill
	beq	0, .Ll3s_BB11_5
# %bb.2:                                # %vector.ph
	rlwinm 7, 6, 0, 1, 30
	addi 8, 4, -8
	addi 9, 5, -8
	addi 10, 3, -8
	mr	11, 7
.Ll3s_BB11_3:                               # %vector.body
                                        # =>This Inner Loop Header: Depth=1
	lwzu 12, 8(8)
	addi 11, 11, -2
	cmplwi	11, 0
	lwzu 0, 8(9)
	lwz 30, 4(8)
	add 12, 0, 12
	lwz 29, 4(9)
	stwu 12, 8(10)
	add 12, 29, 30
	stw 12, 4(10)
	bne	0, .Ll3s_BB11_3
# %bb.4:                                # %middle.block
	cmplw	6, 7
	beq	0, .Ll3s_BB11_7
.Ll3s_BB11_5:                               # %for.body.preheader9
	slwi 8, 7, 2
	sub	6, 6, 7
	addi 8, 8, -4
	add 4, 4, 8
	add 5, 5, 8
	add 3, 3, 8
.Ll3s_BB11_6:                               # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lwzu 7, 4(4)
	addi 6, 6, -1
	cmplwi	6, 0
	lwzu 8, 4(5)
	add 7, 8, 7
	stwu 7, 4(3)
	bne	0, .Ll3s_BB11_6
.Ll3s_BB11_7:
	ld 30, 24(1)                            # 8-byte Folded Reload
	ld 29, 16(1)                            # 8-byte Folded Reload
	addi 1, 1, 48
	blr
.Ll3s_func_end11:
	.size	l3s_k_add_i32_r, .Ll3s_func_end11-.Ll3s_func_begin11
                                        # -- End function
	.section	.text.l3s_k_add_i16_r,"ax",@progbits
	.globl	l3s_k_add_i16_r                 # -- Begin function l3s_k_add_i16_r
	.p2align	2
	.type	l3s_k_add_i16_r,@function
l3s_k_add_i16_r:                        # @l3s_k_add_i16_r
.Ll3s_func_begin12:
# %bb.0:                                # %entry
	cmpwi	6, 1
	bltlr	0
# %bb.1:                                # %for.body.preheader
	stwu 1, -48(1)
	li 7, 0
	std 29, 16(1)                           # 8-byte Folded Spill
	std 30, 24(1)                           # 8-byte Folded Spill
	beq	0, .Ll3s_BB12_5
# %bb.2:                                # %vector.ph
	rlwinm 7, 6, 0, 1, 30
	addi 8, 4, -4
	addi 9, 5, -4
	addi 10, 3, -4
	mr	11, 7
.Ll3s_BB12_3:                               # %vector.body
                                        # =>This Inner Loop Header: Depth=1
	lhzu 12, 4(8)
	addi 11, 11, -2
	cmplwi	11, 0
	lhzu 0, 4(9)
	lhz 30, 2(8)
	add 12, 0, 12
	lhz 29, 2(9)
	sthu 12, 4(10)
	add 12, 29, 30
	sth 12, 2(10)
	bne	0, .Ll3s_BB12_3
# %bb.4:                                # %middle.block
	cmplw	6, 7
	beq	0, .Ll3s_BB12_7
.Ll3s_BB12_5:                               # %for.body.preheader11
	slwi 8, 7, 1
	sub	6, 6, 7
	addi 8, 8, -2
	add 4, 4, 8
	add 5, 5, 8
	add 3, 3, 8
.Ll3s_BB12_6:                               # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lhzu 7, 2(4)
	addi 6, 6, -1
	cmplwi	6, 0
	lhzu 8, 2(5)
	add 7, 8, 7
	sthu 7, 2(3)
	bne	0, .Ll3s_BB12_6
.Ll3s_BB12_7:
	ld 30, 24(1)                            # 8-byte Folded Reload
	ld 29, 16(1)                            # 8-byte Folded Reload
	addi 1, 1, 48
	blr
.Ll3s_func_end12:
	.size	l3s_k_add_i16_r, .Ll3s_func_end12-.Ll3s_func_begin12
                                        # -- End function
	.section	.text.l3s_k_add_u8_r,"ax",@progbits
	.globl	l3s_k_add_u8_r                  # -- Begin function l3s_k_add_u8_r
	.p2align	2
	.type	l3s_k_add_u8_r,@function
l3s_k_add_u8_r:                         # @l3s_k_add_u8_r
.Ll3s_func_begin13:
# %bb.0:                                # %entry
	cmpwi	6, 1
	bltlr	0
# %bb.1:                                # %for.body.preheader
	stwu 1, -48(1)
	li 7, 0
	std 29, 16(1)                           # 8-byte Folded Spill
	std 30, 24(1)                           # 8-byte Folded Spill
	beq	0, .Ll3s_BB13_5
# %bb.2:                                # %vector.ph
	rlwinm 7, 6, 0, 1, 30
	addi 8, 4, -2
	addi 9, 5, -2
	addi 10, 3, -2
	li 11, 0
.Ll3s_BB13_3:                               # %vector.body
                                        # =>This Inner Loop Header: Depth=1
	lbzu 12, 2(8)
	addi 11, 11, 2
	cmplw	7, 11
	lbzu 0, 2(9)
	lbz 30, 1(8)
	add 12, 0, 12
	lbz 29, 1(9)
	stbu 12, 2(10)
	add 12, 29, 30
	stb 12, 1(10)
	bne	0, .Ll3s_BB13_3
# %bb.4:                                # %middle.block
	cmplw	6, 7
	beq	0, .Ll3s_BB13_7
.Ll3s_BB13_5:                               # %for.body.preheader11
	addi 8, 7, -1
	add 4, 4, 8
	add 5, 5, 8
	add 3, 3, 8
.Ll3s_BB13_6:                               # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lbzu 8, 1(4)
	addi 7, 7, 1
	cmplw	6, 7
	lbzu 9, 1(5)
	add 8, 9, 8
	stbu 8, 1(3)
	bne	0, .Ll3s_BB13_6
.Ll3s_BB13_7:
	ld 30, 24(1)                            # 8-byte Folded Reload
	ld 29, 16(1)                            # 8-byte Folded Reload
	addi 1, 1, 48
	blr
.Ll3s_func_end13:
	.size	l3s_k_add_u8_r, .Ll3s_func_end13-.Ll3s_func_begin13
                                        # -- End function
	.section	.text.l3s_k_avg_u8_r,"ax",@progbits
	.globl	l3s_k_avg_u8_r                  # -- Begin function l3s_k_avg_u8_r
	.p2align	2
	.type	l3s_k_avg_u8_r,@function
l3s_k_avg_u8_r:                         # @l3s_k_avg_u8_r
.Ll3s_func_begin14:
# %bb.0:                                # %entry
	cmpwi	6, 1
	bltlr	0
# %bb.1:                                # %for.body.preheader
	stwu 1, -48(1)
	li 7, 0
	std 30, 24(1)                           # 8-byte Folded Spill
	beq	0, .Ll3s_BB14_5
# %bb.2:                                # %vector.ph
	rlwinm 7, 6, 0, 1, 30
	addi 8, 4, -2
	addi 9, 5, -2
	addi 10, 3, -2
	li 11, 0
.Ll3s_BB14_3:                               # %vector.body
                                        # =>This Inner Loop Header: Depth=1
	lbzu 12, 2(8)
	addi 11, 11, 2
	cmplw	7, 11
	lbzu 0, 2(9)
	lbz 30, 1(8)
	add 12, 12, 0
	addi 12, 12, 1
	srwi 12, 12, 1
	lbz 0, 1(9)
	add 30, 30, 0
	stbu 12, 2(10)
	addi 12, 30, 1
	srwi 12, 12, 1
	stb 12, 1(10)
	bne	0, .Ll3s_BB14_3
# %bb.4:                                # %middle.block
	cmplw	6, 7
	beq	0, .Ll3s_BB14_7
.Ll3s_BB14_5:                               # %for.body.preheader12
	addi 8, 7, -1
	add 4, 4, 8
	add 5, 5, 8
	add 3, 3, 8
.Ll3s_BB14_6:                               # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lbzu 8, 1(4)
	addi 7, 7, 1
	cmplw	6, 7
	lbzu 9, 1(5)
	add 8, 8, 9
	addi 8, 8, 1
	srwi 8, 8, 1
	stbu 8, 1(3)
	bne	0, .Ll3s_BB14_6
.Ll3s_BB14_7:
	ld 30, 24(1)                            # 8-byte Folded Reload
	addi 1, 1, 48
	blr
.Ll3s_func_end14:
	.size	l3s_k_avg_u8_r, .Ll3s_func_end14-.Ll3s_func_begin14
                                        # -- End function
	.section	.text.l3s_k_addsat_u8_r,"ax",@progbits
	.globl	l3s_k_addsat_u8_r               # -- Begin function l3s_k_addsat_u8_r
	.p2align	2
	.type	l3s_k_addsat_u8_r,@function
l3s_k_addsat_u8_r:                      # @l3s_k_addsat_u8_r
.Ll3s_func_begin15:
# %bb.0:                                # %entry
	cmpwi	6, 1
	bltlr	0
# %bb.1:                                # %for.body.preheader
	stwu 1, -48(1)
	li 7, 0
	std 30, 24(1)                           # 8-byte Folded Spill
	beq	0, .Ll3s_BB15_10
# %bb.2:                                # %vector.ph
	rlwinm 7, 6, 0, 1, 30
	addi 8, 4, -2
	addi 9, 5, -2
	addi 10, 3, -2
	li 11, 0
	b .Ll3s_BB15_4
.Ll3s_BB15_3:                               # %vector.body
                                        #   in Loop: Header=BB15_4 Depth=1
	addi 11, 11, 2
	stb 30, 1(10)
	cmplw	7, 11
	beq	0, .Ll3s_BB15_8
.Ll3s_BB15_4:                               # %vector.body
                                        # =>This Inner Loop Header: Depth=1
	lbzu 12, 2(8)
	lbzu 0, 2(9)
	add 12, 0, 12
	cmplwi	12, 255
	blt	0, .Ll3s_BB15_6
# %bb.5:                                # %vector.body
                                        #   in Loop: Header=BB15_4 Depth=1
	li 12, 255
.Ll3s_BB15_6:                               # %vector.body
                                        #   in Loop: Header=BB15_4 Depth=1
	lbz 0, 1(8)
	lbz 30, 1(9)
	add 30, 30, 0
	stbu 12, 2(10)
	cmplwi	30, 255
	blt	0, .Ll3s_BB15_3
# %bb.7:                                # %vector.body
                                        #   in Loop: Header=BB15_4 Depth=1
	li 30, 255
	b .Ll3s_BB15_3
.Ll3s_BB15_8:                               # %middle.block
	cmplw	6, 7
	bne	0, .Ll3s_BB15_10
.Ll3s_BB15_9:
	ld 30, 24(1)                            # 8-byte Folded Reload
	addi 1, 1, 48
	blr
.Ll3s_BB15_10:                              # %for.body.preheader14
	addi 8, 7, -1
	add 4, 4, 8
	add 5, 5, 8
	add 3, 3, 8
	b .Ll3s_BB15_12
.Ll3s_BB15_11:                              # %for.body
                                        #   in Loop: Header=BB15_12 Depth=1
	addi 7, 7, 1
	stbu 8, 1(3)
	cmplw	6, 7
	beq	0, .Ll3s_BB15_9
.Ll3s_BB15_12:                              # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lbzu 8, 1(4)
	lbzu 9, 1(5)
	add 8, 9, 8
	cmplwi	8, 255
	blt	0, .Ll3s_BB15_11
# %bb.13:                               # %for.body
                                        #   in Loop: Header=BB15_12 Depth=1
	li 8, 255
	b .Ll3s_BB15_11
.Ll3s_func_end15:
	.size	l3s_k_addsat_u8_r, .Ll3s_func_end15-.Ll3s_func_begin15
                                        # -- End function
	.section	.text.l3s_k_and_u32_r,"ax",@progbits
	.globl	l3s_k_and_u32_r                 # -- Begin function l3s_k_and_u32_r
	.p2align	2
	.type	l3s_k_and_u32_r,@function
l3s_k_and_u32_r:                        # @l3s_k_and_u32_r
.Ll3s_func_begin16:
# %bb.0:                                # %entry
	cmpwi	6, 1
	bltlr	0
# %bb.1:                                # %for.body.preheader
	stwu 1, -48(1)
	li 7, 0
	std 29, 16(1)                           # 8-byte Folded Spill
	std 30, 24(1)                           # 8-byte Folded Spill
	beq	0, .Ll3s_BB16_5
# %bb.2:                                # %vector.ph
	rlwinm 7, 6, 0, 1, 30
	addi 8, 4, -8
	addi 9, 5, -8
	addi 10, 3, -8
	mr	11, 7
.Ll3s_BB16_3:                               # %vector.body
                                        # =>This Inner Loop Header: Depth=1
	lwzu 12, 8(8)
	addi 11, 11, -2
	cmplwi	11, 0
	lwzu 0, 8(9)
	lwz 30, 4(8)
	and 12, 0, 12
	lwz 29, 4(9)
	stwu 12, 8(10)
	and 12, 29, 30
	stw 12, 4(10)
	bne	0, .Ll3s_BB16_3
# %bb.4:                                # %middle.block
	cmplw	6, 7
	beq	0, .Ll3s_BB16_7
.Ll3s_BB16_5:                               # %for.body.preheader9
	slwi 8, 7, 2
	sub	6, 6, 7
	addi 8, 8, -4
	add 4, 4, 8
	add 5, 5, 8
	add 3, 3, 8
.Ll3s_BB16_6:                               # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lwzu 7, 4(4)
	addi 6, 6, -1
	cmplwi	6, 0
	lwzu 8, 4(5)
	and 7, 8, 7
	stwu 7, 4(3)
	bne	0, .Ll3s_BB16_6
.Ll3s_BB16_7:
	ld 30, 24(1)                            # 8-byte Folded Reload
	ld 29, 16(1)                            # 8-byte Folded Reload
	addi 1, 1, 48
	blr
.Ll3s_func_end16:
	.size	l3s_k_and_u32_r, .Ll3s_func_end16-.Ll3s_func_begin16
                                        # -- End function
	.section	.text.l3s_k_shl_i32_r,"ax",@progbits
	.globl	l3s_k_shl_i32_r                 # -- Begin function l3s_k_shl_i32_r
	.p2align	2
	.type	l3s_k_shl_i32_r,@function
l3s_k_shl_i32_r:                        # @l3s_k_shl_i32_r
.Ll3s_func_begin17:
# %bb.0:                                # %entry
	cmpwi	5, 1
	bltlr	0
# %bb.1:                                # %for.body.preheader
	li 6, 0
	beq	0, .Ll3s_BB17_5
# %bb.2:                                # %vector.ph
	rlwinm 6, 5, 0, 1, 30
	addi 7, 4, -8
	addi 8, 3, -8
	mr	9, 6
.Ll3s_BB17_3:                               # %vector.body
                                        # =>This Inner Loop Header: Depth=1
	lwzu 10, 8(7)
	addi 9, 9, -2
	cmplwi	9, 0
	lwz 11, 4(7)
	slwi 10, 10, 3
	stwu 10, 8(8)
	slwi 10, 11, 3
	stw 10, 4(8)
	bne	0, .Ll3s_BB17_3
# %bb.4:                                # %middle.block
	cmplw	5, 6
	beqlr	0
.Ll3s_BB17_5:                               # %for.body.preheader7
	slwi 7, 6, 2
	sub	5, 5, 6
	addi 6, 7, -4
	add 4, 4, 6
	add 3, 3, 6
.Ll3s_BB17_6:                               # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lwzu 6, 4(4)
	addi 5, 5, -1
	cmplwi	5, 0
	slwi 6, 6, 3
	stwu 6, 4(3)
	bne	0, .Ll3s_BB17_6
	blr
.Ll3s_func_end17:
	.size	l3s_k_shl_i32_r, .Ll3s_func_end17-.Ll3s_func_begin17
                                        # -- End function
	.section	.text.l3s_k_mul_i32_r,"ax",@progbits
	.globl	l3s_k_mul_i32_r                 # -- Begin function l3s_k_mul_i32_r
	.p2align	2
	.type	l3s_k_mul_i32_r,@function
l3s_k_mul_i32_r:                        # @l3s_k_mul_i32_r
.Ll3s_func_begin18:
# %bb.0:                                # %entry
	cmpwi	6, 1
	bltlr	0
# %bb.1:                                # %for.body.preheader
	stwu 1, -48(1)
	li 7, 0
	std 30, 24(1)                           # 8-byte Folded Spill
	beq	0, .Ll3s_BB18_5
# %bb.2:                                # %vector.ph
	rlwinm 7, 6, 0, 1, 30
	addi 8, 4, -8
	addi 9, 5, -8
	addi 10, 3, -8
	mr	11, 7
.Ll3s_BB18_3:                               # %vector.body
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
	bne	0, .Ll3s_BB18_3
# %bb.4:                                # %middle.block
	cmplw	6, 7
	beq	0, .Ll3s_BB18_7
.Ll3s_BB18_5:                               # %for.body.preheader9
	slwi 8, 7, 2
	sub	6, 6, 7
	addi 8, 8, -4
	add 4, 4, 8
	add 5, 5, 8
	add 3, 3, 8
.Ll3s_BB18_6:                               # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lwzu 7, 4(4)
	addi 6, 6, -1
	cmplwi	6, 0
	lwzu 8, 4(5)
	mullw 7, 8, 7
	stwu 7, 4(3)
	bne	0, .Ll3s_BB18_6
.Ll3s_BB18_7:
	ld 30, 24(1)                            # 8-byte Folded Reload
	addi 1, 1, 48
	blr
.Ll3s_func_end18:
	.size	l3s_k_mul_i32_r, .Ll3s_func_end18-.Ll3s_func_begin18
                                        # -- End function
	.section	.text.l3s_k_min_f32_r,"ax",@progbits
	.globl	l3s_k_min_f32_r                 # -- Begin function l3s_k_min_f32_r
	.p2align	2
	.type	l3s_k_min_f32_r,@function
l3s_k_min_f32_r:                        # @l3s_k_min_f32_r
.Ll3s_func_begin19:
# %bb.0:                                # %entry
	cmpwi	6, 1
	bltlr	0
# %bb.1:                                # %for.body.preheader
	li 7, 0
	beq	0, .Ll3s_BB19_9
# %bb.2:                                # %vector.ph
	rlwinm 7, 6, 0, 1, 30
	addi 8, 4, -8
	addi 9, 5, -8
	addi 10, 3, -8
	mr	11, 7
	b .Ll3s_BB19_4
.Ll3s_BB19_3:                               # %vector.body
                                        #   in Loop: Header=BB19_4 Depth=1
	addi 11, 11, -2
	stfs 0, 4(10)
	cmplwi	11, 0
	beq	0, .Ll3s_BB19_8
.Ll3s_BB19_4:                               # %vector.body
                                        # =>This Inner Loop Header: Depth=1
	lfsu 1, 8(8)
	lfsu 0, 8(9)
	fcmpu 0, 1, 0
	blt	0, .Ll3s_BB19_6
# %bb.5:                                # %vector.body
                                        #   in Loop: Header=BB19_4 Depth=1
	fmr 1, 0
.Ll3s_BB19_6:                               # %vector.body
                                        #   in Loop: Header=BB19_4 Depth=1
	lfs 0, 4(8)
	lfs 2, 4(9)
	fcmpu 0, 0, 2
	stfsu 1, 8(10)
	blt	0, .Ll3s_BB19_3
# %bb.7:                                # %vector.body
                                        #   in Loop: Header=BB19_4 Depth=1
	fmr 0, 2
	b .Ll3s_BB19_3
.Ll3s_BB19_8:                               # %middle.block
	cmplw	6, 7
	beqlr	0
.Ll3s_BB19_9:                               # %for.body.preheader17
	slwi 8, 7, 2
	sub	6, 6, 7
	addi 8, 8, -4
	add 4, 4, 8
	add 5, 5, 8
	add 3, 3, 8
	b .Ll3s_BB19_11
.Ll3s_BB19_10:                              # %for.body
                                        #   in Loop: Header=BB19_11 Depth=1
	addi 6, 6, -1
	stfsu 0, 4(3)
	cmplwi	6, 0
	beqlr	0
.Ll3s_BB19_11:                              # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lfsu 0, 4(4)
	lfsu 1, 4(5)
	fcmpu 0, 0, 1
	blt	0, .Ll3s_BB19_10
# %bb.12:                               # %for.body
                                        #   in Loop: Header=BB19_11 Depth=1
	fmr 0, 1
	b .Ll3s_BB19_10
.Ll3s_func_end19:
	.size	l3s_k_min_f32_r, .Ll3s_func_end19-.Ll3s_func_begin19
                                        # -- End function
	.section	.text.l3s_k_max_i32_r,"ax",@progbits
	.globl	l3s_k_max_i32_r                 # -- Begin function l3s_k_max_i32_r
	.p2align	2
	.type	l3s_k_max_i32_r,@function
l3s_k_max_i32_r:                        # @l3s_k_max_i32_r
.Ll3s_func_begin20:
# %bb.0:                                # %entry
	cmpwi	6, 1
	bltlr	0
# %bb.1:                                # %for.body.preheader
	stwu 1, -48(1)
	li 7, 0
	std 29, 16(1)                           # 8-byte Folded Spill
	std 30, 24(1)                           # 8-byte Folded Spill
	beq	0, .Ll3s_BB20_10
# %bb.2:                                # %vector.ph
	rlwinm 7, 6, 0, 1, 30
	addi 8, 4, -8
	addi 9, 5, -8
	addi 10, 3, -8
	mr	11, 7
	b .Ll3s_BB20_4
.Ll3s_BB20_3:                               # %vector.body
                                        #   in Loop: Header=BB20_4 Depth=1
	addi 11, 11, -2
	stw 12, 4(10)
	cmplwi	11, 0
	beq	0, .Ll3s_BB20_8
.Ll3s_BB20_4:                               # %vector.body
                                        # =>This Inner Loop Header: Depth=1
	lwzu 30, 8(8)
	lwzu 12, 8(9)
	cmpw	30, 12
	bgt	0, .Ll3s_BB20_6
# %bb.5:                                # %vector.body
                                        #   in Loop: Header=BB20_4 Depth=1
	mr	30, 12
.Ll3s_BB20_6:                               # %vector.body
                                        #   in Loop: Header=BB20_4 Depth=1
	lwz 12, 4(8)
	lwz 29, 4(9)
	cmpw	12, 29
	stwu 30, 8(10)
	bgt	0, .Ll3s_BB20_3
# %bb.7:                                # %vector.body
                                        #   in Loop: Header=BB20_4 Depth=1
	mr	12, 29
	b .Ll3s_BB20_3
.Ll3s_BB20_8:                               # %middle.block
	cmplw	6, 7
	bne	0, .Ll3s_BB20_10
.Ll3s_BB20_9:
	ld 30, 24(1)                            # 8-byte Folded Reload
	ld 29, 16(1)                            # 8-byte Folded Reload
	addi 1, 1, 48
	blr
.Ll3s_BB20_10:                              # %for.body.preheader17
	slwi 8, 7, 2
	sub	6, 6, 7
	addi 8, 8, -4
	add 4, 4, 8
	add 5, 5, 8
	add 3, 3, 8
	b .Ll3s_BB20_12
.Ll3s_BB20_11:                              # %for.body
                                        #   in Loop: Header=BB20_12 Depth=1
	addi 6, 6, -1
	stwu 7, 4(3)
	cmplwi	6, 0
	beq	0, .Ll3s_BB20_9
.Ll3s_BB20_12:                              # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lwzu 7, 4(4)
	lwzu 8, 4(5)
	cmpw	7, 8
	bgt	0, .Ll3s_BB20_11
# %bb.13:                               # %for.body
                                        #   in Loop: Header=BB20_12 Depth=1
	mr	7, 8
	b .Ll3s_BB20_11
.Ll3s_func_end20:
	.size	l3s_k_max_i32_r, .Ll3s_func_end20-.Ll3s_func_begin20
                                        # -- End function
	.section	.text.l3s_k_clamp_f32_r,"ax",@progbits
	.globl	l3s_k_clamp_f32_r               # -- Begin function l3s_k_clamp_f32_r
	.p2align	2
	.type	l3s_k_clamp_f32_r,@function
l3s_k_clamp_f32_r:                      # @l3s_k_clamp_f32_r
.Ll3s_func_begin21:
# %bb.0:                                # %entry
	cmpwi	7, 1
	bltlr	0
# %bb.1:                                # %for.body.preheader
	li 5, 0
	beq	0, .Ll3s_BB21_13
# %bb.2:                                # %vector.ph
	rlwinm 5, 7, 0, 1, 30
	addi 6, 4, -8
	addi 8, 3, -8
	mr	9, 5
	b .Ll3s_BB21_4
.Ll3s_BB21_3:                               # %vector.body
                                        #   in Loop: Header=BB21_4 Depth=1
	addi 9, 9, -2
	stfs 0, 4(8)
	cmplwi	9, 0
	beq	0, .Ll3s_BB21_12
.Ll3s_BB21_4:                               # %vector.body
                                        # =>This Inner Loop Header: Depth=1
	lfsu 3, 8(6)
	fmr 0, 1
	fcmpu 0, 3, 1
	blt	0, .Ll3s_BB21_6
# %bb.5:                                # %vector.body
                                        #   in Loop: Header=BB21_4 Depth=1
	fmr 0, 3
.Ll3s_BB21_6:                               # %vector.body
                                        #   in Loop: Header=BB21_4 Depth=1
	fcmpu 0, 0, 2
	fmr 3, 2
	bgt	0, .Ll3s_BB21_8
# %bb.7:                                # %vector.body
                                        #   in Loop: Header=BB21_4 Depth=1
	fmr 3, 0
.Ll3s_BB21_8:                               # %vector.body
                                        #   in Loop: Header=BB21_4 Depth=1
	lfs 0, 4(6)
	fcmpu 0, 0, 1
	stfsu 3, 8(8)
	fmr 3, 1
	bge 0, .Ll3s_BB21_10
# %bb.9:                                # %vector.body
                                        #   in Loop: Header=BB21_4 Depth=1
	fcmpu 0, 3, 2
	fmr 0, 2
	bgt	0, .Ll3s_BB21_3
	b .Ll3s_BB21_11
.Ll3s_BB21_10:                              # %vector.body
                                        #   in Loop: Header=BB21_4 Depth=1
	fmr 3, 0
	fcmpu 0, 3, 2
	fmr 0, 2
	bgt	0, .Ll3s_BB21_3
.Ll3s_BB21_11:                              # %vector.body
                                        #   in Loop: Header=BB21_4 Depth=1
	fmr 0, 3
	b .Ll3s_BB21_3
.Ll3s_BB21_12:                              # %middle.block
	cmplw	7, 5
	beqlr	0
.Ll3s_BB21_13:                              # %for.body.preheader19
	slwi 6, 5, 2
	sub	5, 7, 5
	addi 6, 6, -4
	add 4, 4, 6
	add 3, 3, 6
	b .Ll3s_BB21_15
.Ll3s_BB21_14:                              # %for.body
                                        #   in Loop: Header=BB21_15 Depth=1
	addi 5, 5, -1
	stfsu 0, 4(3)
	cmplwi	5, 0
	beqlr	0
.Ll3s_BB21_15:                              # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lfsu 0, 4(4)
	fmr 3, 1
	fcmpu 0, 0, 1
	bge 0, .Ll3s_BB21_17
# %bb.16:                               # %for.body
                                        #   in Loop: Header=BB21_15 Depth=1
	fcmpu 0, 3, 2
	fmr 0, 2
	bgt	0, .Ll3s_BB21_14
	b .Ll3s_BB21_18
.Ll3s_BB21_17:                              # %for.body
                                        #   in Loop: Header=BB21_15 Depth=1
	fmr 3, 0
	fcmpu 0, 3, 2
	fmr 0, 2
	bgt	0, .Ll3s_BB21_14
.Ll3s_BB21_18:                              # %for.body
                                        #   in Loop: Header=BB21_15 Depth=1
	fmr 0, 3
	b .Ll3s_BB21_14
.Ll3s_func_end21:
	.size	l3s_k_clamp_f32_r, .Ll3s_func_end21-.Ll3s_func_begin21
                                        # -- End function
	.section	.text.l3s_k_clamp_i16_r,"ax",@progbits
	.globl	l3s_k_clamp_i16_r               # -- Begin function l3s_k_clamp_i16_r
	.p2align	2
	.type	l3s_k_clamp_i16_r,@function
l3s_k_clamp_i16_r:                      # @l3s_k_clamp_i16_r
.Ll3s_func_begin22:
# %bb.0:                                # %entry
	cmpwi	5, 1
	bltlr	0
# %bb.1:                                # %for.body.preheader
	li 6, 0
	beq	0, .Ll3s_BB22_13
# %bb.2:                                # %vector.ph
	rlwinm 6, 5, 0, 1, 30
	addi 7, 4, -4
	addi 8, 3, -4
	mr	9, 6
	b .Ll3s_BB22_4
.Ll3s_BB22_3:                               # %vector.body
                                        #   in Loop: Header=BB22_4 Depth=1
	addi 9, 9, -2
	sth 11, 2(8)
	cmplwi	9, 0
	beq	0, .Ll3s_BB22_12
.Ll3s_BB22_4:                               # %vector.body
                                        # =>This Inner Loop Header: Depth=1
	lhau 10, 4(7)
	cmpwi	10, -1000
	ble 0, .Ll3s_BB22_8
# %bb.5:                                # %vector.body
                                        #   in Loop: Header=BB22_4 Depth=1
	cmpwi	10, 1000
	bge 0, .Ll3s_BB22_9
.Ll3s_BB22_6:                               # %vector.body
                                        #   in Loop: Header=BB22_4 Depth=1
	lha 11, 2(7)
	cmpwi	11, -1000
	sthu 10, 4(8)
	ble 0, .Ll3s_BB22_10
.Ll3s_BB22_7:                               # %vector.body
                                        #   in Loop: Header=BB22_4 Depth=1
	cmpwi	11, 1000
	blt	0, .Ll3s_BB22_3
	b .Ll3s_BB22_11
.Ll3s_BB22_8:                               # %vector.body
                                        #   in Loop: Header=BB22_4 Depth=1
	li 10, -1000
	cmpwi	10, 1000
	blt	0, .Ll3s_BB22_6
.Ll3s_BB22_9:                               # %vector.body
                                        #   in Loop: Header=BB22_4 Depth=1
	li 10, 1000
	lha 11, 2(7)
	cmpwi	11, -1000
	sthu 10, 4(8)
	bgt	0, .Ll3s_BB22_7
.Ll3s_BB22_10:                              # %vector.body
                                        #   in Loop: Header=BB22_4 Depth=1
	li 11, -1000
	cmpwi	11, 1000
	blt	0, .Ll3s_BB22_3
.Ll3s_BB22_11:                              # %vector.body
                                        #   in Loop: Header=BB22_4 Depth=1
	li 11, 1000
	b .Ll3s_BB22_3
.Ll3s_BB22_12:                              # %middle.block
	cmplw	5, 6
	beqlr	0
.Ll3s_BB22_13:                              # %for.body.preheader20
	slwi 7, 6, 1
	sub	5, 5, 6
	addi 6, 7, -2
	add 4, 4, 6
	add 3, 3, 6
	b .Ll3s_BB22_15
.Ll3s_BB22_14:                              # %for.body
                                        #   in Loop: Header=BB22_15 Depth=1
	addi 5, 5, -1
	sthu 6, 2(3)
	cmplwi	5, 0
	beqlr	0
.Ll3s_BB22_15:                              # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lhau 6, 2(4)
	cmpwi	6, -1000
	ble 0, .Ll3s_BB22_17
# %bb.16:                               # %for.body
                                        #   in Loop: Header=BB22_15 Depth=1
	cmpwi	6, 1000
	blt	0, .Ll3s_BB22_14
	b .Ll3s_BB22_18
.Ll3s_BB22_17:                              # %for.body
                                        #   in Loop: Header=BB22_15 Depth=1
	li 6, -1000
	cmpwi	6, 1000
	blt	0, .Ll3s_BB22_14
.Ll3s_BB22_18:                              # %for.body
                                        #   in Loop: Header=BB22_15 Depth=1
	li 6, 1000
	b .Ll3s_BB22_14
.Ll3s_func_end22:
	.size	l3s_k_clamp_i16_r, .Ll3s_func_end22-.Ll3s_func_begin22
                                        # -- End function
	.section	.rodata.cst4,"aM",@progbits,4
	.p2align	2, 0x0                          # -- Begin function l3s_k_abs_f32_r
.Ll3s_CPI23_0:
	.long	0x00000000                      # float 0
	.section	.text.l3s_k_abs_f32_r,"ax",@progbits
	.globl	l3s_k_abs_f32_r
	.p2align	2
	.type	l3s_k_abs_f32_r,@function
l3s_k_abs_f32_r:                        # @l3s_k_abs_f32_r
.Ll3s_func_begin23:
# %bb.0:                                # %entry
	cmpwi	5, 1
	bltlr	0
# %bb.1:                                # %for.body.preheader
	li 6, 0
	beq	0, .Ll3s_BB23_9
# %bb.2:                                # %vector.ph
	lis 9, .Ll3s_CPI23_0@ha
	rlwinm 6, 5, 0, 1, 30
	lfs 0, .Ll3s_CPI23_0@l(9)
	addi 7, 4, -8
	addi 8, 3, -8
	mr	9, 6
	b .Ll3s_BB23_4
.Ll3s_BB23_3:                               # %vector.body
                                        #   in Loop: Header=BB23_4 Depth=1
	addi 9, 9, -2
	stfs 1, 4(8)
	cmplwi	9, 0
	beq	0, .Ll3s_BB23_8
.Ll3s_BB23_4:                               # %vector.body
                                        # =>This Inner Loop Header: Depth=1
	lfsu 2, 8(7)
	fcmpu 0, 2, 0
	blt	0, .Ll3s_BB23_6
# %bb.5:                                # %vector.body
                                        #   in Loop: Header=BB23_4 Depth=1
	lfs 1, 4(7)
	fcmpu 0, 1, 0
	stfsu 2, 8(8)
	bge 0, .Ll3s_BB23_3
	b .Ll3s_BB23_7
.Ll3s_BB23_6:                               #   in Loop: Header=BB23_4 Depth=1
	fneg 2, 2
	lfs 1, 4(7)
	fcmpu 0, 1, 0
	stfsu 2, 8(8)
	bge 0, .Ll3s_BB23_3
.Ll3s_BB23_7:                               #   in Loop: Header=BB23_4 Depth=1
	fneg 1, 1
	b .Ll3s_BB23_3
.Ll3s_BB23_8:                               # %middle.block
	cmplw	5, 6
	beqlr	0
.Ll3s_BB23_9:                               # %for.body.preheader10
	lis 7, .Ll3s_CPI23_0@ha
	sub	5, 5, 6
	lfs 0, .Ll3s_CPI23_0@l(7)
	slwi 6, 6, 2
	addi 6, 6, -4
	add 4, 4, 6
	add 3, 3, 6
	b .Ll3s_BB23_11
.Ll3s_BB23_10:                              # %for.body
                                        #   in Loop: Header=BB23_11 Depth=1
	addi 5, 5, -1
	stfsu 1, 4(3)
	cmplwi	5, 0
	beqlr	0
.Ll3s_BB23_11:                              # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lfsu 1, 4(4)
	fcmpu 0, 1, 0
	bge 0, .Ll3s_BB23_10
# %bb.12:                               #   in Loop: Header=BB23_11 Depth=1
	fneg 1, 1
	b .Ll3s_BB23_10
.Ll3s_func_end23:
	.size	l3s_k_abs_f32_r, .Ll3s_func_end23-.Ll3s_func_begin23
                                        # -- End function
	.section	.text.l3s_k_abs_i32_r,"ax",@progbits
	.globl	l3s_k_abs_i32_r                 # -- Begin function l3s_k_abs_i32_r
	.p2align	2
	.type	l3s_k_abs_i32_r,@function
l3s_k_abs_i32_r:                        # @l3s_k_abs_i32_r
.Ll3s_func_begin24:
# %bb.0:                                # %entry
	cmpwi	5, 1
	bltlr	0
# %bb.1:                                # %for.body.preheader
	li 6, 0
	beq	0, .Ll3s_BB24_5
# %bb.2:                                # %vector.ph
	rlwinm 6, 5, 0, 1, 30
	addi 7, 4, -8
	addi 8, 3, -8
	mr	9, 6
.Ll3s_BB24_3:                               # %vector.body
                                        # =>This Inner Loop Header: Depth=1
	lwzu 10, 8(7)
	addi 9, 9, -2
	cmplwi	9, 0
	lwz 12, 4(7)
	srawi 11, 10, 31
	xor 10, 10, 11
	sub	10, 10, 11
	srawi 11, 12, 31
	stwu 10, 8(8)
	xor 10, 12, 11
	sub	10, 10, 11
	stw 10, 4(8)
	bne	0, .Ll3s_BB24_3
# %bb.4:                                # %middle.block
	cmplw	5, 6
	beqlr	0
.Ll3s_BB24_5:                               # %for.body.preheader10
	slwi 7, 6, 2
	sub	5, 5, 6
	addi 6, 7, -4
	add 4, 4, 6
	add 3, 3, 6
.Ll3s_BB24_6:                               # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lwzu 6, 4(4)
	addi 5, 5, -1
	cmplwi	5, 0
	srawi 7, 6, 31
	xor 6, 6, 7
	sub	6, 6, 7
	stwu 6, 4(3)
	bne	0, .Ll3s_BB24_6
	blr
.Ll3s_func_end24:
	.size	l3s_k_abs_i32_r, .Ll3s_func_end24-.Ll3s_func_begin24
                                        # -- End function
	.section	.text.l3s_k_f2i_r,"ax",@progbits
	.globl	l3s_k_f2i_r                     # -- Begin function l3s_k_f2i_r
	.p2align	2
	.type	l3s_k_f2i_r,@function
l3s_k_f2i_r:                            # @l3s_k_f2i_r
.Ll3s_func_begin25:
# %bb.0:                                # %entry
	cmpwi	5, 1
	bltlr	0
# %bb.1:                                # %for.body.preheader
	stwu 1, -32(1)
	li 6, 0
	beq	0, .Ll3s_BB25_5
# %bb.2:                                # %vector.ph
	rlwinm 6, 5, 0, 1, 30
	addi 7, 4, -8
	addi 8, 3, -8
	addi 9, 1, 28
	addi 10, 1, 24
	mr	11, 6
.Ll3s_BB25_3:                               # %vector.body
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
	bne	0, .Ll3s_BB25_3
# %bb.4:                                # %middle.block
	cmplw	5, 6
	beq	0, .Ll3s_BB25_7
.Ll3s_BB25_5:                               # %for.body.preheader7
	slwi 7, 6, 2
	sub	5, 5, 6
	addi 7, 7, -4
	addi 6, 1, 20
	add 4, 4, 7
	add 3, 3, 7
.Ll3s_BB25_6:                               # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lfsu 0, 4(4)
	addi 5, 5, -1
	cmplwi	5, 0
	fctiwz 0, 0
	stfiwx 0, 0, 6
	lwz 7, 20(1)
	stwu 7, 4(3)
	bne	0, .Ll3s_BB25_6
.Ll3s_BB25_7:
	addi 1, 1, 32
	blr
.Ll3s_func_end25:
	.size	l3s_k_f2i_r, .Ll3s_func_end25-.Ll3s_func_begin25
                                        # -- End function
	.section	.text.l3s_k_i2f_r,"ax",@progbits
	.globl	l3s_k_i2f_r                     # -- Begin function l3s_k_i2f_r
	.p2align	2
	.type	l3s_k_i2f_r,@function
l3s_k_i2f_r:                            # @l3s_k_i2f_r
.Ll3s_func_begin26:
# %bb.0:                                # %entry
	cmpwi	5, 1
	bltlr	0
# %bb.1:                                # %for.body.preheader
	stwu 1, -48(1)
	li 6, 0
	beq	0, .Ll3s_BB26_5
# %bb.2:                                # %vector.ph
	rlwinm 6, 5, 0, 1, 30
	addi 9, 4, -8
	addi 7, 3, -8
	mr	8, 6
.Ll3s_BB26_3:                               # %vector.body
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
	bne	0, .Ll3s_BB26_3
# %bb.4:                                # %middle.block
	cmplw	5, 6
	beq	0, .Ll3s_BB26_7
.Ll3s_BB26_5:                               # %for.body.preheader7
	slwi 7, 6, 2
	sub	5, 5, 6
	addi 6, 7, -4
	add 4, 4, 6
	add 3, 3, 6
.Ll3s_BB26_6:                               # %for.body
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
	bne	0, .Ll3s_BB26_6
.Ll3s_BB26_7:
	addi 1, 1, 48
	blr
.Ll3s_func_end26:
	.size	l3s_k_i2f_r, .Ll3s_func_end26-.Ll3s_func_begin26
                                        # -- End function
	.section	.rodata.cst4,"aM",@progbits,4
	.p2align	2, 0x0                          # -- Begin function l3s_k_u8_to_f32_r
.Ll3s_CPI27_0:
	.long	0x3b808081                      # float 0.00392156886
	.section	.text.l3s_k_u8_to_f32_r,"ax",@progbits
	.globl	l3s_k_u8_to_f32_r
	.p2align	2
	.type	l3s_k_u8_to_f32_r,@function
l3s_k_u8_to_f32_r:                      # @l3s_k_u8_to_f32_r
.Ll3s_func_begin27:
# %bb.0:                                # %entry
	cmpwi	5, 1
	bltlr	0
# %bb.1:                                # %for.body.preheader
	stwu 1, -48(1)
	li 6, 0
	beq	0, .Ll3s_BB27_5
# %bb.2:                                # %vector.ph
	lis 9, .Ll3s_CPI27_0@ha
	rlwinm 6, 5, 0, 1, 30
	lfs 0, .Ll3s_CPI27_0@l(9)
	addi 7, 4, -2
	addi 8, 3, -8
	mr	9, 6
.Ll3s_BB27_3:                               # %vector.body
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
	bne	0, .Ll3s_BB27_3
# %bb.4:                                # %middle.block
	cmplw	5, 6
	beq	0, .Ll3s_BB27_7
.Ll3s_BB27_5:                               # %for.body.preheader7
	sub	5, 5, 6
	slwi 7, 6, 2
	add 4, 6, 4
	lis 6, .Ll3s_CPI27_0@ha
	lfs 0, .Ll3s_CPI27_0@l(6)
	add 7, 7, 3
	addi 3, 4, -1
	addi 4, 7, -4
.Ll3s_BB27_6:                               # %for.body
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
	bne	0, .Ll3s_BB27_6
.Ll3s_BB27_7:
	addi 1, 1, 48
	blr
.Ll3s_func_end27:
	.size	l3s_k_u8_to_f32_r, .Ll3s_func_end27-.Ll3s_func_begin27
                                        # -- End function
	.section	.text.l3s_k_mat4_vec4_r,"ax",@progbits
	.globl	l3s_k_mat4_vec4_r               # -- Begin function l3s_k_mat4_vec4_r
	.p2align	2
	.type	l3s_k_mat4_vec4_r,@function
l3s_k_mat4_vec4_r:                      # @l3s_k_mat4_vec4_r
.Ll3s_func_begin28:
# %bb.0:                                # %entry
	cmpwi	6, 1
	bltlr	0
# %bb.1:                                # %for.body.lr.ph
	stwu 1, -96(1)
	stfd 30, 72(1)                          # 8-byte Folded Spill
	addi 3, 3, -16
	stfd 31, 80(1)                          # 8-byte Folded Spill
	lfs 0, 0(4)
	lfs 1, 16(4)
	lfs 2, 32(4)
	lfs 3, 48(4)
	lfs 4, 4(4)
	lfs 5, 20(4)
	lfs 6, 36(4)
	lfs 7, 52(4)
	lfs 8, 8(4)
	lfs 9, 24(4)
	lfs 10, 40(4)
	lfs 11, 56(4)
	lfs 12, 12(4)
	lfs 13, 28(4)
	lfs 31, 44(4)
	lfs 30, 60(4)
	addi 4, 5, -16
	stfd 24, 24(1)                          # 8-byte Folded Spill
	stfd 25, 32(1)                          # 8-byte Folded Spill
	stfd 26, 40(1)                          # 8-byte Folded Spill
	stfd 27, 48(1)                          # 8-byte Folded Spill
	stfd 28, 56(1)                          # 8-byte Folded Spill
	stfd 29, 64(1)                          # 8-byte Folded Spill
.Ll3s_BB28_2:                               # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lfsu 29, 16(4)
	addi 6, 6, -1
	cmplwi	6, 0
	lfs 28, 4(4)
	fmuls 27, 28, 1
	fmuls 26, 28, 5
	lfs 24, 12(4)
	fmuls 25, 28, 9
	fmuls 28, 28, 13
	fmadds 27, 0, 29, 27
	fmadds 26, 4, 29, 26
	fmadds 25, 8, 29, 25
	fmadds 29, 12, 29, 28
	lfs 28, 8(4)
	fmadds 27, 2, 28, 27
	fmadds 26, 6, 28, 26
	fmadds 27, 3, 24, 27
	fmadds 29, 31, 28, 29
	stfsu 27, 16(3)
	fmadds 27, 10, 28, 25
	fmadds 26, 7, 24, 26
	stfs 26, 4(3)
	fmadds 27, 11, 24, 27
	fmadds 29, 30, 24, 29
	stfs 27, 8(3)
	stfs 29, 12(3)
	bne	0, .Ll3s_BB28_2
# %bb.3:
	lfd 31, 80(1)                           # 8-byte Folded Reload
	lfd 30, 72(1)                           # 8-byte Folded Reload
	lfd 29, 64(1)                           # 8-byte Folded Reload
	lfd 28, 56(1)                           # 8-byte Folded Reload
	lfd 27, 48(1)                           # 8-byte Folded Reload
	lfd 26, 40(1)                           # 8-byte Folded Reload
	lfd 25, 32(1)                           # 8-byte Folded Reload
	lfd 24, 24(1)                           # 8-byte Folded Reload
	addi 1, 1, 96
	blr
.Ll3s_func_end28:
	.size	l3s_k_mat4_vec4_r, .Ll3s_func_end28-.Ll3s_func_begin28
                                        # -- End function
	.section	.text.l3s_k_copy_u32_r,"ax",@progbits
	.globl	l3s_k_copy_u32_r                # -- Begin function l3s_k_copy_u32_r
	.p2align	2
	.type	l3s_k_copy_u32_r,@function
l3s_k_copy_u32_r:                       # @l3s_k_copy_u32_r
.Ll3s_func_begin29:
# %bb.0:                                # %entry
	cmpwi	5, 1
	bltlr	0
# %bb.1:                                # %for.body.preheader
	li 6, 0
	beq	0, .Ll3s_BB29_5
# %bb.2:                                # %vector.ph
	rlwinm 6, 5, 0, 1, 30
	addi 7, 4, -8
	addi 8, 3, -8
	mr	9, 6
.Ll3s_BB29_3:                               # %vector.body
                                        # =>This Inner Loop Header: Depth=1
	lwzu 10, 8(7)
	addi 9, 9, -2
	cmplwi	9, 0
	stwu 10, 8(8)
	lwz 10, 4(7)
	stw 10, 4(8)
	bne	0, .Ll3s_BB29_3
# %bb.4:                                # %middle.block
	cmplw	5, 6
	beqlr	0
.Ll3s_BB29_5:                               # %for.body.preheader7
	slwi 7, 6, 2
	sub	5, 5, 6
	addi 6, 7, -4
	add 4, 4, 6
	add 3, 3, 6
.Ll3s_BB29_6:                               # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lwzu 6, 4(4)
	addi 5, 5, -1
	cmplwi	5, 0
	stwu 6, 4(3)
	bne	0, .Ll3s_BB29_6
	blr
.Ll3s_func_end29:
	.size	l3s_k_copy_u32_r, .Ll3s_func_end29-.Ll3s_func_begin29
                                        # -- End function
	.section	.text.l3s_k_copy_u8,"ax",@progbits
	.globl	l3s_k_copy_u8                   # -- Begin function l3s_k_copy_u8
	.p2align	2
	.type	l3s_k_copy_u8,@function
l3s_k_copy_u8:                          # @l3s_k_copy_u8
.Ll3s_func_begin30:
# %bb.0:                                # %entry
	cmpwi	5, 1
	bltlr	0
# %bb.1:                                # %for.body.preheader
	addi 4, 4, -1
	addi 3, 3, -1
	li 6, 0
.Ll3s_BB30_2:                               # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lbzu 7, 1(4)
	addi 6, 6, 1
	cmplw	5, 6
	stbu 7, 1(3)
	bne	0, .Ll3s_BB30_2
# %bb.3:                                # %for.end
	blr
.Ll3s_func_end30:
	.size	l3s_k_copy_u8, .Ll3s_func_end30-.Ll3s_func_begin30
                                        # -- End function
	.section	.text.l3s_k_set_u32,"ax",@progbits
	.globl	l3s_k_set_u32                   # -- Begin function l3s_k_set_u32
	.p2align	2
	.type	l3s_k_set_u32,@function
l3s_k_set_u32:                          # @l3s_k_set_u32
.Ll3s_func_begin31:
# %bb.0:                                # %entry
	cmpwi	5, 1
	bltlr	0
# %bb.1:                                # %for.body.preheader
	li 6, 0
	beq	0, .Ll3s_BB31_5
# %bb.2:                                # %vector.ph
	rlwinm 6, 5, 0, 1, 30
	addi 7, 3, -8
	mr	8, 6
.Ll3s_BB31_3:                               # %vector.body
                                        # =>This Inner Loop Header: Depth=1
	addi 8, 8, -2
	stwu 4, 8(7)
	cmplwi	8, 0
	stw 4, 4(7)
	bne	0, .Ll3s_BB31_3
# %bb.4:                                # %middle.block
	cmplw	5, 6
	beqlr	0
.Ll3s_BB31_5:                               # %for.body.preheader5
	slwi 7, 6, 2
	add 7, 7, 3
	sub	3, 5, 6
	addi 5, 7, -4
.Ll3s_BB31_6:                               # %for.body
                                        # =>This Inner Loop Header: Depth=1
	addi 3, 3, -1
	stwu 4, 4(5)
	cmplwi	3, 0
	bne	0, .Ll3s_BB31_6
	blr
.Ll3s_func_end31:
	.size	l3s_k_set_u32, .Ll3s_func_end31-.Ll3s_func_begin31
                                        # -- End function
	.section	.text.l3s_k_set_f32,"ax",@progbits
	.globl	l3s_k_set_f32                   # -- Begin function l3s_k_set_f32
	.p2align	2
	.type	l3s_k_set_f32,@function
l3s_k_set_f32:                          # @l3s_k_set_f32
.Ll3s_func_begin32:
# %bb.0:                                # %entry
	cmpwi	5, 1
	bltlr	0
# %bb.1:                                # %for.body.preheader
	li 4, 0
	beq	0, .Ll3s_BB32_5
# %bb.2:                                # %vector.ph
	rlwinm 4, 5, 0, 1, 30
	addi 6, 3, -8
	mr	7, 4
.Ll3s_BB32_3:                               # %vector.body
                                        # =>This Inner Loop Header: Depth=1
	addi 7, 7, -2
	stfsu 1, 8(6)
	cmplwi	7, 0
	stfs 1, 4(6)
	bne	0, .Ll3s_BB32_3
# %bb.4:                                # %middle.block
	cmplw	5, 4
	beqlr	0
.Ll3s_BB32_5:                               # %for.body.preheader5
	slwi 6, 4, 2
	add 6, 6, 3
	sub	3, 5, 4
	addi 4, 6, -4
.Ll3s_BB32_6:                               # %for.body
                                        # =>This Inner Loop Header: Depth=1
	addi 3, 3, -1
	stfsu 1, 4(4)
	cmplwi	3, 0
	bne	0, .Ll3s_BB32_6
	blr
.Ll3s_func_end32:
	.size	l3s_k_set_f32, .Ll3s_func_end32-.Ll3s_func_begin32
                                        # -- End function
	.section	.text.l3s_k_zero_u8,"ax",@progbits
	.globl	l3s_k_zero_u8                   # -- Begin function l3s_k_zero_u8
	.p2align	2
	.type	l3s_k_zero_u8,@function
l3s_k_zero_u8:                          # @l3s_k_zero_u8
.Ll3s_func_begin33:
# %bb.0:                                # %entry
	cmpwi	4, 1
	bltlr	0
# %bb.1:                                # %for.body.preheader
	li 5, 0
	beq	0, .Ll3s_BB33_5
# %bb.2:                                # %vector.ph
	rlwinm 5, 4, 0, 1, 30
	addi 6, 3, -2
	li 7, 0
	mr	8, 5
.Ll3s_BB33_3:                               # %vector.body
                                        # =>This Inner Loop Header: Depth=1
	addi 8, 8, -2
	stbu 7, 2(6)
	cmplwi	8, 0
	stb 7, 1(6)
	bne	0, .Ll3s_BB33_3
# %bb.4:                                # %middle.block
	cmplw	4, 5
	beqlr	0
.Ll3s_BB33_5:                               # %for.body.preheader5
	add 3, 5, 3
	sub	4, 4, 5
	addi 3, 3, -1
	li 5, 0
.Ll3s_BB33_6:                               # %for.body
                                        # =>This Inner Loop Header: Depth=1
	addi 4, 4, -1
	stbu 5, 1(3)
	cmplwi	4, 0
	bne	0, .Ll3s_BB33_6
	blr
.Ll3s_func_end33:
	.size	l3s_k_zero_u8, .Ll3s_func_end33-.Ll3s_func_begin33
                                        # -- End function
	.section	.text.l3s_k_stride2_f32_r,"ax",@progbits
	.globl	l3s_k_stride2_f32_r             # -- Begin function l3s_k_stride2_f32_r
	.p2align	2
	.type	l3s_k_stride2_f32_r,@function
l3s_k_stride2_f32_r:                    # @l3s_k_stride2_f32_r
.Ll3s_func_begin34:
# %bb.0:                                # %entry
	cmpwi	5, 1
	bltlr	0
# %bb.1:                                # %for.body.preheader
	li 6, 0
	beq	0, .Ll3s_BB34_5
# %bb.2:                                # %vector.ph
	rlwinm 6, 5, 0, 1, 30
	addi 7, 4, -16
	addi 8, 3, -8
	mr	9, 6
.Ll3s_BB34_3:                               # %vector.body
                                        # =>This Inner Loop Header: Depth=1
	lfsu 0, 16(7)
	addi 9, 9, -2
	cmplwi	9, 0
	lfs 1, 4(7)
	fadds 0, 0, 1
	lfs 1, 12(7)
	stfsu 0, 8(8)
	lfs 0, 8(7)
	fadds 0, 0, 1
	stfs 0, 4(8)
	bne	0, .Ll3s_BB34_3
# %bb.4:                                # %middle.block
	cmplw	5, 6
	beqlr	0
.Ll3s_BB34_5:                               # %for.body.preheader12
	sub	5, 5, 6
	slwi 7, 6, 2
	slwi 6, 6, 3
	add 4, 6, 4
	add 6, 7, 3
	addi 3, 4, -8
	addi 4, 6, -4
.Ll3s_BB34_6:                               # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lfsu 0, 8(3)
	addi 5, 5, -1
	cmplwi	5, 0
	lfs 1, 4(3)
	fadds 0, 0, 1
	stfsu 0, 4(4)
	bne	0, .Ll3s_BB34_6
	blr
.Ll3s_func_end34:
	.size	l3s_k_stride2_f32_r, .Ll3s_func_end34-.Ll3s_func_begin34
                                        # -- End function
	.section	.text.l3s_k_gather_f32_r,"ax",@progbits
	.globl	l3s_k_gather_f32_r              # -- Begin function l3s_k_gather_f32_r
	.p2align	2
	.type	l3s_k_gather_f32_r,@function
l3s_k_gather_f32_r:                     # @l3s_k_gather_f32_r
.Ll3s_func_begin35:
# %bb.0:                                # %entry
	cmpwi	6, 1
	bltlr	0
# %bb.1:                                # %for.body.preheader
	li 7, 0
	beq	0, .Ll3s_BB35_5
# %bb.2:                                # %vector.ph
	rlwinm 7, 6, 0, 1, 30
	addi 8, 5, -8
	addi 9, 3, -8
	mr	10, 7
.Ll3s_BB35_3:                               # %vector.body
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
	bne	0, .Ll3s_BB35_3
# %bb.4:                                # %middle.block
	cmplw	6, 7
	beqlr	0
.Ll3s_BB35_5:                               # %for.body.preheader8
	slwi 8, 7, 2
	sub	6, 6, 7
	addi 7, 8, -4
	add 5, 5, 7
	add 3, 3, 7
.Ll3s_BB35_6:                               # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lwzu 7, 4(5)
	addi 6, 6, -1
	cmplwi	6, 0
	slwi 7, 7, 2
	lwzx 7, 4, 7
	stwu 7, 4(3)
	bne	0, .Ll3s_BB35_6
	blr
.Ll3s_func_end35:
	.size	l3s_k_gather_f32_r, .Ll3s_func_end35-.Ll3s_func_begin35
                                        # -- End function
	.section	.text.l3s_k_offset1_f32_r,"ax",@progbits
	.globl	l3s_k_offset1_f32_r             # -- Begin function l3s_k_offset1_f32_r
	.p2align	2
	.type	l3s_k_offset1_f32_r,@function
l3s_k_offset1_f32_r:                    # @l3s_k_offset1_f32_r
.Ll3s_func_begin36:
# %bb.0:                                # %entry
	cmpwi	5, 1
	bltlr	0
# %bb.1:                                # %for.body.preheader
	lfs 0, 0(4)
	li 6, 0
	beq	0, .Ll3s_BB36_5
# %bb.2:                                # %vector.ph
	rlwinm 6, 5, 0, 1, 30
	addi 7, 4, -4
	addi 8, 3, -8
	mr	9, 6
.Ll3s_BB36_3:                               # %vector.body
                                        # =>This Inner Loop Header: Depth=1
	lfsu 1, 8(7)
	addi 9, 9, -2
	cmplwi	9, 0
	fadds 0, 0, 1
	stfsu 0, 8(8)
	lfs 0, 4(7)
	fadds 1, 1, 0
	stfs 1, 4(8)
	bne	0, .Ll3s_BB36_3
# %bb.4:                                # %middle.block
	cmplw	5, 6
	beqlr	0
.Ll3s_BB36_5:                               # %for.body.preheader11
	slwi 7, 6, 2
	sub	5, 5, 6
	add 6, 7, 3
	add 3, 4, 7
	addi 4, 6, -4
.Ll3s_BB36_6:                               # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lfsu 1, 4(3)
	addi 5, 5, -1
	cmplwi	5, 0
	fadds 0, 0, 1
	stfsu 0, 4(4)
	fmr 0, 1
	bne	0, .Ll3s_BB36_6
	blr
.Ll3s_func_end36:
	.size	l3s_k_offset1_f32_r, .Ll3s_func_end36-.Ll3s_func_begin36
                                        # -- End function
	.section	.rodata.cst4,"aM",@progbits,4
	.p2align	2, 0x0                          # -- Begin function l3s_k_fir3_f32_r
.Ll3s_CPI37_0:
	.long	0x3f000000                      # float 0.5
.Ll3s_CPI37_1:
	.long	0x3e800000                      # float 0.25
	.section	.text.l3s_k_fir3_f32_r,"ax",@progbits
	.globl	l3s_k_fir3_f32_r
	.p2align	2
	.type	l3s_k_fir3_f32_r,@function
l3s_k_fir3_f32_r:                       # @l3s_k_fir3_f32_r
.Ll3s_func_begin37:
# %bb.0:                                # %entry
	cmpwi	5, 1
	bltlr	0
# %bb.1:                                # %for.body.preheader
	lfs 0, 0(4)
	li 6, 0
	beq	0, .Ll3s_BB37_5
# %bb.2:                                # %vector.ph
	lis 6, .Ll3s_CPI37_0@ha
	lis 9, .Ll3s_CPI37_1@ha
	lfs 1, .Ll3s_CPI37_0@l(6)
	rlwinm 6, 5, 0, 1, 30
	addi 7, 4, -4
	lfs 2, .Ll3s_CPI37_1@l(9)
	addi 8, 3, -8
	mr	9, 6
.Ll3s_BB37_3:                               # %vector.body
                                        # =>This Inner Loop Header: Depth=1
	lfsu 3, 8(7)
	addi 9, 9, -2
	cmplwi	9, 0
	fmuls 4, 3, 1
	fmadds 4, 0, 2, 4
	lfs 0, 4(7)
	lfs 5, 8(7)
	fmadds 4, 0, 2, 4
	stfsu 4, 8(8)
	fmuls 4, 0, 1
	fmadds 3, 3, 2, 4
	fmadds 3, 5, 2, 3
	stfs 3, 4(8)
	bne	0, .Ll3s_BB37_3
# %bb.4:                                # %middle.block
	cmplw	5, 6
	beqlr	0
.Ll3s_BB37_5:                               # %for.body.preheader15
	slwi 7, 6, 2
	sub	5, 5, 6
	lis 6, .Ll3s_CPI37_0@ha
	add 8, 7, 3
	lfs 1, .Ll3s_CPI37_0@l(6)
	lis 6, .Ll3s_CPI37_1@ha
	add 3, 4, 7
	lfs 2, .Ll3s_CPI37_1@l(6)
	addi 4, 8, -4
.Ll3s_BB37_6:                               # %for.body
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
	bne	0, .Ll3s_BB37_6
	blr
.Ll3s_func_end37:
	.size	l3s_k_fir3_f32_r, .Ll3s_func_end37-.Ll3s_func_begin37
                                        # -- End function
	.section	.text.l3s_k_prefix_f32,"ax",@progbits
	.globl	l3s_k_prefix_f32                # -- Begin function l3s_k_prefix_f32
	.p2align	2
	.type	l3s_k_prefix_f32,@function
l3s_k_prefix_f32:                       # @l3s_k_prefix_f32
.Ll3s_func_begin38:
# %bb.0:                                # %entry
	cmpwi	4, 2
	bltlr	0
# %bb.1:                                # %for.body.preheader
	lfs 0, 0(3)
	addi 4, 4, -1
.Ll3s_BB38_2:                               # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lfsu 1, 4(3)
	addi 4, 4, -1
	cmplwi	4, 0
	fadds 0, 0, 1
	stfs 0, 0(3)
	bne	0, .Ll3s_BB38_2
# %bb.3:                                # %for.end
	blr
.Ll3s_func_end38:
	.size	l3s_k_prefix_f32, .Ll3s_func_end38-.Ll3s_func_begin38
                                        # -- End function
	.section	.text.l3s_k_g_add_f32,"ax",@progbits
	.globl	l3s_k_g_add_f32                 # -- Begin function l3s_k_g_add_f32
	.p2align	2
	.type	l3s_k_g_add_f32,@function
l3s_k_g_add_f32:                        # @l3s_k_g_add_f32
.Ll3s_func_begin39:
# %bb.0:                                # %entry
	lis 3, l3s_g_c@ha
	lis 4, l3s_g_b@ha
	lis 5, l3s_g_a@ha
	la 3, l3s_g_c@l(3)
	la 4, l3s_g_b@l(4)
	la 5, l3s_g_a@l(5)
	addi 3, 3, -8
	addi 4, 4, -8
	addi 5, 5, -8
	li 6, 1024
.Ll3s_BB39_1:                               # %vector.body
                                        # =>This Inner Loop Header: Depth=1
	lfsu 0, 8(5)
	addi 6, 6, -2
	cmplwi	6, 0
	lfsu 1, 8(4)
	fadds 0, 0, 1
	stfsu 0, 8(3)
	lfs 0, 4(5)
	lfs 1, 4(4)
	fadds 0, 0, 1
	stfs 0, 4(3)
	bne	0, .Ll3s_BB39_1
# %bb.2:                                # %for.end
	blr
.Ll3s_func_end39:
	.size	l3s_k_g_add_f32, .Ll3s_func_end39-.Ll3s_func_begin39
                                        # -- End function
	.section	.text.l3s_k_g_saxpy,"ax",@progbits
	.globl	l3s_k_g_saxpy                   # -- Begin function l3s_k_g_saxpy
	.p2align	2
	.type	l3s_k_g_saxpy,@function
l3s_k_g_saxpy:                          # @l3s_k_g_saxpy
.Ll3s_func_begin40:
# %bb.0:                                # %entry
	lis 3, l3s_g_c@ha
	lis 4, l3s_g_b@ha
	lis 5, l3s_g_a@ha
	la 3, l3s_g_c@l(3)
	la 4, l3s_g_b@l(4)
	la 5, l3s_g_a@l(5)
	addi 3, 3, -8
	addi 4, 4, -8
	addi 5, 5, -8
	li 6, 1024
.Ll3s_BB40_1:                               # %vector.body
                                        # =>This Inner Loop Header: Depth=1
	lfsu 0, 8(5)
	addi 6, 6, -2
	cmplwi	6, 0
	lfsu 2, 8(4)
	fmadds 0, 1, 0, 2
	stfsu 0, 8(3)
	lfs 0, 4(5)
	lfs 2, 4(4)
	fmadds 0, 1, 0, 2
	stfs 0, 4(3)
	bne	0, .Ll3s_BB40_1
# %bb.2:                                # %for.end
	blr
.Ll3s_func_end40:
	.size	l3s_k_g_saxpy, .Ll3s_func_end40-.Ll3s_func_begin40
                                        # -- End function
	.section	.text.l3s_k_g_add_i32,"ax",@progbits
	.globl	l3s_k_g_add_i32                 # -- Begin function l3s_k_g_add_i32
	.p2align	2
	.type	l3s_k_g_add_i32,@function
l3s_k_g_add_i32:                        # @l3s_k_g_add_i32
.Ll3s_func_begin41:
# %bb.0:                                # %entry
	lis 3, l3s_g_ic@ha
	lis 4, l3s_g_ib@ha
	lis 5, l3s_g_ia@ha
	la 3, l3s_g_ic@l(3)
	la 4, l3s_g_ib@l(4)
	la 5, l3s_g_ia@l(5)
	addi 3, 3, -8
	addi 4, 4, -8
	addi 5, 5, -8
	li 6, 1024
.Ll3s_BB41_1:                               # %vector.body
                                        # =>This Inner Loop Header: Depth=1
	lwzu 7, 8(5)
	addi 6, 6, -2
	cmplwi	6, 0
	lwzu 8, 8(4)
	lwz 9, 4(5)
	add 7, 8, 7
	lwz 10, 4(4)
	stwu 7, 8(3)
	add 7, 10, 9
	stw 7, 4(3)
	bne	0, .Ll3s_BB41_1
# %bb.2:                                # %for.end
	blr
.Ll3s_func_end41:
	.size	l3s_k_g_add_i32, .Ll3s_func_end41-.Ll3s_func_begin41
                                        # -- End function
	.section	.rodata.cst4,"aM",@progbits,4
	.p2align	2, 0x0                          # -- Begin function l3s_k_g_dot
.Ll3s_CPI42_0:
	.long	0x00000000                      # float 0
	.section	.text.l3s_k_g_dot,"ax",@progbits
	.globl	l3s_k_g_dot
	.p2align	2
	.type	l3s_k_g_dot,@function
l3s_k_g_dot:                            # @l3s_k_g_dot
.Ll3s_func_begin42:
# %bb.0:                                # %entry
	lis 6, .Ll3s_CPI42_0@ha
	lis 3, l3s_g_b@ha
	lfs 1, .Ll3s_CPI42_0@l(6)
	lis 4, l3s_g_a@ha
	la 3, l3s_g_b@l(3)
	la 4, l3s_g_a@l(4)
	addi 3, 3, -4
	addi 4, 4, -4
	li 5, 0
.Ll3s_BB42_1:                               # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lfsu 0, 4(3)
	addi 5, 5, 4
	cmplwi	5, 4096
	lfsu 2, 4(4)
	fmadds 1, 2, 0, 1
	bne	0, .Ll3s_BB42_1
# %bb.2:                                # %for.end
	blr
.Ll3s_func_end42:
	.size	l3s_k_g_dot, .Ll3s_func_end42-.Ll3s_func_begin42
                                        # -- End function
	.section	.text.l3s_k_add16_f32_r,"ax",@progbits
	.globl	l3s_k_add16_f32_r               # -- Begin function l3s_k_add16_f32_r
	.p2align	2
	.type	l3s_k_add16_f32_r,@function
l3s_k_add16_f32_r:                      # @l3s_k_add16_f32_r
.Ll3s_func_begin43:
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
.Ll3s_func_end43:
	.size	l3s_k_add16_f32_r, .Ll3s_func_end43-.Ll3s_func_begin43
                                        # -- End function
	.type	l3s_g_a,@object                 # @l3s_g_a
	.section	.bss.l3s_g_a,"aw",@nobits
	.globl	l3s_g_a
	.p2align	4, 0x0
l3s_g_a:
	.space	4096
	.size	l3s_g_a, 4096

	.type	l3s_g_b,@object                 # @l3s_g_b
	.section	.bss.l3s_g_b,"aw",@nobits
	.globl	l3s_g_b
	.p2align	4, 0x0
l3s_g_b:
	.space	4096
	.size	l3s_g_b, 4096

	.type	l3s_g_c,@object                 # @l3s_g_c
	.section	.bss.l3s_g_c,"aw",@nobits
	.globl	l3s_g_c
	.p2align	4, 0x0
l3s_g_c:
	.space	4096
	.size	l3s_g_c, 4096

	.type	l3s_g_ia,@object                # @l3s_g_ia
	.section	.bss.l3s_g_ia,"aw",@nobits
	.globl	l3s_g_ia
	.p2align	4, 0x0
l3s_g_ia:
	.space	4096
	.size	l3s_g_ia, 4096

	.type	l3s_g_ib,@object                # @l3s_g_ib
	.section	.bss.l3s_g_ib,"aw",@nobits
	.globl	l3s_g_ib
	.p2align	4, 0x0
l3s_g_ib:
	.space	4096
	.size	l3s_g_ib, 4096

	.type	l3s_g_ic,@object                # @l3s_g_ic
	.section	.bss.l3s_g_ic,"aw",@nobits
	.globl	l3s_g_ic
	.p2align	4, 0x0
l3s_g_ic:
	.space	4096
	.size	l3s_g_ic, 4096

