# kernels.c: clang version 22.1.8 (git@github.com:landaire/llvm-project.git 2f714b11080e5a449de3969e3c4477cf5cbe92ab)
# flags: --target=powerpc64-unknown-xbox360 -ffreestanding -fno-builtin -fms-extensions -ffunction-sections -fdata-sections -fno-ident -fno-addrsig -fno-asynchronous-unwind-tables -g0 -O3
	.section	.text.l3v_k_saxpy,"ax",@progbits
	.globl	l3v_k_saxpy                     # -- Begin function l3v_k_saxpy
	.p2align	2
	.type	l3v_k_saxpy,@function
l3v_k_saxpy:                            # @l3v_k_saxpy
.Ll3v_func_begin0:
# %bb.0:                                # %entry
	cmpwi	6, 1
	bltlr	0
# %bb.1:                                # %iter.check
	stwu 1, -208(1)
	std 20, 104(1)                          # 8-byte Folded Spill
	cmplwi	6, 4
	li 20, 0
	std 14, 56(1)                           # 8-byte Folded Spill
	std 15, 64(1)                           # 8-byte Folded Spill
	std 16, 72(1)                           # 8-byte Folded Spill
	std 17, 80(1)                           # 8-byte Folded Spill
	std 18, 88(1)                           # 8-byte Folded Spill
	std 19, 96(1)                           # 8-byte Folded Spill
	std 21, 112(1)                          # 8-byte Folded Spill
	std 22, 120(1)                          # 8-byte Folded Spill
	std 23, 128(1)                          # 8-byte Folded Spill
	std 24, 136(1)                          # 8-byte Folded Spill
	std 25, 144(1)                          # 8-byte Folded Spill
	std 26, 152(1)                          # 8-byte Folded Spill
	std 27, 160(1)                          # 8-byte Folded Spill
	std 28, 168(1)                          # 8-byte Folded Spill
	std 29, 176(1)                          # 8-byte Folded Spill
	std 30, 184(1)                          # 8-byte Folded Spill
	std 31, 192(1)                          # 8-byte Folded Spill
	blt	0, .Ll3v_BB0_12
# %bb.2:                                # %vector.memcheck
	slwi 5, 6, 2
	add 7, 4, 5
	cmplw	3, 7
	add 5, 3, 5
	bc 4, 0, .Ll3v_BB0_4
# %bb.3:                                # %vector.memcheck
	cmplw	4, 5
	bc 12, 0, .Ll3v_BB0_12
.Ll3v_BB0_4:                                # %vector.main.loop.iter.check
	cmplwi	6, 32
	blt	0, .Ll3v_BB0_9
# %bb.5:                                # %vector.ph
	addi 5, 1, 32
	stfs 1, 32(1)
	rlwinm 20, 6, 0, 1, 26
	li 7, 0
	li 8, 16
	lvx128 2, 0, 5
	li 9, 32
	li 10, 48
	li 11, 64
	li 12, 80
	vspltw128 2, 2, 0
	li 0, 96
	li 30, 112
	li 29, 127
	li 17, 52
	li 16, 76
	li 15, 72
	li 14, 68
	li 31, 92
	li 28, 88
	li 27, 84
	li 26, 108
	li 25, 104
	li 24, 100
	li 23, 124
	li 22, 120
	li 21, 116
	mr	5, 20
.Ll3v_BB0_6:                                # %vector.body
                                        # =>This Inner Loop Header: Depth=1
	add 19, 4, 7
	lvx128 28, 4, 7
	li 18, 28
	addi 5, 5, -32
	lvx128 27, 19, 8
	cmplwi	5, 0
	lvx128 1, 19, 9
	lvx128 6, 19, 10
	lvx128 7, 19, 11
	lvx128 8, 19, 12
	lvx128 9, 19, 0
	lvx128 10, 19, 30
	lvx128 11, 19, 29
	add 19, 3, 7
	lvx128 12, 3, 7
	lvx128 13, 19, 8
	lvx128 14, 19, 9
	lvsl 3, 4, 7
	vperm128 31, 27, 1, 3
	lvsl 4, 3, 7
	vperm128 29, 13, 14, 4
	vmaddfp128 29, 2, 31, 29
	lvsr 5, 19, 8
	vperm128 5, 29, 29, 5
	lvx128 15, 19, 10
	vperm128 1, 1, 6, 3
	lvx128 16, 19, 11
	lvx128 17, 19, 12
	lvx128 18, 19, 0
	lvx128 19, 19, 30
	lvx128 30, 19, 29
	stvewx 5, 19, 18
	li 18, 24
	stvewx 5, 19, 18
	li 18, 20
	stvewx 5, 19, 18
	li 18, 44
	stvewx 5, 19, 8
	vperm128 5, 14, 15, 4
	vmaddfp128 5, 2, 1, 5
	lvsr 0, 19, 9
	vperm128 5, 5, 5, 0
	stvewx 5, 19, 18
	li 18, 40
	vperm128 0, 6, 7, 3
	stvewx 5, 19, 18
	li 18, 36
	stvewx 5, 19, 18
	li 18, 60
	stvewx 5, 19, 9
	vperm128 5, 15, 16, 4
	vmaddfp128 5, 2, 0, 5
	lvsr 1, 19, 10
	vperm128 0, 7, 8, 3
	vperm128 5, 5, 5, 1
	lvsr 1, 19, 11
	stvewx 5, 19, 18
	li 18, 56
	stvewx 5, 19, 18
	li 18, 12
	stvewx 5, 19, 17
	stvewx 5, 19, 10
	vperm128 5, 16, 17, 4
	vmaddfp128 5, 2, 0, 5
	vperm128 5, 5, 5, 1
	lvsr 1, 19, 12
	stvewx 5, 19, 16
	vperm128 0, 8, 9, 3
	stvewx 5, 19, 15
	stvewx 5, 19, 14
	stvewx 5, 19, 11
	vperm128 5, 17, 18, 4
	vmaddfp128 5, 2, 0, 5
	vperm128 5, 5, 5, 1
	lvsr 1, 19, 0
	stvewx 5, 19, 31
	vperm128 0, 9, 10, 3
	stvewx 5, 19, 28
	stvewx 5, 19, 27
	stvewx 5, 19, 12
	vperm128 5, 18, 19, 4
	vmaddfp128 5, 2, 0, 5
	vperm128 5, 5, 5, 1
	lvsr 1, 19, 30
	stvewx 5, 19, 26
	vperm128 0, 10, 11, 3
	stvewx 5, 19, 25
	vperm128 3, 28, 27, 3
	stvewx 5, 19, 24
	stvewx 5, 19, 0
	vperm128 5, 19, 30, 4
	vmaddfp128 5, 2, 0, 5
	vperm128 5, 5, 5, 1
	stvewx 5, 19, 23
	vperm128 4, 12, 13, 4
	stvewx 5, 19, 22
	vmaddfp128 4, 2, 3, 4
	stvewx 5, 19, 21
	stvewx 5, 19, 30
	lvsr 5, 3, 7
	vperm128 3, 4, 4, 5
	stvewx 3, 19, 18
	li 18, 8
	stvewx 3, 19, 18
	li 18, 4
	stvewx 3, 3, 7
	addi 7, 7, 128
	stvewx 3, 19, 18
	bne	0, .Ll3v_BB0_6
# %bb.7:                                # %middle.block
	cmplw	6, 20
	beq	0, .Ll3v_BB0_14
# %bb.8:                                # %vec.epilog.iter.check
	andi. 5, 6, 28
	beq	0, .Ll3v_BB0_12
.Ll3v_BB0_9:                                # %vec.epilog.ph
	stfs 1, 16(1)
	addi 8, 1, 16
	mr	7, 20
	rlwinm 20, 6, 0, 1, 29
	li 9, 12
	lvx128 2, 0, 8
	sub	5, 7, 20
	slwi 7, 7, 2
	li 8, 15
	li 10, 8
	vspltw128 2, 2, 0
	li 11, 4
.Ll3v_BB0_10:                               # %vec.epilog.vector.body
                                        # =>This Inner Loop Header: Depth=1
	add 12, 4, 7
	lvx128 4, 4, 7
	addi 5, 5, 4
	cmplwi	5, 0
	lvx128 5, 12, 8
	add 12, 3, 7
	lvx128 0, 3, 7
	lvx128 1, 12, 8
	lvsl 3, 4, 7
	vperm128 3, 4, 5, 3
	lvsl 4, 3, 7
	vperm128 4, 0, 1, 4
	vmaddfp128 4, 2, 3, 4
	lvsr 5, 3, 7
	vperm128 3, 4, 4, 5
	stvewx 3, 3, 7
	addi 7, 7, 16
	stvewx 3, 12, 9
	stvewx 3, 12, 10
	stvewx 3, 12, 11
	bne	0, .Ll3v_BB0_10
# %bb.11:                               # %vec.epilog.middle.block
	cmplw	6, 20
	beq	0, .Ll3v_BB0_14
.Ll3v_BB0_12:                               # %for.body.preheader
	slwi 7, 20, 2
	sub	5, 6, 20
	addi 6, 7, -4
	add 4, 4, 6
	add 3, 3, 6
.Ll3v_BB0_13:                               # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lfsu 0, 4(3)
	addi 5, 5, -1
	cmplwi	5, 0
	lfsu 2, 4(4)
	fmadds 0, 1, 2, 0
	stfs 0, 0(3)
	bne	0, .Ll3v_BB0_13
.Ll3v_BB0_14:
	ld 31, 192(1)                           # 8-byte Folded Reload
	ld 30, 184(1)                           # 8-byte Folded Reload
	ld 29, 176(1)                           # 8-byte Folded Reload
	ld 28, 168(1)                           # 8-byte Folded Reload
	ld 27, 160(1)                           # 8-byte Folded Reload
	ld 26, 152(1)                           # 8-byte Folded Reload
	ld 25, 144(1)                           # 8-byte Folded Reload
	ld 24, 136(1)                           # 8-byte Folded Reload
	ld 23, 128(1)                           # 8-byte Folded Reload
	ld 22, 120(1)                           # 8-byte Folded Reload
	ld 21, 112(1)                           # 8-byte Folded Reload
	ld 20, 104(1)                           # 8-byte Folded Reload
	ld 19, 96(1)                            # 8-byte Folded Reload
	ld 18, 88(1)                            # 8-byte Folded Reload
	ld 17, 80(1)                            # 8-byte Folded Reload
	ld 16, 72(1)                            # 8-byte Folded Reload
	ld 15, 64(1)                            # 8-byte Folded Reload
	ld 14, 56(1)                            # 8-byte Folded Reload
	addi 1, 1, 208
	blr
.Ll3v_func_end0:
	.size	l3v_k_saxpy, .Ll3v_func_end0-.Ll3v_func_begin0
                                        # -- End function
	.section	.text.l3v_k_saxpy_r,"ax",@progbits
	.globl	l3v_k_saxpy_r                   # -- Begin function l3v_k_saxpy_r
	.p2align	2
	.type	l3v_k_saxpy_r,@function
l3v_k_saxpy_r:                          # @l3v_k_saxpy_r
.Ll3v_func_begin1:
# %bb.0:                                # %entry
	cmpwi	6, 1
	bltlr	0
# %bb.1:                                # %iter.check
	stwu 1, -224(1)
	cmplwi	6, 4
	li 0, 0
	std 14, 72(1)                           # 8-byte Folded Spill
	std 15, 80(1)                           # 8-byte Folded Spill
	std 16, 88(1)                           # 8-byte Folded Spill
	std 17, 96(1)                           # 8-byte Folded Spill
	std 18, 104(1)                          # 8-byte Folded Spill
	std 19, 112(1)                          # 8-byte Folded Spill
	std 20, 120(1)                          # 8-byte Folded Spill
	std 21, 128(1)                          # 8-byte Folded Spill
	std 22, 136(1)                          # 8-byte Folded Spill
	std 23, 144(1)                          # 8-byte Folded Spill
	std 24, 152(1)                          # 8-byte Folded Spill
	std 25, 160(1)                          # 8-byte Folded Spill
	std 26, 168(1)                          # 8-byte Folded Spill
	std 27, 176(1)                          # 8-byte Folded Spill
	std 28, 184(1)                          # 8-byte Folded Spill
	std 29, 192(1)                          # 8-byte Folded Spill
	std 30, 200(1)                          # 8-byte Folded Spill
	std 31, 208(1)                          # 8-byte Folded Spill
	blt	0, .Ll3v_BB1_10
# %bb.2:                                # %vector.main.loop.iter.check
	cmplwi	6, 32
	blt	0, .Ll3v_BB1_7
# %bb.3:                                # %vector.ph
	addi 25, 1, 48
	stfs 1, 48(1)
	rlwinm 21, 6, 0, 1, 26
	li 7, 0
	li 8, 16
	lvx128 2, 0, 25
	li 9, 32
	li 10, 48
	li 11, 64
	li 12, 80
	vspltw128 2, 2, 0
	stw 21, 28(1)                           # 4-byte Folded Spill
	li 0, 96
	li 30, 112
	li 29, 127
	li 18, 56
	li 17, 52
	li 16, 76
	li 15, 72
	li 14, 68
	li 31, 92
	li 5, 88
	li 28, 84
	li 27, 108
	li 26, 104
	li 25, 100
	li 24, 124
	li 23, 120
	li 22, 116
.Ll3v_BB1_4:                                # %vector.body
                                        # =>This Inner Loop Header: Depth=1
	add 20, 4, 7
	lvx128 28, 4, 7
	li 19, 28
	addi 21, 21, -32
	lvx128 27, 20, 8
	cmplwi	21, 0
	lvx128 1, 20, 9
	lvx128 6, 20, 10
	lvx128 7, 20, 11
	lvx128 8, 20, 12
	lvx128 9, 20, 0
	lvx128 10, 20, 30
	lvx128 11, 20, 29
	add 20, 3, 7
	lvx128 12, 3, 7
	lvx128 13, 20, 8
	lvx128 14, 20, 9
	lvsl 3, 4, 7
	vperm128 31, 27, 1, 3
	lvsl 4, 3, 7
	vperm128 29, 13, 14, 4
	vmaddfp128 29, 2, 31, 29
	lvsr 5, 20, 8
	vperm128 5, 29, 29, 5
	lvx128 15, 20, 10
	vperm128 1, 1, 6, 3
	lvx128 16, 20, 11
	lvx128 17, 20, 12
	lvx128 18, 20, 0
	lvx128 19, 20, 30
	lvx128 30, 20, 29
	stvewx 5, 20, 19
	li 19, 24
	stvewx 5, 20, 19
	li 19, 20
	stvewx 5, 20, 19
	li 19, 44
	stvewx 5, 20, 8
	vperm128 5, 14, 15, 4
	vmaddfp128 5, 2, 1, 5
	lvsr 0, 20, 9
	vperm128 5, 5, 5, 0
	stvewx 5, 20, 19
	li 19, 40
	vperm128 0, 6, 7, 3
	stvewx 5, 20, 19
	li 19, 36
	stvewx 5, 20, 19
	li 19, 60
	stvewx 5, 20, 9
	vperm128 5, 15, 16, 4
	vmaddfp128 5, 2, 0, 5
	lvsr 1, 20, 10
	vperm128 0, 7, 8, 3
	vperm128 5, 5, 5, 1
	lvsr 1, 20, 11
	stvewx 5, 20, 19
	li 19, 12
	stvewx 5, 20, 18
	stvewx 5, 20, 17
	stvewx 5, 20, 10
	vperm128 5, 16, 17, 4
	vmaddfp128 5, 2, 0, 5
	vperm128 5, 5, 5, 1
	lvsr 1, 20, 12
	stvewx 5, 20, 16
	vperm128 0, 8, 9, 3
	stvewx 5, 20, 15
	stvewx 5, 20, 14
	stvewx 5, 20, 11
	vperm128 5, 17, 18, 4
	vmaddfp128 5, 2, 0, 5
	vperm128 5, 5, 5, 1
	lvsr 1, 20, 0
	stvewx 5, 20, 31
	vperm128 0, 9, 10, 3
	stvewx 5, 20, 5
	stvewx 5, 20, 28
	stvewx 5, 20, 12
	vperm128 5, 18, 19, 4
	vmaddfp128 5, 2, 0, 5
	vperm128 5, 5, 5, 1
	lvsr 1, 20, 30
	stvewx 5, 20, 27
	vperm128 0, 10, 11, 3
	stvewx 5, 20, 26
	vperm128 3, 28, 27, 3
	stvewx 5, 20, 25
	stvewx 5, 20, 0
	vperm128 5, 19, 30, 4
	vmaddfp128 5, 2, 0, 5
	vperm128 5, 5, 5, 1
	stvewx 5, 20, 24
	vperm128 4, 12, 13, 4
	stvewx 5, 20, 23
	vmaddfp128 4, 2, 3, 4
	stvewx 5, 20, 22
	stvewx 5, 20, 30
	lvsr 5, 3, 7
	vperm128 3, 4, 4, 5
	stvewx 3, 20, 19
	li 19, 8
	stvewx 3, 20, 19
	li 19, 4
	stvewx 3, 3, 7
	addi 7, 7, 128
	stvewx 3, 20, 19
	bne	0, .Ll3v_BB1_4
# %bb.5:                                # %middle.block
	lwz 0, 28(1)                            # 4-byte Folded Reload
	cmplw	6, 0
	beq	0, .Ll3v_BB1_12
# %bb.6:                                # %vec.epilog.iter.check
	andi. 5, 6, 28
	beq	0, .Ll3v_BB1_10
.Ll3v_BB1_7:                                # %vec.epilog.ph
	stfs 1, 32(1)
	addi 8, 1, 32
	mr	7, 0
	rlwinm 0, 6, 0, 1, 29
	li 9, 12
	lvx128 2, 0, 8
	sub	5, 7, 0
	slwi 7, 7, 2
	li 8, 15
	li 10, 8
	vspltw128 2, 2, 0
	li 11, 4
.Ll3v_BB1_8:                                # %vec.epilog.vector.body
                                        # =>This Inner Loop Header: Depth=1
	add 12, 4, 7
	lvx128 4, 4, 7
	addi 5, 5, 4
	cmplwi	5, 0
	lvx128 5, 12, 8
	add 12, 3, 7
	lvx128 0, 3, 7
	lvx128 1, 12, 8
	lvsl 3, 4, 7
	vperm128 3, 4, 5, 3
	lvsl 4, 3, 7
	vperm128 4, 0, 1, 4
	vmaddfp128 4, 2, 3, 4
	lvsr 5, 3, 7
	vperm128 3, 4, 4, 5
	stvewx 3, 3, 7
	addi 7, 7, 16
	stvewx 3, 12, 9
	stvewx 3, 12, 10
	stvewx 3, 12, 11
	bne	0, .Ll3v_BB1_8
# %bb.9:                                # %vec.epilog.middle.block
	cmplw	6, 0
	beq	0, .Ll3v_BB1_12
.Ll3v_BB1_10:                               # %for.body.preheader
	slwi 7, 0, 2
	sub	5, 6, 0
	addi 6, 7, -4
	add 4, 4, 6
	add 3, 3, 6
.Ll3v_BB1_11:                               # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lfsu 0, 4(3)
	addi 5, 5, -1
	cmplwi	5, 0
	lfsu 2, 4(4)
	fmadds 0, 1, 2, 0
	stfs 0, 0(3)
	bne	0, .Ll3v_BB1_11
.Ll3v_BB1_12:
	ld 31, 208(1)                           # 8-byte Folded Reload
	ld 30, 200(1)                           # 8-byte Folded Reload
	ld 29, 192(1)                           # 8-byte Folded Reload
	ld 28, 184(1)                           # 8-byte Folded Reload
	ld 27, 176(1)                           # 8-byte Folded Reload
	ld 26, 168(1)                           # 8-byte Folded Reload
	ld 25, 160(1)                           # 8-byte Folded Reload
	ld 24, 152(1)                           # 8-byte Folded Reload
	ld 23, 144(1)                           # 8-byte Folded Reload
	ld 22, 136(1)                           # 8-byte Folded Reload
	ld 21, 128(1)                           # 8-byte Folded Reload
	ld 20, 120(1)                           # 8-byte Folded Reload
	ld 19, 112(1)                           # 8-byte Folded Reload
	ld 18, 104(1)                           # 8-byte Folded Reload
	ld 17, 96(1)                            # 8-byte Folded Reload
	ld 16, 88(1)                            # 8-byte Folded Reload
	ld 15, 80(1)                            # 8-byte Folded Reload
	ld 14, 72(1)                            # 8-byte Folded Reload
	addi 1, 1, 224
	blr
.Ll3v_func_end1:
	.size	l3v_k_saxpy_r, .Ll3v_func_end1-.Ll3v_func_begin1
                                        # -- End function
	.section	.text.l3v_k_scale_inplace,"ax",@progbits
	.globl	l3v_k_scale_inplace             # -- Begin function l3v_k_scale_inplace
	.p2align	2
	.type	l3v_k_scale_inplace,@function
l3v_k_scale_inplace:                    # @l3v_k_scale_inplace
.Ll3v_func_begin2:
# %bb.0:                                # %entry
	cmpwi	5, 1
	bltlr	0
# %bb.1:                                # %iter.check
	stwu 1, -224(1)
	cmplwi	5, 4
	li 11, 0
	std 14, 72(1)                           # 8-byte Folded Spill
	std 15, 80(1)                           # 8-byte Folded Spill
	std 16, 88(1)                           # 8-byte Folded Spill
	std 17, 96(1)                           # 8-byte Folded Spill
	std 18, 104(1)                          # 8-byte Folded Spill
	std 19, 112(1)                          # 8-byte Folded Spill
	std 20, 120(1)                          # 8-byte Folded Spill
	std 21, 128(1)                          # 8-byte Folded Spill
	std 22, 136(1)                          # 8-byte Folded Spill
	std 23, 144(1)                          # 8-byte Folded Spill
	std 24, 152(1)                          # 8-byte Folded Spill
	std 25, 160(1)                          # 8-byte Folded Spill
	std 26, 168(1)                          # 8-byte Folded Spill
	std 27, 176(1)                          # 8-byte Folded Spill
	std 28, 184(1)                          # 8-byte Folded Spill
	std 29, 192(1)                          # 8-byte Folded Spill
	std 30, 200(1)                          # 8-byte Folded Spill
	std 31, 208(1)                          # 8-byte Folded Spill
	blt	0, .Ll3v_BB2_10
# %bb.2:                                # %vector.main.loop.iter.check
	cmplwi	5, 32
	blt	0, .Ll3v_BB2_7
# %bb.3:                                # %vector.ph
	stw 5, 28(1)                            # 4-byte Folded Spill
	rlwinm 26, 5, 0, 1, 26
	addi 5, 1, 48
	addi 6, 3, 112
	li 7, -112
	stw 3, 24(1)                            # 4-byte Folded Spill
	li 8, -96
	li 9, -80
	stfs 1, 48(1)
	li 10, -64
	li 11, -48
	li 12, -32
	li 0, -16
	lvx128 2, 0, 5
	li 24, -92
	li 23, -68
	li 22, -72
	li 21, -76
	vspltw128 2, 2, 0
	stw 26, 20(1)                           # 4-byte Folded Spill
	li 20, -52
	li 19, -56
	li 18, -60
	li 17, -36
	li 16, -40
	li 15, -44
	li 14, -20
	li 31, -24
	li 4, -28
	li 3, -4
	li 30, -8
	li 29, -12
	li 5, 12
	li 28, 8
	li 27, 4
.Ll3v_BB2_4:                                # %vector.body
                                        # =>This Inner Loop Header: Depth=1
	lvx128 4, 6, 7
	li 25, 15
	addi 26, 26, -32
	cmplwi	26, 0
	lvx128 5, 6, 8
	lvsl 3, 0, 6
	vperm128 4, 4, 5, 3
	vmulfp128 4, 2, 4
	lvsr 7, 6, 7
	vperm128 4, 4, 4, 7
	lvx128 0, 6, 9
	lvx128 1, 6, 10
	lvx128 6, 6, 11
	lvx128 11, 6, 12
	lvx128 8, 6, 0
	lvx128 9, 0, 6
	lvx128 10, 6, 25
	li 25, -100
	stvewx 4, 6, 25
	li 25, -104
	stvewx 4, 6, 25
	li 25, -108
	stvewx 4, 6, 25
	li 25, -84
	stvewx 4, 6, 7
	vperm128 4, 5, 0, 3
	vmulfp128 4, 2, 4
	lvsr 5, 6, 8
	vperm128 4, 4, 4, 5
	stvewx 4, 6, 25
	li 25, -88
	stvewx 4, 6, 25
	stvewx 4, 6, 24
	stvewx 4, 6, 8
	vperm128 4, 0, 1, 3
	vmulfp128 4, 2, 4
	lvsr 5, 6, 9
	vperm128 4, 4, 4, 5
	stvewx 4, 6, 23
	stvewx 4, 6, 22
	stvewx 4, 6, 21
	stvewx 4, 6, 9
	vperm128 4, 1, 6, 3
	vmulfp128 4, 2, 4
	lvsr 5, 6, 10
	vperm128 4, 4, 4, 5
	stvewx 4, 6, 20
	stvewx 4, 6, 19
	stvewx 4, 6, 18
	stvewx 4, 6, 10
	vperm128 4, 6, 11, 3
	vmulfp128 4, 2, 4
	lvsr 5, 6, 11
	vperm128 4, 4, 4, 5
	stvewx 4, 6, 17
	stvewx 4, 6, 16
	stvewx 4, 6, 15
	stvewx 4, 6, 11
	vperm128 4, 11, 8, 3
	vmulfp128 4, 2, 4
	lvsr 5, 6, 12
	vperm128 4, 4, 4, 5
	stvewx 4, 6, 14
	stvewx 4, 6, 31
	stvewx 4, 6, 4
	stvewx 4, 6, 12
	vperm128 4, 8, 9, 3
	vmulfp128 4, 2, 4
	lvsr 5, 6, 0
	vperm128 3, 9, 10, 3
	vperm128 4, 4, 4, 5
	stvewx 4, 6, 3
	vmulfp128 3, 2, 3
	stvewx 4, 6, 30
	stvewx 4, 6, 29
	stvewx 4, 6, 0
	lvsr 4, 0, 6
	vperm128 3, 3, 3, 4
	stvewx 3, 6, 5
	stvewx 3, 6, 28
	stvewx 3, 6, 27
	stvewx 3, 0, 6
	addi 6, 6, 128
	bne	0, .Ll3v_BB2_4
# %bb.5:                                # %middle.block
	lwz 5, 28(1)                            # 4-byte Folded Reload
	lwz 11, 20(1)                           # 4-byte Folded Reload
	lwz 3, 24(1)                            # 4-byte Folded Reload
	cmplw	5, 11
	beq	0, .Ll3v_BB2_12
# %bb.6:                                # %vec.epilog.iter.check
	andi. 4, 5, 28
	beq	0, .Ll3v_BB2_10
.Ll3v_BB2_7:                                # %vec.epilog.ph
	stfs 1, 32(1)
	addi 6, 1, 32
	mr	7, 11
	rlwinm 11, 5, 0, 1, 29
	slwi 4, 7, 2
	lvx128 2, 0, 6
	sub	10, 7, 11
	add 4, 3, 4
	li 9, 15
	li 6, 12
	vspltw128 2, 2, 0
	li 7, 8
	li 8, 4
.Ll3v_BB2_8:                                # %vec.epilog.vector.body
                                        # =>This Inner Loop Header: Depth=1
	lvx128 3, 4, 9
	addi 10, 10, 4
	cmplwi	10, 0
	lvx128 4, 0, 4
	lvsl 5, 0, 4
	vperm128 3, 4, 3, 5
	vmulfp128 3, 2, 3
	lvsr 4, 0, 4
	vperm128 3, 3, 3, 4
	stvewx 3, 4, 6
	stvewx 3, 4, 7
	stvewx 3, 4, 8
	stvewx 3, 0, 4
	addi 4, 4, 16
	bne	0, .Ll3v_BB2_8
# %bb.9:                                # %vec.epilog.middle.block
	cmplw	5, 11
	beq	0, .Ll3v_BB2_12
.Ll3v_BB2_10:                               # %for.body.preheader
	slwi 4, 11, 2
	add 4, 4, 3
	sub	3, 5, 11
	addi 4, 4, -4
.Ll3v_BB2_11:                               # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lfsu 0, 4(4)
	addi 3, 3, -1
	cmplwi	3, 0
	fmuls 0, 1, 0
	stfs 0, 0(4)
	bne	0, .Ll3v_BB2_11
.Ll3v_BB2_12:
	ld 31, 208(1)                           # 8-byte Folded Reload
	ld 30, 200(1)                           # 8-byte Folded Reload
	ld 29, 192(1)                           # 8-byte Folded Reload
	ld 28, 184(1)                           # 8-byte Folded Reload
	ld 27, 176(1)                           # 8-byte Folded Reload
	ld 26, 168(1)                           # 8-byte Folded Reload
	ld 25, 160(1)                           # 8-byte Folded Reload
	ld 24, 152(1)                           # 8-byte Folded Reload
	ld 23, 144(1)                           # 8-byte Folded Reload
	ld 22, 136(1)                           # 8-byte Folded Reload
	ld 21, 128(1)                           # 8-byte Folded Reload
	ld 20, 120(1)                           # 8-byte Folded Reload
	ld 19, 112(1)                           # 8-byte Folded Reload
	ld 18, 104(1)                           # 8-byte Folded Reload
	ld 17, 96(1)                            # 8-byte Folded Reload
	ld 16, 88(1)                            # 8-byte Folded Reload
	ld 15, 80(1)                            # 8-byte Folded Reload
	ld 14, 72(1)                            # 8-byte Folded Reload
	addi 1, 1, 224
	blr
.Ll3v_func_end2:
	.size	l3v_k_scale_inplace, .Ll3v_func_end2-.Ll3v_func_begin2
                                        # -- End function
	.section	.text.l3v_k_add_f32_r,"ax",@progbits
	.globl	l3v_k_add_f32_r                 # -- Begin function l3v_k_add_f32_r
	.p2align	2
	.type	l3v_k_add_f32_r,@function
l3v_k_add_f32_r:                        # @l3v_k_add_f32_r
.Ll3v_func_begin3:
# %bb.0:                                # %entry
	cmpwi	6, 1
	bltlr	0
# %bb.1:                                # %iter.check
	stwu 1, -176(1)
	cmplwi	6, 4
	li 0, 0
	std 14, 24(1)                           # 8-byte Folded Spill
	std 15, 32(1)                           # 8-byte Folded Spill
	std 16, 40(1)                           # 8-byte Folded Spill
	std 17, 48(1)                           # 8-byte Folded Spill
	std 18, 56(1)                           # 8-byte Folded Spill
	std 19, 64(1)                           # 8-byte Folded Spill
	std 20, 72(1)                           # 8-byte Folded Spill
	std 21, 80(1)                           # 8-byte Folded Spill
	std 22, 88(1)                           # 8-byte Folded Spill
	std 23, 96(1)                           # 8-byte Folded Spill
	std 24, 104(1)                          # 8-byte Folded Spill
	std 25, 112(1)                          # 8-byte Folded Spill
	std 26, 120(1)                          # 8-byte Folded Spill
	std 27, 128(1)                          # 8-byte Folded Spill
	std 28, 136(1)                          # 8-byte Folded Spill
	std 29, 144(1)                          # 8-byte Folded Spill
	std 30, 152(1)                          # 8-byte Folded Spill
	std 31, 160(1)                          # 8-byte Folded Spill
	blt	0, .Ll3v_BB3_10
# %bb.2:                                # %vector.main.loop.iter.check
	cmplwi	6, 32
	blt	0, .Ll3v_BB3_7
# %bb.3:                                # %vector.ph
	rlwinm 19, 6, 0, 1, 26
	li 8, 0
	stw 19, 20(1)                           # 4-byte Folded Spill
	li 9, 16
	li 10, 32
	li 11, 48
	li 12, 64
	li 0, 80
	li 30, 96
	li 29, 112
	li 28, 127
	li 16, 52
	li 15, 76
	li 14, 72
	li 31, 68
	li 7, 92
	li 27, 88
	li 26, 84
	li 25, 108
	li 24, 104
	li 23, 100
	li 22, 124
	li 21, 120
	li 20, 116
.Ll3v_BB3_4:                                # %vector.body
                                        # =>This Inner Loop Header: Depth=1
	add 18, 4, 8
	lvx128 29, 4, 8
	li 17, 28
	addi 19, 19, -32
	lvx128 28, 18, 9
	cmplwi	19, 0
	lvx128 0, 18, 10
	lvx128 1, 18, 11
	lvx128 6, 18, 12
	lvx128 7, 18, 0
	lvx128 8, 18, 30
	lvx128 9, 18, 29
	lvx128 10, 18, 28
	add 18, 5, 8
	lvx128 11, 5, 8
	lvx128 12, 18, 9
	lvx128 13, 18, 10
	lvsl 2, 4, 8
	vperm128 19, 28, 0, 2
	lvsl 3, 5, 8
	vperm128 30, 12, 13, 3
	vaddfp128 19, 19, 30
	lvx128 14, 18, 11
	vperm128 0, 0, 1, 2
	lvx128 15, 18, 12
	lvx128 16, 18, 0
	lvx128 17, 18, 30
	lvx128 18, 18, 29
	lvx128 31, 18, 28
	add 18, 3, 8
	lvsr 4, 18, 9
	vperm128 4, 19, 19, 4
	stvewx 4, 18, 17
	li 17, 24
	stvewx 4, 18, 17
	li 17, 20
	stvewx 4, 18, 17
	li 17, 44
	stvewx 4, 18, 9
	vperm128 4, 13, 14, 3
	vaddfp128 4, 0, 4
	lvsr 5, 18, 10
	vperm128 4, 4, 4, 5
	stvewx 4, 18, 17
	li 17, 40
	vperm128 5, 1, 6, 2
	stvewx 4, 18, 17
	li 17, 36
	stvewx 4, 18, 17
	li 17, 60
	stvewx 4, 18, 10
	vperm128 4, 14, 15, 3
	vaddfp128 4, 5, 4
	lvsr 0, 18, 11
	vperm128 5, 6, 7, 2
	vperm128 4, 4, 4, 0
	lvsr 0, 18, 12
	stvewx 4, 18, 17
	li 17, 56
	stvewx 4, 18, 17
	li 17, 12
	stvewx 4, 18, 16
	stvewx 4, 18, 11
	vperm128 4, 15, 16, 3
	vaddfp128 4, 5, 4
	vperm128 4, 4, 4, 0
	lvsr 0, 18, 0
	stvewx 4, 18, 15
	vperm128 5, 7, 8, 2
	stvewx 4, 18, 14
	stvewx 4, 18, 31
	stvewx 4, 18, 12
	vperm128 4, 16, 17, 3
	vaddfp128 4, 5, 4
	vperm128 4, 4, 4, 0
	lvsr 0, 18, 30
	stvewx 4, 18, 7
	vperm128 5, 8, 9, 2
	stvewx 4, 18, 27
	stvewx 4, 18, 26
	stvewx 4, 18, 0
	vperm128 4, 17, 18, 3
	vaddfp128 4, 5, 4
	vperm128 4, 4, 4, 0
	lvsr 0, 18, 29
	stvewx 4, 18, 25
	vperm128 5, 9, 10, 2
	stvewx 4, 18, 24
	vperm128 2, 29, 28, 2
	stvewx 4, 18, 23
	stvewx 4, 18, 30
	vperm128 4, 18, 31, 3
	vaddfp128 4, 5, 4
	vperm128 4, 4, 4, 0
	stvewx 4, 18, 22
	vperm128 3, 11, 12, 3
	stvewx 4, 18, 21
	vaddfp128 2, 2, 3
	stvewx 4, 18, 20
	stvewx 4, 18, 29
	lvsr 4, 3, 8
	vperm128 2, 2, 2, 4
	stvewx 2, 18, 17
	li 17, 8
	stvewx 2, 18, 17
	li 17, 4
	stvewx 2, 3, 8
	addi 8, 8, 128
	stvewx 2, 18, 17
	bne	0, .Ll3v_BB3_4
# %bb.5:                                # %middle.block
	lwz 0, 20(1)                            # 4-byte Folded Reload
	cmplw	6, 0
	beq	0, .Ll3v_BB3_12
# %bb.6:                                # %vec.epilog.iter.check
	andi. 7, 6, 28
	beq	0, .Ll3v_BB3_10
.Ll3v_BB3_7:                                # %vec.epilog.ph
	mr	10, 0
	rlwinm 0, 6, 0, 1, 29
	li 7, 15
	li 8, 12
	sub	9, 10, 0
	slwi 10, 10, 2
	li 11, 8
	li 12, 4
.Ll3v_BB3_8:                                # %vec.epilog.vector.body
                                        # =>This Inner Loop Header: Depth=1
	add 30, 4, 10
	lvx128 3, 4, 10
	addi 9, 9, 4
	cmplwi	9, 0
	lvx128 4, 30, 7
	add 30, 5, 10
	lvx128 5, 5, 10
	lvx128 0, 30, 7
	add 30, 3, 10
	lvsl 2, 4, 10
	vperm128 2, 3, 4, 2
	lvsl 3, 5, 10
	vperm128 3, 5, 0, 3
	vaddfp128 2, 2, 3
	lvsr 4, 3, 10
	vperm128 2, 2, 2, 4
	stvewx 2, 3, 10
	addi 10, 10, 16
	stvewx 2, 30, 8
	stvewx 2, 30, 11
	stvewx 2, 30, 12
	bne	0, .Ll3v_BB3_8
# %bb.9:                                # %vec.epilog.middle.block
	cmplw	6, 0
	beq	0, .Ll3v_BB3_12
.Ll3v_BB3_10:                               # %for.body.preheader
	slwi 7, 0, 2
	sub	6, 6, 0
	addi 7, 7, -4
	add 4, 4, 7
	add 5, 5, 7
	add 3, 3, 7
.Ll3v_BB3_11:                               # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lfsu 0, 4(4)
	addi 6, 6, -1
	cmplwi	6, 0
	lfsu 1, 4(5)
	fadds 0, 0, 1
	stfsu 0, 4(3)
	bne	0, .Ll3v_BB3_11
.Ll3v_BB3_12:
	ld 31, 160(1)                           # 8-byte Folded Reload
	ld 30, 152(1)                           # 8-byte Folded Reload
	ld 29, 144(1)                           # 8-byte Folded Reload
	ld 28, 136(1)                           # 8-byte Folded Reload
	ld 27, 128(1)                           # 8-byte Folded Reload
	ld 26, 120(1)                           # 8-byte Folded Reload
	ld 25, 112(1)                           # 8-byte Folded Reload
	ld 24, 104(1)                           # 8-byte Folded Reload
	ld 23, 96(1)                            # 8-byte Folded Reload
	ld 22, 88(1)                            # 8-byte Folded Reload
	ld 21, 80(1)                            # 8-byte Folded Reload
	ld 20, 72(1)                            # 8-byte Folded Reload
	ld 19, 64(1)                            # 8-byte Folded Reload
	ld 18, 56(1)                            # 8-byte Folded Reload
	ld 17, 48(1)                            # 8-byte Folded Reload
	ld 16, 40(1)                            # 8-byte Folded Reload
	ld 15, 32(1)                            # 8-byte Folded Reload
	ld 14, 24(1)                            # 8-byte Folded Reload
	addi 1, 1, 176
	blr
.Ll3v_func_end3:
	.size	l3v_k_add_f32_r, .Ll3v_func_end3-.Ll3v_func_begin3
                                        # -- End function
	.section	.text.l3v_k_mul_f32_r,"ax",@progbits
	.globl	l3v_k_mul_f32_r                 # -- Begin function l3v_k_mul_f32_r
	.p2align	2
	.type	l3v_k_mul_f32_r,@function
l3v_k_mul_f32_r:                        # @l3v_k_mul_f32_r
.Ll3v_func_begin4:
# %bb.0:                                # %entry
	cmpwi	6, 1
	bltlr	0
# %bb.1:                                # %iter.check
	stwu 1, -176(1)
	cmplwi	6, 4
	li 0, 0
	std 14, 24(1)                           # 8-byte Folded Spill
	std 15, 32(1)                           # 8-byte Folded Spill
	std 16, 40(1)                           # 8-byte Folded Spill
	std 17, 48(1)                           # 8-byte Folded Spill
	std 18, 56(1)                           # 8-byte Folded Spill
	std 19, 64(1)                           # 8-byte Folded Spill
	std 20, 72(1)                           # 8-byte Folded Spill
	std 21, 80(1)                           # 8-byte Folded Spill
	std 22, 88(1)                           # 8-byte Folded Spill
	std 23, 96(1)                           # 8-byte Folded Spill
	std 24, 104(1)                          # 8-byte Folded Spill
	std 25, 112(1)                          # 8-byte Folded Spill
	std 26, 120(1)                          # 8-byte Folded Spill
	std 27, 128(1)                          # 8-byte Folded Spill
	std 28, 136(1)                          # 8-byte Folded Spill
	std 29, 144(1)                          # 8-byte Folded Spill
	std 30, 152(1)                          # 8-byte Folded Spill
	std 31, 160(1)                          # 8-byte Folded Spill
	blt	0, .Ll3v_BB4_10
# %bb.2:                                # %vector.main.loop.iter.check
	cmplwi	6, 32
	blt	0, .Ll3v_BB4_7
# %bb.3:                                # %vector.ph
	rlwinm 19, 6, 0, 1, 26
	li 8, 0
	stw 19, 20(1)                           # 4-byte Folded Spill
	li 9, 16
	li 10, 32
	li 11, 48
	li 12, 64
	li 0, 80
	li 30, 96
	li 29, 112
	li 28, 127
	li 16, 52
	li 15, 76
	li 14, 72
	li 31, 68
	li 7, 92
	li 27, 88
	li 26, 84
	li 25, 108
	li 24, 104
	li 23, 100
	li 22, 124
	li 21, 120
	li 20, 116
.Ll3v_BB4_4:                                # %vector.body
                                        # =>This Inner Loop Header: Depth=1
	add 18, 4, 8
	lvx128 29, 4, 8
	li 17, 28
	addi 19, 19, -32
	lvx128 28, 18, 9
	cmplwi	19, 0
	lvx128 0, 18, 10
	lvx128 1, 18, 11
	lvx128 6, 18, 12
	lvx128 7, 18, 0
	lvx128 8, 18, 30
	lvx128 9, 18, 29
	lvx128 10, 18, 28
	add 18, 5, 8
	lvx128 11, 5, 8
	lvx128 12, 18, 9
	lvx128 13, 18, 10
	lvsl 2, 4, 8
	vperm128 19, 28, 0, 2
	lvsl 3, 5, 8
	vperm128 30, 12, 13, 3
	vmulfp128 19, 19, 30
	lvx128 14, 18, 11
	vperm128 0, 0, 1, 2
	lvx128 15, 18, 12
	lvx128 16, 18, 0
	lvx128 17, 18, 30
	lvx128 18, 18, 29
	lvx128 31, 18, 28
	add 18, 3, 8
	lvsr 4, 18, 9
	vperm128 4, 19, 19, 4
	stvewx 4, 18, 17
	li 17, 24
	stvewx 4, 18, 17
	li 17, 20
	stvewx 4, 18, 17
	li 17, 44
	stvewx 4, 18, 9
	vperm128 4, 13, 14, 3
	vmulfp128 4, 0, 4
	lvsr 5, 18, 10
	vperm128 4, 4, 4, 5
	stvewx 4, 18, 17
	li 17, 40
	vperm128 5, 1, 6, 2
	stvewx 4, 18, 17
	li 17, 36
	stvewx 4, 18, 17
	li 17, 60
	stvewx 4, 18, 10
	vperm128 4, 14, 15, 3
	vmulfp128 4, 5, 4
	lvsr 0, 18, 11
	vperm128 5, 6, 7, 2
	vperm128 4, 4, 4, 0
	lvsr 0, 18, 12
	stvewx 4, 18, 17
	li 17, 56
	stvewx 4, 18, 17
	li 17, 12
	stvewx 4, 18, 16
	stvewx 4, 18, 11
	vperm128 4, 15, 16, 3
	vmulfp128 4, 5, 4
	vperm128 4, 4, 4, 0
	lvsr 0, 18, 0
	stvewx 4, 18, 15
	vperm128 5, 7, 8, 2
	stvewx 4, 18, 14
	stvewx 4, 18, 31
	stvewx 4, 18, 12
	vperm128 4, 16, 17, 3
	vmulfp128 4, 5, 4
	vperm128 4, 4, 4, 0
	lvsr 0, 18, 30
	stvewx 4, 18, 7
	vperm128 5, 8, 9, 2
	stvewx 4, 18, 27
	stvewx 4, 18, 26
	stvewx 4, 18, 0
	vperm128 4, 17, 18, 3
	vmulfp128 4, 5, 4
	vperm128 4, 4, 4, 0
	lvsr 0, 18, 29
	stvewx 4, 18, 25
	vperm128 5, 9, 10, 2
	stvewx 4, 18, 24
	vperm128 2, 29, 28, 2
	stvewx 4, 18, 23
	stvewx 4, 18, 30
	vperm128 4, 18, 31, 3
	vmulfp128 4, 5, 4
	vperm128 4, 4, 4, 0
	stvewx 4, 18, 22
	vperm128 3, 11, 12, 3
	stvewx 4, 18, 21
	vmulfp128 2, 2, 3
	stvewx 4, 18, 20
	stvewx 4, 18, 29
	lvsr 4, 3, 8
	vperm128 2, 2, 2, 4
	stvewx 2, 18, 17
	li 17, 8
	stvewx 2, 18, 17
	li 17, 4
	stvewx 2, 3, 8
	addi 8, 8, 128
	stvewx 2, 18, 17
	bne	0, .Ll3v_BB4_4
# %bb.5:                                # %middle.block
	lwz 0, 20(1)                            # 4-byte Folded Reload
	cmplw	6, 0
	beq	0, .Ll3v_BB4_12
# %bb.6:                                # %vec.epilog.iter.check
	andi. 7, 6, 28
	beq	0, .Ll3v_BB4_10
.Ll3v_BB4_7:                                # %vec.epilog.ph
	mr	10, 0
	rlwinm 0, 6, 0, 1, 29
	li 7, 15
	li 8, 12
	sub	9, 10, 0
	slwi 10, 10, 2
	li 11, 8
	li 12, 4
.Ll3v_BB4_8:                                # %vec.epilog.vector.body
                                        # =>This Inner Loop Header: Depth=1
	add 30, 4, 10
	lvx128 3, 4, 10
	addi 9, 9, 4
	cmplwi	9, 0
	lvx128 4, 30, 7
	add 30, 5, 10
	lvx128 5, 5, 10
	lvx128 0, 30, 7
	add 30, 3, 10
	lvsl 2, 4, 10
	vperm128 2, 3, 4, 2
	lvsl 3, 5, 10
	vperm128 3, 5, 0, 3
	vmulfp128 2, 2, 3
	lvsr 4, 3, 10
	vperm128 2, 2, 2, 4
	stvewx 2, 3, 10
	addi 10, 10, 16
	stvewx 2, 30, 8
	stvewx 2, 30, 11
	stvewx 2, 30, 12
	bne	0, .Ll3v_BB4_8
# %bb.9:                                # %vec.epilog.middle.block
	cmplw	6, 0
	beq	0, .Ll3v_BB4_12
.Ll3v_BB4_10:                               # %for.body.preheader
	slwi 7, 0, 2
	sub	6, 6, 0
	addi 7, 7, -4
	add 4, 4, 7
	add 5, 5, 7
	add 3, 3, 7
.Ll3v_BB4_11:                               # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lfsu 0, 4(4)
	addi 6, 6, -1
	cmplwi	6, 0
	lfsu 1, 4(5)
	fmuls 0, 0, 1
	stfsu 0, 4(3)
	bne	0, .Ll3v_BB4_11
.Ll3v_BB4_12:
	ld 31, 160(1)                           # 8-byte Folded Reload
	ld 30, 152(1)                           # 8-byte Folded Reload
	ld 29, 144(1)                           # 8-byte Folded Reload
	ld 28, 136(1)                           # 8-byte Folded Reload
	ld 27, 128(1)                           # 8-byte Folded Reload
	ld 26, 120(1)                           # 8-byte Folded Reload
	ld 25, 112(1)                           # 8-byte Folded Reload
	ld 24, 104(1)                           # 8-byte Folded Reload
	ld 23, 96(1)                            # 8-byte Folded Reload
	ld 22, 88(1)                            # 8-byte Folded Reload
	ld 21, 80(1)                            # 8-byte Folded Reload
	ld 20, 72(1)                            # 8-byte Folded Reload
	ld 19, 64(1)                            # 8-byte Folded Reload
	ld 18, 56(1)                            # 8-byte Folded Reload
	ld 17, 48(1)                            # 8-byte Folded Reload
	ld 16, 40(1)                            # 8-byte Folded Reload
	ld 15, 32(1)                            # 8-byte Folded Reload
	ld 14, 24(1)                            # 8-byte Folded Reload
	addi 1, 1, 176
	blr
.Ll3v_func_end4:
	.size	l3v_k_mul_f32_r, .Ll3v_func_end4-.Ll3v_func_begin4
                                        # -- End function
	.section	.rodata.cst4,"aM",@progbits,4
	.p2align	2, 0x0                          # -- Begin function l3v_k_sum_f32
.Ll3v_CPI5_0:
	.long	0x00000000                      # float 0
	.section	.text.l3v_k_sum_f32,"ax",@progbits
	.globl	l3v_k_sum_f32
	.p2align	2
	.type	l3v_k_sum_f32,@function
l3v_k_sum_f32:                          # @l3v_k_sum_f32
.Ll3v_func_begin5:
# %bb.0:                                # %entry
	cmpwi	4, 1
	lis 5, .Ll3v_CPI5_0@ha
	blt	0, .Ll3v_BB5_4
# %bb.1:                                # %for.body.preheader
	lfs 1, .Ll3v_CPI5_0@l(5)
	addi 3, 3, -4
.Ll3v_BB5_2:                                # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lfsu 0, 4(3)
	addi 4, 4, -1
	cmplwi	4, 0
	fadds 1, 1, 0
	bne	0, .Ll3v_BB5_2
# %bb.3:                                # %for.end
	blr
.Ll3v_BB5_4:
	lfs 1, .Ll3v_CPI5_0@l(5)
	blr
.Ll3v_func_end5:
	.size	l3v_k_sum_f32, .Ll3v_func_end5-.Ll3v_func_begin5
                                        # -- End function
	.section	.rodata.cst4,"aM",@progbits,4
	.p2align	2, 0x0                          # -- Begin function l3v_k_dot_f32
.Ll3v_CPI6_0:
	.long	0x00000000                      # float 0
	.section	.text.l3v_k_dot_f32,"ax",@progbits
	.globl	l3v_k_dot_f32
	.p2align	2
	.type	l3v_k_dot_f32,@function
l3v_k_dot_f32:                          # @l3v_k_dot_f32
.Ll3v_func_begin6:
# %bb.0:                                # %entry
	cmpwi	5, 1
	lis 6, .Ll3v_CPI6_0@ha
	blt	0, .Ll3v_BB6_4
# %bb.1:                                # %for.body.preheader
	lfs 1, .Ll3v_CPI6_0@l(6)
	addi 3, 3, -4
	addi 4, 4, -4
.Ll3v_BB6_2:                                # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lfsu 0, 4(4)
	addi 5, 5, -1
	cmplwi	5, 0
	lfsu 2, 4(3)
	fmadds 1, 2, 0, 1
	bne	0, .Ll3v_BB6_2
# %bb.3:                                # %for.end
	blr
.Ll3v_BB6_4:
	lfs 1, .Ll3v_CPI6_0@l(6)
	blr
.Ll3v_func_end6:
	.size	l3v_k_dot_f32, .Ll3v_func_end6-.Ll3v_func_begin6
                                        # -- End function
	.section	.rodata.cst16,"aM",@progbits,16
	.p2align	4, 0x0                          # -- Begin function l3v_k_sum_i32
.Ll3v_CPI7_0:
	.byte	0                               # 0x0
	.byte	1                               # 0x1
	.byte	2                               # 0x2
	.byte	3                               # 0x3
	.byte	16                              # 0x10
	.byte	17                              # 0x11
	.byte	18                              # 0x12
	.byte	19                              # 0x13
	.byte	16                              # 0x10
	.byte	17                              # 0x11
	.byte	18                              # 0x12
	.byte	19                              # 0x13
	.byte	16                              # 0x10
	.byte	17                              # 0x11
	.byte	18                              # 0x12
	.byte	19                              # 0x13
	.section	.text.l3v_k_sum_i32,"ax",@progbits
	.globl	l3v_k_sum_i32
	.p2align	2
	.type	l3v_k_sum_i32,@function
l3v_k_sum_i32:                          # @l3v_k_sum_i32
.Ll3v_func_begin7:
# %bb.0:                                # %entry
	cmpwi	4, 1
	blt	0, .Ll3v_BB7_13
# %bb.1:                                # %iter.check
	stwu 1, -96(1)
	cmplwi	4, 4
	li 5, 0
	std 29, 64(1)                           # 8-byte Folded Spill
	li 6, 0
	std 30, 72(1)                           # 8-byte Folded Spill
	blt	0, .Ll3v_BB7_10
# %bb.2:                                # %vector.main.loop.iter.check
	cmplwi	4, 32
	li 5, 0
	blt	0, .Ll3v_BB7_7
# %bb.3:                                # %vector.ph
	rlwinm 6, 4, 0, 1, 26
	vspltisw128 8, 0
	addi 5, 3, 112
	li 7, -112
	li 8, -96
	vspltisw128 3, 0
	li 9, -80
	li 10, -64
	vspltisw128 4, 0
	li 11, -48
	li 12, -32
	li 0, -16
	li 30, 15
	vspltisw128 5, 0
	mr	29, 6
	vspltisw128 0, 0
	vspltisw128 1, 0
	vspltisw128 6, 0
	vspltisw128 7, 0
.Ll3v_BB7_4:                                # %vector.body
                                        # =>This Inner Loop Header: Depth=1
	lvx128 9, 5, 7
	addi 29, 29, -32
	cmplwi	29, 0
	lvx128 10, 5, 8
	lvx128 11, 5, 9
	lvx128 12, 5, 10
	lvx128 13, 5, 11
	lvsl 2, 0, 5
	vperm128 9, 9, 10, 2
	vadduwm 8, 9, 8
	lvx128 9, 5, 12
	vperm128 10, 10, 11, 2
	vadduwm 3, 10, 3
	lvx128 10, 5, 0
	vperm128 11, 11, 12, 2
	vadduwm 4, 11, 4
	lvx128 11, 0, 5
	vperm128 12, 12, 13, 2
	vadduwm 5, 12, 5
	lvx128 12, 5, 30
	addi 5, 5, 128
	vperm128 13, 13, 9, 2
	vadduwm 0, 13, 0
	vperm128 9, 9, 10, 2
	vadduwm 1, 9, 1
	vperm128 10, 10, 11, 2
	vadduwm 6, 10, 6
	vperm128 2, 11, 12, 2
	vadduwm 7, 2, 7
	bne	0, .Ll3v_BB7_4
# %bb.5:                                # %middle.block
	vadduwm 2, 3, 8
	addi 5, 1, 48
	cmplw	4, 6
	vadduwm 2, 4, 2
	vadduwm 2, 5, 2
	vadduwm 2, 0, 2
	vadduwm 2, 1, 2
	vadduwm 2, 6, 2
	vadduwm 2, 7, 2
	vsldoi128 3, 2, 2, 8
	vadduwm 2, 2, 3
	vsldoi128 3, 2, 2, 4
	vadduwm 2, 2, 3
	stvx128 2, 0, 5
	lwz 5, 48(1)
	beq	0, .Ll3v_BB7_12
# %bb.6:                                # %vec.epilog.iter.check
	andi. 7, 4, 28
	beq	0, .Ll3v_BB7_10
.Ll3v_BB7_7:                                # %vec.epilog.ph
	sldi 5, 5, 32
	lis 7, .Ll3v_CPI7_0@ha
	vspltisw128 4, 0
	std 5, 32(1)
	li 8, 15
	std 5, 40(1)
	li 5, .Ll3v_CPI7_0@l
	lvx128 2, 7, 5
	addi 5, 1, 32
	mr	7, 6
	rlwinm 6, 4, 0, 1, 29
	lvx128 3, 0, 5
	sub	5, 7, 6
	slwi 7, 7, 2
	add 7, 3, 7
	vperm128 2, 3, 4, 2
.Ll3v_BB7_8:                                # %vec.epilog.vector.body
                                        # =>This Inner Loop Header: Depth=1
	lvx128 3, 7, 8
	addi 5, 5, 4
	cmplwi	5, 0
	lvx128 4, 0, 7
	lvsl 5, 0, 7
	addi 7, 7, 16
	vperm128 3, 4, 3, 5
	vadduwm 2, 3, 2
	bne	0, .Ll3v_BB7_8
# %bb.9:                                # %vec.epilog.middle.block
	vsldoi128 3, 2, 2, 8
	addi 5, 1, 16
	cmplw	4, 6
	vadduwm 2, 2, 3
	vsldoi128 3, 2, 2, 4
	vadduwm 2, 2, 3
	stvx128 2, 0, 5
	lwz 5, 16(1)
	beq	0, .Ll3v_BB7_12
.Ll3v_BB7_10:                               # %for.body.preheader
	slwi 7, 6, 2
	add 7, 7, 3
	sub	3, 4, 6
	addi 4, 7, -4
.Ll3v_BB7_11:                               # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lwzu 6, 4(4)
	addi 3, 3, -1
	cmplwi	3, 0
	add 5, 6, 5
	bne	0, .Ll3v_BB7_11
.Ll3v_BB7_12:
	ld 30, 72(1)                            # 8-byte Folded Reload
	ld 29, 64(1)                            # 8-byte Folded Reload
	addi 1, 1, 96
	mr	3, 5
	blr
.Ll3v_BB7_13:
	li 5, 0
	mr	3, 5
	blr
.Ll3v_func_end7:
	.size	l3v_k_sum_i32, .Ll3v_func_end7-.Ll3v_func_begin7
                                        # -- End function
	.section	.text.l3v_k_dot_i16,"ax",@progbits
	.globl	l3v_k_dot_i16                   # -- Begin function l3v_k_dot_i16
	.p2align	2
	.type	l3v_k_dot_i16,@function
l3v_k_dot_i16:                          # @l3v_k_dot_i16
.Ll3v_func_begin8:
# %bb.0:                                # %entry
	cmpwi	5, 1
	blt	0, .Ll3v_BB8_8
# %bb.1:                                # %for.body.preheader
	stwu 1, -48(1)
	li 6, 0
	li 7, 0
	std 30, 24(1)                           # 8-byte Folded Spill
	beq	0, .Ll3v_BB8_5
# %bb.2:                                # %vector.ph
	rlwinm 7, 5, 0, 1, 30
	addi 8, 3, -4
	addi 9, 4, -4
	mr	11, 7
	li 10, 0
.Ll3v_BB8_3:                                # %vector.body
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
	bne	0, .Ll3v_BB8_3
# %bb.4:                                # %middle.block
	cmplw	5, 7
	add 6, 10, 6
	beq	0, .Ll3v_BB8_7
.Ll3v_BB8_5:                                # %for.body.preheader11
	slwi 8, 7, 1
	sub	5, 5, 7
	addi 7, 8, -2
	add 3, 3, 7
	add 4, 4, 7
.Ll3v_BB8_6:                                # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lhzu 7, 2(3)
	addi 5, 5, -1
	cmplwi	5, 0
	lhzu 8, 2(4)
	extsh 7, 7
	extsh 8, 8
	mullw 7, 8, 7
	add 6, 7, 6
	bne	0, .Ll3v_BB8_6
.Ll3v_BB8_7:
	ld 30, 24(1)                            # 8-byte Folded Reload
	addi 1, 1, 48
	mr	3, 6
	blr
.Ll3v_BB8_8:
	li 3, 0
	blr
.Ll3v_func_end8:
	.size	l3v_k_dot_i16, .Ll3v_func_end8-.Ll3v_func_begin8
                                        # -- End function
	.section	.text.l3v_k_max_f32,"ax",@progbits
	.globl	l3v_k_max_f32                   # -- Begin function l3v_k_max_f32
	.p2align	2
	.type	l3v_k_max_f32,@function
l3v_k_max_f32:                          # @l3v_k_max_f32
.Ll3v_func_begin9:
# %bb.0:                                # %entry
	lfs 0, 0(3)
	cmpwi	4, 2
	blt	0, .Ll3v_BB9_5
# %bb.1:                                # %for.body.preheader
	addi 4, 4, -1
	b .Ll3v_BB9_3
.Ll3v_BB9_2:                                # %for.body
                                        #   in Loop: Header=BB9_3 Depth=1
	addi 4, 4, -1
	fmr 0, 1
	cmplwi	4, 0
	beqlr	0
.Ll3v_BB9_3:                                # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lfsu 1, 4(3)
	fcmpu 0, 1, 0
	bgt	0, .Ll3v_BB9_2
# %bb.4:                                # %for.body
                                        #   in Loop: Header=BB9_3 Depth=1
	fmr 1, 0
	b .Ll3v_BB9_2
.Ll3v_BB9_5:
	fmr 1, 0
	blr
.Ll3v_func_end9:
	.size	l3v_k_max_f32, .Ll3v_func_end9-.Ll3v_func_begin9
                                        # -- End function
	.section	.text.l3v_k_add_i32,"ax",@progbits
	.globl	l3v_k_add_i32                   # -- Begin function l3v_k_add_i32
	.p2align	2
	.type	l3v_k_add_i32,@function
l3v_k_add_i32:                          # @l3v_k_add_i32
.Ll3v_func_begin10:
# %bb.0:                                # %entry
	cmpwi	6, 1
	bltlr	0
# %bb.1:                                # %iter.check
	stwu 1, -176(1)
	std 18, 56(1)                           # 8-byte Folded Spill
	cmplwi	6, 4
	li 18, 0
	std 14, 24(1)                           # 8-byte Folded Spill
	std 15, 32(1)                           # 8-byte Folded Spill
	std 16, 40(1)                           # 8-byte Folded Spill
	std 17, 48(1)                           # 8-byte Folded Spill
	std 19, 64(1)                           # 8-byte Folded Spill
	std 20, 72(1)                           # 8-byte Folded Spill
	std 21, 80(1)                           # 8-byte Folded Spill
	std 22, 88(1)                           # 8-byte Folded Spill
	std 23, 96(1)                           # 8-byte Folded Spill
	std 24, 104(1)                          # 8-byte Folded Spill
	std 25, 112(1)                          # 8-byte Folded Spill
	std 26, 120(1)                          # 8-byte Folded Spill
	std 27, 128(1)                          # 8-byte Folded Spill
	std 28, 136(1)                          # 8-byte Folded Spill
	std 29, 144(1)                          # 8-byte Folded Spill
	std 30, 152(1)                          # 8-byte Folded Spill
	std 31, 160(1)                          # 8-byte Folded Spill
	bge 0, .Ll3v_BB10_5
.Ll3v_BB10_2:                               # %for.body.preheader
	slwi 7, 18, 2
	sub	6, 6, 18
	addi 7, 7, -4
	add 4, 4, 7
	add 5, 5, 7
	add 3, 3, 7
.Ll3v_BB10_3:                               # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lwzu 7, 4(4)
	addi 6, 6, -1
	cmplwi	6, 0
	lwzu 8, 4(5)
	add 7, 8, 7
	stwu 7, 4(3)
	bne	0, .Ll3v_BB10_3
.Ll3v_BB10_4:
	ld 31, 160(1)                           # 8-byte Folded Reload
	ld 30, 152(1)                           # 8-byte Folded Reload
	ld 29, 144(1)                           # 8-byte Folded Reload
	ld 28, 136(1)                           # 8-byte Folded Reload
	ld 27, 128(1)                           # 8-byte Folded Reload
	ld 26, 120(1)                           # 8-byte Folded Reload
	ld 25, 112(1)                           # 8-byte Folded Reload
	ld 24, 104(1)                           # 8-byte Folded Reload
	ld 23, 96(1)                            # 8-byte Folded Reload
	ld 22, 88(1)                            # 8-byte Folded Reload
	ld 21, 80(1)                            # 8-byte Folded Reload
	ld 20, 72(1)                            # 8-byte Folded Reload
	ld 19, 64(1)                            # 8-byte Folded Reload
	ld 18, 56(1)                            # 8-byte Folded Reload
	ld 17, 48(1)                            # 8-byte Folded Reload
	ld 16, 40(1)                            # 8-byte Folded Reload
	ld 15, 32(1)                            # 8-byte Folded Reload
	ld 14, 24(1)                            # 8-byte Folded Reload
	addi 1, 1, 176
	blr
.Ll3v_BB10_5:                               # %vector.memcheck
	sub	7, 3, 4
	cmplwi	7, 128
	sub	7, 3, 5
	bc 12, 0, .Ll3v_BB10_2
# %bb.6:                                # %vector.memcheck
	cmplwi	7, 128
	bc 12, 0, .Ll3v_BB10_2
# %bb.7:                                # %vector.main.loop.iter.check
	cmplwi	6, 32
	blt	0, .Ll3v_BB10_12
# %bb.8:                                # %vector.ph
	rlwinm 18, 6, 0, 1, 26
	li 8, 0
	li 9, 16
	li 10, 32
	li 11, 48
	li 12, 64
	li 0, 80
	li 30, 96
	li 29, 112
	li 28, 127
	li 15, 76
	li 14, 72
	li 31, 68
	li 27, 92
	li 26, 88
	li 25, 84
	li 24, 108
	li 23, 104
	li 22, 100
	li 21, 124
	li 20, 120
	li 19, 116
	mr	7, 18
.Ll3v_BB10_9:                               # %vector.body
                                        # =>This Inner Loop Header: Depth=1
	add 17, 4, 8
	lvx128 29, 4, 8
	li 16, 28
	addi 7, 7, -32
	lvx128 26, 17, 9
	cmplwi	7, 0
	lvx128 5, 17, 10
	lvx128 6, 17, 11
	lvx128 27, 17, 12
	lvx128 8, 17, 0
	lvx128 9, 17, 30
	lvx128 10, 17, 29
	lvx128 11, 17, 28
	add 17, 5, 8
	lvx128 12, 5, 8
	lvx128 13, 17, 9
	lvx128 14, 17, 10
	lvsl 2, 4, 8
	vperm128 30, 26, 5, 2
	lvsl 3, 5, 8
	vperm128 28, 13, 14, 3
	vadduwm 30, 28, 30
	lvx128 15, 17, 11
	vperm128 5, 5, 6, 2
	lvx128 16, 17, 12
	vperm128 12, 12, 13, 3
	lvx128 17, 17, 0
	lvx128 18, 17, 30
	lvx128 19, 17, 29
	lvx128 31, 17, 28
	add 17, 3, 8
	lvsr 0, 17, 9
	vperm128 0, 30, 30, 0
	stvewx 0, 17, 16
	li 16, 24
	stvewx 0, 17, 16
	li 16, 20
	stvewx 0, 17, 16
	li 16, 44
	stvewx 0, 17, 9
	vperm128 0, 14, 15, 3
	vadduwm 5, 0, 5
	lvsr 7, 17, 10
	vperm128 0, 6, 27, 2
	vperm128 5, 5, 5, 7
	lvsr 4, 3, 8
	vperm128 7, 29, 26, 2
	stvewx 5, 17, 16
	li 16, 40
	vadduwm 7, 12, 7
	vperm128 6, 15, 16, 3
	lvsr 1, 17, 11
	vadduwm 0, 6, 0
	stvewx 5, 17, 16
	li 16, 36
	vperm128 4, 7, 7, 4
	vperm128 7, 27, 8, 2
	stvewx 5, 17, 16
	li 16, 60
	vperm128 12, 16, 17, 3
	stvewx 5, 17, 10
	vadduwm 7, 12, 7
	lvsr 5, 17, 12
	vperm128 0, 0, 0, 1
	stvewx 0, 17, 16
	li 16, 56
	vperm128 5, 7, 7, 5
	vperm128 7, 8, 9, 2
	stvewx 0, 17, 16
	li 16, 52
	vperm128 8, 17, 18, 3
	lvsr 6, 17, 0
	stvewx 5, 17, 15
	stvewx 5, 17, 14
	stvewx 5, 17, 31
	stvewx 5, 17, 12
	vadduwm 5, 8, 7
	vperm128 7, 18, 19, 3
	stvewx 0, 17, 16
	li 16, 12
	vperm128 5, 5, 5, 6
	vperm128 6, 9, 10, 2
	stvewx 0, 17, 11
	vperm128 2, 10, 11, 2
	lvsr 1, 17, 30
	vperm128 3, 19, 31, 3
	lvsr 0, 17, 29
	vadduwm 2, 3, 2
	stvewx 4, 17, 16
	li 16, 8
	vperm128 2, 2, 2, 0
	stvewx 5, 17, 27
	stvewx 5, 17, 26
	stvewx 5, 17, 25
	stvewx 5, 17, 0
	vadduwm 5, 7, 6
	vperm128 5, 5, 5, 1
	stvewx 4, 17, 16
	li 16, 4
	stvewx 4, 3, 8
	addi 8, 8, 128
	stvewx 4, 17, 16
	stvewx 5, 17, 24
	stvewx 5, 17, 23
	stvewx 5, 17, 22
	stvewx 5, 17, 30
	stvewx 2, 17, 21
	stvewx 2, 17, 20
	stvewx 2, 17, 19
	stvewx 2, 17, 29
	bne	0, .Ll3v_BB10_9
# %bb.10:                               # %middle.block
	cmplw	6, 18
	beq	0, .Ll3v_BB10_4
# %bb.11:                               # %vec.epilog.iter.check
	andi. 7, 6, 28
	beq	0, .Ll3v_BB10_2
.Ll3v_BB10_12:                              # %vec.epilog.ph
	mr	10, 18
	rlwinm 18, 6, 0, 1, 29
	li 7, 15
	li 8, 12
	sub	9, 10, 18
	slwi 10, 10, 2
	li 11, 8
	li 12, 4
.Ll3v_BB10_13:                              # %vec.epilog.vector.body
                                        # =>This Inner Loop Header: Depth=1
	add 30, 4, 10
	lvx128 3, 4, 10
	addi 9, 9, 4
	cmplwi	9, 0
	lvx128 4, 30, 7
	add 30, 5, 10
	lvx128 5, 5, 10
	lvx128 0, 30, 7
	add 30, 3, 10
	lvsl 2, 4, 10
	vperm128 2, 3, 4, 2
	lvsl 3, 5, 10
	vperm128 3, 5, 0, 3
	vadduwm 2, 3, 2
	lvsr 4, 3, 10
	vperm128 2, 2, 2, 4
	stvewx 2, 3, 10
	addi 10, 10, 16
	stvewx 2, 30, 8
	stvewx 2, 30, 11
	stvewx 2, 30, 12
	bne	0, .Ll3v_BB10_13
# %bb.14:                               # %vec.epilog.middle.block
	cmplw	6, 18
	bne	0, .Ll3v_BB10_2
	b .Ll3v_BB10_4
.Ll3v_func_end10:
	.size	l3v_k_add_i32, .Ll3v_func_end10-.Ll3v_func_begin10
                                        # -- End function
	.section	.text.l3v_k_add_i32_r,"ax",@progbits
	.globl	l3v_k_add_i32_r                 # -- Begin function l3v_k_add_i32_r
	.p2align	2
	.type	l3v_k_add_i32_r,@function
l3v_k_add_i32_r:                        # @l3v_k_add_i32_r
.Ll3v_func_begin11:
# %bb.0:                                # %entry
	cmpwi	6, 1
	bltlr	0
# %bb.1:                                # %iter.check
	stwu 1, -176(1)
	cmplwi	6, 4
	li 0, 0
	std 14, 24(1)                           # 8-byte Folded Spill
	std 15, 32(1)                           # 8-byte Folded Spill
	std 16, 40(1)                           # 8-byte Folded Spill
	std 17, 48(1)                           # 8-byte Folded Spill
	std 18, 56(1)                           # 8-byte Folded Spill
	std 19, 64(1)                           # 8-byte Folded Spill
	std 20, 72(1)                           # 8-byte Folded Spill
	std 21, 80(1)                           # 8-byte Folded Spill
	std 22, 88(1)                           # 8-byte Folded Spill
	std 23, 96(1)                           # 8-byte Folded Spill
	std 24, 104(1)                          # 8-byte Folded Spill
	std 25, 112(1)                          # 8-byte Folded Spill
	std 26, 120(1)                          # 8-byte Folded Spill
	std 27, 128(1)                          # 8-byte Folded Spill
	std 28, 136(1)                          # 8-byte Folded Spill
	std 29, 144(1)                          # 8-byte Folded Spill
	std 30, 152(1)                          # 8-byte Folded Spill
	std 31, 160(1)                          # 8-byte Folded Spill
	blt	0, .Ll3v_BB11_10
# %bb.2:                                # %vector.main.loop.iter.check
	cmplwi	6, 32
	blt	0, .Ll3v_BB11_7
# %bb.3:                                # %vector.ph
	rlwinm 19, 6, 0, 1, 26
	li 8, 0
	stw 19, 20(1)                           # 4-byte Folded Spill
	li 9, 16
	li 10, 32
	li 11, 48
	li 12, 64
	li 0, 80
	li 30, 96
	li 29, 112
	li 28, 127
	li 16, 52
	li 15, 76
	li 14, 72
	li 31, 68
	li 7, 92
	li 27, 88
	li 26, 84
	li 25, 108
	li 24, 104
	li 23, 100
	li 22, 124
	li 21, 120
	li 20, 116
.Ll3v_BB11_4:                               # %vector.body
                                        # =>This Inner Loop Header: Depth=1
	add 18, 4, 8
	lvx128 29, 4, 8
	li 17, 28
	addi 19, 19, -32
	lvx128 26, 18, 9
	cmplwi	19, 0
	lvx128 5, 18, 10
	lvx128 6, 18, 11
	lvx128 27, 18, 12
	lvx128 8, 18, 0
	lvx128 9, 18, 30
	lvx128 10, 18, 29
	lvx128 11, 18, 28
	add 18, 5, 8
	lvx128 12, 5, 8
	lvx128 13, 18, 9
	lvx128 14, 18, 10
	lvsl 2, 4, 8
	vperm128 30, 26, 5, 2
	lvsl 3, 5, 8
	vperm128 28, 13, 14, 3
	vadduwm 30, 28, 30
	lvx128 15, 18, 11
	vperm128 5, 5, 6, 2
	lvx128 16, 18, 12
	vperm128 12, 12, 13, 3
	lvx128 17, 18, 0
	lvx128 18, 18, 30
	lvx128 19, 18, 29
	lvx128 31, 18, 28
	add 18, 3, 8
	lvsr 0, 18, 9
	vperm128 0, 30, 30, 0
	stvewx 0, 18, 17
	li 17, 24
	stvewx 0, 18, 17
	li 17, 20
	stvewx 0, 18, 17
	li 17, 44
	stvewx 0, 18, 9
	vperm128 0, 14, 15, 3
	vadduwm 5, 0, 5
	lvsr 7, 18, 10
	vperm128 0, 6, 27, 2
	vperm128 5, 5, 5, 7
	lvsr 4, 3, 8
	vperm128 7, 29, 26, 2
	stvewx 5, 18, 17
	li 17, 40
	vadduwm 7, 12, 7
	stvewx 5, 18, 17
	li 17, 36
	vperm128 6, 15, 16, 3
	vadduwm 0, 6, 0
	vperm128 4, 7, 7, 4
	lvsr 1, 18, 11
	vperm128 7, 27, 8, 2
	stvewx 5, 18, 17
	li 17, 60
	vperm128 12, 16, 17, 3
	stvewx 5, 18, 10
	vadduwm 7, 12, 7
	lvsr 5, 18, 12
	vperm128 0, 0, 0, 1
	vperm128 5, 7, 7, 5
	stvewx 0, 18, 17
	li 17, 56
	vperm128 7, 8, 9, 2
	lvsr 6, 18, 0
	vperm128 8, 17, 18, 3
	stvewx 5, 18, 15
	stvewx 5, 18, 14
	stvewx 5, 18, 31
	stvewx 5, 18, 12
	vadduwm 5, 8, 7
	vperm128 7, 18, 19, 3
	stvewx 0, 18, 17
	li 17, 12
	vperm128 5, 5, 5, 6
	vperm128 6, 9, 10, 2
	stvewx 0, 18, 16
	vperm128 2, 10, 11, 2
	stvewx 0, 18, 11
	vperm128 3, 19, 31, 3
	lvsr 1, 18, 30
	vadduwm 2, 3, 2
	lvsr 0, 18, 29
	vperm128 2, 2, 2, 0
	stvewx 4, 18, 17
	li 17, 8
	stvewx 5, 18, 7
	stvewx 5, 18, 27
	stvewx 5, 18, 26
	stvewx 5, 18, 0
	vadduwm 5, 7, 6
	vperm128 5, 5, 5, 1
	stvewx 4, 18, 17
	li 17, 4
	stvewx 4, 3, 8
	addi 8, 8, 128
	stvewx 4, 18, 17
	stvewx 5, 18, 25
	stvewx 5, 18, 24
	stvewx 5, 18, 23
	stvewx 5, 18, 30
	stvewx 2, 18, 22
	stvewx 2, 18, 21
	stvewx 2, 18, 20
	stvewx 2, 18, 29
	bne	0, .Ll3v_BB11_4
# %bb.5:                                # %middle.block
	lwz 0, 20(1)                            # 4-byte Folded Reload
	cmplw	6, 0
	beq	0, .Ll3v_BB11_12
# %bb.6:                                # %vec.epilog.iter.check
	andi. 7, 6, 28
	beq	0, .Ll3v_BB11_10
.Ll3v_BB11_7:                               # %vec.epilog.ph
	mr	10, 0
	rlwinm 0, 6, 0, 1, 29
	li 7, 15
	li 8, 12
	sub	9, 10, 0
	slwi 10, 10, 2
	li 11, 8
	li 12, 4
.Ll3v_BB11_8:                               # %vec.epilog.vector.body
                                        # =>This Inner Loop Header: Depth=1
	add 30, 4, 10
	lvx128 3, 4, 10
	addi 9, 9, 4
	cmplwi	9, 0
	lvx128 4, 30, 7
	add 30, 5, 10
	lvx128 5, 5, 10
	lvx128 0, 30, 7
	add 30, 3, 10
	lvsl 2, 4, 10
	vperm128 2, 3, 4, 2
	lvsl 3, 5, 10
	vperm128 3, 5, 0, 3
	vadduwm 2, 3, 2
	lvsr 4, 3, 10
	vperm128 2, 2, 2, 4
	stvewx 2, 3, 10
	addi 10, 10, 16
	stvewx 2, 30, 8
	stvewx 2, 30, 11
	stvewx 2, 30, 12
	bne	0, .Ll3v_BB11_8
# %bb.9:                                # %vec.epilog.middle.block
	cmplw	6, 0
	beq	0, .Ll3v_BB11_12
.Ll3v_BB11_10:                              # %for.body.preheader
	slwi 7, 0, 2
	sub	6, 6, 0
	addi 7, 7, -4
	add 4, 4, 7
	add 5, 5, 7
	add 3, 3, 7
.Ll3v_BB11_11:                              # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lwzu 7, 4(4)
	addi 6, 6, -1
	cmplwi	6, 0
	lwzu 8, 4(5)
	add 7, 8, 7
	stwu 7, 4(3)
	bne	0, .Ll3v_BB11_11
.Ll3v_BB11_12:
	ld 31, 160(1)                           # 8-byte Folded Reload
	ld 30, 152(1)                           # 8-byte Folded Reload
	ld 29, 144(1)                           # 8-byte Folded Reload
	ld 28, 136(1)                           # 8-byte Folded Reload
	ld 27, 128(1)                           # 8-byte Folded Reload
	ld 26, 120(1)                           # 8-byte Folded Reload
	ld 25, 112(1)                           # 8-byte Folded Reload
	ld 24, 104(1)                           # 8-byte Folded Reload
	ld 23, 96(1)                            # 8-byte Folded Reload
	ld 22, 88(1)                            # 8-byte Folded Reload
	ld 21, 80(1)                            # 8-byte Folded Reload
	ld 20, 72(1)                            # 8-byte Folded Reload
	ld 19, 64(1)                            # 8-byte Folded Reload
	ld 18, 56(1)                            # 8-byte Folded Reload
	ld 17, 48(1)                            # 8-byte Folded Reload
	ld 16, 40(1)                            # 8-byte Folded Reload
	ld 15, 32(1)                            # 8-byte Folded Reload
	ld 14, 24(1)                            # 8-byte Folded Reload
	addi 1, 1, 176
	blr
.Ll3v_func_end11:
	.size	l3v_k_add_i32_r, .Ll3v_func_end11-.Ll3v_func_begin11
                                        # -- End function
	.section	.text.l3v_k_add_i16_r,"ax",@progbits
	.globl	l3v_k_add_i16_r                 # -- Begin function l3v_k_add_i16_r
	.p2align	2
	.type	l3v_k_add_i16_r,@function
l3v_k_add_i16_r:                        # @l3v_k_add_i16_r
.Ll3v_func_begin12:
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
	blt	0, .Ll3v_BB12_5
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
.Ll3v_BB12_3:                               # %vector.body
                                        # =>This Inner Loop Header: Depth=1
	add 26, 4, 8
	lvx128 3, 4, 8
	addi 27, 27, -8
	cmplwi	27, 0
	lvx128 4, 26, 9
	add 26, 5, 8
	lvx128 5, 5, 8
	lvx128 0, 26, 9
	add 26, 3, 8
	lvsl 2, 4, 8
	vperm128 2, 3, 4, 2
	lvsl 3, 5, 8
	vperm128 3, 5, 0, 3
	vadduhm 2, 3, 2
	lvsr 4, 3, 8
	vperm128 2, 2, 2, 4
	stvehx 2, 3, 8
	addi 8, 8, 16
	stvehx 2, 26, 10
	stvehx 2, 26, 11
	stvehx 2, 26, 12
	stvehx 2, 26, 0
	stvehx 2, 26, 30
	stvehx 2, 26, 29
	stvehx 2, 26, 28
	bne	0, .Ll3v_BB12_3
# %bb.4:                                # %middle.block
	cmplw	6, 7
	beq	0, .Ll3v_BB12_7
.Ll3v_BB12_5:                               # %for.body.preheader12
	slwi 8, 7, 1
	sub	6, 6, 7
	addi 8, 8, -2
	add 4, 4, 8
	add 5, 5, 8
	add 3, 3, 8
.Ll3v_BB12_6:                               # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lhzu 7, 2(4)
	addi 6, 6, -1
	cmplwi	6, 0
	lhzu 8, 2(5)
	add 7, 8, 7
	sthu 7, 2(3)
	bne	0, .Ll3v_BB12_6
.Ll3v_BB12_7:
	ld 30, 56(1)                            # 8-byte Folded Reload
	ld 29, 48(1)                            # 8-byte Folded Reload
	ld 28, 40(1)                            # 8-byte Folded Reload
	ld 27, 32(1)                            # 8-byte Folded Reload
	ld 26, 24(1)                            # 8-byte Folded Reload
	addi 1, 1, 80
	blr
.Ll3v_func_end12:
	.size	l3v_k_add_i16_r, .Ll3v_func_end12-.Ll3v_func_begin12
                                        # -- End function
	.section	.text.l3v_k_add_u8_r,"ax",@progbits
	.globl	l3v_k_add_u8_r                  # -- Begin function l3v_k_add_u8_r
	.p2align	2
	.type	l3v_k_add_u8_r,@function
l3v_k_add_u8_r:                         # @l3v_k_add_u8_r
.Ll3v_func_begin13:
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
	blt	0, .Ll3v_BB13_10
# %bb.2:                                # %vector.main.loop.iter.check
	cmplwi	6, 16
	bge 0, .Ll3v_BB13_6
.Ll3v_BB13_3:                               # %vec.epilog.ph
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
.Ll3v_BB13_4:                               # %vec.epilog.vector.body
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
	lvx128 2, 0, 9
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
	lvx128 3, 0, 10
	lvsr 4, 3, 8
	vaddubm 2, 3, 2
	vperm128 2, 2, 2, 4
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
	bne	0, .Ll3v_BB13_4
# %bb.5:                                # %vec.epilog.middle.block
	cmplw	6, 7
	bne	0, .Ll3v_BB13_10
	b .Ll3v_BB13_12
.Ll3v_BB13_6:                               # %vector.ph
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
.Ll3v_BB13_7:                               # %vector.body
                                        # =>This Inner Loop Header: Depth=1
	add 20, 4, 8
	lvx128 3, 4, 8
	lvx128 4, 20, 9
	add 20, 5, 8
	lvx128 5, 5, 8
	lvx128 0, 20, 9
	add 20, 3, 8
	lvsl 2, 4, 8
	vperm128 2, 3, 4, 2
	lvsl 3, 5, 8
	vperm128 3, 5, 0, 3
	vaddubm 2, 3, 2
	lvsr 4, 3, 8
	vperm128 2, 2, 2, 4
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
	bne	0, .Ll3v_BB13_7
# %bb.8:                                # %middle.block
	cmplw	6, 7
	beq	0, .Ll3v_BB13_12
# %bb.9:                                # %vec.epilog.iter.check
	andi. 8, 6, 8
	bne	0, .Ll3v_BB13_3
.Ll3v_BB13_10:                              # %for.body.preheader
	addi 8, 7, -1
	add 4, 4, 8
	add 5, 5, 8
	add 3, 3, 8
.Ll3v_BB13_11:                              # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lbzu 8, 1(4)
	addi 7, 7, 1
	cmplw	6, 7
	lbzu 9, 1(5)
	add 8, 9, 8
	stbu 8, 1(3)
	bne	0, .Ll3v_BB13_11
.Ll3v_BB13_12:
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
.Ll3v_func_end13:
	.size	l3v_k_add_u8_r, .Ll3v_func_end13-.Ll3v_func_begin13
                                        # -- End function
	.section	.text.l3v_k_avg_u8_r,"ax",@progbits
	.globl	l3v_k_avg_u8_r                  # -- Begin function l3v_k_avg_u8_r
	.p2align	2
	.type	l3v_k_avg_u8_r,@function
l3v_k_avg_u8_r:                         # @l3v_k_avg_u8_r
.Ll3v_func_begin14:
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
	blt	0, .Ll3v_BB14_10
# %bb.2:                                # %vector.main.loop.iter.check
	cmplwi	6, 16
	vspltisb 2, 1
	bge 0, .Ll3v_BB14_6
.Ll3v_BB14_3:                               # %vec.epilog.ph
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
.Ll3v_BB14_4:                               # %vec.epilog.vector.body
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
	lvx128 3, 0, 9
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
	lvx128 4, 0, 10
	vor128 5, 3, 4
	vxor128 3, 3, 4
	lvsr 4, 3, 8
	vsrb 3, 3, 2
	vsububm 3, 5, 3
	vperm128 3, 3, 3, 4
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
	bne	0, .Ll3v_BB14_4
# %bb.5:                                # %vec.epilog.middle.block
	cmplw	6, 7
	bne	0, .Ll3v_BB14_10
	b .Ll3v_BB14_12
.Ll3v_BB14_6:                               # %vector.ph
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
.Ll3v_BB14_7:                               # %vector.body
                                        # =>This Inner Loop Header: Depth=1
	add 20, 4, 8
	lvx128 4, 4, 8
	lvx128 5, 20, 9
	add 20, 5, 8
	lvx128 0, 5, 8
	lvx128 1, 20, 9
	add 20, 3, 8
	lvsl 3, 4, 8
	vperm128 3, 4, 5, 3
	lvsl 4, 5, 8
	vperm128 4, 0, 1, 4
	vor128 5, 3, 4
	vxor128 3, 3, 4
	lvsr 4, 3, 8
	vsrb 3, 3, 2
	vsububm 3, 5, 3
	vperm128 3, 3, 3, 4
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
	bne	0, .Ll3v_BB14_7
# %bb.8:                                # %middle.block
	cmplw	6, 7
	beq	0, .Ll3v_BB14_12
# %bb.9:                                # %vec.epilog.iter.check
	andi. 8, 6, 8
	bne	0, .Ll3v_BB14_3
.Ll3v_BB14_10:                              # %for.body.preheader
	addi 8, 7, -1
	add 4, 4, 8
	add 5, 5, 8
	add 3, 3, 8
.Ll3v_BB14_11:                              # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lbzu 8, 1(4)
	addi 7, 7, 1
	cmplw	6, 7
	lbzu 9, 1(5)
	add 8, 8, 9
	addi 8, 8, 1
	srwi 8, 8, 1
	stbu 8, 1(3)
	bne	0, .Ll3v_BB14_11
.Ll3v_BB14_12:
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
.Ll3v_func_end14:
	.size	l3v_k_avg_u8_r, .Ll3v_func_end14-.Ll3v_func_begin14
                                        # -- End function
	.section	.rodata.cst16,"aM",@progbits,16
	.p2align	4, 0x0                          # -- Begin function l3v_k_addsat_u8_r
.Ll3v_CPI15_0:
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
.Ll3v_CPI15_1:
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
	.section	.text.l3v_k_addsat_u8_r,"ax",@progbits
	.globl	l3v_k_addsat_u8_r
	.p2align	2
	.type	l3v_k_addsat_u8_r,@function
l3v_k_addsat_u8_r:                      # @l3v_k_addsat_u8_r
.Ll3v_func_begin15:
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
	blt	0, .Ll3v_BB15_11
# %bb.2:                                # %vector.main.loop.iter.check
	cmplwi	6, 16
	blt	0, .Ll3v_BB15_7
# %bb.3:                                # %vector.ph
	li 22, .Ll3v_CPI15_0@l
	lis 21, .Ll3v_CPI15_0@ha
	vspltisw128 4, -8
	lvx128 3, 21, 22
	vsrw128 4, 4, 4
	rlwinm 7, 6, 0, 1, 27
	li 8, 0
	vspltisw128 2, 0
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
.Ll3v_BB15_4:                               # %vector.body
                                        # =>This Inner Loop Header: Depth=1
	add 20, 4, 8
	lvx128 0, 4, 8
	lvx128 1, 20, 9
	add 20, 5, 8
	lvx128 6, 5, 8
	lvx128 7, 20, 9
	add 20, 3, 8
	lvsl 5, 4, 8
	vperm128 5, 0, 1, 5
	lvsl 0, 5, 8
	vperm128 0, 6, 7, 0
	vperm128 1, 2, 5, 3
	vsldoi128 6, 5, 5, 4
	vsldoi128 7, 5, 5, 12
	vsldoi128 5, 5, 5, 8
	vperm128 8, 2, 0, 3
	vadduwm 1, 8, 1
	vminuw 1, 1, 4
	vsldoi128 9, 0, 0, 4
	vsldoi128 10, 0, 0, 12
	vsldoi128 0, 0, 0, 8
	vperm128 5, 2, 5, 3
	vperm128 0, 2, 0, 3
	vadduwm 5, 0, 5
	vminuw 5, 5, 4
	vperm128 7, 2, 7, 3
	vperm128 8, 2, 10, 3
	vadduwm 7, 8, 7
	vperm128 0, 2, 6, 3
	vminuw 7, 7, 4
	vperm128 6, 2, 9, 3
	vadduwm 0, 6, 0
	vpkuwum128 5, 5, 7
	vminuw 0, 0, 4
	vpkuwum128 0, 1, 0
	vpkuhum128 5, 0, 5
	lvsr 0, 3, 8
	vperm128 5, 5, 5, 0
	stvebx 5, 3, 8
	addi 8, 8, 16
	cmplw	7, 8
	stvebx 5, 20, 9
	stvebx 5, 20, 10
	stvebx 5, 20, 11
	stvebx 5, 20, 12
	stvebx 5, 20, 0
	stvebx 5, 20, 30
	stvebx 5, 20, 29
	stvebx 5, 20, 28
	stvebx 5, 20, 27
	stvebx 5, 20, 26
	stvebx 5, 20, 25
	stvebx 5, 20, 24
	stvebx 5, 20, 23
	stvebx 5, 20, 22
	stvebx 5, 20, 21
	bne	0, .Ll3v_BB15_4
# %bb.5:                                # %middle.block
	cmplw	6, 7
	beq	0, .Ll3v_BB15_10
# %bb.6:                                # %vec.epilog.iter.check
	andi. 8, 6, 12
	beq	0, .Ll3v_BB15_11
.Ll3v_BB15_7:                               # %vec.epilog.ph
	li 11, .Ll3v_CPI15_0@l
	lis 12, .Ll3v_CPI15_0@ha
	vspltisw128 4, -8
	lvx128 3, 12, 11
	vsrw128 4, 4, 4
	li 0, .Ll3v_CPI15_1@l
	lis 12, .Ll3v_CPI15_1@ha
	vspltisw128 2, 0
	lvx128 5, 12, 0
	mr	8, 7
	rlwinm 7, 6, 0, 1, 29
	addi 9, 1, 16
	addi 10, 1, 32
	li 11, 3
	li 12, 2
	li 0, 1
.Ll3v_BB15_8:                               # %vec.epilog.vector.body
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
	lvx128 0, 0, 9
	stb 30, 32(1)
	add 30, 5, 8
	vperm128 0, 2, 0, 3
	lbz 29, 1(30)
	stb 29, 33(1)
	lbz 29, 2(30)
	stb 29, 34(1)
	lbz 30, 3(30)
	stb 30, 35(1)
	add 30, 3, 8
	lvx128 1, 0, 10
	vperm128 1, 2, 1, 3
	vadduwm 0, 1, 0
	lvsr 1, 3, 8
	vminuw 0, 0, 4
	vperm128 0, 0, 0, 5
	vperm128 0, 0, 0, 1
	stvebx 0, 3, 8
	addi 8, 8, 4
	cmplw	7, 8
	stvebx 0, 30, 11
	stvebx 0, 30, 12
	stvebx 0, 30, 0
	bne	0, .Ll3v_BB15_8
# %bb.9:                                # %vec.epilog.middle.block
	cmplw	6, 7
	bne	0, .Ll3v_BB15_11
.Ll3v_BB15_10:
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
.Ll3v_BB15_11:                              # %for.body.preheader
	addi 8, 7, -1
	add 4, 4, 8
	add 5, 5, 8
	add 3, 3, 8
	b .Ll3v_BB15_13
.Ll3v_BB15_12:                              # %for.body
                                        #   in Loop: Header=BB15_13 Depth=1
	addi 7, 7, 1
	stbu 8, 1(3)
	cmplw	6, 7
	beq	0, .Ll3v_BB15_10
.Ll3v_BB15_13:                              # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lbzu 8, 1(4)
	lbzu 9, 1(5)
	add 8, 9, 8
	cmplwi	8, 255
	blt	0, .Ll3v_BB15_12
# %bb.14:                               # %for.body
                                        #   in Loop: Header=BB15_13 Depth=1
	li 8, 255
	b .Ll3v_BB15_12
.Ll3v_func_end15:
	.size	l3v_k_addsat_u8_r, .Ll3v_func_end15-.Ll3v_func_begin15
                                        # -- End function
	.section	.text.l3v_k_and_u32_r,"ax",@progbits
	.globl	l3v_k_and_u32_r                 # -- Begin function l3v_k_and_u32_r
	.p2align	2
	.type	l3v_k_and_u32_r,@function
l3v_k_and_u32_r:                        # @l3v_k_and_u32_r
.Ll3v_func_begin16:
# %bb.0:                                # %entry
	cmpwi	6, 1
	bltlr	0
# %bb.1:                                # %iter.check
	stwu 1, -176(1)
	cmplwi	6, 4
	li 0, 0
	std 14, 24(1)                           # 8-byte Folded Spill
	std 15, 32(1)                           # 8-byte Folded Spill
	std 16, 40(1)                           # 8-byte Folded Spill
	std 17, 48(1)                           # 8-byte Folded Spill
	std 18, 56(1)                           # 8-byte Folded Spill
	std 19, 64(1)                           # 8-byte Folded Spill
	std 20, 72(1)                           # 8-byte Folded Spill
	std 21, 80(1)                           # 8-byte Folded Spill
	std 22, 88(1)                           # 8-byte Folded Spill
	std 23, 96(1)                           # 8-byte Folded Spill
	std 24, 104(1)                          # 8-byte Folded Spill
	std 25, 112(1)                          # 8-byte Folded Spill
	std 26, 120(1)                          # 8-byte Folded Spill
	std 27, 128(1)                          # 8-byte Folded Spill
	std 28, 136(1)                          # 8-byte Folded Spill
	std 29, 144(1)                          # 8-byte Folded Spill
	std 30, 152(1)                          # 8-byte Folded Spill
	std 31, 160(1)                          # 8-byte Folded Spill
	blt	0, .Ll3v_BB16_10
# %bb.2:                                # %vector.main.loop.iter.check
	cmplwi	6, 32
	blt	0, .Ll3v_BB16_7
# %bb.3:                                # %vector.ph
	rlwinm 19, 6, 0, 1, 26
	li 8, 0
	stw 19, 20(1)                           # 4-byte Folded Spill
	li 9, 16
	li 10, 32
	li 11, 48
	li 12, 64
	li 0, 80
	li 30, 96
	li 29, 112
	li 28, 127
	li 16, 52
	li 15, 76
	li 14, 72
	li 31, 68
	li 7, 92
	li 27, 88
	li 26, 84
	li 25, 108
	li 24, 104
	li 23, 100
	li 22, 124
	li 21, 120
	li 20, 116
.Ll3v_BB16_4:                               # %vector.body
                                        # =>This Inner Loop Header: Depth=1
	add 18, 4, 8
	lvx128 29, 4, 8
	li 17, 28
	addi 19, 19, -32
	lvx128 28, 18, 9
	cmplwi	19, 0
	lvx128 0, 18, 10
	lvx128 1, 18, 11
	lvx128 6, 18, 12
	lvx128 7, 18, 0
	lvx128 8, 18, 30
	lvx128 9, 18, 29
	lvx128 10, 18, 28
	add 18, 5, 8
	lvx128 11, 5, 8
	lvx128 12, 18, 9
	lvx128 13, 18, 10
	lvsl 2, 4, 8
	vperm128 19, 28, 0, 2
	lvsl 3, 5, 8
	vperm128 30, 12, 13, 3
	vand128 19, 30, 19
	lvx128 14, 18, 11
	vperm128 0, 0, 1, 2
	lvx128 15, 18, 12
	lvx128 16, 18, 0
	lvx128 17, 18, 30
	lvx128 18, 18, 29
	lvx128 31, 18, 28
	add 18, 3, 8
	lvsr 4, 18, 9
	vperm128 4, 19, 19, 4
	stvewx 4, 18, 17
	li 17, 24
	stvewx 4, 18, 17
	li 17, 20
	stvewx 4, 18, 17
	li 17, 44
	stvewx 4, 18, 9
	vperm128 4, 13, 14, 3
	vand128 4, 4, 0
	lvsr 5, 18, 10
	vperm128 4, 4, 4, 5
	stvewx 4, 18, 17
	li 17, 40
	vperm128 5, 1, 6, 2
	stvewx 4, 18, 17
	li 17, 36
	stvewx 4, 18, 17
	li 17, 60
	stvewx 4, 18, 10
	vperm128 4, 14, 15, 3
	vand128 4, 4, 5
	lvsr 0, 18, 11
	vperm128 5, 6, 7, 2
	vperm128 4, 4, 4, 0
	lvsr 0, 18, 12
	stvewx 4, 18, 17
	li 17, 56
	stvewx 4, 18, 17
	li 17, 12
	stvewx 4, 18, 16
	stvewx 4, 18, 11
	vperm128 4, 15, 16, 3
	vand128 4, 4, 5
	vperm128 4, 4, 4, 0
	lvsr 0, 18, 0
	stvewx 4, 18, 15
	vperm128 5, 7, 8, 2
	stvewx 4, 18, 14
	stvewx 4, 18, 31
	stvewx 4, 18, 12
	vperm128 4, 16, 17, 3
	vand128 4, 4, 5
	vperm128 4, 4, 4, 0
	lvsr 0, 18, 30
	stvewx 4, 18, 7
	vperm128 5, 8, 9, 2
	stvewx 4, 18, 27
	stvewx 4, 18, 26
	stvewx 4, 18, 0
	vperm128 4, 17, 18, 3
	vand128 4, 4, 5
	vperm128 4, 4, 4, 0
	lvsr 0, 18, 29
	stvewx 4, 18, 25
	vperm128 5, 9, 10, 2
	stvewx 4, 18, 24
	vperm128 2, 29, 28, 2
	stvewx 4, 18, 23
	stvewx 4, 18, 30
	vperm128 4, 18, 31, 3
	vand128 4, 4, 5
	vperm128 4, 4, 4, 0
	stvewx 4, 18, 22
	vperm128 3, 11, 12, 3
	stvewx 4, 18, 21
	vand128 2, 3, 2
	stvewx 4, 18, 20
	stvewx 4, 18, 29
	lvsr 4, 3, 8
	vperm128 2, 2, 2, 4
	stvewx 2, 18, 17
	li 17, 8
	stvewx 2, 18, 17
	li 17, 4
	stvewx 2, 3, 8
	addi 8, 8, 128
	stvewx 2, 18, 17
	bne	0, .Ll3v_BB16_4
# %bb.5:                                # %middle.block
	lwz 0, 20(1)                            # 4-byte Folded Reload
	cmplw	6, 0
	beq	0, .Ll3v_BB16_12
# %bb.6:                                # %vec.epilog.iter.check
	andi. 7, 6, 28
	beq	0, .Ll3v_BB16_10
.Ll3v_BB16_7:                               # %vec.epilog.ph
	mr	10, 0
	rlwinm 0, 6, 0, 1, 29
	li 7, 15
	li 8, 12
	sub	9, 10, 0
	slwi 10, 10, 2
	li 11, 8
	li 12, 4
.Ll3v_BB16_8:                               # %vec.epilog.vector.body
                                        # =>This Inner Loop Header: Depth=1
	add 30, 4, 10
	lvx128 3, 4, 10
	addi 9, 9, 4
	cmplwi	9, 0
	lvx128 4, 30, 7
	add 30, 5, 10
	lvx128 5, 5, 10
	lvx128 0, 30, 7
	add 30, 3, 10
	lvsl 2, 4, 10
	vperm128 2, 3, 4, 2
	lvsl 3, 5, 10
	vperm128 3, 5, 0, 3
	vand128 2, 3, 2
	lvsr 4, 3, 10
	vperm128 2, 2, 2, 4
	stvewx 2, 3, 10
	addi 10, 10, 16
	stvewx 2, 30, 8
	stvewx 2, 30, 11
	stvewx 2, 30, 12
	bne	0, .Ll3v_BB16_8
# %bb.9:                                # %vec.epilog.middle.block
	cmplw	6, 0
	beq	0, .Ll3v_BB16_12
.Ll3v_BB16_10:                              # %for.body.preheader
	slwi 7, 0, 2
	sub	6, 6, 0
	addi 7, 7, -4
	add 4, 4, 7
	add 5, 5, 7
	add 3, 3, 7
.Ll3v_BB16_11:                              # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lwzu 7, 4(4)
	addi 6, 6, -1
	cmplwi	6, 0
	lwzu 8, 4(5)
	and 7, 8, 7
	stwu 7, 4(3)
	bne	0, .Ll3v_BB16_11
.Ll3v_BB16_12:
	ld 31, 160(1)                           # 8-byte Folded Reload
	ld 30, 152(1)                           # 8-byte Folded Reload
	ld 29, 144(1)                           # 8-byte Folded Reload
	ld 28, 136(1)                           # 8-byte Folded Reload
	ld 27, 128(1)                           # 8-byte Folded Reload
	ld 26, 120(1)                           # 8-byte Folded Reload
	ld 25, 112(1)                           # 8-byte Folded Reload
	ld 24, 104(1)                           # 8-byte Folded Reload
	ld 23, 96(1)                            # 8-byte Folded Reload
	ld 22, 88(1)                            # 8-byte Folded Reload
	ld 21, 80(1)                            # 8-byte Folded Reload
	ld 20, 72(1)                            # 8-byte Folded Reload
	ld 19, 64(1)                            # 8-byte Folded Reload
	ld 18, 56(1)                            # 8-byte Folded Reload
	ld 17, 48(1)                            # 8-byte Folded Reload
	ld 16, 40(1)                            # 8-byte Folded Reload
	ld 15, 32(1)                            # 8-byte Folded Reload
	ld 14, 24(1)                            # 8-byte Folded Reload
	addi 1, 1, 176
	blr
.Ll3v_func_end16:
	.size	l3v_k_and_u32_r, .Ll3v_func_end16-.Ll3v_func_begin16
                                        # -- End function
	.section	.text.l3v_k_shl_i32_r,"ax",@progbits
	.globl	l3v_k_shl_i32_r                 # -- Begin function l3v_k_shl_i32_r
	.p2align	2
	.type	l3v_k_shl_i32_r,@function
l3v_k_shl_i32_r:                        # @l3v_k_shl_i32_r
.Ll3v_func_begin17:
# %bb.0:                                # %entry
	cmpwi	5, 1
	bltlr	0
# %bb.1:                                # %iter.check
	stwu 1, -176(1)
	cmplwi	5, 4
	li 0, 0
	std 14, 24(1)                           # 8-byte Folded Spill
	std 15, 32(1)                           # 8-byte Folded Spill
	std 16, 40(1)                           # 8-byte Folded Spill
	std 17, 48(1)                           # 8-byte Folded Spill
	std 18, 56(1)                           # 8-byte Folded Spill
	std 19, 64(1)                           # 8-byte Folded Spill
	std 20, 72(1)                           # 8-byte Folded Spill
	std 21, 80(1)                           # 8-byte Folded Spill
	std 22, 88(1)                           # 8-byte Folded Spill
	std 23, 96(1)                           # 8-byte Folded Spill
	std 24, 104(1)                          # 8-byte Folded Spill
	std 25, 112(1)                          # 8-byte Folded Spill
	std 26, 120(1)                          # 8-byte Folded Spill
	std 27, 128(1)                          # 8-byte Folded Spill
	std 28, 136(1)                          # 8-byte Folded Spill
	std 29, 144(1)                          # 8-byte Folded Spill
	std 30, 152(1)                          # 8-byte Folded Spill
	std 31, 160(1)                          # 8-byte Folded Spill
	blt	0, .Ll3v_BB17_10
# %bb.2:                                # %vector.main.loop.iter.check
	cmplwi	5, 32
	blt	0, .Ll3v_BB17_7
# %bb.3:                                # %vector.ph
	stw 5, 20(1)                            # 4-byte Folded Spill
	rlwinm 5, 5, 0, 1, 26
	li 7, 0
	vspltisw128 2, 3
	li 8, 16
	li 9, 32
	stw 5, 16(1)                            # 4-byte Folded Spill
	li 10, 48
	li 11, 64
	li 12, 80
	li 0, 96
	li 30, 112
	li 20, 36
	li 19, 60
	li 18, 56
	li 17, 52
	li 16, 76
	li 15, 72
	li 14, 68
	li 31, 92
	li 6, 88
	li 29, 84
	li 28, 108
	li 27, 104
	li 26, 100
	li 25, 124
	li 24, 120
	li 23, 116
.Ll3v_BB17_4:                               # %vector.body
                                        # =>This Inner Loop Header: Depth=1
	add 22, 4, 7
	lvx128 12, 4, 7
	li 21, 127
	addi 5, 5, -32
	lvx128 5, 22, 8
	cmplwi	5, 0
	lvx128 0, 22, 9
	lvsl 3, 4, 7
	vperm128 11, 5, 0, 3
	vslw128 11, 11, 2
	lvx128 1, 22, 10
	lvx128 6, 22, 11
	lvx128 7, 22, 12
	lvx128 8, 22, 0
	lvx128 9, 22, 30
	lvx128 10, 22, 21
	add 22, 3, 7
	li 21, 28
	lvsr 4, 22, 8
	vperm128 4, 11, 11, 4
	stvewx 4, 22, 21
	li 21, 24
	stvewx 4, 22, 21
	li 21, 20
	stvewx 4, 22, 21
	li 21, 44
	stvewx 4, 22, 8
	vperm128 4, 0, 1, 3
	vslw128 4, 4, 2
	lvsr 0, 22, 9
	vperm128 4, 4, 4, 0
	stvewx 4, 22, 21
	li 21, 40
	stvewx 4, 22, 21
	li 21, 12
	stvewx 4, 22, 20
	stvewx 4, 22, 9
	vperm128 4, 1, 6, 3
	vslw128 4, 4, 2
	lvsr 0, 22, 10
	vperm128 4, 4, 4, 0
	stvewx 4, 22, 19
	stvewx 4, 22, 18
	stvewx 4, 22, 17
	stvewx 4, 22, 10
	vperm128 4, 6, 7, 3
	vslw128 4, 4, 2
	lvsr 0, 22, 11
	vperm128 4, 4, 4, 0
	stvewx 4, 22, 16
	stvewx 4, 22, 15
	stvewx 4, 22, 14
	stvewx 4, 22, 11
	vperm128 4, 7, 8, 3
	vslw128 4, 4, 2
	lvsr 0, 22, 12
	vperm128 4, 4, 4, 0
	stvewx 4, 22, 31
	stvewx 4, 22, 6
	stvewx 4, 22, 29
	stvewx 4, 22, 12
	vperm128 4, 8, 9, 3
	vslw128 4, 4, 2
	lvsr 0, 22, 0
	vperm128 4, 4, 4, 0
	stvewx 4, 22, 28
	stvewx 4, 22, 27
	stvewx 4, 22, 26
	stvewx 4, 22, 0
	vperm128 4, 9, 10, 3
	vslw128 4, 4, 2
	lvsr 0, 22, 30
	vperm128 3, 12, 5, 3
	vslw128 3, 3, 2
	vperm128 4, 4, 4, 0
	stvewx 4, 22, 25
	stvewx 4, 22, 24
	stvewx 4, 22, 23
	stvewx 4, 22, 30
	lvsr 4, 3, 7
	vperm128 3, 3, 3, 4
	stvewx 3, 22, 21
	li 21, 8
	stvewx 3, 22, 21
	li 21, 4
	stvewx 3, 3, 7
	addi 7, 7, 128
	stvewx 3, 22, 21
	bne	0, .Ll3v_BB17_4
# %bb.5:                                # %middle.block
	lwz 5, 20(1)                            # 4-byte Folded Reload
	lwz 0, 16(1)                            # 4-byte Folded Reload
	cmplw	5, 0
	beq	0, .Ll3v_BB17_12
# %bb.6:                                # %vec.epilog.iter.check
	andi. 6, 5, 28
	beq	0, .Ll3v_BB17_10
.Ll3v_BB17_7:                               # %vec.epilog.ph
	mr	8, 0
	rlwinm 0, 5, 0, 1, 29
	vspltisw128 2, 3
	li 12, 15
	li 6, 12
	sub	7, 8, 0
	slwi 8, 8, 2
	li 9, 8
	li 10, 4
.Ll3v_BB17_8:                               # %vec.epilog.vector.body
                                        # =>This Inner Loop Header: Depth=1
	add 11, 4, 8
	lvx128 3, 4, 8
	addi 7, 7, 4
	cmplwi	7, 0
	lvx128 4, 11, 12
	add 11, 3, 8
	lvsl 5, 4, 8
	vperm128 3, 3, 4, 5
	vslw128 3, 3, 2
	lvsr 4, 3, 8
	vperm128 3, 3, 3, 4
	stvewx 3, 3, 8
	addi 8, 8, 16
	stvewx 3, 11, 6
	stvewx 3, 11, 9
	stvewx 3, 11, 10
	bne	0, .Ll3v_BB17_8
# %bb.9:                                # %vec.epilog.middle.block
	cmplw	5, 0
	beq	0, .Ll3v_BB17_12
.Ll3v_BB17_10:                              # %for.body.preheader
	slwi 6, 0, 2
	sub	5, 5, 0
	addi 6, 6, -4
	add 4, 4, 6
	add 3, 3, 6
.Ll3v_BB17_11:                              # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lwzu 6, 4(4)
	addi 5, 5, -1
	cmplwi	5, 0
	slwi 6, 6, 3
	stwu 6, 4(3)
	bne	0, .Ll3v_BB17_11
.Ll3v_BB17_12:
	ld 31, 160(1)                           # 8-byte Folded Reload
	ld 30, 152(1)                           # 8-byte Folded Reload
	ld 29, 144(1)                           # 8-byte Folded Reload
	ld 28, 136(1)                           # 8-byte Folded Reload
	ld 27, 128(1)                           # 8-byte Folded Reload
	ld 26, 120(1)                           # 8-byte Folded Reload
	ld 25, 112(1)                           # 8-byte Folded Reload
	ld 24, 104(1)                           # 8-byte Folded Reload
	ld 23, 96(1)                            # 8-byte Folded Reload
	ld 22, 88(1)                            # 8-byte Folded Reload
	ld 21, 80(1)                            # 8-byte Folded Reload
	ld 20, 72(1)                            # 8-byte Folded Reload
	ld 19, 64(1)                            # 8-byte Folded Reload
	ld 18, 56(1)                            # 8-byte Folded Reload
	ld 17, 48(1)                            # 8-byte Folded Reload
	ld 16, 40(1)                            # 8-byte Folded Reload
	ld 15, 32(1)                            # 8-byte Folded Reload
	ld 14, 24(1)                            # 8-byte Folded Reload
	addi 1, 1, 176
	blr
.Ll3v_func_end17:
	.size	l3v_k_shl_i32_r, .Ll3v_func_end17-.Ll3v_func_begin17
                                        # -- End function
	.section	.text.l3v_k_mul_i32_r,"ax",@progbits
	.globl	l3v_k_mul_i32_r                 # -- Begin function l3v_k_mul_i32_r
	.p2align	2
	.type	l3v_k_mul_i32_r,@function
l3v_k_mul_i32_r:                        # @l3v_k_mul_i32_r
.Ll3v_func_begin18:
# %bb.0:                                # %entry
	cmpwi	6, 1
	bltlr	0
# %bb.1:                                # %for.body.preheader
	stwu 1, -48(1)
	li 7, 0
	std 30, 24(1)                           # 8-byte Folded Spill
	beq	0, .Ll3v_BB18_5
# %bb.2:                                # %vector.ph
	rlwinm 7, 6, 0, 1, 30
	addi 8, 4, -8
	addi 9, 5, -8
	addi 10, 3, -8
	mr	11, 7
.Ll3v_BB18_3:                               # %vector.body
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
	bne	0, .Ll3v_BB18_3
# %bb.4:                                # %middle.block
	cmplw	6, 7
	beq	0, .Ll3v_BB18_7
.Ll3v_BB18_5:                               # %for.body.preheader9
	slwi 8, 7, 2
	sub	6, 6, 7
	addi 8, 8, -4
	add 4, 4, 8
	add 5, 5, 8
	add 3, 3, 8
.Ll3v_BB18_6:                               # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lwzu 7, 4(4)
	addi 6, 6, -1
	cmplwi	6, 0
	lwzu 8, 4(5)
	mullw 7, 8, 7
	stwu 7, 4(3)
	bne	0, .Ll3v_BB18_6
.Ll3v_BB18_7:
	ld 30, 24(1)                            # 8-byte Folded Reload
	addi 1, 1, 48
	blr
.Ll3v_func_end18:
	.size	l3v_k_mul_i32_r, .Ll3v_func_end18-.Ll3v_func_begin18
                                        # -- End function
	.section	.text.l3v_k_min_f32_r,"ax",@progbits
	.globl	l3v_k_min_f32_r                 # -- Begin function l3v_k_min_f32_r
	.p2align	2
	.type	l3v_k_min_f32_r,@function
l3v_k_min_f32_r:                        # @l3v_k_min_f32_r
.Ll3v_func_begin19:
# %bb.0:                                # %entry
	cmpwi	6, 1
	bltlr	0
# %bb.1:                                # %iter.check
	stwu 1, -176(1)
	cmplwi	6, 4
	li 0, 0
	std 14, 24(1)                           # 8-byte Folded Spill
	std 15, 32(1)                           # 8-byte Folded Spill
	std 16, 40(1)                           # 8-byte Folded Spill
	std 17, 48(1)                           # 8-byte Folded Spill
	std 18, 56(1)                           # 8-byte Folded Spill
	std 19, 64(1)                           # 8-byte Folded Spill
	std 20, 72(1)                           # 8-byte Folded Spill
	std 21, 80(1)                           # 8-byte Folded Spill
	std 22, 88(1)                           # 8-byte Folded Spill
	std 23, 96(1)                           # 8-byte Folded Spill
	std 24, 104(1)                          # 8-byte Folded Spill
	std 25, 112(1)                          # 8-byte Folded Spill
	std 26, 120(1)                          # 8-byte Folded Spill
	std 27, 128(1)                          # 8-byte Folded Spill
	std 28, 136(1)                          # 8-byte Folded Spill
	std 29, 144(1)                          # 8-byte Folded Spill
	std 30, 152(1)                          # 8-byte Folded Spill
	std 31, 160(1)                          # 8-byte Folded Spill
	blt	0, .Ll3v_BB19_11
# %bb.2:                                # %vector.main.loop.iter.check
	cmplwi	6, 32
	blt	0, .Ll3v_BB19_7
# %bb.3:                                # %vector.ph
	rlwinm 19, 6, 0, 1, 26
	li 8, 0
	stw 19, 20(1)                           # 4-byte Folded Spill
	li 9, 16
	li 10, 32
	li 11, 48
	li 12, 64
	li 0, 80
	li 30, 96
	li 29, 112
	li 28, 127
	li 16, 52
	li 15, 76
	li 14, 72
	li 31, 68
	li 7, 92
	li 27, 88
	li 26, 84
	li 25, 108
	li 24, 104
	li 23, 100
	li 22, 124
	li 21, 120
	li 20, 116
.Ll3v_BB19_4:                               # %vector.body
                                        # =>This Inner Loop Header: Depth=1
	add 18, 4, 8
	lvx128 29, 4, 8
	li 17, 28
	addi 19, 19, -32
	lvx128 27, 18, 9
	cmplwi	19, 0
	lvx128 0, 18, 10
	lvx128 1, 18, 11
	lvx128 6, 18, 12
	lvx128 7, 18, 0
	lvx128 8, 18, 30
	lvx128 9, 18, 29
	lvx128 10, 18, 28
	add 18, 5, 8
	lvx128 11, 5, 8
	lvx128 12, 18, 9
	lvx128 13, 18, 10
	lvsl 2, 4, 8
	vperm128 19, 27, 0, 2
	lvsl 3, 5, 8
	vperm128 30, 12, 13, 3
	vcmpgtfp128 28, 30, 19
	lvx128 14, 18, 11
	vsel128 28, 30, 19, 28
	vperm128 0, 0, 1, 2
	lvx128 15, 18, 12
	lvx128 16, 18, 0
	lvx128 17, 18, 30
	lvx128 18, 18, 29
	lvx128 31, 18, 28
	add 18, 3, 8
	lvsr 4, 18, 9
	vperm128 4, 28, 28, 4
	stvewx 4, 18, 17
	li 17, 24
	stvewx 4, 18, 17
	li 17, 20
	stvewx 4, 18, 17
	li 17, 44
	stvewx 4, 18, 9
	vperm128 4, 13, 14, 3
	vcmpgtfp128 13, 4, 0
	vsel128 13, 4, 0, 13
	lvsr 5, 18, 10
	vperm128 4, 13, 13, 5
	stvewx 4, 18, 17
	li 17, 40
	vperm128 5, 1, 6, 2
	stvewx 4, 18, 17
	li 17, 36
	stvewx 4, 18, 17
	li 17, 60
	stvewx 4, 18, 10
	vperm128 4, 14, 15, 3
	vcmpgtfp128 1, 4, 5
	vsel128 1, 4, 5, 1
	lvsr 0, 18, 11
	vperm128 5, 6, 7, 2
	vperm128 4, 1, 1, 0
	lvsr 0, 18, 12
	stvewx 4, 18, 17
	li 17, 56
	stvewx 4, 18, 17
	li 17, 12
	stvewx 4, 18, 16
	stvewx 4, 18, 11
	vperm128 4, 15, 16, 3
	vcmpgtfp128 1, 4, 5
	vsel128 1, 4, 5, 1
	vperm128 4, 1, 1, 0
	lvsr 0, 18, 0
	stvewx 4, 18, 15
	vperm128 5, 7, 8, 2
	stvewx 4, 18, 14
	stvewx 4, 18, 31
	stvewx 4, 18, 12
	vperm128 4, 16, 17, 3
	vcmpgtfp128 1, 4, 5
	vsel128 1, 4, 5, 1
	vperm128 4, 1, 1, 0
	lvsr 0, 18, 30
	stvewx 4, 18, 7
	vperm128 5, 8, 9, 2
	stvewx 4, 18, 27
	stvewx 4, 18, 26
	stvewx 4, 18, 0
	vperm128 4, 17, 18, 3
	vcmpgtfp128 1, 4, 5
	vsel128 1, 4, 5, 1
	vperm128 4, 1, 1, 0
	lvsr 0, 18, 29
	stvewx 4, 18, 25
	vperm128 5, 9, 10, 2
	stvewx 4, 18, 24
	vperm128 2, 29, 27, 2
	stvewx 4, 18, 23
	stvewx 4, 18, 30
	vperm128 4, 18, 31, 3
	vcmpgtfp128 1, 4, 5
	vsel128 1, 4, 5, 1
	vperm128 3, 11, 12, 3
	vcmpgtfp128 5, 3, 2
	vsel128 5, 3, 2, 5
	vperm128 4, 1, 1, 0
	stvewx 4, 18, 22
	stvewx 4, 18, 21
	stvewx 4, 18, 20
	stvewx 4, 18, 29
	lvsr 4, 3, 8
	vperm128 2, 5, 5, 4
	stvewx 2, 18, 17
	li 17, 8
	stvewx 2, 18, 17
	li 17, 4
	stvewx 2, 3, 8
	addi 8, 8, 128
	stvewx 2, 18, 17
	bne	0, .Ll3v_BB19_4
# %bb.5:                                # %middle.block
	lwz 0, 20(1)                            # 4-byte Folded Reload
	cmplw	6, 0
	beq	0, .Ll3v_BB19_10
# %bb.6:                                # %vec.epilog.iter.check
	andi. 7, 6, 28
	beq	0, .Ll3v_BB19_11
.Ll3v_BB19_7:                               # %vec.epilog.ph
	mr	10, 0
	rlwinm 0, 6, 0, 1, 29
	li 7, 15
	li 8, 12
	sub	9, 10, 0
	slwi 10, 10, 2
	li 11, 8
	li 12, 4
.Ll3v_BB19_8:                               # %vec.epilog.vector.body
                                        # =>This Inner Loop Header: Depth=1
	add 30, 4, 10
	lvx128 3, 4, 10
	addi 9, 9, 4
	cmplwi	9, 0
	lvx128 4, 30, 7
	add 30, 5, 10
	lvx128 5, 5, 10
	lvx128 0, 30, 7
	add 30, 3, 10
	lvsl 2, 4, 10
	vperm128 2, 3, 4, 2
	lvsl 3, 5, 10
	vperm128 3, 5, 0, 3
	vcmpgtfp128 5, 3, 2
	lvsr 4, 3, 10
	vsel128 5, 3, 2, 5
	vperm128 2, 5, 5, 4
	stvewx 2, 3, 10
	addi 10, 10, 16
	stvewx 2, 30, 8
	stvewx 2, 30, 11
	stvewx 2, 30, 12
	bne	0, .Ll3v_BB19_8
# %bb.9:                                # %vec.epilog.middle.block
	cmplw	6, 0
	bne	0, .Ll3v_BB19_11
.Ll3v_BB19_10:
	ld 31, 160(1)                           # 8-byte Folded Reload
	ld 30, 152(1)                           # 8-byte Folded Reload
	ld 29, 144(1)                           # 8-byte Folded Reload
	ld 28, 136(1)                           # 8-byte Folded Reload
	ld 27, 128(1)                           # 8-byte Folded Reload
	ld 26, 120(1)                           # 8-byte Folded Reload
	ld 25, 112(1)                           # 8-byte Folded Reload
	ld 24, 104(1)                           # 8-byte Folded Reload
	ld 23, 96(1)                            # 8-byte Folded Reload
	ld 22, 88(1)                            # 8-byte Folded Reload
	ld 21, 80(1)                            # 8-byte Folded Reload
	ld 20, 72(1)                            # 8-byte Folded Reload
	ld 19, 64(1)                            # 8-byte Folded Reload
	ld 18, 56(1)                            # 8-byte Folded Reload
	ld 17, 48(1)                            # 8-byte Folded Reload
	ld 16, 40(1)                            # 8-byte Folded Reload
	ld 15, 32(1)                            # 8-byte Folded Reload
	ld 14, 24(1)                            # 8-byte Folded Reload
	addi 1, 1, 176
	blr
.Ll3v_BB19_11:                              # %for.body.preheader
	slwi 7, 0, 2
	sub	6, 6, 0
	addi 7, 7, -4
	add 4, 4, 7
	add 5, 5, 7
	add 3, 3, 7
	b .Ll3v_BB19_13
.Ll3v_BB19_12:                              # %for.body
                                        #   in Loop: Header=BB19_13 Depth=1
	addi 6, 6, -1
	stfsu 0, 4(3)
	cmplwi	6, 0
	beq	0, .Ll3v_BB19_10
.Ll3v_BB19_13:                              # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lfsu 0, 4(4)
	lfsu 1, 4(5)
	fcmpu 0, 0, 1
	blt	0, .Ll3v_BB19_12
# %bb.14:                               # %for.body
                                        #   in Loop: Header=BB19_13 Depth=1
	fmr 0, 1
	b .Ll3v_BB19_12
.Ll3v_func_end19:
	.size	l3v_k_min_f32_r, .Ll3v_func_end19-.Ll3v_func_begin19
                                        # -- End function
	.section	.text.l3v_k_max_i32_r,"ax",@progbits
	.globl	l3v_k_max_i32_r                 # -- Begin function l3v_k_max_i32_r
	.p2align	2
	.type	l3v_k_max_i32_r,@function
l3v_k_max_i32_r:                        # @l3v_k_max_i32_r
.Ll3v_func_begin20:
# %bb.0:                                # %entry
	cmpwi	6, 1
	bltlr	0
# %bb.1:                                # %iter.check
	stwu 1, -176(1)
	cmplwi	6, 4
	li 0, 0
	std 14, 24(1)                           # 8-byte Folded Spill
	std 15, 32(1)                           # 8-byte Folded Spill
	std 16, 40(1)                           # 8-byte Folded Spill
	std 17, 48(1)                           # 8-byte Folded Spill
	std 18, 56(1)                           # 8-byte Folded Spill
	std 19, 64(1)                           # 8-byte Folded Spill
	std 20, 72(1)                           # 8-byte Folded Spill
	std 21, 80(1)                           # 8-byte Folded Spill
	std 22, 88(1)                           # 8-byte Folded Spill
	std 23, 96(1)                           # 8-byte Folded Spill
	std 24, 104(1)                          # 8-byte Folded Spill
	std 25, 112(1)                          # 8-byte Folded Spill
	std 26, 120(1)                          # 8-byte Folded Spill
	std 27, 128(1)                          # 8-byte Folded Spill
	std 28, 136(1)                          # 8-byte Folded Spill
	std 29, 144(1)                          # 8-byte Folded Spill
	std 30, 152(1)                          # 8-byte Folded Spill
	std 31, 160(1)                          # 8-byte Folded Spill
	blt	0, .Ll3v_BB20_11
# %bb.2:                                # %vector.main.loop.iter.check
	cmplwi	6, 32
	blt	0, .Ll3v_BB20_7
# %bb.3:                                # %vector.ph
	rlwinm 19, 6, 0, 1, 26
	li 8, 0
	stw 19, 20(1)                           # 4-byte Folded Spill
	li 9, 16
	li 10, 32
	li 11, 48
	li 12, 64
	li 0, 80
	li 30, 96
	li 29, 112
	li 28, 127
	li 16, 52
	li 15, 76
	li 14, 72
	li 31, 68
	li 7, 92
	li 27, 88
	li 26, 84
	li 25, 108
	li 24, 104
	li 23, 100
	li 22, 124
	li 21, 120
	li 20, 116
.Ll3v_BB20_4:                               # %vector.body
                                        # =>This Inner Loop Header: Depth=1
	add 18, 4, 8
	lvx128 29, 4, 8
	li 17, 28
	addi 19, 19, -32
	lvx128 26, 18, 9
	cmplwi	19, 0
	lvx128 5, 18, 10
	lvx128 6, 18, 11
	lvx128 27, 18, 12
	lvx128 8, 18, 0
	lvx128 9, 18, 30
	lvx128 10, 18, 29
	lvx128 11, 18, 28
	add 18, 5, 8
	lvx128 12, 5, 8
	lvx128 13, 18, 9
	lvx128 14, 18, 10
	lvsl 2, 4, 8
	vperm128 30, 26, 5, 2
	lvsl 3, 5, 8
	vperm128 28, 13, 14, 3
	vmaxsw 30, 30, 28
	lvx128 15, 18, 11
	vperm128 5, 5, 6, 2
	lvx128 16, 18, 12
	vperm128 12, 12, 13, 3
	lvx128 17, 18, 0
	lvx128 18, 18, 30
	lvx128 19, 18, 29
	lvx128 31, 18, 28
	add 18, 3, 8
	lvsr 0, 18, 9
	vperm128 0, 30, 30, 0
	stvewx 0, 18, 17
	li 17, 24
	stvewx 0, 18, 17
	li 17, 20
	stvewx 0, 18, 17
	li 17, 44
	stvewx 0, 18, 9
	vperm128 0, 14, 15, 3
	vmaxsw 5, 5, 0
	lvsr 7, 18, 10
	vperm128 0, 6, 27, 2
	vperm128 5, 5, 5, 7
	lvsr 4, 3, 8
	vperm128 7, 29, 26, 2
	stvewx 5, 18, 17
	li 17, 40
	vmaxsw 7, 7, 12
	stvewx 5, 18, 17
	li 17, 36
	vperm128 6, 15, 16, 3
	vperm128 4, 7, 7, 4
	lvsr 1, 18, 11
	vperm128 7, 27, 8, 2
	stvewx 5, 18, 17
	vmaxsw 0, 0, 6
	li 17, 60
	vperm128 12, 16, 17, 3
	stvewx 5, 18, 10
	lvsr 5, 18, 12
	vperm128 0, 0, 0, 1
	vmaxsw 7, 7, 12
	vperm128 5, 7, 7, 5
	stvewx 0, 18, 17
	li 17, 56
	vperm128 7, 8, 9, 2
	lvsr 6, 18, 0
	vperm128 8, 17, 18, 3
	stvewx 5, 18, 15
	stvewx 5, 18, 14
	stvewx 5, 18, 31
	stvewx 5, 18, 12
	vmaxsw 5, 7, 8
	vperm128 7, 18, 19, 3
	stvewx 0, 18, 17
	li 17, 12
	vperm128 5, 5, 5, 6
	vperm128 6, 9, 10, 2
	stvewx 0, 18, 16
	vperm128 2, 10, 11, 2
	stvewx 0, 18, 11
	vperm128 3, 19, 31, 3
	lvsr 1, 18, 30
	vmaxsw 2, 2, 3
	lvsr 0, 18, 29
	vperm128 2, 2, 2, 0
	stvewx 4, 18, 17
	li 17, 8
	stvewx 5, 18, 7
	stvewx 5, 18, 27
	stvewx 5, 18, 26
	stvewx 5, 18, 0
	vmaxsw 5, 6, 7
	vperm128 5, 5, 5, 1
	stvewx 4, 18, 17
	li 17, 4
	stvewx 4, 3, 8
	addi 8, 8, 128
	stvewx 4, 18, 17
	stvewx 5, 18, 25
	stvewx 5, 18, 24
	stvewx 5, 18, 23
	stvewx 5, 18, 30
	stvewx 2, 18, 22
	stvewx 2, 18, 21
	stvewx 2, 18, 20
	stvewx 2, 18, 29
	bne	0, .Ll3v_BB20_4
# %bb.5:                                # %middle.block
	lwz 0, 20(1)                            # 4-byte Folded Reload
	cmplw	6, 0
	beq	0, .Ll3v_BB20_10
# %bb.6:                                # %vec.epilog.iter.check
	andi. 7, 6, 28
	beq	0, .Ll3v_BB20_11
.Ll3v_BB20_7:                               # %vec.epilog.ph
	mr	10, 0
	rlwinm 0, 6, 0, 1, 29
	li 7, 15
	li 8, 12
	sub	9, 10, 0
	slwi 10, 10, 2
	li 11, 8
	li 12, 4
.Ll3v_BB20_8:                               # %vec.epilog.vector.body
                                        # =>This Inner Loop Header: Depth=1
	add 30, 4, 10
	lvx128 3, 4, 10
	addi 9, 9, 4
	cmplwi	9, 0
	lvx128 4, 30, 7
	add 30, 5, 10
	lvx128 5, 5, 10
	lvx128 0, 30, 7
	add 30, 3, 10
	lvsl 2, 4, 10
	vperm128 2, 3, 4, 2
	lvsl 3, 5, 10
	vperm128 3, 5, 0, 3
	vmaxsw 2, 2, 3
	lvsr 4, 3, 10
	vperm128 2, 2, 2, 4
	stvewx 2, 3, 10
	addi 10, 10, 16
	stvewx 2, 30, 8
	stvewx 2, 30, 11
	stvewx 2, 30, 12
	bne	0, .Ll3v_BB20_8
# %bb.9:                                # %vec.epilog.middle.block
	cmplw	6, 0
	bne	0, .Ll3v_BB20_11
.Ll3v_BB20_10:
	ld 31, 160(1)                           # 8-byte Folded Reload
	ld 30, 152(1)                           # 8-byte Folded Reload
	ld 29, 144(1)                           # 8-byte Folded Reload
	ld 28, 136(1)                           # 8-byte Folded Reload
	ld 27, 128(1)                           # 8-byte Folded Reload
	ld 26, 120(1)                           # 8-byte Folded Reload
	ld 25, 112(1)                           # 8-byte Folded Reload
	ld 24, 104(1)                           # 8-byte Folded Reload
	ld 23, 96(1)                            # 8-byte Folded Reload
	ld 22, 88(1)                            # 8-byte Folded Reload
	ld 21, 80(1)                            # 8-byte Folded Reload
	ld 20, 72(1)                            # 8-byte Folded Reload
	ld 19, 64(1)                            # 8-byte Folded Reload
	ld 18, 56(1)                            # 8-byte Folded Reload
	ld 17, 48(1)                            # 8-byte Folded Reload
	ld 16, 40(1)                            # 8-byte Folded Reload
	ld 15, 32(1)                            # 8-byte Folded Reload
	ld 14, 24(1)                            # 8-byte Folded Reload
	addi 1, 1, 176
	blr
.Ll3v_BB20_11:                              # %for.body.preheader
	slwi 7, 0, 2
	sub	6, 6, 0
	addi 7, 7, -4
	add 4, 4, 7
	add 5, 5, 7
	add 3, 3, 7
	b .Ll3v_BB20_13
.Ll3v_BB20_12:                              # %for.body
                                        #   in Loop: Header=BB20_13 Depth=1
	addi 6, 6, -1
	stwu 7, 4(3)
	cmplwi	6, 0
	beq	0, .Ll3v_BB20_10
.Ll3v_BB20_13:                              # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lwzu 7, 4(4)
	lwzu 8, 4(5)
	cmpw	7, 8
	bgt	0, .Ll3v_BB20_12
# %bb.14:                               # %for.body
                                        #   in Loop: Header=BB20_13 Depth=1
	mr	7, 8
	b .Ll3v_BB20_12
.Ll3v_func_end20:
	.size	l3v_k_max_i32_r, .Ll3v_func_end20-.Ll3v_func_begin20
                                        # -- End function
	.section	.text.l3v_k_clamp_f32_r,"ax",@progbits
	.globl	l3v_k_clamp_f32_r               # -- Begin function l3v_k_clamp_f32_r
	.p2align	2
	.type	l3v_k_clamp_f32_r,@function
l3v_k_clamp_f32_r:                      # @l3v_k_clamp_f32_r
.Ll3v_func_begin21:
# %bb.0:                                # %entry
	cmpwi	7, 1
	bltlr	0
# %bb.1:                                # %iter.check
	stwu 1, -256(1)
	cmplwi	7, 4
	li 0, 0
	std 14, 104(1)                          # 8-byte Folded Spill
	std 15, 112(1)                          # 8-byte Folded Spill
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
	std 31, 240(1)                          # 8-byte Folded Spill
	blt	0, .Ll3v_BB21_11
# %bb.2:                                # %vector.main.loop.iter.check
	cmplwi	7, 32
	blt	0, .Ll3v_BB21_7
# %bb.3:                                # %vector.ph
	addi 5, 1, 80
	stfs 1, 80(1)
	addi 26, 1, 64
	rlwinm 22, 7, 0, 1, 26
	li 6, 0
	lvx128 2, 0, 5
	li 8, 16
	li 9, 32
	li 10, 48
	li 11, 64
	stfs 2, 64(1)
	vspltw128 2, 2, 0
	li 12, 80
	li 0, 96
	li 30, 112
	li 19, 60
	lvx128 3, 0, 26
	li 18, 56
	li 17, 52
	li 16, 76
	li 15, 72
	vspltw128 3, 3, 0
	stw 22, 28(1)                           # 4-byte Folded Spill
	li 14, 68
	li 31, 92
	li 5, 88
	li 29, 84
	li 28, 108
	li 27, 104
	li 26, 100
	li 25, 124
	li 24, 120
	li 23, 116
.Ll3v_BB21_4:                               # %vector.body
                                        # =>This Inner Loop Header: Depth=1
	add 21, 4, 6
	lvx128 14, 4, 6
	li 20, 127
	addi 22, 22, -32
	lvx128 0, 21, 8
	cmplwi	22, 0
	lvx128 1, 21, 9
	lvsl 4, 4, 6
	vperm128 12, 0, 1, 4
	vcmpgtfp128 13, 2, 12
	vsel128 13, 12, 2, 13
	lvx128 6, 21, 10
	vcmpgtfp128 12, 13, 3
	vsel128 12, 13, 3, 12
	lvx128 7, 21, 11
	vperm128 1, 1, 6, 4
	lvx128 8, 21, 12
	lvx128 9, 21, 0
	lvx128 10, 21, 30
	lvx128 11, 21, 20
	add 21, 3, 6
	li 20, 28
	lvsr 5, 21, 8
	vperm128 5, 12, 12, 5
	stvewx 5, 21, 20
	li 20, 24
	stvewx 5, 21, 20
	li 20, 20
	stvewx 5, 21, 20
	li 20, 44
	stvewx 5, 21, 8
	vcmpgtfp128 5, 2, 1
	vsel128 5, 1, 2, 5
	vcmpgtfp128 12, 5, 3
	lvsr 1, 21, 9
	vsel128 12, 5, 3, 12
	vperm128 5, 12, 12, 1
	stvewx 5, 21, 20
	li 20, 40
	vperm128 1, 6, 7, 4
	stvewx 5, 21, 20
	li 20, 36
	stvewx 5, 21, 20
	li 20, 12
	stvewx 5, 21, 9
	vcmpgtfp128 5, 2, 1
	vsel128 5, 1, 2, 5
	vcmpgtfp128 6, 5, 3
	lvsr 1, 21, 10
	vsel128 6, 5, 3, 6
	vperm128 5, 6, 6, 1
	stvewx 5, 21, 19
	vperm128 1, 7, 8, 4
	stvewx 5, 21, 18
	stvewx 5, 21, 17
	stvewx 5, 21, 10
	vcmpgtfp128 5, 2, 1
	vsel128 5, 1, 2, 5
	vcmpgtfp128 6, 5, 3
	lvsr 1, 21, 11
	vsel128 6, 5, 3, 6
	vperm128 5, 6, 6, 1
	stvewx 5, 21, 16
	vperm128 1, 8, 9, 4
	stvewx 5, 21, 15
	stvewx 5, 21, 14
	stvewx 5, 21, 11
	vcmpgtfp128 5, 2, 1
	vsel128 5, 1, 2, 5
	vcmpgtfp128 6, 5, 3
	lvsr 1, 21, 12
	vsel128 6, 5, 3, 6
	vperm128 5, 6, 6, 1
	stvewx 5, 21, 31
	vperm128 1, 9, 10, 4
	stvewx 5, 21, 5
	stvewx 5, 21, 29
	stvewx 5, 21, 12
	vcmpgtfp128 5, 2, 1
	vsel128 5, 1, 2, 5
	vcmpgtfp128 6, 5, 3
	lvsr 1, 21, 0
	vsel128 6, 5, 3, 6
	vperm128 5, 6, 6, 1
	stvewx 5, 21, 28
	vperm128 1, 10, 11, 4
	stvewx 5, 21, 27
	vperm128 4, 14, 0, 4
	stvewx 5, 21, 26
	stvewx 5, 21, 0
	vcmpgtfp128 5, 2, 1
	vsel128 5, 1, 2, 5
	vcmpgtfp128 6, 5, 3
	lvsr 1, 21, 30
	vsel128 6, 5, 3, 6
	vperm128 5, 6, 6, 1
	stvewx 5, 21, 25
	stvewx 5, 21, 24
	stvewx 5, 21, 23
	stvewx 5, 21, 30
	vcmpgtfp128 5, 2, 4
	vsel128 5, 4, 2, 5
	vcmpgtfp128 0, 5, 3
	lvsr 4, 3, 6
	vsel128 0, 5, 3, 0
	vperm128 4, 0, 0, 4
	stvewx 4, 21, 20
	li 20, 8
	stvewx 4, 21, 20
	li 20, 4
	stvewx 4, 3, 6
	addi 6, 6, 128
	stvewx 4, 21, 20
	bne	0, .Ll3v_BB21_4
# %bb.5:                                # %middle.block
	lwz 0, 28(1)                            # 4-byte Folded Reload
	cmplw	7, 0
	beq	0, .Ll3v_BB21_10
# %bb.6:                                # %vec.epilog.iter.check
	andi. 5, 7, 28
	beq	0, .Ll3v_BB21_11
.Ll3v_BB21_7:                               # %vec.epilog.ph
	stfs 1, 48(1)
	addi 5, 1, 48
	addi 8, 1, 32
	mr	9, 0
	rlwinm 0, 7, 0, 1, 29
	lvx128 2, 0, 5
	li 5, 15
	li 6, 12
	li 10, 8
	li 11, 4
	stfs 2, 32(1)
	vspltw128 2, 2, 0
	lvx128 3, 0, 8
	sub	8, 9, 0
	slwi 9, 9, 2
	vspltw128 3, 3, 0
.Ll3v_BB21_8:                               # %vec.epilog.vector.body
                                        # =>This Inner Loop Header: Depth=1
	add 12, 4, 9
	lvx128 4, 4, 9
	addi 8, 8, 4
	cmplwi	8, 0
	lvx128 5, 12, 5
	add 12, 3, 9
	lvsl 0, 4, 9
	vperm128 4, 4, 5, 0
	vcmpgtfp128 5, 2, 4
	vsel128 5, 4, 2, 5
	lvsr 4, 3, 9
	vcmpgtfp128 0, 5, 3
	vsel128 0, 5, 3, 0
	vperm128 4, 0, 0, 4
	stvewx 4, 3, 9
	addi 9, 9, 16
	stvewx 4, 12, 6
	stvewx 4, 12, 10
	stvewx 4, 12, 11
	bne	0, .Ll3v_BB21_8
# %bb.9:                                # %vec.epilog.middle.block
	cmplw	7, 0
	bne	0, .Ll3v_BB21_11
.Ll3v_BB21_10:
	ld 31, 240(1)                           # 8-byte Folded Reload
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
	ld 14, 104(1)                           # 8-byte Folded Reload
	addi 1, 1, 256
	blr
.Ll3v_BB21_11:                              # %for.body.preheader
	slwi 6, 0, 2
	sub	5, 7, 0
	addi 6, 6, -4
	add 4, 4, 6
	add 3, 3, 6
	b .Ll3v_BB21_13
.Ll3v_BB21_12:                              # %for.body
                                        #   in Loop: Header=BB21_13 Depth=1
	addi 5, 5, -1
	stfsu 0, 4(3)
	cmplwi	5, 0
	beq	0, .Ll3v_BB21_10
.Ll3v_BB21_13:                              # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lfsu 0, 4(4)
	fmr 3, 1
	fcmpu 0, 0, 1
	bge 0, .Ll3v_BB21_15
# %bb.14:                               # %for.body
                                        #   in Loop: Header=BB21_13 Depth=1
	fcmpu 0, 3, 2
	fmr 0, 2
	bgt	0, .Ll3v_BB21_12
	b .Ll3v_BB21_16
.Ll3v_BB21_15:                              # %for.body
                                        #   in Loop: Header=BB21_13 Depth=1
	fmr 3, 0
	fcmpu 0, 3, 2
	fmr 0, 2
	bgt	0, .Ll3v_BB21_12
.Ll3v_BB21_16:                              # %for.body
                                        #   in Loop: Header=BB21_13 Depth=1
	fmr 0, 3
	b .Ll3v_BB21_12
.Ll3v_func_end21:
	.size	l3v_k_clamp_f32_r, .Ll3v_func_end21-.Ll3v_func_begin21
                                        # -- End function
	.section	.rodata.cst16,"aM",@progbits,16
	.p2align	4, 0x0                          # -- Begin function l3v_k_clamp_i16_r
.Ll3v_CPI22_0:
	.short	64536                           # 0xfc18
	.short	64536                           # 0xfc18
	.short	64536                           # 0xfc18
	.short	64536                           # 0xfc18
	.short	64536                           # 0xfc18
	.short	64536                           # 0xfc18
	.short	64536                           # 0xfc18
	.short	64536                           # 0xfc18
.Ll3v_CPI22_1:
	.short	1000                            # 0x3e8
	.short	1000                            # 0x3e8
	.short	1000                            # 0x3e8
	.short	1000                            # 0x3e8
	.short	1000                            # 0x3e8
	.short	1000                            # 0x3e8
	.short	1000                            # 0x3e8
	.short	1000                            # 0x3e8
	.section	.text.l3v_k_clamp_i16_r,"ax",@progbits
	.globl	l3v_k_clamp_i16_r
	.p2align	2
	.type	l3v_k_clamp_i16_r,@function
l3v_k_clamp_i16_r:                      # @l3v_k_clamp_i16_r
.Ll3v_func_begin22:
# %bb.0:                                # %entry
	cmpwi	5, 1
	bltlr	0
# %bb.1:                                # %iter.check
	stwu 1, -176(1)
	std 27, 128(1)                          # 8-byte Folded Spill
	cmplwi	5, 8
	li 27, 0
	std 14, 24(1)                           # 8-byte Folded Spill
	std 15, 32(1)                           # 8-byte Folded Spill
	std 16, 40(1)                           # 8-byte Folded Spill
	std 17, 48(1)                           # 8-byte Folded Spill
	std 18, 56(1)                           # 8-byte Folded Spill
	std 19, 64(1)                           # 8-byte Folded Spill
	std 20, 72(1)                           # 8-byte Folded Spill
	std 21, 80(1)                           # 8-byte Folded Spill
	std 22, 88(1)                           # 8-byte Folded Spill
	std 23, 96(1)                           # 8-byte Folded Spill
	std 24, 104(1)                          # 8-byte Folded Spill
	std 25, 112(1)                          # 8-byte Folded Spill
	std 26, 120(1)                          # 8-byte Folded Spill
	std 28, 136(1)                          # 8-byte Folded Spill
	std 29, 144(1)                          # 8-byte Folded Spill
	std 30, 152(1)                          # 8-byte Folded Spill
	std 31, 160(1)                          # 8-byte Folded Spill
	blt	0, .Ll3v_BB22_11
# %bb.2:                                # %vector.main.loop.iter.check
	cmplwi	5, 64
	blt	0, .Ll3v_BB22_7
# %bb.3:                                # %vector.ph
	li 28, .Ll3v_CPI22_0@l
	lis 6, .Ll3v_CPI22_0@ha
	stw 5, 20(1)                            # 4-byte Folded Spill
	li 27, .Ll3v_CPI22_1@l
	lis 29, .Ll3v_CPI22_1@ha
	lvx128 2, 6, 28
	rlwinm 26, 5, 0, 1, 25
	li 7, 0
	lvx128 3, 29, 27
	li 8, 16
	li 9, 32
	li 10, 48
	li 11, 64
	stw 26, 16(1)                           # 4-byte Folded Spill
	li 12, 80
	li 0, 96
	li 30, 112
	li 24, 84
	li 23, 82
	li 22, 110
	li 21, 108
	li 20, 106
	li 19, 104
	li 18, 102
	li 17, 100
	li 5, 98
	li 16, 126
	li 15, 124
	li 14, 122
	li 31, 120
	li 6, 118
	li 29, 116
	li 28, 114
	mr	27, 26
.Ll3v_BB22_4:                               # %vector.body
                                        # =>This Inner Loop Header: Depth=1
	add 26, 4, 7
	lvx128 14, 4, 7
	li 25, 127
	addi 27, 27, -64
	lvx128 1, 26, 8
	cmplwi	27, 0
	lvx128 6, 26, 9
	lvsl 4, 4, 7
	vperm128 11, 1, 6, 4
	vmaxsh 11, 11, 2
	lvx128 7, 26, 10
	vperm128 1, 14, 1, 4
	lvx128 8, 26, 11
	vperm128 6, 6, 7, 4
	lvx128 9, 26, 12
	vminsh 11, 11, 3
	lvx128 10, 26, 0
	lvx128 12, 26, 30
	lvx128 13, 26, 25
	add 26, 3, 7
	li 25, 30
	vmaxsh 6, 6, 2
	lvsr 0, 26, 8
	vperm128 0, 11, 11, 0
	stvehx 0, 26, 25
	li 25, 28
	vminsh 6, 6, 3
	stvehx 0, 26, 25
	li 25, 26
	stvehx 0, 26, 25
	li 25, 24
	stvehx 0, 26, 25
	li 25, 22
	vmaxsh 1, 1, 2
	stvehx 0, 26, 25
	li 25, 20
	stvehx 0, 26, 25
	li 25, 18
	stvehx 0, 26, 25
	li 25, 46
	vminsh 1, 1, 3
	stvehx 0, 26, 8
	lvsr 0, 26, 9
	vperm128 0, 6, 6, 0
	stvehx 0, 26, 25
	li 25, 44
	vperm128 6, 7, 8, 4
	vmaxsh 6, 6, 2
	stvehx 0, 26, 25
	li 25, 42
	stvehx 0, 26, 25
	li 25, 40
	stvehx 0, 26, 25
	li 25, 38
	vminsh 6, 6, 3
	stvehx 0, 26, 25
	li 25, 36
	stvehx 0, 26, 25
	li 25, 34
	stvehx 0, 26, 25
	li 25, 62
	stvehx 0, 26, 9
	lvsr 0, 26, 10
	vperm128 0, 6, 6, 0
	stvehx 0, 26, 25
	li 25, 60
	vperm128 6, 8, 9, 4
	vmaxsh 6, 6, 2
	stvehx 0, 26, 25
	li 25, 58
	stvehx 0, 26, 25
	li 25, 56
	stvehx 0, 26, 25
	li 25, 54
	vminsh 6, 6, 3
	stvehx 0, 26, 25
	li 25, 52
	stvehx 0, 26, 25
	li 25, 50
	stvehx 0, 26, 25
	li 25, 78
	stvehx 0, 26, 10
	lvsr 0, 26, 11
	vperm128 0, 6, 6, 0
	stvehx 0, 26, 25
	li 25, 76
	vperm128 6, 9, 10, 4
	vmaxsh 6, 6, 2
	stvehx 0, 26, 25
	li 25, 74
	stvehx 0, 26, 25
	li 25, 72
	stvehx 0, 26, 25
	li 25, 70
	vminsh 6, 6, 3
	stvehx 0, 26, 25
	li 25, 68
	stvehx 0, 26, 25
	li 25, 66
	stvehx 0, 26, 25
	li 25, 94
	stvehx 0, 26, 11
	lvsr 0, 26, 12
	vperm128 0, 6, 6, 0
	stvehx 0, 26, 25
	li 25, 92
	vperm128 6, 10, 12, 4
	vmaxsh 6, 6, 2
	stvehx 0, 26, 25
	li 25, 90
	vperm128 4, 12, 13, 4
	stvehx 0, 26, 25
	li 25, 88
	lvsr 5, 3, 7
	vperm128 5, 1, 1, 5
	vminsh 6, 6, 3
	stvehx 0, 26, 25
	li 25, 86
	stvehx 0, 26, 25
	li 25, 14
	stvehx 5, 26, 25
	li 25, 12
	vmaxsh 4, 4, 2
	stvehx 5, 26, 25
	li 25, 10
	stvehx 0, 26, 24
	vminsh 4, 4, 3
	stvehx 0, 26, 23
	stvehx 0, 26, 12
	lvsr 0, 26, 0
	vperm128 0, 6, 6, 0
	stvehx 5, 26, 25
	li 25, 8
	stvehx 5, 26, 25
	li 25, 6
	stvehx 0, 26, 22
	stvehx 0, 26, 21
	stvehx 0, 26, 20
	stvehx 0, 26, 19
	stvehx 0, 26, 18
	stvehx 0, 26, 17
	stvehx 0, 26, 5
	stvehx 0, 26, 0
	lvsr 0, 26, 30
	vperm128 4, 4, 4, 0
	stvehx 5, 26, 25
	li 25, 4
	stvehx 5, 26, 25
	li 25, 2
	stvehx 5, 3, 7
	addi 7, 7, 128
	stvehx 5, 26, 25
	stvehx 4, 26, 16
	stvehx 4, 26, 15
	stvehx 4, 26, 14
	stvehx 4, 26, 31
	stvehx 4, 26, 6
	stvehx 4, 26, 29
	stvehx 4, 26, 28
	stvehx 4, 26, 30
	bne	0, .Ll3v_BB22_4
# %bb.5:                                # %middle.block
	lwz 5, 20(1)                            # 4-byte Folded Reload
	lwz 27, 16(1)                           # 4-byte Folded Reload
	cmplw	5, 27
	beq	0, .Ll3v_BB22_10
# %bb.6:                                # %vec.epilog.iter.check
	andi. 6, 5, 56
	beq	0, .Ll3v_BB22_11
.Ll3v_BB22_7:                               # %vec.epilog.ph
	li 11, .Ll3v_CPI22_0@l
	lis 30, .Ll3v_CPI22_0@ha
	lvx128 2, 30, 11
	li 0, .Ll3v_CPI22_1@l
	lis 29, .Ll3v_CPI22_1@ha
	lvx128 3, 29, 0
	mr	12, 27
	rlwinm 27, 5, 0, 1, 28
	li 28, 15
	li 6, 14
	li 7, 12
	li 8, 10
	li 9, 8
	li 10, 6
	sub	11, 12, 27
	slwi 12, 12, 1
	li 0, 4
	li 30, 2
.Ll3v_BB22_8:                               # %vec.epilog.vector.body
                                        # =>This Inner Loop Header: Depth=1
	add 29, 4, 12
	lvx128 4, 4, 12
	addi 11, 11, 8
	cmplwi	11, 0
	lvx128 5, 29, 28
	add 29, 3, 12
	lvsl 0, 4, 12
	vperm128 4, 4, 5, 0
	vmaxsh 4, 4, 2
	lvsr 5, 3, 12
	vminsh 4, 4, 3
	vperm128 4, 4, 4, 5
	stvehx 4, 3, 12
	addi 12, 12, 16
	stvehx 4, 29, 6
	stvehx 4, 29, 7
	stvehx 4, 29, 8
	stvehx 4, 29, 9
	stvehx 4, 29, 10
	stvehx 4, 29, 0
	stvehx 4, 29, 30
	bne	0, .Ll3v_BB22_8
# %bb.9:                                # %vec.epilog.middle.block
	cmplw	5, 27
	bne	0, .Ll3v_BB22_11
.Ll3v_BB22_10:
	ld 31, 160(1)                           # 8-byte Folded Reload
	ld 30, 152(1)                           # 8-byte Folded Reload
	ld 29, 144(1)                           # 8-byte Folded Reload
	ld 28, 136(1)                           # 8-byte Folded Reload
	ld 27, 128(1)                           # 8-byte Folded Reload
	ld 26, 120(1)                           # 8-byte Folded Reload
	ld 25, 112(1)                           # 8-byte Folded Reload
	ld 24, 104(1)                           # 8-byte Folded Reload
	ld 23, 96(1)                            # 8-byte Folded Reload
	ld 22, 88(1)                            # 8-byte Folded Reload
	ld 21, 80(1)                            # 8-byte Folded Reload
	ld 20, 72(1)                            # 8-byte Folded Reload
	ld 19, 64(1)                            # 8-byte Folded Reload
	ld 18, 56(1)                            # 8-byte Folded Reload
	ld 17, 48(1)                            # 8-byte Folded Reload
	ld 16, 40(1)                            # 8-byte Folded Reload
	ld 15, 32(1)                            # 8-byte Folded Reload
	ld 14, 24(1)                            # 8-byte Folded Reload
	addi 1, 1, 176
	blr
.Ll3v_BB22_11:                              # %for.body.preheader
	slwi 6, 27, 1
	sub	5, 5, 27
	addi 6, 6, -2
	add 4, 4, 6
	add 3, 3, 6
	b .Ll3v_BB22_13
.Ll3v_BB22_12:                              # %for.body
                                        #   in Loop: Header=BB22_13 Depth=1
	addi 5, 5, -1
	sthu 6, 2(3)
	cmplwi	5, 0
	beq	0, .Ll3v_BB22_10
.Ll3v_BB22_13:                              # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lhzu 6, 2(4)
	extsh 6, 6
	cmpwi	6, -1000
	ble 0, .Ll3v_BB22_15
# %bb.14:                               # %for.body
                                        #   in Loop: Header=BB22_13 Depth=1
	cmpwi	6, 1000
	blt	0, .Ll3v_BB22_12
	b .Ll3v_BB22_16
.Ll3v_BB22_15:                              # %for.body
                                        #   in Loop: Header=BB22_13 Depth=1
	li 6, -1000
	cmpwi	6, 1000
	blt	0, .Ll3v_BB22_12
.Ll3v_BB22_16:                              # %for.body
                                        #   in Loop: Header=BB22_13 Depth=1
	li 6, 1000
	b .Ll3v_BB22_12
.Ll3v_func_end22:
	.size	l3v_k_clamp_i16_r, .Ll3v_func_end22-.Ll3v_func_begin22
                                        # -- End function
	.section	.rodata.cst4,"aM",@progbits,4
	.p2align	2, 0x0                          # -- Begin function l3v_k_abs_f32_r
.Ll3v_CPI23_0:
	.long	0x00000000                      # float 0
	.section	.text.l3v_k_abs_f32_r,"ax",@progbits
	.globl	l3v_k_abs_f32_r
	.p2align	2
	.type	l3v_k_abs_f32_r,@function
l3v_k_abs_f32_r:                        # @l3v_k_abs_f32_r
.Ll3v_func_begin23:
# %bb.0:                                # %entry
	cmpwi	5, 1
	bltlr	0
# %bb.1:                                # %for.body.preheader
	li 6, 0
	beq	0, .Ll3v_BB23_9
# %bb.2:                                # %vector.ph
	lis 9, .Ll3v_CPI23_0@ha
	rlwinm 6, 5, 0, 1, 30
	lfs 0, .Ll3v_CPI23_0@l(9)
	addi 7, 4, -8
	addi 8, 3, -8
	mr	9, 6
	b .Ll3v_BB23_4
.Ll3v_BB23_3:                               # %vector.body
                                        #   in Loop: Header=BB23_4 Depth=1
	addi 9, 9, -2
	stfs 1, 4(8)
	cmplwi	9, 0
	beq	0, .Ll3v_BB23_8
.Ll3v_BB23_4:                               # %vector.body
                                        # =>This Inner Loop Header: Depth=1
	lfsu 2, 8(7)
	fcmpu 0, 2, 0
	blt	0, .Ll3v_BB23_6
# %bb.5:                                # %vector.body
                                        #   in Loop: Header=BB23_4 Depth=1
	lfs 1, 4(7)
	fcmpu 0, 1, 0
	stfsu 2, 8(8)
	bge 0, .Ll3v_BB23_3
	b .Ll3v_BB23_7
.Ll3v_BB23_6:                               #   in Loop: Header=BB23_4 Depth=1
	fneg 2, 2
	lfs 1, 4(7)
	fcmpu 0, 1, 0
	stfsu 2, 8(8)
	bge 0, .Ll3v_BB23_3
.Ll3v_BB23_7:                               #   in Loop: Header=BB23_4 Depth=1
	fneg 1, 1
	b .Ll3v_BB23_3
.Ll3v_BB23_8:                               # %middle.block
	cmplw	5, 6
	beqlr	0
.Ll3v_BB23_9:                               # %for.body.preheader10
	lis 7, .Ll3v_CPI23_0@ha
	sub	5, 5, 6
	lfs 0, .Ll3v_CPI23_0@l(7)
	slwi 6, 6, 2
	addi 6, 6, -4
	add 4, 4, 6
	add 3, 3, 6
	b .Ll3v_BB23_11
.Ll3v_BB23_10:                              # %for.body
                                        #   in Loop: Header=BB23_11 Depth=1
	addi 5, 5, -1
	stfsu 1, 4(3)
	cmplwi	5, 0
	beqlr	0
.Ll3v_BB23_11:                              # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lfsu 1, 4(4)
	fcmpu 0, 1, 0
	bge 0, .Ll3v_BB23_10
# %bb.12:                               #   in Loop: Header=BB23_11 Depth=1
	fneg 1, 1
	b .Ll3v_BB23_10
.Ll3v_func_end23:
	.size	l3v_k_abs_f32_r, .Ll3v_func_end23-.Ll3v_func_begin23
                                        # -- End function
	.section	.text.l3v_k_abs_i32_r,"ax",@progbits
	.globl	l3v_k_abs_i32_r                 # -- Begin function l3v_k_abs_i32_r
	.p2align	2
	.type	l3v_k_abs_i32_r,@function
l3v_k_abs_i32_r:                        # @l3v_k_abs_i32_r
.Ll3v_func_begin24:
# %bb.0:                                # %entry
	cmpwi	5, 1
	bltlr	0
# %bb.1:                                # %iter.check
	stwu 1, -176(1)
	cmplwi	5, 4
	li 0, 0
	std 14, 24(1)                           # 8-byte Folded Spill
	std 15, 32(1)                           # 8-byte Folded Spill
	std 16, 40(1)                           # 8-byte Folded Spill
	std 17, 48(1)                           # 8-byte Folded Spill
	std 18, 56(1)                           # 8-byte Folded Spill
	std 19, 64(1)                           # 8-byte Folded Spill
	std 20, 72(1)                           # 8-byte Folded Spill
	std 21, 80(1)                           # 8-byte Folded Spill
	std 22, 88(1)                           # 8-byte Folded Spill
	std 23, 96(1)                           # 8-byte Folded Spill
	std 24, 104(1)                          # 8-byte Folded Spill
	std 25, 112(1)                          # 8-byte Folded Spill
	std 26, 120(1)                          # 8-byte Folded Spill
	std 27, 128(1)                          # 8-byte Folded Spill
	std 28, 136(1)                          # 8-byte Folded Spill
	std 29, 144(1)                          # 8-byte Folded Spill
	std 30, 152(1)                          # 8-byte Folded Spill
	std 31, 160(1)                          # 8-byte Folded Spill
	blt	0, .Ll3v_BB24_10
# %bb.2:                                # %vector.main.loop.iter.check
	cmplwi	5, 32
	blt	0, .Ll3v_BB24_7
# %bb.3:                                # %vector.ph
	rlwinm 22, 5, 0, 1, 26
	li 7, 0
	vspltisw128 8, 0
	stw 22, 20(1)                           # 4-byte Folded Spill
	li 8, 16
	li 9, 32
	li 10, 48
	li 11, 64
	li 12, 80
	li 0, 96
	li 30, 112
	li 19, 60
	li 18, 56
	li 17, 52
	li 16, 76
	li 15, 72
	li 14, 68
	li 31, 92
	li 6, 88
	li 29, 84
	li 28, 108
	li 27, 104
	li 26, 100
	li 25, 124
	li 24, 120
	li 23, 116
.Ll3v_BB24_4:                               # %vector.body
                                        # =>This Inner Loop Header: Depth=1
	add 21, 4, 7
	lvx128 9, 4, 7
	li 20, 127
	addi 22, 22, -32
	lvx128 10, 21, 8
	cmplwi	22, 0
	lvx128 11, 21, 9
	lvsl 3, 4, 7
	vperm128 18, 10, 11, 3
	vsubuwm 19, 8, 18
	lvx128 12, 21, 10
	vmaxsw 18, 18, 19
	vperm128 9, 9, 10, 3
	vsubuwm 10, 8, 9
	lvx128 13, 21, 11
	lvx128 14, 21, 12
	vmaxsw 9, 9, 10
	lvx128 15, 21, 0
	lvx128 16, 21, 30
	lvx128 17, 21, 20
	add 21, 3, 7
	li 20, 28
	lvsr 7, 21, 8
	vperm128 7, 18, 18, 7
	stvewx 7, 21, 20
	li 20, 24
	stvewx 7, 21, 20
	li 20, 20
	stvewx 7, 21, 20
	li 20, 44
	stvewx 7, 21, 8
	vperm128 7, 11, 12, 3
	vsubuwm 11, 8, 7
	vmaxsw 11, 7, 11
	lvsr 2, 21, 9
	vperm128 2, 11, 11, 2
	stvewx 2, 21, 20
	li 20, 40
	stvewx 2, 21, 20
	li 20, 36
	lvsr 4, 3, 7
	vperm128 4, 9, 9, 4
	stvewx 2, 21, 20
	li 20, 12
	stvewx 2, 21, 9
	vperm128 2, 12, 13, 3
	vsubuwm 9, 8, 2
	vmaxsw 2, 2, 9
	lvsr 6, 21, 10
	vperm128 2, 2, 2, 6
	stvewx 2, 21, 19
	stvewx 2, 21, 18
	stvewx 2, 21, 17
	stvewx 2, 21, 10
	vperm128 2, 13, 14, 3
	vsubuwm 6, 8, 2
	vmaxsw 2, 2, 6
	lvsr 1, 21, 11
	vperm128 2, 2, 2, 1
	stvewx 2, 21, 16
	stvewx 2, 21, 15
	stvewx 2, 21, 14
	stvewx 2, 21, 11
	vperm128 2, 14, 15, 3
	vsubuwm 1, 8, 2
	vmaxsw 2, 2, 1
	lvsr 0, 21, 12
	vperm128 2, 2, 2, 0
	lvsr 5, 21, 0
	vperm128 0, 15, 16, 3
	vsubuwm 1, 8, 0
	stvewx 2, 21, 31
	vperm128 3, 16, 17, 3
	stvewx 2, 21, 6
	stvewx 2, 21, 29
	stvewx 2, 21, 12
	vmaxsw 2, 0, 1
	vperm128 2, 2, 2, 5
	vsubuwm 5, 8, 3
	lvsr 7, 21, 30
	stvewx 4, 21, 20
	li 20, 8
	stvewx 2, 21, 28
	stvewx 2, 21, 27
	stvewx 2, 21, 26
	stvewx 2, 21, 0
	vmaxsw 2, 3, 5
	vperm128 2, 2, 2, 7
	stvewx 4, 21, 20
	li 20, 4
	stvewx 4, 3, 7
	addi 7, 7, 128
	stvewx 4, 21, 20
	stvewx 2, 21, 25
	stvewx 2, 21, 24
	stvewx 2, 21, 23
	stvewx 2, 21, 30
	bne	0, .Ll3v_BB24_4
# %bb.5:                                # %middle.block
	lwz 0, 20(1)                            # 4-byte Folded Reload
	cmplw	5, 0
	beq	0, .Ll3v_BB24_12
# %bb.6:                                # %vec.epilog.iter.check
	andi. 6, 5, 28
	beq	0, .Ll3v_BB24_10
.Ll3v_BB24_7:                               # %vec.epilog.ph
	mr	9, 0
	rlwinm 0, 5, 0, 1, 29
	vspltisw128 2, 0
	li 6, 15
	li 7, 12
	sub	8, 9, 0
	slwi 9, 9, 2
	li 10, 8
	li 11, 4
.Ll3v_BB24_8:                               # %vec.epilog.vector.body
                                        # =>This Inner Loop Header: Depth=1
	add 12, 4, 9
	lvx128 3, 4, 9
	addi 8, 8, 4
	cmplwi	8, 0
	lvx128 4, 12, 6
	add 12, 3, 9
	lvsl 5, 4, 9
	vperm128 3, 3, 4, 5
	vsubuwm 5, 2, 3
	lvsr 4, 3, 9
	vmaxsw 3, 3, 5
	vperm128 3, 3, 3, 4
	stvewx 3, 3, 9
	addi 9, 9, 16
	stvewx 3, 12, 7
	stvewx 3, 12, 10
	stvewx 3, 12, 11
	bne	0, .Ll3v_BB24_8
# %bb.9:                                # %vec.epilog.middle.block
	cmplw	5, 0
	beq	0, .Ll3v_BB24_12
.Ll3v_BB24_10:                              # %for.body.preheader
	slwi 6, 0, 2
	sub	5, 5, 0
	addi 6, 6, -4
	add 4, 4, 6
	add 3, 3, 6
.Ll3v_BB24_11:                              # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lwzu 6, 4(4)
	addi 5, 5, -1
	cmplwi	5, 0
	srawi 7, 6, 31
	xor 6, 6, 7
	sub	6, 6, 7
	stwu 6, 4(3)
	bne	0, .Ll3v_BB24_11
.Ll3v_BB24_12:
	ld 31, 160(1)                           # 8-byte Folded Reload
	ld 30, 152(1)                           # 8-byte Folded Reload
	ld 29, 144(1)                           # 8-byte Folded Reload
	ld 28, 136(1)                           # 8-byte Folded Reload
	ld 27, 128(1)                           # 8-byte Folded Reload
	ld 26, 120(1)                           # 8-byte Folded Reload
	ld 25, 112(1)                           # 8-byte Folded Reload
	ld 24, 104(1)                           # 8-byte Folded Reload
	ld 23, 96(1)                            # 8-byte Folded Reload
	ld 22, 88(1)                            # 8-byte Folded Reload
	ld 21, 80(1)                            # 8-byte Folded Reload
	ld 20, 72(1)                            # 8-byte Folded Reload
	ld 19, 64(1)                            # 8-byte Folded Reload
	ld 18, 56(1)                            # 8-byte Folded Reload
	ld 17, 48(1)                            # 8-byte Folded Reload
	ld 16, 40(1)                            # 8-byte Folded Reload
	ld 15, 32(1)                            # 8-byte Folded Reload
	ld 14, 24(1)                            # 8-byte Folded Reload
	addi 1, 1, 176
	blr
.Ll3v_func_end24:
	.size	l3v_k_abs_i32_r, .Ll3v_func_end24-.Ll3v_func_begin24
                                        # -- End function
	.section	.text.l3v_k_f2i_r,"ax",@progbits
	.globl	l3v_k_f2i_r                     # -- Begin function l3v_k_f2i_r
	.p2align	2
	.type	l3v_k_f2i_r,@function
l3v_k_f2i_r:                            # @l3v_k_f2i_r
.Ll3v_func_begin25:
# %bb.0:                                # %entry
	cmpwi	5, 1
	bltlr	0
# %bb.1:                                # %iter.check
	stwu 1, -176(1)
	cmplwi	5, 4
	li 0, 0
	std 14, 24(1)                           # 8-byte Folded Spill
	std 15, 32(1)                           # 8-byte Folded Spill
	std 16, 40(1)                           # 8-byte Folded Spill
	std 17, 48(1)                           # 8-byte Folded Spill
	std 18, 56(1)                           # 8-byte Folded Spill
	std 19, 64(1)                           # 8-byte Folded Spill
	std 20, 72(1)                           # 8-byte Folded Spill
	std 21, 80(1)                           # 8-byte Folded Spill
	std 22, 88(1)                           # 8-byte Folded Spill
	std 23, 96(1)                           # 8-byte Folded Spill
	std 24, 104(1)                          # 8-byte Folded Spill
	std 25, 112(1)                          # 8-byte Folded Spill
	std 26, 120(1)                          # 8-byte Folded Spill
	std 27, 128(1)                          # 8-byte Folded Spill
	std 28, 136(1)                          # 8-byte Folded Spill
	std 29, 144(1)                          # 8-byte Folded Spill
	std 30, 152(1)                          # 8-byte Folded Spill
	std 31, 160(1)                          # 8-byte Folded Spill
	blt	0, .Ll3v_BB25_10
# %bb.2:                                # %vector.main.loop.iter.check
	cmplwi	5, 32
	blt	0, .Ll3v_BB25_7
# %bb.3:                                # %vector.ph
	stw 5, 20(1)                            # 4-byte Folded Spill
	rlwinm 23, 5, 0, 1, 26
	li 7, 0
	li 8, 16
	li 9, 32
	stw 23, 16(1)                           # 4-byte Folded Spill
	li 10, 48
	li 11, 64
	li 12, 80
	li 0, 96
	li 30, 112
	li 20, 36
	li 19, 60
	li 18, 56
	li 17, 52
	li 16, 76
	li 15, 72
	li 14, 68
	li 31, 92
	li 6, 88
	li 29, 84
	li 28, 108
	li 27, 104
	li 26, 100
	li 25, 124
	li 5, 120
	li 24, 116
.Ll3v_BB25_4:                               # %vector.body
                                        # =>This Inner Loop Header: Depth=1
	add 22, 4, 7
	lvx128 3, 4, 7
	li 21, 127
	addi 23, 23, -32
	lvx128 4, 22, 8
	cmplwi	23, 0
	lvx128 5, 22, 9
	lvsl 2, 4, 7
	vperm128 3, 3, 4, 2
	lvx128 0, 22, 10
	vperm128 4, 4, 5, 2
	lvx128 1, 22, 11
	vperm128 5, 5, 0, 2
	lvx128 6, 22, 12
	vperm128 0, 0, 1, 2
	lvx128 7, 22, 0
	vperm128 1, 1, 6, 2
	lvx128 8, 22, 30
	vperm128 6, 6, 7, 2
	lvx128 9, 22, 21
	add 22, 3, 7
	vcfpsxws128 10, 3, 0
	li 21, 28
	vperm128 7, 7, 8, 2
	lvsr 3, 22, 8
	vperm128 2, 8, 9, 2
	vcfpsxws128 4, 4, 0
	vperm128 3, 4, 4, 3
	vcfpsxws128 5, 5, 0
	vcfpsxws128 0, 0, 0
	vcfpsxws128 1, 1, 0
	vcfpsxws128 6, 6, 0
	vcfpsxws128 7, 7, 0
	vcfpsxws128 2, 2, 0
	stvewx 3, 22, 21
	li 21, 24
	stvewx 3, 22, 21
	li 21, 20
	stvewx 3, 22, 21
	li 21, 44
	stvewx 3, 22, 8
	lvsr 3, 22, 9
	vperm128 3, 5, 5, 3
	stvewx 3, 22, 21
	li 21, 40
	stvewx 3, 22, 21
	li 21, 12
	stvewx 3, 22, 20
	stvewx 3, 22, 9
	lvsr 3, 22, 10
	vperm128 3, 0, 0, 3
	stvewx 3, 22, 19
	stvewx 3, 22, 18
	stvewx 3, 22, 17
	stvewx 3, 22, 10
	lvsr 3, 22, 11
	vperm128 3, 1, 1, 3
	stvewx 3, 22, 16
	stvewx 3, 22, 15
	stvewx 3, 22, 14
	stvewx 3, 22, 11
	lvsr 3, 22, 12
	vperm128 3, 6, 6, 3
	stvewx 3, 22, 31
	stvewx 3, 22, 6
	stvewx 3, 22, 29
	stvewx 3, 22, 12
	lvsr 3, 22, 0
	vperm128 3, 7, 7, 3
	stvewx 3, 22, 28
	stvewx 3, 22, 27
	stvewx 3, 22, 26
	stvewx 3, 22, 0
	lvsr 3, 22, 30
	vperm128 2, 2, 2, 3
	stvewx 2, 22, 25
	stvewx 2, 22, 5
	stvewx 2, 22, 24
	stvewx 2, 22, 30
	lvsr 2, 3, 7
	vperm128 2, 10, 10, 2
	stvewx 2, 22, 21
	li 21, 8
	stvewx 2, 22, 21
	li 21, 4
	stvewx 2, 3, 7
	addi 7, 7, 128
	stvewx 2, 22, 21
	bne	0, .Ll3v_BB25_4
# %bb.5:                                # %middle.block
	lwz 5, 20(1)                            # 4-byte Folded Reload
	lwz 0, 16(1)                            # 4-byte Folded Reload
	cmplw	5, 0
	beq	0, .Ll3v_BB25_12
# %bb.6:                                # %vec.epilog.iter.check
	andi. 6, 5, 28
	beq	0, .Ll3v_BB25_10
.Ll3v_BB25_7:                               # %vec.epilog.ph
	mr	8, 0
	rlwinm 0, 5, 0, 1, 29
	li 12, 15
	li 6, 12
	sub	7, 8, 0
	slwi 8, 8, 2
	li 9, 8
	li 10, 4
.Ll3v_BB25_8:                               # %vec.epilog.vector.body
                                        # =>This Inner Loop Header: Depth=1
	add 11, 4, 8
	lvx128 2, 4, 8
	addi 7, 7, 4
	cmplwi	7, 0
	lvx128 3, 11, 12
	add 11, 3, 8
	lvsl 4, 4, 8
	vperm128 2, 2, 3, 4
	vcfpsxws128 2, 2, 0
	lvsr 3, 3, 8
	vperm128 2, 2, 2, 3
	stvewx 2, 3, 8
	addi 8, 8, 16
	stvewx 2, 11, 6
	stvewx 2, 11, 9
	stvewx 2, 11, 10
	bne	0, .Ll3v_BB25_8
# %bb.9:                                # %vec.epilog.middle.block
	cmplw	5, 0
	beq	0, .Ll3v_BB25_12
.Ll3v_BB25_10:                              # %for.body.preheader
	slwi 6, 0, 2
	sub	5, 5, 0
	addi 6, 6, -4
	add 4, 4, 6
	add 3, 3, 6
.Ll3v_BB25_11:                              # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lfsu 0, 4(4)
	addi 5, 5, -1
	addi 3, 3, 4
	cmplwi	5, 0
	fctiwz 0, 0
	stfiwx 0, 0, 3
	bne	0, .Ll3v_BB25_11
.Ll3v_BB25_12:
	ld 31, 160(1)                           # 8-byte Folded Reload
	ld 30, 152(1)                           # 8-byte Folded Reload
	ld 29, 144(1)                           # 8-byte Folded Reload
	ld 28, 136(1)                           # 8-byte Folded Reload
	ld 27, 128(1)                           # 8-byte Folded Reload
	ld 26, 120(1)                           # 8-byte Folded Reload
	ld 25, 112(1)                           # 8-byte Folded Reload
	ld 24, 104(1)                           # 8-byte Folded Reload
	ld 23, 96(1)                            # 8-byte Folded Reload
	ld 22, 88(1)                            # 8-byte Folded Reload
	ld 21, 80(1)                            # 8-byte Folded Reload
	ld 20, 72(1)                            # 8-byte Folded Reload
	ld 19, 64(1)                            # 8-byte Folded Reload
	ld 18, 56(1)                            # 8-byte Folded Reload
	ld 17, 48(1)                            # 8-byte Folded Reload
	ld 16, 40(1)                            # 8-byte Folded Reload
	ld 15, 32(1)                            # 8-byte Folded Reload
	ld 14, 24(1)                            # 8-byte Folded Reload
	addi 1, 1, 176
	blr
.Ll3v_func_end25:
	.size	l3v_k_f2i_r, .Ll3v_func_end25-.Ll3v_func_begin25
                                        # -- End function
	.section	.text.l3v_k_i2f_r,"ax",@progbits
	.globl	l3v_k_i2f_r                     # -- Begin function l3v_k_i2f_r
	.p2align	2
	.type	l3v_k_i2f_r,@function
l3v_k_i2f_r:                            # @l3v_k_i2f_r
.Ll3v_func_begin26:
# %bb.0:                                # %entry
	cmpwi	5, 1
	bltlr	0
# %bb.1:                                # %iter.check
	stwu 1, -192(1)
	cmplwi	5, 4
	li 0, 0
	std 14, 40(1)                           # 8-byte Folded Spill
	std 15, 48(1)                           # 8-byte Folded Spill
	std 16, 56(1)                           # 8-byte Folded Spill
	std 17, 64(1)                           # 8-byte Folded Spill
	std 18, 72(1)                           # 8-byte Folded Spill
	std 19, 80(1)                           # 8-byte Folded Spill
	std 20, 88(1)                           # 8-byte Folded Spill
	std 21, 96(1)                           # 8-byte Folded Spill
	std 22, 104(1)                          # 8-byte Folded Spill
	std 23, 112(1)                          # 8-byte Folded Spill
	std 24, 120(1)                          # 8-byte Folded Spill
	std 25, 128(1)                          # 8-byte Folded Spill
	std 26, 136(1)                          # 8-byte Folded Spill
	std 27, 144(1)                          # 8-byte Folded Spill
	std 28, 152(1)                          # 8-byte Folded Spill
	std 29, 160(1)                          # 8-byte Folded Spill
	std 30, 168(1)                          # 8-byte Folded Spill
	std 31, 176(1)                          # 8-byte Folded Spill
	blt	0, .Ll3v_BB26_10
# %bb.2:                                # %vector.main.loop.iter.check
	cmplwi	5, 32
	blt	0, .Ll3v_BB26_7
# %bb.3:                                # %vector.ph
	rlwinm 22, 5, 0, 1, 26
	li 7, 0
	stw 5, 28(1)                            # 4-byte Folded Spill
	li 8, 16
	li 9, 32
	stw 22, 24(1)                           # 4-byte Folded Spill
	li 10, 48
	li 11, 64
	li 12, 80
	li 0, 96
	li 30, 112
	li 20, 36
	li 19, 60
	li 18, 56
	li 17, 52
	li 16, 76
	li 15, 72
	li 14, 68
	li 31, 92
	li 6, 88
	li 29, 84
	li 28, 108
	li 27, 104
	li 26, 100
	li 25, 124
	li 24, 120
	li 23, 116
.Ll3v_BB26_4:                               # %vector.body
                                        # =>This Inner Loop Header: Depth=1
	add 5, 4, 7
	lvx128 11, 4, 7
	li 21, 127
	addi 22, 22, -32
	lvx128 4, 5, 8
	cmplwi	22, 0
	lvx128 5, 5, 9
	lvsl 2, 4, 7
	vperm128 10, 4, 5, 2
	vcsxwfp128 10, 10, 0
	lvx128 0, 5, 10
	lvx128 1, 5, 11
	lvx128 6, 5, 12
	lvx128 7, 5, 0
	lvx128 8, 5, 30
	lvx128 9, 5, 21
	add 5, 3, 7
	li 21, 28
	lvsr 3, 5, 8
	vperm128 3, 10, 10, 3
	stvewx 3, 5, 21
	li 21, 24
	stvewx 3, 5, 21
	li 21, 20
	stvewx 3, 5, 21
	li 21, 44
	stvewx 3, 5, 8
	vperm128 3, 5, 0, 2
	vcsxwfp128 3, 3, 0
	lvsr 5, 5, 9
	vperm128 3, 3, 3, 5
	stvewx 3, 5, 21
	li 21, 40
	stvewx 3, 5, 21
	li 21, 12
	stvewx 3, 5, 20
	stvewx 3, 5, 9
	vperm128 3, 0, 1, 2
	vcsxwfp128 3, 3, 0
	lvsr 5, 5, 10
	vperm128 3, 3, 3, 5
	stvewx 3, 5, 19
	stvewx 3, 5, 18
	stvewx 3, 5, 17
	stvewx 3, 5, 10
	vperm128 3, 1, 6, 2
	vcsxwfp128 3, 3, 0
	lvsr 5, 5, 11
	vperm128 3, 3, 3, 5
	stvewx 3, 5, 16
	stvewx 3, 5, 15
	stvewx 3, 5, 14
	stvewx 3, 5, 11
	vperm128 3, 6, 7, 2
	vcsxwfp128 3, 3, 0
	lvsr 5, 5, 12
	vperm128 3, 3, 3, 5
	stvewx 3, 5, 31
	stvewx 3, 5, 6
	stvewx 3, 5, 29
	stvewx 3, 5, 12
	vperm128 3, 7, 8, 2
	vcsxwfp128 3, 3, 0
	lvsr 5, 5, 0
	vperm128 3, 3, 3, 5
	stvewx 3, 5, 28
	stvewx 3, 5, 27
	stvewx 3, 5, 26
	stvewx 3, 5, 0
	vperm128 3, 8, 9, 2
	vcsxwfp128 3, 3, 0
	lvsr 5, 5, 30
	vperm128 2, 11, 4, 2
	vperm128 3, 3, 3, 5
	stvewx 3, 5, 25
	vcsxwfp128 2, 2, 0
	stvewx 3, 5, 24
	stvewx 3, 5, 23
	stvewx 3, 5, 30
	lvsr 3, 3, 7
	vperm128 2, 2, 2, 3
	stvewx 2, 5, 21
	li 21, 8
	stvewx 2, 5, 21
	li 21, 4
	stvewx 2, 3, 7
	addi 7, 7, 128
	stvewx 2, 5, 21
	bne	0, .Ll3v_BB26_4
# %bb.5:                                # %middle.block
	lwz 5, 28(1)                            # 4-byte Folded Reload
	lwz 0, 24(1)                            # 4-byte Folded Reload
	cmplw	5, 0
	beq	0, .Ll3v_BB26_12
# %bb.6:                                # %vec.epilog.iter.check
	andi. 6, 5, 28
	beq	0, .Ll3v_BB26_10
.Ll3v_BB26_7:                               # %vec.epilog.ph
	mr	8, 0
	rlwinm 0, 5, 0, 1, 29
	li 12, 15
	li 6, 12
	sub	7, 8, 0
	slwi 8, 8, 2
	li 9, 8
	li 10, 4
.Ll3v_BB26_8:                               # %vec.epilog.vector.body
                                        # =>This Inner Loop Header: Depth=1
	add 11, 4, 8
	lvx128 2, 4, 8
	addi 7, 7, 4
	cmplwi	7, 0
	lvx128 3, 11, 12
	add 11, 3, 8
	lvsl 4, 4, 8
	vperm128 2, 2, 3, 4
	vcsxwfp128 2, 2, 0
	lvsr 3, 3, 8
	vperm128 2, 2, 2, 3
	stvewx 2, 3, 8
	addi 8, 8, 16
	stvewx 2, 11, 6
	stvewx 2, 11, 9
	stvewx 2, 11, 10
	bne	0, .Ll3v_BB26_8
# %bb.9:                                # %vec.epilog.middle.block
	cmplw	5, 0
	beq	0, .Ll3v_BB26_12
.Ll3v_BB26_10:                              # %for.body.preheader
	slwi 6, 0, 2
	sub	5, 5, 0
	addi 6, 6, -4
	add 4, 4, 6
	add 3, 3, 6
.Ll3v_BB26_11:                              # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lwa 6, 4(4)
	addi 5, 5, -1
	addi 4, 4, 4
	cmplwi	5, 0
	std 6, 32(1)
	lfd 0, 32(1)
	fcfid 0, 0
	frsp 0, 0
	stfsu 0, 4(3)
	bne	0, .Ll3v_BB26_11
.Ll3v_BB26_12:
	ld 31, 176(1)                           # 8-byte Folded Reload
	ld 30, 168(1)                           # 8-byte Folded Reload
	ld 29, 160(1)                           # 8-byte Folded Reload
	ld 28, 152(1)                           # 8-byte Folded Reload
	ld 27, 144(1)                           # 8-byte Folded Reload
	ld 26, 136(1)                           # 8-byte Folded Reload
	ld 25, 128(1)                           # 8-byte Folded Reload
	ld 24, 120(1)                           # 8-byte Folded Reload
	ld 23, 112(1)                           # 8-byte Folded Reload
	ld 22, 104(1)                           # 8-byte Folded Reload
	ld 21, 96(1)                            # 8-byte Folded Reload
	ld 20, 88(1)                            # 8-byte Folded Reload
	ld 19, 80(1)                            # 8-byte Folded Reload
	ld 18, 72(1)                            # 8-byte Folded Reload
	ld 17, 64(1)                            # 8-byte Folded Reload
	ld 16, 56(1)                            # 8-byte Folded Reload
	ld 15, 48(1)                            # 8-byte Folded Reload
	ld 14, 40(1)                            # 8-byte Folded Reload
	addi 1, 1, 192
	blr
.Ll3v_func_end26:
	.size	l3v_k_i2f_r, .Ll3v_func_end26-.Ll3v_func_begin26
                                        # -- End function
	.section	.rodata.cst16,"aM",@progbits,16
	.p2align	4, 0x0                          # -- Begin function l3v_k_u8_to_f32_r
.Ll3v_CPI27_0:
	.long	0x3b808081                      # float 0.00392156886
	.long	0x3b808081                      # float 0.00392156886
	.long	0x3b808081                      # float 0.00392156886
	.long	0x3b808081                      # float 0.00392156886
	.section	.rodata.cst4,"aM",@progbits,4
	.p2align	2, 0x0
.Ll3v_CPI27_1:
	.long	0x3b808081                      # float 0.00392156886
	.section	.text.l3v_k_u8_to_f32_r,"ax",@progbits
	.globl	l3v_k_u8_to_f32_r
	.p2align	2
	.type	l3v_k_u8_to_f32_r,@function
l3v_k_u8_to_f32_r:                      # @l3v_k_u8_to_f32_r
.Ll3v_func_begin27:
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
	blt	0, .Ll3v_BB27_5
# %bb.2:                                # %vector.ph
	li 9, .Ll3v_CPI27_0@l
	lis 10, .Ll3v_CPI27_0@ha
	lvx128 2, 10, 9
	rlwinm 6, 5, 0, 1, 29
	addi 7, 1, 64
	li 8, 12
	li 9, 8
	li 10, 4
	mr	11, 4
	mr	12, 3
	mr	30, 6
.Ll3v_BB27_3:                               # %vector.body
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
	lvsr 4, 0, 12
	frsp 0, 0
	stfs 0, 76(1)
	lvx128 3, 0, 7
	vmulfp128 3, 3, 2
	vperm128 3, 3, 3, 4
	stvewx 3, 12, 8
	stvewx 3, 12, 9
	stvewx 3, 12, 10
	stvewx 3, 0, 12
	addi 12, 12, 16
	bne	0, .Ll3v_BB27_3
# %bb.4:                                # %middle.block
	cmplw	5, 6
	beq	0, .Ll3v_BB27_7
.Ll3v_BB27_5:                               # %for.body.preheader7
	sub	5, 5, 6
	slwi 7, 6, 2
	add 4, 6, 4
	lis 6, .Ll3v_CPI27_1@ha
	lfs 0, .Ll3v_CPI27_1@l(6)
	add 7, 7, 3
	addi 3, 4, -1
	addi 4, 7, -4
.Ll3v_BB27_6:                               # %for.body
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
	bne	0, .Ll3v_BB27_6
.Ll3v_BB27_7:
	ld 30, 104(1)                           # 8-byte Folded Reload
	ld 29, 96(1)                            # 8-byte Folded Reload
	ld 28, 88(1)                            # 8-byte Folded Reload
	ld 27, 80(1)                            # 8-byte Folded Reload
	addi 1, 1, 128
	blr
.Ll3v_func_end27:
	.size	l3v_k_u8_to_f32_r, .Ll3v_func_end27-.Ll3v_func_begin27
                                        # -- End function
	.section	.rodata.cst16,"aM",@progbits,16
	.p2align	4, 0x0                          # -- Begin function l3v_k_mat4_vec4_r
.Ll3v_CPI28_0:
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
.Ll3v_CPI28_1:
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
.Ll3v_CPI28_2:
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
.Ll3v_CPI28_3:
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
.Ll3v_CPI28_4:
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
	.section	.text.l3v_k_mat4_vec4_r,"ax",@progbits
	.globl	l3v_k_mat4_vec4_r
	.p2align	2
	.type	l3v_k_mat4_vec4_r,@function
l3v_k_mat4_vec4_r:                      # @l3v_k_mat4_vec4_r
.Ll3v_func_begin28:
# %bb.0:                                # %entry
	cmpwi	6, 1
	bltlr	0
# %bb.1:                                # %for.body.lr.ph
	stwu 1, -192(1)
	std 19, 80(1)                           # 8-byte Folded Spill
	li 7, 16
	li 8, 32
	li 9, 48
	li 10, 63
	std 20, 88(1)                           # 8-byte Folded Spill
	cmplwi	6, 4
	std 21, 96(1)                           # 8-byte Folded Spill
	std 22, 104(1)                          # 8-byte Folded Spill
	std 23, 112(1)                          # 8-byte Folded Spill
	std 24, 120(1)                          # 8-byte Folded Spill
	std 25, 128(1)                          # 8-byte Folded Spill
	std 26, 136(1)                          # 8-byte Folded Spill
	std 27, 144(1)                          # 8-byte Folded Spill
	std 28, 152(1)                          # 8-byte Folded Spill
	std 29, 160(1)                          # 8-byte Folded Spill
	std 30, 168(1)                          # 8-byte Folded Spill
	lvx128 3, 0, 4
	lvx128 4, 4, 7
	lvx128 5, 4, 8
	lvx128 0, 4, 9
	lvx128 1, 4, 10
	lvsl 2, 0, 4
	li 4, 0
	vperm128 27, 3, 4, 2
	vperm128 26, 4, 5, 2
	vperm128 35, 5, 0, 2
	vperm128 5, 0, 1, 2
	blt	0, .Ll3v_BB28_5
# %bb.2:                                # %vector.ph
	li 12, .Ll3v_CPI28_0@l
	lis 30, .Ll3v_CPI28_0@ha
	vspltw128 16, 27, 0
	lvx128 0, 30, 12
	li 29, .Ll3v_CPI28_1@l
	lis 28, .Ll3v_CPI28_1@ha
	vspltw128 18, 26, 0
	li 26, .Ll3v_CPI28_2@l
	lis 25, .Ll3v_CPI28_2@ha
	lvx128 1, 28, 29
	li 24, .Ll3v_CPI28_3@l
	lis 23, .Ll3v_CPI28_3@ha
	vspltw128 31, 35, 0
	lvx128 6, 25, 26
	li 22, .Ll3v_CPI28_4@l
	lis 21, .Ll3v_CPI28_4@ha
	vspltw128 29, 5, 0
	lvx128 7, 23, 24
	rlwinm 4, 6, 0, 1, 29
	li 11, 0
	vspltw128 8, 27, 1
	li 12, 60
	li 0, 56
	lvx128 2, 21, 22
	li 30, 52
	li 29, 44
	vspltw128 9, 26, 1
	li 28, 40
	li 27, 36
	vspltw128 10, 35, 1
	li 26, 28
	li 25, 24
	li 24, 20
	li 23, 12
	vspltw128 11, 5, 1
	li 22, 8
	li 21, 4
	vspltw128 12, 27, 2
	mr	20, 4
	vspltw128 13, 26, 2
	vspltw128 14, 35, 2
	vspltw128 15, 5, 2
	vspltw128 17, 27, 3
	vspltw128 19, 26, 3
	vspltw128 30, 35, 3
	vspltw128 28, 5, 3
.Ll3v_BB28_3:                               # %vector.body
                                        # =>This Inner Loop Header: Depth=1
	add 19, 5, 11
	lvsl 3, 5, 11
	addi 20, 20, -4
	cmplwi	20, 0
	lvx128 25, 19, 8
	lvx128 24, 19, 7
	lvx128 23, 5, 11
	vperm128 21, 24, 25, 3
	lvx128 22, 19, 10
	vperm128 24, 23, 24, 3
	lvx128 20, 19, 9
	vmrghw128 34, 24, 21
	add 19, 3, 11
	vperm128 23, 20, 22, 3
	lvsr 4, 19, 9
	vperm128 3, 25, 20, 3
	vperm128 25, 24, 21, 6
	vmrghw128 22, 3, 23
	vperm128 25, 25, 22, 1
	vmulfp128 22, 25, 18
	vperm128 33, 3, 23, 0
	vperm128 33, 34, 33, 1
	vmulfp128 20, 25, 9
	vmrglw128 34, 24, 21
	vperm128 24, 24, 21, 2
	vmulfp128 32, 25, 13
	vmulfp128 25, 25, 19
	vmaddfp128 22, 16, 33, 22
	vmaddfp128 20, 8, 33, 20
	vmaddfp128 32, 12, 33, 32
	vmaddfp128 25, 17, 33, 25
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
	vperm128 24, 22, 20, 2
	vmaddfp128 32, 15, 3, 32
	vmaddfp128 25, 28, 3, 25
	vmrglw128 3, 32, 25
	vperm128 3, 24, 3, 1
	vperm128 3, 3, 3, 4
	stvewx 3, 19, 12
	vperm128 4, 32, 25, 7
	stvewx 3, 19, 0
	vmrglw128 24, 22, 20
	stvewx 3, 19, 30
	stvewx 3, 19, 9
	vperm128 3, 24, 4, 1
	lvsr 4, 19, 8
	vmrghw128 24, 32, 25
	vperm128 3, 3, 3, 4
	stvewx 3, 19, 29
	vperm128 4, 22, 20, 6
	stvewx 3, 19, 28
	stvewx 3, 19, 27
	stvewx 3, 19, 8
	vperm128 3, 4, 24, 1
	lvsr 4, 19, 7
	vperm128 3, 3, 3, 4
	stvewx 3, 19, 26
	vperm128 4, 32, 25, 0
	stvewx 3, 19, 25
	vmrghw128 25, 22, 20
	stvewx 3, 19, 24
	stvewx 3, 19, 7
	vperm128 3, 25, 4, 1
	lvsr 4, 3, 11
	vperm128 3, 3, 3, 4
	stvewx 3, 3, 11
	addi 11, 11, 64
	stvewx 3, 19, 23
	stvewx 3, 19, 22
	stvewx 3, 19, 21
	bne	0, .Ll3v_BB28_3
# %bb.4:                                # %middle.block
	cmplw	6, 4
	beq	0, .Ll3v_BB28_7
.Ll3v_BB28_5:                               # %for.body.preheader
	slwi 11, 4, 4
	sub	6, 6, 4
	add 5, 11, 5
	addi 4, 1, 32
	addi 7, 1, 16
	addi 8, 1, 48
	addi 9, 1, 64
	li 10, 12
	add 3, 3, 11
	addi 5, 5, -16
	li 11, 8
	li 12, 4
.Ll3v_BB28_6:                               # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lwzu 0, 16(5)
	addi 6, 6, -1
	cmplwi	6, 0
	lwz 30, 4(5)
	lwz 29, 8(5)
	lwz 28, 12(5)
	stw 30, 32(1)
	lvx128 2, 0, 4
	stw 0, 16(1)
	vspltw128 2, 2, 0
	vmulfp128 2, 2, 26
	lvx128 3, 0, 7
	vspltw128 3, 3, 0
	stw 29, 48(1)
	vmaddfp128 2, 27, 3, 2
	lvx128 3, 0, 8
	vspltw128 3, 3, 0
	stw 28, 64(1)
	lvsr 4, 0, 3
	vmaddfp128 2, 35, 3, 2
	lvx128 3, 0, 9
	vspltw128 3, 3, 0
	vmaddfp128 2, 5, 3, 2
	vperm128 2, 2, 2, 4
	stvewx 2, 3, 10
	stvewx 2, 3, 11
	stvewx 2, 3, 12
	stvewx 2, 0, 3
	addi 3, 3, 16
	bne	0, .Ll3v_BB28_6
.Ll3v_BB28_7:
	ld 30, 168(1)                           # 8-byte Folded Reload
	ld 29, 160(1)                           # 8-byte Folded Reload
	ld 28, 152(1)                           # 8-byte Folded Reload
	ld 27, 144(1)                           # 8-byte Folded Reload
	ld 26, 136(1)                           # 8-byte Folded Reload
	ld 25, 128(1)                           # 8-byte Folded Reload
	ld 24, 120(1)                           # 8-byte Folded Reload
	ld 23, 112(1)                           # 8-byte Folded Reload
	ld 22, 104(1)                           # 8-byte Folded Reload
	ld 21, 96(1)                            # 8-byte Folded Reload
	ld 20, 88(1)                            # 8-byte Folded Reload
	ld 19, 80(1)                            # 8-byte Folded Reload
	addi 1, 1, 192
	blr
.Ll3v_func_end28:
	.size	l3v_k_mat4_vec4_r, .Ll3v_func_end28-.Ll3v_func_begin28
                                        # -- End function
	.section	.text.l3v_k_copy_u32_r,"ax",@progbits
	.globl	l3v_k_copy_u32_r                # -- Begin function l3v_k_copy_u32_r
	.p2align	2
	.type	l3v_k_copy_u32_r,@function
l3v_k_copy_u32_r:                       # @l3v_k_copy_u32_r
.Ll3v_func_begin29:
# %bb.0:                                # %entry
	cmpwi	5, 1
	bltlr	0
# %bb.1:                                # %iter.check
	stwu 1, -176(1)
	cmplwi	5, 4
	li 0, 0
	std 14, 24(1)                           # 8-byte Folded Spill
	std 15, 32(1)                           # 8-byte Folded Spill
	std 16, 40(1)                           # 8-byte Folded Spill
	std 17, 48(1)                           # 8-byte Folded Spill
	std 18, 56(1)                           # 8-byte Folded Spill
	std 19, 64(1)                           # 8-byte Folded Spill
	std 20, 72(1)                           # 8-byte Folded Spill
	std 21, 80(1)                           # 8-byte Folded Spill
	std 22, 88(1)                           # 8-byte Folded Spill
	std 23, 96(1)                           # 8-byte Folded Spill
	std 24, 104(1)                          # 8-byte Folded Spill
	std 25, 112(1)                          # 8-byte Folded Spill
	std 26, 120(1)                          # 8-byte Folded Spill
	std 27, 128(1)                          # 8-byte Folded Spill
	std 28, 136(1)                          # 8-byte Folded Spill
	std 29, 144(1)                          # 8-byte Folded Spill
	std 30, 152(1)                          # 8-byte Folded Spill
	std 31, 160(1)                          # 8-byte Folded Spill
	blt	0, .Ll3v_BB29_10
# %bb.2:                                # %vector.main.loop.iter.check
	cmplwi	5, 32
	blt	0, .Ll3v_BB29_7
# %bb.3:                                # %vector.ph
	stw 5, 20(1)                            # 4-byte Folded Spill
	rlwinm 23, 5, 0, 1, 26
	li 7, 0
	li 8, 16
	li 9, 32
	stw 23, 16(1)                           # 4-byte Folded Spill
	li 10, 48
	li 11, 64
	li 12, 80
	li 0, 96
	li 30, 112
	li 20, 36
	li 19, 60
	li 18, 56
	li 17, 52
	li 16, 76
	li 15, 72
	li 14, 68
	li 31, 92
	li 6, 88
	li 29, 84
	li 28, 108
	li 27, 104
	li 5, 100
	li 26, 124
	li 25, 120
	li 24, 116
.Ll3v_BB29_4:                               # %vector.body
                                        # =>This Inner Loop Header: Depth=1
	add 22, 4, 7
	lvx128 11, 4, 7
	li 21, 127
	addi 23, 23, -32
	lvx128 4, 22, 8
	cmplwi	23, 0
	lvx128 5, 22, 9
	lvsl 2, 4, 7
	vperm128 10, 4, 5, 2
	lvx128 0, 22, 10
	lvx128 1, 22, 11
	lvx128 6, 22, 12
	lvx128 7, 22, 0
	lvx128 8, 22, 30
	lvx128 9, 22, 21
	add 22, 3, 7
	li 21, 28
	lvsr 3, 22, 8
	vperm128 3, 10, 10, 3
	stvewx 3, 22, 21
	li 21, 24
	stvewx 3, 22, 21
	li 21, 20
	stvewx 3, 22, 21
	li 21, 44
	stvewx 3, 22, 8
	vperm128 3, 5, 0, 2
	lvsr 5, 22, 9
	vperm128 3, 3, 3, 5
	stvewx 3, 22, 21
	li 21, 40
	stvewx 3, 22, 21
	li 21, 12
	stvewx 3, 22, 20
	stvewx 3, 22, 9
	vperm128 3, 0, 1, 2
	lvsr 5, 22, 10
	vperm128 3, 3, 3, 5
	stvewx 3, 22, 19
	stvewx 3, 22, 18
	stvewx 3, 22, 17
	stvewx 3, 22, 10
	vperm128 3, 1, 6, 2
	lvsr 5, 22, 11
	vperm128 3, 3, 3, 5
	stvewx 3, 22, 16
	stvewx 3, 22, 15
	stvewx 3, 22, 14
	stvewx 3, 22, 11
	vperm128 3, 6, 7, 2
	lvsr 5, 22, 12
	vperm128 3, 3, 3, 5
	stvewx 3, 22, 31
	stvewx 3, 22, 6
	stvewx 3, 22, 29
	stvewx 3, 22, 12
	vperm128 3, 7, 8, 2
	lvsr 5, 22, 0
	vperm128 3, 3, 3, 5
	stvewx 3, 22, 28
	stvewx 3, 22, 27
	stvewx 3, 22, 5
	stvewx 3, 22, 0
	vperm128 3, 8, 9, 2
	lvsr 5, 22, 30
	vperm128 2, 11, 4, 2
	vperm128 3, 3, 3, 5
	stvewx 3, 22, 26
	stvewx 3, 22, 25
	stvewx 3, 22, 24
	stvewx 3, 22, 30
	lvsr 3, 3, 7
	vperm128 2, 2, 2, 3
	stvewx 2, 22, 21
	li 21, 8
	stvewx 2, 22, 21
	li 21, 4
	stvewx 2, 3, 7
	addi 7, 7, 128
	stvewx 2, 22, 21
	bne	0, .Ll3v_BB29_4
# %bb.5:                                # %middle.block
	lwz 5, 20(1)                            # 4-byte Folded Reload
	lwz 0, 16(1)                            # 4-byte Folded Reload
	cmplw	5, 0
	beq	0, .Ll3v_BB29_12
# %bb.6:                                # %vec.epilog.iter.check
	andi. 6, 5, 28
	beq	0, .Ll3v_BB29_10
.Ll3v_BB29_7:                               # %vec.epilog.ph
	mr	8, 0
	rlwinm 0, 5, 0, 1, 29
	li 12, 15
	li 6, 12
	sub	7, 8, 0
	slwi 8, 8, 2
	li 9, 8
	li 10, 4
.Ll3v_BB29_8:                               # %vec.epilog.vector.body
                                        # =>This Inner Loop Header: Depth=1
	add 11, 4, 8
	lvx128 2, 4, 8
	addi 7, 7, 4
	cmplwi	7, 0
	lvx128 3, 11, 12
	add 11, 3, 8
	lvsl 4, 4, 8
	vperm128 2, 2, 3, 4
	lvsr 3, 3, 8
	vperm128 2, 2, 2, 3
	stvewx 2, 3, 8
	addi 8, 8, 16
	stvewx 2, 11, 6
	stvewx 2, 11, 9
	stvewx 2, 11, 10
	bne	0, .Ll3v_BB29_8
# %bb.9:                                # %vec.epilog.middle.block
	cmplw	5, 0
	beq	0, .Ll3v_BB29_12
.Ll3v_BB29_10:                              # %for.body.preheader
	slwi 6, 0, 2
	sub	5, 5, 0
	addi 6, 6, -4
	add 4, 4, 6
	add 3, 3, 6
.Ll3v_BB29_11:                              # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lwzu 6, 4(4)
	addi 5, 5, -1
	cmplwi	5, 0
	stwu 6, 4(3)
	bne	0, .Ll3v_BB29_11
.Ll3v_BB29_12:
	ld 31, 160(1)                           # 8-byte Folded Reload
	ld 30, 152(1)                           # 8-byte Folded Reload
	ld 29, 144(1)                           # 8-byte Folded Reload
	ld 28, 136(1)                           # 8-byte Folded Reload
	ld 27, 128(1)                           # 8-byte Folded Reload
	ld 26, 120(1)                           # 8-byte Folded Reload
	ld 25, 112(1)                           # 8-byte Folded Reload
	ld 24, 104(1)                           # 8-byte Folded Reload
	ld 23, 96(1)                            # 8-byte Folded Reload
	ld 22, 88(1)                            # 8-byte Folded Reload
	ld 21, 80(1)                            # 8-byte Folded Reload
	ld 20, 72(1)                            # 8-byte Folded Reload
	ld 19, 64(1)                            # 8-byte Folded Reload
	ld 18, 56(1)                            # 8-byte Folded Reload
	ld 17, 48(1)                            # 8-byte Folded Reload
	ld 16, 40(1)                            # 8-byte Folded Reload
	ld 15, 32(1)                            # 8-byte Folded Reload
	ld 14, 24(1)                            # 8-byte Folded Reload
	addi 1, 1, 176
	blr
.Ll3v_func_end29:
	.size	l3v_k_copy_u32_r, .Ll3v_func_end29-.Ll3v_func_begin29
                                        # -- End function
	.section	.text.l3v_k_copy_u8,"ax",@progbits
	.globl	l3v_k_copy_u8                   # -- Begin function l3v_k_copy_u8
	.p2align	2
	.type	l3v_k_copy_u8,@function
l3v_k_copy_u8:                          # @l3v_k_copy_u8
.Ll3v_func_begin30:
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
	bc 12, 0, .Ll3v_BB30_11
# %bb.2:                                # %iter.check
	cmplwi	7, 16
	bc 12, 0, .Ll3v_BB30_11
# %bb.3:                                # %vector.main.loop.iter.check
	cmplwi	5, 16
	bge 0, .Ll3v_BB30_7
.Ll3v_BB30_4:                               # %vec.epilog.ph
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
.Ll3v_BB30_5:                               # %vec.epilog.vector.body
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
	lvx128 2, 0, 8
	lvsr 3, 3, 7
	vperm128 2, 2, 2, 3
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
	bne	0, .Ll3v_BB30_5
# %bb.6:                                # %vec.epilog.middle.block
	cmplw	5, 6
	bne	0, .Ll3v_BB30_11
	b .Ll3v_BB30_13
.Ll3v_BB30_7:                               # %vector.ph
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
.Ll3v_BB30_8:                               # %vector.body
                                        # =>This Inner Loop Header: Depth=1
	add 21, 4, 7
	lvx128 2, 4, 7
	lvx128 3, 21, 8
	add 21, 3, 7
	lvsl 4, 4, 7
	vperm128 2, 2, 3, 4
	lvsr 3, 3, 7
	vperm128 2, 2, 2, 3
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
	bne	0, .Ll3v_BB30_8
# %bb.9:                                # %middle.block
	cmplw	5, 6
	beq	0, .Ll3v_BB30_13
# %bb.10:                               # %vec.epilog.iter.check
	andi. 7, 5, 8
	bne	0, .Ll3v_BB30_4
.Ll3v_BB30_11:                              # %for.body.preheader
	addi 7, 6, -1
	add 4, 4, 7
	add 3, 3, 7
.Ll3v_BB30_12:                              # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lbzu 7, 1(4)
	addi 6, 6, 1
	cmplw	5, 6
	stbu 7, 1(3)
	bne	0, .Ll3v_BB30_12
.Ll3v_BB30_13:
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
.Ll3v_func_end30:
	.size	l3v_k_copy_u8, .Ll3v_func_end30-.Ll3v_func_begin30
                                        # -- End function
	.section	.text.l3v_k_set_u32,"ax",@progbits
	.globl	l3v_k_set_u32                   # -- Begin function l3v_k_set_u32
	.p2align	2
	.type	l3v_k_set_u32,@function
l3v_k_set_u32:                          # @l3v_k_set_u32
.Ll3v_func_begin31:
# %bb.0:                                # %entry
	cmpwi	5, 1
                                        # kill: def $r4 killed $r4 def $x4
	bltlr	0
# %bb.1:                                # %iter.check
	stwu 1, -224(1)
	cmplwi	5, 4
	li 11, 0
	std 14, 72(1)                           # 8-byte Folded Spill
	std 15, 80(1)                           # 8-byte Folded Spill
	std 16, 88(1)                           # 8-byte Folded Spill
	std 17, 96(1)                           # 8-byte Folded Spill
	std 18, 104(1)                          # 8-byte Folded Spill
	std 19, 112(1)                          # 8-byte Folded Spill
	std 20, 120(1)                          # 8-byte Folded Spill
	std 21, 128(1)                          # 8-byte Folded Spill
	std 22, 136(1)                          # 8-byte Folded Spill
	std 23, 144(1)                          # 8-byte Folded Spill
	std 24, 152(1)                          # 8-byte Folded Spill
	std 25, 160(1)                          # 8-byte Folded Spill
	std 26, 168(1)                          # 8-byte Folded Spill
	std 27, 176(1)                          # 8-byte Folded Spill
	std 28, 184(1)                          # 8-byte Folded Spill
	std 29, 192(1)                          # 8-byte Folded Spill
	std 30, 200(1)                          # 8-byte Folded Spill
	std 31, 208(1)                          # 8-byte Folded Spill
	blt	0, .Ll3v_BB31_10
# %bb.2:                                # %vector.main.loop.iter.check
	cmplwi	5, 32
	blt	0, .Ll3v_BB31_7
# %bb.3:                                # %vector.ph
	sldi 9, 4, 32
	stw 5, 28(1)                            # 4-byte Folded Spill
	rlwinm 30, 5, 0, 1, 26
	addi 5, 1, 48
	addi 7, 3, 112
	stw 3, 24(1)                            # 4-byte Folded Spill
	li 8, -112
	li 12, -96
	std 9, 48(1)
	li 28, -80
	li 27, -68
	li 26, -72
	li 25, -76
	std 9, 56(1)
	li 24, -64
	li 23, -52
	lvx128 2, 0, 5
	li 22, -56
	li 21, -60
	li 20, -48
	li 19, -36
	vspltw128 2, 2, 0
	stw 30, 20(1)                           # 4-byte Folded Spill
	li 18, -40
	li 17, -44
	li 16, -32
	li 15, -20
	li 14, -24
	li 31, -28
	li 6, -16
	li 3, -4
	li 9, -8
	li 5, -12
	li 10, 12
	li 11, 8
	li 0, 4
.Ll3v_BB31_4:                               # %vector.body
                                        # =>This Inner Loop Header: Depth=1
	lvsr 3, 7, 8
	li 29, -100
	addi 30, 30, -32
	vperm128 3, 2, 2, 3
	cmplwi	30, 0
	stvewx 3, 7, 29
	li 29, -104
	stvewx 3, 7, 29
	li 29, -108
	stvewx 3, 7, 29
	li 29, -84
	stvewx 3, 7, 8
	lvsr 3, 7, 12
	vperm128 3, 2, 2, 3
	stvewx 3, 7, 29
	li 29, -88
	stvewx 3, 7, 29
	li 29, -92
	stvewx 3, 7, 29
	stvewx 3, 7, 12
	lvsr 3, 7, 28
	vperm128 3, 2, 2, 3
	stvewx 3, 7, 27
	stvewx 3, 7, 26
	stvewx 3, 7, 25
	stvewx 3, 7, 28
	lvsr 3, 7, 24
	vperm128 3, 2, 2, 3
	stvewx 3, 7, 23
	stvewx 3, 7, 22
	stvewx 3, 7, 21
	stvewx 3, 7, 24
	lvsr 3, 7, 20
	vperm128 3, 2, 2, 3
	stvewx 3, 7, 19
	stvewx 3, 7, 18
	stvewx 3, 7, 17
	stvewx 3, 7, 20
	lvsr 3, 7, 16
	vperm128 3, 2, 2, 3
	stvewx 3, 7, 15
	stvewx 3, 7, 14
	stvewx 3, 7, 31
	stvewx 3, 7, 16
	lvsr 3, 7, 6
	vperm128 3, 2, 2, 3
	stvewx 3, 7, 3
	stvewx 3, 7, 9
	stvewx 3, 7, 5
	stvewx 3, 7, 6
	lvsr 3, 0, 7
	vperm128 3, 2, 2, 3
	stvewx 3, 7, 10
	stvewx 3, 7, 11
	stvewx 3, 7, 0
	stvewx 3, 0, 7
	addi 7, 7, 128
	bne	0, .Ll3v_BB31_4
# %bb.5:                                # %middle.block
	lwz 5, 28(1)                            # 4-byte Folded Reload
	lwz 11, 20(1)                           # 4-byte Folded Reload
	lwz 3, 24(1)                            # 4-byte Folded Reload
	cmplw	5, 11
	beq	0, .Ll3v_BB31_12
# %bb.6:                                # %vec.epilog.iter.check
	andi. 6, 5, 28
	beq	0, .Ll3v_BB31_10
.Ll3v_BB31_7:                               # %vec.epilog.ph
	sldi 8, 4, 32
	addi 6, 1, 32
	std 8, 32(1)
	mr	7, 11
	rlwinm 11, 5, 0, 1, 29
	std 8, 40(1)
	sub	10, 7, 11
	slwi 7, 7, 2
	add 9, 3, 7
	li 7, 8
	lvx128 2, 0, 6
	li 6, 12
	li 8, 4
	vspltw128 2, 2, 0
.Ll3v_BB31_8:                               # %vec.epilog.vector.body
                                        # =>This Inner Loop Header: Depth=1
	lvsr 3, 0, 9
	addi 10, 10, 4
	vperm128 3, 2, 2, 3
	cmplwi	10, 0
	stvewx 3, 9, 6
	stvewx 3, 9, 7
	stvewx 3, 9, 8
	stvewx 3, 0, 9
	addi 9, 9, 16
	bne	0, .Ll3v_BB31_8
# %bb.9:                                # %vec.epilog.middle.block
	cmplw	5, 11
	beq	0, .Ll3v_BB31_12
.Ll3v_BB31_10:                              # %for.body.preheader
	slwi 6, 11, 2
	add 6, 6, 3
	sub	3, 5, 11
	addi 5, 6, -4
.Ll3v_BB31_11:                              # %for.body
                                        # =>This Inner Loop Header: Depth=1
	addi 3, 3, -1
	stwu 4, 4(5)
	cmplwi	3, 0
	bne	0, .Ll3v_BB31_11
.Ll3v_BB31_12:
	ld 31, 208(1)                           # 8-byte Folded Reload
	ld 30, 200(1)                           # 8-byte Folded Reload
	ld 29, 192(1)                           # 8-byte Folded Reload
	ld 28, 184(1)                           # 8-byte Folded Reload
	ld 27, 176(1)                           # 8-byte Folded Reload
	ld 26, 168(1)                           # 8-byte Folded Reload
	ld 25, 160(1)                           # 8-byte Folded Reload
	ld 24, 152(1)                           # 8-byte Folded Reload
	ld 23, 144(1)                           # 8-byte Folded Reload
	ld 22, 136(1)                           # 8-byte Folded Reload
	ld 21, 128(1)                           # 8-byte Folded Reload
	ld 20, 120(1)                           # 8-byte Folded Reload
	ld 19, 112(1)                           # 8-byte Folded Reload
	ld 18, 104(1)                           # 8-byte Folded Reload
	ld 17, 96(1)                            # 8-byte Folded Reload
	ld 16, 88(1)                            # 8-byte Folded Reload
	ld 15, 80(1)                            # 8-byte Folded Reload
	ld 14, 72(1)                            # 8-byte Folded Reload
	addi 1, 1, 224
	blr
.Ll3v_func_end31:
	.size	l3v_k_set_u32, .Ll3v_func_end31-.Ll3v_func_begin31
                                        # -- End function
	.section	.text.l3v_k_set_f32,"ax",@progbits
	.globl	l3v_k_set_f32                   # -- Begin function l3v_k_set_f32
	.p2align	2
	.type	l3v_k_set_f32,@function
l3v_k_set_f32:                          # @l3v_k_set_f32
.Ll3v_func_begin32:
# %bb.0:                                # %entry
	cmpwi	5, 1
	bltlr	0
# %bb.1:                                # %iter.check
	stwu 1, -224(1)
	cmplwi	5, 4
	li 10, 0
	std 14, 72(1)                           # 8-byte Folded Spill
	std 15, 80(1)                           # 8-byte Folded Spill
	std 16, 88(1)                           # 8-byte Folded Spill
	std 17, 96(1)                           # 8-byte Folded Spill
	std 18, 104(1)                          # 8-byte Folded Spill
	std 19, 112(1)                          # 8-byte Folded Spill
	std 20, 120(1)                          # 8-byte Folded Spill
	std 21, 128(1)                          # 8-byte Folded Spill
	std 22, 136(1)                          # 8-byte Folded Spill
	std 23, 144(1)                          # 8-byte Folded Spill
	std 24, 152(1)                          # 8-byte Folded Spill
	std 25, 160(1)                          # 8-byte Folded Spill
	std 26, 168(1)                          # 8-byte Folded Spill
	std 27, 176(1)                          # 8-byte Folded Spill
	std 28, 184(1)                          # 8-byte Folded Spill
	std 29, 192(1)                          # 8-byte Folded Spill
	std 30, 200(1)                          # 8-byte Folded Spill
	std 31, 208(1)                          # 8-byte Folded Spill
	blt	0, .Ll3v_BB32_10
# %bb.2:                                # %vector.main.loop.iter.check
	cmplwi	5, 32
	blt	0, .Ll3v_BB32_7
# %bb.3:                                # %vector.ph
	stw 5, 28(1)                            # 4-byte Folded Spill
	rlwinm 12, 5, 0, 1, 26
	addi 5, 1, 48
	addi 6, 3, 112
	li 7, -112
	stw 3, 24(1)                            # 4-byte Folded Spill
	li 11, -96
	li 30, -92
	stfs 1, 48(1)
	li 29, -80
	li 28, -68
	li 27, -72
	li 26, -76
	lvx128 2, 0, 5
	li 25, -64
	li 24, -52
	li 23, -56
	li 22, -60
	vspltw128 2, 2, 0
	stw 12, 20(1)                           # 4-byte Folded Spill
	li 21, -48
	li 20, -36
	li 19, -40
	li 18, -44
	li 17, -32
	li 16, -20
	li 15, -24
	li 14, -28
	li 31, -16
	li 4, -4
	li 3, -8
	li 8, -12
	li 5, 12
	li 9, 8
	li 10, 4
.Ll3v_BB32_4:                               # %vector.body
                                        # =>This Inner Loop Header: Depth=1
	lvsr 3, 6, 7
	li 0, -100
	addi 12, 12, -32
	vperm128 3, 2, 2, 3
	cmplwi	12, 0
	stvewx 3, 6, 0
	li 0, -104
	stvewx 3, 6, 0
	li 0, -108
	stvewx 3, 6, 0
	li 0, -84
	stvewx 3, 6, 7
	lvsr 3, 6, 11
	vperm128 3, 2, 2, 3
	stvewx 3, 6, 0
	li 0, -88
	stvewx 3, 6, 0
	stvewx 3, 6, 30
	stvewx 3, 6, 11
	lvsr 3, 6, 29
	vperm128 3, 2, 2, 3
	stvewx 3, 6, 28
	stvewx 3, 6, 27
	stvewx 3, 6, 26
	stvewx 3, 6, 29
	lvsr 3, 6, 25
	vperm128 3, 2, 2, 3
	stvewx 3, 6, 24
	stvewx 3, 6, 23
	stvewx 3, 6, 22
	stvewx 3, 6, 25
	lvsr 3, 6, 21
	vperm128 3, 2, 2, 3
	stvewx 3, 6, 20
	stvewx 3, 6, 19
	stvewx 3, 6, 18
	stvewx 3, 6, 21
	lvsr 3, 6, 17
	vperm128 3, 2, 2, 3
	stvewx 3, 6, 16
	stvewx 3, 6, 15
	stvewx 3, 6, 14
	stvewx 3, 6, 17
	lvsr 3, 6, 31
	vperm128 3, 2, 2, 3
	stvewx 3, 6, 4
	stvewx 3, 6, 3
	stvewx 3, 6, 8
	stvewx 3, 6, 31
	lvsr 3, 0, 6
	vperm128 3, 2, 2, 3
	stvewx 3, 6, 5
	stvewx 3, 6, 9
	stvewx 3, 6, 10
	stvewx 3, 0, 6
	addi 6, 6, 128
	bne	0, .Ll3v_BB32_4
# %bb.5:                                # %middle.block
	lwz 5, 28(1)                            # 4-byte Folded Reload
	lwz 10, 20(1)                           # 4-byte Folded Reload
	lwz 3, 24(1)                            # 4-byte Folded Reload
	cmplw	5, 10
	beq	0, .Ll3v_BB32_12
# %bb.6:                                # %vec.epilog.iter.check
	andi. 4, 5, 28
	beq	0, .Ll3v_BB32_10
.Ll3v_BB32_7:                               # %vec.epilog.ph
	stfs 1, 32(1)
	addi 6, 1, 32
	mr	4, 10
	rlwinm 10, 5, 0, 1, 29
	li 8, 12
	lvx128 2, 0, 6
	sub	9, 4, 10
	slwi 4, 4, 2
	add 4, 3, 4
	li 6, 8
	vspltw128 2, 2, 0
	li 7, 4
.Ll3v_BB32_8:                               # %vec.epilog.vector.body
                                        # =>This Inner Loop Header: Depth=1
	lvsr 3, 0, 4
	addi 9, 9, 4
	vperm128 3, 2, 2, 3
	cmplwi	9, 0
	stvewx 3, 4, 8
	stvewx 3, 4, 6
	stvewx 3, 4, 7
	stvewx 3, 0, 4
	addi 4, 4, 16
	bne	0, .Ll3v_BB32_8
# %bb.9:                                # %vec.epilog.middle.block
	cmplw	5, 10
	beq	0, .Ll3v_BB32_12
.Ll3v_BB32_10:                              # %for.body.preheader
	slwi 4, 10, 2
	add 4, 4, 3
	sub	3, 5, 10
	addi 4, 4, -4
.Ll3v_BB32_11:                              # %for.body
                                        # =>This Inner Loop Header: Depth=1
	addi 3, 3, -1
	stfsu 1, 4(4)
	cmplwi	3, 0
	bne	0, .Ll3v_BB32_11
.Ll3v_BB32_12:
	ld 31, 208(1)                           # 8-byte Folded Reload
	ld 30, 200(1)                           # 8-byte Folded Reload
	ld 29, 192(1)                           # 8-byte Folded Reload
	ld 28, 184(1)                           # 8-byte Folded Reload
	ld 27, 176(1)                           # 8-byte Folded Reload
	ld 26, 168(1)                           # 8-byte Folded Reload
	ld 25, 160(1)                           # 8-byte Folded Reload
	ld 24, 152(1)                           # 8-byte Folded Reload
	ld 23, 144(1)                           # 8-byte Folded Reload
	ld 22, 136(1)                           # 8-byte Folded Reload
	ld 21, 128(1)                           # 8-byte Folded Reload
	ld 20, 120(1)                           # 8-byte Folded Reload
	ld 19, 112(1)                           # 8-byte Folded Reload
	ld 18, 104(1)                           # 8-byte Folded Reload
	ld 17, 96(1)                            # 8-byte Folded Reload
	ld 16, 88(1)                            # 8-byte Folded Reload
	ld 15, 80(1)                            # 8-byte Folded Reload
	ld 14, 72(1)                            # 8-byte Folded Reload
	addi 1, 1, 224
	blr
.Ll3v_func_end32:
	.size	l3v_k_set_f32, .Ll3v_func_end32-.Ll3v_func_begin32
                                        # -- End function
	.section	.text.l3v_k_zero_u8,"ax",@progbits
	.globl	l3v_k_zero_u8                   # -- Begin function l3v_k_zero_u8
	.p2align	2
	.type	l3v_k_zero_u8,@function
l3v_k_zero_u8:                          # @l3v_k_zero_u8
.Ll3v_func_begin33:
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
	blt	0, .Ll3v_BB33_10
# %bb.2:                                # %vector.main.loop.iter.check
	cmplwi	4, 16
	blt	0, .Ll3v_BB33_7
# %bb.3:                                # %vector.ph
	rlwinm 5, 4, 0, 1, 27
	li 6, 15
	vspltisw128 2, 0
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
.Ll3v_BB33_4:                               # %vector.body
                                        # =>This Inner Loop Header: Depth=1
	lvsr 3, 0, 23
	addi 22, 22, -16
	vperm128 3, 2, 2, 3
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
	bne	0, .Ll3v_BB33_4
# %bb.5:                                # %middle.block
	cmplw	4, 5
	beq	0, .Ll3v_BB33_12
# %bb.6:                                # %vec.epilog.iter.check
	andi. 6, 4, 12
	beq	0, .Ll3v_BB33_10
.Ll3v_BB33_7:                               # %vec.epilog.ph
	mr	8, 5
	rlwinm 5, 4, 0, 1, 29
	vspltisw128 2, 0
	li 6, 3
	sub	7, 8, 5
	add 8, 3, 8
	li 9, 2
	li 10, 1
.Ll3v_BB33_8:                               # %vec.epilog.vector.body
                                        # =>This Inner Loop Header: Depth=1
	lvsr 3, 0, 8
	addi 7, 7, 4
	vperm128 3, 2, 2, 3
	cmplwi	7, 0
	stvebx 3, 8, 6
	stvebx 3, 8, 9
	stvebx 3, 8, 10
	stvebx 3, 0, 8
	addi 8, 8, 4
	bne	0, .Ll3v_BB33_8
# %bb.9:                                # %vec.epilog.middle.block
	cmplw	4, 5
	beq	0, .Ll3v_BB33_12
.Ll3v_BB33_10:                              # %for.body.preheader
	add 3, 5, 3
	sub	4, 4, 5
	addi 3, 3, -1
	li 5, 0
.Ll3v_BB33_11:                              # %for.body
                                        # =>This Inner Loop Header: Depth=1
	addi 4, 4, -1
	stbu 5, 1(3)
	cmplwi	4, 0
	bne	0, .Ll3v_BB33_11
.Ll3v_BB33_12:
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
.Ll3v_func_end33:
	.size	l3v_k_zero_u8, .Ll3v_func_end33-.Ll3v_func_begin33
                                        # -- End function
	.section	.rodata.cst16,"aM",@progbits,16
	.p2align	4, 0x0                          # -- Begin function l3v_k_stride2_f32_r
.Ll3v_CPI34_0:
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
.Ll3v_CPI34_1:
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
	.section	.text.l3v_k_stride2_f32_r,"ax",@progbits
	.globl	l3v_k_stride2_f32_r
	.p2align	2
	.type	l3v_k_stride2_f32_r,@function
l3v_k_stride2_f32_r:                    # @l3v_k_stride2_f32_r
.Ll3v_func_begin34:
# %bb.0:                                # %entry
	cmpwi	5, 1
	bltlr	0
# %bb.1:                                # %for.body.preheader
	stwu 1, -48(1)
	cmplwi	5, 4
	li 6, 0
	std 29, 16(1)                           # 8-byte Folded Spill
	std 30, 24(1)                           # 8-byte Folded Spill
	blt	0, .Ll3v_BB34_5
# %bb.2:                                # %vector.ph
	li 9, .Ll3v_CPI34_0@l
	lis 10, .Ll3v_CPI34_0@ha
	lvx128 2, 10, 9
	li 11, .Ll3v_CPI34_1@l
	lis 10, .Ll3v_CPI34_1@ha
	lvx128 3, 10, 11
	rlwinm 6, 5, 0, 1, 29
	li 7, 31
	li 8, 16
	li 9, 12
	li 10, 8
	li 11, 4
	mr	12, 4
	mr	30, 3
	mr	29, 6
.Ll3v_BB34_3:                               # %vector.body
                                        # =>This Inner Loop Header: Depth=1
	lvx128 4, 12, 7
	addi 29, 29, -4
	cmplwi	29, 0
	lvx128 5, 12, 8
	lvx128 1, 0, 12
	lvsl 0, 0, 12
	addi 12, 12, 32
	vperm128 4, 5, 4, 0
	vperm128 5, 1, 5, 0
	vperm128 0, 5, 4, 2
	vperm128 4, 5, 4, 3
	lvsr 5, 0, 30
	vaddfp128 4, 0, 4
	vperm128 4, 4, 4, 5
	stvewx 4, 30, 9
	stvewx 4, 30, 10
	stvewx 4, 30, 11
	stvewx 4, 0, 30
	addi 30, 30, 16
	bne	0, .Ll3v_BB34_3
# %bb.4:                                # %middle.block
	cmplw	5, 6
	beq	0, .Ll3v_BB34_7
.Ll3v_BB34_5:                               # %for.body.preheader13
	sub	5, 5, 6
	slwi 7, 6, 2
	slwi 6, 6, 3
	add 4, 6, 4
	add 6, 7, 3
	addi 3, 4, -8
	addi 4, 6, -4
.Ll3v_BB34_6:                               # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lfsu 0, 8(3)
	addi 5, 5, -1
	cmplwi	5, 0
	lfs 1, 4(3)
	fadds 0, 0, 1
	stfsu 0, 4(4)
	bne	0, .Ll3v_BB34_6
.Ll3v_BB34_7:
	ld 30, 24(1)                            # 8-byte Folded Reload
	ld 29, 16(1)                            # 8-byte Folded Reload
	addi 1, 1, 48
	blr
.Ll3v_func_end34:
	.size	l3v_k_stride2_f32_r, .Ll3v_func_end34-.Ll3v_func_begin34
                                        # -- End function
	.section	.text.l3v_k_gather_f32_r,"ax",@progbits
	.globl	l3v_k_gather_f32_r              # -- Begin function l3v_k_gather_f32_r
	.p2align	2
	.type	l3v_k_gather_f32_r,@function
l3v_k_gather_f32_r:                     # @l3v_k_gather_f32_r
.Ll3v_func_begin35:
# %bb.0:                                # %entry
	cmpwi	6, 1
	bltlr	0
# %bb.1:                                # %for.body.preheader
	li 7, 0
	beq	0, .Ll3v_BB35_5
# %bb.2:                                # %vector.ph
	rlwinm 7, 6, 0, 1, 30
	addi 8, 5, -8
	addi 9, 3, -8
	mr	10, 7
.Ll3v_BB35_3:                               # %vector.body
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
	bne	0, .Ll3v_BB35_3
# %bb.4:                                # %middle.block
	cmplw	6, 7
	beqlr	0
.Ll3v_BB35_5:                               # %for.body.preheader8
	slwi 8, 7, 2
	sub	6, 6, 7
	addi 7, 8, -4
	add 5, 5, 7
	add 3, 3, 7
.Ll3v_BB35_6:                               # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lwzu 7, 4(5)
	addi 6, 6, -1
	cmplwi	6, 0
	slwi 7, 7, 2
	lwzx 7, 4, 7
	stwu 7, 4(3)
	bne	0, .Ll3v_BB35_6
	blr
.Ll3v_func_end35:
	.size	l3v_k_gather_f32_r, .Ll3v_func_end35-.Ll3v_func_begin35
                                        # -- End function
	.section	.text.l3v_k_offset1_f32_r,"ax",@progbits
	.globl	l3v_k_offset1_f32_r             # -- Begin function l3v_k_offset1_f32_r
	.p2align	2
	.type	l3v_k_offset1_f32_r,@function
l3v_k_offset1_f32_r:                    # @l3v_k_offset1_f32_r
.Ll3v_func_begin36:
# %bb.0:                                # %entry
	cmpwi	5, 1
	bltlr	0
# %bb.1:                                # %for.body.preheader
	stwu 1, -224(1)
	lfs 0, 0(4)
	cmplwi	5, 32
	li 7, 0
	std 14, 72(1)                           # 8-byte Folded Spill
	std 15, 80(1)                           # 8-byte Folded Spill
	std 16, 88(1)                           # 8-byte Folded Spill
	std 17, 96(1)                           # 8-byte Folded Spill
	std 18, 104(1)                          # 8-byte Folded Spill
	std 19, 112(1)                          # 8-byte Folded Spill
	std 20, 120(1)                          # 8-byte Folded Spill
	std 21, 128(1)                          # 8-byte Folded Spill
	std 22, 136(1)                          # 8-byte Folded Spill
	std 23, 144(1)                          # 8-byte Folded Spill
	std 24, 152(1)                          # 8-byte Folded Spill
	std 25, 160(1)                          # 8-byte Folded Spill
	std 26, 168(1)                          # 8-byte Folded Spill
	std 27, 176(1)                          # 8-byte Folded Spill
	std 28, 184(1)                          # 8-byte Folded Spill
	std 29, 192(1)                          # 8-byte Folded Spill
	std 30, 200(1)                          # 8-byte Folded Spill
	std 31, 208(1)                          # 8-byte Folded Spill
	blt	0, .Ll3v_BB36_6
# %bb.2:                                # %vector.ph
	stw 5, 28(1)                            # 4-byte Folded Spill
	addi 26, 1, 48
	rlwinm 20, 5, 0, 1, 26
	li 8, 4
	li 9, 20
	stfs 0, 48(1)
	li 10, 116
	li 11, 36
	li 12, 52
	li 0, 68
	lvx128 2, 0, 26
	li 30, 84
	li 29, 100
	li 25, 16
	li 22, 32
	vpermwi128 2, 2, 24
	stw 20, 24(1)                           # 4-byte Folded Spill
	li 19, 48
	li 16, 64
	li 15, 76
	li 14, 72
	li 31, 80
	li 6, 92
	li 28, 88
	li 5, 96
	li 27, 108
	li 26, 104
	li 24, 112
	li 23, 124
	li 21, 120
.Ll3v_BB36_3:                               # %vector.body
                                        # =>This Inner Loop Header: Depth=1
	add 18, 4, 7
	li 17, 131
	lvx128 4, 18, 8
	addi 20, 20, -32
	cmplwi	20, 0
	lvx128 5, 18, 9
	lvx128 0, 18, 11
	lvsl 3, 18, 10
	vperm128 4, 4, 5, 3
	lvx128 1, 18, 12
	vperm128 5, 5, 0, 3
	lvx128 6, 18, 0
	vsldoi128 11, 4, 5, 12
	vaddfp128 11, 11, 5
	lvx128 12, 18, 30
	vperm128 0, 0, 1, 3
	lvx128 8, 18, 29
	vsldoi128 5, 5, 0, 12
	lvx128 9, 18, 10
	vaddfp128 5, 5, 0
	vperm128 1, 1, 6, 3
	lvx128 10, 18, 17
	add 18, 3, 7
	li 17, 28
	lvsr 7, 18, 25
	vperm128 7, 11, 11, 7
	stvewx 7, 18, 17
	li 17, 24
	stvewx 7, 18, 17
	li 17, 44
	stvewx 7, 18, 9
	stvewx 7, 18, 25
	lvsr 7, 18, 22
	vperm128 5, 5, 5, 7
	stvewx 5, 18, 17
	li 17, 40
	stvewx 5, 18, 17
	li 17, 60
	stvewx 5, 18, 11
	stvewx 5, 18, 22
	vsldoi128 5, 0, 1, 12
	vaddfp128 5, 5, 1
	lvsr 0, 18, 19
	vperm128 5, 5, 5, 0
	stvewx 5, 18, 17
	li 17, 56
	vperm128 0, 6, 12, 3
	stvewx 5, 18, 17
	li 17, 12
	stvewx 5, 18, 12
	stvewx 5, 18, 19
	vsldoi128 5, 1, 0, 12
	vaddfp128 5, 5, 0
	lvsr 1, 18, 16
	vperm128 5, 5, 5, 1
	stvewx 5, 18, 15
	vperm128 1, 12, 8, 3
	stvewx 5, 18, 14
	stvewx 5, 18, 0
	stvewx 5, 18, 16
	vsldoi128 5, 0, 1, 12
	vaddfp128 5, 5, 1
	lvsr 0, 18, 31
	vperm128 5, 5, 5, 0
	stvewx 5, 18, 6
	vperm128 0, 8, 9, 3
	stvewx 5, 18, 28
	stvewx 5, 18, 30
	stvewx 5, 18, 31
	vsldoi128 5, 1, 0, 12
	vaddfp128 5, 5, 0
	lvsr 1, 18, 5
	vperm128 5, 5, 5, 1
	stvewx 5, 18, 27
	vmr	1, 2
	vperm128 2, 9, 10, 3
	stvewx 5, 18, 26
	vsldoi128 3, 0, 2, 12
	stvewx 5, 18, 29
	stvewx 5, 18, 5
	vaddfp128 3, 3, 2
	lvsr 5, 18, 24
	vperm128 3, 3, 3, 5
	stvewx 3, 18, 23
	stvewx 3, 18, 21
	stvewx 3, 18, 10
	stvewx 3, 18, 24
	vsldoi128 3, 1, 4, 12
	vaddfp128 3, 3, 4
	lvsr 5, 3, 7
	vperm128 3, 3, 3, 5
	stvewx 3, 18, 17
	li 17, 8
	stvewx 3, 3, 7
	addi 7, 7, 128
	stvewx 3, 18, 17
	stvewx 3, 18, 8
	bne	0, .Ll3v_BB36_3
# %bb.4:                                # %middle.block
	lwz 5, 28(1)                            # 4-byte Folded Reload
	addi 6, 1, 32
	lwz 7, 24(1)                            # 4-byte Folded Reload
	cmplw	5, 7
	stvx128 2, 0, 6
	beq	0, .Ll3v_BB36_8
# %bb.5:
	lfs 0, 44(1)
.Ll3v_BB36_6:                               # %for.body.preheader18
	slwi 6, 7, 2
	sub	5, 5, 7
	add 7, 6, 3
	add 3, 4, 6
	addi 4, 7, -4
.Ll3v_BB36_7:                               # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lfsu 1, 4(3)
	addi 5, 5, -1
	cmplwi	5, 0
	fadds 0, 0, 1
	stfsu 0, 4(4)
	fmr 0, 1
	bne	0, .Ll3v_BB36_7
.Ll3v_BB36_8:
	ld 31, 208(1)                           # 8-byte Folded Reload
	ld 30, 200(1)                           # 8-byte Folded Reload
	ld 29, 192(1)                           # 8-byte Folded Reload
	ld 28, 184(1)                           # 8-byte Folded Reload
	ld 27, 176(1)                           # 8-byte Folded Reload
	ld 26, 168(1)                           # 8-byte Folded Reload
	ld 25, 160(1)                           # 8-byte Folded Reload
	ld 24, 152(1)                           # 8-byte Folded Reload
	ld 23, 144(1)                           # 8-byte Folded Reload
	ld 22, 136(1)                           # 8-byte Folded Reload
	ld 21, 128(1)                           # 8-byte Folded Reload
	ld 20, 120(1)                           # 8-byte Folded Reload
	ld 19, 112(1)                           # 8-byte Folded Reload
	ld 18, 104(1)                           # 8-byte Folded Reload
	ld 17, 96(1)                            # 8-byte Folded Reload
	ld 16, 88(1)                            # 8-byte Folded Reload
	ld 15, 80(1)                            # 8-byte Folded Reload
	ld 14, 72(1)                            # 8-byte Folded Reload
	addi 1, 1, 224
	blr
.Ll3v_func_end36:
	.size	l3v_k_offset1_f32_r, .Ll3v_func_end36-.Ll3v_func_begin36
                                        # -- End function
	.section	.rodata.cst16,"aM",@progbits,16
	.p2align	4, 0x0                          # -- Begin function l3v_k_fir3_f32_r
.Ll3v_CPI37_0:
	.long	0x3f000000                      # float 0.5
	.long	0x3f000000                      # float 0.5
	.long	0x3f000000                      # float 0.5
	.long	0x3f000000                      # float 0.5
.Ll3v_CPI37_1:
	.long	0x3e800000                      # float 0.25
	.long	0x3e800000                      # float 0.25
	.long	0x3e800000                      # float 0.25
	.long	0x3e800000                      # float 0.25
	.section	.rodata.cst4,"aM",@progbits,4
	.p2align	2, 0x0
.Ll3v_CPI37_2:
	.long	0x3f000000                      # float 0.5
.Ll3v_CPI37_3:
	.long	0x3e800000                      # float 0.25
	.section	.text.l3v_k_fir3_f32_r,"ax",@progbits
	.globl	l3v_k_fir3_f32_r
	.p2align	2
	.type	l3v_k_fir3_f32_r,@function
l3v_k_fir3_f32_r:                       # @l3v_k_fir3_f32_r
.Ll3v_func_begin37:
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
	blt	0, .Ll3v_BB37_6
# %bb.2:                                # %vector.ph
	stfs 0, 32(1)
	addi 11, 1, 32
	li 12, .Ll3v_CPI37_0@l
	li 0, .Ll3v_CPI37_1@l
	rlwinm 6, 5, 0, 1, 29
	lvx128 3, 0, 11
	lis 11, .Ll3v_CPI37_0@ha
	addi 7, 4, 4
	addi 8, 4, 8
	li 9, 0
	lvx128 2, 11, 12
	vpermwi128 3, 3, 24
	lis 12, .Ll3v_CPI37_1@ha
	li 10, 15
	lvx128 4, 12, 0
	li 11, 12
	li 12, 8
	li 0, 4
	mr	30, 6
.Ll3v_BB37_3:                               # %vector.body
                                        # =>This Inner Loop Header: Depth=1
	add 29, 7, 9
	lvx128 5, 7, 9
	vmr	6, 3
	addi 30, 30, -4
	cmplwi	30, 0
	lvx128 0, 29, 10
	add 29, 8, 9
	lvsl 1, 7, 9
	vperm128 3, 5, 0, 1
	vmulfp128 5, 3, 2
	lvx128 0, 8, 9
	vsldoi128 1, 6, 3, 12
	lvsl 6, 8, 9
	vmaddfp128 5, 1, 4, 5
	lvx128 1, 29, 10
	add 29, 3, 9
	vperm128 0, 0, 1, 6
	lvsr 1, 3, 9
	vmaddfp128 5, 0, 4, 5
	vperm128 5, 5, 5, 1
	stvewx 5, 3, 9
	addi 9, 9, 16
	stvewx 5, 29, 11
	stvewx 5, 29, 12
	stvewx 5, 29, 0
	bne	0, .Ll3v_BB37_3
# %bb.4:                                # %middle.block
	addi 7, 1, 16
	cmplw	5, 6
	stvx128 3, 0, 7
	beq	0, .Ll3v_BB37_8
# %bb.5:
	lfs 0, 28(1)
.Ll3v_BB37_6:                               # %for.body.preheader16
	slwi 7, 6, 2
	sub	5, 5, 6
	lis 6, .Ll3v_CPI37_2@ha
	add 8, 7, 3
	lfs 1, .Ll3v_CPI37_2@l(6)
	lis 6, .Ll3v_CPI37_3@ha
	add 3, 4, 7
	lfs 2, .Ll3v_CPI37_3@l(6)
	addi 4, 8, -4
.Ll3v_BB37_7:                               # %for.body
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
	bne	0, .Ll3v_BB37_7
.Ll3v_BB37_8:
	ld 30, 56(1)                            # 8-byte Folded Reload
	ld 29, 48(1)                            # 8-byte Folded Reload
	addi 1, 1, 80
	blr
.Ll3v_func_end37:
	.size	l3v_k_fir3_f32_r, .Ll3v_func_end37-.Ll3v_func_begin37
                                        # -- End function
	.section	.text.l3v_k_prefix_f32,"ax",@progbits
	.globl	l3v_k_prefix_f32                # -- Begin function l3v_k_prefix_f32
	.p2align	2
	.type	l3v_k_prefix_f32,@function
l3v_k_prefix_f32:                       # @l3v_k_prefix_f32
.Ll3v_func_begin38:
# %bb.0:                                # %entry
	cmpwi	4, 2
	bltlr	0
# %bb.1:                                # %for.body.preheader
	lfs 0, 0(3)
	addi 4, 4, -1
.Ll3v_BB38_2:                               # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lfsu 1, 4(3)
	addi 4, 4, -1
	cmplwi	4, 0
	fadds 0, 0, 1
	stfs 0, 0(3)
	bne	0, .Ll3v_BB38_2
# %bb.3:                                # %for.end
	blr
.Ll3v_func_end38:
	.size	l3v_k_prefix_f32, .Ll3v_func_end38-.Ll3v_func_begin38
                                        # -- End function
	.section	.text.l3v_k_g_add_f32,"ax",@progbits
	.globl	l3v_k_g_add_f32                 # -- Begin function l3v_k_g_add_f32
	.p2align	2
	.type	l3v_k_g_add_f32,@function
l3v_k_g_add_f32:                        # @l3v_k_g_add_f32
.Ll3v_func_begin39:
# %bb.0:                                # %entry
	stwu 1, -64(1)
	std 30, 40(1)                           # 8-byte Folded Spill
	lis 11, l3v_g_a@ha
	lis 12, l3v_g_b@ha
	lis 30, l3v_g_c@ha
	li 3, 0
	std 28, 24(1)                           # 8-byte Folded Spill
	li 4, 16
	li 5, 32
	li 6, 48
	li 7, 64
	std 29, 32(1)                           # 8-byte Folded Spill
	li 8, 80
	li 9, 96
	li 10, 112
	la 11, l3v_g_a@l(11)
	la 12, l3v_g_b@l(12)
	la 30, l3v_g_c@l(30)
.Ll3v_BB39_1:                               # %vector.body
                                        # =>This Inner Loop Header: Depth=1
	lvx128 2, 11, 3
	add 29, 11, 3
	add 28, 12, 3
	lvx128 3, 12, 3
	lvx128 4, 29, 4
	vaddfp128 2, 2, 3
	lvx128 0, 29, 6
	lvx128 3, 28, 4
	lvx128 8, 28, 6
	vaddfp128 3, 4, 3
	lvx128 5, 29, 5
	lvx128 1, 29, 7
	vaddfp128 0, 0, 8
	lvx128 6, 28, 5
	lvx128 9, 28, 7
	lvx128 4, 29, 8
	vaddfp128 5, 5, 6
	lvx128 8, 28, 8
	lvx128 7, 29, 9
	lvx128 6, 29, 10
	vaddfp128 1, 1, 9
	add 29, 30, 3
	lvx128 10, 28, 9
	lvx128 9, 28, 10
	stvx128 2, 30, 3
	vaddfp128 2, 4, 8
	addi 3, 3, 128
	cmplwi	3, 4096
	stvx128 3, 29, 4
	stvx128 2, 29, 8
	vaddfp128 3, 7, 10
	stvx128 5, 29, 5
	stvx128 0, 29, 6
	stvx128 1, 29, 7
	vaddfp128 2, 6, 9
	stvx128 3, 29, 9
	stvx128 2, 29, 10
	bne	0, .Ll3v_BB39_1
# %bb.2:                                # %for.end
	ld 30, 40(1)                            # 8-byte Folded Reload
	ld 29, 32(1)                            # 8-byte Folded Reload
	ld 28, 24(1)                            # 8-byte Folded Reload
	addi 1, 1, 64
	blr
.Ll3v_func_end39:
	.size	l3v_k_g_add_f32, .Ll3v_func_end39-.Ll3v_func_begin39
                                        # -- End function
	.section	.text.l3v_k_g_saxpy,"ax",@progbits
	.globl	l3v_k_g_saxpy                   # -- Begin function l3v_k_g_saxpy
	.p2align	2
	.type	l3v_k_g_saxpy,@function
l3v_k_g_saxpy:                          # @l3v_k_g_saxpy
.Ll3v_func_begin40:
# %bb.0:                                # %entry
	stwu 1, -80(1)
	std 28, 40(1)                           # 8-byte Folded Spill
	addi 12, 1, 16
	lis 9, l3v_g_a@ha
	li 3, 0
	li 4, 16
	std 29, 48(1)                           # 8-byte Folded Spill
	li 5, 32
	li 6, 48
	std 30, 56(1)                           # 8-byte Folded Spill
	lis 30, l3v_g_c@ha
	li 7, 64
	li 8, 80
	la 9, l3v_g_a@l(9)
	stfs 1, 16(1)
	li 10, 96
	li 11, 112
	la 30, l3v_g_c@l(30)
	lvx128 2, 0, 12
	lis 12, l3v_g_b@ha
	la 12, l3v_g_b@l(12)
	vspltw128 2, 2, 0
.Ll3v_BB40_1:                               # %vector.body
                                        # =>This Inner Loop Header: Depth=1
	lvx128 3, 9, 3
	add 29, 9, 3
	add 28, 12, 3
	lvx128 4, 12, 3
	lvx128 5, 29, 4
	vmaddfp128 4, 2, 3, 4
	lvx128 0, 29, 5
	lvx128 1, 29, 6
	lvx128 3, 28, 4
	lvx128 6, 29, 7
	vmaddfp128 3, 2, 5, 3
	lvx128 7, 28, 5
	lvx128 9, 28, 6
	vmaddfp128 7, 2, 0, 7
	lvx128 10, 28, 7
	lvx128 5, 29, 8
	lvx128 8, 29, 10
	vmaddfp128 9, 2, 1, 9
	lvx128 0, 29, 11
	add 29, 30, 3
	lvx128 1, 28, 8
	lvx128 11, 28, 10
	vmaddfp128 10, 2, 6, 10
	lvx128 6, 28, 11
	stvx128 4, 30, 3
	addi 3, 3, 128
	cmplwi	3, 4096
	vmaddfp128 1, 2, 5, 1
	stvx128 3, 29, 4
	stvx128 7, 29, 5
	stvx128 9, 29, 6
	vmaddfp128 11, 2, 8, 11
	stvx128 10, 29, 7
	stvx128 1, 29, 8
	vmaddfp128 6, 2, 0, 6
	stvx128 11, 29, 10
	stvx128 6, 29, 11
	bne	0, .Ll3v_BB40_1
# %bb.2:                                # %for.end
	ld 30, 56(1)                            # 8-byte Folded Reload
	ld 29, 48(1)                            # 8-byte Folded Reload
	ld 28, 40(1)                            # 8-byte Folded Reload
	addi 1, 1, 80
	blr
.Ll3v_func_end40:
	.size	l3v_k_g_saxpy, .Ll3v_func_end40-.Ll3v_func_begin40
                                        # -- End function
	.section	.text.l3v_k_g_add_i32,"ax",@progbits
	.globl	l3v_k_g_add_i32                 # -- Begin function l3v_k_g_add_i32
	.p2align	2
	.type	l3v_k_g_add_i32,@function
l3v_k_g_add_i32:                        # @l3v_k_g_add_i32
.Ll3v_func_begin41:
# %bb.0:                                # %entry
	stwu 1, -48(1)
	std 30, 24(1)                           # 8-byte Folded Spill
	lis 11, l3v_g_ia@ha
	lis 12, l3v_g_ib@ha
	lis 30, l3v_g_ic@ha
	li 3, 0
	std 29, 16(1)                           # 8-byte Folded Spill
	li 4, 16
	li 5, 32
	li 6, 48
	li 7, 64
	li 8, 80
	li 9, 96
	li 10, 112
	la 11, l3v_g_ia@l(11)
	la 12, l3v_g_ib@l(12)
	la 30, l3v_g_ic@l(30)
.Ll3v_BB41_1:                               # %vector.body
                                        # =>This Inner Loop Header: Depth=1
	add 29, 11, 3
	lvx128 2, 11, 3
	lvx128 3, 12, 3
	lvx128 4, 29, 4
	vadduwm 2, 3, 2
	lvx128 5, 29, 5
	lvx128 0, 29, 6
	lvx128 1, 29, 7
	lvx128 6, 29, 8
	lvx128 7, 29, 9
	lvx128 8, 29, 10
	add 29, 12, 3
	lvx128 9, 29, 4
	lvx128 10, 29, 5
	lvx128 11, 29, 6
	lvx128 12, 29, 7
	lvx128 13, 29, 8
	lvx128 14, 29, 9
	lvx128 15, 29, 10
	add 29, 30, 3
	stvx128 2, 30, 3
	vadduwm 2, 9, 4
	addi 3, 3, 128
	cmplwi	3, 4096
	stvx128 2, 29, 4
	vadduwm 2, 10, 5
	stvx128 2, 29, 5
	vadduwm 2, 11, 0
	stvx128 2, 29, 6
	vadduwm 2, 12, 1
	stvx128 2, 29, 7
	vadduwm 2, 13, 6
	stvx128 2, 29, 8
	vadduwm 2, 14, 7
	stvx128 2, 29, 9
	vadduwm 2, 15, 8
	stvx128 2, 29, 10
	bne	0, .Ll3v_BB41_1
# %bb.2:                                # %for.end
	ld 30, 24(1)                            # 8-byte Folded Reload
	ld 29, 16(1)                            # 8-byte Folded Reload
	addi 1, 1, 48
	blr
.Ll3v_func_end41:
	.size	l3v_k_g_add_i32, .Ll3v_func_end41-.Ll3v_func_begin41
                                        # -- End function
	.section	.rodata.cst4,"aM",@progbits,4
	.p2align	2, 0x0                          # -- Begin function l3v_k_g_dot
.Ll3v_CPI42_0:
	.long	0x00000000                      # float 0
	.section	.text.l3v_k_g_dot,"ax",@progbits
	.globl	l3v_k_g_dot
	.p2align	2
	.type	l3v_k_g_dot,@function
l3v_k_g_dot:                            # @l3v_k_g_dot
.Ll3v_func_begin42:
# %bb.0:                                # %entry
	lis 6, .Ll3v_CPI42_0@ha
	lis 3, l3v_g_b@ha
	lfs 1, .Ll3v_CPI42_0@l(6)
	lis 4, l3v_g_a@ha
	la 3, l3v_g_b@l(3)
	la 4, l3v_g_a@l(4)
	addi 3, 3, -4
	addi 4, 4, -4
	li 5, 0
.Ll3v_BB42_1:                               # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lfsu 0, 4(3)
	addi 5, 5, 4
	cmplwi	5, 4096
	lfsu 2, 4(4)
	fmadds 1, 2, 0, 1
	bne	0, .Ll3v_BB42_1
# %bb.2:                                # %for.end
	blr
.Ll3v_func_end42:
	.size	l3v_k_g_dot, .Ll3v_func_end42-.Ll3v_func_begin42
                                        # -- End function
	.section	.text.l3v_k_add16_f32_r,"ax",@progbits
	.globl	l3v_k_add16_f32_r               # -- Begin function l3v_k_add16_f32_r
	.p2align	2
	.type	l3v_k_add16_f32_r,@function
l3v_k_add16_f32_r:                      # @l3v_k_add16_f32_r
.Ll3v_func_begin43:
# %bb.0:                                # %entry
	li 6, 16
	lvx128 3, 0, 4
	li 7, 12
	li 8, 28
	lvx128 4, 4, 6
	lvx128 5, 0, 5
	lvx128 0, 5, 6
	lvsl 2, 0, 4
	vperm128 1, 3, 4, 2
	lvsl 3, 0, 5
	vperm128 5, 5, 0, 3
	vaddfp128 5, 1, 5
	lvsr 6, 0, 3
	vperm128 5, 5, 5, 6
	stvewx 5, 3, 7
	li 7, 8
	stvewx 5, 3, 7
	li 7, 4
	stvewx 5, 3, 7
	li 7, 32
	stvewx 5, 0, 3
	lvx128 5, 4, 7
	lvx128 1, 5, 7
	vperm128 4, 4, 5, 2
	vperm128 0, 0, 1, 3
	lvsr 6, 3, 6
	vaddfp128 4, 4, 0
	vperm128 4, 4, 4, 6
	lvsr 6, 3, 7
	stvewx 4, 3, 8
	li 8, 24
	stvewx 4, 3, 8
	li 8, 20
	stvewx 4, 3, 8
	li 8, 44
	stvewx 4, 3, 6
	li 6, 48
	lvx128 4, 4, 6
	lvx128 0, 5, 6
	vperm128 5, 5, 4, 2
	vperm128 1, 1, 0, 3
	vaddfp128 5, 5, 1
	vperm128 5, 5, 5, 6
	stvewx 5, 3, 8
	li 8, 40
	stvewx 5, 3, 8
	li 8, 36
	stvewx 5, 3, 8
	stvewx 5, 3, 7
	li 7, 63
	lvx128 5, 4, 7
	li 4, 60
	lvx128 1, 5, 7
	vperm128 2, 4, 5, 2
	vperm128 3, 0, 1, 3
	lvsr 4, 3, 6
	vaddfp128 2, 2, 3
	vperm128 2, 2, 2, 4
	stvewx 2, 3, 4
	li 4, 56
	stvewx 2, 3, 4
	li 4, 52
	stvewx 2, 3, 4
	stvewx 2, 3, 6
	blr
.Ll3v_func_end43:
	.size	l3v_k_add16_f32_r, .Ll3v_func_end43-.Ll3v_func_begin43
                                        # -- End function
	.type	l3v_g_a,@object                 # @l3v_g_a
	.section	.bss.l3v_g_a,"aw",@nobits
	.globl	l3v_g_a
	.p2align	4, 0x0
l3v_g_a:
	.space	4096
	.size	l3v_g_a, 4096

	.type	l3v_g_b,@object                 # @l3v_g_b
	.section	.bss.l3v_g_b,"aw",@nobits
	.globl	l3v_g_b
	.p2align	4, 0x0
l3v_g_b:
	.space	4096
	.size	l3v_g_b, 4096

	.type	l3v_g_c,@object                 # @l3v_g_c
	.section	.bss.l3v_g_c,"aw",@nobits
	.globl	l3v_g_c
	.p2align	4, 0x0
l3v_g_c:
	.space	4096
	.size	l3v_g_c, 4096

	.type	l3v_g_ia,@object                # @l3v_g_ia
	.section	.bss.l3v_g_ia,"aw",@nobits
	.globl	l3v_g_ia
	.p2align	4, 0x0
l3v_g_ia:
	.space	4096
	.size	l3v_g_ia, 4096

	.type	l3v_g_ib,@object                # @l3v_g_ib
	.section	.bss.l3v_g_ib,"aw",@nobits
	.globl	l3v_g_ib
	.p2align	4, 0x0
l3v_g_ib:
	.space	4096
	.size	l3v_g_ib, 4096

	.type	l3v_g_ic,@object                # @l3v_g_ic
	.section	.bss.l3v_g_ic,"aw",@nobits
	.globl	l3v_g_ic
	.p2align	4, 0x0
l3v_g_ic:
	.space	4096
	.size	l3v_g_ic, 4096

