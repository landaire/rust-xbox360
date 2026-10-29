# Vector register probes for the register tests.

# vmx_clobber_volatile(): overwrites v0-v63, which the convention lets
# every call change, with values the tests never use.
	.section .text.vmx_clobber_volatile,"ax",@progbits
	.p2align 4
	.globl vmx_clobber_volatile
	.type vmx_clobber_volatile, @function
vmx_clobber_volatile:
	vspltisw128 0, -15
	vspltisw128 1, -14
	vspltisw128 2, -13
	vspltisw128 3, -12
	vspltisw128 4, -11
	vspltisw128 5, -10
	vspltisw128 6, -9
	vspltisw128 7, -8
	vspltisw128 8, -7
	vspltisw128 9, -6
	vspltisw128 10, -5
	vspltisw128 11, -4
	vspltisw128 12, -3
	vspltisw128 13, -2
	vspltisw128 14, -1
	vspltisw128 15, 0
	vspltisw128 16, 1
	vspltisw128 17, 2
	vspltisw128 18, 3
	vspltisw128 19, 4
	vspltisw128 20, 5
	vspltisw128 21, 6
	vspltisw128 22, 7
	vspltisw128 23, 8
	vspltisw128 24, 9
	vspltisw128 25, 10
	vspltisw128 26, 11
	vspltisw128 27, 12
	vspltisw128 28, 13
	vspltisw128 29, 14
	vspltisw128 30, 15
	vspltisw128 31, -15
	vspltisw128 32, -14
	vspltisw128 33, -13
	vspltisw128 34, -12
	vspltisw128 35, -11
	vspltisw128 36, -10
	vspltisw128 37, -9
	vspltisw128 38, -8
	vspltisw128 39, -7
	vspltisw128 40, -6
	vspltisw128 41, -5
	vspltisw128 42, -4
	vspltisw128 43, -3
	vspltisw128 44, -2
	vspltisw128 45, -1
	vspltisw128 46, 0
	vspltisw128 47, 1
	vspltisw128 48, 2
	vspltisw128 49, 3
	vspltisw128 50, 4
	vspltisw128 51, 5
	vspltisw128 52, 6
	vspltisw128 53, 7
	vspltisw128 54, 8
	vspltisw128 55, 9
	vspltisw128 56, 10
	vspltisw128 57, 11
	vspltisw128 58, 12
	vspltisw128 59, 13
	vspltisw128 60, 14
	vspltisw128 61, 15
	vspltisw128 62, -15
	vspltisw128 63, -14
	blr
	.size vmx_clobber_volatile, .-vmx_clobber_volatile

# vmx_csr_probe(f: r3, pattern: r4, out: r5): saves the caller's
# v64-v127, loads v64-v127 from the 64 vectors at `pattern`, calls f,
# stores v64-v127 to the 64 vectors at `out`, and restores the caller's
# registers. Frame: LR at -8, r29-r31 at -32..-16 (SP_in relative),
# saved vectors at SP+80 (16-aligned), 80 + 1024 + 32 = 1136 bytes.
	.section .text.vmx_csr_probe,"ax",@progbits
	.p2align 4
	.globl vmx_csr_probe
	.type vmx_csr_probe, @function
vmx_csr_probe:
	mflr 12
	stw 12, -8(1)
	std 29, -32(1)
	std 30, -24(1)
	std 31, -16(1)
	stwu 1, -1136(1)
	mr 29, 3
	mr 30, 4
	mr 31, 5
	addi 11, 1, 80
	li 12, 0
	stvx128 64, 11, 12
	li 12, 16
	stvx128 65, 11, 12
	li 12, 32
	stvx128 66, 11, 12
	li 12, 48
	stvx128 67, 11, 12
	li 12, 64
	stvx128 68, 11, 12
	li 12, 80
	stvx128 69, 11, 12
	li 12, 96
	stvx128 70, 11, 12
	li 12, 112
	stvx128 71, 11, 12
	li 12, 128
	stvx128 72, 11, 12
	li 12, 144
	stvx128 73, 11, 12
	li 12, 160
	stvx128 74, 11, 12
	li 12, 176
	stvx128 75, 11, 12
	li 12, 192
	stvx128 76, 11, 12
	li 12, 208
	stvx128 77, 11, 12
	li 12, 224
	stvx128 78, 11, 12
	li 12, 240
	stvx128 79, 11, 12
	li 12, 256
	stvx128 80, 11, 12
	li 12, 272
	stvx128 81, 11, 12
	li 12, 288
	stvx128 82, 11, 12
	li 12, 304
	stvx128 83, 11, 12
	li 12, 320
	stvx128 84, 11, 12
	li 12, 336
	stvx128 85, 11, 12
	li 12, 352
	stvx128 86, 11, 12
	li 12, 368
	stvx128 87, 11, 12
	li 12, 384
	stvx128 88, 11, 12
	li 12, 400
	stvx128 89, 11, 12
	li 12, 416
	stvx128 90, 11, 12
	li 12, 432
	stvx128 91, 11, 12
	li 12, 448
	stvx128 92, 11, 12
	li 12, 464
	stvx128 93, 11, 12
	li 12, 480
	stvx128 94, 11, 12
	li 12, 496
	stvx128 95, 11, 12
	li 12, 512
	stvx128 96, 11, 12
	li 12, 528
	stvx128 97, 11, 12
	li 12, 544
	stvx128 98, 11, 12
	li 12, 560
	stvx128 99, 11, 12
	li 12, 576
	stvx128 100, 11, 12
	li 12, 592
	stvx128 101, 11, 12
	li 12, 608
	stvx128 102, 11, 12
	li 12, 624
	stvx128 103, 11, 12
	li 12, 640
	stvx128 104, 11, 12
	li 12, 656
	stvx128 105, 11, 12
	li 12, 672
	stvx128 106, 11, 12
	li 12, 688
	stvx128 107, 11, 12
	li 12, 704
	stvx128 108, 11, 12
	li 12, 720
	stvx128 109, 11, 12
	li 12, 736
	stvx128 110, 11, 12
	li 12, 752
	stvx128 111, 11, 12
	li 12, 768
	stvx128 112, 11, 12
	li 12, 784
	stvx128 113, 11, 12
	li 12, 800
	stvx128 114, 11, 12
	li 12, 816
	stvx128 115, 11, 12
	li 12, 832
	stvx128 116, 11, 12
	li 12, 848
	stvx128 117, 11, 12
	li 12, 864
	stvx128 118, 11, 12
	li 12, 880
	stvx128 119, 11, 12
	li 12, 896
	stvx128 120, 11, 12
	li 12, 912
	stvx128 121, 11, 12
	li 12, 928
	stvx128 122, 11, 12
	li 12, 944
	stvx128 123, 11, 12
	li 12, 960
	stvx128 124, 11, 12
	li 12, 976
	stvx128 125, 11, 12
	li 12, 992
	stvx128 126, 11, 12
	li 12, 1008
	stvx128 127, 11, 12
	li 12, 0
	lvx128 64, 30, 12
	li 12, 16
	lvx128 65, 30, 12
	li 12, 32
	lvx128 66, 30, 12
	li 12, 48
	lvx128 67, 30, 12
	li 12, 64
	lvx128 68, 30, 12
	li 12, 80
	lvx128 69, 30, 12
	li 12, 96
	lvx128 70, 30, 12
	li 12, 112
	lvx128 71, 30, 12
	li 12, 128
	lvx128 72, 30, 12
	li 12, 144
	lvx128 73, 30, 12
	li 12, 160
	lvx128 74, 30, 12
	li 12, 176
	lvx128 75, 30, 12
	li 12, 192
	lvx128 76, 30, 12
	li 12, 208
	lvx128 77, 30, 12
	li 12, 224
	lvx128 78, 30, 12
	li 12, 240
	lvx128 79, 30, 12
	li 12, 256
	lvx128 80, 30, 12
	li 12, 272
	lvx128 81, 30, 12
	li 12, 288
	lvx128 82, 30, 12
	li 12, 304
	lvx128 83, 30, 12
	li 12, 320
	lvx128 84, 30, 12
	li 12, 336
	lvx128 85, 30, 12
	li 12, 352
	lvx128 86, 30, 12
	li 12, 368
	lvx128 87, 30, 12
	li 12, 384
	lvx128 88, 30, 12
	li 12, 400
	lvx128 89, 30, 12
	li 12, 416
	lvx128 90, 30, 12
	li 12, 432
	lvx128 91, 30, 12
	li 12, 448
	lvx128 92, 30, 12
	li 12, 464
	lvx128 93, 30, 12
	li 12, 480
	lvx128 94, 30, 12
	li 12, 496
	lvx128 95, 30, 12
	li 12, 512
	lvx128 96, 30, 12
	li 12, 528
	lvx128 97, 30, 12
	li 12, 544
	lvx128 98, 30, 12
	li 12, 560
	lvx128 99, 30, 12
	li 12, 576
	lvx128 100, 30, 12
	li 12, 592
	lvx128 101, 30, 12
	li 12, 608
	lvx128 102, 30, 12
	li 12, 624
	lvx128 103, 30, 12
	li 12, 640
	lvx128 104, 30, 12
	li 12, 656
	lvx128 105, 30, 12
	li 12, 672
	lvx128 106, 30, 12
	li 12, 688
	lvx128 107, 30, 12
	li 12, 704
	lvx128 108, 30, 12
	li 12, 720
	lvx128 109, 30, 12
	li 12, 736
	lvx128 110, 30, 12
	li 12, 752
	lvx128 111, 30, 12
	li 12, 768
	lvx128 112, 30, 12
	li 12, 784
	lvx128 113, 30, 12
	li 12, 800
	lvx128 114, 30, 12
	li 12, 816
	lvx128 115, 30, 12
	li 12, 832
	lvx128 116, 30, 12
	li 12, 848
	lvx128 117, 30, 12
	li 12, 864
	lvx128 118, 30, 12
	li 12, 880
	lvx128 119, 30, 12
	li 12, 896
	lvx128 120, 30, 12
	li 12, 912
	lvx128 121, 30, 12
	li 12, 928
	lvx128 122, 30, 12
	li 12, 944
	lvx128 123, 30, 12
	li 12, 960
	lvx128 124, 30, 12
	li 12, 976
	lvx128 125, 30, 12
	li 12, 992
	lvx128 126, 30, 12
	li 12, 1008
	lvx128 127, 30, 12
	mtctr 29
	bctrl
	li 12, 0
	stvx128 64, 31, 12
	li 12, 16
	stvx128 65, 31, 12
	li 12, 32
	stvx128 66, 31, 12
	li 12, 48
	stvx128 67, 31, 12
	li 12, 64
	stvx128 68, 31, 12
	li 12, 80
	stvx128 69, 31, 12
	li 12, 96
	stvx128 70, 31, 12
	li 12, 112
	stvx128 71, 31, 12
	li 12, 128
	stvx128 72, 31, 12
	li 12, 144
	stvx128 73, 31, 12
	li 12, 160
	stvx128 74, 31, 12
	li 12, 176
	stvx128 75, 31, 12
	li 12, 192
	stvx128 76, 31, 12
	li 12, 208
	stvx128 77, 31, 12
	li 12, 224
	stvx128 78, 31, 12
	li 12, 240
	stvx128 79, 31, 12
	li 12, 256
	stvx128 80, 31, 12
	li 12, 272
	stvx128 81, 31, 12
	li 12, 288
	stvx128 82, 31, 12
	li 12, 304
	stvx128 83, 31, 12
	li 12, 320
	stvx128 84, 31, 12
	li 12, 336
	stvx128 85, 31, 12
	li 12, 352
	stvx128 86, 31, 12
	li 12, 368
	stvx128 87, 31, 12
	li 12, 384
	stvx128 88, 31, 12
	li 12, 400
	stvx128 89, 31, 12
	li 12, 416
	stvx128 90, 31, 12
	li 12, 432
	stvx128 91, 31, 12
	li 12, 448
	stvx128 92, 31, 12
	li 12, 464
	stvx128 93, 31, 12
	li 12, 480
	stvx128 94, 31, 12
	li 12, 496
	stvx128 95, 31, 12
	li 12, 512
	stvx128 96, 31, 12
	li 12, 528
	stvx128 97, 31, 12
	li 12, 544
	stvx128 98, 31, 12
	li 12, 560
	stvx128 99, 31, 12
	li 12, 576
	stvx128 100, 31, 12
	li 12, 592
	stvx128 101, 31, 12
	li 12, 608
	stvx128 102, 31, 12
	li 12, 624
	stvx128 103, 31, 12
	li 12, 640
	stvx128 104, 31, 12
	li 12, 656
	stvx128 105, 31, 12
	li 12, 672
	stvx128 106, 31, 12
	li 12, 688
	stvx128 107, 31, 12
	li 12, 704
	stvx128 108, 31, 12
	li 12, 720
	stvx128 109, 31, 12
	li 12, 736
	stvx128 110, 31, 12
	li 12, 752
	stvx128 111, 31, 12
	li 12, 768
	stvx128 112, 31, 12
	li 12, 784
	stvx128 113, 31, 12
	li 12, 800
	stvx128 114, 31, 12
	li 12, 816
	stvx128 115, 31, 12
	li 12, 832
	stvx128 116, 31, 12
	li 12, 848
	stvx128 117, 31, 12
	li 12, 864
	stvx128 118, 31, 12
	li 12, 880
	stvx128 119, 31, 12
	li 12, 896
	stvx128 120, 31, 12
	li 12, 912
	stvx128 121, 31, 12
	li 12, 928
	stvx128 122, 31, 12
	li 12, 944
	stvx128 123, 31, 12
	li 12, 960
	stvx128 124, 31, 12
	li 12, 976
	stvx128 125, 31, 12
	li 12, 992
	stvx128 126, 31, 12
	li 12, 1008
	stvx128 127, 31, 12
	addi 11, 1, 80
	li 12, 0
	lvx128 64, 11, 12
	li 12, 16
	lvx128 65, 11, 12
	li 12, 32
	lvx128 66, 11, 12
	li 12, 48
	lvx128 67, 11, 12
	li 12, 64
	lvx128 68, 11, 12
	li 12, 80
	lvx128 69, 11, 12
	li 12, 96
	lvx128 70, 11, 12
	li 12, 112
	lvx128 71, 11, 12
	li 12, 128
	lvx128 72, 11, 12
	li 12, 144
	lvx128 73, 11, 12
	li 12, 160
	lvx128 74, 11, 12
	li 12, 176
	lvx128 75, 11, 12
	li 12, 192
	lvx128 76, 11, 12
	li 12, 208
	lvx128 77, 11, 12
	li 12, 224
	lvx128 78, 11, 12
	li 12, 240
	lvx128 79, 11, 12
	li 12, 256
	lvx128 80, 11, 12
	li 12, 272
	lvx128 81, 11, 12
	li 12, 288
	lvx128 82, 11, 12
	li 12, 304
	lvx128 83, 11, 12
	li 12, 320
	lvx128 84, 11, 12
	li 12, 336
	lvx128 85, 11, 12
	li 12, 352
	lvx128 86, 11, 12
	li 12, 368
	lvx128 87, 11, 12
	li 12, 384
	lvx128 88, 11, 12
	li 12, 400
	lvx128 89, 11, 12
	li 12, 416
	lvx128 90, 11, 12
	li 12, 432
	lvx128 91, 11, 12
	li 12, 448
	lvx128 92, 11, 12
	li 12, 464
	lvx128 93, 11, 12
	li 12, 480
	lvx128 94, 11, 12
	li 12, 496
	lvx128 95, 11, 12
	li 12, 512
	lvx128 96, 11, 12
	li 12, 528
	lvx128 97, 11, 12
	li 12, 544
	lvx128 98, 11, 12
	li 12, 560
	lvx128 99, 11, 12
	li 12, 576
	lvx128 100, 11, 12
	li 12, 592
	lvx128 101, 11, 12
	li 12, 608
	lvx128 102, 11, 12
	li 12, 624
	lvx128 103, 11, 12
	li 12, 640
	lvx128 104, 11, 12
	li 12, 656
	lvx128 105, 11, 12
	li 12, 672
	lvx128 106, 11, 12
	li 12, 688
	lvx128 107, 11, 12
	li 12, 704
	lvx128 108, 11, 12
	li 12, 720
	lvx128 109, 11, 12
	li 12, 736
	lvx128 110, 11, 12
	li 12, 752
	lvx128 111, 11, 12
	li 12, 768
	lvx128 112, 11, 12
	li 12, 784
	lvx128 113, 11, 12
	li 12, 800
	lvx128 114, 11, 12
	li 12, 816
	lvx128 115, 11, 12
	li 12, 832
	lvx128 116, 11, 12
	li 12, 848
	lvx128 117, 11, 12
	li 12, 864
	lvx128 118, 11, 12
	li 12, 880
	lvx128 119, 11, 12
	li 12, 896
	lvx128 120, 11, 12
	li 12, 912
	lvx128 121, 11, 12
	li 12, 928
	lvx128 122, 11, 12
	li 12, 944
	lvx128 123, 11, 12
	li 12, 960
	lvx128 124, 11, 12
	li 12, 976
	lvx128 125, 11, 12
	li 12, 992
	lvx128 126, 11, 12
	li 12, 1008
	lvx128 127, 11, 12
	addi 1, 1, 1136
	lwz 12, -8(1)
	mtlr 12
	ld 29, -32(1)
	ld 30, -24(1)
	ld 31, -16(1)
	blr
	.size vmx_csr_probe, .-vmx_csr_probe
