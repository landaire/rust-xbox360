# speed.c: clang version 22.1.8 (git@github.com:landaire/llvm-project.git 2f714b11080e5a449de3969e3c4477cf5cbe92ab)
# flags: --target=powerpc64-unknown-xbox360 -ffreestanding -fno-builtin -fms-extensions -ffunction-sections -fdata-sections -fno-ident -fno-addrsig -fno-asynchronous-unwind-tables -g0 -O3 -ffast-math -mno-altivec
	.section	.text.cs_sp_saxpy_s,"ax",@progbits
	.globl	cs_sp_saxpy_s                   # -- Begin function cs_sp_saxpy_s
	.p2align	2
	.type	cs_sp_saxpy_s,@function
cs_sp_saxpy_s:                          # @cs_sp_saxpy_s
.Lcs_func_begin0:
# %bb.0:                                # %entry
	cmpwi	6, 1
	bltlr	0
# %bb.1:                                # %for.body.preheader
	li 5, 0
	beq	0, .Lcs_BB0_5
# %bb.2:                                # %vector.ph
	rlwinm 5, 6, 0, 1, 30
	addi 7, 4, -8
	addi 8, 3, -8
	mr	9, 5
.Lcs_BB0_3:                                # %vector.body
                                        # =>This Inner Loop Header: Depth=1
	lfsu 0, 8(8)
	addi 9, 9, -2
	cmplwi	9, 0
	lfsu 2, 8(7)
	fmadds 0, 2, 1, 0
	lfs 3, 4(7)
	lfs 4, 4(8)
	stfs 0, 0(8)
	fmadds 0, 3, 1, 4
	stfs 0, 4(8)
	bne	0, .Lcs_BB0_3
# %bb.4:                                # %middle.block
	cmplw	6, 5
	beqlr	0
.Lcs_BB0_5:                                # %for.body.preheader10
	slwi 7, 5, 2
	sub	5, 6, 5
	addi 6, 7, -4
	add 4, 4, 6
	add 3, 3, 6
.Lcs_BB0_6:                                # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lfsu 0, 4(3)
	addi 5, 5, -1
	cmplwi	5, 0
	lfsu 2, 4(4)
	fmadds 0, 2, 1, 0
	stfs 0, 0(3)
	bne	0, .Lcs_BB0_6
	blr
.Lcs_func_end0:
	.size	cs_sp_saxpy_s, .Lcs_func_end0-.Lcs_func_begin0
                                        # -- End function
	.section	.text.cs_sp_saxpy_v,"ax",@progbits
	.globl	cs_sp_saxpy_v                   # -- Begin function cs_sp_saxpy_v
	.p2align	2
	.type	cs_sp_saxpy_v,@function
cs_sp_saxpy_v:                          # @cs_sp_saxpy_v
.Lcs_func_begin1:
# %bb.0:                                # %entry
	srawi 5, 6, 2
	cmpwi	6, 4
	addze 5, 5
	bltlr	0
# %bb.1:                                # %for.body.preheader
	addi 4, 4, -16
	addi 3, 3, -16
.Lcs_BB1_2:                                # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lfsu 0, 16(3)
	addi 5, 5, -1
	cmplwi	5, 0
	lfsu 2, 16(4)
	fmadds 0, 2, 1, 0
	lfs 3, 8(4)
	lfs 4, 8(3)
	stfs 0, 0(3)
	fmadds 0, 3, 1, 4
	stfs 0, 8(3)
	lfs 0, 4(4)
	lfs 3, 4(3)
	lfs 2, 12(4)
	fmadds 0, 0, 1, 3
	lfs 4, 12(3)
	stfs 0, 4(3)
	fmadds 0, 2, 1, 4
	stfs 0, 12(3)
	bne	0, .Lcs_BB1_2
# %bb.3:                                # %for.cond.cleanup
	blr
.Lcs_func_end1:
	.size	cs_sp_saxpy_v, .Lcs_func_end1-.Lcs_func_begin1
                                        # -- End function
	.section	.rodata.cst4,"aM",@progbits,4
	.p2align	2, 0x0                          # -- Begin function cs_sp_dot_s
.Lcs_CPI2_0:
	.long	0x00000000                      # float 0
	.section	.text.cs_sp_dot_s,"ax",@progbits
	.globl	cs_sp_dot_s
	.p2align	2
	.type	cs_sp_dot_s,@function
cs_sp_dot_s:                            # @cs_sp_dot_s
.Lcs_func_begin2:
# %bb.0:                                # %entry
	cmpwi	5, 1
	lis 7, .Lcs_CPI2_0@ha
	blt	0, .Lcs_BB2_3
# %bb.1:                                # %for.body.preheader
	bne	0, .Lcs_BB2_4
# %bb.2:
	li 6, 0
	lfs 1, .Lcs_CPI2_0@l(7)
	b .Lcs_BB2_7
.Lcs_BB2_3:
	lfs 1, .Lcs_CPI2_0@l(7)
	blr
.Lcs_BB2_4:                                # %vector.ph
	lis 6, .Lcs_CPI2_0@ha
	addi 7, 3, -8
	lfs 0, .Lcs_CPI2_0@l(6)
	rlwinm 6, 5, 0, 1, 30
	addi 8, 4, -8
	mr	9, 6
	fmr 1, 0
.Lcs_BB2_5:                                # %vector.body
                                        # =>This Inner Loop Header: Depth=1
	lfsu 2, 8(8)
	addi 9, 9, -2
	cmplwi	9, 0
	lfsu 3, 8(7)
	fmadds 0, 2, 3, 0
	lfs 4, 4(7)
	lfs 5, 4(8)
	fmadds 1, 5, 4, 1
	bne	0, .Lcs_BB2_5
# %bb.6:                                # %middle.block
	cmplw	5, 6
	fadds 1, 1, 0
	beqlr	0
.Lcs_BB2_7:                                # %for.body.preheader10
	slwi 7, 6, 2
	sub	5, 5, 6
	addi 6, 7, -4
	add 3, 3, 6
	add 4, 4, 6
.Lcs_BB2_8:                                # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lfsu 0, 4(4)
	addi 5, 5, -1
	cmplwi	5, 0
	lfsu 2, 4(3)
	fmadds 1, 0, 2, 1
	bne	0, .Lcs_BB2_8
# %bb.9:                                # %for.cond.cleanup
	blr
.Lcs_func_end2:
	.size	cs_sp_dot_s, .Lcs_func_end2-.Lcs_func_begin2
                                        # -- End function
	.section	.rodata.cst4,"aM",@progbits,4
	.p2align	2, 0x0                          # -- Begin function cs_sp_dot_v
.Lcs_CPI3_0:
	.long	0x00000000                      # float 0
	.section	.text.cs_sp_dot_v,"ax",@progbits
	.globl	cs_sp_dot_v
	.p2align	2
	.type	cs_sp_dot_v,@function
cs_sp_dot_v:                            # @cs_sp_dot_v
.Lcs_func_begin3:
# %bb.0:                                # %entry
	srawi 7, 5, 2
	lis 6, .Lcs_CPI3_0@ha
	cmpwi	5, 4
	addze 5, 7
	blt	0, .Lcs_BB3_4
# %bb.1:                                # %for.body.preheader
	stwu 1, -192(1)
	lfs 2, .Lcs_CPI3_0@l(6)
	addi 3, 3, -64
	addi 4, 4, -64
	li 6, 0
	stfd 29, 160(1)                         # 8-byte Folded Spill
	fmr 4, 2
	fmr 3, 2
	stfd 30, 168(1)                         # 8-byte Folded Spill
	fmr 1, 2
	fmr 29, 2
	stfd 31, 176(1)                         # 8-byte Folded Spill
	fmr 8, 2
	fmr 7, 2
	stfd 14, 40(1)                          # 8-byte Folded Spill
	stfd 15, 48(1)                          # 8-byte Folded Spill
	fmr 6, 2
	fmr 5, 2
	stfd 16, 56(1)                          # 8-byte Folded Spill
	fmr 9, 2
	fmr 10, 2
	stfd 17, 64(1)                          # 8-byte Folded Spill
	stfd 18, 72(1)                          # 8-byte Folded Spill
	fmr 11, 2
	fmr 12, 2
	stfd 19, 80(1)                          # 8-byte Folded Spill
	fmr 13, 2
	fmr 31, 2
	stfd 20, 88(1)                          # 8-byte Folded Spill
	stfd 21, 96(1)                          # 8-byte Folded Spill
	fmr 30, 2
	stfd 22, 104(1)                         # 8-byte Folded Spill
	stfd 23, 112(1)                         # 8-byte Folded Spill
	stfd 24, 120(1)                         # 8-byte Folded Spill
	stfd 25, 128(1)                         # 8-byte Folded Spill
	stfd 26, 136(1)                         # 8-byte Folded Spill
	stfd 27, 144(1)                         # 8-byte Folded Spill
	stfd 28, 152(1)                         # 8-byte Folded Spill
.Lcs_BB3_2:                                # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lfsu 0, 64(4)
	addi 6, 6, 4
	cmpw	6, 5
	lfsu 28, 64(3)
	stfs 0, 32(1)                           # 4-byte Folded Spill
	lfs 0, 12(3)
	lfs 27, 32(1)                           # 4-byte Folded Reload
	stfs 0, 28(1)                           # 4-byte Folded Spill
	fmadds 4, 27, 28, 4
	lfs 26, 4(3)
	lfs 24, 12(4)
	lfs 23, 4(4)
	lfs 0, 44(3)
	fmadds 3, 23, 26, 3
	lfs 27, 28(1)                           # 4-byte Folded Reload
	lfs 25, 8(3)
	fmadds 29, 24, 27, 29
	lfs 22, 8(4)
	stfs 0, 36(1)                           # 4-byte Folded Spill
	fmadds 1, 22, 25, 1
	lfs 0, 32(3)
	lfs 28, 36(3)
	lfs 26, 32(4)
	lfs 24, 36(4)
	fmadds 9, 26, 0, 9
	lfs 21, 28(3)
	fmadds 10, 24, 28, 10
	lfs 17, 28(4)
	lfs 25, 40(3)
	fmadds 30, 17, 21, 30
	lfs 27, 44(4)
	lfs 23, 40(4)
	lfs 28, 52(3)
	fmadds 11, 23, 25, 11
	lfs 0, 52(4)
	lfs 26, 36(1)                           # 4-byte Folded Reload
	fmadds 7, 0, 28, 7
	lfs 20, 16(3)
	fmadds 12, 27, 26, 12
	lfs 19, 20(3)
	lfs 18, 24(3)
	lfs 16, 16(4)
	lfs 15, 20(4)
	fmadds 13, 16, 20, 13
	lfs 14, 24(4)
	fmadds 2, 15, 19, 2
	lfs 22, 60(3)
	fmadds 31, 14, 18, 31
	lfs 21, 48(3)
	lfs 25, 56(3)
	lfs 27, 56(4)
	lfs 26, 60(4)
	fmadds 6, 27, 25, 6
	lfs 0, 48(4)
	fmadds 5, 26, 22, 5
	fmadds 8, 0, 21, 8
	blt	0, .Lcs_BB3_2
# %bb.3:                                # %for.cond.cleanup.loopexit
	fadds 2, 10, 2
	fadds 10, 11, 31
	lfd 31, 176(1)                          # 8-byte Folded Reload
	lfd 28, 152(1)                          # 8-byte Folded Reload
	fadds 11, 12, 30
	fadds 5, 5, 29
	lfd 30, 168(1)                          # 8-byte Folded Reload
	lfd 29, 160(1)                          # 8-byte Folded Reload
	lfd 27, 144(1)                          # 8-byte Folded Reload
	fadds 9, 9, 13
	fadds 4, 8, 4
	lfd 26, 136(1)                          # 8-byte Folded Reload
	lfd 25, 128(1)                          # 8-byte Folded Reload
	fadds 3, 7, 3
	fadds 6, 6, 1
	lfd 24, 120(1)                          # 8-byte Folded Reload
	lfd 23, 112(1)                          # 8-byte Folded Reload
	fadds 0, 9, 4
	fadds 1, 2, 3
	lfd 22, 104(1)                          # 8-byte Folded Reload
	lfd 21, 96(1)                           # 8-byte Folded Reload
	fadds 2, 10, 6
	fadds 3, 11, 5
	lfd 20, 88(1)                           # 8-byte Folded Reload
	lfd 19, 80(1)                           # 8-byte Folded Reload
	lfd 18, 72(1)                           # 8-byte Folded Reload
	lfd 17, 64(1)                           # 8-byte Folded Reload
	lfd 16, 56(1)                           # 8-byte Folded Reload
	lfd 15, 48(1)                           # 8-byte Folded Reload
	lfd 14, 40(1)                           # 8-byte Folded Reload
	addi 1, 1, 192
	fadds 0, 0, 2
	fadds 1, 1, 3
	fadds 1, 0, 1
	blr
.Lcs_BB3_4:
	lfs 0, .Lcs_CPI3_0@l(6)
	fmr 1, 0
	fmr 2, 0
	fmr 3, 0
	fadds 0, 0, 2
	fadds 1, 1, 3
	fadds 1, 0, 1
	blr
.Lcs_func_end3:
	.size	cs_sp_dot_v, .Lcs_func_end3-.Lcs_func_begin3
                                        # -- End function
	.section	.text.cs_sp_mat4_s,"ax",@progbits
	.globl	cs_sp_mat4_s                    # -- Begin function cs_sp_mat4_s
	.p2align	2
	.type	cs_sp_mat4_s,@function
cs_sp_mat4_s:                           # @cs_sp_mat4_s
.Lcs_func_begin4:
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
.Lcs_BB4_2:                                # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lfsu 29, 16(4)
	addi 6, 6, -1
	cmplwi	6, 0
	fmuls 28, 0, 29
	fmuls 27, 4, 29
	lfs 25, 4(4)
	lfs 24, 12(4)
	fmuls 26, 8, 29
	fmuls 29, 12, 29
	fmadds 28, 1, 25, 28
	fmadds 27, 5, 25, 27
	fmadds 26, 9, 25, 26
	fmadds 29, 13, 25, 29
	lfs 25, 8(4)
	fmadds 28, 2, 25, 28
	fmadds 27, 6, 25, 27
	fmadds 28, 3, 24, 28
	fmadds 29, 31, 25, 29
	stfsu 28, 16(3)
	fmadds 28, 10, 25, 26
	fmadds 27, 7, 24, 27
	stfs 27, 4(3)
	fmadds 28, 11, 24, 28
	fmadds 29, 30, 24, 29
	stfs 28, 8(3)
	stfs 29, 12(3)
	bne	0, .Lcs_BB4_2
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
.Lcs_func_end4:
	.size	cs_sp_mat4_s, .Lcs_func_end4-.Lcs_func_begin4
                                        # -- End function
	.section	.text.cs_sp_mat4_v,"ax",@progbits
	.globl	cs_sp_mat4_v                    # -- Begin function cs_sp_mat4_v
	.p2align	2
	.type	cs_sp_mat4_v,@function
cs_sp_mat4_v:                           # @cs_sp_mat4_v
.Lcs_func_begin5:
# %bb.0:                                # %entry
	cmpwi	6, 1
	bltlr	0
# %bb.1:                                # %for.body.preheader
	stwu 1, -96(1)
	stfd 30, 72(1)                          # 8-byte Folded Spill
	addi 3, 3, -16
	stfd 31, 80(1)                          # 8-byte Folded Spill
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
	stfd 23, 16(1)                          # 8-byte Folded Spill
	stfd 24, 24(1)                          # 8-byte Folded Spill
	stfd 25, 32(1)                          # 8-byte Folded Spill
	stfd 26, 40(1)                          # 8-byte Folded Spill
	stfd 27, 48(1)                          # 8-byte Folded Spill
	stfd 28, 56(1)                          # 8-byte Folded Spill
	stfd 29, 64(1)                          # 8-byte Folded Spill
.Lcs_BB5_2:                                # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lfsu 29, 16(4)
	addi 6, 6, -1
	cmplwi	6, 0
	fmuls 28, 29, 3
	fmuls 27, 29, 2
	lfs 25, 12(4)
	lfs 24, 4(4)
	fmuls 26, 29, 1
	fmuls 29, 29, 0
	fmadds 28, 25, 30, 28
	fmadds 27, 25, 31, 27
	fmadds 26, 25, 13, 26
	fmadds 29, 25, 12, 29
	lfs 25, 8(4)
	fmuls 23, 24, 7
	fmadds 23, 25, 11, 23
	fadds 28, 23, 28
	fmuls 23, 24, 4
	stfsu 28, 16(3)
	fmadds 23, 25, 8, 23
	fadds 29, 23, 29
	stfs 29, 12(3)
	fmuls 23, 24, 5
	fmadds 28, 25, 9, 23
	fmuls 24, 24, 6
	fadds 29, 28, 26
	stfs 29, 8(3)
	fmadds 29, 25, 10, 24
	fadds 29, 29, 27
	stfs 29, 4(3)
	bne	0, .Lcs_BB5_2
# %bb.3:
	lfd 31, 80(1)                           # 8-byte Folded Reload
	lfd 30, 72(1)                           # 8-byte Folded Reload
	lfd 29, 64(1)                           # 8-byte Folded Reload
	lfd 28, 56(1)                           # 8-byte Folded Reload
	lfd 27, 48(1)                           # 8-byte Folded Reload
	lfd 26, 40(1)                           # 8-byte Folded Reload
	lfd 25, 32(1)                           # 8-byte Folded Reload
	lfd 24, 24(1)                           # 8-byte Folded Reload
	lfd 23, 16(1)                           # 8-byte Folded Reload
	addi 1, 1, 96
	blr
.Lcs_func_end5:
	.size	cs_sp_mat4_v, .Lcs_func_end5-.Lcs_func_begin5
                                        # -- End function
	.section	.text.cs_sp_mat4_v8,"ax",@progbits
	.globl	cs_sp_mat4_v8                   # -- Begin function cs_sp_mat4_v8
	.p2align	2
	.type	cs_sp_mat4_v8,@function
cs_sp_mat4_v8:                          # @cs_sp_mat4_v8
.Lcs_func_begin6:
# %bb.0:                                # %entry
	cmpwi	6, 1
	bltlr	0
# %bb.1:                                # %for.body.preheader
	stwu 1, -688(1)
	stfd 30, 600(1)                         # 8-byte Folded Spill
	addi 7, 1, 384
	addi 8, 1, 256
	li 9, 0
	addi 10, 1, 128
	stfd 31, 608(1)                         # 8-byte Folded Spill
	addi 11, 1, 144
	mr	12, 5
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
	addi 4, 1, 16
	std 30, 664(1)                          # 8-byte Folded Spill
	li 30, 0
	std 24, 616(1)                          # 8-byte Folded Spill
	std 25, 624(1)                          # 8-byte Folded Spill
	std 26, 632(1)                          # 8-byte Folded Spill
	std 27, 640(1)                          # 8-byte Folded Spill
	std 28, 648(1)                          # 8-byte Folded Spill
	std 29, 656(1)                          # 8-byte Folded Spill
	stfd 21, 528(1)                         # 8-byte Folded Spill
	stfd 22, 536(1)                         # 8-byte Folded Spill
	stfd 23, 544(1)                         # 8-byte Folded Spill
	stfd 24, 552(1)                         # 8-byte Folded Spill
	stfd 25, 560(1)                         # 8-byte Folded Spill
	stfd 26, 568(1)                         # 8-byte Folded Spill
	stfd 27, 576(1)                         # 8-byte Folded Spill
	stfd 28, 584(1)                         # 8-byte Folded Spill
	stfd 29, 592(1)                         # 8-byte Folded Spill
.Lcs_BB6_2:                                # %for.body
                                        # =>This Loop Header: Depth=1
                                        #     Child Loop BB6_3 Depth 2
                                        #     Child Loop BB6_5 Depth 2
	slwi 0, 9, 7
	li 28, 0
	add 29, 0, 5
	mr	27, 7
	addi 29, 29, -16
	mr	26, 8
.Lcs_BB6_3:                                # %for.body7
                                        #   Parent Loop BB6_2 Depth=1
                                        # =>  This Inner Loop Header: Depth=2
	lfsu 29, 16(29)
	mr	25, 28
	addi 0, 28, 16
	cmplwi	0, 128
	lfs 28, 8(29)
	stfsux 28, 25, 11
	stfsu 29, 16(27)
	stfs 28, 12(25)
	stfs 28, 8(25)
	stfs 28, 4(25)
	lfs 28, 12(29)
	stfs 29, 12(27)
	stfs 29, 8(27)
	stfs 29, 4(27)
	lfs 29, 4(29)
	stfsux 28, 28, 4
	stfsu 29, 16(26)
	stfs 28, 12(28)
	stfs 28, 8(28)
	stfs 28, 4(28)
	mr	28, 0
	stfs 29, 12(26)
	stfs 29, 8(26)
	stfs 29, 4(26)
	bne	0, .Lcs_BB6_3
# %bb.4:                                # %for.cond17.preheader
                                        #   in Loop: Header=BB6_2 Depth=1
	li 29, 0
	mr	28, 7
	mr	27, 8
	mr	26, 10
.Lcs_BB6_5:                                # %for.body20
                                        #   Parent Loop BB6_2 Depth=1
                                        # =>  This Inner Loop Header: Depth=2
	lfsu 28, 16(28)
	mr	25, 29
	mr	24, 3
	fmuls 28, 28, 3
	lfsu 27, 16(27)
	fmuls 27, 27, 7
	lfsu 29, 16(26)
	fmadds 29, 29, 11, 28
	lfsux 26, 25, 4
	fmadds 28, 26, 30, 27
	fadds 29, 29, 28
	lfs 27, 8(28)
	lfs 26, 12(28)
	fmuls 28, 27, 1
	lfs 27, 4(28)
	fmuls 26, 26, 0
	lfs 22, 12(26)
	fmuls 27, 27, 2
	lfs 21, 4(26)
	fmadds 26, 22, 8, 26
	lfs 23, 8(27)
	lfs 22, 8(26)
	fmuls 23, 23, 5
	stfsux 29, 24, 29
	fmadds 29, 21, 10, 27
	addi 29, 29, 16
	cmplwi	29, 128
	fmadds 28, 22, 9, 28
	lfs 27, 8(25)
	lfs 25, 4(27)
	fmadds 27, 27, 13, 23
	fadds 28, 28, 27
	lfs 22, 4(25)
	lfs 24, 12(27)
	fmuls 25, 25, 6
	lfs 27, 12(25)
	fmuls 24, 24, 4
	stfs 28, 8(24)
	fmadds 28, 22, 31, 25
	fadds 29, 29, 28
	stfs 29, 4(24)
	fmadds 29, 27, 12, 24
	fadds 29, 26, 29
	stfs 29, 12(24)
	bne	0, .Lcs_BB6_5
# %bb.6:                                # %for.cond.cleanup19
                                        #   in Loop: Header=BB6_2 Depth=1
	addi 30, 30, 8
	addi 12, 12, 128
	cmpw	30, 6
	addi 3, 3, 128
	addi 9, 9, 1
	blt	0, .Lcs_BB6_2
# %bb.7:
	lfd 31, 608(1)                          # 8-byte Folded Reload
	lfd 30, 600(1)                          # 8-byte Folded Reload
	lfd 29, 592(1)                          # 8-byte Folded Reload
	lfd 28, 584(1)                          # 8-byte Folded Reload
	lfd 27, 576(1)                          # 8-byte Folded Reload
	lfd 26, 568(1)                          # 8-byte Folded Reload
	lfd 25, 560(1)                          # 8-byte Folded Reload
	lfd 24, 552(1)                          # 8-byte Folded Reload
	lfd 23, 544(1)                          # 8-byte Folded Reload
	lfd 22, 536(1)                          # 8-byte Folded Reload
	lfd 21, 528(1)                          # 8-byte Folded Reload
	ld 30, 664(1)                           # 8-byte Folded Reload
	ld 29, 656(1)                           # 8-byte Folded Reload
	ld 28, 648(1)                           # 8-byte Folded Reload
	ld 27, 640(1)                           # 8-byte Folded Reload
	ld 26, 632(1)                           # 8-byte Folded Reload
	ld 25, 624(1)                           # 8-byte Folded Reload
	ld 24, 616(1)                           # 8-byte Folded Reload
	addi 1, 1, 688
	blr
.Lcs_func_end6:
	.size	cs_sp_mat4_v8, .Lcs_func_end6-.Lcs_func_begin6
                                        # -- End function
	.section	.text.cs_sp_mat4_v16,"ax",@progbits
	.globl	cs_sp_mat4_v16                  # -- Begin function cs_sp_mat4_v16
	.p2align	2
	.type	cs_sp_mat4_v16,@function
cs_sp_mat4_v16:                         # @cs_sp_mat4_v16
.Lcs_func_begin7:
# %bb.0:                                # %entry
	cmpwi	6, 1
	bltlr	0
# %bb.1:                                # %for.body.preheader
	stwu 1, -1200(1)
	stfd 30, 1112(1)                        # 8-byte Folded Spill
	addi 7, 1, 768
	addi 8, 1, 512
	li 9, 0
	addi 10, 1, 256
	stfd 31, 1120(1)                        # 8-byte Folded Spill
	addi 11, 1, 272
	mr	12, 5
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
	addi 4, 1, 16
	std 30, 1176(1)                         # 8-byte Folded Spill
	li 30, 0
	std 24, 1128(1)                         # 8-byte Folded Spill
	std 25, 1136(1)                         # 8-byte Folded Spill
	std 26, 1144(1)                         # 8-byte Folded Spill
	std 27, 1152(1)                         # 8-byte Folded Spill
	std 28, 1160(1)                         # 8-byte Folded Spill
	std 29, 1168(1)                         # 8-byte Folded Spill
	stfd 21, 1040(1)                        # 8-byte Folded Spill
	stfd 22, 1048(1)                        # 8-byte Folded Spill
	stfd 23, 1056(1)                        # 8-byte Folded Spill
	stfd 24, 1064(1)                        # 8-byte Folded Spill
	stfd 25, 1072(1)                        # 8-byte Folded Spill
	stfd 26, 1080(1)                        # 8-byte Folded Spill
	stfd 27, 1088(1)                        # 8-byte Folded Spill
	stfd 28, 1096(1)                        # 8-byte Folded Spill
	stfd 29, 1104(1)                        # 8-byte Folded Spill
.Lcs_BB7_2:                                # %for.body
                                        # =>This Loop Header: Depth=1
                                        #     Child Loop BB7_3 Depth 2
                                        #     Child Loop BB7_5 Depth 2
	slwi 0, 9, 8
	li 28, 0
	add 29, 0, 5
	mr	27, 7
	addi 29, 29, -16
	mr	26, 8
.Lcs_BB7_3:                                # %for.body7
                                        #   Parent Loop BB7_2 Depth=1
                                        # =>  This Inner Loop Header: Depth=2
	lfsu 29, 16(29)
	mr	25, 28
	addi 0, 28, 16
	cmplwi	0, 256
	lfs 28, 8(29)
	stfsux 28, 25, 11
	stfsu 29, 16(27)
	stfs 28, 12(25)
	stfs 28, 8(25)
	stfs 28, 4(25)
	lfs 28, 12(29)
	stfs 29, 12(27)
	stfs 29, 8(27)
	stfs 29, 4(27)
	lfs 29, 4(29)
	stfsux 28, 28, 4
	stfsu 29, 16(26)
	stfs 28, 12(28)
	stfs 28, 8(28)
	stfs 28, 4(28)
	mr	28, 0
	stfs 29, 12(26)
	stfs 29, 8(26)
	stfs 29, 4(26)
	bne	0, .Lcs_BB7_3
# %bb.4:                                # %for.cond17.preheader
                                        #   in Loop: Header=BB7_2 Depth=1
	li 29, 0
	mr	28, 7
	mr	27, 8
	mr	26, 10
.Lcs_BB7_5:                                # %for.body20
                                        #   Parent Loop BB7_2 Depth=1
                                        # =>  This Inner Loop Header: Depth=2
	lfsu 28, 16(28)
	mr	25, 29
	mr	24, 3
	fmuls 28, 28, 3
	lfsu 27, 16(27)
	fmuls 27, 27, 7
	lfsu 29, 16(26)
	fmadds 29, 29, 11, 28
	lfsux 26, 25, 4
	fmadds 28, 26, 30, 27
	fadds 29, 29, 28
	lfs 27, 8(28)
	lfs 26, 12(28)
	fmuls 28, 27, 1
	lfs 27, 4(28)
	fmuls 26, 26, 0
	lfs 22, 12(26)
	fmuls 27, 27, 2
	lfs 21, 4(26)
	fmadds 26, 22, 8, 26
	lfs 23, 8(27)
	lfs 22, 8(26)
	fmuls 23, 23, 5
	stfsux 29, 24, 29
	fmadds 29, 21, 10, 27
	addi 29, 29, 16
	cmplwi	29, 256
	fmadds 28, 22, 9, 28
	lfs 27, 8(25)
	lfs 25, 4(27)
	fmadds 27, 27, 13, 23
	fadds 28, 28, 27
	lfs 22, 4(25)
	lfs 24, 12(27)
	fmuls 25, 25, 6
	lfs 27, 12(25)
	fmuls 24, 24, 4
	stfs 28, 8(24)
	fmadds 28, 22, 31, 25
	fadds 29, 29, 28
	stfs 29, 4(24)
	fmadds 29, 27, 12, 24
	fadds 29, 26, 29
	stfs 29, 12(24)
	bne	0, .Lcs_BB7_5
# %bb.6:                                # %for.cond.cleanup19
                                        #   in Loop: Header=BB7_2 Depth=1
	addi 30, 30, 16
	addi 12, 12, 256
	cmpw	30, 6
	addi 3, 3, 256
	addi 9, 9, 1
	blt	0, .Lcs_BB7_2
# %bb.7:
	lfd 31, 1120(1)                         # 8-byte Folded Reload
	lfd 30, 1112(1)                         # 8-byte Folded Reload
	lfd 29, 1104(1)                         # 8-byte Folded Reload
	lfd 28, 1096(1)                         # 8-byte Folded Reload
	lfd 27, 1088(1)                         # 8-byte Folded Reload
	lfd 26, 1080(1)                         # 8-byte Folded Reload
	lfd 25, 1072(1)                         # 8-byte Folded Reload
	lfd 24, 1064(1)                         # 8-byte Folded Reload
	lfd 23, 1056(1)                         # 8-byte Folded Reload
	lfd 22, 1048(1)                         # 8-byte Folded Reload
	lfd 21, 1040(1)                         # 8-byte Folded Reload
	ld 30, 1176(1)                          # 8-byte Folded Reload
	ld 29, 1168(1)                          # 8-byte Folded Reload
	ld 28, 1160(1)                          # 8-byte Folded Reload
	ld 27, 1152(1)                          # 8-byte Folded Reload
	ld 26, 1144(1)                          # 8-byte Folded Reload
	ld 25, 1136(1)                          # 8-byte Folded Reload
	ld 24, 1128(1)                          # 8-byte Folded Reload
	addi 1, 1, 1200
	blr
.Lcs_func_end7:
	.size	cs_sp_mat4_v16, .Lcs_func_end7-.Lcs_func_begin7
                                        # -- End function
	.section	.text.cs_sp_copy_s,"ax",@progbits
	.globl	cs_sp_copy_s                    # -- Begin function cs_sp_copy_s
	.p2align	2
	.type	cs_sp_copy_s,@function
cs_sp_copy_s:                           # @cs_sp_copy_s
.Lcs_func_begin8:
# %bb.0:                                # %entry
	cmpwi	5, 1
	bltlr	0
# %bb.1:                                # %for.body.preheader
	li 6, 0
	beq	0, .Lcs_BB8_5
# %bb.2:                                # %vector.ph
	rlwinm 6, 5, 0, 1, 30
	addi 7, 4, -8
	addi 8, 3, -8
	mr	9, 6
.Lcs_BB8_3:                                # %vector.body
                                        # =>This Inner Loop Header: Depth=1
	lwzu 10, 8(7)
	addi 9, 9, -2
	cmplwi	9, 0
	stwu 10, 8(8)
	lwz 10, 4(7)
	stw 10, 4(8)
	bne	0, .Lcs_BB8_3
# %bb.4:                                # %middle.block
	cmplw	5, 6
	beqlr	0
.Lcs_BB8_5:                                # %for.body.preheader7
	slwi 7, 6, 2
	sub	5, 5, 6
	addi 6, 7, -4
	add 4, 4, 6
	add 3, 3, 6
.Lcs_BB8_6:                                # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lwzu 6, 4(4)
	addi 5, 5, -1
	cmplwi	5, 0
	stwu 6, 4(3)
	bne	0, .Lcs_BB8_6
	blr
.Lcs_func_end8:
	.size	cs_sp_copy_s, .Lcs_func_end8-.Lcs_func_begin8
                                        # -- End function
	.section	.text.cs_sp_copy_v,"ax",@progbits
	.globl	cs_sp_copy_v                    # -- Begin function cs_sp_copy_v
	.p2align	2
	.type	cs_sp_copy_v,@function
cs_sp_copy_v:                           # @cs_sp_copy_v
.Lcs_func_begin9:
# %bb.0:                                # %entry
	srawi 6, 5, 2
	cmpwi	5, 4
	addze 5, 6
	bltlr	0
# %bb.1:                                # %for.body.preheader
	addi 4, 4, -16
	addi 3, 3, -16
.Lcs_BB9_2:                                # %for.body
                                        # =>This Inner Loop Header: Depth=1
	ldu 6, 16(4)
	addi 5, 5, -1
	cmplwi	5, 0
	stdu 6, 16(3)
	ld 6, 8(4)
	std 6, 8(3)
	bne	0, .Lcs_BB9_2
# %bb.3:                                # %for.cond.cleanup
	blr
.Lcs_func_end9:
	.size	cs_sp_copy_v, .Lcs_func_end9-.Lcs_func_begin9
                                        # -- End function
	.section	.text.cs_sp_bright_s,"ax",@progbits
	.globl	cs_sp_bright_s                  # -- Begin function cs_sp_bright_s
	.p2align	2
	.type	cs_sp_bright_s,@function
cs_sp_bright_s:                         # @cs_sp_bright_s
.Lcs_func_begin10:
# %bb.0:                                # %entry
	cmpwi	5, 1
	bltlr	0
# %bb.1:                                # %for.body.preheader
	li 6, 0
	beq	0, .Lcs_BB10_9
# %bb.2:                                # %vector.ph
	rlwinm 6, 5, 0, 1, 30
	addi 7, 4, -2
	addi 8, 3, -2
	li 9, 0
	b .Lcs_BB10_4
.Lcs_BB10_3:                               # %vector.body
                                        #   in Loop: Header=BB10_4 Depth=1
	addi 9, 9, 2
	stb 11, 1(8)
	cmplw	6, 9
	beq	0, .Lcs_BB10_8
.Lcs_BB10_4:                               # %vector.body
                                        # =>This Inner Loop Header: Depth=1
	lbzu 10, 2(7)
	addi 10, 10, 40
	cmplwi	10, 255
	blt	0, .Lcs_BB10_6
# %bb.5:                                # %vector.body
                                        #   in Loop: Header=BB10_4 Depth=1
	li 10, 255
.Lcs_BB10_6:                               # %vector.body
                                        #   in Loop: Header=BB10_4 Depth=1
	lbz 11, 1(7)
	addi 11, 11, 40
	stbu 10, 2(8)
	cmplwi	11, 255
	blt	0, .Lcs_BB10_3
# %bb.7:                                # %vector.body
                                        #   in Loop: Header=BB10_4 Depth=1
	li 11, 255
	b .Lcs_BB10_3
.Lcs_BB10_8:                               # %middle.block
	cmplw	5, 6
	beqlr	0
.Lcs_BB10_9:                               # %for.body.preheader11
	addi 7, 6, -1
	add 4, 4, 7
	add 3, 3, 7
	b .Lcs_BB10_11
.Lcs_BB10_10:                              # %for.body
                                        #   in Loop: Header=BB10_11 Depth=1
	addi 6, 6, 1
	stbu 7, 1(3)
	cmplw	5, 6
	beqlr	0
.Lcs_BB10_11:                              # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lbzu 7, 1(4)
	addi 7, 7, 40
	cmplwi	7, 255
	blt	0, .Lcs_BB10_10
# %bb.12:                               # %for.body
                                        #   in Loop: Header=BB10_11 Depth=1
	li 7, 255
	b .Lcs_BB10_10
.Lcs_func_end10:
	.size	cs_sp_bright_s, .Lcs_func_end10-.Lcs_func_begin10
                                        # -- End function
	.section	.text.cs_sp_bright_v,"ax",@progbits
	.globl	cs_sp_bright_v                  # -- Begin function cs_sp_bright_v
	.p2align	2
	.type	cs_sp_bright_v,@function
cs_sp_bright_v:                         # @cs_sp_bright_v
.Lcs_func_begin11:
# %bb.0:                                # %entry
	srawi 6, 5, 4
	cmpwi	5, 16
	addze 5, 6
	bltlr	0
# %bb.1:                                # %for.body.preheader
	stwu 1, -96(1)
	addi 4, 4, -16
	addi 3, 3, -16
	std 23, 16(1)                           # 8-byte Folded Spill
	std 24, 24(1)                           # 8-byte Folded Spill
	std 25, 32(1)                           # 8-byte Folded Spill
	std 26, 40(1)                           # 8-byte Folded Spill
	std 27, 48(1)                           # 8-byte Folded Spill
	std 28, 56(1)                           # 8-byte Folded Spill
	std 29, 64(1)                           # 8-byte Folded Spill
	std 30, 72(1)                           # 8-byte Folded Spill
	b .Lcs_BB11_3
.Lcs_BB11_2:                               # %for.body
                                        #   in Loop: Header=BB11_3 Depth=1
	addi 5, 5, -1
	stb 27, 15(3)
	cmplwi	5, 0
	stb 25, 14(3)
	stb 23, 13(3)
	stb 24, 12(3)
	stb 26, 11(3)
	stb 28, 10(3)
	stb 29, 9(3)
	stb 30, 8(3)
	stb 12, 7(3)
	stb 11, 6(3)
	stb 10, 5(3)
	stb 9, 4(3)
	stb 8, 3(3)
	stb 7, 2(3)
	stb 6, 1(3)
	beq	0, .Lcs_BB11_35
.Lcs_BB11_3:                               # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lbzu 6, 16(4)
	addi 7, 6, 40
	cmplwi	7, 255
	blt	0, .Lcs_BB11_5
# %bb.4:                                # %for.body
                                        #   in Loop: Header=BB11_3 Depth=1
	li 7, 255
.Lcs_BB11_5:                               # %for.body
                                        #   in Loop: Header=BB11_3 Depth=1
	lbz 6, 1(4)
	addi 6, 6, 40
	stbu 7, 16(3)
	cmplwi	6, 255
	bge 0, .Lcs_BB11_21
# %bb.6:                                # %for.body
                                        #   in Loop: Header=BB11_3 Depth=1
	lbz 7, 2(4)
	addi 7, 7, 40
	cmplwi	7, 255
	bge 0, .Lcs_BB11_22
.Lcs_BB11_7:                               # %for.body
                                        #   in Loop: Header=BB11_3 Depth=1
	lbz 8, 3(4)
	addi 8, 8, 40
	cmplwi	8, 255
	bge 0, .Lcs_BB11_23
.Lcs_BB11_8:                               # %for.body
                                        #   in Loop: Header=BB11_3 Depth=1
	lbz 9, 4(4)
	addi 9, 9, 40
	cmplwi	9, 255
	bge 0, .Lcs_BB11_24
.Lcs_BB11_9:                               # %for.body
                                        #   in Loop: Header=BB11_3 Depth=1
	lbz 10, 5(4)
	addi 10, 10, 40
	cmplwi	10, 255
	bge 0, .Lcs_BB11_25
.Lcs_BB11_10:                              # %for.body
                                        #   in Loop: Header=BB11_3 Depth=1
	lbz 11, 6(4)
	addi 11, 11, 40
	cmplwi	11, 255
	bge 0, .Lcs_BB11_26
.Lcs_BB11_11:                              # %for.body
                                        #   in Loop: Header=BB11_3 Depth=1
	lbz 12, 7(4)
	addi 12, 12, 40
	cmplwi	12, 255
	bge 0, .Lcs_BB11_27
.Lcs_BB11_12:                              # %for.body
                                        #   in Loop: Header=BB11_3 Depth=1
	lbz 30, 8(4)
	addi 30, 30, 40
	cmplwi	30, 255
	bge 0, .Lcs_BB11_28
.Lcs_BB11_13:                              # %for.body
                                        #   in Loop: Header=BB11_3 Depth=1
	lbz 29, 9(4)
	addi 29, 29, 40
	cmplwi	29, 255
	blt	0, .Lcs_BB11_15
.Lcs_BB11_14:                              # %for.body
                                        #   in Loop: Header=BB11_3 Depth=1
	li 29, 255
.Lcs_BB11_15:                              # %for.body
                                        #   in Loop: Header=BB11_3 Depth=1
	lbz 28, 10(4)
	lbz 27, 15(4)
	addi 28, 28, 40
	cmplwi	28, 255
	lbz 25, 14(4)
	lbz 23, 13(4)
	lbz 24, 12(4)
	lbz 26, 11(4)
	bge 0, .Lcs_BB11_29
# %bb.16:                               # %for.body
                                        #   in Loop: Header=BB11_3 Depth=1
	addi 26, 26, 40
	cmplwi	26, 255
	bge 0, .Lcs_BB11_30
.Lcs_BB11_17:                              # %for.body
                                        #   in Loop: Header=BB11_3 Depth=1
	addi 24, 24, 40
	cmplwi	24, 255
	bge 0, .Lcs_BB11_31
.Lcs_BB11_18:                              # %for.body
                                        #   in Loop: Header=BB11_3 Depth=1
	addi 23, 23, 40
	cmplwi	23, 255
	bge 0, .Lcs_BB11_32
.Lcs_BB11_19:                              # %for.body
                                        #   in Loop: Header=BB11_3 Depth=1
	addi 25, 25, 40
	cmplwi	25, 255
	bge 0, .Lcs_BB11_33
.Lcs_BB11_20:                              # %for.body
                                        #   in Loop: Header=BB11_3 Depth=1
	addi 27, 27, 40
	cmplwi	27, 255
	blt	0, .Lcs_BB11_2
	b .Lcs_BB11_34
.Lcs_BB11_21:                              # %for.body
                                        #   in Loop: Header=BB11_3 Depth=1
	li 6, 255
	lbz 7, 2(4)
	addi 7, 7, 40
	cmplwi	7, 255
	blt	0, .Lcs_BB11_7
.Lcs_BB11_22:                              # %for.body
                                        #   in Loop: Header=BB11_3 Depth=1
	li 7, 255
	lbz 8, 3(4)
	addi 8, 8, 40
	cmplwi	8, 255
	blt	0, .Lcs_BB11_8
.Lcs_BB11_23:                              # %for.body
                                        #   in Loop: Header=BB11_3 Depth=1
	li 8, 255
	lbz 9, 4(4)
	addi 9, 9, 40
	cmplwi	9, 255
	blt	0, .Lcs_BB11_9
.Lcs_BB11_24:                              # %for.body
                                        #   in Loop: Header=BB11_3 Depth=1
	li 9, 255
	lbz 10, 5(4)
	addi 10, 10, 40
	cmplwi	10, 255
	blt	0, .Lcs_BB11_10
.Lcs_BB11_25:                              # %for.body
                                        #   in Loop: Header=BB11_3 Depth=1
	li 10, 255
	lbz 11, 6(4)
	addi 11, 11, 40
	cmplwi	11, 255
	blt	0, .Lcs_BB11_11
.Lcs_BB11_26:                              # %for.body
                                        #   in Loop: Header=BB11_3 Depth=1
	li 11, 255
	lbz 12, 7(4)
	addi 12, 12, 40
	cmplwi	12, 255
	blt	0, .Lcs_BB11_12
.Lcs_BB11_27:                              # %for.body
                                        #   in Loop: Header=BB11_3 Depth=1
	li 12, 255
	lbz 30, 8(4)
	addi 30, 30, 40
	cmplwi	30, 255
	blt	0, .Lcs_BB11_13
.Lcs_BB11_28:                              # %for.body
                                        #   in Loop: Header=BB11_3 Depth=1
	li 30, 255
	lbz 29, 9(4)
	addi 29, 29, 40
	cmplwi	29, 255
	bge 0, .Lcs_BB11_14
	b .Lcs_BB11_15
.Lcs_BB11_29:                              # %for.body
                                        #   in Loop: Header=BB11_3 Depth=1
	li 28, 255
	addi 26, 26, 40
	cmplwi	26, 255
	blt	0, .Lcs_BB11_17
.Lcs_BB11_30:                              # %for.body
                                        #   in Loop: Header=BB11_3 Depth=1
	li 26, 255
	addi 24, 24, 40
	cmplwi	24, 255
	blt	0, .Lcs_BB11_18
.Lcs_BB11_31:                              # %for.body
                                        #   in Loop: Header=BB11_3 Depth=1
	li 24, 255
	addi 23, 23, 40
	cmplwi	23, 255
	blt	0, .Lcs_BB11_19
.Lcs_BB11_32:                              # %for.body
                                        #   in Loop: Header=BB11_3 Depth=1
	li 23, 255
	addi 25, 25, 40
	cmplwi	25, 255
	blt	0, .Lcs_BB11_20
.Lcs_BB11_33:                              # %for.body
                                        #   in Loop: Header=BB11_3 Depth=1
	li 25, 255
	addi 27, 27, 40
	cmplwi	27, 255
	blt	0, .Lcs_BB11_2
.Lcs_BB11_34:                              # %for.body
                                        #   in Loop: Header=BB11_3 Depth=1
	li 27, 255
	b .Lcs_BB11_2
.Lcs_BB11_35:
	ld 30, 72(1)                            # 8-byte Folded Reload
	ld 29, 64(1)                            # 8-byte Folded Reload
	ld 28, 56(1)                            # 8-byte Folded Reload
	ld 27, 48(1)                            # 8-byte Folded Reload
	ld 26, 40(1)                            # 8-byte Folded Reload
	ld 25, 32(1)                            # 8-byte Folded Reload
	ld 24, 24(1)                            # 8-byte Folded Reload
	ld 23, 16(1)                            # 8-byte Folded Reload
	addi 1, 1, 96
	blr
.Lcs_func_end11:
	.size	cs_sp_bright_v, .Lcs_func_end11-.Lcs_func_begin11
                                        # -- End function
	.section	.rodata.cst4,"aM",@progbits,4
	.p2align	2, 0x0                          # -- Begin function cs_sp_u8f_s
.Lcs_CPI12_0:
	.long	0x3b808081                      # float 0.00392156886
	.section	.text.cs_sp_u8f_s,"ax",@progbits
	.globl	cs_sp_u8f_s
	.p2align	2
	.type	cs_sp_u8f_s,@function
cs_sp_u8f_s:                            # @cs_sp_u8f_s
.Lcs_func_begin12:
# %bb.0:                                # %entry
	cmpwi	5, 1
	bltlr	0
# %bb.1:                                # %for.body.preheader
	stwu 1, -48(1)
	li 6, 0
	beq	0, .Lcs_BB12_5
# %bb.2:                                # %vector.ph
	lis 9, .Lcs_CPI12_0@ha
	rlwinm 6, 5, 0, 1, 30
	lfs 0, .Lcs_CPI12_0@l(9)
	addi 7, 4, -2
	addi 8, 3, -8
	mr	9, 6
.Lcs_BB12_3:                               # %vector.body
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
	bne	0, .Lcs_BB12_3
# %bb.4:                                # %middle.block
	cmplw	5, 6
	beq	0, .Lcs_BB12_7
.Lcs_BB12_5:                               # %for.body.preheader7
	sub	5, 5, 6
	slwi 7, 6, 2
	add 4, 6, 4
	lis 6, .Lcs_CPI12_0@ha
	lfs 0, .Lcs_CPI12_0@l(6)
	add 7, 7, 3
	addi 3, 4, -1
	addi 4, 7, -4
.Lcs_BB12_6:                               # %for.body
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
	bne	0, .Lcs_BB12_6
.Lcs_BB12_7:
	addi 1, 1, 48
	blr
.Lcs_func_end12:
	.size	cs_sp_u8f_s, .Lcs_func_end12-.Lcs_func_begin12
                                        # -- End function
	.section	.rodata.cst4,"aM",@progbits,4
	.p2align	2, 0x0                          # -- Begin function cs_sp_u8f_v
.Lcs_CPI13_0:
	.long	0x3b808081                      # float 0.00392156886
	.section	.text.cs_sp_u8f_v,"ax",@progbits
	.globl	cs_sp_u8f_v
	.p2align	2
	.type	cs_sp_u8f_v,@function
cs_sp_u8f_v:                            # @cs_sp_u8f_v
.Lcs_func_begin13:
# %bb.0:                                # %entry
	srawi 6, 5, 4
	cmpwi	5, 16
	addze 5, 6
	bltlr	0
# %bb.1:                                # %for.body.preheader
	stwu 1, -224(1)
	lis 6, .Lcs_CPI13_0@ha
	addi 4, 4, -16
	std 24, 152(1)                          # 8-byte Folded Spill
	addi 3, 3, -64
	lfs 0, .Lcs_CPI13_0@l(6)
	std 25, 160(1)                          # 8-byte Folded Spill
	std 26, 168(1)                          # 8-byte Folded Spill
	std 27, 176(1)                          # 8-byte Folded Spill
	std 28, 184(1)                          # 8-byte Folded Spill
	std 29, 192(1)                          # 8-byte Folded Spill
	std 30, 200(1)                          # 8-byte Folded Spill
.Lcs_BB13_2:                               # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lbzu 6, 16(4)
	addi 5, 5, -1
	cmplwi	5, 0
	std 6, 144(1)
	lfd 1, 144(1)
	fcfid 1, 1
	lbz 24, 3(4)
	frsp 1, 1
	lbz 25, 2(4)
	fmuls 1, 1, 0
	lbz 6, 12(4)
	stfsu 1, 64(3)
	lbz 7, 13(4)
	lbz 8, 14(4)
	lbz 9, 15(4)
	lbz 10, 8(4)
	lbz 11, 9(4)
	lbz 12, 10(4)
	lbz 0, 11(4)
	lbz 30, 4(4)
	lbz 29, 5(4)
	lbz 28, 6(4)
	lbz 27, 7(4)
	lbz 26, 1(4)
	std 24, 128(1)
	std 25, 120(1)
	lfd 1, 128(1)
	lfd 2, 120(1)
	fcfid 1, 1
	frsp 1, 1
	std 26, 136(1)
	lfd 3, 136(1)
	fcfid 2, 2
	fmuls 1, 1, 0
	stfs 1, 12(3)
	frsp 2, 2
	fcfid 3, 3
	fmuls 1, 2, 0
	stfs 1, 8(3)
	frsp 1, 3
	fmuls 1, 1, 0
	stfs 1, 4(3)
	std 27, 104(1)
	lfd 1, 104(1)
	std 29, 112(1)
	fcfid 1, 1
	frsp 1, 1
	lfd 3, 112(1)
	std 28, 96(1)
	fcfid 3, 3
	fmuls 1, 1, 0
	std 30, 88(1)
	lfd 2, 96(1)
	lfd 4, 88(1)
	fcfid 2, 2
	frsp 2, 2
	stfs 1, 28(3)
	frsp 1, 3
	fcfid 4, 4
	fmuls 1, 1, 0
	stfs 1, 20(3)
	frsp 1, 4
	fmuls 2, 2, 0
	fmuls 1, 1, 0
	stfs 2, 24(3)
	stfs 1, 16(3)
	std 0, 72(1)
	lfd 1, 72(1)
	std 11, 80(1)
	fcfid 1, 1
	frsp 1, 1
	lfd 3, 80(1)
	std 12, 64(1)
	fcfid 3, 3
	fmuls 1, 1, 0
	std 10, 24(1)
	lfd 2, 64(1)
	lfd 4, 24(1)
	fcfid 2, 2
	frsp 2, 2
	stfs 1, 44(3)
	frsp 1, 3
	fcfid 4, 4
	fmuls 1, 1, 0
	stfs 1, 36(3)
	frsp 1, 4
	fmuls 2, 2, 0
	fmuls 1, 1, 0
	stfs 2, 40(3)
	stfs 1, 32(3)
	std 9, 48(1)
	lfd 1, 48(1)
	std 7, 56(1)
	fcfid 1, 1
	frsp 1, 1
	lfd 3, 56(1)
	std 8, 40(1)
	fcfid 3, 3
	fmuls 1, 1, 0
	std 6, 32(1)
	lfd 2, 40(1)
	lfd 4, 32(1)
	fcfid 2, 2
	frsp 2, 2
	stfs 1, 60(3)
	frsp 1, 3
	fcfid 4, 4
	fmuls 1, 1, 0
	stfs 1, 52(3)
	frsp 1, 4
	fmuls 2, 2, 0
	fmuls 1, 1, 0
	stfs 2, 56(3)
	stfs 1, 48(3)
	bne	0, .Lcs_BB13_2
# %bb.3:
	ld 30, 200(1)                           # 8-byte Folded Reload
	ld 29, 192(1)                           # 8-byte Folded Reload
	ld 28, 184(1)                           # 8-byte Folded Reload
	ld 27, 176(1)                           # 8-byte Folded Reload
	ld 26, 168(1)                           # 8-byte Folded Reload
	ld 25, 160(1)                           # 8-byte Folded Reload
	ld 24, 152(1)                           # 8-byte Folded Reload
	addi 1, 1, 224
	blr
.Lcs_func_end13:
	.size	cs_sp_u8f_v, .Lcs_func_end13-.Lcs_func_begin13
                                        # -- End function
	.section	.rodata.cst4,"aM",@progbits,4
	.p2align	2, 0x0                          # -- Begin function cs_sp_f2u8_s
.Lcs_CPI14_0:
	.long	0x3f000000                      # float 0.5
.Lcs_CPI14_1:
	.long	0x437f0000                      # float 255
.Lcs_CPI14_2:
	.long	0x00000000                      # float 0
	.section	.text.cs_sp_f2u8_s,"ax",@progbits
	.globl	cs_sp_f2u8_s
	.p2align	2
	.type	cs_sp_f2u8_s,@function
cs_sp_f2u8_s:                           # @cs_sp_f2u8_s
.Lcs_func_begin14:
# %bb.0:                                # %entry
	cmpwi	5, 1
	bltlr	0
# %bb.1:                                # %for.body.preheader
	stwu 1, -32(1)
	lis 6, .Lcs_CPI14_0@ha
	addi 4, 4, -4
	lfs 0, .Lcs_CPI14_0@l(6)
	lis 6, .Lcs_CPI14_1@ha
	addi 3, 3, -1
	lfs 1, .Lcs_CPI14_1@l(6)
	lis 6, .Lcs_CPI14_2@ha
	lfs 2, .Lcs_CPI14_2@l(6)
.Lcs_BB14_2:                               # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lfsu 3, 4(4)
	addi 5, 5, -1
	cmplwi	5, 0
	fmadds 3, 3, 1, 0
	fsubs 4, 2, 3
	fsel 3, 4, 2, 3
	fsubs 4, 3, 1
	fsel 3, 4, 1, 3
	fctidz 3, 3
	stfd 3, 24(1)
	lwz 6, 28(1)
	stbu 6, 1(3)
	bne	0, .Lcs_BB14_2
# %bb.3:
	addi 1, 1, 32
	blr
.Lcs_func_end14:
	.size	cs_sp_f2u8_s, .Lcs_func_end14-.Lcs_func_begin14
                                        # -- End function
	.section	.rodata.cst4,"aM",@progbits,4
	.p2align	2, 0x0                          # -- Begin function cs_sp_f2u8_v
.Lcs_CPI15_0:
	.long	0x3f000000                      # float 0.5
.Lcs_CPI15_1:
	.long	0x437f0000                      # float 255
.Lcs_CPI15_2:
	.long	0x00000000                      # float 0
	.section	.text.cs_sp_f2u8_v,"ax",@progbits
	.globl	cs_sp_f2u8_v
	.p2align	2
	.type	cs_sp_f2u8_v,@function
cs_sp_f2u8_v:                           # @cs_sp_f2u8_v
.Lcs_func_begin15:
# %bb.0:                                # %entry
	srawi 6, 5, 4
	cmpwi	5, 16
	addze 5, 6
	bltlr	0
# %bb.1:                                # %for.body.lr.ph
	stwu 1, -256(1)
	lis 7, .Lcs_CPI15_0@ha
	lis 8, .Lcs_CPI15_1@ha
	std 15, 112(1)                          # 8-byte Folded Spill
	lis 11, .Lcs_CPI15_2@ha
	addi 6, 1, 64
	lfs 0, .Lcs_CPI15_0@l(7)
	addi 7, 1, 80
	li 9, 0
	lfs 1, .Lcs_CPI15_1@l(8)
	addi 8, 1, 96
	addi 10, 1, 32
	lfs 2, .Lcs_CPI15_2@l(11)
	mr	11, 4
	std 16, 120(1)                          # 8-byte Folded Spill
	std 17, 128(1)                          # 8-byte Folded Spill
	std 18, 136(1)                          # 8-byte Folded Spill
	std 19, 144(1)                          # 8-byte Folded Spill
	std 20, 152(1)                          # 8-byte Folded Spill
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
.Lcs_BB15_2:                               # %for.body
                                        # =>This Loop Header: Depth=1
                                        #     Child Loop BB15_3 Depth 2
	slwi 12, 9, 6
	li 30, 0
	add 12, 12, 4
	mr	29, 10
	addi 12, 12, -16
.Lcs_BB15_3:                               # %for.body4
                                        #   Parent Loop BB15_2 Depth=1
                                        # =>  This Inner Loop Header: Depth=2
	lfsu 3, 16(12)
	addi 30, 30, 16
	cmplwi	30, 64
	fmadds 3, 3, 1, 0
	fsubs 4, 2, 3
	fsel 3, 4, 2, 3
	fsubs 4, 3, 1
	fsel 3, 4, 1, 3
	fctidz 3, 3
	stfd 3, 40(1)
	lfs 5, 4(12)
	lfs 4, 8(12)
	fmadds 5, 5, 1, 0
	fsubs 6, 2, 5
	lfs 3, 12(12)
	fmadds 4, 4, 1, 0
	lwz 0, 44(1)
	fmadds 3, 3, 1, 0
	stwu 0, 16(29)
	fsel 5, 6, 2, 5
	fsubs 6, 2, 4
	fsel 4, 6, 2, 4
	fsubs 6, 2, 3
	fsel 3, 6, 2, 3
	fsubs 6, 4, 1
	fsel 4, 6, 1, 4
	fsubs 6, 5, 1
	fsel 5, 6, 1, 5
	fsubs 6, 3, 1
	fsel 3, 6, 1, 3
	fctidz 5, 5
	stfd 5, 32(1)
	fctidz 4, 4
	lwz 0, 36(1)
	fctidz 3, 3
	stfd 4, 16(1)
	stfd 3, 24(1)
	lwz 28, 20(1)
	lwz 27, 28(1)
	stw 27, 12(29)
	stw 28, 8(29)
	stw 0, 4(29)
	bne	0, .Lcs_BB15_3
# %bb.4:                                # %for.cond.cleanup3
                                        #   in Loop: Header=BB15_2 Depth=1
	lbz 29, 51(1)
	slwi 16, 9, 4
	mr	15, 3
	addi 9, 9, 1
	addi 11, 11, 64
	lbz 12, 55(1)
	cmplw	9, 5
	lbz 0, 59(1)
	lbz 30, 63(1)
	lbz 28, 3(6)
	lbz 27, 7(6)
	lbz 26, 11(6)
	lbz 25, 15(6)
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
	bne	0, .Lcs_BB15_2
# %bb.5:
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
	ld 18, 136(1)                           # 8-byte Folded Reload
	ld 17, 128(1)                           # 8-byte Folded Reload
	ld 16, 120(1)                           # 8-byte Folded Reload
	ld 15, 112(1)                           # 8-byte Folded Reload
	addi 1, 1, 256
	blr
.Lcs_func_end15:
	.size	cs_sp_f2u8_v, .Lcs_func_end15-.Lcs_func_begin15
                                        # -- End function
