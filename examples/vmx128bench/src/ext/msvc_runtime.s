# MSVC-compatible runtime helpers for converted cl.exe code (gen_runtime.py).
	.section .text.msvc_runtime,"ax",@progbits
	.p2align 3
	.globl __savegprlr_14
	.type __savegprlr_14, @function
__savegprlr_14:
	.long 0xF9C1FF68	# std r14,-152(r1)
	.globl __savegprlr_15
	.type __savegprlr_15, @function
__savegprlr_15:
	.long 0xF9E1FF70	# std r15,-144(r1)
	.globl __savegprlr_16
	.type __savegprlr_16, @function
__savegprlr_16:
	.long 0xFA01FF78	# std r16,-136(r1)
	.globl __savegprlr_17
	.type __savegprlr_17, @function
__savegprlr_17:
	.long 0xFA21FF80	# std r17,-128(r1)
	.globl __savegprlr_18
	.type __savegprlr_18, @function
__savegprlr_18:
	.long 0xFA41FF88	# std r18,-120(r1)
	.globl __savegprlr_19
	.type __savegprlr_19, @function
__savegprlr_19:
	.long 0xFA61FF90	# std r19,-112(r1)
	.globl __savegprlr_20
	.type __savegprlr_20, @function
__savegprlr_20:
	.long 0xFA81FF98	# std r20,-104(r1)
	.globl __savegprlr_21
	.type __savegprlr_21, @function
__savegprlr_21:
	.long 0xFAA1FFA0	# std r21,-96(r1)
	.globl __savegprlr_22
	.type __savegprlr_22, @function
__savegprlr_22:
	.long 0xFAC1FFA8	# std r22,-88(r1)
	.globl __savegprlr_23
	.type __savegprlr_23, @function
__savegprlr_23:
	.long 0xFAE1FFB0	# std r23,-80(r1)
	.globl __savegprlr_24
	.type __savegprlr_24, @function
__savegprlr_24:
	.long 0xFB01FFB8	# std r24,-72(r1)
	.globl __savegprlr_25
	.type __savegprlr_25, @function
__savegprlr_25:
	.long 0xFB21FFC0	# std r25,-64(r1)
	.globl __savegprlr_26
	.type __savegprlr_26, @function
__savegprlr_26:
	.long 0xFB41FFC8	# std r26,-56(r1)
	.globl __savegprlr_27
	.type __savegprlr_27, @function
__savegprlr_27:
	.long 0xFB61FFD0	# std r27,-48(r1)
	.globl __savegprlr_28
	.type __savegprlr_28, @function
__savegprlr_28:
	.long 0xFB81FFD8	# std r28,-40(r1)
	.globl __savegprlr_29
	.type __savegprlr_29, @function
__savegprlr_29:
	.long 0xFBA1FFE0	# std r29,-32(r1)
	.globl __savegprlr_30
	.type __savegprlr_30, @function
__savegprlr_30:
	.long 0xFBC1FFE8	# std r30,-24(r1)
	.globl __savegprlr_31
	.type __savegprlr_31, @function
__savegprlr_31:
	.long 0xFBE1FFF0	# std r31,-16(r1)
	stw 12, -8(1)
	blr
	.globl __restgprlr_14
	.type __restgprlr_14, @function
__restgprlr_14:
	.long 0xE9C1FF68	# ld r14,-152(r1)
	.globl __restgprlr_15
	.type __restgprlr_15, @function
__restgprlr_15:
	.long 0xE9E1FF70	# ld r15,-144(r1)
	.globl __restgprlr_16
	.type __restgprlr_16, @function
__restgprlr_16:
	.long 0xEA01FF78	# ld r16,-136(r1)
	.globl __restgprlr_17
	.type __restgprlr_17, @function
__restgprlr_17:
	.long 0xEA21FF80	# ld r17,-128(r1)
	.globl __restgprlr_18
	.type __restgprlr_18, @function
__restgprlr_18:
	.long 0xEA41FF88	# ld r18,-120(r1)
	.globl __restgprlr_19
	.type __restgprlr_19, @function
__restgprlr_19:
	.long 0xEA61FF90	# ld r19,-112(r1)
	.globl __restgprlr_20
	.type __restgprlr_20, @function
__restgprlr_20:
	.long 0xEA81FF98	# ld r20,-104(r1)
	.globl __restgprlr_21
	.type __restgprlr_21, @function
__restgprlr_21:
	.long 0xEAA1FFA0	# ld r21,-96(r1)
	.globl __restgprlr_22
	.type __restgprlr_22, @function
__restgprlr_22:
	.long 0xEAC1FFA8	# ld r22,-88(r1)
	.globl __restgprlr_23
	.type __restgprlr_23, @function
__restgprlr_23:
	.long 0xEAE1FFB0	# ld r23,-80(r1)
	.globl __restgprlr_24
	.type __restgprlr_24, @function
__restgprlr_24:
	.long 0xEB01FFB8	# ld r24,-72(r1)
	.globl __restgprlr_25
	.type __restgprlr_25, @function
__restgprlr_25:
	.long 0xEB21FFC0	# ld r25,-64(r1)
	.globl __restgprlr_26
	.type __restgprlr_26, @function
__restgprlr_26:
	.long 0xEB41FFC8	# ld r26,-56(r1)
	.globl __restgprlr_27
	.type __restgprlr_27, @function
__restgprlr_27:
	.long 0xEB61FFD0	# ld r27,-48(r1)
	.globl __restgprlr_28
	.type __restgprlr_28, @function
__restgprlr_28:
	.long 0xEB81FFD8	# ld r28,-40(r1)
	.globl __restgprlr_29
	.type __restgprlr_29, @function
__restgprlr_29:
	.long 0xEBA1FFE0	# ld r29,-32(r1)
	.globl __restgprlr_30
	.type __restgprlr_30, @function
__restgprlr_30:
	.long 0xEBC1FFE8	# ld r30,-24(r1)
	.globl __restgprlr_31
	.type __restgprlr_31, @function
__restgprlr_31:
	.long 0xEBE1FFF0	# ld r31,-16(r1)
	lwz 12, -8(1)
	mtlr 12
	blr
	.globl __savefpr_14
	.type __savefpr_14, @function
__savefpr_14:
	stfd 14, -144(12)
	.globl __savefpr_15
	.type __savefpr_15, @function
__savefpr_15:
	stfd 15, -136(12)
	.globl __savefpr_16
	.type __savefpr_16, @function
__savefpr_16:
	stfd 16, -128(12)
	.globl __savefpr_17
	.type __savefpr_17, @function
__savefpr_17:
	stfd 17, -120(12)
	.globl __savefpr_18
	.type __savefpr_18, @function
__savefpr_18:
	stfd 18, -112(12)
	.globl __savefpr_19
	.type __savefpr_19, @function
__savefpr_19:
	stfd 19, -104(12)
	.globl __savefpr_20
	.type __savefpr_20, @function
__savefpr_20:
	stfd 20, -96(12)
	.globl __savefpr_21
	.type __savefpr_21, @function
__savefpr_21:
	stfd 21, -88(12)
	.globl __savefpr_22
	.type __savefpr_22, @function
__savefpr_22:
	stfd 22, -80(12)
	.globl __savefpr_23
	.type __savefpr_23, @function
__savefpr_23:
	stfd 23, -72(12)
	.globl __savefpr_24
	.type __savefpr_24, @function
__savefpr_24:
	stfd 24, -64(12)
	.globl __savefpr_25
	.type __savefpr_25, @function
__savefpr_25:
	stfd 25, -56(12)
	.globl __savefpr_26
	.type __savefpr_26, @function
__savefpr_26:
	stfd 26, -48(12)
	.globl __savefpr_27
	.type __savefpr_27, @function
__savefpr_27:
	stfd 27, -40(12)
	.globl __savefpr_28
	.type __savefpr_28, @function
__savefpr_28:
	stfd 28, -32(12)
	.globl __savefpr_29
	.type __savefpr_29, @function
__savefpr_29:
	stfd 29, -24(12)
	.globl __savefpr_30
	.type __savefpr_30, @function
__savefpr_30:
	stfd 30, -16(12)
	.globl __savefpr_31
	.type __savefpr_31, @function
__savefpr_31:
	stfd 31, -8(12)
	blr
	.globl __restfpr_14
	.type __restfpr_14, @function
__restfpr_14:
	lfd 14, -144(12)
	.globl __restfpr_15
	.type __restfpr_15, @function
__restfpr_15:
	lfd 15, -136(12)
	.globl __restfpr_16
	.type __restfpr_16, @function
__restfpr_16:
	lfd 16, -128(12)
	.globl __restfpr_17
	.type __restfpr_17, @function
__restfpr_17:
	lfd 17, -120(12)
	.globl __restfpr_18
	.type __restfpr_18, @function
__restfpr_18:
	lfd 18, -112(12)
	.globl __restfpr_19
	.type __restfpr_19, @function
__restfpr_19:
	lfd 19, -104(12)
	.globl __restfpr_20
	.type __restfpr_20, @function
__restfpr_20:
	lfd 20, -96(12)
	.globl __restfpr_21
	.type __restfpr_21, @function
__restfpr_21:
	lfd 21, -88(12)
	.globl __restfpr_22
	.type __restfpr_22, @function
__restfpr_22:
	lfd 22, -80(12)
	.globl __restfpr_23
	.type __restfpr_23, @function
__restfpr_23:
	lfd 23, -72(12)
	.globl __restfpr_24
	.type __restfpr_24, @function
__restfpr_24:
	lfd 24, -64(12)
	.globl __restfpr_25
	.type __restfpr_25, @function
__restfpr_25:
	lfd 25, -56(12)
	.globl __restfpr_26
	.type __restfpr_26, @function
__restfpr_26:
	lfd 26, -48(12)
	.globl __restfpr_27
	.type __restfpr_27, @function
__restfpr_27:
	lfd 27, -40(12)
	.globl __restfpr_28
	.type __restfpr_28, @function
__restfpr_28:
	lfd 28, -32(12)
	.globl __restfpr_29
	.type __restfpr_29, @function
__restfpr_29:
	lfd 29, -24(12)
	.globl __restfpr_30
	.type __restfpr_30, @function
__restfpr_30:
	lfd 30, -16(12)
	.globl __restfpr_31
	.type __restfpr_31, @function
__restfpr_31:
	lfd 31, -8(12)
	blr
	.globl _blkmov
	.type _blkmov, @function
_blkmov:
	cmplwi 5, 0
	beqlr 0
	or 6, 3, 4
	or 6, 6, 5
	andi. 6, 6, 3
	bne 0, .Lblkmov_bytes
	srwi 5, 5, 2
	mtctr 5
	addi 3, 3, -4
	addi 4, 4, -4
.Lblkmov_words:
	lwzu 6, 4(4)
	stwu 6, 4(3)
	bdnz .Lblkmov_words
	blr
.Lblkmov_bytes:
	mtctr 5
	addi 3, 3, -1
	addi 4, 4, -1
.Lblkmov_b:
	lbzu 6, 1(4)
	stbu 6, 1(3)
	bdnz .Lblkmov_b
	blr
