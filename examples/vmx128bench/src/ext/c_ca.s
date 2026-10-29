# speed.c: clang version 22.1.8 (git@github.com:landaire/llvm-project.git 2f714b11080e5a449de3969e3c4477cf5cbe92ab)
# flags: --target=powerpc64-unknown-xbox360 -ffreestanding -fno-builtin -fms-extensions -ffunction-sections -fdata-sections -fno-ident -fno-addrsig -fno-asynchronous-unwind-tables -g0 -O3 -ffast-math -mno-vmx128
	.section	.text.ca_sp_saxpy_s,"ax",@progbits
	.globl	ca_sp_saxpy_s                   # -- Begin function ca_sp_saxpy_s
	.p2align	2
	.type	ca_sp_saxpy_s,@function
ca_sp_saxpy_s:                          # @ca_sp_saxpy_s
.Lca_func_begin0:
# %bb.0:                                # %entry
	cmpwi	6, 1
	bltlr	0
# %bb.1:                                # %for.body.preheader
	stwu 1, -64(1)
	cmplwi	6, 4
	li 5, 0
	std 30, 40(1)                           # 8-byte Folded Spill
	blt	0, .Lca_BB0_5
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
.Lca_BB0_3:                                # %vector.body
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
	bne	0, .Lca_BB0_3
# %bb.4:                                # %middle.block
	cmplw	6, 5
	beq	0, .Lca_BB0_7
.Lca_BB0_5:                                # %for.body.preheader11
	slwi 7, 5, 2
	sub	5, 6, 5
	addi 6, 7, -4
	add 4, 4, 6
	add 3, 3, 6
.Lca_BB0_6:                                # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lfsu 0, 4(3)
	addi 5, 5, -1
	cmplwi	5, 0
	lfsu 2, 4(4)
	fmadds 0, 2, 1, 0
	stfs 0, 0(3)
	bne	0, .Lca_BB0_6
.Lca_BB0_7:
	ld 30, 40(1)                            # 8-byte Folded Reload
	addi 1, 1, 64
	blr
.Lca_func_end0:
	.size	ca_sp_saxpy_s, .Lca_func_end0-.Lca_func_begin0
                                        # -- End function
	.section	.text.ca_sp_saxpy_v,"ax",@progbits
	.globl	ca_sp_saxpy_v                   # -- Begin function ca_sp_saxpy_v
	.p2align	2
	.type	ca_sp_saxpy_v,@function
ca_sp_saxpy_v:                          # @ca_sp_saxpy_v
.Lca_func_begin1:
# %bb.0:                                # %entry
	stwu 1, -32(1)
	srawi 5, 6, 2
	cmpwi	6, 4
	stfs 1, 16(1)
	addze 5, 5
	blt	0, .Lca_BB1_3
# %bb.1:                                # %for.body.preheader
	addi 6, 1, 16
	lvx 2, 0, 6
	li 6, 0
	vspltw 2, 2, 0
.Lca_BB1_2:                                # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lvx 3, 4, 6
	addi 5, 5, -1
	cmplwi	5, 0
	lvx 4, 3, 6
	vmaddfp 3, 3, 2, 4
	stvx 3, 3, 6
	addi 6, 6, 16
	bne	0, .Lca_BB1_2
.Lca_BB1_3:                                # %for.cond.cleanup
	addi 1, 1, 32
	blr
.Lca_func_end1:
	.size	ca_sp_saxpy_v, .Lca_func_end1-.Lca_func_begin1
                                        # -- End function
	.section	.rodata.cst4,"aM",@progbits,4
	.p2align	2, 0x0                          # -- Begin function ca_sp_dot_s
.Lca_CPI2_0:
	.long	0x00000000                      # float 0
	.section	.text.ca_sp_dot_s,"ax",@progbits
	.globl	ca_sp_dot_s
	.p2align	2
	.type	ca_sp_dot_s,@function
ca_sp_dot_s:                            # @ca_sp_dot_s
.Lca_func_begin2:
# %bb.0:                                # %entry
	cmpwi	5, 1
	lis 7, .Lca_CPI2_0@ha
	blt	0, .Lca_BB2_3
# %bb.1:                                # %for.body.preheader
	stwu 1, -32(1)
	cmplwi	5, 8
	bge 0, .Lca_BB2_4
# %bb.2:
	li 6, 0
	lfs 1, .Lca_CPI2_0@l(7)
	b .Lca_BB2_7
.Lca_BB2_3:
	lfs 1, .Lca_CPI2_0@l(7)
	blr
.Lca_BB2_4:                                # %vector.ph
	rlwinm 6, 5, 0, 1, 28
	vxor 2, 2, 2
	addi 7, 4, 16
	addi 8, 3, 16
	li 9, 0
	li 10, 15
	mr	11, 6
	vxor 3, 3, 3
.Lca_BB2_5:                                # %vector.body
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
	bne	0, .Lca_BB2_5
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
	beq	0, .Lca_BB2_9
.Lca_BB2_7:                                # %for.body.preheader13
	slwi 7, 6, 2
	sub	5, 5, 6
	addi 6, 7, -4
	add 3, 3, 6
	add 4, 4, 6
.Lca_BB2_8:                                # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lfsu 0, 4(4)
	addi 5, 5, -1
	cmplwi	5, 0
	lfsu 2, 4(3)
	fmadds 1, 0, 2, 1
	bne	0, .Lca_BB2_8
.Lca_BB2_9:
	addi 1, 1, 32
	blr
.Lca_func_end2:
	.size	ca_sp_dot_s, .Lca_func_end2-.Lca_func_begin2
                                        # -- End function
	.section	.text.ca_sp_dot_v,"ax",@progbits
	.globl	ca_sp_dot_v                     # -- Begin function ca_sp_dot_v
	.p2align	2
	.type	ca_sp_dot_v,@function
ca_sp_dot_v:                            # @ca_sp_dot_v
.Lca_func_begin3:
# %bb.0:                                # %entry
	srawi 6, 5, 2
	vxor 2, 2, 2
	cmpwi	5, 4
	addze 5, 6
	blt	0, .Lca_BB3_4
# %bb.1:                                # %for.body.preheader
	vxor 3, 3, 3
	li 6, 0
	li 7, 16
	li 8, 32
	li 9, 48
	li 10, 0
	vxor 4, 4, 4
	vxor 5, 5, 5
.Lca_BB3_2:                                # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lvx 0, 3, 6
	add 11, 3, 6
	add 12, 4, 6
	addi 10, 10, 4
	lvx 1, 4, 6
	cmpw	10, 5
	addi 6, 6, 64
	vmaddfp 2, 1, 0, 2
	lvx 0, 11, 7
	lvx 1, 12, 7
	vmaddfp 5, 1, 0, 5
	lvx 0, 11, 8
	lvx 1, 12, 8
	vmaddfp 4, 1, 0, 4
	lvx 0, 11, 9
	lvx 1, 12, 9
	vmaddfp 3, 1, 0, 3
	blt	0, .Lca_BB3_2
# %bb.3:                                # %for.cond.cleanup.loopexit
	vaddfp 4, 4, 5
	vaddfp 2, 3, 2
	vaddfp 2, 4, 2
.Lca_BB3_4:                                # %for.cond.cleanup
	stwu 1, -32(1)
	vsldoi 3, 2, 2, 8
	addi 3, 1, 16
	vaddfp 2, 2, 3
	vspltw 3, 2, 1
	vaddfp 2, 2, 3
	stvx 2, 0, 3
	lfs 1, 16(1)
	addi 1, 1, 32
	blr
.Lca_func_end3:
	.size	ca_sp_dot_v, .Lca_func_end3-.Lca_func_begin3
                                        # -- End function
	.section	.rodata.cst16,"aM",@progbits,16
	.p2align	4, 0x0                          # -- Begin function ca_sp_mat4_s
.Lca_CPI4_0:
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
.Lca_CPI4_1:
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
.Lca_CPI4_2:
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
.Lca_CPI4_3:
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
.Lca_CPI4_4:
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
	.section	.text.ca_sp_mat4_s,"ax",@progbits
	.globl	ca_sp_mat4_s
	.p2align	2
	.type	ca_sp_mat4_s,@function
ca_sp_mat4_s:                           # @ca_sp_mat4_s
.Lca_func_begin4:
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
	blt	0, .Lca_BB4_5
# %bb.2:                                # %vector.ph
	vspltw 0, 2, 0
	li 4, 64
	li 12, .Lca_CPI4_0@l
	vspltisw 26, -1
	stvx 0, 1, 4                            # 16-byte Folded Spill
	li 4, 48
	lis 30, .Lca_CPI4_0@ha
	vspltw 0, 3, 0
	li 29, .Lca_CPI4_1@l
	lis 28, .Lca_CPI4_1@ha
	stvx 0, 1, 4                            # 16-byte Folded Spill
	li 26, .Lca_CPI4_2@l
	lis 25, .Lca_CPI4_2@ha
	vspltw 0, 4, 0
	lvx 16, 30, 12
	li 24, .Lca_CPI4_3@l
	lis 23, .Lca_CPI4_3@ha
	vslw 26, 26, 26
	vspltw 7, 5, 0
	li 22, .Lca_CPI4_4@l
	lis 21, .Lca_CPI4_4@ha
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
.Lca_BB4_3:                                # %vector.body
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
	bne	0, .Lca_BB4_3
# %bb.4:                                # %middle.block
	cmplw	6, 4
	beq	0, .Lca_BB4_7
.Lca_BB4_5:                                # %for.body.preheader
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
.Lca_BB4_6:                                # %for.body
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
	bne	0, .Lca_BB4_6
.Lca_BB4_7:
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
.Lca_func_end4:
	.size	ca_sp_mat4_s, .Lca_func_end4-.Lca_func_begin4
                                        # -- End function
	.section	.text.ca_sp_mat4_v,"ax",@progbits
	.globl	ca_sp_mat4_v                    # -- Begin function ca_sp_mat4_v
	.p2align	2
	.type	ca_sp_mat4_v,@function
ca_sp_mat4_v:                           # @ca_sp_mat4_v
.Lca_func_begin5:
# %bb.0:                                # %entry
	cmpwi	6, 1
	bltlr	0
# %bb.1:                                # %for.body.preheader
	li 7, 16
	lvx 2, 0, 4
	vspltisw 0, -1
	lvx 3, 4, 7
	li 7, 32
	lvx 4, 4, 7
	li 7, 48
	vslw 0, 0, 0
	lvx 5, 4, 7
	li 4, 0
.Lca_BB5_2:                                # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lvx 1, 5, 4
	addi 6, 6, -1
	cmplwi	6, 0
	vspltw 6, 1, 1
	vmaddfp 6, 6, 3, 0
	vspltw 7, 1, 0
	vmaddfp 6, 7, 2, 6
	vspltw 7, 1, 2
	vspltw 1, 1, 3
	vmaddfp 6, 7, 4, 6
	vmaddfp 1, 1, 5, 6
	stvx 1, 3, 4
	addi 4, 4, 16
	bne	0, .Lca_BB5_2
# %bb.3:                                # %for.cond.cleanup
	blr
.Lca_func_end5:
	.size	ca_sp_mat4_v, .Lca_func_end5-.Lca_func_begin5
                                        # -- End function
	.section	.text.ca_sp_mat4_v8,"ax",@progbits
	.globl	ca_sp_mat4_v8                   # -- Begin function ca_sp_mat4_v8
	.p2align	2
	.type	ca_sp_mat4_v8,@function
ca_sp_mat4_v8:                          # @ca_sp_mat4_v8
.Lca_func_begin6:
# %bb.0:                                # %entry
	cmpwi	6, 1
	bltlr	0
# %bb.1:                                # %for.body.preheader
	stwu 1, -48(1)
	li 7, 16
	li 8, 32
	lvx 2, 0, 4
	vspltisw 0, -1
	li 9, 48
	li 10, 0
	lvx 3, 4, 7
	li 11, 64
	li 12, 80
	lvx 4, 4, 8
	li 0, 96
	vslw 0, 0, 0
	lvx 5, 4, 9
	li 4, 112
	std 30, 24(1)                           # 8-byte Folded Spill
	li 30, 0
	std 29, 16(1)                           # 8-byte Folded Spill
.Lca_BB6_2:                                # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lvx 1, 5, 10
	add 29, 5, 10
	addi 30, 30, 8
	cmpw	30, 6
	vspltw 6, 1, 0
	lvx 15, 29, 12
	vmaddfp 6, 6, 2, 0
	vspltw 7, 1, 1
	lvx 17, 29, 0
	vspltw 30, 1, 2
	lvx 19, 29, 4
	vmaddfp 10, 7, 3, 6
	lvx 6, 29, 7
	vspltw 1, 1, 3
	vspltw 7, 6, 0
	vspltw 16, 15, 0
	vmaddfp 11, 7, 2, 0
	lvx 7, 29, 8
	vspltw 18, 17, 0
	vspltw 8, 7, 0
	vspltw 31, 19, 0
	vmaddfp 12, 8, 2, 0
	lvx 8, 29, 9
	vspltw 9, 8, 0
	vmaddfp 13, 9, 2, 0
	lvx 9, 29, 11
	add 29, 3, 10
	vspltw 14, 9, 0
	vmaddfp 10, 30, 4, 10
	vspltw 30, 6, 1
	vmaddfp 11, 30, 3, 11
	vspltw 30, 7, 1
	vmaddfp 1, 1, 5, 10
	vspltw 10, 6, 2
	stvx 1, 3, 10
	addi 10, 10, 128
	vspltw 6, 6, 3
	vspltw 1, 15, 3
	vmaddfp 12, 30, 3, 12
	vspltw 30, 8, 1
	vmaddfp 10, 10, 4, 11
	vspltw 11, 7, 2
	vspltw 7, 7, 3
	vmaddfp 14, 14, 2, 0
	vmaddfp 13, 30, 3, 13
	vspltw 30, 9, 1
	vmaddfp 11, 11, 4, 12
	vspltw 12, 8, 2
	vspltw 8, 8, 3
	vmaddfp 16, 16, 2, 0
	vmaddfp 14, 30, 3, 14
	vspltw 30, 15, 1
	vmaddfp 12, 12, 4, 13
	vspltw 13, 9, 2
	vspltw 9, 9, 3
	vmaddfp 18, 18, 2, 0
	vmaddfp 16, 30, 3, 16
	vspltw 30, 17, 1
	vmaddfp 13, 13, 4, 14
	vspltw 14, 15, 2
	vmaddfp 31, 31, 2, 0
	vmaddfp 18, 30, 3, 18
	vspltw 30, 19, 1
	vmaddfp 14, 14, 4, 16
	vspltw 16, 17, 2
	vmaddfp 31, 30, 3, 31
	vmaddfp 16, 16, 4, 18
	vspltw 18, 19, 2
	vmaddfp 6, 6, 5, 10
	stvx 6, 29, 7
	vspltw 6, 17, 3
	vmaddfp 1, 1, 5, 14
	stvx 1, 29, 12
	vspltw 1, 19, 3
	vmaddfp 18, 18, 4, 31
	vmaddfp 7, 7, 5, 11
	stvx 7, 29, 8
	vmaddfp 8, 8, 5, 12
	stvx 8, 29, 9
	vmaddfp 9, 9, 5, 13
	stvx 9, 29, 11
	vmaddfp 6, 6, 5, 16
	stvx 6, 29, 0
	vmaddfp 1, 1, 5, 18
	stvx 1, 29, 4
	blt	0, .Lca_BB6_2
# %bb.3:
	ld 30, 24(1)                            # 8-byte Folded Reload
	ld 29, 16(1)                            # 8-byte Folded Reload
	addi 1, 1, 48
	blr
.Lca_func_end6:
	.size	ca_sp_mat4_v8, .Lca_func_end6-.Lca_func_begin6
                                        # -- End function
	.section	.text.ca_sp_mat4_v16,"ax",@progbits
	.globl	ca_sp_mat4_v16                  # -- Begin function ca_sp_mat4_v16
	.p2align	2
	.type	ca_sp_mat4_v16,@function
ca_sp_mat4_v16:                         # @ca_sp_mat4_v16
.Lca_func_begin7:
# %bb.0:                                # %entry
	cmpwi	6, 1
	bltlr	0
# %bb.1:                                # %for.body.preheader
	stwu 1, -352(1)
	lvx 2, 0, 4
	li 7, 16
	li 8, 32
	vspltisw 0, -1
	li 9, 48
	li 10, 0
	std 30, 328(1)                          # 8-byte Folded Spill
	li 30, 96
	li 11, 64
	stvx 2, 1, 30                           # 16-byte Folded Spill
	li 12, 80
	li 0, 96
	li 30, 112
	lvx 2, 4, 7
	std 26, 296(1)                          # 8-byte Folded Spill
	li 26, 80
	stvx 2, 1, 26                           # 16-byte Folded Spill
	li 26, 176
	lvx 2, 4, 8
	std 24, 280(1)                          # 8-byte Folded Spill
	li 24, 64
	stvx 2, 1, 24                           # 16-byte Folded Spill
	li 24, 208
	lvx 2, 4, 9
	li 4, 48
	std 22, 264(1)                          # 8-byte Folded Spill
	li 22, 32
	stvx 2, 1, 4                            # 16-byte Folded Spill
	vslw 2, 0, 0
	li 4, 240
	std 23, 272(1)                          # 8-byte Folded Spill
	li 23, 224
	std 25, 288(1)                          # 8-byte Folded Spill
	li 25, 192
	std 27, 304(1)                          # 8-byte Folded Spill
	li 27, 160
	std 28, 312(1)                          # 8-byte Folded Spill
	li 28, 144
	std 29, 320(1)                          # 8-byte Folded Spill
	li 29, 128
	stvx 2, 1, 22                           # 16-byte Folded Spill
	li 22, 0
	std 21, 256(1)                          # 8-byte Folded Spill
.Lca_BB7_2:                                # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lvx 5, 5, 10
	add 21, 5, 10
	addi 22, 22, 16
	std 6, 24(1)                            # 8-byte Folded Spill
	li 6, 96
	vspltw 6, 5, 0
	lvx 0, 1, 6                             # 16-byte Folded Reload
	li 6, 32
	vspltw 7, 5, 1
	lvx 3, 1, 6                             # 16-byte Folded Reload
	li 6, 112
	vspltw 28, 5, 2
	stvx 5, 1, 6                            # 16-byte Folded Spill
	li 6, 80
	vmaddfp 6, 6, 0, 3
	lvx 4, 1, 6                             # 16-byte Folded Reload
	li 6, 240
	lvx 1, 21, 7
	vmaddfp 27, 7, 4, 6
	lvx 6, 21, 8
	vspltw 7, 1, 0
	vspltw 8, 6, 0
	lvx 24, 21, 12
	vspltw 11, 24, 0
	lvx 29, 21, 9
	vmaddfp 25, 8, 0, 3
	lvx 8, 21, 11
	vspltw 9, 29, 0
	vspltw 10, 8, 0
	lvx 22, 21, 0
	vmaddfp 26, 7, 0, 3
	lvx 7, 21, 28
	vspltw 12, 22, 0
	vspltw 15, 7, 0
	lvx 21, 21, 30
	vspltw 13, 21, 0
	lvx 30, 21, 29
	vmaddfp 2, 10, 0, 3
	stvx 2, 1, 6                            # 16-byte Folded Spill
	li 6, 224
	vspltw 14, 30, 0
	vmaddfp 2, 11, 0, 3
	stvx 2, 1, 6                            # 16-byte Folded Spill
	li 6, 208
	vmaddfp 2, 15, 0, 3
	lvx 15, 21, 27
	vspltw 16, 15, 0
	stvx 2, 1, 6                            # 16-byte Folded Spill
	li 6, 192
	vmaddfp 2, 16, 0, 3
	lvx 16, 21, 26
	stvx 2, 1, 6                            # 16-byte Folded Spill
	li 6, 64
	vspltw 17, 16, 0
	lvx 5, 1, 6                             # 16-byte Folded Reload
	li 6, 176
	vmr	2, 1
	vmaddfp 28, 28, 5, 27
	vspltw 27, 1, 1
	vmaddfp 1, 27, 4, 26
	vspltw 26, 6, 1
	stvx 1, 1, 6                            # 16-byte Folded Spill
	li 6, 240
	vmr	27, 6
	vmaddfp 26, 26, 4, 25
	vspltw 25, 8, 1
	vmr	6, 8
	lvx 8, 1, 6                             # 16-byte Folded Reload
	li 6, 224
	vmaddfp 25, 25, 4, 8
	lvx 8, 1, 6                             # 16-byte Folded Reload
	vmaddfp 23, 9, 0, 3
	vmr	1, 29
	vspltw 29, 29, 1
	vmr	11, 24
	vspltw 24, 24, 1
	vmaddfp 12, 12, 0, 3
	vmaddfp 29, 29, 4, 23
	vspltw 23, 22, 1
	vmaddfp 8, 24, 4, 8
	stvx 8, 1, 6                            # 16-byte Folded Spill
	li 6, 160
	vmaddfp 20, 17, 0, 3
	lvx 17, 21, 25
	vspltw 18, 17, 0
	vmaddfp 8, 23, 4, 12
	stvx 8, 1, 6                            # 16-byte Folded Spill
	li 6, 208
	lvx 8, 1, 6                             # 16-byte Folded Reload
	vmaddfp 13, 13, 0, 3
	vmr	24, 22
	vspltw 22, 21, 1
	vmaddfp 10, 18, 0, 3
	lvx 18, 21, 24
	vspltw 19, 18, 0
	vmaddfp 22, 22, 4, 13
	vmr	13, 7
	vspltw 7, 7, 1
	vmaddfp 7, 7, 4, 8
	vspltw 8, 16, 1
	stvx 7, 1, 6                            # 16-byte Folded Spill
	li 6, 192
	lvx 7, 1, 6                             # 16-byte Folded Reload
	vmaddfp 9, 19, 0, 3
	lvx 19, 21, 23
	vspltw 31, 19, 0
	vmr	23, 21
	vspltw 21, 15, 1
	vmaddfp 21, 21, 4, 7
	vmaddfp 7, 8, 4, 20
	vspltw 20, 17, 1
	stvx 7, 1, 6                            # 16-byte Folded Spill
	li 6, 144
	vmaddfp 7, 20, 4, 10
	vspltw 20, 18, 1
	stvx 7, 1, 6                            # 16-byte Folded Spill
	li 6, 128
	vmaddfp 31, 31, 0, 3
	vmaddfp 7, 20, 4, 9
	vspltw 20, 19, 1
	stvx 7, 1, 6                            # 16-byte Folded Spill
	li 6, 112
	vmaddfp 20, 20, 4, 31
	lvx 31, 21, 4
	add 21, 3, 10
	vspltw 9, 31, 0
	vspltw 10, 31, 1
	vmaddfp 14, 14, 0, 3
	vmaddfp 9, 9, 0, 3
	lvx 3, 1, 6                             # 16-byte Folded Reload
	li 6, 48
	vmr	12, 30
	vspltw 30, 30, 1
	vmaddfp 30, 30, 4, 14
	vmaddfp 14, 10, 4, 9
	vspltw 10, 3, 3
	lvx 3, 1, 6                             # 16-byte Folded Reload
	li 6, 240
	vmaddfp 4, 10, 3, 28
	vspltw 28, 2, 2
	stvx 4, 1, 6                            # 16-byte Folded Spill
	li 6, 176
	vmr	8, 2
	lvx 2, 1, 6                             # 16-byte Folded Reload
	li 6, 224
	vmr	4, 27
	vspltw 27, 27, 2
	vmaddfp 28, 28, 5, 2
	lvx 2, 1, 6                             # 16-byte Folded Reload
	li 6, 160
	vmaddfp 27, 27, 5, 26
	vspltw 26, 1, 2
	vmaddfp 29, 26, 5, 29
	vspltw 26, 6, 2
	vmaddfp 26, 26, 5, 25
	vspltw 25, 11, 2
	vmaddfp 25, 25, 5, 2
	lvx 2, 1, 6                             # 16-byte Folded Reload
	li 6, 208
	lvx 10, 1, 6                            # 16-byte Folded Reload
	li 6, 192
	vmr	0, 24
	vspltw 24, 24, 2
	vmaddfp 24, 24, 5, 2
	vmr	2, 23
	vspltw 23, 23, 2
	vmaddfp 23, 23, 5, 22
	vspltw 22, 12, 2
	vmr	9, 1
	vmr	1, 12
	lvx 12, 1, 6                            # 16-byte Folded Reload
	vmaddfp 30, 22, 5, 30
	vspltw 22, 13, 2
	vmaddfp 10, 22, 5, 10
	vspltw 22, 15, 2
	vmaddfp 22, 22, 5, 21
	vspltw 21, 16, 2
	vmaddfp 12, 21, 5, 12
	vspltw 21, 17, 2
	stvx 12, 1, 6                           # 16-byte Folded Spill
	li 6, 144
	lvx 12, 1, 6                            # 16-byte Folded Reload
	li 6, 208
	vmaddfp 12, 21, 5, 12
	vspltw 21, 18, 2
	stvx 12, 1, 6                           # 16-byte Folded Spill
	li 6, 128
	lvx 12, 1, 6                            # 16-byte Folded Reload
	li 6, 224
	vmaddfp 12, 21, 5, 12
	vspltw 21, 19, 2
	stvx 12, 1, 6                           # 16-byte Folded Spill
	li 6, 240
	vspltw 12, 2, 3
	lvx 2, 1, 6                             # 16-byte Folded Reload
	li 6, 192
	stvx 2, 3, 10
	addi 10, 10, 256
	vmaddfp 21, 21, 5, 20
	vspltw 20, 31, 2
	lvx 2, 1, 6                             # 16-byte Folded Reload
	li 6, 208
	vmaddfp 5, 20, 5, 14
	vspltw 20, 8, 3
	vmr	7, 13
	vspltw 13, 1, 3
	vspltw 14, 7, 3
	vmaddfp 28, 20, 3, 28
	vspltw 20, 4, 3
	stvx 28, 21, 7
	vmaddfp 27, 20, 3, 27
	vspltw 20, 9, 3
	stvx 27, 21, 8
	vmaddfp 29, 20, 3, 29
	vspltw 20, 6, 3
	stvx 29, 21, 9
	vmaddfp 7, 14, 3, 10
	vspltw 14, 16, 3
	stvx 7, 21, 28
	vspltw 7, 18, 3
	vspltw 10, 15, 3
	vmaddfp 26, 20, 3, 26
	vspltw 20, 11, 3
	stvx 26, 21, 11
	vspltw 11, 0, 3
	vmaddfp 8, 14, 3, 2
	lvx 2, 1, 6                             # 16-byte Folded Reload
	li 6, 224
	stvx 8, 21, 26
	vmaddfp 11, 11, 3, 24
	stvx 11, 21, 0
	vspltw 11, 17, 3
	vmaddfp 6, 11, 3, 2
	lvx 2, 1, 6                             # 16-byte Folded Reload
	ld 6, 24(1)                             # 8-byte Folded Reload
	cmpw	22, 6
	stvx 6, 21, 25
	vmaddfp 1, 7, 3, 2
	vspltw 7, 19, 3
	stvx 1, 21, 24
	vspltw 1, 31, 3
	vmaddfp 25, 20, 3, 25
	stvx 25, 21, 12
	vmaddfp 12, 12, 3, 23
	stvx 12, 21, 30
	vmaddfp 13, 13, 3, 30
	stvx 13, 21, 29
	vmaddfp 10, 10, 3, 22
	stvx 10, 21, 27
	vmaddfp 7, 7, 3, 21
	stvx 7, 21, 23
	vmaddfp 1, 1, 3, 5
	stvx 1, 21, 4
	blt	0, .Lca_BB7_2
# %bb.3:
	ld 30, 328(1)                           # 8-byte Folded Reload
	ld 29, 320(1)                           # 8-byte Folded Reload
	ld 28, 312(1)                           # 8-byte Folded Reload
	ld 27, 304(1)                           # 8-byte Folded Reload
	ld 26, 296(1)                           # 8-byte Folded Reload
	ld 25, 288(1)                           # 8-byte Folded Reload
	ld 24, 280(1)                           # 8-byte Folded Reload
	ld 23, 272(1)                           # 8-byte Folded Reload
	ld 22, 264(1)                           # 8-byte Folded Reload
	ld 21, 256(1)                           # 8-byte Folded Reload
	addi 1, 1, 352
	blr
.Lca_func_end7:
	.size	ca_sp_mat4_v16, .Lca_func_end7-.Lca_func_begin7
                                        # -- End function
	.section	.text.ca_sp_copy_s,"ax",@progbits
	.globl	ca_sp_copy_s                    # -- Begin function ca_sp_copy_s
	.p2align	2
	.type	ca_sp_copy_s,@function
ca_sp_copy_s:                           # @ca_sp_copy_s
.Lca_func_begin8:
# %bb.0:                                # %entry
	cmpwi	5, 1
	bltlr	0
# %bb.1:                                # %for.body.preheader
	stwu 1, -48(1)
	cmplwi	5, 8
	li 6, 0
	std 29, 16(1)                           # 8-byte Folded Spill
	std 30, 24(1)                           # 8-byte Folded Spill
	blt	0, .Lca_BB8_5
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
.Lca_BB8_3:                                # %vector.body
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
	bne	0, .Lca_BB8_3
# %bb.4:                                # %middle.block
	cmplw	5, 6
	beq	0, .Lca_BB8_7
.Lca_BB8_5:                                # %for.body.preheader8
	slwi 7, 6, 2
	sub	5, 5, 6
	addi 6, 7, -4
	add 4, 4, 6
	add 3, 3, 6
.Lca_BB8_6:                                # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lwzu 6, 4(4)
	addi 5, 5, -1
	cmplwi	5, 0
	stwu 6, 4(3)
	bne	0, .Lca_BB8_6
.Lca_BB8_7:
	ld 30, 24(1)                            # 8-byte Folded Reload
	ld 29, 16(1)                            # 8-byte Folded Reload
	addi 1, 1, 48
	blr
.Lca_func_end8:
	.size	ca_sp_copy_s, .Lca_func_end8-.Lca_func_begin8
                                        # -- End function
	.section	.text.ca_sp_copy_v,"ax",@progbits
	.globl	ca_sp_copy_v                    # -- Begin function ca_sp_copy_v
	.p2align	2
	.type	ca_sp_copy_v,@function
ca_sp_copy_v:                           # @ca_sp_copy_v
.Lca_func_begin9:
# %bb.0:                                # %entry
	srawi 6, 5, 2
	cmpwi	5, 4
	addze 5, 6
	bltlr	0
# %bb.1:                                # %for.body.preheader
	li 6, 0
.Lca_BB9_2:                                # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lvx 2, 4, 6
	addi 5, 5, -1
	cmplwi	5, 0
	stvx 2, 3, 6
	addi 6, 6, 16
	bne	0, .Lca_BB9_2
# %bb.3:                                # %for.cond.cleanup
	blr
.Lca_func_end9:
	.size	ca_sp_copy_v, .Lca_func_end9-.Lca_func_begin9
                                        # -- End function
	.section	.text.ca_sp_bright_s,"ax",@progbits
	.globl	ca_sp_bright_s                  # -- Begin function ca_sp_bright_s
	.p2align	2
	.type	ca_sp_bright_s,@function
ca_sp_bright_s:                         # @ca_sp_bright_s
.Lca_func_begin10:
# %bb.0:                                # %entry
	cmpwi	5, 1
	bltlr	0
# %bb.1:                                # %iter.check
	stwu 1, -128(1)
	cmplwi	5, 4
	li 6, 0
	std 21, 32(1)                           # 8-byte Folded Spill
	std 22, 40(1)                           # 8-byte Folded Spill
	std 23, 48(1)                           # 8-byte Folded Spill
	std 24, 56(1)                           # 8-byte Folded Spill
	std 25, 64(1)                           # 8-byte Folded Spill
	std 26, 72(1)                           # 8-byte Folded Spill
	std 27, 80(1)                           # 8-byte Folded Spill
	std 28, 88(1)                           # 8-byte Folded Spill
	std 29, 96(1)                           # 8-byte Folded Spill
	std 30, 104(1)                          # 8-byte Folded Spill
	blt	0, .Lca_BB10_11
# %bb.2:                                # %vector.main.loop.iter.check
	cmplwi	5, 16
	vspltisb 2, 10
	blt	0, .Lca_BB10_7
# %bb.3:                                # %vector.ph
	rlwinm 6, 5, 0, 1, 27
	li 7, 0
	vslb 3, 2, 2
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
.Lca_BB10_4:                               # %vector.body
                                        # =>This Inner Loop Header: Depth=1
	add 21, 4, 7
	lvx 4, 4, 7
	lvx 5, 21, 8
	add 21, 3, 7
	lvsl 0, 4, 7
	vperm 4, 4, 5, 0
	lvsr 5, 3, 7
	vaddubs 4, 4, 3
	vperm 4, 4, 4, 5
	stvebx 4, 3, 7
	addi 7, 7, 16
	cmplw	6, 7
	stvebx 4, 21, 8
	stvebx 4, 21, 9
	stvebx 4, 21, 10
	stvebx 4, 21, 11
	stvebx 4, 21, 12
	stvebx 4, 21, 0
	stvebx 4, 21, 30
	stvebx 4, 21, 29
	stvebx 4, 21, 28
	stvebx 4, 21, 27
	stvebx 4, 21, 26
	stvebx 4, 21, 25
	stvebx 4, 21, 24
	stvebx 4, 21, 23
	stvebx 4, 21, 22
	bne	0, .Lca_BB10_4
# %bb.5:                                # %middle.block
	cmplw	5, 6
	beq	0, .Lca_BB10_10
# %bb.6:                                # %vec.epilog.iter.check
	andi. 7, 5, 12
	beq	0, .Lca_BB10_11
.Lca_BB10_7:                               # %vec.epilog.ph
	mr	7, 6
	rlwinm 6, 5, 0, 1, 29
	vslb 2, 2, 2
	addi 8, 1, 16
	li 9, 3
	li 10, 2
	li 11, 1
.Lca_BB10_8:                               # %vec.epilog.vector.body
                                        # =>This Inner Loop Header: Depth=1
	lbzx 12, 4, 7
	stb 12, 16(1)
	add 12, 4, 7
	lbz 0, 1(12)
	stb 0, 17(1)
	lbz 0, 2(12)
	stb 0, 18(1)
	lbz 12, 3(12)
	stb 12, 19(1)
	add 12, 3, 7
	lvx 3, 0, 8
	vaddubs 3, 3, 2
	lvsr 4, 3, 7
	vperm 3, 3, 3, 4
	stvebx 3, 3, 7
	addi 7, 7, 4
	cmplw	6, 7
	stvebx 3, 12, 9
	stvebx 3, 12, 10
	stvebx 3, 12, 11
	bne	0, .Lca_BB10_8
# %bb.9:                                # %vec.epilog.middle.block
	cmplw	5, 6
	bne	0, .Lca_BB10_11
.Lca_BB10_10:
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
.Lca_BB10_11:                              # %for.body.preheader
	addi 7, 6, -1
	add 4, 4, 7
	add 3, 3, 7
	b .Lca_BB10_13
.Lca_BB10_12:                              # %for.body
                                        #   in Loop: Header=BB10_13 Depth=1
	addi 6, 6, 1
	stbu 7, 1(3)
	cmplw	5, 6
	beq	0, .Lca_BB10_10
.Lca_BB10_13:                              # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lbzu 7, 1(4)
	addi 7, 7, 40
	cmplwi	7, 255
	blt	0, .Lca_BB10_12
# %bb.14:                               # %for.body
                                        #   in Loop: Header=BB10_13 Depth=1
	li 7, 255
	b .Lca_BB10_12
.Lca_func_end10:
	.size	ca_sp_bright_s, .Lca_func_end10-.Lca_func_begin10
                                        # -- End function
	.section	.text.ca_sp_bright_v,"ax",@progbits
	.globl	ca_sp_bright_v                  # -- Begin function ca_sp_bright_v
	.p2align	2
	.type	ca_sp_bright_v,@function
ca_sp_bright_v:                         # @ca_sp_bright_v
.Lca_func_begin11:
# %bb.0:                                # %entry
	srawi 6, 5, 4
	cmpwi	5, 16
	addze 5, 6
	bltlr	0
# %bb.1:                                # %for.body.preheader
	vspltisb 2, 10
	li 6, 0
	vslb 2, 2, 2
.Lca_BB11_2:                               # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lvx 3, 4, 6
	addi 5, 5, -1
	cmplwi	5, 0
	vaddubs 3, 3, 2
	stvx 3, 3, 6
	addi 6, 6, 16
	bne	0, .Lca_BB11_2
# %bb.3:                                # %for.cond.cleanup
	blr
.Lca_func_end11:
	.size	ca_sp_bright_v, .Lca_func_end11-.Lca_func_begin11
                                        # -- End function
	.section	.rodata.cst16,"aM",@progbits,16
	.p2align	4, 0x0                          # -- Begin function ca_sp_u8f_s
.Lca_CPI12_0:
	.long	0x3b808081                      # float 0.00392156886
	.long	0x3b808081                      # float 0.00392156886
	.long	0x3b808081                      # float 0.00392156886
	.long	0x3b808081                      # float 0.00392156886
	.section	.rodata.cst4,"aM",@progbits,4
	.p2align	2, 0x0
.Lca_CPI12_1:
	.long	0x3b808081                      # float 0.00392156886
	.section	.text.ca_sp_u8f_s,"ax",@progbits
	.globl	ca_sp_u8f_s
	.p2align	2
	.type	ca_sp_u8f_s,@function
ca_sp_u8f_s:                            # @ca_sp_u8f_s
.Lca_func_begin12:
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
	blt	0, .Lca_BB12_5
# %bb.2:                                # %vector.ph
	li 9, .Lca_CPI12_0@l
	lis 10, .Lca_CPI12_0@ha
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
.Lca_BB12_3:                               # %vector.body
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
	bne	0, .Lca_BB12_3
# %bb.4:                                # %middle.block
	cmplw	5, 6
	beq	0, .Lca_BB12_7
.Lca_BB12_5:                               # %for.body.preheader7
	sub	5, 5, 6
	slwi 7, 6, 2
	add 4, 6, 4
	lis 6, .Lca_CPI12_1@ha
	lfs 0, .Lca_CPI12_1@l(6)
	add 7, 7, 3
	addi 3, 4, -1
	addi 4, 7, -4
.Lca_BB12_6:                               # %for.body
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
	bne	0, .Lca_BB12_6
.Lca_BB12_7:
	ld 30, 104(1)                           # 8-byte Folded Reload
	ld 29, 96(1)                            # 8-byte Folded Reload
	ld 28, 88(1)                            # 8-byte Folded Reload
	ld 27, 80(1)                            # 8-byte Folded Reload
	addi 1, 1, 128
	blr
.Lca_func_end12:
	.size	ca_sp_u8f_s, .Lca_func_end12-.Lca_func_begin12
                                        # -- End function
	.section	.rodata.cst16,"aM",@progbits,16
	.p2align	4, 0x0                          # -- Begin function ca_sp_u8f_v
.Lca_CPI13_0:
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
.Lca_CPI13_1:
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
.Lca_CPI13_2:
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
.Lca_CPI13_3:
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
.Lca_CPI13_4:
	.long	0x3b808081                      # float 0.00392156886
	.long	0x3b808081                      # float 0.00392156886
	.long	0x3b808081                      # float 0.00392156886
	.long	0x3b808081                      # float 0.00392156886
	.section	.text.ca_sp_u8f_v,"ax",@progbits
	.globl	ca_sp_u8f_v
	.p2align	2
	.type	ca_sp_u8f_v,@function
ca_sp_u8f_v:                            # @ca_sp_u8f_v
.Lca_func_begin13:
# %bb.0:                                # %entry
	srawi 6, 5, 4
	cmpwi	5, 16
	addze 5, 6
	bltlr	0
# %bb.1:
	li 6, .Lca_CPI13_0@l
	lis 7, .Lca_CPI13_0@ha
	vspltisw 6, -1
	lvx 3, 7, 6
	li 6, .Lca_CPI13_1@l
	lis 7, .Lca_CPI13_1@ha
	lvx 4, 7, 6
	li 6, .Lca_CPI13_2@l
	lis 7, .Lca_CPI13_2@ha
	li 8, .Lca_CPI13_3@l
	lvx 5, 7, 6
	lis 6, .Lca_CPI13_3@ha
	li 7, .Lca_CPI13_4@l
	vxor 2, 2, 2
	lvx 0, 6, 8
	lis 8, .Lca_CPI13_4@ha
	li 6, 16
	lvx 1, 8, 7
	li 7, 32
	li 8, 48
	vslw 6, 6, 6
.Lca_BB13_2:                               # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lvx 7, 0, 4
	addi 5, 5, -1
	addi 4, 4, 16
	cmplwi	5, 0
	vperm 8, 2, 7, 3
	vperm 9, 2, 7, 4
	vperm 10, 2, 7, 5
	vperm 7, 2, 7, 0
	vcfux 8, 8, 0
	vcfux 9, 9, 0
	vcfux 10, 10, 0
	vcfux 7, 7, 0
	vmaddfp 8, 8, 1, 6
	stvx 8, 0, 3
	vmaddfp 9, 9, 1, 6
	stvx 9, 3, 6
	vmaddfp 10, 10, 1, 6
	stvx 10, 3, 7
	vmaddfp 7, 7, 1, 6
	stvx 7, 3, 8
	addi 3, 3, 64
	bne	0, .Lca_BB13_2
# %bb.3:                                # %for.cond.cleanup
	blr
.Lca_func_end13:
	.size	ca_sp_u8f_v, .Lca_func_end13-.Lca_func_begin13
                                        # -- End function
	.section	.rodata.cst4,"aM",@progbits,4
	.p2align	2, 0x0                          # -- Begin function ca_sp_f2u8_s
.Lca_CPI14_0:
	.long	0x3f000000                      # float 0.5
.Lca_CPI14_1:
	.long	0x437f0000                      # float 255
.Lca_CPI14_2:
	.long	0x00000000                      # float 0
	.section	.text.ca_sp_f2u8_s,"ax",@progbits
	.globl	ca_sp_f2u8_s
	.p2align	2
	.type	ca_sp_f2u8_s,@function
ca_sp_f2u8_s:                           # @ca_sp_f2u8_s
.Lca_func_begin14:
# %bb.0:                                # %entry
	cmpwi	5, 1
	bltlr	0
# %bb.1:                                # %for.body.preheader
	stwu 1, -32(1)
	lis 6, .Lca_CPI14_0@ha
	addi 4, 4, -4
	lfs 0, .Lca_CPI14_0@l(6)
	lis 6, .Lca_CPI14_1@ha
	addi 3, 3, -1
	lfs 1, .Lca_CPI14_1@l(6)
	lis 6, .Lca_CPI14_2@ha
	lfs 2, .Lca_CPI14_2@l(6)
.Lca_BB14_2:                               # %for.body
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
	bne	0, .Lca_BB14_2
# %bb.3:
	addi 1, 1, 32
	blr
.Lca_func_end14:
	.size	ca_sp_f2u8_s, .Lca_func_end14-.Lca_func_begin14
                                        # -- End function
	.section	.rodata.cst16,"aM",@progbits,16
	.p2align	4, 0x0                          # -- Begin function ca_sp_f2u8_v
.Lca_CPI15_0:
	.long	0x437f0000                      # float 255
	.long	0x437f0000                      # float 255
	.long	0x437f0000                      # float 255
	.long	0x437f0000                      # float 255
.Lca_CPI15_1:
	.long	0x3f000000                      # float 0.5
	.long	0x3f000000                      # float 0.5
	.long	0x3f000000                      # float 0.5
	.long	0x3f000000                      # float 0.5
.Lca_CPI15_2:
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
.Lca_CPI15_3:
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
	.section	.text.ca_sp_f2u8_v,"ax",@progbits
	.globl	ca_sp_f2u8_v
	.p2align	2
	.type	ca_sp_f2u8_v,@function
ca_sp_f2u8_v:                           # @ca_sp_f2u8_v
.Lca_func_begin15:
# %bb.0:                                # %entry
	srawi 6, 5, 4
	cmpwi	5, 16
	addze 5, 6
	bltlr	0
# %bb.1:                                # %for.body.lr.ph
	stwu 1, -80(1)
	li 9, .Lca_CPI15_1@l
	lis 10, .Lca_CPI15_1@ha
	vxor 5, 5, 5
	lvx 3, 10, 9
	li 7, .Lca_CPI15_0@l
	lis 8, .Lca_CPI15_0@ha
	li 9, .Lca_CPI15_2@l
	lis 10, .Lca_CPI15_2@ha
	lvx 2, 8, 7
	li 11, .Lca_CPI15_3@l
	lis 12, .Lca_CPI15_3@ha
	lvx 4, 10, 9
	addi 6, 1, 32
	addi 7, 1, 48
	lvx 0, 12, 11
	li 8, 0
	addi 9, 1, 64
	addi 10, 1, 16
.Lca_BB15_2:                               # %for.body
                                        # =>This Loop Header: Depth=1
                                        #     Child Loop BB15_3 Depth 2
	li 11, 0
.Lca_BB15_3:                               # %for.body4
                                        #   Parent Loop BB15_2 Depth=1
                                        # =>  This Inner Loop Header: Depth=2
	lvx 1, 4, 11
	vmaddfp 1, 1, 2, 3
	vcmpgtfp 6, 1, 5
	vand 1, 6, 1
	vcmpgtfp 6, 2, 1
	vsel 1, 2, 1, 6
	vctuxs 1, 1, 0
	stvx 1, 10, 11
	addi 11, 11, 16
	cmplwi	11, 64
	bne	0, .Lca_BB15_3
# %bb.4:                                # %for.cond.cleanup3
                                        #   in Loop: Header=BB15_2 Depth=1
	lvx 1, 0, 10
	slwi 11, 8, 4
	addi 8, 8, 1
	cmplw	8, 5
	addi 4, 4, 64
	lvx 6, 0, 6
	vperm 1, 1, 6, 4
	lvx 6, 0, 7
	lvx 7, 0, 9
	vperm 6, 6, 7, 4
	vperm 1, 1, 6, 0
	stvx 1, 3, 11
	bne	0, .Lca_BB15_2
# %bb.5:
	addi 1, 1, 80
	blr
.Lca_func_end15:
	.size	ca_sp_f2u8_v, .Lca_func_end15-.Lca_func_begin15
                                        # -- End function
