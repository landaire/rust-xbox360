# kernels.c: clang version 22.1.8 (git@github.com:landaire/llvm-project.git 2f714b11080e5a449de3969e3c4477cf5cbe92ab)
# flags: --target=powerpc64-unknown-xbox360 -ffreestanding -fno-builtin -fms-extensions -ffunction-sections -fdata-sections -fno-ident -fno-addrsig -fno-asynchronous-unwind-tables -g0 -O3 -ffast-math -mno-vmx128
	.section	.text.lfa_k_saxpy,"ax",@progbits
	.globl	lfa_k_saxpy                     # -- Begin function lfa_k_saxpy
	.p2align	2
	.type	lfa_k_saxpy,@function
lfa_k_saxpy:                            # @lfa_k_saxpy
.Llfa_func_begin0:
# %bb.0:                                # %entry
	cmpwi	6, 1
	bltlr	0
# %bb.1:                                # %for.body.preheader
	stwu 1, -64(1)
	cmplwi	6, 8
	li 5, 0
	std 30, 40(1)                           # 8-byte Folded Spill
	blt	0, .Llfa_BB0_7
# %bb.2:                                # %vector.memcheck
	slwi 7, 6, 2
	add 8, 4, 7
	cmplw	3, 8
	add 7, 3, 7
	bc 4, 0, .Llfa_BB0_4
# %bb.3:                                # %vector.memcheck
	cmplw	4, 7
	bc 12, 0, .Llfa_BB0_7
.Llfa_BB0_4:                                # %vector.ph
	stfs 1, 16(1)
	addi 8, 1, 16
	rlwinm 5, 6, 0, 1, 29
	li 7, 0
	li 9, 12
	lvx 2, 0, 8
	li 8, 15
	li 10, 8
	li 11, 4
	mr	12, 5
	vspltw 2, 2, 0
.Llfa_BB0_5:                                # %vector.body
                                        # =>This Inner Loop Header: Depth=1
	add 30, 4, 7
	lvx 3, 4, 7
	addi 12, 12, -4
	cmplwi	12, 0
	lvx 4, 30, 8
	add 30, 3, 7
	lvsl 5, 4, 7
	vperm 3, 3, 4, 5
	lvx 4, 3, 7
	lvx 5, 30, 8
	lvsl 0, 3, 7
	vperm 4, 4, 5, 0
	vmaddfp 3, 3, 2, 4
	lvsr 4, 3, 7
	vperm 3, 3, 3, 4
	stvewx 3, 3, 7
	addi 7, 7, 16
	stvewx 3, 30, 9
	stvewx 3, 30, 10
	stvewx 3, 30, 11
	bne	0, .Llfa_BB0_5
# %bb.6:                                # %middle.block
	cmplw	6, 5
	beq	0, .Llfa_BB0_9
.Llfa_BB0_7:                                # %for.body.preheader12
	slwi 7, 5, 2
	sub	5, 6, 5
	addi 6, 7, -4
	add 4, 4, 6
	add 3, 3, 6
.Llfa_BB0_8:                                # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lfsu 0, 4(3)
	addi 5, 5, -1
	cmplwi	5, 0
	lfsu 2, 4(4)
	fmadds 0, 2, 1, 0
	stfs 0, 0(3)
	bne	0, .Llfa_BB0_8
.Llfa_BB0_9:
	ld 30, 40(1)                            # 8-byte Folded Reload
	addi 1, 1, 64
	blr
.Llfa_func_end0:
	.size	lfa_k_saxpy, .Llfa_func_end0-.Llfa_func_begin0
                                        # -- End function
	.section	.text.lfa_k_saxpy_r,"ax",@progbits
	.globl	lfa_k_saxpy_r                   # -- Begin function lfa_k_saxpy_r
	.p2align	2
	.type	lfa_k_saxpy_r,@function
lfa_k_saxpy_r:                          # @lfa_k_saxpy_r
.Llfa_func_begin1:
# %bb.0:                                # %entry
	cmpwi	6, 1
	bltlr	0
# %bb.1:                                # %for.body.preheader
	stwu 1, -64(1)
	cmplwi	6, 4
	li 5, 0
	std 30, 40(1)                           # 8-byte Folded Spill
	blt	0, .Llfa_BB1_5
# %bb.2:                                # %vector.ph
	stfs 1, 16(1)
	addi 8, 1, 16
	rlwinm 5, 6, 0, 1, 29
	li 7, 0
	li 9, 12
	lvx 2, 0, 8
	li 8, 15
	li 10, 8
	li 11, 4
	mr	12, 5
	vspltw 2, 2, 0
.Llfa_BB1_3:                                # %vector.body
                                        # =>This Inner Loop Header: Depth=1
	add 30, 4, 7
	lvx 3, 4, 7
	addi 12, 12, -4
	cmplwi	12, 0
	lvx 4, 30, 8
	add 30, 3, 7
	lvsl 5, 4, 7
	vperm 3, 3, 4, 5
	lvx 4, 3, 7
	lvx 5, 30, 8
	lvsl 0, 3, 7
	vperm 4, 4, 5, 0
	vmaddfp 3, 3, 2, 4
	lvsr 4, 3, 7
	vperm 3, 3, 3, 4
	stvewx 3, 3, 7
	addi 7, 7, 16
	stvewx 3, 30, 9
	stvewx 3, 30, 10
	stvewx 3, 30, 11
	bne	0, .Llfa_BB1_3
# %bb.4:                                # %middle.block
	cmplw	6, 5
	beq	0, .Llfa_BB1_7
.Llfa_BB1_5:                                # %for.body.preheader11
	slwi 7, 5, 2
	sub	5, 6, 5
	addi 6, 7, -4
	add 4, 4, 6
	add 3, 3, 6
.Llfa_BB1_6:                                # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lfsu 0, 4(3)
	addi 5, 5, -1
	cmplwi	5, 0
	lfsu 2, 4(4)
	fmadds 0, 2, 1, 0
	stfs 0, 0(3)
	bne	0, .Llfa_BB1_6
.Llfa_BB1_7:
	ld 30, 40(1)                            # 8-byte Folded Reload
	addi 1, 1, 64
	blr
.Llfa_func_end1:
	.size	lfa_k_saxpy_r, .Llfa_func_end1-.Llfa_func_begin1
                                        # -- End function
	.section	.text.lfa_k_scale_inplace,"ax",@progbits
	.globl	lfa_k_scale_inplace             # -- Begin function lfa_k_scale_inplace
	.p2align	2
	.type	lfa_k_scale_inplace,@function
lfa_k_scale_inplace:                    # @lfa_k_scale_inplace
.Llfa_func_begin2:
# %bb.0:                                # %entry
	cmpwi	5, 1
	bltlr	0
# %bb.1:                                # %for.body.preheader
	stwu 1, -64(1)
	cmplwi	5, 8
	li 4, 0
	std 29, 32(1)                           # 8-byte Folded Spill
	std 30, 40(1)                           # 8-byte Folded Spill
	blt	0, .Llfa_BB2_5
# %bb.2:                                # %vector.ph
	stfs 1, 16(1)
	addi 8, 1, 16
	vspltisw 3, -1
	rlwinm 4, 5, 0, 1, 28
	addi 6, 3, 16
	li 7, 0
	lvx 2, 0, 8
	li 8, 15
	li 9, 12
	li 10, 8
	li 11, 4
	vspltw 2, 2, 0
	vslw 3, 3, 3
	mr	12, 4
.Llfa_BB2_3:                                # %vector.body
                                        # =>This Inner Loop Header: Depth=1
	add 30, 3, 7
	lvx 4, 3, 7
	add 29, 6, 7
	addi 12, 12, -8
	lvx 5, 30, 8
	cmplwi	12, 0
	lvsl 0, 3, 7
	vperm 4, 4, 5, 0
	lvx 5, 6, 7
	lvx 0, 29, 8
	lvsl 1, 6, 7
	vmaddfp 4, 4, 2, 3
	vperm 5, 5, 0, 1
	lvsr 0, 3, 7
	vperm 4, 4, 4, 0
	lvsr 0, 6, 7
	stvewx 4, 30, 9
	stvewx 4, 30, 10
	vmaddfp 5, 5, 2, 3
	stvewx 4, 30, 11
	stvewx 4, 3, 7
	vperm 4, 5, 5, 0
	stvewx 4, 6, 7
	addi 7, 7, 32
	stvewx 4, 29, 9
	stvewx 4, 29, 10
	stvewx 4, 29, 11
	bne	0, .Llfa_BB2_3
# %bb.4:                                # %middle.block
	cmplw	5, 4
	beq	0, .Llfa_BB2_7
.Llfa_BB2_5:                                # %for.body.preheader6
	slwi 6, 4, 2
	add 6, 6, 3
	sub	3, 5, 4
	addi 4, 6, -4
.Llfa_BB2_6:                                # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lfsu 0, 4(4)
	addi 3, 3, -1
	cmplwi	3, 0
	fmuls 0, 0, 1
	stfs 0, 0(4)
	bne	0, .Llfa_BB2_6
.Llfa_BB2_7:
	ld 30, 40(1)                            # 8-byte Folded Reload
	ld 29, 32(1)                            # 8-byte Folded Reload
	addi 1, 1, 64
	blr
.Llfa_func_end2:
	.size	lfa_k_scale_inplace, .Llfa_func_end2-.Llfa_func_begin2
                                        # -- End function
	.section	.text.lfa_k_add_f32_r,"ax",@progbits
	.globl	lfa_k_add_f32_r                 # -- Begin function lfa_k_add_f32_r
	.p2align	2
	.type	lfa_k_add_f32_r,@function
lfa_k_add_f32_r:                        # @lfa_k_add_f32_r
.Llfa_func_begin3:
# %bb.0:                                # %entry
	cmpwi	6, 1
	bltlr	0
# %bb.1:                                # %for.body.preheader
	stwu 1, -64(1)
	cmplwi	6, 8
	li 7, 0
	std 27, 16(1)                           # 8-byte Folded Spill
	std 28, 24(1)                           # 8-byte Folded Spill
	std 29, 32(1)                           # 8-byte Folded Spill
	std 30, 40(1)                           # 8-byte Folded Spill
	blt	0, .Llfa_BB3_5
# %bb.2:                                # %vector.ph
	rlwinm 7, 6, 0, 1, 28
	addi 8, 3, 16
	addi 9, 5, 16
	addi 10, 4, 16
	li 11, 0
	li 12, 15
	li 0, 12
	li 30, 8
	li 29, 4
	mr	28, 7
.Llfa_BB3_3:                                # %vector.body
                                        # =>This Inner Loop Header: Depth=1
	add 27, 4, 11
	lvx 3, 4, 11
	addi 28, 28, -8
	cmplwi	28, 0
	lvx 4, 27, 12
	add 27, 10, 11
	lvsl 2, 4, 11
	vperm 2, 3, 4, 2
	lvx 3, 10, 11
	lvx 4, 27, 12
	add 27, 5, 11
	lvsl 5, 10, 11
	vperm 3, 3, 4, 5
	lvx 0, 5, 11
	lvx 4, 27, 12
	add 27, 9, 11
	lvsl 5, 5, 11
	vperm 4, 0, 4, 5
	lvx 5, 9, 11
	lvx 0, 27, 12
	add 27, 3, 11
	lvsl 1, 9, 11
	vaddfp 2, 4, 2
	lvsr 4, 3, 11
	vperm 5, 5, 0, 1
	vperm 2, 2, 2, 4
	lvsr 4, 8, 11
	stvewx 2, 27, 0
	stvewx 2, 27, 30
	vaddfp 3, 5, 3
	stvewx 2, 27, 29
	add 27, 8, 11
	stvewx 2, 3, 11
	vperm 2, 3, 3, 4
	stvewx 2, 8, 11
	addi 11, 11, 32
	stvewx 2, 27, 0
	stvewx 2, 27, 30
	stvewx 2, 27, 29
	bne	0, .Llfa_BB3_3
# %bb.4:                                # %middle.block
	cmplw	6, 7
	beq	0, .Llfa_BB3_7
.Llfa_BB3_5:                                # %for.body.preheader12
	slwi 8, 7, 2
	sub	6, 6, 7
	addi 8, 8, -4
	add 4, 4, 8
	add 5, 5, 8
	add 3, 3, 8
.Llfa_BB3_6:                                # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lfsu 0, 4(4)
	addi 6, 6, -1
	cmplwi	6, 0
	lfsu 1, 4(5)
	fadds 0, 1, 0
	stfsu 0, 4(3)
	bne	0, .Llfa_BB3_6
.Llfa_BB3_7:
	ld 30, 40(1)                            # 8-byte Folded Reload
	ld 29, 32(1)                            # 8-byte Folded Reload
	ld 28, 24(1)                            # 8-byte Folded Reload
	ld 27, 16(1)                            # 8-byte Folded Reload
	addi 1, 1, 64
	blr
.Llfa_func_end3:
	.size	lfa_k_add_f32_r, .Llfa_func_end3-.Llfa_func_begin3
                                        # -- End function
	.section	.text.lfa_k_mul_f32_r,"ax",@progbits
	.globl	lfa_k_mul_f32_r                 # -- Begin function lfa_k_mul_f32_r
	.p2align	2
	.type	lfa_k_mul_f32_r,@function
lfa_k_mul_f32_r:                        # @lfa_k_mul_f32_r
.Llfa_func_begin4:
# %bb.0:                                # %entry
	cmpwi	6, 1
	bltlr	0
# %bb.1:                                # %for.body.preheader
	stwu 1, -64(1)
	cmplwi	6, 8
	li 7, 0
	std 27, 16(1)                           # 8-byte Folded Spill
	std 28, 24(1)                           # 8-byte Folded Spill
	std 29, 32(1)                           # 8-byte Folded Spill
	std 30, 40(1)                           # 8-byte Folded Spill
	blt	0, .Llfa_BB4_5
# %bb.2:                                # %vector.ph
	rlwinm 7, 6, 0, 1, 28
	vspltisw 2, -1
	addi 8, 3, 16
	addi 9, 5, 16
	addi 10, 4, 16
	li 11, 0
	li 12, 15
	li 0, 12
	li 30, 8
	vslw 2, 2, 2
	li 29, 4
	mr	28, 7
.Llfa_BB4_3:                                # %vector.body
                                        # =>This Inner Loop Header: Depth=1
	add 27, 4, 11
	lvx 4, 4, 11
	addi 28, 28, -8
	cmplwi	28, 0
	lvx 5, 27, 12
	add 27, 10, 11
	lvsl 3, 4, 11
	vperm 3, 4, 5, 3
	lvx 4, 10, 11
	lvx 5, 27, 12
	add 27, 5, 11
	lvsl 0, 10, 11
	vperm 4, 4, 5, 0
	lvx 1, 5, 11
	lvx 5, 27, 12
	add 27, 9, 11
	lvsl 0, 5, 11
	vperm 5, 1, 5, 0
	lvx 0, 9, 11
	lvx 1, 27, 12
	add 27, 3, 11
	lvsl 6, 9, 11
	vmaddfp 3, 5, 3, 2
	lvsr 5, 3, 11
	vperm 0, 0, 1, 6
	vperm 3, 3, 3, 5
	lvsr 5, 8, 11
	stvewx 3, 27, 0
	stvewx 3, 27, 30
	vmaddfp 4, 0, 4, 2
	stvewx 3, 27, 29
	add 27, 8, 11
	stvewx 3, 3, 11
	vperm 3, 4, 4, 5
	stvewx 3, 8, 11
	addi 11, 11, 32
	stvewx 3, 27, 0
	stvewx 3, 27, 30
	stvewx 3, 27, 29
	bne	0, .Llfa_BB4_3
# %bb.4:                                # %middle.block
	cmplw	6, 7
	beq	0, .Llfa_BB4_7
.Llfa_BB4_5:                                # %for.body.preheader12
	slwi 8, 7, 2
	sub	6, 6, 7
	addi 8, 8, -4
	add 4, 4, 8
	add 5, 5, 8
	add 3, 3, 8
.Llfa_BB4_6:                                # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lfsu 0, 4(4)
	addi 6, 6, -1
	cmplwi	6, 0
	lfsu 1, 4(5)
	fmuls 0, 1, 0
	stfsu 0, 4(3)
	bne	0, .Llfa_BB4_6
.Llfa_BB4_7:
	ld 30, 40(1)                            # 8-byte Folded Reload
	ld 29, 32(1)                            # 8-byte Folded Reload
	ld 28, 24(1)                            # 8-byte Folded Reload
	ld 27, 16(1)                            # 8-byte Folded Reload
	addi 1, 1, 64
	blr
.Llfa_func_end4:
	.size	lfa_k_mul_f32_r, .Llfa_func_end4-.Llfa_func_begin4
                                        # -- End function
	.section	.rodata.cst4,"aM",@progbits,4
	.p2align	2, 0x0                          # -- Begin function lfa_k_sum_f32
.Llfa_CPI5_0:
	.long	0x00000000                      # float 0
	.section	.text.lfa_k_sum_f32,"ax",@progbits
	.globl	lfa_k_sum_f32
	.p2align	2
	.type	lfa_k_sum_f32,@function
lfa_k_sum_f32:                          # @lfa_k_sum_f32
.Llfa_func_begin5:
# %bb.0:                                # %entry
	cmpwi	4, 1
	lis 6, .Llfa_CPI5_0@ha
	blt	0, .Llfa_BB5_3
# %bb.1:                                # %for.body.preheader
	stwu 1, -32(1)
	cmplwi	4, 8
	bge 0, .Llfa_BB5_4
# %bb.2:
	li 5, 0
	lfs 1, .Llfa_CPI5_0@l(6)
	b .Llfa_BB5_7
.Llfa_BB5_3:
	lfs 1, .Llfa_CPI5_0@l(6)
	blr
.Llfa_BB5_4:                                # %vector.ph
	rlwinm 5, 4, 0, 1, 28
	vxor 2, 2, 2
	addi 6, 3, 16
	li 7, 0
	li 8, 15
	mr	9, 5
	vxor 3, 3, 3
.Llfa_BB5_5:                                # %vector.body
                                        # =>This Inner Loop Header: Depth=1
	add 10, 3, 7
	lvx 4, 3, 7
	addi 9, 9, -8
	cmplwi	9, 0
	lvx 5, 10, 8
	add 10, 6, 7
	lvsl 0, 3, 7
	vperm 4, 4, 5, 0
	lvx 5, 6, 7
	lvx 0, 10, 8
	lvsl 1, 6, 7
	vaddfp 2, 4, 2
	addi 7, 7, 32
	vperm 5, 5, 0, 1
	vaddfp 3, 5, 3
	bne	0, .Llfa_BB5_5
# %bb.6:                                # %middle.block
	vaddfp 2, 3, 2
	addi 6, 1, 16
	cmplw	4, 5
	vsldoi 3, 2, 2, 8
	vaddfp 2, 2, 3
	vspltw 3, 2, 1
	vaddfp 2, 2, 3
	stvx 2, 0, 6
	lfs 1, 16(1)
	beq	0, .Llfa_BB5_9
.Llfa_BB5_7:                                # %for.body.preheader9
	slwi 6, 5, 2
	add 6, 6, 3
	sub	3, 4, 5
	addi 4, 6, -4
.Llfa_BB5_8:                                # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lfsu 0, 4(4)
	addi 3, 3, -1
	cmplwi	3, 0
	fadds 1, 0, 1
	bne	0, .Llfa_BB5_8
.Llfa_BB5_9:
	addi 1, 1, 32
	blr
.Llfa_func_end5:
	.size	lfa_k_sum_f32, .Llfa_func_end5-.Llfa_func_begin5
                                        # -- End function
	.section	.rodata.cst4,"aM",@progbits,4
	.p2align	2, 0x0                          # -- Begin function lfa_k_dot_f32
.Llfa_CPI6_0:
	.long	0x00000000                      # float 0
	.section	.text.lfa_k_dot_f32,"ax",@progbits
	.globl	lfa_k_dot_f32
	.p2align	2
	.type	lfa_k_dot_f32,@function
lfa_k_dot_f32:                          # @lfa_k_dot_f32
.Llfa_func_begin6:
# %bb.0:                                # %entry
	cmpwi	5, 1
	lis 7, .Llfa_CPI6_0@ha
	blt	0, .Llfa_BB6_3
# %bb.1:                                # %for.body.preheader
	stwu 1, -32(1)
	cmplwi	5, 8
	bge 0, .Llfa_BB6_4
# %bb.2:
	li 6, 0
	lfs 1, .Llfa_CPI6_0@l(7)
	b .Llfa_BB6_7
.Llfa_BB6_3:
	lfs 1, .Llfa_CPI6_0@l(7)
	blr
.Llfa_BB6_4:                                # %vector.ph
	rlwinm 6, 5, 0, 1, 28
	vxor 2, 2, 2
	addi 7, 4, 16
	addi 8, 3, 16
	li 9, 0
	li 10, 15
	mr	11, 6
	vxor 3, 3, 3
.Llfa_BB6_5:                                # %vector.body
                                        # =>This Inner Loop Header: Depth=1
	add 12, 3, 9
	lvx 5, 3, 9
	addi 11, 11, -8
	cmplwi	11, 0
	lvx 0, 12, 10
	add 12, 8, 9
	lvsl 4, 3, 9
	vperm 4, 5, 0, 4
	lvx 5, 8, 9
	lvx 0, 12, 10
	add 12, 4, 9
	lvsl 1, 8, 9
	vperm 5, 5, 0, 1
	lvx 6, 4, 9
	lvx 0, 12, 10
	add 12, 7, 9
	lvsl 1, 4, 9
	vperm 0, 6, 0, 1
	lvx 1, 7, 9
	lvx 6, 12, 10
	lvsl 7, 7, 9
	vmaddfp 2, 0, 4, 2
	addi 9, 9, 32
	vperm 1, 1, 6, 7
	vmaddfp 3, 1, 5, 3
	bne	0, .Llfa_BB6_5
# %bb.6:                                # %middle.block
	vaddfp 2, 3, 2
	addi 7, 1, 16
	cmplw	5, 6
	vsldoi 3, 2, 2, 8
	vaddfp 2, 2, 3
	vspltw 3, 2, 1
	vaddfp 2, 2, 3
	stvx 2, 0, 7
	lfs 1, 16(1)
	beq	0, .Llfa_BB6_9
.Llfa_BB6_7:                                # %for.body.preheader13
	slwi 7, 6, 2
	sub	5, 5, 6
	addi 6, 7, -4
	add 3, 3, 6
	add 4, 4, 6
.Llfa_BB6_8:                                # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lfsu 0, 4(4)
	addi 5, 5, -1
	cmplwi	5, 0
	lfsu 2, 4(3)
	fmadds 1, 0, 2, 1
	bne	0, .Llfa_BB6_8
.Llfa_BB6_9:
	addi 1, 1, 32
	blr
.Llfa_func_end6:
	.size	lfa_k_dot_f32, .Llfa_func_end6-.Llfa_func_begin6
                                        # -- End function
	.section	.text.lfa_k_sum_i32,"ax",@progbits
	.globl	lfa_k_sum_i32                   # -- Begin function lfa_k_sum_i32
	.p2align	2
	.type	lfa_k_sum_i32,@function
lfa_k_sum_i32:                          # @lfa_k_sum_i32
.Llfa_func_begin7:
# %bb.0:                                # %entry
	cmpwi	4, 1
	blt	0, .Llfa_BB7_8
# %bb.1:                                # %for.body.preheader
	stwu 1, -32(1)
	cmplwi	4, 8
	li 5, 0
	li 6, 0
	blt	0, .Llfa_BB7_5
# %bb.2:                                # %vector.ph
	rlwinm 6, 4, 0, 1, 28
	vxor 2, 2, 2
	addi 5, 3, 16
	li 7, 0
	li 8, 15
	mr	9, 6
	vxor 3, 3, 3
.Llfa_BB7_3:                                # %vector.body
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
	bne	0, .Llfa_BB7_3
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
	beq	0, .Llfa_BB7_7
.Llfa_BB7_5:                                # %for.body.preheader9
	slwi 7, 6, 2
	add 7, 7, 3
	sub	3, 4, 6
	addi 4, 7, -4
.Llfa_BB7_6:                                # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lwzu 6, 4(4)
	addi 3, 3, -1
	cmplwi	3, 0
	add 5, 6, 5
	bne	0, .Llfa_BB7_6
.Llfa_BB7_7:
	addi 1, 1, 32
	mr	3, 5
	blr
.Llfa_BB7_8:
	li 3, 0
	blr
.Llfa_func_end7:
	.size	lfa_k_sum_i32, .Llfa_func_end7-.Llfa_func_begin7
                                        # -- End function
	.section	.text.lfa_k_dot_i16,"ax",@progbits
	.globl	lfa_k_dot_i16                   # -- Begin function lfa_k_dot_i16
	.p2align	2
	.type	lfa_k_dot_i16,@function
lfa_k_dot_i16:                          # @lfa_k_dot_i16
.Llfa_func_begin8:
# %bb.0:                                # %entry
	cmpwi	5, 1
	blt	0, .Llfa_BB8_8
# %bb.1:                                # %for.body.preheader
	stwu 1, -48(1)
	li 6, 0
	li 7, 0
	std 30, 24(1)                           # 8-byte Folded Spill
	beq	0, .Llfa_BB8_5
# %bb.2:                                # %vector.ph
	rlwinm 7, 5, 0, 1, 30
	addi 8, 3, -4
	addi 9, 4, -4
	mr	11, 7
	li 10, 0
.Llfa_BB8_3:                                # %vector.body
                                        # =>This Inner Loop Header: Depth=1
	lhzu 12, 4(8)
	addi 11, 11, -2
	cmplwi	11, 0
	lhzu 0, 4(9)
	extsh 12, 12
	extsh 0, 0
	lhz 30, 2(8)
	mullw 12, 0, 12
	add 6, 12, 6
	lhz 0, 2(9)
	extsh 30, 30
	extsh 0, 0
	mullw 0, 0, 30
	add 10, 0, 10
	bne	0, .Llfa_BB8_3
# %bb.4:                                # %middle.block
	cmplw	5, 7
	add 6, 10, 6
	beq	0, .Llfa_BB8_7
.Llfa_BB8_5:                                # %for.body.preheader11
	slwi 8, 7, 1
	sub	5, 5, 7
	addi 7, 8, -2
	add 3, 3, 7
	add 4, 4, 7
.Llfa_BB8_6:                                # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lhzu 7, 2(3)
	addi 5, 5, -1
	cmplwi	5, 0
	lhzu 8, 2(4)
	extsh 7, 7
	extsh 8, 8
	mullw 7, 8, 7
	add 6, 7, 6
	bne	0, .Llfa_BB8_6
.Llfa_BB8_7:
	ld 30, 24(1)                            # 8-byte Folded Reload
	addi 1, 1, 48
	mr	3, 6
	blr
.Llfa_BB8_8:
	li 3, 0
	blr
.Llfa_func_end8:
	.size	lfa_k_dot_i16, .Llfa_func_end8-.Llfa_func_begin8
                                        # -- End function
	.section	.text.lfa_k_max_f32,"ax",@progbits
	.globl	lfa_k_max_f32                   # -- Begin function lfa_k_max_f32
	.p2align	2
	.type	lfa_k_max_f32,@function
lfa_k_max_f32:                          # @lfa_k_max_f32
.Llfa_func_begin9:
# %bb.0:                                # %entry
	lfs 1, 0(3)
	cmpwi	4, 2
	bltlr	0
# %bb.1:                                # %for.body.preheader
	li 5, 1
	beq	0, .Llfa_BB9_5
# %bb.2:                                # %vector.ph
	addi 6, 4, -1
	addi 8, 3, -4
	fmr 0, 1
	rlwinm 7, 6, 0, 0, 30
	ori 5, 6, 1
	mr	9, 7
.Llfa_BB9_3:                                # %vector.body
                                        # =>This Inner Loop Header: Depth=1
	lfsu 2, 8(8)
	addi 9, 9, -2
	cmplwi	9, 0
	fsubs 3, 1, 2
	fsel 1, 3, 1, 2
	lfs 4, 4(8)
	fsubs 5, 0, 4
	fsel 0, 5, 0, 4
	bne	0, .Llfa_BB9_3
# %bb.4:                                # %middle.block
	fsubs 2, 0, 1
	cmplw	6, 7
	fsel 1, 2, 0, 1
	beqlr	0
.Llfa_BB9_5:                                # %for.body.preheader15
	slwi 6, 5, 2
	add 6, 6, 3
	sub	3, 4, 5
	addi 4, 6, -4
.Llfa_BB9_6:                                # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lfsu 0, 4(4)
	addi 3, 3, -1
	cmplwi	3, 0
	fsubs 2, 1, 0
	fsel 1, 2, 1, 0
	bne	0, .Llfa_BB9_6
	blr
.Llfa_func_end9:
	.size	lfa_k_max_f32, .Llfa_func_end9-.Llfa_func_begin9
                                        # -- End function
	.section	.text.lfa_k_add_i32,"ax",@progbits
	.globl	lfa_k_add_i32                   # -- Begin function lfa_k_add_i32
	.p2align	2
	.type	lfa_k_add_i32,@function
lfa_k_add_i32:                          # @lfa_k_add_i32
.Llfa_func_begin10:
# %bb.0:                                # %entry
	cmpwi	6, 1
	bltlr	0
# %bb.1:                                # %for.body.preheader
	stwu 1, -64(1)
	cmplwi	6, 12
	li 7, 0
	std 27, 16(1)                           # 8-byte Folded Spill
	std 28, 24(1)                           # 8-byte Folded Spill
	std 29, 32(1)                           # 8-byte Folded Spill
	std 30, 40(1)                           # 8-byte Folded Spill
	bge 0, .Llfa_BB10_5
.Llfa_BB10_2:                               # %for.body.preheader16
	slwi 8, 7, 2
	sub	6, 6, 7
	addi 8, 8, -4
	add 4, 4, 8
	add 5, 5, 8
	add 3, 3, 8
.Llfa_BB10_3:                               # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lwzu 7, 4(4)
	addi 6, 6, -1
	cmplwi	6, 0
	lwzu 8, 4(5)
	add 7, 8, 7
	stwu 7, 4(3)
	bne	0, .Llfa_BB10_3
.Llfa_BB10_4:
	ld 30, 40(1)                            # 8-byte Folded Reload
	ld 29, 32(1)                            # 8-byte Folded Reload
	ld 28, 24(1)                            # 8-byte Folded Reload
	ld 27, 16(1)                            # 8-byte Folded Reload
	addi 1, 1, 64
	blr
.Llfa_BB10_5:                               # %vector.memcheck
	sub	8, 3, 4
	cmplwi	8, 32
	sub	8, 3, 5
	bc 12, 0, .Llfa_BB10_2
# %bb.6:                                # %vector.memcheck
	cmplwi	8, 32
	bc 12, 0, .Llfa_BB10_2
# %bb.7:                                # %vector.ph
	rlwinm 7, 6, 0, 1, 28
	addi 8, 3, 16
	addi 9, 5, 16
	addi 10, 4, 16
	li 11, 0
	li 12, 15
	li 0, 12
	li 30, 8
	li 29, 4
	mr	28, 7
.Llfa_BB10_8:                               # %vector.body
                                        # =>This Inner Loop Header: Depth=1
	add 27, 4, 11
	lvx 3, 4, 11
	addi 28, 28, -8
	cmplwi	28, 0
	lvx 5, 27, 12
	add 27, 10, 11
	lvsl 2, 4, 11
	vperm 2, 3, 5, 2
	lvx 3, 10, 11
	lvx 5, 27, 12
	add 27, 5, 11
	lvsl 4, 10, 11
	vperm 3, 3, 5, 4
	lvx 1, 5, 11
	lvx 4, 27, 12
	add 27, 9, 11
	lvsl 0, 5, 11
	vperm 4, 1, 4, 0
	vadduwm 2, 4, 2
	lvsr 0, 3, 11
	lvx 4, 9, 11
	vperm 2, 2, 2, 0
	lvx 0, 27, 12
	add 27, 3, 11
	lvsl 5, 9, 11
	lvsr 1, 8, 11
	vperm 4, 4, 0, 5
	stvewx 2, 27, 0
	stvewx 2, 27, 30
	stvewx 2, 27, 29
	add 27, 8, 11
	stvewx 2, 3, 11
	vadduwm 2, 4, 3
	vperm 2, 2, 2, 1
	stvewx 2, 8, 11
	addi 11, 11, 32
	stvewx 2, 27, 0
	stvewx 2, 27, 30
	stvewx 2, 27, 29
	bne	0, .Llfa_BB10_8
# %bb.9:                                # %middle.block
	cmplw	6, 7
	bne	0, .Llfa_BB10_2
	b .Llfa_BB10_4
.Llfa_func_end10:
	.size	lfa_k_add_i32, .Llfa_func_end10-.Llfa_func_begin10
                                        # -- End function
	.section	.text.lfa_k_add_i32_r,"ax",@progbits
	.globl	lfa_k_add_i32_r                 # -- Begin function lfa_k_add_i32_r
	.p2align	2
	.type	lfa_k_add_i32_r,@function
lfa_k_add_i32_r:                        # @lfa_k_add_i32_r
.Llfa_func_begin11:
# %bb.0:                                # %entry
	cmpwi	6, 1
	bltlr	0
# %bb.1:                                # %for.body.preheader
	stwu 1, -64(1)
	cmplwi	6, 8
	li 7, 0
	std 27, 16(1)                           # 8-byte Folded Spill
	std 28, 24(1)                           # 8-byte Folded Spill
	std 29, 32(1)                           # 8-byte Folded Spill
	std 30, 40(1)                           # 8-byte Folded Spill
	blt	0, .Llfa_BB11_5
# %bb.2:                                # %vector.ph
	rlwinm 7, 6, 0, 1, 28
	addi 8, 3, 16
	addi 9, 5, 16
	addi 10, 4, 16
	li 11, 0
	li 12, 15
	li 0, 12
	li 30, 8
	li 29, 4
	mr	28, 7
.Llfa_BB11_3:                               # %vector.body
                                        # =>This Inner Loop Header: Depth=1
	add 27, 4, 11
	lvx 3, 4, 11
	addi 28, 28, -8
	cmplwi	28, 0
	lvx 5, 27, 12
	add 27, 10, 11
	lvsl 2, 4, 11
	vperm 2, 3, 5, 2
	lvx 3, 10, 11
	lvx 5, 27, 12
	add 27, 5, 11
	lvsl 4, 10, 11
	vperm 3, 3, 5, 4
	lvx 1, 5, 11
	lvx 4, 27, 12
	add 27, 9, 11
	lvsl 0, 5, 11
	vperm 4, 1, 4, 0
	vadduwm 2, 4, 2
	lvsr 0, 3, 11
	lvx 4, 9, 11
	vperm 2, 2, 2, 0
	lvx 0, 27, 12
	add 27, 3, 11
	lvsl 5, 9, 11
	lvsr 1, 8, 11
	vperm 4, 4, 0, 5
	stvewx 2, 27, 0
	stvewx 2, 27, 30
	stvewx 2, 27, 29
	add 27, 8, 11
	stvewx 2, 3, 11
	vadduwm 2, 4, 3
	vperm 2, 2, 2, 1
	stvewx 2, 8, 11
	addi 11, 11, 32
	stvewx 2, 27, 0
	stvewx 2, 27, 30
	stvewx 2, 27, 29
	bne	0, .Llfa_BB11_3
# %bb.4:                                # %middle.block
	cmplw	6, 7
	beq	0, .Llfa_BB11_7
.Llfa_BB11_5:                               # %for.body.preheader12
	slwi 8, 7, 2
	sub	6, 6, 7
	addi 8, 8, -4
	add 4, 4, 8
	add 5, 5, 8
	add 3, 3, 8
.Llfa_BB11_6:                               # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lwzu 7, 4(4)
	addi 6, 6, -1
	cmplwi	6, 0
	lwzu 8, 4(5)
	add 7, 8, 7
	stwu 7, 4(3)
	bne	0, .Llfa_BB11_6
.Llfa_BB11_7:
	ld 30, 40(1)                            # 8-byte Folded Reload
	ld 29, 32(1)                            # 8-byte Folded Reload
	ld 28, 24(1)                            # 8-byte Folded Reload
	ld 27, 16(1)                            # 8-byte Folded Reload
	addi 1, 1, 64
	blr
.Llfa_func_end11:
	.size	lfa_k_add_i32_r, .Llfa_func_end11-.Llfa_func_begin11
                                        # -- End function
	.section	.text.lfa_k_add_i16_r,"ax",@progbits
	.globl	lfa_k_add_i16_r                 # -- Begin function lfa_k_add_i16_r
	.p2align	2
	.type	lfa_k_add_i16_r,@function
lfa_k_add_i16_r:                        # @lfa_k_add_i16_r
.Llfa_func_begin12:
# %bb.0:                                # %entry
	cmpwi	6, 1
	bltlr	0
# %bb.1:                                # %for.body.preheader
	stwu 1, -80(1)
	cmplwi	6, 8
	li 7, 0
	std 26, 24(1)                           # 8-byte Folded Spill
	std 27, 32(1)                           # 8-byte Folded Spill
	std 28, 40(1)                           # 8-byte Folded Spill
	std 29, 48(1)                           # 8-byte Folded Spill
	std 30, 56(1)                           # 8-byte Folded Spill
	blt	0, .Llfa_BB12_5
# %bb.2:                                # %vector.ph
	rlwinm 7, 6, 0, 1, 28
	li 8, 0
	li 9, 15
	li 10, 14
	li 11, 12
	li 12, 10
	li 0, 8
	li 30, 6
	li 29, 4
	li 28, 2
	mr	27, 7
.Llfa_BB12_3:                               # %vector.body
                                        # =>This Inner Loop Header: Depth=1
	add 26, 4, 8
	lvx 2, 4, 8
	addi 27, 27, -8
	cmplwi	27, 0
	lvx 3, 26, 9
	add 26, 5, 8
	lvsl 4, 4, 8
	vperm 2, 2, 3, 4
	lvx 3, 5, 8
	lvx 4, 26, 9
	add 26, 3, 8
	lvsl 5, 5, 8
	vperm 3, 3, 4, 5
	vadduhm 2, 3, 2
	lvsr 4, 3, 8
	vperm 2, 2, 2, 4
	stvehx 2, 3, 8
	addi 8, 8, 16
	stvehx 2, 26, 10
	stvehx 2, 26, 11
	stvehx 2, 26, 12
	stvehx 2, 26, 0
	stvehx 2, 26, 30
	stvehx 2, 26, 29
	stvehx 2, 26, 28
	bne	0, .Llfa_BB12_3
# %bb.4:                                # %middle.block
	cmplw	6, 7
	beq	0, .Llfa_BB12_7
.Llfa_BB12_5:                               # %for.body.preheader12
	slwi 8, 7, 1
	sub	6, 6, 7
	addi 8, 8, -2
	add 4, 4, 8
	add 5, 5, 8
	add 3, 3, 8
.Llfa_BB12_6:                               # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lhzu 7, 2(4)
	addi 6, 6, -1
	cmplwi	6, 0
	lhzu 8, 2(5)
	add 7, 8, 7
	sthu 7, 2(3)
	bne	0, .Llfa_BB12_6
.Llfa_BB12_7:
	ld 30, 56(1)                            # 8-byte Folded Reload
	ld 29, 48(1)                            # 8-byte Folded Reload
	ld 28, 40(1)                            # 8-byte Folded Reload
	ld 27, 32(1)                            # 8-byte Folded Reload
	ld 26, 24(1)                            # 8-byte Folded Reload
	addi 1, 1, 80
	blr
.Llfa_func_end12:
	.size	lfa_k_add_i16_r, .Llfa_func_end12-.Llfa_func_begin12
                                        # -- End function
	.section	.text.lfa_k_add_u8_r,"ax",@progbits
	.globl	lfa_k_add_u8_r                  # -- Begin function lfa_k_add_u8_r
	.p2align	2
	.type	lfa_k_add_u8_r,@function
lfa_k_add_u8_r:                         # @lfa_k_add_u8_r
.Llfa_func_begin13:
# %bb.0:                                # %entry
	cmpwi	6, 1
	bltlr	0
# %bb.1:                                # %iter.check
	stwu 1, -160(1)
	cmplwi	6, 8
	li 7, 0
	std 20, 56(1)                           # 8-byte Folded Spill
	std 21, 64(1)                           # 8-byte Folded Spill
	std 22, 72(1)                           # 8-byte Folded Spill
	std 23, 80(1)                           # 8-byte Folded Spill
	std 24, 88(1)                           # 8-byte Folded Spill
	std 25, 96(1)                           # 8-byte Folded Spill
	std 26, 104(1)                          # 8-byte Folded Spill
	std 27, 112(1)                          # 8-byte Folded Spill
	std 28, 120(1)                          # 8-byte Folded Spill
	std 29, 128(1)                          # 8-byte Folded Spill
	std 30, 136(1)                          # 8-byte Folded Spill
	blt	0, .Llfa_BB13_10
# %bb.2:                                # %vector.main.loop.iter.check
	cmplwi	6, 16
	bge 0, .Llfa_BB13_6
.Llfa_BB13_3:                               # %vec.epilog.ph
	mr	8, 7
	rlwinm 7, 6, 0, 1, 28
	addi 9, 1, 16
	addi 10, 1, 32
	li 11, 7
	li 12, 6
	li 0, 5
	li 30, 4
	li 29, 3
	li 28, 2
	li 27, 1
.Llfa_BB13_4:                               # %vec.epilog.vector.body
                                        # =>This Inner Loop Header: Depth=1
	lbzx 26, 4, 8
	stb 26, 16(1)
	add 26, 4, 8
	lbz 25, 1(26)
	stb 25, 17(1)
	lbz 25, 2(26)
	stb 25, 18(1)
	lbz 25, 3(26)
	stb 25, 19(1)
	lbz 25, 4(26)
	stb 25, 20(1)
	lbz 25, 5(26)
	stb 25, 21(1)
	lbz 25, 6(26)
	stb 25, 22(1)
	lbz 26, 7(26)
	stb 26, 23(1)
	lbzx 26, 5, 8
	lvx 2, 0, 9
	stb 26, 32(1)
	add 26, 5, 8
	lbz 25, 1(26)
	stb 25, 33(1)
	lbz 25, 2(26)
	stb 25, 34(1)
	lbz 25, 3(26)
	stb 25, 35(1)
	lbz 25, 4(26)
	stb 25, 36(1)
	lbz 25, 5(26)
	stb 25, 37(1)
	lbz 25, 6(26)
	stb 25, 38(1)
	lbz 26, 7(26)
	stb 26, 39(1)
	add 26, 3, 8
	lvx 3, 0, 10
	lvsr 4, 3, 8
	vaddubm 2, 3, 2
	vperm 2, 2, 2, 4
	stvebx 2, 3, 8
	addi 8, 8, 8
	cmplw	7, 8
	stvebx 2, 26, 11
	stvebx 2, 26, 12
	stvebx 2, 26, 0
	stvebx 2, 26, 30
	stvebx 2, 26, 29
	stvebx 2, 26, 28
	stvebx 2, 26, 27
	bne	0, .Llfa_BB13_4
# %bb.5:                                # %vec.epilog.middle.block
	cmplw	6, 7
	bne	0, .Llfa_BB13_10
	b .Llfa_BB13_12
.Llfa_BB13_6:                               # %vector.ph
	rlwinm 7, 6, 0, 1, 27
	li 8, 0
	li 9, 15
	li 10, 14
	li 11, 13
	li 12, 12
	li 0, 11
	li 30, 10
	li 29, 9
	li 28, 8
	li 27, 7
	li 26, 6
	li 25, 5
	li 24, 4
	li 23, 3
	li 22, 2
	li 21, 1
.Llfa_BB13_7:                               # %vector.body
                                        # =>This Inner Loop Header: Depth=1
	add 20, 4, 8
	lvx 2, 4, 8
	lvx 3, 20, 9
	add 20, 5, 8
	lvsl 4, 4, 8
	vperm 2, 2, 3, 4
	lvx 3, 5, 8
	lvx 4, 20, 9
	add 20, 3, 8
	lvsl 5, 5, 8
	vperm 3, 3, 4, 5
	vaddubm 2, 3, 2
	lvsr 4, 3, 8
	vperm 2, 2, 2, 4
	stvebx 2, 3, 8
	addi 8, 8, 16
	cmplw	7, 8
	stvebx 2, 20, 9
	stvebx 2, 20, 10
	stvebx 2, 20, 11
	stvebx 2, 20, 12
	stvebx 2, 20, 0
	stvebx 2, 20, 30
	stvebx 2, 20, 29
	stvebx 2, 20, 28
	stvebx 2, 20, 27
	stvebx 2, 20, 26
	stvebx 2, 20, 25
	stvebx 2, 20, 24
	stvebx 2, 20, 23
	stvebx 2, 20, 22
	stvebx 2, 20, 21
	bne	0, .Llfa_BB13_7
# %bb.8:                                # %middle.block
	cmplw	6, 7
	beq	0, .Llfa_BB13_12
# %bb.9:                                # %vec.epilog.iter.check
	andi. 8, 6, 8
	bne	0, .Llfa_BB13_3
.Llfa_BB13_10:                              # %for.body.preheader
	addi 8, 7, -1
	add 4, 4, 8
	add 5, 5, 8
	add 3, 3, 8
.Llfa_BB13_11:                              # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lbzu 8, 1(4)
	addi 7, 7, 1
	cmplw	6, 7
	lbzu 9, 1(5)
	add 8, 9, 8
	stbu 8, 1(3)
	bne	0, .Llfa_BB13_11
.Llfa_BB13_12:
	ld 30, 136(1)                           # 8-byte Folded Reload
	ld 29, 128(1)                           # 8-byte Folded Reload
	ld 28, 120(1)                           # 8-byte Folded Reload
	ld 27, 112(1)                           # 8-byte Folded Reload
	ld 26, 104(1)                           # 8-byte Folded Reload
	ld 25, 96(1)                            # 8-byte Folded Reload
	ld 24, 88(1)                            # 8-byte Folded Reload
	ld 23, 80(1)                            # 8-byte Folded Reload
	ld 22, 72(1)                            # 8-byte Folded Reload
	ld 21, 64(1)                            # 8-byte Folded Reload
	ld 20, 56(1)                            # 8-byte Folded Reload
	addi 1, 1, 160
	blr
.Llfa_func_end13:
	.size	lfa_k_add_u8_r, .Llfa_func_end13-.Llfa_func_begin13
                                        # -- End function
	.section	.text.lfa_k_avg_u8_r,"ax",@progbits
	.globl	lfa_k_avg_u8_r                  # -- Begin function lfa_k_avg_u8_r
	.p2align	2
	.type	lfa_k_avg_u8_r,@function
lfa_k_avg_u8_r:                         # @lfa_k_avg_u8_r
.Llfa_func_begin14:
# %bb.0:                                # %entry
	cmpwi	6, 1
	bltlr	0
# %bb.1:                                # %iter.check
	stwu 1, -160(1)
	cmplwi	6, 8
	li 7, 0
	std 20, 56(1)                           # 8-byte Folded Spill
	std 21, 64(1)                           # 8-byte Folded Spill
	std 22, 72(1)                           # 8-byte Folded Spill
	std 23, 80(1)                           # 8-byte Folded Spill
	std 24, 88(1)                           # 8-byte Folded Spill
	std 25, 96(1)                           # 8-byte Folded Spill
	std 26, 104(1)                          # 8-byte Folded Spill
	std 27, 112(1)                          # 8-byte Folded Spill
	std 28, 120(1)                          # 8-byte Folded Spill
	std 29, 128(1)                          # 8-byte Folded Spill
	std 30, 136(1)                          # 8-byte Folded Spill
	blt	0, .Llfa_BB14_10
# %bb.2:                                # %vector.main.loop.iter.check
	cmplwi	6, 16
	vspltisb 2, 1
	bge 0, .Llfa_BB14_6
.Llfa_BB14_3:                               # %vec.epilog.ph
	mr	8, 7
	rlwinm 7, 6, 0, 1, 28
	addi 9, 1, 16
	addi 10, 1, 32
	li 11, 7
	li 12, 6
	li 0, 5
	li 30, 4
	li 29, 3
	li 28, 2
	li 27, 1
.Llfa_BB14_4:                               # %vec.epilog.vector.body
                                        # =>This Inner Loop Header: Depth=1
	stb 3, 31(1)
	stb 3, 30(1)
	stb 3, 29(1)
	stb 3, 28(1)
	stb 3, 27(1)
	stb 3, 26(1)
	stb 3, 25(1)
	stb 3, 24(1)
	lbzx 26, 4, 8
	stb 26, 16(1)
	add 26, 4, 8
	lbz 25, 1(26)
	stb 25, 17(1)
	lbz 25, 2(26)
	stb 25, 18(1)
	lbz 25, 3(26)
	stb 25, 19(1)
	lbz 25, 4(26)
	stb 25, 20(1)
	lbz 25, 5(26)
	stb 25, 21(1)
	lbz 25, 6(26)
	stb 25, 22(1)
	lbz 26, 7(26)
	stb 26, 23(1)
	lvx 3, 0, 9
	stb 3, 47(1)
	stb 3, 46(1)
	stb 3, 45(1)
	stb 3, 44(1)
	stb 3, 43(1)
	stb 3, 42(1)
	stb 3, 41(1)
	stb 3, 40(1)
	lbzx 26, 5, 8
	stb 26, 32(1)
	add 26, 5, 8
	lbz 25, 1(26)
	stb 25, 33(1)
	lbz 25, 2(26)
	stb 25, 34(1)
	lbz 25, 3(26)
	stb 25, 35(1)
	lbz 25, 4(26)
	stb 25, 36(1)
	lbz 25, 5(26)
	stb 25, 37(1)
	lbz 25, 6(26)
	stb 25, 38(1)
	lbz 26, 7(26)
	stb 26, 39(1)
	add 26, 3, 8
	lvx 4, 0, 10
	vor 5, 3, 4
	vxor 3, 3, 4
	lvsr 4, 3, 8
	vsrb 3, 3, 2
	vsububm 3, 5, 3
	vperm 3, 3, 3, 4
	stvebx 3, 3, 8
	addi 8, 8, 8
	cmplw	7, 8
	stvebx 3, 26, 11
	stvebx 3, 26, 12
	stvebx 3, 26, 0
	stvebx 3, 26, 30
	stvebx 3, 26, 29
	stvebx 3, 26, 28
	stvebx 3, 26, 27
	bne	0, .Llfa_BB14_4
# %bb.5:                                # %vec.epilog.middle.block
	cmplw	6, 7
	bne	0, .Llfa_BB14_10
	b .Llfa_BB14_12
.Llfa_BB14_6:                               # %vector.ph
	rlwinm 7, 6, 0, 1, 27
	li 8, 0
	li 9, 15
	li 10, 14
	li 11, 13
	li 12, 12
	li 0, 11
	li 30, 10
	li 29, 9
	li 28, 8
	li 27, 7
	li 26, 6
	li 25, 5
	li 24, 4
	li 23, 3
	li 22, 2
	li 21, 1
.Llfa_BB14_7:                               # %vector.body
                                        # =>This Inner Loop Header: Depth=1
	add 20, 4, 8
	lvx 3, 4, 8
	lvx 4, 20, 9
	add 20, 5, 8
	lvsl 5, 4, 8
	vperm 3, 3, 4, 5
	lvx 4, 5, 8
	lvx 5, 20, 9
	add 20, 3, 8
	lvsl 0, 5, 8
	vperm 4, 4, 5, 0
	vor 5, 3, 4
	vxor 3, 3, 4
	lvsr 4, 3, 8
	vsrb 3, 3, 2
	vsububm 3, 5, 3
	vperm 3, 3, 3, 4
	stvebx 3, 3, 8
	addi 8, 8, 16
	cmplw	7, 8
	stvebx 3, 20, 9
	stvebx 3, 20, 10
	stvebx 3, 20, 11
	stvebx 3, 20, 12
	stvebx 3, 20, 0
	stvebx 3, 20, 30
	stvebx 3, 20, 29
	stvebx 3, 20, 28
	stvebx 3, 20, 27
	stvebx 3, 20, 26
	stvebx 3, 20, 25
	stvebx 3, 20, 24
	stvebx 3, 20, 23
	stvebx 3, 20, 22
	stvebx 3, 20, 21
	bne	0, .Llfa_BB14_7
# %bb.8:                                # %middle.block
	cmplw	6, 7
	beq	0, .Llfa_BB14_12
# %bb.9:                                # %vec.epilog.iter.check
	andi. 8, 6, 8
	bne	0, .Llfa_BB14_3
.Llfa_BB14_10:                              # %for.body.preheader
	addi 8, 7, -1
	add 4, 4, 8
	add 5, 5, 8
	add 3, 3, 8
.Llfa_BB14_11:                              # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lbzu 8, 1(4)
	addi 7, 7, 1
	cmplw	6, 7
	lbzu 9, 1(5)
	add 8, 8, 9
	addi 8, 8, 1
	srwi 8, 8, 1
	stbu 8, 1(3)
	bne	0, .Llfa_BB14_11
.Llfa_BB14_12:
	ld 30, 136(1)                           # 8-byte Folded Reload
	ld 29, 128(1)                           # 8-byte Folded Reload
	ld 28, 120(1)                           # 8-byte Folded Reload
	ld 27, 112(1)                           # 8-byte Folded Reload
	ld 26, 104(1)                           # 8-byte Folded Reload
	ld 25, 96(1)                            # 8-byte Folded Reload
	ld 24, 88(1)                            # 8-byte Folded Reload
	ld 23, 80(1)                            # 8-byte Folded Reload
	ld 22, 72(1)                            # 8-byte Folded Reload
	ld 21, 64(1)                            # 8-byte Folded Reload
	ld 20, 56(1)                            # 8-byte Folded Reload
	addi 1, 1, 160
	blr
.Llfa_func_end14:
	.size	lfa_k_avg_u8_r, .Llfa_func_end14-.Llfa_func_begin14
                                        # -- End function
	.section	.rodata.cst16,"aM",@progbits,16
	.p2align	4, 0x0                          # -- Begin function lfa_k_addsat_u8_r
.Llfa_CPI15_0:
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
.Llfa_CPI15_1:
	.byte	3                               # 0x3
	.byte	7                               # 0x7
	.byte	11                              # 0xb
	.byte	15                              # 0xf
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
	.section	.text.lfa_k_addsat_u8_r,"ax",@progbits
	.globl	lfa_k_addsat_u8_r
	.p2align	2
	.type	lfa_k_addsat_u8_r,@function
lfa_k_addsat_u8_r:                      # @lfa_k_addsat_u8_r
.Llfa_func_begin15:
# %bb.0:                                # %entry
	cmpwi	6, 1
	bltlr	0
# %bb.1:                                # %iter.check
	stwu 1, -160(1)
	cmplwi	6, 4
	li 7, 0
	std 20, 56(1)                           # 8-byte Folded Spill
	std 21, 64(1)                           # 8-byte Folded Spill
	std 22, 72(1)                           # 8-byte Folded Spill
	std 23, 80(1)                           # 8-byte Folded Spill
	std 24, 88(1)                           # 8-byte Folded Spill
	std 25, 96(1)                           # 8-byte Folded Spill
	std 26, 104(1)                          # 8-byte Folded Spill
	std 27, 112(1)                          # 8-byte Folded Spill
	std 28, 120(1)                          # 8-byte Folded Spill
	std 29, 128(1)                          # 8-byte Folded Spill
	std 30, 136(1)                          # 8-byte Folded Spill
	blt	0, .Llfa_BB15_11
# %bb.2:                                # %vector.main.loop.iter.check
	cmplwi	6, 16
	vspltisw 2, -8
	blt	0, .Llfa_BB15_7
# %bb.3:                                # %vector.ph
	li 22, .Llfa_CPI15_0@l
	lis 21, .Llfa_CPI15_0@ha
	vxor 3, 3, 3
	lvx 4, 21, 22
	rlwinm 7, 6, 0, 1, 27
	li 8, 0
	li 9, 15
	li 10, 14
	li 11, 13
	li 12, 12
	li 0, 11
	li 30, 10
	vsrw 5, 2, 2
	li 29, 9
	li 28, 8
	li 27, 7
	li 26, 6
	li 25, 5
	li 24, 4
	li 23, 3
	li 22, 2
	li 21, 1
.Llfa_BB15_4:                               # %vector.body
                                        # =>This Inner Loop Header: Depth=1
	add 20, 4, 8
	lvx 0, 4, 8
	lvx 1, 20, 9
	add 20, 5, 8
	lvsl 6, 4, 8
	vperm 0, 0, 1, 6
	vspltw 8, 0, 1
	lvx 1, 5, 8
	vspltw 9, 0, 3
	lvx 6, 20, 9
	add 20, 3, 8
	lvsl 7, 5, 8
	vperm 8, 3, 8, 4
	vperm 1, 1, 6, 7
	vspltw 10, 1, 1
	vspltw 11, 1, 3
	vperm 6, 3, 0, 4
	vspltw 0, 0, 2
	vperm 7, 3, 1, 4
	vspltw 1, 1, 2
	vadduwm 6, 7, 6
	vperm 9, 3, 9, 4
	vperm 0, 3, 0, 4
	vperm 10, 3, 10, 4
	vadduwm 7, 10, 8
	vperm 11, 3, 11, 4
	vadduwm 8, 11, 9
	vperm 1, 3, 1, 4
	vadduwm 0, 1, 0
	vminuw 6, 6, 5
	vminuw 7, 7, 5
	vminuw 8, 8, 5
	vminuw 0, 0, 5
	vpkuwum 0, 0, 8
	vpkuwum 1, 6, 7
	vpkuhum 0, 1, 0
	lvsr 1, 3, 8
	vperm 0, 0, 0, 1
	stvebx 0, 3, 8
	addi 8, 8, 16
	cmplw	7, 8
	stvebx 0, 20, 9
	stvebx 0, 20, 10
	stvebx 0, 20, 11
	stvebx 0, 20, 12
	stvebx 0, 20, 0
	stvebx 0, 20, 30
	stvebx 0, 20, 29
	stvebx 0, 20, 28
	stvebx 0, 20, 27
	stvebx 0, 20, 26
	stvebx 0, 20, 25
	stvebx 0, 20, 24
	stvebx 0, 20, 23
	stvebx 0, 20, 22
	stvebx 0, 20, 21
	bne	0, .Llfa_BB15_4
# %bb.5:                                # %middle.block
	cmplw	6, 7
	beq	0, .Llfa_BB15_10
# %bb.6:                                # %vec.epilog.iter.check
	andi. 8, 6, 12
	beq	0, .Llfa_BB15_11
.Llfa_BB15_7:                               # %vec.epilog.ph
	li 11, .Llfa_CPI15_0@l
	lis 12, .Llfa_CPI15_0@ha
	vxor 3, 3, 3
	lvx 4, 12, 11
	li 0, .Llfa_CPI15_1@l
	lis 12, .Llfa_CPI15_1@ha
	lvx 5, 12, 0
	mr	8, 7
	rlwinm 7, 6, 0, 1, 29
	addi 9, 1, 16
	addi 10, 1, 32
	li 11, 3
	vsrw 2, 2, 2
	li 12, 2
	li 0, 1
.Llfa_BB15_8:                               # %vec.epilog.vector.body
                                        # =>This Inner Loop Header: Depth=1
	lbzx 30, 4, 8
	stb 30, 16(1)
	add 30, 4, 8
	lbz 29, 1(30)
	stb 29, 17(1)
	lbz 29, 2(30)
	stb 29, 18(1)
	lbz 30, 3(30)
	stb 30, 19(1)
	lbzx 30, 5, 8
	lvx 0, 0, 9
	stb 30, 32(1)
	add 30, 5, 8
	vperm 0, 3, 0, 4
	lbz 29, 1(30)
	stb 29, 33(1)
	lbz 29, 2(30)
	stb 29, 34(1)
	lbz 30, 3(30)
	stb 30, 35(1)
	add 30, 3, 8
	lvx 1, 0, 10
	vperm 1, 3, 1, 4
	vadduwm 0, 1, 0
	lvsr 1, 3, 8
	vminuw 0, 0, 2
	vperm 0, 0, 0, 5
	vperm 0, 0, 0, 1
	stvebx 0, 3, 8
	addi 8, 8, 4
	cmplw	7, 8
	stvebx 0, 30, 11
	stvebx 0, 30, 12
	stvebx 0, 30, 0
	bne	0, .Llfa_BB15_8
# %bb.9:                                # %vec.epilog.middle.block
	cmplw	6, 7
	bne	0, .Llfa_BB15_11
.Llfa_BB15_10:
	ld 30, 136(1)                           # 8-byte Folded Reload
	ld 29, 128(1)                           # 8-byte Folded Reload
	ld 28, 120(1)                           # 8-byte Folded Reload
	ld 27, 112(1)                           # 8-byte Folded Reload
	ld 26, 104(1)                           # 8-byte Folded Reload
	ld 25, 96(1)                            # 8-byte Folded Reload
	ld 24, 88(1)                            # 8-byte Folded Reload
	ld 23, 80(1)                            # 8-byte Folded Reload
	ld 22, 72(1)                            # 8-byte Folded Reload
	ld 21, 64(1)                            # 8-byte Folded Reload
	ld 20, 56(1)                            # 8-byte Folded Reload
	addi 1, 1, 160
	blr
.Llfa_BB15_11:                              # %for.body.preheader
	addi 8, 7, -1
	add 4, 4, 8
	add 5, 5, 8
	add 3, 3, 8
	b .Llfa_BB15_13
.Llfa_BB15_12:                              # %for.body
                                        #   in Loop: Header=BB15_13 Depth=1
	addi 7, 7, 1
	stbu 8, 1(3)
	cmplw	6, 7
	beq	0, .Llfa_BB15_10
.Llfa_BB15_13:                              # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lbzu 8, 1(4)
	lbzu 9, 1(5)
	add 8, 9, 8
	cmplwi	8, 255
	blt	0, .Llfa_BB15_12
# %bb.14:                               # %for.body
                                        #   in Loop: Header=BB15_13 Depth=1
	li 8, 255
	b .Llfa_BB15_12
.Llfa_func_end15:
	.size	lfa_k_addsat_u8_r, .Llfa_func_end15-.Llfa_func_begin15
                                        # -- End function
	.section	.text.lfa_k_and_u32_r,"ax",@progbits
	.globl	lfa_k_and_u32_r                 # -- Begin function lfa_k_and_u32_r
	.p2align	2
	.type	lfa_k_and_u32_r,@function
lfa_k_and_u32_r:                        # @lfa_k_and_u32_r
.Llfa_func_begin16:
# %bb.0:                                # %entry
	cmpwi	6, 1
	bltlr	0
# %bb.1:                                # %for.body.preheader
	stwu 1, -64(1)
	cmplwi	6, 8
	li 7, 0
	std 27, 16(1)                           # 8-byte Folded Spill
	std 28, 24(1)                           # 8-byte Folded Spill
	std 29, 32(1)                           # 8-byte Folded Spill
	std 30, 40(1)                           # 8-byte Folded Spill
	blt	0, .Llfa_BB16_5
# %bb.2:                                # %vector.ph
	rlwinm 7, 6, 0, 1, 28
	addi 8, 3, 16
	addi 9, 5, 16
	addi 10, 4, 16
	li 11, 0
	li 12, 15
	li 0, 12
	li 30, 8
	li 29, 4
	mr	28, 7
.Llfa_BB16_3:                               # %vector.body
                                        # =>This Inner Loop Header: Depth=1
	add 27, 4, 11
	lvx 3, 4, 11
	addi 28, 28, -8
	cmplwi	28, 0
	lvx 4, 27, 12
	add 27, 10, 11
	lvsl 2, 4, 11
	vperm 2, 3, 4, 2
	lvx 3, 10, 11
	lvx 4, 27, 12
	add 27, 5, 11
	lvsl 5, 10, 11
	vperm 3, 3, 4, 5
	lvx 0, 5, 11
	lvx 4, 27, 12
	add 27, 9, 11
	lvsl 5, 5, 11
	vperm 4, 0, 4, 5
	lvx 5, 9, 11
	lvx 0, 27, 12
	add 27, 3, 11
	lvsl 1, 9, 11
	vand 2, 4, 2
	lvsr 4, 3, 11
	vperm 5, 5, 0, 1
	vperm 2, 2, 2, 4
	lvsr 4, 8, 11
	stvewx 2, 27, 0
	stvewx 2, 27, 30
	vand 3, 5, 3
	stvewx 2, 27, 29
	add 27, 8, 11
	stvewx 2, 3, 11
	vperm 2, 3, 3, 4
	stvewx 2, 8, 11
	addi 11, 11, 32
	stvewx 2, 27, 0
	stvewx 2, 27, 30
	stvewx 2, 27, 29
	bne	0, .Llfa_BB16_3
# %bb.4:                                # %middle.block
	cmplw	6, 7
	beq	0, .Llfa_BB16_7
.Llfa_BB16_5:                               # %for.body.preheader12
	slwi 8, 7, 2
	sub	6, 6, 7
	addi 8, 8, -4
	add 4, 4, 8
	add 5, 5, 8
	add 3, 3, 8
.Llfa_BB16_6:                               # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lwzu 7, 4(4)
	addi 6, 6, -1
	cmplwi	6, 0
	lwzu 8, 4(5)
	and 7, 8, 7
	stwu 7, 4(3)
	bne	0, .Llfa_BB16_6
.Llfa_BB16_7:
	ld 30, 40(1)                            # 8-byte Folded Reload
	ld 29, 32(1)                            # 8-byte Folded Reload
	ld 28, 24(1)                            # 8-byte Folded Reload
	ld 27, 16(1)                            # 8-byte Folded Reload
	addi 1, 1, 64
	blr
.Llfa_func_end16:
	.size	lfa_k_and_u32_r, .Llfa_func_end16-.Llfa_func_begin16
                                        # -- End function
	.section	.text.lfa_k_shl_i32_r,"ax",@progbits
	.globl	lfa_k_shl_i32_r                 # -- Begin function lfa_k_shl_i32_r
	.p2align	2
	.type	lfa_k_shl_i32_r,@function
lfa_k_shl_i32_r:                        # @lfa_k_shl_i32_r
.Llfa_func_begin17:
# %bb.0:                                # %entry
	cmpwi	5, 1
	bltlr	0
# %bb.1:                                # %for.body.preheader
	stwu 1, -48(1)
	cmplwi	5, 8
	li 6, 0
	std 29, 16(1)                           # 8-byte Folded Spill
	std 30, 24(1)                           # 8-byte Folded Spill
	blt	0, .Llfa_BB17_5
# %bb.2:                                # %vector.ph
	rlwinm 6, 5, 0, 1, 28
	addi 7, 3, 16
	vspltisw 2, 3
	addi 8, 4, 16
	li 9, 0
	li 10, 15
	li 11, 12
	li 12, 8
	li 0, 4
	mr	30, 6
.Llfa_BB17_3:                               # %vector.body
                                        # =>This Inner Loop Header: Depth=1
	add 29, 4, 9
	lvx 3, 4, 9
	addi 30, 30, -8
	cmplwi	30, 0
	lvx 4, 29, 10
	add 29, 8, 9
	lvsl 5, 4, 9
	vperm 3, 3, 4, 5
	lvx 4, 8, 9
	lvx 5, 29, 10
	add 29, 3, 9
	lvsl 0, 8, 9
	vslw 3, 3, 2
	vperm 4, 4, 5, 0
	lvsr 5, 3, 9
	vperm 3, 3, 3, 5
	lvsr 5, 7, 9
	stvewx 3, 29, 11
	stvewx 3, 29, 12
	vslw 4, 4, 2
	stvewx 3, 29, 0
	add 29, 7, 9
	stvewx 3, 3, 9
	vperm 3, 4, 4, 5
	stvewx 3, 7, 9
	addi 9, 9, 32
	stvewx 3, 29, 11
	stvewx 3, 29, 12
	stvewx 3, 29, 0
	bne	0, .Llfa_BB17_3
# %bb.4:                                # %middle.block
	cmplw	5, 6
	beq	0, .Llfa_BB17_7
.Llfa_BB17_5:                               # %for.body.preheader8
	slwi 7, 6, 2
	sub	5, 5, 6
	addi 6, 7, -4
	add 4, 4, 6
	add 3, 3, 6
.Llfa_BB17_6:                               # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lwzu 6, 4(4)
	addi 5, 5, -1
	cmplwi	5, 0
	slwi 6, 6, 3
	stwu 6, 4(3)
	bne	0, .Llfa_BB17_6
.Llfa_BB17_7:
	ld 30, 24(1)                            # 8-byte Folded Reload
	ld 29, 16(1)                            # 8-byte Folded Reload
	addi 1, 1, 48
	blr
.Llfa_func_end17:
	.size	lfa_k_shl_i32_r, .Llfa_func_end17-.Llfa_func_begin17
                                        # -- End function
	.section	.text.lfa_k_mul_i32_r,"ax",@progbits
	.globl	lfa_k_mul_i32_r                 # -- Begin function lfa_k_mul_i32_r
	.p2align	2
	.type	lfa_k_mul_i32_r,@function
lfa_k_mul_i32_r:                        # @lfa_k_mul_i32_r
.Llfa_func_begin18:
# %bb.0:                                # %entry
	cmpwi	6, 1
	bltlr	0
# %bb.1:                                # %for.body.preheader
	stwu 1, -48(1)
	li 7, 0
	std 30, 24(1)                           # 8-byte Folded Spill
	beq	0, .Llfa_BB18_5
# %bb.2:                                # %vector.ph
	rlwinm 7, 6, 0, 1, 30
	addi 8, 4, -8
	addi 9, 5, -8
	addi 10, 3, -8
	mr	11, 7
.Llfa_BB18_3:                               # %vector.body
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
	bne	0, .Llfa_BB18_3
# %bb.4:                                # %middle.block
	cmplw	6, 7
	beq	0, .Llfa_BB18_7
.Llfa_BB18_5:                               # %for.body.preheader9
	slwi 8, 7, 2
	sub	6, 6, 7
	addi 8, 8, -4
	add 4, 4, 8
	add 5, 5, 8
	add 3, 3, 8
.Llfa_BB18_6:                               # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lwzu 7, 4(4)
	addi 6, 6, -1
	cmplwi	6, 0
	lwzu 8, 4(5)
	mullw 7, 8, 7
	stwu 7, 4(3)
	bne	0, .Llfa_BB18_6
.Llfa_BB18_7:
	ld 30, 24(1)                            # 8-byte Folded Reload
	addi 1, 1, 48
	blr
.Llfa_func_end18:
	.size	lfa_k_mul_i32_r, .Llfa_func_end18-.Llfa_func_begin18
                                        # -- End function
	.section	.text.lfa_k_min_f32_r,"ax",@progbits
	.globl	lfa_k_min_f32_r                 # -- Begin function lfa_k_min_f32_r
	.p2align	2
	.type	lfa_k_min_f32_r,@function
lfa_k_min_f32_r:                        # @lfa_k_min_f32_r
.Llfa_func_begin19:
# %bb.0:                                # %entry
	cmpwi	6, 1
	bltlr	0
# %bb.1:                                # %for.body.preheader
	li 7, 0
	beq	0, .Llfa_BB19_5
# %bb.2:                                # %vector.ph
	rlwinm 7, 6, 0, 1, 30
	addi 8, 4, -8
	addi 9, 5, -8
	addi 10, 3, -8
	mr	11, 7
.Llfa_BB19_3:                               # %vector.body
                                        # =>This Inner Loop Header: Depth=1
	lfsu 0, 8(8)
	addi 11, 11, -2
	cmplwi	11, 0
	lfsu 1, 8(9)
	fsubs 3, 0, 1
	fsel 0, 3, 1, 0
	lfs 2, 4(8)
	lfs 1, 4(9)
	stfsu 0, 8(10)
	fsubs 0, 2, 1
	fsel 0, 0, 1, 2
	stfs 0, 4(10)
	bne	0, .Llfa_BB19_3
# %bb.4:                                # %middle.block
	cmplw	6, 7
	beqlr	0
.Llfa_BB19_5:                               # %for.body.preheader17
	slwi 8, 7, 2
	sub	6, 6, 7
	addi 8, 8, -4
	add 4, 4, 8
	add 5, 5, 8
	add 3, 3, 8
.Llfa_BB19_6:                               # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lfsu 0, 4(4)
	addi 6, 6, -1
	cmplwi	6, 0
	lfsu 1, 4(5)
	fsubs 2, 0, 1
	fsel 0, 2, 1, 0
	stfsu 0, 4(3)
	bne	0, .Llfa_BB19_6
	blr
.Llfa_func_end19:
	.size	lfa_k_min_f32_r, .Llfa_func_end19-.Llfa_func_begin19
                                        # -- End function
	.section	.text.lfa_k_max_i32_r,"ax",@progbits
	.globl	lfa_k_max_i32_r                 # -- Begin function lfa_k_max_i32_r
	.p2align	2
	.type	lfa_k_max_i32_r,@function
lfa_k_max_i32_r:                        # @lfa_k_max_i32_r
.Llfa_func_begin20:
# %bb.0:                                # %entry
	cmpwi	6, 1
	bltlr	0
# %bb.1:                                # %for.body.preheader
	stwu 1, -64(1)
	cmplwi	6, 8
	li 7, 0
	std 27, 16(1)                           # 8-byte Folded Spill
	std 28, 24(1)                           # 8-byte Folded Spill
	std 29, 32(1)                           # 8-byte Folded Spill
	std 30, 40(1)                           # 8-byte Folded Spill
	blt	0, .Llfa_BB20_6
# %bb.2:                                # %vector.ph
	rlwinm 7, 6, 0, 1, 28
	addi 8, 3, 16
	addi 9, 5, 16
	addi 10, 4, 16
	li 11, 0
	li 12, 15
	li 0, 12
	li 30, 8
	li 29, 4
	mr	28, 7
.Llfa_BB20_3:                               # %vector.body
                                        # =>This Inner Loop Header: Depth=1
	add 27, 4, 11
	lvx 3, 4, 11
	addi 28, 28, -8
	cmplwi	28, 0
	lvx 4, 27, 12
	add 27, 10, 11
	lvsl 2, 4, 11
	vperm 2, 3, 4, 2
	lvx 3, 10, 11
	lvx 4, 27, 12
	add 27, 5, 11
	lvsl 5, 10, 11
	vperm 3, 3, 4, 5
	lvx 0, 5, 11
	lvx 4, 27, 12
	add 27, 9, 11
	lvsl 5, 5, 11
	vperm 4, 0, 4, 5
	lvx 5, 9, 11
	lvx 0, 27, 12
	add 27, 3, 11
	lvsl 1, 9, 11
	vmaxsw 2, 2, 4
	lvsr 4, 3, 11
	vperm 5, 5, 0, 1
	vperm 2, 2, 2, 4
	lvsr 4, 8, 11
	stvewx 2, 27, 0
	stvewx 2, 27, 30
	vmaxsw 3, 3, 5
	stvewx 2, 27, 29
	add 27, 8, 11
	stvewx 2, 3, 11
	vperm 2, 3, 3, 4
	stvewx 2, 8, 11
	addi 11, 11, 32
	stvewx 2, 27, 0
	stvewx 2, 27, 30
	stvewx 2, 27, 29
	bne	0, .Llfa_BB20_3
# %bb.4:                                # %middle.block
	cmplw	6, 7
	bne	0, .Llfa_BB20_6
.Llfa_BB20_5:
	ld 30, 40(1)                            # 8-byte Folded Reload
	ld 29, 32(1)                            # 8-byte Folded Reload
	ld 28, 24(1)                            # 8-byte Folded Reload
	ld 27, 16(1)                            # 8-byte Folded Reload
	addi 1, 1, 64
	blr
.Llfa_BB20_6:                               # %for.body.preheader20
	slwi 8, 7, 2
	sub	6, 6, 7
	addi 8, 8, -4
	add 4, 4, 8
	add 5, 5, 8
	add 3, 3, 8
	b .Llfa_BB20_8
.Llfa_BB20_7:                               # %for.body
                                        #   in Loop: Header=BB20_8 Depth=1
	addi 6, 6, -1
	stwu 7, 4(3)
	cmplwi	6, 0
	beq	0, .Llfa_BB20_5
.Llfa_BB20_8:                               # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lwzu 7, 4(4)
	lwzu 8, 4(5)
	cmpw	7, 8
	bgt	0, .Llfa_BB20_7
# %bb.9:                                # %for.body
                                        #   in Loop: Header=BB20_8 Depth=1
	mr	7, 8
	b .Llfa_BB20_7
.Llfa_func_end20:
	.size	lfa_k_max_i32_r, .Llfa_func_end20-.Llfa_func_begin20
                                        # -- End function
	.section	.text.lfa_k_clamp_f32_r,"ax",@progbits
	.globl	lfa_k_clamp_f32_r               # -- Begin function lfa_k_clamp_f32_r
	.p2align	2
	.type	lfa_k_clamp_f32_r,@function
lfa_k_clamp_f32_r:                      # @lfa_k_clamp_f32_r
.Llfa_func_begin21:
# %bb.0:                                # %entry
	cmpwi	7, 1
	bltlr	0
# %bb.1:                                # %for.body.preheader
	addi 4, 4, -4
	addi 3, 3, -4
.Llfa_BB21_2:                               # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lfsu 0, 4(4)
	addi 7, 7, -1
	cmplwi	7, 0
	fsubs 3, 1, 0
	fsel 0, 3, 1, 0
	fsubs 3, 0, 2
	fsel 0, 3, 2, 0
	stfsu 0, 4(3)
	bne	0, .Llfa_BB21_2
# %bb.3:                                # %for.end
	blr
.Llfa_func_end21:
	.size	lfa_k_clamp_f32_r, .Llfa_func_end21-.Llfa_func_begin21
                                        # -- End function
	.section	.rodata.cst16,"aM",@progbits,16
	.p2align	4, 0x0                          # -- Begin function lfa_k_clamp_i16_r
.Llfa_CPI22_0:
	.short	64536                           # 0xfc18
	.short	64536                           # 0xfc18
	.short	64536                           # 0xfc18
	.short	64536                           # 0xfc18
	.short	64536                           # 0xfc18
	.short	64536                           # 0xfc18
	.short	64536                           # 0xfc18
	.short	64536                           # 0xfc18
.Llfa_CPI22_1:
	.short	1000                            # 0x3e8
	.short	1000                            # 0x3e8
	.short	1000                            # 0x3e8
	.short	1000                            # 0x3e8
	.short	1000                            # 0x3e8
	.short	1000                            # 0x3e8
	.short	1000                            # 0x3e8
	.short	1000                            # 0x3e8
	.section	.text.lfa_k_clamp_i16_r,"ax",@progbits
	.globl	lfa_k_clamp_i16_r
	.p2align	2
	.type	lfa_k_clamp_i16_r,@function
lfa_k_clamp_i16_r:                      # @lfa_k_clamp_i16_r
.Llfa_func_begin22:
# %bb.0:                                # %entry
	cmpwi	5, 1
	bltlr	0
# %bb.1:                                # %iter.check
	stwu 1, -80(1)
	cmplwi	5, 8
	li 6, 0
	std 25, 16(1)                           # 8-byte Folded Spill
	std 26, 24(1)                           # 8-byte Folded Spill
	std 27, 32(1)                           # 8-byte Folded Spill
	std 28, 40(1)                           # 8-byte Folded Spill
	std 29, 48(1)                           # 8-byte Folded Spill
	std 30, 56(1)                           # 8-byte Folded Spill
	blt	0, .Llfa_BB22_11
# %bb.2:                                # %vector.main.loop.iter.check
	cmplwi	5, 16
	bge 0, .Llfa_BB22_6
.Llfa_BB22_3:                               # %vec.epilog.ph
	li 30, .Llfa_CPI22_0@l
	lis 29, .Llfa_CPI22_0@ha
	lvx 2, 29, 30
	li 28, .Llfa_CPI22_1@l
	lis 27, .Llfa_CPI22_1@ha
	lvx 3, 27, 28
	mr	0, 6
	rlwinm 6, 5, 0, 1, 28
	li 7, 15
	li 8, 14
	li 9, 12
	li 10, 10
	li 11, 8
	li 12, 6
	sub	30, 0, 6
	slwi 29, 0, 1
	li 0, 4
	li 28, 2
.Llfa_BB22_4:                               # %vec.epilog.vector.body
                                        # =>This Inner Loop Header: Depth=1
	add 27, 4, 29
	lvx 4, 4, 29
	addi 30, 30, 8
	cmplwi	30, 0
	lvx 5, 27, 7
	add 27, 3, 29
	lvsl 0, 4, 29
	vperm 4, 4, 5, 0
	lvsr 5, 3, 29
	vmaxsh 4, 4, 2
	vminsh 4, 4, 3
	vperm 4, 4, 4, 5
	stvehx 4, 3, 29
	addi 29, 29, 16
	stvehx 4, 27, 8
	stvehx 4, 27, 9
	stvehx 4, 27, 10
	stvehx 4, 27, 11
	stvehx 4, 27, 12
	stvehx 4, 27, 0
	stvehx 4, 27, 28
	bne	0, .Llfa_BB22_4
# %bb.5:                                # %vec.epilog.middle.block
	cmplw	5, 6
	beq	0, .Llfa_BB22_9
	b .Llfa_BB22_11
.Llfa_BB22_6:                               # %vector.ph
	li 30, .Llfa_CPI22_0@l
	lis 29, .Llfa_CPI22_0@ha
	lvx 2, 29, 30
	li 28, .Llfa_CPI22_1@l
	lis 27, .Llfa_CPI22_1@ha
	lvx 3, 27, 28
	rlwinm 6, 5, 0, 1, 27
	addi 7, 3, 16
	addi 8, 4, 16
	li 9, 0
	li 10, 15
	li 11, 14
	li 12, 12
	li 0, 10
	li 30, 8
	li 29, 6
	li 28, 4
	li 27, 2
	mr	26, 6
.Llfa_BB22_7:                               # %vector.body
                                        # =>This Inner Loop Header: Depth=1
	add 25, 4, 9
	lvx 4, 4, 9
	addi 26, 26, -16
	cmplwi	26, 0
	lvx 5, 25, 10
	add 25, 8, 9
	lvsl 0, 4, 9
	vperm 4, 4, 5, 0
	lvx 5, 8, 9
	lvx 0, 25, 10
	add 25, 3, 9
	lvsl 1, 8, 9
	vmaxsh 4, 4, 2
	vperm 5, 5, 0, 1
	lvsr 0, 3, 9
	lvsr 1, 7, 9
	vminsh 4, 4, 3
	vmaxsh 5, 5, 2
	vperm 4, 4, 4, 0
	stvehx 4, 25, 11
	stvehx 4, 25, 12
	stvehx 4, 25, 0
	vminsh 5, 5, 3
	stvehx 4, 25, 30
	stvehx 4, 25, 29
	stvehx 4, 25, 28
	stvehx 4, 25, 27
	add 25, 7, 9
	stvehx 4, 3, 9
	vperm 4, 5, 5, 1
	stvehx 4, 7, 9
	addi 9, 9, 32
	stvehx 4, 25, 11
	stvehx 4, 25, 12
	stvehx 4, 25, 0
	stvehx 4, 25, 30
	stvehx 4, 25, 29
	stvehx 4, 25, 28
	stvehx 4, 25, 27
	bne	0, .Llfa_BB22_7
# %bb.8:                                # %middle.block
	cmplw	5, 6
	bne	0, .Llfa_BB22_10
.Llfa_BB22_9:
	ld 30, 56(1)                            # 8-byte Folded Reload
	ld 29, 48(1)                            # 8-byte Folded Reload
	ld 28, 40(1)                            # 8-byte Folded Reload
	ld 27, 32(1)                            # 8-byte Folded Reload
	ld 26, 24(1)                            # 8-byte Folded Reload
	ld 25, 16(1)                            # 8-byte Folded Reload
	addi 1, 1, 80
	blr
.Llfa_BB22_10:                              # %vec.epilog.iter.check
	andi. 7, 5, 8
	bne	0, .Llfa_BB22_3
.Llfa_BB22_11:                              # %for.body.preheader
	slwi 7, 6, 1
	sub	5, 5, 6
	addi 6, 7, -2
	add 4, 4, 6
	add 3, 3, 6
	b .Llfa_BB22_13
.Llfa_BB22_12:                              # %for.body
                                        #   in Loop: Header=BB22_13 Depth=1
	addi 5, 5, -1
	sthu 6, 2(3)
	cmplwi	5, 0
	beq	0, .Llfa_BB22_9
.Llfa_BB22_13:                              # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lhzu 6, 2(4)
	extsh 6, 6
	cmpwi	6, -1000
	ble 0, .Llfa_BB22_15
# %bb.14:                               # %for.body
                                        #   in Loop: Header=BB22_13 Depth=1
	cmpwi	6, 1000
	blt	0, .Llfa_BB22_12
	b .Llfa_BB22_16
.Llfa_BB22_15:                              # %for.body
                                        #   in Loop: Header=BB22_13 Depth=1
	li 6, -1000
	cmpwi	6, 1000
	blt	0, .Llfa_BB22_12
.Llfa_BB22_16:                              # %for.body
                                        #   in Loop: Header=BB22_13 Depth=1
	li 6, 1000
	b .Llfa_BB22_12
.Llfa_func_end22:
	.size	lfa_k_clamp_i16_r, .Llfa_func_end22-.Llfa_func_begin22
                                        # -- End function
	.section	.text.lfa_k_abs_f32_r,"ax",@progbits
	.globl	lfa_k_abs_f32_r                 # -- Begin function lfa_k_abs_f32_r
	.p2align	2
	.type	lfa_k_abs_f32_r,@function
lfa_k_abs_f32_r:                        # @lfa_k_abs_f32_r
.Llfa_func_begin23:
# %bb.0:                                # %entry
	cmpwi	5, 1
	bltlr	0
# %bb.1:                                # %for.body.preheader
	li 6, 0
	beq	0, .Llfa_BB23_5
# %bb.2:                                # %vector.ph
	rlwinm 6, 5, 0, 1, 30
	addi 7, 4, -8
	addi 8, 3, -8
	mr	9, 6
.Llfa_BB23_3:                               # %vector.body
                                        # =>This Inner Loop Header: Depth=1
	lfsu 0, 8(7)
	addi 9, 9, -2
	cmplwi	9, 0
	fabs 0, 0
	stfsu 0, 8(8)
	lfs 0, 4(7)
	fabs 0, 0
	stfs 0, 4(8)
	bne	0, .Llfa_BB23_3
# %bb.4:                                # %middle.block
	cmplw	5, 6
	beqlr	0
.Llfa_BB23_5:                               # %for.body.preheader10
	slwi 7, 6, 2
	sub	5, 5, 6
	addi 6, 7, -4
	add 4, 4, 6
	add 3, 3, 6
.Llfa_BB23_6:                               # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lfsu 0, 4(4)
	addi 5, 5, -1
	cmplwi	5, 0
	fabs 0, 0
	stfsu 0, 4(3)
	bne	0, .Llfa_BB23_6
	blr
.Llfa_func_end23:
	.size	lfa_k_abs_f32_r, .Llfa_func_end23-.Llfa_func_begin23
                                        # -- End function
	.section	.text.lfa_k_abs_i32_r,"ax",@progbits
	.globl	lfa_k_abs_i32_r                 # -- Begin function lfa_k_abs_i32_r
	.p2align	2
	.type	lfa_k_abs_i32_r,@function
lfa_k_abs_i32_r:                        # @lfa_k_abs_i32_r
.Llfa_func_begin24:
# %bb.0:                                # %entry
	cmpwi	5, 1
	bltlr	0
# %bb.1:                                # %for.body.preheader
	stwu 1, -48(1)
	cmplwi	5, 8
	li 6, 0
	std 29, 16(1)                           # 8-byte Folded Spill
	std 30, 24(1)                           # 8-byte Folded Spill
	blt	0, .Llfa_BB24_5
# %bb.2:                                # %vector.ph
	rlwinm 6, 5, 0, 1, 28
	addi 7, 3, 16
	vxor 2, 2, 2
	addi 8, 4, 16
	li 9, 0
	li 10, 15
	li 11, 12
	li 12, 8
	li 0, 4
	mr	30, 6
.Llfa_BB24_3:                               # %vector.body
                                        # =>This Inner Loop Header: Depth=1
	add 29, 4, 9
	lvx 3, 4, 9
	addi 30, 30, -8
	cmplwi	30, 0
	lvx 4, 29, 10
	add 29, 8, 9
	lvsl 5, 4, 9
	vperm 3, 3, 4, 5
	lvx 4, 8, 9
	lvx 5, 29, 10
	add 29, 3, 9
	lvsl 0, 8, 9
	vperm 4, 4, 5, 0
	vsubuwm 5, 2, 3
	vsubuwm 0, 2, 4
	vmaxsw 3, 3, 5
	lvsr 5, 3, 9
	vperm 3, 3, 3, 5
	lvsr 5, 7, 9
	stvewx 3, 29, 11
	stvewx 3, 29, 12
	vmaxsw 4, 4, 0
	stvewx 3, 29, 0
	add 29, 7, 9
	stvewx 3, 3, 9
	vperm 3, 4, 4, 5
	stvewx 3, 7, 9
	addi 9, 9, 32
	stvewx 3, 29, 11
	stvewx 3, 29, 12
	stvewx 3, 29, 0
	bne	0, .Llfa_BB24_3
# %bb.4:                                # %middle.block
	cmplw	5, 6
	beq	0, .Llfa_BB24_7
.Llfa_BB24_5:                               # %for.body.preheader11
	slwi 7, 6, 2
	sub	5, 5, 6
	addi 6, 7, -4
	add 4, 4, 6
	add 3, 3, 6
.Llfa_BB24_6:                               # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lwzu 6, 4(4)
	addi 5, 5, -1
	cmplwi	5, 0
	srawi 7, 6, 31
	xor 6, 6, 7
	sub	6, 6, 7
	stwu 6, 4(3)
	bne	0, .Llfa_BB24_6
.Llfa_BB24_7:
	ld 30, 24(1)                            # 8-byte Folded Reload
	ld 29, 16(1)                            # 8-byte Folded Reload
	addi 1, 1, 48
	blr
.Llfa_func_end24:
	.size	lfa_k_abs_i32_r, .Llfa_func_end24-.Llfa_func_begin24
                                        # -- End function
	.section	.text.lfa_k_f2i_r,"ax",@progbits
	.globl	lfa_k_f2i_r                     # -- Begin function lfa_k_f2i_r
	.p2align	2
	.type	lfa_k_f2i_r,@function
lfa_k_f2i_r:                            # @lfa_k_f2i_r
.Llfa_func_begin25:
# %bb.0:                                # %entry
	cmpwi	5, 1
	bltlr	0
# %bb.1:                                # %for.body.preheader
	stwu 1, -48(1)
	cmplwi	5, 8
	li 6, 0
	std 29, 16(1)                           # 8-byte Folded Spill
	std 30, 24(1)                           # 8-byte Folded Spill
	blt	0, .Llfa_BB25_5
# %bb.2:                                # %vector.ph
	rlwinm 6, 5, 0, 1, 28
	addi 7, 3, 16
	addi 8, 4, 16
	li 9, 0
	li 10, 15
	li 11, 12
	li 12, 8
	li 0, 4
	mr	30, 6
.Llfa_BB25_3:                               # %vector.body
                                        # =>This Inner Loop Header: Depth=1
	add 29, 4, 9
	lvx 2, 4, 9
	addi 30, 30, -8
	cmplwi	30, 0
	lvx 3, 29, 10
	add 29, 8, 9
	lvsl 4, 4, 9
	vperm 2, 2, 3, 4
	lvx 3, 8, 9
	lvx 4, 29, 10
	add 29, 3, 9
	lvsl 5, 8, 9
	vctsxs 2, 2, 0
	vperm 3, 3, 4, 5
	lvsr 4, 3, 9
	vperm 2, 2, 2, 4
	lvsr 4, 7, 9
	vctsxs 3, 3, 0
	stvewx 2, 29, 11
	stvewx 2, 29, 12
	stvewx 2, 29, 0
	add 29, 7, 9
	stvewx 2, 3, 9
	vperm 2, 3, 3, 4
	stvewx 2, 7, 9
	addi 9, 9, 32
	stvewx 2, 29, 11
	stvewx 2, 29, 12
	stvewx 2, 29, 0
	bne	0, .Llfa_BB25_3
# %bb.4:                                # %middle.block
	cmplw	5, 6
	beq	0, .Llfa_BB25_7
.Llfa_BB25_5:                               # %for.body.preheader8
	slwi 7, 6, 2
	sub	5, 5, 6
	addi 6, 7, -4
	add 4, 4, 6
	add 3, 3, 6
.Llfa_BB25_6:                               # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lfsu 0, 4(4)
	addi 5, 5, -1
	addi 3, 3, 4
	cmplwi	5, 0
	fctiwz 0, 0
	stfiwx 0, 0, 3
	bne	0, .Llfa_BB25_6
.Llfa_BB25_7:
	ld 30, 24(1)                            # 8-byte Folded Reload
	ld 29, 16(1)                            # 8-byte Folded Reload
	addi 1, 1, 48
	blr
.Llfa_func_end25:
	.size	lfa_k_f2i_r, .Llfa_func_end25-.Llfa_func_begin25
                                        # -- End function
	.section	.text.lfa_k_i2f_r,"ax",@progbits
	.globl	lfa_k_i2f_r                     # -- Begin function lfa_k_i2f_r
	.p2align	2
	.type	lfa_k_i2f_r,@function
lfa_k_i2f_r:                            # @lfa_k_i2f_r
.Llfa_func_begin26:
# %bb.0:                                # %entry
	cmpwi	5, 1
	bltlr	0
# %bb.1:                                # %for.body.preheader
	stwu 1, -64(1)
	cmplwi	5, 8
	li 6, 0
	std 29, 32(1)                           # 8-byte Folded Spill
	std 30, 40(1)                           # 8-byte Folded Spill
	blt	0, .Llfa_BB26_5
# %bb.2:                                # %vector.ph
	rlwinm 6, 5, 0, 1, 28
	addi 7, 3, 16
	addi 8, 4, 16
	li 9, 0
	li 10, 15
	li 11, 12
	li 12, 8
	li 0, 4
	mr	30, 6
.Llfa_BB26_3:                               # %vector.body
                                        # =>This Inner Loop Header: Depth=1
	add 29, 4, 9
	lvx 2, 4, 9
	addi 30, 30, -8
	cmplwi	30, 0
	lvx 3, 29, 10
	add 29, 8, 9
	lvsl 4, 4, 9
	vperm 2, 2, 3, 4
	lvx 3, 8, 9
	lvx 4, 29, 10
	add 29, 3, 9
	lvsl 5, 8, 9
	vcfsx 2, 2, 0
	vperm 3, 3, 4, 5
	lvsr 4, 3, 9
	vperm 2, 2, 2, 4
	lvsr 4, 7, 9
	stvewx 2, 29, 11
	stvewx 2, 29, 12
	vcfsx 3, 3, 0
	stvewx 2, 29, 0
	add 29, 7, 9
	stvewx 2, 3, 9
	vperm 2, 3, 3, 4
	stvewx 2, 7, 9
	addi 9, 9, 32
	stvewx 2, 29, 11
	stvewx 2, 29, 12
	stvewx 2, 29, 0
	bne	0, .Llfa_BB26_3
# %bb.4:                                # %middle.block
	cmplw	5, 6
	beq	0, .Llfa_BB26_7
.Llfa_BB26_5:                               # %for.body.preheader8
	slwi 7, 6, 2
	sub	5, 5, 6
	addi 6, 7, -4
	add 4, 4, 6
	add 3, 3, 6
.Llfa_BB26_6:                               # %for.body
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
	bne	0, .Llfa_BB26_6
.Llfa_BB26_7:
	ld 30, 40(1)                            # 8-byte Folded Reload
	ld 29, 32(1)                            # 8-byte Folded Reload
	addi 1, 1, 64
	blr
.Llfa_func_end26:
	.size	lfa_k_i2f_r, .Llfa_func_end26-.Llfa_func_begin26
                                        # -- End function
	.section	.rodata.cst16,"aM",@progbits,16
	.p2align	4, 0x0                          # -- Begin function lfa_k_u8_to_f32_r
.Llfa_CPI27_0:
	.long	0x3b808081                      # float 0.00392156886
	.long	0x3b808081                      # float 0.00392156886
	.long	0x3b808081                      # float 0.00392156886
	.long	0x3b808081                      # float 0.00392156886
	.section	.rodata.cst4,"aM",@progbits,4
	.p2align	2, 0x0
.Llfa_CPI27_1:
	.long	0x3b808081                      # float 0.00392156886
	.section	.text.lfa_k_u8_to_f32_r,"ax",@progbits
	.globl	lfa_k_u8_to_f32_r
	.p2align	2
	.type	lfa_k_u8_to_f32_r,@function
lfa_k_u8_to_f32_r:                      # @lfa_k_u8_to_f32_r
.Llfa_func_begin27:
# %bb.0:                                # %entry
	cmpwi	5, 1
	bltlr	0
# %bb.1:                                # %for.body.preheader
	stwu 1, -128(1)
	cmplwi	5, 4
	li 6, 0
	std 27, 80(1)                           # 8-byte Folded Spill
	std 28, 88(1)                           # 8-byte Folded Spill
	std 29, 96(1)                           # 8-byte Folded Spill
	std 30, 104(1)                          # 8-byte Folded Spill
	blt	0, .Llfa_BB27_5
# %bb.2:                                # %vector.ph
	li 9, .Llfa_CPI27_0@l
	lis 10, .Llfa_CPI27_0@ha
	vspltisw 3, -1
	lvx 2, 10, 9
	rlwinm 6, 5, 0, 1, 29
	addi 7, 1, 64
	li 8, 12
	li 9, 8
	li 10, 4
	mr	11, 4
	vslw 3, 3, 3
	mr	12, 3
	mr	30, 6
.Llfa_BB27_3:                               # %vector.body
                                        # =>This Inner Loop Header: Depth=1
	lbz 0, 0(11)
	addi 30, 30, -4
	cmplwi	30, 0
	clrlwi	0, 0, 24
	lbz 29, 1(11)
	lbz 28, 2(11)
	lbz 27, 3(11)
	addi 11, 11, 4
	std 0, 32(1)
	clrlwi	0, 29, 24
	lfd 0, 32(1)
	fcfid 0, 0
	std 0, 40(1)
	clrlwi	0, 28, 24
	frsp 0, 0
	stfs 0, 64(1)
	lfd 0, 40(1)
	fcfid 0, 0
	std 0, 48(1)
	clrlwi	0, 27, 24
	frsp 0, 0
	stfs 0, 68(1)
	lfd 0, 48(1)
	fcfid 0, 0
	std 0, 56(1)
	frsp 0, 0
	stfs 0, 72(1)
	lfd 0, 56(1)
	fcfid 0, 0
	lvsr 5, 0, 12
	frsp 0, 0
	stfs 0, 76(1)
	lvx 4, 0, 7
	vmaddfp 4, 4, 2, 3
	vperm 4, 4, 4, 5
	stvewx 4, 12, 8
	stvewx 4, 12, 9
	stvewx 4, 12, 10
	stvewx 4, 0, 12
	addi 12, 12, 16
	bne	0, .Llfa_BB27_3
# %bb.4:                                # %middle.block
	cmplw	5, 6
	beq	0, .Llfa_BB27_7
.Llfa_BB27_5:                               # %for.body.preheader7
	sub	5, 5, 6
	slwi 7, 6, 2
	add 4, 6, 4
	lis 6, .Llfa_CPI27_1@ha
	lfs 0, .Llfa_CPI27_1@l(6)
	add 7, 7, 3
	addi 3, 4, -1
	addi 4, 7, -4
.Llfa_BB27_6:                               # %for.body
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
	bne	0, .Llfa_BB27_6
.Llfa_BB27_7:
	ld 30, 104(1)                           # 8-byte Folded Reload
	ld 29, 96(1)                            # 8-byte Folded Reload
	ld 28, 88(1)                            # 8-byte Folded Reload
	ld 27, 80(1)                            # 8-byte Folded Reload
	addi 1, 1, 128
	blr
.Llfa_func_end27:
	.size	lfa_k_u8_to_f32_r, .Llfa_func_end27-.Llfa_func_begin27
                                        # -- End function
	.section	.rodata.cst16,"aM",@progbits,16
	.p2align	4, 0x0                          # -- Begin function lfa_k_mat4_vec4_r
.Llfa_CPI28_0:
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
.Llfa_CPI28_1:
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
.Llfa_CPI28_2:
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
.Llfa_CPI28_3:
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
.Llfa_CPI28_4:
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
	.section	.text.lfa_k_mat4_vec4_r,"ax",@progbits
	.globl	lfa_k_mat4_vec4_r
	.p2align	2
	.type	lfa_k_mat4_vec4_r,@function
lfa_k_mat4_vec4_r:                      # @lfa_k_mat4_vec4_r
.Llfa_func_begin28:
# %bb.0:                                # %entry
	cmpwi	6, 1
	bltlr	0
# %bb.1:                                # %for.body.lr.ph
	stwu 1, -256(1)
	std 19, 144(1)                          # 8-byte Folded Spill
	li 7, 16
	li 8, 32
	li 9, 48
	li 10, 63
	std 20, 152(1)                          # 8-byte Folded Spill
	cmplwi	6, 4
	std 21, 160(1)                          # 8-byte Folded Spill
	std 22, 168(1)                          # 8-byte Folded Spill
	std 23, 176(1)                          # 8-byte Folded Spill
	std 24, 184(1)                          # 8-byte Folded Spill
	std 25, 192(1)                          # 8-byte Folded Spill
	std 26, 200(1)                          # 8-byte Folded Spill
	std 27, 208(1)                          # 8-byte Folded Spill
	std 28, 216(1)                          # 8-byte Folded Spill
	std 29, 224(1)                          # 8-byte Folded Spill
	std 30, 232(1)                          # 8-byte Folded Spill
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
	blt	0, .Llfa_BB28_5
# %bb.2:                                # %vector.ph
	vspltw 0, 2, 0
	li 4, 64
	li 12, .Llfa_CPI28_0@l
	vspltisw 26, -1
	stvx 0, 1, 4                            # 16-byte Folded Spill
	li 4, 48
	lis 30, .Llfa_CPI28_0@ha
	vspltw 0, 3, 0
	li 29, .Llfa_CPI28_1@l
	lis 28, .Llfa_CPI28_1@ha
	stvx 0, 1, 4                            # 16-byte Folded Spill
	li 26, .Llfa_CPI28_2@l
	lis 25, .Llfa_CPI28_2@ha
	vspltw 0, 4, 0
	lvx 16, 30, 12
	li 24, .Llfa_CPI28_3@l
	lis 23, .Llfa_CPI28_3@ha
	vslw 26, 26, 26
	vspltw 7, 5, 0
	li 22, .Llfa_CPI28_4@l
	lis 21, .Llfa_CPI28_4@ha
	lvx 18, 28, 29
	li 4, 32
	vspltw 8, 2, 1
	li 11, 0
	lvx 31, 25, 26
	li 12, 60
	li 0, 56
	vspltw 9, 3, 1
	lvx 29, 23, 24
	li 30, 52
	li 29, 44
	vspltw 10, 4, 1
	li 28, 40
	li 27, 36
	lvx 27, 21, 22
	li 26, 28
	li 25, 24
	vspltw 11, 5, 1
	stvx 0, 1, 4                            # 16-byte Folded Spill
	rlwinm 4, 6, 0, 1, 29
	li 24, 20
	vspltw 12, 2, 2
	li 23, 12
	li 22, 8
	li 21, 4
	mr	20, 4
	vspltw 13, 3, 2
	vspltw 14, 4, 2
	vspltw 15, 5, 2
	vspltw 17, 2, 3
	vspltw 19, 3, 3
	vspltw 30, 4, 3
	vspltw 28, 5, 3
.Llfa_BB28_3:                               # %vector.body
                                        # =>This Inner Loop Header: Depth=1
	add 19, 5, 11
	lvsl 23, 5, 11
	addi 20, 20, -4
	cmplwi	20, 0
	lvx 25, 19, 8
	lvx 24, 19, 7
	lvx 21, 5, 11
	vperm 22, 24, 25, 23
	vperm 24, 21, 24, 23
	lvx 21, 19, 10
	lvx 20, 19, 9
	li 19, 64
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
	lvx 25, 1, 19                           # 16-byte Folded Reload
	li 19, 48
	lvx 21, 1, 19                           # 16-byte Folded Reload
	li 19, 32
	vperm 23, 20, 23, 18
	vmaddfp 25, 25, 23, 26
	vmaddfp 24, 8, 23, 26
	vmaddfp 22, 12, 23, 26
	vmaddfp 23, 17, 23, 26
	vmaddfp 25, 21, 0, 25
	vmaddfp 24, 9, 0, 24
	vmaddfp 22, 13, 0, 22
	vmaddfp 0, 19, 0, 23
	lvx 23, 1, 19                           # 16-byte Folded Reload
	add 19, 3, 11
	vmaddfp 25, 23, 1, 25
	vmaddfp 24, 10, 1, 24
	vmaddfp 23, 14, 1, 22
	vmaddfp 0, 30, 1, 0
	vmaddfp 1, 7, 6, 25
	vmaddfp 25, 11, 6, 24
	vmaddfp 24, 15, 6, 23
	vmaddfp 0, 28, 6, 0
	vperm 6, 24, 0, 16
	vmrghw 23, 1, 25
	vperm 22, 1, 25, 31
	vmrghw 21, 24, 0
	vmrglw 20, 1, 25
	vperm 1, 1, 25, 27
	vperm 25, 24, 0, 29
	vmrglw 0, 24, 0
	vperm 24, 22, 21, 18
	lvsr 22, 19, 9
	lvsr 21, 19, 8
	vperm 0, 1, 0, 18
	lvsr 1, 3, 11
	vperm 0, 0, 0, 22
	lvsr 22, 19, 7
	stvewx 0, 19, 12
	stvewx 0, 19, 0
	vperm 25, 20, 25, 18
	stvewx 0, 19, 30
	stvewx 0, 19, 9
	vperm 0, 25, 25, 21
	stvewx 0, 19, 29
	stvewx 0, 19, 28
	stvewx 0, 19, 27
	vperm 25, 24, 24, 22
	stvewx 0, 19, 8
	stvewx 25, 19, 26
	vperm 0, 23, 6, 18
	stvewx 25, 19, 25
	stvewx 25, 19, 24
	stvewx 25, 19, 7
	vperm 0, 0, 0, 1
	stvewx 0, 3, 11
	addi 11, 11, 64
	stvewx 0, 19, 23
	stvewx 0, 19, 22
	stvewx 0, 19, 21
	bne	0, .Llfa_BB28_3
# %bb.4:                                # %middle.block
	cmplw	6, 4
	beq	0, .Llfa_BB28_7
.Llfa_BB28_5:                               # %for.body.preheader
	slwi 11, 4, 4
	vspltisw 0, -1
	sub	6, 6, 4
	add 5, 11, 5
	addi 4, 1, 80
	addi 7, 1, 96
	addi 8, 1, 112
	addi 9, 1, 128
	li 10, 12
	add 3, 3, 11
	addi 5, 5, -16
	vslw 0, 0, 0
	li 11, 8
	li 12, 4
.Llfa_BB28_6:                               # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lwzu 0, 16(5)
	addi 6, 6, -1
	cmplwi	6, 0
	lwz 30, 4(5)
	lwz 29, 8(5)
	lwz 28, 12(5)
	stw 0, 80(1)
	lvx 1, 0, 4
	stw 30, 96(1)
	vspltw 1, 1, 0
	vmaddfp 1, 2, 1, 0
	lvx 6, 0, 7
	vspltw 6, 6, 0
	stw 29, 112(1)
	vmaddfp 1, 3, 6, 1
	lvx 6, 0, 8
	vspltw 6, 6, 0
	stw 28, 128(1)
	vmaddfp 1, 4, 6, 1
	lvx 6, 0, 9
	vspltw 6, 6, 0
	vmaddfp 1, 5, 6, 1
	lvsr 6, 0, 3
	vperm 1, 1, 1, 6
	stvewx 1, 3, 10
	stvewx 1, 3, 11
	stvewx 1, 3, 12
	stvewx 1, 0, 3
	addi 3, 3, 16
	bne	0, .Llfa_BB28_6
.Llfa_BB28_7:
	ld 30, 232(1)                           # 8-byte Folded Reload
	ld 29, 224(1)                           # 8-byte Folded Reload
	ld 28, 216(1)                           # 8-byte Folded Reload
	ld 27, 208(1)                           # 8-byte Folded Reload
	ld 26, 200(1)                           # 8-byte Folded Reload
	ld 25, 192(1)                           # 8-byte Folded Reload
	ld 24, 184(1)                           # 8-byte Folded Reload
	ld 23, 176(1)                           # 8-byte Folded Reload
	ld 22, 168(1)                           # 8-byte Folded Reload
	ld 21, 160(1)                           # 8-byte Folded Reload
	ld 20, 152(1)                           # 8-byte Folded Reload
	ld 19, 144(1)                           # 8-byte Folded Reload
	addi 1, 1, 256
	blr
.Llfa_func_end28:
	.size	lfa_k_mat4_vec4_r, .Llfa_func_end28-.Llfa_func_begin28
                                        # -- End function
	.section	.text.lfa_k_copy_u32_r,"ax",@progbits
	.globl	lfa_k_copy_u32_r                # -- Begin function lfa_k_copy_u32_r
	.p2align	2
	.type	lfa_k_copy_u32_r,@function
lfa_k_copy_u32_r:                       # @lfa_k_copy_u32_r
.Llfa_func_begin29:
# %bb.0:                                # %entry
	cmpwi	5, 1
	bltlr	0
# %bb.1:                                # %for.body.preheader
	stwu 1, -48(1)
	cmplwi	5, 8
	li 6, 0
	std 29, 16(1)                           # 8-byte Folded Spill
	std 30, 24(1)                           # 8-byte Folded Spill
	blt	0, .Llfa_BB29_5
# %bb.2:                                # %vector.ph
	rlwinm 6, 5, 0, 1, 28
	addi 7, 3, 16
	addi 8, 4, 16
	li 9, 0
	li 10, 15
	li 11, 12
	li 12, 8
	li 0, 4
	mr	30, 6
.Llfa_BB29_3:                               # %vector.body
                                        # =>This Inner Loop Header: Depth=1
	add 29, 4, 9
	lvx 3, 4, 9
	addi 30, 30, -8
	cmplwi	30, 0
	lvx 4, 29, 10
	add 29, 8, 9
	lvsl 2, 4, 9
	vperm 2, 3, 4, 2
	lvsr 3, 3, 9
	lvx 0, 8, 9
	vperm 2, 2, 2, 3
	lvx 3, 29, 10
	add 29, 3, 9
	lvsl 5, 8, 9
	lvsr 4, 7, 9
	vperm 3, 0, 3, 5
	stvewx 2, 29, 11
	stvewx 2, 29, 12
	stvewx 2, 29, 0
	add 29, 7, 9
	stvewx 2, 3, 9
	vperm 2, 3, 3, 4
	stvewx 2, 7, 9
	addi 9, 9, 32
	stvewx 2, 29, 11
	stvewx 2, 29, 12
	stvewx 2, 29, 0
	bne	0, .Llfa_BB29_3
# %bb.4:                                # %middle.block
	cmplw	5, 6
	beq	0, .Llfa_BB29_7
.Llfa_BB29_5:                               # %for.body.preheader8
	slwi 7, 6, 2
	sub	5, 5, 6
	addi 6, 7, -4
	add 4, 4, 6
	add 3, 3, 6
.Llfa_BB29_6:                               # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lwzu 6, 4(4)
	addi 5, 5, -1
	cmplwi	5, 0
	stwu 6, 4(3)
	bne	0, .Llfa_BB29_6
.Llfa_BB29_7:
	ld 30, 24(1)                            # 8-byte Folded Reload
	ld 29, 16(1)                            # 8-byte Folded Reload
	addi 1, 1, 48
	blr
.Llfa_func_end29:
	.size	lfa_k_copy_u32_r, .Llfa_func_end29-.Llfa_func_begin29
                                        # -- End function
	.section	.text.lfa_k_copy_u8,"ax",@progbits
	.globl	lfa_k_copy_u8                   # -- Begin function lfa_k_copy_u8
	.p2align	2
	.type	lfa_k_copy_u8,@function
lfa_k_copy_u8:                          # @lfa_k_copy_u8
.Llfa_func_begin30:
# %bb.0:                                # %entry
	cmpwi	5, 1
	bltlr	0
# %bb.1:                                # %iter.check
	stwu 1, -128(1)
	cmplwi	5, 8
	sub	7, 3, 4
	std 21, 32(1)                           # 8-byte Folded Spill
	li 6, 0
	std 22, 40(1)                           # 8-byte Folded Spill
	std 23, 48(1)                           # 8-byte Folded Spill
	std 24, 56(1)                           # 8-byte Folded Spill
	std 25, 64(1)                           # 8-byte Folded Spill
	std 26, 72(1)                           # 8-byte Folded Spill
	std 27, 80(1)                           # 8-byte Folded Spill
	std 28, 88(1)                           # 8-byte Folded Spill
	std 29, 96(1)                           # 8-byte Folded Spill
	std 30, 104(1)                          # 8-byte Folded Spill
	bc 12, 0, .Llfa_BB30_11
# %bb.2:                                # %iter.check
	cmplwi	7, 16
	bc 12, 0, .Llfa_BB30_11
# %bb.3:                                # %vector.main.loop.iter.check
	cmplwi	5, 16
	bge 0, .Llfa_BB30_7
.Llfa_BB30_4:                               # %vec.epilog.ph
	mr	7, 6
	rlwinm 6, 5, 0, 1, 28
	addi 8, 1, 16
	li 9, 7
	li 10, 6
	li 11, 5
	li 12, 4
	li 0, 3
	li 30, 2
	li 29, 1
.Llfa_BB30_5:                               # %vec.epilog.vector.body
                                        # =>This Inner Loop Header: Depth=1
	lbzx 28, 4, 7
	stb 28, 16(1)
	add 28, 4, 7
	lbz 27, 1(28)
	stb 27, 17(1)
	lbz 27, 2(28)
	stb 27, 18(1)
	lbz 27, 3(28)
	stb 27, 19(1)
	lbz 27, 4(28)
	stb 27, 20(1)
	lbz 27, 5(28)
	stb 27, 21(1)
	lbz 27, 6(28)
	stb 27, 22(1)
	lbz 28, 7(28)
	stb 28, 23(1)
	add 28, 3, 7
	lvx 2, 0, 8
	lvsr 3, 3, 7
	vperm 2, 2, 2, 3
	stvebx 2, 3, 7
	addi 7, 7, 8
	cmplw	6, 7
	stvebx 2, 28, 9
	stvebx 2, 28, 10
	stvebx 2, 28, 11
	stvebx 2, 28, 12
	stvebx 2, 28, 0
	stvebx 2, 28, 30
	stvebx 2, 28, 29
	bne	0, .Llfa_BB30_5
# %bb.6:                                # %vec.epilog.middle.block
	cmplw	5, 6
	bne	0, .Llfa_BB30_11
	b .Llfa_BB30_13
.Llfa_BB30_7:                               # %vector.ph
	rlwinm 6, 5, 0, 1, 27
	li 7, 0
	li 8, 15
	li 9, 14
	li 10, 13
	li 11, 12
	li 12, 11
	li 0, 10
	li 30, 9
	li 29, 8
	li 28, 7
	li 27, 6
	li 26, 5
	li 25, 4
	li 24, 3
	li 23, 2
	li 22, 1
.Llfa_BB30_8:                               # %vector.body
                                        # =>This Inner Loop Header: Depth=1
	add 21, 4, 7
	lvx 2, 4, 7
	lvx 3, 21, 8
	add 21, 3, 7
	lvsl 4, 4, 7
	vperm 2, 2, 3, 4
	lvsr 3, 3, 7
	vperm 2, 2, 2, 3
	stvebx 2, 3, 7
	addi 7, 7, 16
	cmplw	6, 7
	stvebx 2, 21, 8
	stvebx 2, 21, 9
	stvebx 2, 21, 10
	stvebx 2, 21, 11
	stvebx 2, 21, 12
	stvebx 2, 21, 0
	stvebx 2, 21, 30
	stvebx 2, 21, 29
	stvebx 2, 21, 28
	stvebx 2, 21, 27
	stvebx 2, 21, 26
	stvebx 2, 21, 25
	stvebx 2, 21, 24
	stvebx 2, 21, 23
	stvebx 2, 21, 22
	bne	0, .Llfa_BB30_8
# %bb.9:                                # %middle.block
	cmplw	5, 6
	beq	0, .Llfa_BB30_13
# %bb.10:                               # %vec.epilog.iter.check
	andi. 7, 5, 8
	bne	0, .Llfa_BB30_4
.Llfa_BB30_11:                              # %for.body.preheader
	addi 7, 6, -1
	add 4, 4, 7
	add 3, 3, 7
.Llfa_BB30_12:                              # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lbzu 7, 1(4)
	addi 6, 6, 1
	cmplw	5, 6
	stbu 7, 1(3)
	bne	0, .Llfa_BB30_12
.Llfa_BB30_13:
	ld 30, 104(1)                           # 8-byte Folded Reload
	ld 29, 96(1)                            # 8-byte Folded Reload
	ld 28, 88(1)                            # 8-byte Folded Reload
	ld 27, 80(1)                            # 8-byte Folded Reload
	ld 26, 72(1)                            # 8-byte Folded Reload
	ld 25, 64(1)                            # 8-byte Folded Reload
	ld 24, 56(1)                            # 8-byte Folded Reload
	ld 23, 48(1)                            # 8-byte Folded Reload
	ld 22, 40(1)                            # 8-byte Folded Reload
	ld 21, 32(1)                            # 8-byte Folded Reload
	addi 1, 1, 128
	blr
.Llfa_func_end30:
	.size	lfa_k_copy_u8, .Llfa_func_end30-.Llfa_func_begin30
                                        # -- End function
	.section	.text.lfa_k_set_u32,"ax",@progbits
	.globl	lfa_k_set_u32                   # -- Begin function lfa_k_set_u32
	.p2align	2
	.type	lfa_k_set_u32,@function
lfa_k_set_u32:                          # @lfa_k_set_u32
.Llfa_func_begin31:
# %bb.0:                                # %entry
	cmpwi	5, 1
                                        # kill: def $r4 killed $r4 def $x4
	bltlr	0
# %bb.1:                                # %for.body.preheader
	stwu 1, -64(1)
	cmplwi	5, 8
	li 6, 0
	std 30, 40(1)                           # 8-byte Folded Spill
	blt	0, .Llfa_BB31_5
# %bb.2:                                # %vector.ph
	sldi 7, 4, 32
	addi 8, 1, 16
	std 7, 16(1)
	rlwinm 6, 5, 0, 1, 28
	li 9, 12
	std 7, 24(1)
	addi 7, 3, 16
	li 10, 8
	li 11, 4
	mr	12, 6
	lvx 2, 0, 8
	li 8, 0
	vspltw 2, 2, 0
.Llfa_BB31_3:                               # %vector.body
                                        # =>This Inner Loop Header: Depth=1
	lvsr 3, 3, 8
	add 30, 3, 8
	addi 12, 12, -8
	vperm 3, 2, 2, 3
	cmplwi	12, 0
	stvewx 3, 30, 9
	stvewx 3, 30, 10
	stvewx 3, 30, 11
	add 30, 7, 8
	stvewx 3, 3, 8
	lvsr 3, 7, 8
	vperm 3, 2, 2, 3
	stvewx 3, 7, 8
	addi 8, 8, 32
	stvewx 3, 30, 9
	stvewx 3, 30, 10
	stvewx 3, 30, 11
	bne	0, .Llfa_BB31_3
# %bb.4:                                # %middle.block
	cmplw	5, 6
	beq	0, .Llfa_BB31_7
.Llfa_BB31_5:                               # %for.body.preheader5
	slwi 7, 6, 2
	add 7, 7, 3
	sub	3, 5, 6
	addi 5, 7, -4
.Llfa_BB31_6:                               # %for.body
                                        # =>This Inner Loop Header: Depth=1
	addi 3, 3, -1
	stwu 4, 4(5)
	cmplwi	3, 0
	bne	0, .Llfa_BB31_6
.Llfa_BB31_7:
	ld 30, 40(1)                            # 8-byte Folded Reload
	addi 1, 1, 64
	blr
.Llfa_func_end31:
	.size	lfa_k_set_u32, .Llfa_func_end31-.Llfa_func_begin31
                                        # -- End function
	.section	.text.lfa_k_set_f32,"ax",@progbits
	.globl	lfa_k_set_f32                   # -- Begin function lfa_k_set_f32
	.p2align	2
	.type	lfa_k_set_f32,@function
lfa_k_set_f32:                          # @lfa_k_set_f32
.Llfa_func_begin32:
# %bb.0:                                # %entry
	cmpwi	5, 1
	bltlr	0
# %bb.1:                                # %for.body.preheader
	stwu 1, -32(1)
	cmplwi	5, 8
	li 4, 0
	blt	0, .Llfa_BB32_5
# %bb.2:                                # %vector.ph
	stfs 1, 16(1)
	addi 7, 1, 16
	rlwinm 4, 5, 0, 1, 28
	addi 6, 3, 16
	li 8, 12
	lvx 2, 0, 7
	li 7, 0
	li 9, 8
	li 10, 4
	mr	11, 4
	vspltw 2, 2, 0
.Llfa_BB32_3:                               # %vector.body
                                        # =>This Inner Loop Header: Depth=1
	lvsr 3, 3, 7
	add 12, 3, 7
	addi 11, 11, -8
	vperm 3, 2, 2, 3
	cmplwi	11, 0
	stvewx 3, 12, 8
	stvewx 3, 12, 9
	stvewx 3, 12, 10
	add 12, 6, 7
	stvewx 3, 3, 7
	lvsr 3, 6, 7
	vperm 3, 2, 2, 3
	stvewx 3, 6, 7
	addi 7, 7, 32
	stvewx 3, 12, 8
	stvewx 3, 12, 9
	stvewx 3, 12, 10
	bne	0, .Llfa_BB32_3
# %bb.4:                                # %middle.block
	cmplw	5, 4
	beq	0, .Llfa_BB32_7
.Llfa_BB32_5:                               # %for.body.preheader5
	slwi 6, 4, 2
	add 6, 6, 3
	sub	3, 5, 4
	addi 4, 6, -4
.Llfa_BB32_6:                               # %for.body
                                        # =>This Inner Loop Header: Depth=1
	addi 3, 3, -1
	stfsu 1, 4(4)
	cmplwi	3, 0
	bne	0, .Llfa_BB32_6
.Llfa_BB32_7:
	addi 1, 1, 32
	blr
.Llfa_func_end32:
	.size	lfa_k_set_f32, .Llfa_func_end32-.Llfa_func_begin32
                                        # -- End function
	.section	.text.lfa_k_zero_u8,"ax",@progbits
	.globl	lfa_k_zero_u8                   # -- Begin function lfa_k_zero_u8
	.p2align	2
	.type	lfa_k_zero_u8,@function
lfa_k_zero_u8:                          # @lfa_k_zero_u8
.Llfa_func_begin33:
# %bb.0:                                # %entry
	cmpwi	4, 1
	bltlr	0
# %bb.1:                                # %iter.check
	stwu 1, -112(1)
	cmplwi	4, 4
	li 5, 0
	std 22, 24(1)                           # 8-byte Folded Spill
	std 23, 32(1)                           # 8-byte Folded Spill
	std 24, 40(1)                           # 8-byte Folded Spill
	std 25, 48(1)                           # 8-byte Folded Spill
	std 26, 56(1)                           # 8-byte Folded Spill
	std 27, 64(1)                           # 8-byte Folded Spill
	std 28, 72(1)                           # 8-byte Folded Spill
	std 29, 80(1)                           # 8-byte Folded Spill
	std 30, 88(1)                           # 8-byte Folded Spill
	blt	0, .Llfa_BB33_10
# %bb.2:                                # %vector.main.loop.iter.check
	cmplwi	4, 16
	blt	0, .Llfa_BB33_7
# %bb.3:                                # %vector.ph
	rlwinm 5, 4, 0, 1, 27
	li 6, 15
	vxor 2, 2, 2
	li 7, 14
	li 8, 13
	li 9, 12
	li 10, 11
	li 11, 10
	li 12, 9
	li 0, 8
	li 30, 7
	li 29, 6
	li 28, 5
	li 27, 4
	li 26, 3
	li 25, 2
	li 24, 1
	mr	23, 3
	mr	22, 5
.Llfa_BB33_4:                               # %vector.body
                                        # =>This Inner Loop Header: Depth=1
	lvsr 3, 0, 23
	addi 22, 22, -16
	vperm 3, 2, 2, 3
	cmplwi	22, 0
	stvebx 3, 23, 6
	stvebx 3, 23, 7
	stvebx 3, 23, 8
	stvebx 3, 23, 9
	stvebx 3, 23, 10
	stvebx 3, 23, 11
	stvebx 3, 23, 12
	stvebx 3, 23, 0
	stvebx 3, 23, 30
	stvebx 3, 23, 29
	stvebx 3, 23, 28
	stvebx 3, 23, 27
	stvebx 3, 23, 26
	stvebx 3, 23, 25
	stvebx 3, 23, 24
	stvebx 3, 0, 23
	addi 23, 23, 16
	bne	0, .Llfa_BB33_4
# %bb.5:                                # %middle.block
	cmplw	4, 5
	beq	0, .Llfa_BB33_12
# %bb.6:                                # %vec.epilog.iter.check
	andi. 6, 4, 12
	beq	0, .Llfa_BB33_10
.Llfa_BB33_7:                               # %vec.epilog.ph
	mr	8, 5
	rlwinm 5, 4, 0, 1, 29
	vxor 2, 2, 2
	li 6, 3
	sub	7, 8, 5
	add 8, 3, 8
	li 9, 2
	li 10, 1
.Llfa_BB33_8:                               # %vec.epilog.vector.body
                                        # =>This Inner Loop Header: Depth=1
	lvsr 3, 0, 8
	addi 7, 7, 4
	vperm 3, 2, 2, 3
	cmplwi	7, 0
	stvebx 3, 8, 6
	stvebx 3, 8, 9
	stvebx 3, 8, 10
	stvebx 3, 0, 8
	addi 8, 8, 4
	bne	0, .Llfa_BB33_8
# %bb.9:                                # %vec.epilog.middle.block
	cmplw	4, 5
	beq	0, .Llfa_BB33_12
.Llfa_BB33_10:                              # %for.body.preheader
	add 3, 5, 3
	sub	4, 4, 5
	addi 3, 3, -1
	li 5, 0
.Llfa_BB33_11:                              # %for.body
                                        # =>This Inner Loop Header: Depth=1
	addi 4, 4, -1
	stbu 5, 1(3)
	cmplwi	4, 0
	bne	0, .Llfa_BB33_11
.Llfa_BB33_12:
	ld 30, 88(1)                            # 8-byte Folded Reload
	ld 29, 80(1)                            # 8-byte Folded Reload
	ld 28, 72(1)                            # 8-byte Folded Reload
	ld 27, 64(1)                            # 8-byte Folded Reload
	ld 26, 56(1)                            # 8-byte Folded Reload
	ld 25, 48(1)                            # 8-byte Folded Reload
	ld 24, 40(1)                            # 8-byte Folded Reload
	ld 23, 32(1)                            # 8-byte Folded Reload
	ld 22, 24(1)                            # 8-byte Folded Reload
	addi 1, 1, 112
	blr
.Llfa_func_end33:
	.size	lfa_k_zero_u8, .Llfa_func_end33-.Llfa_func_begin33
                                        # -- End function
	.section	.rodata.cst16,"aM",@progbits,16
	.p2align	4, 0x0                          # -- Begin function lfa_k_stride2_f32_r
.Llfa_CPI34_0:
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
.Llfa_CPI34_1:
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
	.section	.text.lfa_k_stride2_f32_r,"ax",@progbits
	.globl	lfa_k_stride2_f32_r
	.p2align	2
	.type	lfa_k_stride2_f32_r,@function
lfa_k_stride2_f32_r:                    # @lfa_k_stride2_f32_r
.Llfa_func_begin34:
# %bb.0:                                # %entry
	cmpwi	5, 1
	bltlr	0
# %bb.1:                                # %for.body.preheader
	stwu 1, -48(1)
	cmplwi	5, 4
	li 6, 0
	std 29, 16(1)                           # 8-byte Folded Spill
	std 30, 24(1)                           # 8-byte Folded Spill
	blt	0, .Llfa_BB34_5
# %bb.2:                                # %vector.ph
	li 9, .Llfa_CPI34_0@l
	lis 10, .Llfa_CPI34_0@ha
	lvx 2, 10, 9
	li 11, .Llfa_CPI34_1@l
	lis 10, .Llfa_CPI34_1@ha
	lvx 3, 10, 11
	rlwinm 6, 5, 0, 1, 29
	li 7, 31
	li 8, 16
	li 9, 12
	li 10, 8
	li 11, 4
	mr	12, 4
	mr	30, 3
	mr	29, 6
.Llfa_BB34_3:                               # %vector.body
                                        # =>This Inner Loop Header: Depth=1
	lvx 4, 12, 7
	addi 29, 29, -4
	cmplwi	29, 0
	lvx 5, 12, 8
	lvx 1, 0, 12
	lvsl 0, 0, 12
	addi 12, 12, 32
	vperm 4, 5, 4, 0
	vperm 5, 1, 5, 0
	vperm 0, 5, 4, 2
	vperm 4, 5, 4, 3
	lvsr 5, 0, 30
	vaddfp 4, 4, 0
	vperm 4, 4, 4, 5
	stvewx 4, 30, 9
	stvewx 4, 30, 10
	stvewx 4, 30, 11
	stvewx 4, 0, 30
	addi 30, 30, 16
	bne	0, .Llfa_BB34_3
# %bb.4:                                # %middle.block
	cmplw	5, 6
	beq	0, .Llfa_BB34_7
.Llfa_BB34_5:                               # %for.body.preheader13
	sub	5, 5, 6
	slwi 7, 6, 2
	slwi 6, 6, 3
	add 4, 6, 4
	add 6, 7, 3
	addi 3, 4, -8
	addi 4, 6, -4
.Llfa_BB34_6:                               # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lfsu 0, 8(3)
	addi 5, 5, -1
	cmplwi	5, 0
	lfs 1, 4(3)
	fadds 0, 1, 0
	stfsu 0, 4(4)
	bne	0, .Llfa_BB34_6
.Llfa_BB34_7:
	ld 30, 24(1)                            # 8-byte Folded Reload
	ld 29, 16(1)                            # 8-byte Folded Reload
	addi 1, 1, 48
	blr
.Llfa_func_end34:
	.size	lfa_k_stride2_f32_r, .Llfa_func_end34-.Llfa_func_begin34
                                        # -- End function
	.section	.text.lfa_k_gather_f32_r,"ax",@progbits
	.globl	lfa_k_gather_f32_r              # -- Begin function lfa_k_gather_f32_r
	.p2align	2
	.type	lfa_k_gather_f32_r,@function
lfa_k_gather_f32_r:                     # @lfa_k_gather_f32_r
.Llfa_func_begin35:
# %bb.0:                                # %entry
	cmpwi	6, 1
	bltlr	0
# %bb.1:                                # %for.body.preheader
	li 7, 0
	beq	0, .Llfa_BB35_5
# %bb.2:                                # %vector.ph
	rlwinm 7, 6, 0, 1, 30
	addi 8, 5, -8
	addi 9, 3, -8
	mr	10, 7
.Llfa_BB35_3:                               # %vector.body
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
	bne	0, .Llfa_BB35_3
# %bb.4:                                # %middle.block
	cmplw	6, 7
	beqlr	0
.Llfa_BB35_5:                               # %for.body.preheader8
	slwi 8, 7, 2
	sub	6, 6, 7
	addi 7, 8, -4
	add 5, 5, 7
	add 3, 3, 7
.Llfa_BB35_6:                               # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lwzu 7, 4(5)
	addi 6, 6, -1
	cmplwi	6, 0
	slwi 7, 7, 2
	lwzx 7, 4, 7
	stwu 7, 4(3)
	bne	0, .Llfa_BB35_6
	blr
.Llfa_func_end35:
	.size	lfa_k_gather_f32_r, .Llfa_func_end35-.Llfa_func_begin35
                                        # -- End function
	.section	.rodata.cst16,"aM",@progbits,16
	.p2align	4, 0x0                          # -- Begin function lfa_k_offset1_f32_r
.Llfa_CPI36_0:
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
	.section	.text.lfa_k_offset1_f32_r,"ax",@progbits
	.globl	lfa_k_offset1_f32_r
	.p2align	2
	.type	lfa_k_offset1_f32_r,@function
lfa_k_offset1_f32_r:                    # @lfa_k_offset1_f32_r
.Llfa_func_begin36:
# %bb.0:                                # %entry
	cmpwi	5, 1
	bltlr	0
# %bb.1:                                # %for.body.preheader
	stwu 1, -96(1)
	lfs 0, 0(4)
	cmplwi	5, 8
	li 6, 0
	std 28, 56(1)                           # 8-byte Folded Spill
	std 29, 64(1)                           # 8-byte Folded Spill
	std 30, 72(1)                           # 8-byte Folded Spill
	blt	0, .Llfa_BB36_6
# %bb.2:                                # %vector.ph
	stfs 0, 32(1)
	li 9, .Llfa_CPI36_0@l
	lis 10, .Llfa_CPI36_0@ha
	addi 11, 1, 32
	rlwinm 6, 5, 0, 1, 28
	lvx 2, 10, 9
	addi 7, 4, 20
	addi 8, 4, 4
	addi 9, 3, 16
	li 10, 0
	lvx 3, 0, 11
	li 11, 15
	li 12, 12
	vperm 2, 3, 3, 2
	li 0, 8
	li 30, 4
	mr	29, 6
.Llfa_BB36_3:                               # %vector.body
                                        # =>This Inner Loop Header: Depth=1
	add 28, 8, 10
	lvx 3, 8, 10
	addi 29, 29, -8
	cmplwi	29, 0
	lvx 4, 28, 11
	add 28, 7, 10
	lvsl 5, 8, 10
	vperm 3, 3, 4, 5
	lvsl 0, 7, 10
	vsldoi 4, 2, 3, 12
	lvx 2, 7, 10
	lvx 5, 28, 11
	add 28, 3, 10
	vperm 2, 2, 5, 0
	vsldoi 5, 3, 2, 12
	vaddfp 3, 3, 4
	lvsr 4, 3, 10
	vperm 3, 3, 3, 4
	lvsr 4, 9, 10
	stvewx 3, 28, 12
	stvewx 3, 28, 0
	vaddfp 5, 2, 5
	stvewx 3, 28, 30
	add 28, 9, 10
	stvewx 3, 3, 10
	vperm 3, 5, 5, 4
	stvewx 3, 9, 10
	addi 10, 10, 32
	stvewx 3, 28, 12
	stvewx 3, 28, 0
	stvewx 3, 28, 30
	bne	0, .Llfa_BB36_3
# %bb.4:                                # %middle.block
	addi 7, 1, 16
	cmplw	5, 6
	stvx 2, 0, 7
	beq	0, .Llfa_BB36_8
# %bb.5:
	lfs 0, 28(1)
.Llfa_BB36_6:                               # %for.body.preheader12
	slwi 7, 6, 2
	sub	5, 5, 6
	add 6, 7, 3
	add 3, 4, 7
	addi 4, 6, -4
.Llfa_BB36_7:                               # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lfsu 1, 4(3)
	addi 5, 5, -1
	cmplwi	5, 0
	fadds 0, 1, 0
	stfsu 0, 4(4)
	fmr 0, 1
	bne	0, .Llfa_BB36_7
.Llfa_BB36_8:
	ld 30, 72(1)                            # 8-byte Folded Reload
	ld 29, 64(1)                            # 8-byte Folded Reload
	ld 28, 56(1)                            # 8-byte Folded Reload
	addi 1, 1, 96
	blr
.Llfa_func_end36:
	.size	lfa_k_offset1_f32_r, .Llfa_func_end36-.Llfa_func_begin36
                                        # -- End function
	.section	.rodata.cst16,"aM",@progbits,16
	.p2align	4, 0x0                          # -- Begin function lfa_k_fir3_f32_r
.Llfa_CPI37_0:
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
.Llfa_CPI37_1:
	.long	0x3f000000                      # float 0.5
	.long	0x3f000000                      # float 0.5
	.long	0x3f000000                      # float 0.5
	.long	0x3f000000                      # float 0.5
.Llfa_CPI37_2:
	.long	0x3e800000                      # float 0.25
	.long	0x3e800000                      # float 0.25
	.long	0x3e800000                      # float 0.25
	.long	0x3e800000                      # float 0.25
	.section	.rodata.cst4,"aM",@progbits,4
	.p2align	2, 0x0
.Llfa_CPI37_3:
	.long	0x3f000000                      # float 0.5
.Llfa_CPI37_4:
	.long	0x3e800000                      # float 0.25
	.section	.text.lfa_k_fir3_f32_r,"ax",@progbits
	.globl	lfa_k_fir3_f32_r
	.p2align	2
	.type	lfa_k_fir3_f32_r,@function
lfa_k_fir3_f32_r:                       # @lfa_k_fir3_f32_r
.Llfa_func_begin37:
# %bb.0:                                # %entry
	cmpwi	5, 1
	bltlr	0
# %bb.1:                                # %for.body.preheader
	stwu 1, -80(1)
	lfs 0, 0(4)
	cmplwi	5, 4
	li 6, 0
	std 29, 48(1)                           # 8-byte Folded Spill
	std 30, 56(1)                           # 8-byte Folded Spill
	blt	0, .Llfa_BB37_6
# %bb.2:                                # %vector.ph
	li 6, .Llfa_CPI37_0@l
	lis 7, .Llfa_CPI37_0@ha
	stfs 0, 32(1)
	vspltisw 4, -1
	li 11, .Llfa_CPI37_1@l
	lis 12, .Llfa_CPI37_1@ha
	lvx 2, 7, 6
	addi 6, 1, 32
	li 0, .Llfa_CPI37_2@l
	addi 7, 4, 4
	addi 8, 4, 8
	lvx 3, 0, 6
	rlwinm 6, 5, 0, 1, 29
	li 9, 0
	vslw 4, 4, 4
	li 10, 15
	mr	30, 6
	vperm 2, 3, 3, 2
	lvx 3, 12, 11
	lis 12, .Llfa_CPI37_2@ha
	li 11, 12
	lvx 5, 12, 0
	li 12, 8
	li 0, 4
.Llfa_BB37_3:                               # %vector.body
                                        # =>This Inner Loop Header: Depth=1
	add 29, 7, 9
	lvx 0, 7, 9
	vmr	7, 2
	addi 30, 30, -4
	cmplwi	30, 0
	lvx 1, 29, 10
	add 29, 8, 9
	lvsl 6, 7, 9
	vperm 2, 0, 1, 6
	lvx 0, 8, 9
	lvx 1, 29, 10
	add 29, 3, 9
	lvsl 6, 8, 9
	vperm 0, 0, 1, 6
	vsldoi 1, 7, 2, 12
	vmaddfp 6, 2, 3, 4
	vaddfp 0, 0, 1
	lvsr 1, 3, 9
	vmaddfp 0, 0, 5, 6
	vperm 0, 0, 0, 1
	stvewx 0, 3, 9
	addi 9, 9, 16
	stvewx 0, 29, 11
	stvewx 0, 29, 12
	stvewx 0, 29, 0
	bne	0, .Llfa_BB37_3
# %bb.4:                                # %middle.block
	addi 7, 1, 16
	cmplw	5, 6
	stvx 2, 0, 7
	beq	0, .Llfa_BB37_8
# %bb.5:
	lfs 0, 28(1)
.Llfa_BB37_6:                               # %for.body.preheader19
	slwi 7, 6, 2
	sub	5, 5, 6
	lis 6, .Llfa_CPI37_3@ha
	add 8, 7, 3
	lfs 1, .Llfa_CPI37_3@l(6)
	lis 6, .Llfa_CPI37_4@ha
	add 3, 4, 7
	lfs 2, .Llfa_CPI37_4@l(6)
	addi 4, 8, -4
.Llfa_BB37_7:                               # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lfsu 3, 4(3)
	addi 5, 5, -1
	cmplwi	5, 0
	fmuls 4, 3, 1
	lfs 5, 4(3)
	fadds 0, 5, 0
	fmadds 0, 0, 2, 4
	stfsu 0, 4(4)
	fmr 0, 3
	bne	0, .Llfa_BB37_7
.Llfa_BB37_8:
	ld 30, 56(1)                            # 8-byte Folded Reload
	ld 29, 48(1)                            # 8-byte Folded Reload
	addi 1, 1, 80
	blr
.Llfa_func_end37:
	.size	lfa_k_fir3_f32_r, .Llfa_func_end37-.Llfa_func_begin37
                                        # -- End function
	.section	.text.lfa_k_prefix_f32,"ax",@progbits
	.globl	lfa_k_prefix_f32                # -- Begin function lfa_k_prefix_f32
	.p2align	2
	.type	lfa_k_prefix_f32,@function
lfa_k_prefix_f32:                       # @lfa_k_prefix_f32
.Llfa_func_begin38:
# %bb.0:                                # %entry
	cmpwi	4, 2
	bltlr	0
# %bb.1:                                # %for.body.preheader
	lfs 0, 0(3)
	addi 4, 4, -1
.Llfa_BB38_2:                               # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lfsu 1, 4(3)
	addi 4, 4, -1
	cmplwi	4, 0
	fadds 0, 1, 0
	stfs 0, 0(3)
	bne	0, .Llfa_BB38_2
# %bb.3:                                # %for.end
	blr
.Llfa_func_end38:
	.size	lfa_k_prefix_f32, .Llfa_func_end38-.Llfa_func_begin38
                                        # -- End function
	.section	.text.lfa_k_g_add_f32,"ax",@progbits
	.globl	lfa_k_g_add_f32                 # -- Begin function lfa_k_g_add_f32
	.p2align	2
	.type	lfa_k_g_add_f32,@function
lfa_k_g_add_f32:                        # @lfa_k_g_add_f32
.Llfa_func_begin39:
# %bb.0:                                # %entry
	lis 5, lfa_g_a@ha
	lis 6, lfa_g_b@ha
	lis 7, lfa_g_c@ha
	li 3, 0
	li 4, 16
	la 5, lfa_g_a@l(5)
	la 6, lfa_g_b@l(6)
	la 7, lfa_g_c@l(7)
.Llfa_BB39_1:                               # %vector.body
                                        # =>This Inner Loop Header: Depth=1
	lvx 2, 5, 3
	add 8, 5, 3
	lvx 3, 6, 3
	lvx 4, 8, 4
	add 8, 6, 3
	vaddfp 2, 3, 2
	lvx 3, 8, 4
	add 8, 7, 3
	stvx 2, 7, 3
	addi 3, 3, 32
	cmplwi	3, 4096
	vaddfp 2, 3, 4
	stvx 2, 8, 4
	bne	0, .Llfa_BB39_1
# %bb.2:                                # %for.end
	blr
.Llfa_func_end39:
	.size	lfa_k_g_add_f32, .Llfa_func_end39-.Llfa_func_begin39
                                        # -- End function
	.section	.text.lfa_k_g_saxpy,"ax",@progbits
	.globl	lfa_k_g_saxpy                   # -- Begin function lfa_k_g_saxpy
	.p2align	2
	.type	lfa_k_g_saxpy,@function
lfa_k_g_saxpy:                          # @lfa_k_g_saxpy
.Llfa_func_begin40:
# %bb.0:                                # %entry
	stwu 1, -32(1)
	stfs 1, 16(1)
	addi 5, 1, 16
	lis 4, lfa_g_a@ha
	lis 6, lfa_g_c@ha
	li 3, 0
	lvx 2, 0, 5
	lis 5, lfa_g_b@ha
	la 4, lfa_g_a@l(4)
	la 5, lfa_g_b@l(5)
	la 6, lfa_g_c@l(6)
	vspltw 2, 2, 0
.Llfa_BB40_1:                               # %vector.body
                                        # =>This Inner Loop Header: Depth=1
	lvx 3, 4, 3
	lvx 4, 5, 3
	vmaddfp 3, 3, 2, 4
	stvx 3, 6, 3
	addi 3, 3, 16
	cmplwi	3, 4096
	bne	0, .Llfa_BB40_1
# %bb.2:                                # %for.end
	addi 1, 1, 32
	blr
.Llfa_func_end40:
	.size	lfa_k_g_saxpy, .Llfa_func_end40-.Llfa_func_begin40
                                        # -- End function
	.section	.text.lfa_k_g_add_i32,"ax",@progbits
	.globl	lfa_k_g_add_i32                 # -- Begin function lfa_k_g_add_i32
	.p2align	2
	.type	lfa_k_g_add_i32,@function
lfa_k_g_add_i32:                        # @lfa_k_g_add_i32
.Llfa_func_begin41:
# %bb.0:                                # %entry
	lis 5, lfa_g_ia@ha
	lis 6, lfa_g_ib@ha
	lis 7, lfa_g_ic@ha
	li 3, 0
	li 4, 16
	la 5, lfa_g_ia@l(5)
	la 6, lfa_g_ib@l(6)
	la 7, lfa_g_ic@l(7)
.Llfa_BB41_1:                               # %vector.body
                                        # =>This Inner Loop Header: Depth=1
	lvx 2, 5, 3
	add 8, 5, 3
	lvx 3, 6, 3
	lvx 4, 8, 4
	add 8, 6, 3
	vadduwm 2, 3, 2
	lvx 5, 8, 4
	add 8, 7, 3
	stvx 2, 7, 3
	addi 3, 3, 32
	vadduwm 2, 5, 4
	cmplwi	3, 4096
	stvx 2, 8, 4
	bne	0, .Llfa_BB41_1
# %bb.2:                                # %for.end
	blr
.Llfa_func_end41:
	.size	lfa_k_g_add_i32, .Llfa_func_end41-.Llfa_func_begin41
                                        # -- End function
	.section	.text.lfa_k_g_dot,"ax",@progbits
	.globl	lfa_k_g_dot                     # -- Begin function lfa_k_g_dot
	.p2align	2
	.type	lfa_k_g_dot,@function
lfa_k_g_dot:                            # @lfa_k_g_dot
.Llfa_func_begin42:
# %bb.0:                                # %entry
	lis 4, lfa_g_a@ha
	lis 6, lfa_g_b@ha
	vxor 2, 2, 2
	li 3, 0
	la 4, lfa_g_a@l(4)
	li 5, 16
	la 6, lfa_g_b@l(6)
	vxor 3, 3, 3
.Llfa_BB42_1:                               # %vector.body
                                        # =>This Inner Loop Header: Depth=1
	lvx 4, 4, 3
	add 7, 4, 3
	lvx 5, 6, 3
	vmaddfp 2, 5, 4, 2
	lvx 4, 7, 5
	add 7, 6, 3
	addi 3, 3, 32
	cmplwi	3, 4096
	lvx 5, 7, 5
	vmaddfp 3, 5, 4, 3
	bne	0, .Llfa_BB42_1
# %bb.2:                                # %for.end
	stwu 1, -32(1)
	vaddfp 2, 3, 2
	addi 3, 1, 16
	vsldoi 3, 2, 2, 8
	vaddfp 2, 2, 3
	vspltw 3, 2, 1
	vaddfp 2, 2, 3
	stvx 2, 0, 3
	lfs 1, 16(1)
	addi 1, 1, 32
	blr
.Llfa_func_end42:
	.size	lfa_k_g_dot, .Llfa_func_end42-.Llfa_func_begin42
                                        # -- End function
	.section	.text.lfa_k_add16_f32_r,"ax",@progbits
	.globl	lfa_k_add16_f32_r               # -- Begin function lfa_k_add16_f32_r
	.p2align	2
	.type	lfa_k_add16_f32_r,@function
lfa_k_add16_f32_r:                      # @lfa_k_add16_f32_r
.Llfa_func_begin43:
# %bb.0:                                # %entry
	li 6, 16
	lvx 2, 0, 4
	li 7, 12
	li 8, 28
	lvx 3, 4, 6
	lvx 5, 0, 5
	lvx 0, 5, 6
	lvsl 4, 0, 4
	vperm 2, 2, 3, 4
	lvsl 1, 0, 5
	vperm 5, 5, 0, 1
	vaddfp 2, 5, 2
	lvsr 5, 0, 3
	vperm 2, 2, 2, 5
	stvewx 2, 3, 7
	li 7, 8
	stvewx 2, 3, 7
	li 7, 4
	stvewx 2, 3, 7
	li 7, 32
	stvewx 2, 0, 3
	lvx 2, 4, 7
	lvx 5, 5, 7
	vperm 3, 3, 2, 4
	vperm 0, 0, 5, 1
	vaddfp 3, 0, 3
	lvsr 0, 3, 6
	vperm 3, 3, 3, 0
	stvewx 3, 3, 8
	li 8, 24
	stvewx 3, 3, 8
	li 8, 20
	stvewx 3, 3, 8
	li 8, 44
	stvewx 3, 3, 6
	li 6, 48
	lvx 3, 4, 6
	lvx 0, 5, 6
	vperm 2, 2, 3, 4
	vperm 5, 5, 0, 1
	vaddfp 2, 5, 2
	lvsr 5, 3, 7
	vperm 2, 2, 2, 5
	stvewx 2, 3, 8
	li 8, 40
	stvewx 2, 3, 8
	li 8, 36
	stvewx 2, 3, 8
	stvewx 2, 3, 7
	li 7, 63
	lvx 2, 4, 7
	li 4, 60
	vperm 2, 3, 2, 4
	lvx 3, 5, 7
	vperm 3, 0, 3, 1
	vaddfp 2, 3, 2
	lvsr 3, 3, 6
	vperm 2, 2, 2, 3
	stvewx 2, 3, 4
	li 4, 56
	stvewx 2, 3, 4
	li 4, 52
	stvewx 2, 3, 4
	stvewx 2, 3, 6
	blr
.Llfa_func_end43:
	.size	lfa_k_add16_f32_r, .Llfa_func_end43-.Llfa_func_begin43
                                        # -- End function
	.type	lfa_g_a,@object                 # @lfa_g_a
	.section	.bss.lfa_g_a,"aw",@nobits
	.globl	lfa_g_a
	.p2align	4, 0x0
lfa_g_a:
	.space	4096
	.size	lfa_g_a, 4096

	.type	lfa_g_b,@object                 # @lfa_g_b
	.section	.bss.lfa_g_b,"aw",@nobits
	.globl	lfa_g_b
	.p2align	4, 0x0
lfa_g_b:
	.space	4096
	.size	lfa_g_b, 4096

	.type	lfa_g_c,@object                 # @lfa_g_c
	.section	.bss.lfa_g_c,"aw",@nobits
	.globl	lfa_g_c
	.p2align	4, 0x0
lfa_g_c:
	.space	4096
	.size	lfa_g_c, 4096

	.type	lfa_g_ia,@object                # @lfa_g_ia
	.section	.bss.lfa_g_ia,"aw",@nobits
	.globl	lfa_g_ia
	.p2align	4, 0x0
lfa_g_ia:
	.space	4096
	.size	lfa_g_ia, 4096

	.type	lfa_g_ib,@object                # @lfa_g_ib
	.section	.bss.lfa_g_ib,"aw",@nobits
	.globl	lfa_g_ib
	.p2align	4, 0x0
lfa_g_ib:
	.space	4096
	.size	lfa_g_ib, 4096

	.type	lfa_g_ic,@object                # @lfa_g_ic
	.section	.bss.lfa_g_ic,"aw",@nobits
	.globl	lfa_g_ic
	.p2align	4, 0x0
lfa_g_ic:
	.space	4096
	.size	lfa_g_ic, 4096

