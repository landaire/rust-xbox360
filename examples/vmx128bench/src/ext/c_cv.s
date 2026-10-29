# speed.c: clang version 22.1.8 (git@github.com:landaire/llvm-project.git 5f59e9fff8d5dc5b1fa65127b3579d4b1a71f950)
# flags: --target=powerpc64-unknown-xbox360 -ffreestanding -fno-builtin -fms-extensions -ffunction-sections -fdata-sections -fno-ident -fno-addrsig -fno-asynchronous-unwind-tables -g0 -O3 -ffast-math
	.section	.text.cv_sp_saxpy_s,"ax",@progbits
	.globl	cv_sp_saxpy_s                   # -- Begin function cv_sp_saxpy_s
	.p2align	2
	.type	cv_sp_saxpy_s,@function
cv_sp_saxpy_s:                          # @cv_sp_saxpy_s
.Lcv_func_begin0:
# %bb.0:                                # %entry
	cmpwi	6, 1
	bltlr	0
# %bb.1:                                # %for.body.preheader
	stwu 1, -48(1)
	cmplwi	6, 4
	li 5, 0
	blt	0, .Lcv_BB0_5
# %bb.2:                                # %vector.ph
	stfs 1, 32(1)
	addi 5, 1, 32
	li 7, 0
	li 8, 15
	addi 9, 1, 16
	lvx128 2, 0, 5
	rlwinm 5, 6, 0, 1, 29
	mr	10, 5
	vspltw128 2, 2, 0
.Lcv_BB0_3:                                # %vector.body
                                        # =>This Inner Loop Header: Depth=1
	add 11, 4, 7
	lvx128 4, 4, 7
	addi 10, 10, -4
	cmplwi	10, 0
	lvx128 5, 11, 8
	add 11, 3, 7
	lvx128 0, 3, 7
	lvx128 1, 11, 8
	lvsl 3, 4, 7
	vperm128 3, 4, 5, 3
	lvsl 4, 3, 7
	vperm128 4, 0, 1, 4
	vmaddfp128 4, 3, 2, 4
	stvx128 4, 0, 9
	ld 12, 16(1)
	stdx 12, 3, 7
	addi 7, 7, 16
	ld 12, 24(1)
	std 12, 8(11)
	bne	0, .Lcv_BB0_3
# %bb.4:                                # %middle.block
	cmplw	6, 5
	beq	0, .Lcv_BB0_7
.Lcv_BB0_5:                                # %for.body.preheader11
	slwi 7, 5, 2
	sub	5, 6, 5
	addi 6, 7, -4
	add 4, 4, 6
	add 3, 3, 6
.Lcv_BB0_6:                                # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lfsu 0, 4(3)
	addi 5, 5, -1
	cmplwi	5, 0
	lfsu 2, 4(4)
	fmadds 0, 2, 1, 0
	stfs 0, 0(3)
	bne	0, .Lcv_BB0_6
.Lcv_BB0_7:
	addi 1, 1, 48
	blr
.Lcv_func_end0:
	.size	cv_sp_saxpy_s, .Lcv_func_end0-.Lcv_func_begin0
                                        # -- End function
	.section	.text.cv_sp_saxpy_v,"ax",@progbits
	.globl	cv_sp_saxpy_v                   # -- Begin function cv_sp_saxpy_v
	.p2align	2
	.type	cv_sp_saxpy_v,@function
cv_sp_saxpy_v:                          # @cv_sp_saxpy_v
.Lcv_func_begin1:
# %bb.0:                                # %entry
	stwu 1, -32(1)
	srawi 5, 6, 2
	cmpwi	6, 4
	stfs 1, 16(1)
	addze 5, 5
	blt	0, .Lcv_BB1_3
# %bb.1:                                # %for.body.preheader
	addi 6, 1, 16
	lvx128 2, 0, 6
	li 6, 0
	vspltw128 2, 2, 0
.Lcv_BB1_2:                                # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lvx128 3, 4, 6
	addi 5, 5, -1
	cmplwi	5, 0
	lvx128 4, 3, 6
	vmaddfp128 4, 3, 2, 4
	stvx128 4, 3, 6
	addi 6, 6, 16
	bne	0, .Lcv_BB1_2
.Lcv_BB1_3:                                # %for.cond.cleanup
	addi 1, 1, 32
	blr
.Lcv_func_end1:
	.size	cv_sp_saxpy_v, .Lcv_func_end1-.Lcv_func_begin1
                                        # -- End function
	.section	.rodata.cst4,"aM",@progbits,4
	.p2align	2, 0x0                          # -- Begin function cv_sp_dot_s
.Lcv_CPI2_0:
	.long	0x00000000                      # float 0
	.section	.text.cv_sp_dot_s,"ax",@progbits
	.globl	cv_sp_dot_s
	.p2align	2
	.type	cv_sp_dot_s,@function
cv_sp_dot_s:                            # @cv_sp_dot_s
.Lcv_func_begin2:
# %bb.0:                                # %entry
	cmpwi	5, 1
	lis 7, .Lcv_CPI2_0@ha
	blt	0, .Lcv_BB2_3
# %bb.1:                                # %for.body.preheader
	stwu 1, -32(1)
	cmplwi	5, 8
	bge 0, .Lcv_BB2_4
# %bb.2:
	li 6, 0
	lfs 1, .Lcv_CPI2_0@l(7)
	b .Lcv_BB2_7
.Lcv_BB2_3:
	lfs 1, .Lcv_CPI2_0@l(7)
	blr
.Lcv_BB2_4:                                # %vector.ph
	rlwinm 6, 5, 0, 1, 28
	vspltisw128 2, 0
	addi 7, 4, 16
	addi 8, 3, 16
	li 9, 0
	vspltisw128 3, 0
	li 10, 15
	mr	11, 6
.Lcv_BB2_5:                                # %vector.body
                                        # =>This Inner Loop Header: Depth=1
	add 12, 3, 9
	lvx128 0, 3, 9
	addi 11, 11, -8
	cmplwi	11, 0
	lvx128 1, 12, 10
	add 12, 8, 9
	lvx128 6, 8, 9
	lvx128 7, 12, 10
	add 12, 4, 9
	lvx128 8, 4, 9
	lvx128 9, 12, 10
	add 12, 7, 9
	lvsl 5, 3, 9
	vperm128 5, 0, 1, 5
	lvsl 0, 4, 9
	vperm128 0, 8, 9, 0
	vmaddfp128 2, 0, 5, 2
	lvx128 1, 7, 9
	lvx128 5, 12, 10
	lvsl 4, 8, 9
	vperm128 4, 6, 7, 4
	lvsl 0, 7, 9
	addi 9, 9, 32
	vperm128 5, 1, 5, 0
	vmaddfp128 3, 5, 4, 3
	bne	0, .Lcv_BB2_5
# %bb.6:                                # %middle.block
	vaddfp128 2, 3, 2
	addi 7, 1, 16
	cmplw	5, 6
	vsldoi128 3, 2, 2, 8
	vaddfp128 2, 2, 3
	vsldoi128 3, 2, 2, 4
	vaddfp128 2, 2, 3
	stvx128 2, 0, 7
	lfs 1, 16(1)
	beq	0, .Lcv_BB2_9
.Lcv_BB2_7:                                # %for.body.preheader13
	slwi 7, 6, 2
	sub	5, 5, 6
	addi 6, 7, -4
	add 3, 3, 6
	add 4, 4, 6
.Lcv_BB2_8:                                # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lfsu 0, 4(4)
	addi 5, 5, -1
	cmplwi	5, 0
	lfsu 2, 4(3)
	fmadds 1, 0, 2, 1
	bne	0, .Lcv_BB2_8
.Lcv_BB2_9:
	addi 1, 1, 32
	blr
.Lcv_func_end2:
	.size	cv_sp_dot_s, .Lcv_func_end2-.Lcv_func_begin2
                                        # -- End function
	.section	.text.cv_sp_dot_v,"ax",@progbits
	.globl	cv_sp_dot_v                     # -- Begin function cv_sp_dot_v
	.p2align	2
	.type	cv_sp_dot_v,@function
cv_sp_dot_v:                            # @cv_sp_dot_v
.Lcv_func_begin3:
# %bb.0:                                # %entry
	srawi 6, 5, 2
	vspltisw128 2, 0
	cmpwi	5, 4
	addze 5, 6
	blt	0, .Lcv_BB3_4
# %bb.1:                                # %for.body.preheader
	vspltisw128 3, 0
	li 6, 0
	li 7, 16
	li 8, 32
	li 9, 48
	vspltisw128 4, 0
	li 10, 0
	vspltisw128 5, 0
.Lcv_BB3_2:                                # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lvx128 0, 3, 6
	add 11, 3, 6
	add 12, 4, 6
	addi 10, 10, 4
	lvx128 1, 4, 6
	cmpw	10, 5
	addi 6, 6, 64
	vmaddfp128 2, 1, 0, 2
	lvx128 0, 11, 7
	lvx128 1, 12, 7
	vmaddfp128 5, 1, 0, 5
	lvx128 0, 11, 8
	lvx128 1, 12, 8
	vmaddfp128 4, 1, 0, 4
	lvx128 0, 11, 9
	lvx128 1, 12, 9
	vmaddfp128 3, 1, 0, 3
	blt	0, .Lcv_BB3_2
# %bb.3:                                # %for.cond.cleanup.loopexit
	vaddfp128 4, 4, 5
	vaddfp128 3, 4, 3
	vaddfp128 2, 3, 2
.Lcv_BB3_4:                                # %for.cond.cleanup
	stwu 1, -32(1)
	vsldoi128 3, 2, 2, 8
	addi 3, 1, 16
	vaddfp128 2, 2, 3
	vsldoi128 3, 2, 2, 4
	vaddfp128 2, 2, 3
	stvx128 2, 0, 3
	lfs 1, 16(1)
	addi 1, 1, 32
	blr
.Lcv_func_end3:
	.size	cv_sp_dot_v, .Lcv_func_end3-.Lcv_func_begin3
                                        # -- End function
	.section	.rodata.cst16,"aM",@progbits,16
	.p2align	4, 0x0                          # -- Begin function cv_sp_mat4_s
.Lcv_CPI4_0:
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
.Lcv_CPI4_1:
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
.Lcv_CPI4_2:
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
.Lcv_CPI4_3:
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
.Lcv_CPI4_4:
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
	.section	.text.cv_sp_mat4_s,"ax",@progbits
	.globl	cv_sp_mat4_s
	.p2align	2
	.type	cv_sp_mat4_s,@function
cv_sp_mat4_s:                           # @cv_sp_mat4_s
.Lcv_func_begin4:
# %bb.0:                                # %entry
	cmpwi	6, 1
	bltlr	0
# %bb.1:                                # %for.body.lr.ph
	stwu 1, -224(1)
	std 26, 168(1)                          # 8-byte Folded Spill
	li 7, 16
	li 8, 32
	li 9, 48
	li 10, 63
	std 27, 176(1)                          # 8-byte Folded Spill
	cmplwi	6, 4
	std 28, 184(1)                          # 8-byte Folded Spill
	std 29, 192(1)                          # 8-byte Folded Spill
	std 30, 200(1)                          # 8-byte Folded Spill
	lvx128 3, 0, 4
	lvx128 4, 4, 7
	lvx128 5, 4, 8
	lvx128 0, 4, 9
	lvx128 1, 4, 10
	lvsl 2, 0, 4
	li 4, 0
	vperm128 27, 3, 4, 2
	vperm128 26, 4, 5, 2
	vperm128 4, 5, 0, 2
	vperm128 5, 0, 1, 2
	blt	0, .Lcv_BB4_5
# %bb.2:                                # %vector.ph
	li 12, .Lcv_CPI4_0@l
	lis 30, .Lcv_CPI4_0@ha
	vspltw128 16, 27, 0
	lvx128 0, 30, 12
	li 12, .Lcv_CPI4_1@l
	lis 30, .Lcv_CPI4_1@ha
	vspltw128 18, 26, 0
	lvx128 1, 30, 12
	li 12, .Lcv_CPI4_2@l
	lis 30, .Lcv_CPI4_2@ha
	li 0, .Lcv_CPI4_3@l
	lis 29, .Lcv_CPI4_3@ha
	vspltw128 31, 4, 0
	lvx128 6, 30, 12
	li 30, .Lcv_CPI4_4@l
	lis 28, .Lcv_CPI4_4@ha
	vspltw128 29, 5, 0
	lvx128 7, 29, 0
	rlwinm 4, 6, 0, 1, 29
	li 11, 0
	vspltw128 8, 27, 1
	addi 12, 1, 96
	addi 0, 1, 144
	lvx128 2, 28, 30
	addi 30, 1, 128
	addi 29, 1, 112
	vspltw128 9, 26, 1
	mr	28, 4
	vspltw128 10, 4, 1
	vspltw128 11, 5, 1
	vspltw128 12, 27, 2
	vspltw128 13, 26, 2
	vspltw128 14, 4, 2
	vspltw128 15, 5, 2
	vspltw128 17, 27, 3
	vspltw128 19, 26, 3
	vspltw128 30, 4, 3
	vspltw128 28, 5, 3
.Lcv_BB4_3:                                # %vector.body
                                        # =>This Inner Loop Header: Depth=1
	add 27, 5, 11
	lvsl 3, 5, 11
	mr	26, 3
	addi 28, 28, -4
	lvx128 25, 27, 8
	cmplwi	28, 0
	lvx128 24, 27, 7
	lvx128 23, 5, 11
	vperm128 21, 24, 25, 3
	lvx128 22, 27, 10
	vperm128 24, 23, 24, 3
	lvx128 20, 27, 9
	vperm128 33, 24, 21, 6
	vperm128 23, 20, 22, 3
	vperm128 3, 25, 20, 3
	vperm128 25, 3, 23, 0
	vmrghw128 22, 24, 21
	vperm128 25, 22, 25, 1
	vmulfp128 22, 16, 25
	vmrghw128 34, 3, 23
	vperm128 33, 33, 34, 1
	vmulfp128 20, 8, 25
	vmrglw128 34, 24, 21
	vperm128 24, 24, 21, 2
	vmulfp128 32, 12, 25
	vmulfp128 25, 17, 25
	vmaddfp128 22, 18, 33, 22
	vmaddfp128 20, 9, 33, 20
	vmaddfp128 32, 13, 33, 32
	vmaddfp128 25, 19, 33, 25
	vperm128 33, 3, 23, 7
	vperm128 33, 34, 33, 1
	vmrglw128 3, 3, 23
	vmaddfp128 22, 31, 33, 22
	vperm128 3, 24, 3, 1
	vmaddfp128 20, 10, 33, 20
	vmaddfp128 32, 14, 33, 32
	vmaddfp128 25, 30, 33, 25
	vmaddfp128 22, 29, 3, 22
	vmaddfp128 20, 11, 3, 20
	vperm128 24, 22, 20, 6
	vmrglw128 21, 22, 20
	vmaddfp128 32, 15, 3, 32
	vmaddfp128 25, 28, 3, 25
	vmrghw128 3, 32, 25
	vperm128 23, 32, 25, 7
	vmrglw128 33, 32, 25
	vperm128 25, 32, 25, 0
	vperm128 32, 22, 20, 2
	vmrghw128 22, 22, 20
	vperm128 25, 22, 25, 1
	stvx128 25, 0, 12
	vperm128 25, 32, 33, 1
	stvx128 25, 0, 0
	vperm128 25, 21, 23, 1
	stvx128 25, 0, 30
	vperm128 3, 24, 3, 1
	stvx128 3, 0, 29
	ld 27, 96(1)
	stdux 27, 26, 11
	addi 11, 11, 64
	ld 27, 152(1)
	std 27, 56(26)
	ld 27, 144(1)
	std 27, 48(26)
	ld 27, 136(1)
	std 27, 40(26)
	ld 27, 128(1)
	std 27, 32(26)
	ld 27, 120(1)
	std 27, 24(26)
	ld 27, 112(1)
	std 27, 16(26)
	ld 27, 104(1)
	std 27, 8(26)
	bne	0, .Lcv_BB4_3
# %bb.4:                                # %middle.block
	cmplw	6, 4
	beq	0, .Lcv_BB4_7
.Lcv_BB4_5:                                # %for.body.preheader
	slwi 9, 4, 4
	sub	6, 6, 4
	add 5, 9, 5
	addi 4, 1, 32
	addi 7, 1, 48
	addi 8, 1, 64
	add 3, 3, 9
	addi 5, 5, -16
	addi 9, 1, 80
	addi 10, 1, 16
.Lcv_BB4_6:                                # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lwzu 11, 16(5)
	addi 6, 6, -1
	cmplwi	6, 0
	lwz 12, 4(5)
	lwz 0, 8(5)
	lwz 30, 12(5)
	stw 11, 32(1)
	lvx128 2, 0, 4
	stw 12, 48(1)
	vspltw128 2, 2, 0
	vmulfp128 2, 27, 2
	lvx128 3, 0, 7
	vspltw128 3, 3, 0
	stw 0, 64(1)
	vmaddfp128 2, 26, 3, 2
	lvx128 3, 0, 8
	vspltw128 3, 3, 0
	stw 30, 80(1)
	vmaddfp128 2, 4, 3, 2
	lvx128 3, 0, 9
	vspltw128 3, 3, 0
	vmaddfp128 2, 5, 3, 2
	stvx128 2, 0, 10
	ld 11, 24(1)
	std 11, 8(3)
	ld 11, 16(1)
	std 11, 0(3)
	addi 3, 3, 16
	bne	0, .Lcv_BB4_6
.Lcv_BB4_7:
	ld 30, 200(1)                           # 8-byte Folded Reload
	ld 29, 192(1)                           # 8-byte Folded Reload
	ld 28, 184(1)                           # 8-byte Folded Reload
	ld 27, 176(1)                           # 8-byte Folded Reload
	ld 26, 168(1)                           # 8-byte Folded Reload
	addi 1, 1, 224
	blr
.Lcv_func_end4:
	.size	cv_sp_mat4_s, .Lcv_func_end4-.Lcv_func_begin4
                                        # -- End function
	.section	.text.cv_sp_mat4_v,"ax",@progbits
	.globl	cv_sp_mat4_v                    # -- Begin function cv_sp_mat4_v
	.p2align	2
	.type	cv_sp_mat4_v,@function
cv_sp_mat4_v:                           # @cv_sp_mat4_v
.Lcv_func_begin5:
# %bb.0:                                # %entry
	cmpwi	6, 1
	bltlr	0
# %bb.1:                                # %for.body.preheader
	li 7, 16
	lvx128 2, 0, 4
	lvx128 3, 4, 7
	li 7, 32
	lvx128 4, 4, 7
	li 7, 48
	lvx128 5, 4, 7
	li 4, 0
.Lcv_BB5_2:                                # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lvx128 0, 5, 4
	addi 6, 6, -1
	cmplwi	6, 0
	vspltw128 1, 0, 1
	vmulfp128 1, 1, 3
	vspltw128 6, 0, 0
	vmaddfp128 1, 6, 2, 1
	vspltw128 6, 0, 2
	vspltw128 0, 0, 3
	vmaddfp128 1, 6, 4, 1
	vmaddfp128 1, 0, 5, 1
	stvx128 1, 3, 4
	addi 4, 4, 16
	bne	0, .Lcv_BB5_2
# %bb.3:                                # %for.cond.cleanup
	blr
.Lcv_func_end5:
	.size	cv_sp_mat4_v, .Lcv_func_end5-.Lcv_func_begin5
                                        # -- End function
	.section	.text.cv_sp_mat4_v8,"ax",@progbits
	.globl	cv_sp_mat4_v8                   # -- Begin function cv_sp_mat4_v8
	.p2align	2
	.type	cv_sp_mat4_v8,@function
cv_sp_mat4_v8:                          # @cv_sp_mat4_v8
.Lcv_func_begin6:
# %bb.0:                                # %entry
	cmpwi	6, 1
	bltlr	0
# %bb.1:                                # %for.body.preheader
	stwu 1, -48(1)
	li 7, 16
	li 8, 32
	lvx128 2, 0, 4
	li 9, 48
	li 10, 0
	lvx128 3, 4, 7
	li 11, 64
	li 12, 80
	lvx128 4, 4, 8
	li 0, 96
	lvx128 5, 4, 9
	li 4, 112
	std 30, 24(1)                           # 8-byte Folded Spill
	li 30, 0
	std 29, 16(1)                           # 8-byte Folded Spill
.Lcv_BB6_2:                                # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lvx128 0, 5, 10
	add 29, 5, 10
	addi 30, 30, 8
	cmpw	30, 6
	vspltw128 1, 0, 0
	lvx128 8, 29, 8
	vmulfp128 1, 1, 2
	vspltw128 6, 0, 1
	lvx128 10, 29, 9
	lvx128 12, 29, 11
	vspltw128 31, 0, 2
	vmaddfp128 1, 6, 3, 1
	lvx128 6, 29, 7
	vspltw128 9, 8, 0
	vspltw128 7, 6, 0
	lvx128 14, 29, 12
	vspltw128 0, 0, 3
	lvx128 16, 29, 0
	vmulfp128 7, 7, 2
	vspltw128 11, 10, 0
	lvx128 18, 29, 4
	add 29, 3, 10
	vspltw128 13, 12, 0
	vspltw128 15, 14, 0
	vmaddfp128 1, 31, 4, 1
	vspltw128 31, 6, 1
	vspltw128 17, 16, 0
	vmulfp128 9, 9, 2
	vspltw128 19, 18, 0
	vmaddfp128 7, 31, 3, 7
	vspltw128 31, 8, 1
	vmaddfp128 1, 0, 5, 1
	vspltw128 0, 6, 2
	stvx128 1, 3, 10
	addi 10, 10, 128
	vmulfp128 11, 11, 2
	vmaddfp128 9, 31, 3, 9
	vspltw128 31, 10, 1
	vmaddfp128 7, 0, 4, 7
	vspltw128 0, 8, 2
	vmulfp128 13, 13, 2
	vmaddfp128 11, 31, 3, 11
	vspltw128 31, 12, 1
	vmaddfp128 9, 0, 4, 9
	vspltw128 0, 10, 2
	vmulfp128 15, 15, 2
	vmaddfp128 13, 31, 3, 13
	vspltw128 31, 14, 1
	vmaddfp128 11, 0, 4, 11
	vspltw128 0, 12, 2
	vmulfp128 17, 17, 2
	vmaddfp128 15, 31, 3, 15
	vspltw128 31, 16, 1
	vmaddfp128 13, 0, 4, 13
	vspltw128 0, 14, 2
	vmulfp128 19, 19, 2
	vmaddfp128 17, 31, 3, 17
	vspltw128 31, 18, 1
	vmaddfp128 15, 0, 4, 15
	vspltw128 0, 16, 2
	vmaddfp128 19, 31, 3, 19
	vmaddfp128 17, 0, 4, 17
	vspltw128 0, 18, 2
	vmaddfp128 19, 0, 4, 19
	vspltw128 0, 6, 3
	vmaddfp128 7, 0, 5, 7
	vspltw128 0, 8, 3
	stvx128 7, 29, 7
	vmaddfp128 9, 0, 5, 9
	vspltw128 0, 10, 3
	stvx128 9, 29, 8
	vmaddfp128 11, 0, 5, 11
	vspltw128 0, 12, 3
	stvx128 11, 29, 9
	vmaddfp128 13, 0, 5, 13
	vspltw128 0, 14, 3
	stvx128 13, 29, 11
	vmaddfp128 15, 0, 5, 15
	vspltw128 0, 16, 3
	stvx128 15, 29, 12
	vmaddfp128 17, 0, 5, 17
	vspltw128 0, 18, 3
	stvx128 17, 29, 0
	vmaddfp128 19, 0, 5, 19
	stvx128 19, 29, 4
	blt	0, .Lcv_BB6_2
# %bb.3:
	ld 30, 24(1)                            # 8-byte Folded Reload
	ld 29, 16(1)                            # 8-byte Folded Reload
	addi 1, 1, 48
	blr
.Lcv_func_end6:
	.size	cv_sp_mat4_v8, .Lcv_func_end6-.Lcv_func_begin6
                                        # -- End function
	.section	.text.cv_sp_mat4_v16,"ax",@progbits
	.globl	cv_sp_mat4_v16                  # -- Begin function cv_sp_mat4_v16
	.p2align	2
	.type	cv_sp_mat4_v16,@function
cv_sp_mat4_v16:                         # @cv_sp_mat4_v16
.Lcv_func_begin7:
# %bb.0:                                # %entry
	cmpwi	6, 1
	bltlr	0
# %bb.1:                                # %for.body.preheader
	stwu 1, -112(1)
	li 7, 16
	li 8, 32
	lvx128 2, 0, 4
	li 9, 48
	li 10, 0
	lvx128 3, 4, 7
	li 11, 64
	li 12, 80
	lvx128 4, 4, 8
	li 0, 96
	lvx128 5, 4, 9
	li 4, 240
	std 22, 24(1)                           # 8-byte Folded Spill
	li 22, 0
	std 23, 32(1)                           # 8-byte Folded Spill
	li 23, 224
	std 24, 40(1)                           # 8-byte Folded Spill
	li 24, 208
	std 25, 48(1)                           # 8-byte Folded Spill
	li 25, 192
	std 26, 56(1)                           # 8-byte Folded Spill
	li 26, 176
	std 27, 64(1)                           # 8-byte Folded Spill
	li 27, 160
	std 28, 72(1)                           # 8-byte Folded Spill
	li 28, 144
	std 29, 80(1)                           # 8-byte Folded Spill
	li 29, 128
	std 30, 88(1)                           # 8-byte Folded Spill
	li 30, 112
	std 21, 16(1)                           # 8-byte Folded Spill
.Lcv_BB7_2:                                # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lvx128 0, 5, 10
	add 21, 5, 10
	addi 22, 22, 16
	cmpw	22, 6
	vspltw128 1, 0, 0
	lvx128 8, 21, 8
	vmulfp128 1, 1, 2
	vspltw128 6, 0, 1
	lvx128 10, 21, 9
	lvx128 12, 21, 11
	vspltw128 36, 0, 2
	vmaddfp128 1, 6, 3, 1
	lvx128 6, 21, 7
	vspltw128 9, 8, 0
	vspltw128 7, 6, 0
	lvx128 14, 21, 12
	vspltw128 0, 0, 3
	lvx128 16, 21, 0
	vmulfp128 7, 7, 2
	vspltw128 11, 10, 0
	lvx128 18, 21, 30
	vspltw128 13, 12, 0
	lvx128 31, 21, 29
	vspltw128 15, 14, 0
	lvx128 29, 21, 28
	vmaddfp128 1, 36, 4, 1
	vspltw128 36, 6, 1
	lvx128 27, 21, 27
	vspltw128 17, 16, 0
	lvx128 25, 21, 26
	vmulfp128 9, 9, 2
	vspltw128 19, 18, 0
	lvx128 23, 21, 25
	vspltw128 30, 31, 0
	lvx128 21, 21, 24
	vspltw128 28, 29, 0
	lvx128 32, 21, 23
	vmaddfp128 7, 36, 3, 7
	vspltw128 36, 8, 1
	lvx128 34, 21, 4
	add 21, 3, 10
	vspltw128 26, 27, 0
	vspltw128 24, 25, 0
	vmaddfp128 1, 0, 5, 1
	stvx128 1, 3, 10
	addi 10, 10, 256
	vspltw128 0, 6, 2
	vspltw128 22, 23, 0
	vmulfp128 11, 11, 2
	vspltw128 20, 21, 0
	vspltw128 33, 32, 0
	vspltw128 35, 34, 0
	vmaddfp128 9, 36, 3, 9
	vspltw128 36, 10, 1
	vmaddfp128 7, 0, 4, 7
	vspltw128 0, 8, 2
	vmulfp128 13, 13, 2
	vmaddfp128 11, 36, 3, 11
	vspltw128 36, 12, 1
	vmaddfp128 9, 0, 4, 9
	vspltw128 0, 10, 2
	vmulfp128 15, 15, 2
	vmaddfp128 13, 36, 3, 13
	vspltw128 36, 14, 1
	vmaddfp128 11, 0, 4, 11
	vspltw128 0, 12, 2
	vmulfp128 17, 17, 2
	vmaddfp128 15, 36, 3, 15
	vspltw128 36, 16, 1
	vmaddfp128 13, 0, 4, 13
	vspltw128 0, 14, 2
	vmulfp128 19, 19, 2
	vmaddfp128 17, 36, 3, 17
	vspltw128 36, 18, 1
	vmaddfp128 15, 0, 4, 15
	vspltw128 0, 16, 2
	vmulfp128 30, 30, 2
	vmaddfp128 19, 36, 3, 19
	vspltw128 36, 31, 1
	vmaddfp128 17, 0, 4, 17
	vspltw128 0, 18, 2
	vmulfp128 28, 28, 2
	vmaddfp128 30, 36, 3, 30
	vspltw128 36, 29, 1
	vmaddfp128 19, 0, 4, 19
	vspltw128 0, 31, 2
	vmulfp128 26, 26, 2
	vmaddfp128 28, 36, 3, 28
	vspltw128 36, 27, 1
	vmaddfp128 30, 0, 4, 30
	vspltw128 0, 29, 2
	vmulfp128 24, 24, 2
	vmaddfp128 26, 36, 3, 26
	vspltw128 36, 25, 1
	vmaddfp128 28, 0, 4, 28
	vspltw128 0, 27, 2
	vmulfp128 22, 22, 2
	vmaddfp128 24, 36, 3, 24
	vspltw128 36, 23, 1
	vmaddfp128 26, 0, 4, 26
	vspltw128 0, 25, 2
	vmulfp128 20, 20, 2
	vmaddfp128 22, 36, 3, 22
	vspltw128 36, 21, 1
	vmaddfp128 24, 0, 4, 24
	vspltw128 0, 23, 2
	vmulfp128 33, 33, 2
	vmaddfp128 20, 36, 3, 20
	vspltw128 36, 32, 1
	vmaddfp128 22, 0, 4, 22
	vspltw128 0, 21, 2
	vmulfp128 35, 35, 2
	vmaddfp128 33, 36, 3, 33
	vspltw128 36, 34, 1
	vmaddfp128 20, 0, 4, 20
	vspltw128 0, 32, 2
	vmaddfp128 35, 36, 3, 35
	vmaddfp128 33, 0, 4, 33
	vspltw128 0, 34, 2
	vmaddfp128 35, 0, 4, 35
	vspltw128 0, 6, 3
	vmaddfp128 7, 0, 5, 7
	vspltw128 0, 8, 3
	stvx128 7, 21, 7
	vmaddfp128 9, 0, 5, 9
	vspltw128 0, 10, 3
	stvx128 9, 21, 8
	vmaddfp128 11, 0, 5, 11
	vspltw128 0, 12, 3
	stvx128 11, 21, 9
	vmaddfp128 13, 0, 5, 13
	vspltw128 0, 14, 3
	stvx128 13, 21, 11
	vmaddfp128 15, 0, 5, 15
	vspltw128 0, 16, 3
	stvx128 15, 21, 12
	vmaddfp128 17, 0, 5, 17
	vspltw128 0, 18, 3
	stvx128 17, 21, 0
	vmaddfp128 19, 0, 5, 19
	vspltw128 0, 31, 3
	stvx128 19, 21, 30
	vmaddfp128 30, 0, 5, 30
	vspltw128 0, 29, 3
	stvx128 30, 21, 29
	vmaddfp128 28, 0, 5, 28
	vspltw128 0, 27, 3
	stvx128 28, 21, 28
	vmaddfp128 26, 0, 5, 26
	vspltw128 0, 25, 3
	stvx128 26, 21, 27
	vmaddfp128 24, 0, 5, 24
	vspltw128 0, 23, 3
	stvx128 24, 21, 26
	vmaddfp128 22, 0, 5, 22
	vspltw128 0, 21, 3
	stvx128 22, 21, 25
	vmaddfp128 20, 0, 5, 20
	vspltw128 0, 32, 3
	stvx128 20, 21, 24
	vmaddfp128 33, 0, 5, 33
	vspltw128 0, 34, 3
	stvx128 33, 21, 23
	vmaddfp128 35, 0, 5, 35
	stvx128 35, 21, 4
	blt	0, .Lcv_BB7_2
# %bb.3:
	ld 30, 88(1)                            # 8-byte Folded Reload
	ld 29, 80(1)                            # 8-byte Folded Reload
	ld 28, 72(1)                            # 8-byte Folded Reload
	ld 27, 64(1)                            # 8-byte Folded Reload
	ld 26, 56(1)                            # 8-byte Folded Reload
	ld 25, 48(1)                            # 8-byte Folded Reload
	ld 24, 40(1)                            # 8-byte Folded Reload
	ld 23, 32(1)                            # 8-byte Folded Reload
	ld 22, 24(1)                            # 8-byte Folded Reload
	ld 21, 16(1)                            # 8-byte Folded Reload
	addi 1, 1, 112
	blr
.Lcv_func_end7:
	.size	cv_sp_mat4_v16, .Lcv_func_end7-.Lcv_func_begin7
                                        # -- End function
	.section	.text.cv_sp_mat4_x,"ax",@progbits
	.globl	cv_sp_mat4_x                    # -- Begin function cv_sp_mat4_x
	.p2align	2
	.type	cv_sp_mat4_x,@function
cv_sp_mat4_x:                           # @cv_sp_mat4_x
.Lcv_func_begin8:
# %bb.0:                                # %entry
	cmpwi	6, 1
	bltlr	0
# %bb.1:                                # %for.body.preheader
	li 7, 32
	lvx128 2, 0, 4
	lvx128 3, 4, 7
	li 7, 16
	lvx128 4, 4, 7
	li 7, 48
	vmrghw128 0, 2, 3
	lvx128 5, 4, 7
	vmrglw128 6, 2, 3
	li 4, 0
	vmrghw128 1, 4, 5
	vmrglw128 5, 4, 5
	vmrghw128 2, 0, 1
	vmrglw128 3, 0, 1
	vmrghw128 4, 6, 5
	vmrglw128 5, 6, 5
.Lcv_BB8_2:                                # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lvx128 0, 5, 4
	addi 6, 6, -1
	cmplwi	6, 0
	vmsum4fp128 1, 2, 0
	vmsum4fp128 6, 3, 0
	vrlimi128 1, 6, 4, 0
	vmsum4fp128 7, 4, 0
	vrlimi128 1, 7, 2, 0
	vmsum4fp128 0, 5, 0
	vrlimi128 1, 0, 1, 0
	stvx128 1, 3, 4
	addi 4, 4, 16
	bne	0, .Lcv_BB8_2
# %bb.3:                                # %for.cond.cleanup
	blr
.Lcv_func_end8:
	.size	cv_sp_mat4_x, .Lcv_func_end8-.Lcv_func_begin8
                                        # -- End function
	.section	.text.cv_sp_copy_s,"ax",@progbits
	.globl	cv_sp_copy_s                    # -- Begin function cv_sp_copy_s
	.p2align	2
	.type	cv_sp_copy_s,@function
cv_sp_copy_s:                           # @cv_sp_copy_s
.Lcv_func_begin9:
# %bb.0:                                # %entry
	cmpwi	5, 1
	bltlr	0
# %bb.1:                                # %for.body.preheader
	li 6, 0
	beq	0, .Lcv_BB9_5
# %bb.2:                                # %vector.ph
	rlwinm 6, 5, 0, 1, 30
	addi 7, 4, -8
	addi 8, 3, -8
	mr	9, 6
.Lcv_BB9_3:                                # %vector.body
                                        # =>This Inner Loop Header: Depth=1
	lwzu 10, 8(7)
	addi 9, 9, -2
	cmplwi	9, 0
	stwu 10, 8(8)
	lwz 10, 4(7)
	stw 10, 4(8)
	bne	0, .Lcv_BB9_3
# %bb.4:                                # %middle.block
	cmplw	5, 6
	beqlr	0
.Lcv_BB9_5:                                # %for.body.preheader7
	slwi 7, 6, 2
	sub	5, 5, 6
	addi 6, 7, -4
	add 4, 4, 6
	add 3, 3, 6
.Lcv_BB9_6:                                # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lwzu 6, 4(4)
	addi 5, 5, -1
	cmplwi	5, 0
	stwu 6, 4(3)
	bne	0, .Lcv_BB9_6
	blr
.Lcv_func_end9:
	.size	cv_sp_copy_s, .Lcv_func_end9-.Lcv_func_begin9
                                        # -- End function
	.section	.text.cv_sp_copy_v,"ax",@progbits
	.globl	cv_sp_copy_v                    # -- Begin function cv_sp_copy_v
	.p2align	2
	.type	cv_sp_copy_v,@function
cv_sp_copy_v:                           # @cv_sp_copy_v
.Lcv_func_begin10:
# %bb.0:                                # %entry
	srawi 6, 5, 2
	cmpwi	5, 4
	addze 5, 6
	bltlr	0
# %bb.1:                                # %for.body.preheader
	li 6, 0
.Lcv_BB10_2:                               # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lvx128 2, 4, 6
	addi 5, 5, -1
	cmplwi	5, 0
	stvx128 2, 3, 6
	addi 6, 6, 16
	bne	0, .Lcv_BB10_2
# %bb.3:                                # %for.cond.cleanup
	blr
.Lcv_func_end10:
	.size	cv_sp_copy_v, .Lcv_func_end10-.Lcv_func_begin10
                                        # -- End function
	.section	.text.cv_sp_bright_s,"ax",@progbits
	.globl	cv_sp_bright_s                  # -- Begin function cv_sp_bright_s
	.p2align	2
	.type	cv_sp_bright_s,@function
cv_sp_bright_s:                         # @cv_sp_bright_s
.Lcv_func_begin11:
# %bb.0:                                # %entry
	cmpwi	5, 1
	bltlr	0
# %bb.1:                                # %for.body.preheader
	stwu 1, -32(1)
	cmplwi	5, 16
	li 6, 0
	blt	0, .Lcv_BB11_6
# %bb.2:                                # %vector.ph
	vspltisb 2, 10
	rlwinm 6, 5, 0, 1, 27
	li 7, 0
	vslb 2, 2, 2
	li 8, 15
	addi 9, 1, 16
.Lcv_BB11_3:                               # %vector.body
                                        # =>This Inner Loop Header: Depth=1
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
	addi 7, 7, 16
	cmplw	6, 7
	ld 10, 24(1)
	std 10, 8(11)
	bne	0, .Lcv_BB11_3
# %bb.4:                                # %middle.block
	cmplw	5, 6
	bne	0, .Lcv_BB11_6
.Lcv_BB11_5:
	addi 1, 1, 32
	blr
.Lcv_BB11_6:                               # %for.body.preheader11
	addi 7, 6, -1
	add 4, 4, 7
	add 3, 3, 7
	b .Lcv_BB11_8
.Lcv_BB11_7:                               # %for.body
                                        #   in Loop: Header=BB11_8 Depth=1
	addi 6, 6, 1
	stbu 7, 1(3)
	cmplw	5, 6
	beq	0, .Lcv_BB11_5
.Lcv_BB11_8:                               # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lbzu 7, 1(4)
	addi 7, 7, 40
	cmplwi	7, 255
	blt	0, .Lcv_BB11_7
# %bb.9:                                # %for.body
                                        #   in Loop: Header=BB11_8 Depth=1
	li 7, 255
	b .Lcv_BB11_7
.Lcv_func_end11:
	.size	cv_sp_bright_s, .Lcv_func_end11-.Lcv_func_begin11
                                        # -- End function
	.section	.text.cv_sp_bright_v,"ax",@progbits
	.globl	cv_sp_bright_v                  # -- Begin function cv_sp_bright_v
	.p2align	2
	.type	cv_sp_bright_v,@function
cv_sp_bright_v:                         # @cv_sp_bright_v
.Lcv_func_begin12:
# %bb.0:                                # %entry
	srawi 6, 5, 4
	cmpwi	5, 16
	addze 5, 6
	bltlr	0
# %bb.1:                                # %for.body.preheader
	vspltisb 2, 10
	li 6, 0
	vslb 2, 2, 2
.Lcv_BB12_2:                               # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lvx128 3, 4, 6
	addi 5, 5, -1
	cmplwi	5, 0
	vaddubs 3, 3, 2
	stvx128 3, 3, 6
	addi 6, 6, 16
	bne	0, .Lcv_BB12_2
# %bb.3:                                # %for.cond.cleanup
	blr
.Lcv_func_end12:
	.size	cv_sp_bright_v, .Lcv_func_end12-.Lcv_func_begin12
                                        # -- End function
	.section	.rodata.cst4,"aM",@progbits,4
	.p2align	2, 0x0                          # -- Begin function cv_sp_u8f_s
.Lcv_CPI13_0:
	.long	0x3b808081                      # float 0.00392156886
	.section	.text.cv_sp_u8f_s,"ax",@progbits
	.globl	cv_sp_u8f_s
	.p2align	2
	.type	cv_sp_u8f_s,@function
cv_sp_u8f_s:                            # @cv_sp_u8f_s
.Lcv_func_begin13:
# %bb.0:                                # %entry
	cmpwi	5, 1
	bltlr	0
# %bb.1:                                # %for.body.preheader
	stwu 1, -48(1)
	li 6, 0
	beq	0, .Lcv_BB13_5
# %bb.2:                                # %vector.ph
	lis 9, .Lcv_CPI13_0@ha
	rlwinm 6, 5, 0, 1, 30
	lfs 0, .Lcv_CPI13_0@l(9)
	addi 7, 4, -2
	addi 8, 3, -8
	mr	9, 6
.Lcv_BB13_3:                               # %vector.body
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
	bne	0, .Lcv_BB13_3
# %bb.4:                                # %middle.block
	cmplw	5, 6
	beq	0, .Lcv_BB13_7
.Lcv_BB13_5:                               # %for.body.preheader7
	sub	5, 5, 6
	slwi 7, 6, 2
	add 4, 6, 4
	lis 6, .Lcv_CPI13_0@ha
	lfs 0, .Lcv_CPI13_0@l(6)
	add 7, 7, 3
	addi 3, 4, -1
	addi 4, 7, -4
.Lcv_BB13_6:                               # %for.body
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
	bne	0, .Lcv_BB13_6
.Lcv_BB13_7:
	addi 1, 1, 48
	blr
.Lcv_func_end13:
	.size	cv_sp_u8f_s, .Lcv_func_end13-.Lcv_func_begin13
                                        # -- End function
	.section	.rodata.cst16,"aM",@progbits,16
	.p2align	4, 0x0                          # -- Begin function cv_sp_u8f_v
.Lcv_CPI14_0:
	.byte	0                               # 0x0
	.byte	1                               # 0x1
	.byte	2                               # 0x2
	.byte	16                              # 0x10
	.byte	0                               # 0x0
	.byte	1                               # 0x1
	.byte	2                               # 0x2
	.byte	17                              # 0x11
	.byte	0                               # 0x0
	.byte	1                               # 0x1
	.byte	2                               # 0x2
	.byte	18                              # 0x12
	.byte	0                               # 0x0
	.byte	1                               # 0x1
	.byte	2                               # 0x2
	.byte	19                              # 0x13
.Lcv_CPI14_1:
	.byte	0                               # 0x0
	.byte	1                               # 0x1
	.byte	2                               # 0x2
	.byte	20                              # 0x14
	.byte	0                               # 0x0
	.byte	1                               # 0x1
	.byte	2                               # 0x2
	.byte	21                              # 0x15
	.byte	0                               # 0x0
	.byte	1                               # 0x1
	.byte	2                               # 0x2
	.byte	22                              # 0x16
	.byte	0                               # 0x0
	.byte	1                               # 0x1
	.byte	2                               # 0x2
	.byte	23                              # 0x17
.Lcv_CPI14_2:
	.byte	0                               # 0x0
	.byte	1                               # 0x1
	.byte	2                               # 0x2
	.byte	24                              # 0x18
	.byte	0                               # 0x0
	.byte	1                               # 0x1
	.byte	2                               # 0x2
	.byte	25                              # 0x19
	.byte	0                               # 0x0
	.byte	1                               # 0x1
	.byte	2                               # 0x2
	.byte	26                              # 0x1a
	.byte	0                               # 0x0
	.byte	1                               # 0x1
	.byte	2                               # 0x2
	.byte	27                              # 0x1b
.Lcv_CPI14_3:
	.byte	0                               # 0x0
	.byte	1                               # 0x1
	.byte	2                               # 0x2
	.byte	28                              # 0x1c
	.byte	0                               # 0x0
	.byte	1                               # 0x1
	.byte	2                               # 0x2
	.byte	29                              # 0x1d
	.byte	0                               # 0x0
	.byte	1                               # 0x1
	.byte	2                               # 0x2
	.byte	30                              # 0x1e
	.byte	0                               # 0x0
	.byte	1                               # 0x1
	.byte	2                               # 0x2
	.byte	31                              # 0x1f
.Lcv_CPI14_4:
	.long	0x3b808081                      # float 0.00392156886
	.long	0x3b808081                      # float 0.00392156886
	.long	0x3b808081                      # float 0.00392156886
	.long	0x3b808081                      # float 0.00392156886
	.section	.text.cv_sp_u8f_v,"ax",@progbits
	.globl	cv_sp_u8f_v
	.p2align	2
	.type	cv_sp_u8f_v,@function
cv_sp_u8f_v:                            # @cv_sp_u8f_v
.Lcv_func_begin14:
# %bb.0:                                # %entry
	srawi 6, 5, 4
	cmpwi	5, 16
	addze 5, 6
	bltlr	0
# %bb.1:
	li 6, .Lcv_CPI14_0@l
	lis 7, .Lcv_CPI14_0@ha
	vspltisw128 0, 0
	lvx128 2, 7, 6
	li 6, .Lcv_CPI14_1@l
	lis 7, .Lcv_CPI14_1@ha
	lvx128 3, 7, 6
	li 6, .Lcv_CPI14_2@l
	lis 7, .Lcv_CPI14_2@ha
	li 8, .Lcv_CPI14_3@l
	lvx128 4, 7, 6
	lis 6, .Lcv_CPI14_3@ha
	li 7, .Lcv_CPI14_4@l
	lvx128 5, 6, 8
	lis 8, .Lcv_CPI14_4@ha
	li 6, 16
	lvx128 1, 8, 7
	li 7, 32
	li 8, 48
.Lcv_BB14_2:                               # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lvx128 6, 0, 4
	addi 5, 5, -1
	addi 4, 4, 16
	cmplwi	5, 0
	vperm128 7, 0, 6, 2
	vcuxwfp128 7, 7, 0
	vperm128 8, 0, 6, 3
	vperm128 9, 0, 6, 4
	vperm128 6, 0, 6, 5
	vcuxwfp128 8, 8, 0
	vcuxwfp128 9, 9, 0
	vcuxwfp128 6, 6, 0
	vmulfp128 7, 7, 1
	stvx128 7, 0, 3
	vmulfp128 8, 8, 1
	stvx128 8, 3, 6
	vmulfp128 9, 9, 1
	stvx128 9, 3, 7
	vmulfp128 6, 6, 1
	stvx128 6, 3, 8
	addi 3, 3, 64
	bne	0, .Lcv_BB14_2
# %bb.3:                                # %for.cond.cleanup
	blr
.Lcv_func_end14:
	.size	cv_sp_u8f_v, .Lcv_func_end14-.Lcv_func_begin14
                                        # -- End function
	.section	.rodata.cst4,"aM",@progbits,4
	.p2align	2, 0x0                          # -- Begin function cv_sp_f2u8_s
.Lcv_CPI15_0:
	.long	0x3f000000                      # float 0.5
.Lcv_CPI15_1:
	.long	0x437f0000                      # float 255
.Lcv_CPI15_2:
	.long	0x00000000                      # float 0
	.section	.text.cv_sp_f2u8_s,"ax",@progbits
	.globl	cv_sp_f2u8_s
	.p2align	2
	.type	cv_sp_f2u8_s,@function
cv_sp_f2u8_s:                           # @cv_sp_f2u8_s
.Lcv_func_begin15:
# %bb.0:                                # %entry
	cmpwi	5, 1
	bltlr	0
# %bb.1:                                # %for.body.preheader
	stwu 1, -32(1)
	lis 6, .Lcv_CPI15_0@ha
	addi 4, 4, -4
	lfs 0, .Lcv_CPI15_0@l(6)
	lis 6, .Lcv_CPI15_1@ha
	addi 3, 3, -1
	lfs 1, .Lcv_CPI15_1@l(6)
	lis 6, .Lcv_CPI15_2@ha
	lfs 2, .Lcv_CPI15_2@l(6)
.Lcv_BB15_2:                               # %for.body
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
	bne	0, .Lcv_BB15_2
# %bb.3:
	addi 1, 1, 32
	blr
.Lcv_func_end15:
	.size	cv_sp_f2u8_s, .Lcv_func_end15-.Lcv_func_begin15
                                        # -- End function
	.section	.rodata.cst16,"aM",@progbits,16
	.p2align	4, 0x0                          # -- Begin function cv_sp_f2u8_v
.Lcv_CPI16_0:
	.long	0x437f0000                      # float 255
	.long	0x437f0000                      # float 255
	.long	0x437f0000                      # float 255
	.long	0x437f0000                      # float 255
.Lcv_CPI16_1:
	.long	0x3f000000                      # float 0.5
	.long	0x3f000000                      # float 0.5
	.long	0x3f000000                      # float 0.5
	.long	0x3f000000                      # float 0.5
.Lcv_CPI16_2:
	.byte	3                               # 0x3
	.byte	7                               # 0x7
	.byte	11                              # 0xb
	.byte	15                              # 0xf
	.byte	19                              # 0x13
	.byte	23                              # 0x17
	.byte	27                              # 0x1b
	.byte	31                              # 0x1f
	.byte	0                               # 0x0
	.byte	0                               # 0x0
	.byte	0                               # 0x0
	.byte	0                               # 0x0
	.byte	0                               # 0x0
	.byte	0                               # 0x0
	.byte	0                               # 0x0
	.byte	0                               # 0x0
.Lcv_CPI16_3:
	.byte	0                               # 0x0
	.byte	1                               # 0x1
	.byte	2                               # 0x2
	.byte	3                               # 0x3
	.byte	4                               # 0x4
	.byte	5                               # 0x5
	.byte	6                               # 0x6
	.byte	7                               # 0x7
	.byte	16                              # 0x10
	.byte	17                              # 0x11
	.byte	18                              # 0x12
	.byte	19                              # 0x13
	.byte	20                              # 0x14
	.byte	21                              # 0x15
	.byte	22                              # 0x16
	.byte	23                              # 0x17
	.section	.text.cv_sp_f2u8_v,"ax",@progbits
	.globl	cv_sp_f2u8_v
	.p2align	2
	.type	cv_sp_f2u8_v,@function
cv_sp_f2u8_v:                           # @cv_sp_f2u8_v
.Lcv_func_begin16:
# %bb.0:                                # %entry
	srawi 6, 5, 4
	cmpwi	5, 16
	addze 5, 6
	bltlr	0
# %bb.1:                                # %for.body.lr.ph
	stwu 1, -80(1)
	li 9, .Lcv_CPI16_1@l
	lis 10, .Lcv_CPI16_1@ha
	vspltisw128 5, 0
	lvx128 3, 10, 9
	li 7, .Lcv_CPI16_0@l
	lis 8, .Lcv_CPI16_0@ha
	li 9, .Lcv_CPI16_2@l
	lis 10, .Lcv_CPI16_2@ha
	lvx128 2, 8, 7
	li 11, .Lcv_CPI16_3@l
	lis 12, .Lcv_CPI16_3@ha
	lvx128 4, 10, 9
	addi 6, 1, 32
	addi 7, 1, 48
	lvx128 0, 12, 11
	li 8, 0
	addi 9, 1, 64
	addi 10, 1, 16
.Lcv_BB16_2:                               # %for.body
                                        # =>This Loop Header: Depth=1
                                        #     Child Loop BB16_3 Depth 2
	li 11, 0
.Lcv_BB16_3:                               # %for.body4
                                        #   Parent Loop BB16_2 Depth=1
                                        # =>  This Inner Loop Header: Depth=2
	lvx128 1, 4, 11
	vmr	6, 3
	vmaddfp128 6, 1, 2, 6
	vcmpgtfp128 1, 6, 5
	vand128 1, 1, 6
	vcmpgtfp128 6, 2, 1
	vsel128 6, 2, 1, 6
	vcfpuxws128 1, 6, 0
	stvx128 1, 10, 11
	addi 11, 11, 16
	cmplwi	11, 64
	bne	0, .Lcv_BB16_3
# %bb.4:                                # %for.cond.cleanup3
                                        #   in Loop: Header=BB16_2 Depth=1
	lvx128 1, 0, 10
	slwi 11, 8, 4
	addi 8, 8, 1
	cmplw	8, 5
	addi 4, 4, 64
	lvx128 6, 0, 6
	lvx128 7, 0, 7
	vperm128 1, 1, 6, 4
	lvx128 6, 0, 9
	vperm128 6, 7, 6, 4
	vperm128 1, 1, 6, 0
	stvx128 1, 3, 11
	bne	0, .Lcv_BB16_2
# %bb.5:
	addi 1, 1, 80
	blr
.Lcv_func_end16:
	.size	cv_sp_f2u8_v, .Lcv_func_end16-.Lcv_func_begin16
                                        # -- End function
