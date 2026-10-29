# Out-of-line helpers that cl.exe output calls (msvc/interop.s), written
# from the behaviour of the console compiler's call sites:
#   __savegprlr_N: std rK at -8-8*(32-K)(r1) for K = N..31, then the
#     link register, which the caller moved to r12, at -8(r1).
#   __restgprlr_N: reloads them and returns through the reloaded link
#     register; the epilogue branches to it after popping its frame.
#   __savevmx_N / __restvmx_N (N = 64..127): vK at r12 - 16*(128-K) for
#     K = N..127; r12 is the save area top set by the caller, r11 is
#     clobbered.
# Each entry falls through to the next register.
	.section .text.vmx128test_msvc_runtime,"ax",@progbits
	.p2align 4
	.globl __savegprlr_14
__savegprlr_14:
	std 14, -152(1)
	.globl __savegprlr_15
__savegprlr_15:
	std 15, -144(1)
	.globl __savegprlr_16
__savegprlr_16:
	std 16, -136(1)
	.globl __savegprlr_17
__savegprlr_17:
	std 17, -128(1)
	.globl __savegprlr_18
__savegprlr_18:
	std 18, -120(1)
	.globl __savegprlr_19
__savegprlr_19:
	std 19, -112(1)
	.globl __savegprlr_20
__savegprlr_20:
	std 20, -104(1)
	.globl __savegprlr_21
__savegprlr_21:
	std 21, -96(1)
	.globl __savegprlr_22
__savegprlr_22:
	std 22, -88(1)
	.globl __savegprlr_23
__savegprlr_23:
	std 23, -80(1)
	.globl __savegprlr_24
__savegprlr_24:
	std 24, -72(1)
	.globl __savegprlr_25
__savegprlr_25:
	std 25, -64(1)
	.globl __savegprlr_26
__savegprlr_26:
	std 26, -56(1)
	.globl __savegprlr_27
__savegprlr_27:
	std 27, -48(1)
	.globl __savegprlr_28
__savegprlr_28:
	std 28, -40(1)
	.globl __savegprlr_29
__savegprlr_29:
	std 29, -32(1)
	.globl __savegprlr_30
__savegprlr_30:
	std 30, -24(1)
	.globl __savegprlr_31
__savegprlr_31:
	std 31, -16(1)
	stw 12, -8(1)
	blr
	.globl __restgprlr_14
__restgprlr_14:
	ld 14, -152(1)
	.globl __restgprlr_15
__restgprlr_15:
	ld 15, -144(1)
	.globl __restgprlr_16
__restgprlr_16:
	ld 16, -136(1)
	.globl __restgprlr_17
__restgprlr_17:
	ld 17, -128(1)
	.globl __restgprlr_18
__restgprlr_18:
	ld 18, -120(1)
	.globl __restgprlr_19
__restgprlr_19:
	ld 19, -112(1)
	.globl __restgprlr_20
__restgprlr_20:
	ld 20, -104(1)
	.globl __restgprlr_21
__restgprlr_21:
	ld 21, -96(1)
	.globl __restgprlr_22
__restgprlr_22:
	ld 22, -88(1)
	.globl __restgprlr_23
__restgprlr_23:
	ld 23, -80(1)
	.globl __restgprlr_24
__restgprlr_24:
	ld 24, -72(1)
	.globl __restgprlr_25
__restgprlr_25:
	ld 25, -64(1)
	.globl __restgprlr_26
__restgprlr_26:
	ld 26, -56(1)
	.globl __restgprlr_27
__restgprlr_27:
	ld 27, -48(1)
	.globl __restgprlr_28
__restgprlr_28:
	ld 28, -40(1)
	.globl __restgprlr_29
__restgprlr_29:
	ld 29, -32(1)
	.globl __restgprlr_30
__restgprlr_30:
	ld 30, -24(1)
	.globl __restgprlr_31
__restgprlr_31:
	ld 31, -16(1)
	lwz 12, -8(1)
	mtlr 12
	blr
	.globl __savevmx_64
__savevmx_64:
	li 11, -1024
	stvx128 64, 11, 12
	.globl __savevmx_65
__savevmx_65:
	li 11, -1008
	stvx128 65, 11, 12
	.globl __savevmx_66
__savevmx_66:
	li 11, -992
	stvx128 66, 11, 12
	.globl __savevmx_67
__savevmx_67:
	li 11, -976
	stvx128 67, 11, 12
	.globl __savevmx_68
__savevmx_68:
	li 11, -960
	stvx128 68, 11, 12
	.globl __savevmx_69
__savevmx_69:
	li 11, -944
	stvx128 69, 11, 12
	.globl __savevmx_70
__savevmx_70:
	li 11, -928
	stvx128 70, 11, 12
	.globl __savevmx_71
__savevmx_71:
	li 11, -912
	stvx128 71, 11, 12
	.globl __savevmx_72
__savevmx_72:
	li 11, -896
	stvx128 72, 11, 12
	.globl __savevmx_73
__savevmx_73:
	li 11, -880
	stvx128 73, 11, 12
	.globl __savevmx_74
__savevmx_74:
	li 11, -864
	stvx128 74, 11, 12
	.globl __savevmx_75
__savevmx_75:
	li 11, -848
	stvx128 75, 11, 12
	.globl __savevmx_76
__savevmx_76:
	li 11, -832
	stvx128 76, 11, 12
	.globl __savevmx_77
__savevmx_77:
	li 11, -816
	stvx128 77, 11, 12
	.globl __savevmx_78
__savevmx_78:
	li 11, -800
	stvx128 78, 11, 12
	.globl __savevmx_79
__savevmx_79:
	li 11, -784
	stvx128 79, 11, 12
	.globl __savevmx_80
__savevmx_80:
	li 11, -768
	stvx128 80, 11, 12
	.globl __savevmx_81
__savevmx_81:
	li 11, -752
	stvx128 81, 11, 12
	.globl __savevmx_82
__savevmx_82:
	li 11, -736
	stvx128 82, 11, 12
	.globl __savevmx_83
__savevmx_83:
	li 11, -720
	stvx128 83, 11, 12
	.globl __savevmx_84
__savevmx_84:
	li 11, -704
	stvx128 84, 11, 12
	.globl __savevmx_85
__savevmx_85:
	li 11, -688
	stvx128 85, 11, 12
	.globl __savevmx_86
__savevmx_86:
	li 11, -672
	stvx128 86, 11, 12
	.globl __savevmx_87
__savevmx_87:
	li 11, -656
	stvx128 87, 11, 12
	.globl __savevmx_88
__savevmx_88:
	li 11, -640
	stvx128 88, 11, 12
	.globl __savevmx_89
__savevmx_89:
	li 11, -624
	stvx128 89, 11, 12
	.globl __savevmx_90
__savevmx_90:
	li 11, -608
	stvx128 90, 11, 12
	.globl __savevmx_91
__savevmx_91:
	li 11, -592
	stvx128 91, 11, 12
	.globl __savevmx_92
__savevmx_92:
	li 11, -576
	stvx128 92, 11, 12
	.globl __savevmx_93
__savevmx_93:
	li 11, -560
	stvx128 93, 11, 12
	.globl __savevmx_94
__savevmx_94:
	li 11, -544
	stvx128 94, 11, 12
	.globl __savevmx_95
__savevmx_95:
	li 11, -528
	stvx128 95, 11, 12
	.globl __savevmx_96
__savevmx_96:
	li 11, -512
	stvx128 96, 11, 12
	.globl __savevmx_97
__savevmx_97:
	li 11, -496
	stvx128 97, 11, 12
	.globl __savevmx_98
__savevmx_98:
	li 11, -480
	stvx128 98, 11, 12
	.globl __savevmx_99
__savevmx_99:
	li 11, -464
	stvx128 99, 11, 12
	.globl __savevmx_100
__savevmx_100:
	li 11, -448
	stvx128 100, 11, 12
	.globl __savevmx_101
__savevmx_101:
	li 11, -432
	stvx128 101, 11, 12
	.globl __savevmx_102
__savevmx_102:
	li 11, -416
	stvx128 102, 11, 12
	.globl __savevmx_103
__savevmx_103:
	li 11, -400
	stvx128 103, 11, 12
	.globl __savevmx_104
__savevmx_104:
	li 11, -384
	stvx128 104, 11, 12
	.globl __savevmx_105
__savevmx_105:
	li 11, -368
	stvx128 105, 11, 12
	.globl __savevmx_106
__savevmx_106:
	li 11, -352
	stvx128 106, 11, 12
	.globl __savevmx_107
__savevmx_107:
	li 11, -336
	stvx128 107, 11, 12
	.globl __savevmx_108
__savevmx_108:
	li 11, -320
	stvx128 108, 11, 12
	.globl __savevmx_109
__savevmx_109:
	li 11, -304
	stvx128 109, 11, 12
	.globl __savevmx_110
__savevmx_110:
	li 11, -288
	stvx128 110, 11, 12
	.globl __savevmx_111
__savevmx_111:
	li 11, -272
	stvx128 111, 11, 12
	.globl __savevmx_112
__savevmx_112:
	li 11, -256
	stvx128 112, 11, 12
	.globl __savevmx_113
__savevmx_113:
	li 11, -240
	stvx128 113, 11, 12
	.globl __savevmx_114
__savevmx_114:
	li 11, -224
	stvx128 114, 11, 12
	.globl __savevmx_115
__savevmx_115:
	li 11, -208
	stvx128 115, 11, 12
	.globl __savevmx_116
__savevmx_116:
	li 11, -192
	stvx128 116, 11, 12
	.globl __savevmx_117
__savevmx_117:
	li 11, -176
	stvx128 117, 11, 12
	.globl __savevmx_118
__savevmx_118:
	li 11, -160
	stvx128 118, 11, 12
	.globl __savevmx_119
__savevmx_119:
	li 11, -144
	stvx128 119, 11, 12
	.globl __savevmx_120
__savevmx_120:
	li 11, -128
	stvx128 120, 11, 12
	.globl __savevmx_121
__savevmx_121:
	li 11, -112
	stvx128 121, 11, 12
	.globl __savevmx_122
__savevmx_122:
	li 11, -96
	stvx128 122, 11, 12
	.globl __savevmx_123
__savevmx_123:
	li 11, -80
	stvx128 123, 11, 12
	.globl __savevmx_124
__savevmx_124:
	li 11, -64
	stvx128 124, 11, 12
	.globl __savevmx_125
__savevmx_125:
	li 11, -48
	stvx128 125, 11, 12
	.globl __savevmx_126
__savevmx_126:
	li 11, -32
	stvx128 126, 11, 12
	.globl __savevmx_127
__savevmx_127:
	li 11, -16
	stvx128 127, 11, 12
	blr
	.globl __restvmx_64
__restvmx_64:
	li 11, -1024
	lvx128 64, 11, 12
	.globl __restvmx_65
__restvmx_65:
	li 11, -1008
	lvx128 65, 11, 12
	.globl __restvmx_66
__restvmx_66:
	li 11, -992
	lvx128 66, 11, 12
	.globl __restvmx_67
__restvmx_67:
	li 11, -976
	lvx128 67, 11, 12
	.globl __restvmx_68
__restvmx_68:
	li 11, -960
	lvx128 68, 11, 12
	.globl __restvmx_69
__restvmx_69:
	li 11, -944
	lvx128 69, 11, 12
	.globl __restvmx_70
__restvmx_70:
	li 11, -928
	lvx128 70, 11, 12
	.globl __restvmx_71
__restvmx_71:
	li 11, -912
	lvx128 71, 11, 12
	.globl __restvmx_72
__restvmx_72:
	li 11, -896
	lvx128 72, 11, 12
	.globl __restvmx_73
__restvmx_73:
	li 11, -880
	lvx128 73, 11, 12
	.globl __restvmx_74
__restvmx_74:
	li 11, -864
	lvx128 74, 11, 12
	.globl __restvmx_75
__restvmx_75:
	li 11, -848
	lvx128 75, 11, 12
	.globl __restvmx_76
__restvmx_76:
	li 11, -832
	lvx128 76, 11, 12
	.globl __restvmx_77
__restvmx_77:
	li 11, -816
	lvx128 77, 11, 12
	.globl __restvmx_78
__restvmx_78:
	li 11, -800
	lvx128 78, 11, 12
	.globl __restvmx_79
__restvmx_79:
	li 11, -784
	lvx128 79, 11, 12
	.globl __restvmx_80
__restvmx_80:
	li 11, -768
	lvx128 80, 11, 12
	.globl __restvmx_81
__restvmx_81:
	li 11, -752
	lvx128 81, 11, 12
	.globl __restvmx_82
__restvmx_82:
	li 11, -736
	lvx128 82, 11, 12
	.globl __restvmx_83
__restvmx_83:
	li 11, -720
	lvx128 83, 11, 12
	.globl __restvmx_84
__restvmx_84:
	li 11, -704
	lvx128 84, 11, 12
	.globl __restvmx_85
__restvmx_85:
	li 11, -688
	lvx128 85, 11, 12
	.globl __restvmx_86
__restvmx_86:
	li 11, -672
	lvx128 86, 11, 12
	.globl __restvmx_87
__restvmx_87:
	li 11, -656
	lvx128 87, 11, 12
	.globl __restvmx_88
__restvmx_88:
	li 11, -640
	lvx128 88, 11, 12
	.globl __restvmx_89
__restvmx_89:
	li 11, -624
	lvx128 89, 11, 12
	.globl __restvmx_90
__restvmx_90:
	li 11, -608
	lvx128 90, 11, 12
	.globl __restvmx_91
__restvmx_91:
	li 11, -592
	lvx128 91, 11, 12
	.globl __restvmx_92
__restvmx_92:
	li 11, -576
	lvx128 92, 11, 12
	.globl __restvmx_93
__restvmx_93:
	li 11, -560
	lvx128 93, 11, 12
	.globl __restvmx_94
__restvmx_94:
	li 11, -544
	lvx128 94, 11, 12
	.globl __restvmx_95
__restvmx_95:
	li 11, -528
	lvx128 95, 11, 12
	.globl __restvmx_96
__restvmx_96:
	li 11, -512
	lvx128 96, 11, 12
	.globl __restvmx_97
__restvmx_97:
	li 11, -496
	lvx128 97, 11, 12
	.globl __restvmx_98
__restvmx_98:
	li 11, -480
	lvx128 98, 11, 12
	.globl __restvmx_99
__restvmx_99:
	li 11, -464
	lvx128 99, 11, 12
	.globl __restvmx_100
__restvmx_100:
	li 11, -448
	lvx128 100, 11, 12
	.globl __restvmx_101
__restvmx_101:
	li 11, -432
	lvx128 101, 11, 12
	.globl __restvmx_102
__restvmx_102:
	li 11, -416
	lvx128 102, 11, 12
	.globl __restvmx_103
__restvmx_103:
	li 11, -400
	lvx128 103, 11, 12
	.globl __restvmx_104
__restvmx_104:
	li 11, -384
	lvx128 104, 11, 12
	.globl __restvmx_105
__restvmx_105:
	li 11, -368
	lvx128 105, 11, 12
	.globl __restvmx_106
__restvmx_106:
	li 11, -352
	lvx128 106, 11, 12
	.globl __restvmx_107
__restvmx_107:
	li 11, -336
	lvx128 107, 11, 12
	.globl __restvmx_108
__restvmx_108:
	li 11, -320
	lvx128 108, 11, 12
	.globl __restvmx_109
__restvmx_109:
	li 11, -304
	lvx128 109, 11, 12
	.globl __restvmx_110
__restvmx_110:
	li 11, -288
	lvx128 110, 11, 12
	.globl __restvmx_111
__restvmx_111:
	li 11, -272
	lvx128 111, 11, 12
	.globl __restvmx_112
__restvmx_112:
	li 11, -256
	lvx128 112, 11, 12
	.globl __restvmx_113
__restvmx_113:
	li 11, -240
	lvx128 113, 11, 12
	.globl __restvmx_114
__restvmx_114:
	li 11, -224
	lvx128 114, 11, 12
	.globl __restvmx_115
__restvmx_115:
	li 11, -208
	lvx128 115, 11, 12
	.globl __restvmx_116
__restvmx_116:
	li 11, -192
	lvx128 116, 11, 12
	.globl __restvmx_117
__restvmx_117:
	li 11, -176
	lvx128 117, 11, 12
	.globl __restvmx_118
__restvmx_118:
	li 11, -160
	lvx128 118, 11, 12
	.globl __restvmx_119
__restvmx_119:
	li 11, -144
	lvx128 119, 11, 12
	.globl __restvmx_120
__restvmx_120:
	li 11, -128
	lvx128 120, 11, 12
	.globl __restvmx_121
__restvmx_121:
	li 11, -112
	lvx128 121, 11, 12
	.globl __restvmx_122
__restvmx_122:
	li 11, -96
	lvx128 122, 11, 12
	.globl __restvmx_123
__restvmx_123:
	li 11, -80
	lvx128 123, 11, 12
	.globl __restvmx_124
__restvmx_124:
	li 11, -64
	lvx128 124, 11, 12
	.globl __restvmx_125
__restvmx_125:
	li 11, -48
	lvx128 125, 11, 12
	.globl __restvmx_126
__restvmx_126:
	li 11, -32
	lvx128 126, 11, 12
	.globl __restvmx_127
__restvmx_127:
	li 11, -16
	lvx128 127, 11, 12
	blr
