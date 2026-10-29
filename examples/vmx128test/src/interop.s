# msvc/interop.c compiled by cl.exe 16.00.11886 /O2 /GS- and converted by
# tools/coff2gas.py (msvc/build.sh). Every word is from interop.c; the
# helpers it calls are in msvc_runtime.s.
# Converted from cl.exe COFF by coff2gas.py; do not edit.

	.section ".text.sec5","ax",@progbits
	.p2align 3
__coffsec5:
__coffstatic_make_13:
	.long 0x3961FFF0
	.long 0xD021FFF0
	.long 0xD041FFF4
	.long 0xD061FFF8
	.long 0xD081FFFC
	.long 0x102058C3
	.long 0x4E800020

	.section ".text.sec6","ax",@progbits
	.p2align 3
__coffsec6:
__coffstatic_weigh_17:
	.long 0x3961FFF0
	.long 0xD021FFF0
	.long 0xD021FFF4
	.long 0xD021FFF8
	.long 0xD021FFFC
	.long 0x100058C3
	.long 0x1022082E
	.long 0x4E800020

	.section ".text.cl_sum16","ax",@progbits
	.p2align 3
__coffsec7:
	.globl cl_sum16
	.type cl_sum16, @function
cl_sum16:
	addis 11, 0, __vmx_00000000000000000000000000000000@ha
	addis 10, 0, __vmx_3f8000003f8000003f8000003f800000@ha
	addi 9, 11, __vmx_00000000000000000000000000000000@l
	addi 8, 10, __vmx_3f8000003f8000003f8000003f800000@l
	addis 7, 0, __vmx_40000000400000004000000040000000@ha
	addis 6, 0, __vmx_40400000404000004040000040400000@ha
	addi 5, 7, __vmx_40000000400000004000000040000000@l
	.long 0x100048C3
	addi 4, 6, __vmx_40400000404000004040000040400000@l
	.long 0x13E040C3
	addis 3, 0, __vmx_40800000408000004080000040800000@ha
	.long 0x126107EE
	addis 11, 0, __vmx_40a0000040a0000040a0000040a00000@ha
	addi 10, 3, __vmx_40800000408000004080000040800000@l
	.long 0x13C028C3
	addi 9, 11, __vmx_40a0000040a0000040a0000040a00000@l
	.long 0x13A020C3
	addis 8, 0, __vmx_40c0000040c0000040c0000040c00000@ha
	addis 7, 0, __vmx_40e0000040e0000040e0000040e00000@ha
	addi 6, 8, __vmx_40c0000040c0000040c0000040c00000@l
	.long 0x138050C3
	addi 5, 7, __vmx_40e0000040e0000040e0000040e00000@l
	.long 0x136048C3
	addis 4, 0, __vmx_41000000410000004100000041000000@ha
	addis 3, 0, __vmx_41100000411000004110000041100000@ha
	addi 11, 4, __vmx_41000000410000004100000041000000@l
	.long 0x134030C3
	addi 10, 3, __vmx_41100000411000004110000041100000@l
	.long 0x132028C3
	addis 9, 0, __vmx_41200000412000004120000041200000@ha
	.long 0x12629FAE
	addis 8, 0, __vmx_41300000413000004130000041300000@ha
	addi 7, 9, __vmx_41200000412000004120000041200000@l
	.long 0x130058C3
	addi 6, 8, __vmx_41300000413000004130000041300000@l
	.long 0x12E050C3
	addis 5, 0, __vmx_41400000414000004140000041400000@ha
	addis 4, 0, __vmx_41500000415000004150000041500000@ha
	addi 3, 5, __vmx_41400000414000004140000041400000@l
	.long 0x12C038C3
	addi 11, 4, __vmx_41500000415000004150000041500000@l
	.long 0x100030C3
	addis 10, 0, __vmx_41600000416000004160000041600000@ha
	.long 0x392100E0
	addi 8, 10, __vmx_41600000416000004160000041600000@l
	.long 0x102018C3
	addis 7, 0, __vmx_41700000417000004170000041700000@ha
	.long 0x13E058C3
	.long 0x38C100F0
	.long 0x10639F6E
	addi 5, 7, __vmx_41700000417000004170000041700000@l
	.long 0x124048C3
	addis 4, 0, __vmx_41800000418000004180000041800000@ha
	.long 0x12A040C3
	.long 0x38610100
	addi 11, 4, __vmx_41800000418000004180000041800000@l
	.long 0x122030C3
	.long 0x128028C3
	.long 0x13C018C3
	.long 0x104058C3
	.long 0x10841F2E
	.long 0x10A526EE
	.long 0x10C62EAE
	.long 0x10E7366E
	.long 0x11083E2E
	.long 0x112945EE
	.long 0x114A4DAE
	.long 0x100B502E
	.long 0x100C006E
	.long 0x100D07EE
	.long 0x1012056E
	.long 0x1011052E
	.long 0x103E00AE
	.long 0x4E800020
	.size cl_sum16, __coffsec7+296-cl_sum16

	.section ".rodata.__vmx@41800000418000004180000041800000","aG",@progbits,__vmx_41800000418000004180000041800000,comdat
	.p2align 4
__coffsec8:
	.globl __vmx_41800000418000004180000041800000
	.type __vmx_41800000418000004180000041800000, @object
__vmx_41800000418000004180000041800000:
	.long 0x41800000
	.long 0x41800000
	.long 0x41800000
	.long 0x41800000

	.section ".rodata.__vmx@41700000417000004170000041700000","aG",@progbits,__vmx_41700000417000004170000041700000,comdat
	.p2align 4
__coffsec9:
	.globl __vmx_41700000417000004170000041700000
	.type __vmx_41700000417000004170000041700000, @object
__vmx_41700000417000004170000041700000:
	.long 0x41700000
	.long 0x41700000
	.long 0x41700000
	.long 0x41700000

	.section ".rodata.__vmx@41600000416000004160000041600000","aG",@progbits,__vmx_41600000416000004160000041600000,comdat
	.p2align 4
__coffsec10:
	.globl __vmx_41600000416000004160000041600000
	.type __vmx_41600000416000004160000041600000, @object
__vmx_41600000416000004160000041600000:
	.long 0x41600000
	.long 0x41600000
	.long 0x41600000
	.long 0x41600000

	.section ".rodata.__vmx@41500000415000004150000041500000","aG",@progbits,__vmx_41500000415000004150000041500000,comdat
	.p2align 4
__coffsec11:
	.globl __vmx_41500000415000004150000041500000
	.type __vmx_41500000415000004150000041500000, @object
__vmx_41500000415000004150000041500000:
	.long 0x41500000
	.long 0x41500000
	.long 0x41500000
	.long 0x41500000

	.section ".rodata.__vmx@41400000414000004140000041400000","aG",@progbits,__vmx_41400000414000004140000041400000,comdat
	.p2align 4
__coffsec12:
	.globl __vmx_41400000414000004140000041400000
	.type __vmx_41400000414000004140000041400000, @object
__vmx_41400000414000004140000041400000:
	.long 0x41400000
	.long 0x41400000
	.long 0x41400000
	.long 0x41400000

	.section ".rodata.__vmx@41300000413000004130000041300000","aG",@progbits,__vmx_41300000413000004130000041300000,comdat
	.p2align 4
__coffsec13:
	.globl __vmx_41300000413000004130000041300000
	.type __vmx_41300000413000004130000041300000, @object
__vmx_41300000413000004130000041300000:
	.long 0x41300000
	.long 0x41300000
	.long 0x41300000
	.long 0x41300000

	.section ".rodata.__vmx@41200000412000004120000041200000","aG",@progbits,__vmx_41200000412000004120000041200000,comdat
	.p2align 4
__coffsec14:
	.globl __vmx_41200000412000004120000041200000
	.type __vmx_41200000412000004120000041200000, @object
__vmx_41200000412000004120000041200000:
	.long 0x41200000
	.long 0x41200000
	.long 0x41200000
	.long 0x41200000

	.section ".rodata.__vmx@41100000411000004110000041100000","aG",@progbits,__vmx_41100000411000004110000041100000,comdat
	.p2align 4
__coffsec15:
	.globl __vmx_41100000411000004110000041100000
	.type __vmx_41100000411000004110000041100000, @object
__vmx_41100000411000004110000041100000:
	.long 0x41100000
	.long 0x41100000
	.long 0x41100000
	.long 0x41100000

	.section ".rodata.__vmx@41000000410000004100000041000000","aG",@progbits,__vmx_41000000410000004100000041000000,comdat
	.p2align 4
__coffsec16:
	.globl __vmx_41000000410000004100000041000000
	.type __vmx_41000000410000004100000041000000, @object
__vmx_41000000410000004100000041000000:
	.long 0x41000000
	.long 0x41000000
	.long 0x41000000
	.long 0x41000000

	.section ".rodata.__vmx@40e0000040e0000040e0000040e00000","aG",@progbits,__vmx_40e0000040e0000040e0000040e00000,comdat
	.p2align 4
__coffsec17:
	.globl __vmx_40e0000040e0000040e0000040e00000
	.type __vmx_40e0000040e0000040e0000040e00000, @object
__vmx_40e0000040e0000040e0000040e00000:
	.long 0x40E00000
	.long 0x40E00000
	.long 0x40E00000
	.long 0x40E00000

	.section ".rodata.__vmx@40c0000040c0000040c0000040c00000","aG",@progbits,__vmx_40c0000040c0000040c0000040c00000,comdat
	.p2align 4
__coffsec18:
	.globl __vmx_40c0000040c0000040c0000040c00000
	.type __vmx_40c0000040c0000040c0000040c00000, @object
__vmx_40c0000040c0000040c0000040c00000:
	.long 0x40C00000
	.long 0x40C00000
	.long 0x40C00000
	.long 0x40C00000

	.section ".rodata.__vmx@40a0000040a0000040a0000040a00000","aG",@progbits,__vmx_40a0000040a0000040a0000040a00000,comdat
	.p2align 4
__coffsec19:
	.globl __vmx_40a0000040a0000040a0000040a00000
	.type __vmx_40a0000040a0000040a0000040a00000, @object
__vmx_40a0000040a0000040a0000040a00000:
	.long 0x40A00000
	.long 0x40A00000
	.long 0x40A00000
	.long 0x40A00000

	.section ".rodata.__vmx@40800000408000004080000040800000","aG",@progbits,__vmx_40800000408000004080000040800000,comdat
	.p2align 4
__coffsec20:
	.globl __vmx_40800000408000004080000040800000
	.type __vmx_40800000408000004080000040800000, @object
__vmx_40800000408000004080000040800000:
	.long 0x40800000
	.long 0x40800000
	.long 0x40800000
	.long 0x40800000

	.section ".rodata.__vmx@40400000404000004040000040400000","aG",@progbits,__vmx_40400000404000004040000040400000,comdat
	.p2align 4
__coffsec21:
	.globl __vmx_40400000404000004040000040400000
	.type __vmx_40400000404000004040000040400000, @object
__vmx_40400000404000004040000040400000:
	.long 0x40400000
	.long 0x40400000
	.long 0x40400000
	.long 0x40400000

	.section ".rodata.__vmx@40000000400000004000000040000000","aG",@progbits,__vmx_40000000400000004000000040000000,comdat
	.p2align 4
__coffsec22:
	.globl __vmx_40000000400000004000000040000000
	.type __vmx_40000000400000004000000040000000, @object
__vmx_40000000400000004000000040000000:
	.long 0x40000000
	.long 0x40000000
	.long 0x40000000
	.long 0x40000000

	.section ".rodata.__vmx@3f8000003f8000003f8000003f800000","aG",@progbits,__vmx_3f8000003f8000003f8000003f800000,comdat
	.p2align 4
__coffsec23:
	.globl __vmx_3f8000003f8000003f8000003f800000
	.type __vmx_3f8000003f8000003f8000003f800000, @object
__vmx_3f8000003f8000003f8000003f800000:
	.long 0x3F800000
	.long 0x3F800000
	.long 0x3F800000
	.long 0x3F800000

	.section ".rodata.__vmx@00000000000000000000000000000000","aG",@progbits,__vmx_00000000000000000000000000000000,comdat
	.p2align 4
__coffsec24:
	.globl __vmx_00000000000000000000000000000000
	.type __vmx_00000000000000000000000000000000, @object
__vmx_00000000000000000000000000000000:
	.long 0x00000000
	.long 0x00000000
	.long 0x00000000
	.long 0x00000000

	.section ".text.cl_after13","ax",@progbits
	.p2align 3
__coffsec25:
	.globl cl_after13
	.type cl_after13, @function
cl_after13:
	.long 0x7C6A07B4
	.long 0x7C8B07B4
	.long 0xF941FFF0
	.long 0xC801FFF0
	.long 0xF961FFF0
	.long 0xC9A1FFF0
	.long 0xFD60069C
	addis 9, 0, __vmx_3f8000003f8000003f8000003f800000@ha
	.long 0xFD806E9C
	addis 8, 0, __real_00000000@ha
	addi 7, 9, __vmx_3f8000003f8000003f8000003f800000@l
	.long 0xFD205818
	.long 0x38C1FFF0
	.long 0xD121FFF0
	addis 5, 0, __vmx_40000000400000004000000040000000@ha
	addis 4, 0, __vmx_40400000404000004040000040400000@ha
	lfs 0, __real_00000000@l(8)
	addi 3, 5, __vmx_40000000400000004000000040000000@l
	.long 0x100038C3
	addi 11, 4, __vmx_40400000404000004040000040400000@l
	.long 0xD001FFF8
	addis 10, 0, __vmx_40800000408000004080000040800000@ha
	.long 0xD001FFFC
	addis 9, 0, __vmx_40a0000040a0000040a0000040a00000@ha
	addi 8, 10, __vmx_40800000408000004080000040800000@l
	.long 0xFD406018
	.long 0xD141FFF4
	.long 0x134030C3
	.long 0x1021D02E
	.long 0x13E018C3
	addi 7, 9, __vmx_40a0000040a0000040a0000040a00000@l
	.long 0x13020FEE
	.long 0x13C058C3
	.long 0x13A040C3
	addis 6, 0, __vmx_40c0000040c0000040c0000040c00000@ha
	addis 5, 0, __vmx_40e0000040e0000040e0000040e00000@ha
	addi 4, 6, __vmx_40c0000040c0000040c0000040c00000@l
	.long 0x138038C3
	addi 3, 5, __vmx_40e0000040e0000040e0000040e00000@l
	addis 11, 0, __vmx_41000000410000004100000041000000@ha
	addis 10, 0, __vmx_41100000411000004110000041100000@ha
	addi 9, 11, __vmx_41000000410000004100000041000000@l
	.long 0x136020C3
	addi 8, 10, __vmx_41100000411000004110000041100000@l
	.long 0x100018C3
	addis 7, 0, __vmx_41200000412000004120000041200000@ha
	addis 6, 0, __vmx_41300000413000004130000041300000@ha
	addi 5, 7, __vmx_41200000412000004120000041200000@l
	.long 0x1303C7AE
	.long 0x104048C3
	.long 0x102040C3
	addi 4, 6, __vmx_41300000413000004130000041300000@l
	addis 3, 0, __vmx_41400000414000004140000041400000@ha
	addis 11, 0, __vmx_41500000415000004150000041500000@ha
	.long 0x13E028C3
	addi 10, 3, __vmx_41400000414000004140000041400000@l
	addi 9, 11, __vmx_41500000415000004150000041500000@l
	.long 0x134020C3
	addis 8, 0, __vmx_41600000416000004160000041600000@ha
	.long 0x38E100F0
	addi 6, 8, __vmx_41600000416000004160000041600000@l
	.long 0x132050C3
	.long 0x106048C3
	.long 0x1084C76E
	.long 0x12E038C3
	.long 0x13C030C3
	.long 0x10A5272E
	.long 0x10C62EEE
	.long 0x1007302E
	.long 0x100800AE
	.long 0x1009006E
	.long 0x100A07EE
	.long 0x100B06AE
	.long 0x100C066E
	.long 0x100D00EE
	.long 0x103707AE
	.long 0x4E800020
	.size cl_after13, __coffsec25+308-cl_after13

	.section ".rodata.__real@00000000","aG",@progbits,__real_00000000,comdat
	.p2align 2
__coffsec26:
	.globl __real_00000000
	.type __real_00000000, @object
__real_00000000:
	.long 0x00000000

	.section ".text.cl_mix","ax",@progbits
	.p2align 3
__coffsec27:
	.globl cl_mix
	.type cl_mix, @function
cl_mix:
	.long 0x17E21814
	.long 0x7CA907B4
	.long 0x7C6A07B4
	.long 0xFC001018
	.long 0x3901FFF0
	.long 0xF921FFF0
	.long 0xC9A1FFF0
	.long 0x7CEB07B4
	.long 0xF941FFF0
	.long 0xC981FFF0
	.long 0xF961FFF0
	.long 0x38E1FFF0
	.long 0xC961FFF0
	.long 0xFD405E9C
	.long 0x17C1F815
	.long 0xFD206E9C
	.long 0xFCE0669C
	.long 0xFCA05018
	.long 0xFDA04818
	.long 0xFD803818
	.long 0x13C041C7
	.long 0xC101FFFC
	.long 0xC0C1FFF0
	.long 0xC081FFF8
	.long 0xC041FFF4
	.long 0xEC22082A
	.long 0xED2D282A
	.long 0xED60402A
	.long 0xD161FFFC
	.long 0xED41182A
	.long 0xD141FFF4
	.long 0xED0C302A
	.long 0xD101FFF0
	.long 0xECE9202A
	.long 0xD0E1FFF8
	.long 0x102038C3
	.long 0x4E800020
	.size cl_mix, __coffsec27+148-cl_mix

	.section ".text.cl_make","ax",@progbits
	.p2align 3
__coffsec28:
	.globl cl_make
	.type cl_make, @function
cl_make:
	.long 0x3961FFF0
	.long 0xD021FFF0
	.long 0xD041FFF4
	.long 0xD061FFF8
	.long 0xD081FFFC
	.long 0x102058C3
	.long 0x4E800020
	.size cl_make, __coffsec28+28-cl_make

	.section ".text.cl_store","ax",@progbits
	.p2align 3
__coffsec29:
	.globl cl_store
	.type cl_store, @function
cl_store:
	.long 0x102019C3
	.long 0x4E800020
	.size cl_store, __coffsec29+8-cl_store

	.section ".text.cl_s1_in","ax",@progbits
	.p2align 3
__coffsec30:
	.globl cl_s1_in
	.type cl_s1_in, @function
cl_s1_in:
	.long 0x546AC00E
	.long 0x548B063E
	.long 0x7D6A5A14
	.long 0x7C6B2A14
	.long 0x4E800020
	.size cl_s1_in, __coffsec30+20-cl_s1_in

	.section ".text.cl_s2_in","ax",@progbits
	.p2align 3
__coffsec31:
	.globl cl_s2_in
	.type cl_s2_in, @function
cl_s2_in:
	.long 0xB081001E
	.long 0x5469801E
	.long 0x8941001E
	.long 0x8961001F
	.long 0x7D295A14
	.long 0x552B402E
	.long 0x7D6B5214
	.long 0x7C6B2A14
	.long 0x4E800020
	.size cl_s2_in, __coffsec31+36-cl_s2_in

	.section ".text.cl_s3_in","ax",@progbits
	.p2align 3
__coffsec32:
	.globl cl_s3_in
	.type cl_s3_in, @function
cl_s3_in:
	.long 0xF8810018
	.long 0x546A402E
	.long 0x89210019
	.long 0x89010018
	.long 0x8961001A
	.long 0x7CEA5A14
	.long 0x54EB402E
	.long 0x7CCB4A14
	.long 0x54CB402E
	.long 0x7D6B4214
	.long 0x7C6B2A14
	.long 0x4E800020
	.size cl_s3_in, __coffsec32+48-cl_s3_in

	.section ".text.cl_s4_in","ax",@progbits
	.p2align 3
__coffsec33:
	.globl cl_s4_in
	.type cl_s4_in, @function
cl_s4_in:
	.long 0x9081001C
	.long 0x8921001E
	.long 0x8901001D
	.long 0x88E1001C
	.long 0x8961001F
	.long 0x556A403E
	.long 0x7CCA4A14
	.long 0x54CB402E
	.long 0x7C8B4214
	.long 0x548B2036
	.long 0x7C6B1A14
	.long 0x546B2036
	.long 0x7D6B3A14
	.long 0x7C6B2A14
	.long 0x4E800020
	.size cl_s4_in, __coffsec33+60-cl_s4_in

	.section ".text.cl_s5_in","ax",@progbits
	.p2align 3
__coffsec34:
	.globl cl_s5_in
	.type cl_s5_in, @function
cl_s5_in:
	.long 0xF8810018
	.long 0x8961001C
	.long 0x8921001A
	.long 0x88E10019
	.long 0x88C10018
	.long 0x8941001B
	.long 0x5548403E
	.long 0x7D6A5B78
	.long 0x556B083E
	.long 0x7C884A14
	.long 0x7D2A5A14
	.long 0x548B402E
	.long 0x7D6B3A14
	.long 0x556B2036
	.long 0x7D0B1A14
	.long 0x550B2036
	.long 0x7D6B4A14
	.long 0x7D6B3214
	.long 0x7C6B2A14
	.long 0x4E800020
	.size cl_s5_in, __coffsec34+80-cl_s5_in

	.section ".text.cl_s8_in","ax",@progbits
	.p2align 3
__coffsec35:
	.globl cl_s8_in
	.type cl_s8_in, @function
cl_s8_in:
	.long 0xFBC1FFF0
	.long 0xFBE1FFF8
__coffstatic__M2341_106:
	.long 0xF8810018
	.long 0x8921001A
	.long 0x88E10019
	.long 0x88C1001F
	.long 0x8961001B
	.long 0x5568403E
	.long 0x8961001E
	.long 0x7C884A14
	.long 0x8BE10018
	.long 0x7CC83378
	.long 0x5489402E
	.long 0x8941001D
	.long 0x1C88000B
	.long 0x7CE93A14
	.long 0x7D495378
	.long 0x54E72036
	.long 0x554A103E
	.long 0x7CC71A14
	.long 0x5563183E
	.long 0x7D7E5B78
	.long 0x8961001C
	.long 0x54C62036
	.long 0x7CE95214
	.long 0x7D262214
	.long 0x7D1E1850
	.long 0x7D6A5B78
	.long 0x7D094214
	.long 0x5569083E
	.long 0x7D683A14
	.long 0x7D2A4A14
	.long 0x7D6B4A14
	.long 0x7D6BFA14
	.long 0x7C6B2A14
	.long 0xEBC1FFF0
	.long 0xEBE1FFF8
	.long 0x4E800020
__coffstatic__M2342_105:
	.size cl_s8_in, __coffsec35+152-cl_s8_in

	.section ".text.cl_s1_out","ax",@progbits
	.p2align 3
__coffsec37:
	.globl cl_s1_out
	.type cl_s1_out, @function
cl_s1_out:
	.long 0x5463063E
	.long 0x4E800020
	.size cl_s1_out, __coffsec37+8-cl_s1_out

	.section ".text.cl_s2_out","ax",@progbits
	.p2align 3
__coffsec38:
	.globl cl_s2_out
	.type cl_s2_out, @function
cl_s2_out:
	.long 0x39630001
	.long 0x9861FFF0
	.long 0x9961FFF1
	.long 0xA061FFF0
	.long 0x4E800020
	.size cl_s2_out, __coffsec38+20-cl_s2_out

	.section ".text.cl_s3_out","ax",@progbits
	.p2align 3
__coffsec39:
	.globl cl_s3_out
	.type cl_s3_out, @function
cl_s3_out:
	.long 0x39640001
	.long 0x98830000
	.long 0x39440002
	.long 0x99630001
	.long 0x99430002
	.long 0x4E800020
	.size cl_s3_out, __coffsec39+24-cl_s3_out

	.section ".text.cl_s4_out","ax",@progbits
	.p2align 3
__coffsec40:
	.globl cl_s4_out
	.type cl_s4_out, @function
cl_s4_out:
	.long 0x39630001
	.long 0x9861FFF0
	.long 0x39430002
	.long 0x39230003
	.long 0x9961FFF1
	.long 0x9941FFF2
	.long 0x9921FFF3
	.long 0x8061FFF0
	.long 0x4E800020
	.size cl_s4_out, __coffsec40+36-cl_s4_out

	.section ".text.cl_s5_out","ax",@progbits
	.p2align 3
__coffsec41:
	.globl cl_s5_out
	.type cl_s5_out, @function
cl_s5_out:
	.long 0x39640001
	.long 0x98830000
	.long 0x39440002
	.long 0x39240003
	.long 0x99630001
	.long 0x39040004
	.long 0x99430002
	.long 0x99230003
	.long 0x99030004
	.long 0x4E800020
	.size cl_s5_out, __coffsec41+40-cl_s5_out

	.section ".text.cl_s8_out","ax",@progbits
	.p2align 3
__coffsec42:
	.globl cl_s8_out
	.type cl_s8_out, @function
cl_s8_out:
	.long 0x39630001
	.long 0x9861FFF0
	.long 0x39430002
	.long 0x39230003
	.long 0x9961FFF1
	.long 0x39030004
	.long 0x9941FFF2
	.long 0x38E30005
	.long 0x9921FFF3
	.long 0x38C30006
	.long 0x9901FFF4
	.long 0x38A30007
	.long 0x98E1FFF5
	.long 0x98C1FFF6
	.long 0x98A1FFF7
	.long 0xE861FFF0
	.long 0x4E800020
	.size cl_s8_out, __coffsec42+68-cl_s8_out

	.section ".text.cl_char_value","ax",@progbits
	.p2align 3
__coffsec43:
	.globl cl_char_value
	.type cl_char_value, @function
cl_char_value:
	.long 0x7C630774
	.long 0x4E800020
	.size cl_char_value, __coffsec43+8-cl_char_value

	.section ".text.cl_schar_value","ax",@progbits
	.p2align 3
__coffsec44:
	.globl cl_schar_value
	.type cl_schar_value, @function
cl_schar_value:
	.long 0x7C630774
	.long 0x4E800020
	.size cl_schar_value, __coffsec44+8-cl_schar_value

	.section ".text.cl_uchar_value","ax",@progbits
	.p2align 3
__coffsec45:
	.globl cl_uchar_value
	.type cl_uchar_value, @function
cl_uchar_value:
	.long 0x5463063E
	.long 0x4E800020
	.size cl_uchar_value, __coffsec45+8-cl_uchar_value

	.section ".text.cl_short_value","ax",@progbits
	.p2align 3
__coffsec46:
	.globl cl_short_value
	.type cl_short_value, @function
cl_short_value:
	.long 0x7C630734
	.long 0x4E800020
	.size cl_short_value, __coffsec46+8-cl_short_value

	.section ".text.cl_char_is_signed","ax",@progbits
	.p2align 3
__coffsec47:
	.globl cl_char_is_signed
	.type cl_char_is_signed, @function
cl_char_is_signed:
	.long 0x38600001
	.long 0x4E800020
	.size cl_char_is_signed, __coffsec47+8-cl_char_is_signed

	.section ".text.cl_char_from","ax",@progbits
	.p2align 3
__coffsec48:
	.globl cl_char_from
	.type cl_char_from, @function
cl_char_from:
	.long 0x7C630774
	.long 0x4E800020
	.size cl_char_from, __coffsec48+8-cl_char_from

	.section ".text.cl_uchar_from","ax",@progbits
	.p2align 3
__coffsec49:
	.globl cl_uchar_from
	.type cl_uchar_from, @function
cl_uchar_from:
	.long 0x5463063E
	.long 0x4E800020
	.size cl_uchar_from, __coffsec49+8-cl_uchar_from

	.section ".text.cl_short_from","ax",@progbits
	.p2align 3
__coffsec50:
	.globl cl_short_from
	.type cl_short_from, @function
cl_short_from:
	.long 0x7C630734
	.long 0x4E800020
	.size cl_short_from, __coffsec50+8-cl_short_from

	.section ".text.cl_call16","ax",@progbits
	.p2align 3
__coffsec51:
	.globl cl_call16
	.type cl_call16, @function
cl_call16:
	.long 0x7D8802A6
	.long 0x9181FFF8
	.long 0x9421FEE0
__coffstatic__M2362_156:
	.long 0x396000F0
	.long 0x7C6903A6
	.long 0x394000E0
	.long 0x102020C3
	.long 0x392000D0
	.long 0x390000C0
	.long 0x38E000B0
	.long 0x38C000A0
	.long 0x13E458C7
	.long 0x38A00090
	.long 0x13C450C7
	.long 0x13A448C7
	.long 0x38600080
	.long 0x11A440C3
	.long 0x39600070
	.long 0x118438C3
	.long 0x39400060
	.long 0x116430C3
	.long 0x39200050
	.long 0x114428C3
	.long 0x39000040
	.long 0x38E00030
	.long 0x39800100
	.long 0x13E161C7
	.long 0x38C00020
	.long 0x398000F0
	.long 0x13C161C7
	.long 0x38A00010
	.long 0x112418C3
	.long 0x110458C3
	.long 0x10E450C3
	.long 0x10C448C3
	.long 0x10A440C3
	.long 0x108438C3
	.long 0x106430C3
	.long 0x104428C3
	.long 0x398000E0
	.long 0x13A161C7
	.long 0x4E800421
	.long 0x38210120
	.long 0x8181FFF8
	.long 0x7D8803A6
	.long 0x4E800020
__coffstatic__M2363_155:
	.size cl_call16, __coffsec51+184-cl_call16

	.section ".text.cl_callmix","ax",@progbits
	.p2align 3
__coffsec53:
	.globl cl_callmix
	.type cl_callmix, @function
cl_callmix:
	.long 0x7D8802A6
	.long 0x9181FFF8
	.long 0xFBE1FFF0
	.long 0x9421FF70
__coffstatic__M2373_173:
	.long 0x7C681B78
	.long 0x102020C3
	addis 11, 0, __real_bf000000@ha
	addis 10, 0, __real_4002000000000000@ha
	addis 9, 0, __real_3fc00000@ha
	.long 0x38C00020
	.long 0x3BE00010
	.long 0x7D0903A6
	.long 0x38E0000B
	lfs 3, __real_bf000000@l(11)
	.long 0x38A0FFFD
	lfd 2, __real_4002000000000000@l(10)
	.long 0x38600007
	lfs 1, __real_3fc00000@l(9)
	.long 0x106430C3
	.long 0x1044F8C3
	.long 0x4E800421
	.long 0x38210090
	.long 0x8181FFF8
	.long 0x7D8803A6
	.long 0xEBE1FFF0
	.long 0x4E800020
__coffstatic__M2374_163:
	.size cl_callmix, __coffsec53+104-cl_callmix

	.section ".rodata.__real@3fc00000","aG",@progbits,__real_3fc00000,comdat
	.p2align 2
__coffsec54:
	.globl __real_3fc00000
	.type __real_3fc00000, @object
__real_3fc00000:
	.long 0x3FC00000

	.section ".rodata.__real@4002000000000000","aG",@progbits,__real_4002000000000000,comdat
	.p2align 3
__coffsec55:
	.globl __real_4002000000000000
	.type __real_4002000000000000, @object
__real_4002000000000000:
	.long 0x40020000
	.long 0x00000000

	.section ".rodata.__real@bf000000","aG",@progbits,__real_bf000000,comdat
	.p2align 2
__coffsec56:
	.globl __real_bf000000
	.type __real_bf000000, @object
__real_bf000000:
	.long 0xBF000000

	.section ".text.cl_calls3","ax",@progbits
	.p2align 3
__coffsec58:
	.globl cl_calls3
	.type cl_calls3, @function
cl_calls3:
	.long 0x7D8802A6
	.long 0x9181FFF8
	.long 0x9421FFA0
__coffstatic__M2378_181:
	.long 0x39640001
	.long 0x98810050
	.long 0x39040002
	.long 0x7C6903A6
	.long 0x99610051
	.long 0x38A00009
	.long 0xA0E10050
	.long 0x78E447E4
	.long 0x5506063E
	.long 0x38600005
	.long 0x7CCB2378
	.long 0x796445C6
	.long 0x4E800421
	.long 0x38210060
	.long 0x8181FFF8
	.long 0x7D8803A6
	.long 0x4E800020
__coffstatic__M2379_180:
	.size cl_calls3, __coffsec58+80-cl_calls3

	.section ".text.cl_calls3out","ax",@progbits
	.p2align 3
__coffsec60:
	.globl cl_calls3out
	.type cl_calls3out, @function
cl_calls3out:
	.long 0x7D8802A6
	.long 0x9181FFF8
	.long 0x9421FFA0
__coffstatic__M2384_189:
	.long 0x7C6B1B78
	.long 0x38610054
	.long 0x7D6903A6
	.long 0x4E800421
	.long 0x89230000
	.long 0x39410050
	.long 0x89030001
	.long 0x88E30002
	.long 0x992A0000
	.long 0x990A0001
	.long 0x98EA0002
	.long 0x89610051
	.long 0x89210050
	.long 0x88C10052
	.long 0x54CA403E
	.long 0x7C6A5A14
	.long 0x546B402E
	.long 0x7C6B4A14
	.long 0x38210060
	.long 0x8181FFF8
	.long 0x7D8803A6
	.long 0x4E800020
__coffstatic__M2385_188:
	.size cl_calls3out, __coffsec60+100-cl_calls3out

	.section ".text.cl_calls5out","ax",@progbits
	.p2align 3
__coffsec62:
	.globl cl_calls5out
	.type cl_calls5out, @function
cl_calls5out:
	.long 0x7D8802A6
	.long 0x9181FFF8
	.long 0x9421FF90
__coffstatic__M2391_197:
	.long 0x7C6B1B78
	.long 0x38610058
	.long 0x7D6903A6
	.long 0x4E800421
	.long 0x39200005
	.long 0x3941004F
	.long 0x3963FFFF
	.long 0x7D2903A6
	.long 0x8D2B0001
	.long 0x9D2A0001
	.long 0x4200FFF8
	.long 0x89610053
	.long 0x89210052
	.long 0x5568403E
	.long 0x89610054
	.long 0x88E10051
	.long 0x7CA84A14
	.long 0x88C10050
	.long 0x7D6A5B78
	.long 0x54A8402E
	.long 0x556B083E
	.long 0x7C883A14
	.long 0x7D2A5A14
	.long 0x548B402E
	.long 0x7D6B4A14
	.long 0x7C6B3214
	.long 0x38210070
	.long 0x8181FFF8
	.long 0x7D8803A6
	.long 0x4E800020
__coffstatic__M2392_196:
	.size cl_calls5out, __coffsec62+132-cl_calls5out

	.section ".text.cl_calls8","ax",@progbits
	.p2align 3
__coffsec64:
	.globl cl_calls8
	.type cl_calls8, @function
cl_calls8:
	.long 0x7D8802A6
	.long 0x9181FFF8
	.long 0x9421FFA0
__coffstatic__M2396_205:
	.long 0x39640001
	.long 0x7C6903A6
	.long 0x39440002
	.long 0x98810050
	.long 0x39240003
	.long 0x39040004
	.long 0x7D635B78
	.long 0x38E40005
	.long 0x7D4B5378
	.long 0x98610051
	.long 0x38C40006
	.long 0x7D2A4B78
	.long 0x99610052
	.long 0x38A40007
	.long 0x7D094378
	.long 0x99410053
	.long 0x7CE83B78
	.long 0x7CC73378
	.long 0x99210054
	.long 0x7CA62B78
	.long 0x99010055
	.long 0x38A00009
	.long 0x98E10056
	.long 0x38600005
	.long 0x98C10057
	.long 0xE8810050
	.long 0x4E800421
	.long 0x38210060
	.long 0x8181FFF8
	.long 0x7D8803A6
	.long 0x4E800020
__coffstatic__M2397_204:
	.size cl_calls8, __coffsec64+136-cl_calls8

	.section ".text.cl_callchar","ax",@progbits
	.p2align 3
__coffsec66:
	.globl cl_callchar
	.type cl_callchar, @function
cl_callchar:
	.long 0x548B063E
	.long 0x7C6903A6
	.long 0x1D44012C
	.long 0x7D655B78
	.long 0x7D645B78
	.long 0x7D460734
	.long 0x7D635B78
	.long 0x4E800420
	.size cl_callchar, __coffsec66+32-cl_callchar

	.section ".text.cl_keep_across","ax",@progbits
	.p2align 3
__coffsec67:
	.globl cl_keep_across
	.type cl_keep_across, @function
cl_keep_across:
	.long 0x7D8802A6
	bl __savegprlr_17
	.long 0x3981FF80
	bl __savevmx_64
	.long 0x9421FAC0
__coffstatic__M3218_378:
	.long 0x39600010
	.long 0x13E020CF
	.long 0x39400020
	.long 0x39200030
	.long 0x39000040
	.long 0x38E00050
	.long 0x13C458CF
	.long 0x38C00060
	.long 0x13A450CF
	.long 0x38A00070
	.long 0x138448CF
	.long 0x39600080
	.long 0x136440CF
	.long 0x39400090
	.long 0x134438CF
	.long 0x392000A0
	.long 0x390000B0
	.long 0x132430CF
	.long 0x38E000C0
	.long 0x130428CF
	.long 0x12E458CF
	.long 0x38C000D0
	.long 0x12C450CF
	.long 0x38A000E0
	.long 0x12A448CF
	.long 0x396000F0
	.long 0x128440CF
	.long 0x39400100
	.long 0x126438CF
	.long 0x39200110
	.long 0x39000120
	.long 0x124430CF
	.long 0x38E00130
	.long 0x122428CF
	.long 0x120458CF
	.long 0x38C00140
	.long 0x11E450CF
	.long 0x38A00150
	.long 0x11C448CF
	.long 0x39600160
	.long 0x11A440CF
	.long 0x39400170
	.long 0x118438CF
	.long 0x39200180
	.long 0x39000190
	.long 0x116430CF
	.long 0x38E001A0
	.long 0x114428CF
	.long 0x112458CF
	.long 0x38C001B0
	.long 0x110450CF
	.long 0x38A001C0
	.long 0x10E448CF
	.long 0x396001D0
	.long 0x10C440CF
	.long 0x394001E0
	.long 0x10A438CF
	.long 0x392001F0
	.long 0x39000200
	.long 0x108430CF
	.long 0x38E00210
	.long 0x106428CF
	.long 0x104458CF
	.long 0x38C00220
	.long 0x102450CF
	.long 0x38A00230
	.long 0x100448CF
	.long 0x39600240
	.long 0x13E440CB
	.long 0x39400250
	.long 0x13C438CB
	.long 0x39200260
	.long 0x39000270
	.long 0x13A430CB
	.long 0x38E00280
	.long 0x138428CB
	.long 0x136458CB
	.long 0x134450CB
	.long 0x132448CB
	.long 0x130440CB
	.long 0x12E438CB
	addis 6, 0, __vmx_00000000000000000000000000000000@ha
	.long 0x7C6903A6
	.long 0x38A00400
	addi 3, 6, __vmx_00000000000000000000000000000000@l
	.long 0x39600410
	.long 0x39400420
	.long 0x39200430
	.long 0x13E428C7
	.long 0x39000440
	.long 0x13C018C7
	.long 0x38C10050
	.long 0x13A458C7
	.long 0x38A10060
	.long 0x138450C7
	.long 0x38610070
	.long 0x136448C7
	.long 0x39610080
	.long 0x39410090
	.long 0x134440C7
	.long 0x392100A0
	.long 0x13C031C7
	.long 0x38E00450
	.long 0x13E029C7
	.long 0x13A019C7
	.long 0x390100B0
	.long 0x138059C7
	.long 0x38C002A0
	.long 0x136051C7
	.long 0x38A002B0
	.long 0x134049C7
	.long 0x386002C0
	.long 0x132438C7
	.long 0x38E00290
	.long 0x396002D0
	.long 0x132041C7
	.long 0x394002E0
	.long 0x12A430CB
	.long 0x392002F0
	.long 0x128428CB
	.long 0x126418CB
	.long 0x39000300
	.long 0x12C438CB
	.long 0x38E00310
	.long 0x124458CB
	.long 0x38C00320
	.long 0x122450CB
	.long 0x38A00330
	.long 0x120448CB
	.long 0x38600340
	.long 0x39600350
	.long 0x11E440CB
	.long 0x39400360
	.long 0x11C438CB
	.long 0x39200370
	.long 0x11A430CB
	.long 0x118428CB
	.long 0x39000380
	.long 0x116418CB
	.long 0x38E00390
	.long 0x114458CB
	.long 0x38C003A0
	.long 0x112450CB
	.long 0x38A003B0
	.long 0x110448CB
	.long 0x386003C0
	.long 0x396003D0
	.long 0x10E440CB
	.long 0x394003E0
	.long 0x10C438CB
	.long 0x392003F0
	.long 0x10A430CB
	.long 0x108428CB
	.long 0x106418CB
	.long 0x104458CB
	.long 0x102450CB
	.long 0x100448CB
	.long 0x4E800421
	addis 8, 0, __vmx_3f8000003f8000003f8000003f800000@ha
	addis 7, 0, __vmx_40000000400000004000000040000000@ha
	addis 6, 0, __vmx_40400000404000004040000040400000@ha
	addis 5, 0, __vmx_40800000408000004080000040800000@ha
	addis 3, 0, __vmx_40c0000040c0000040c0000040c00000@ha
	addi 8, 8, __vmx_3f8000003f8000003f8000003f800000@l
	addi 3, 3, __vmx_40c0000040c0000040c0000040c00000@l
	addi 7, 7, __vmx_40000000400000004000000040000000@l
	addis 4, 0, __vmx_40a0000040a0000040a0000040a00000@ha
	addis 11, 0, __vmx_40e0000040e0000040e0000040e00000@ha
	addis 10, 0, __vmx_41000000410000004100000041000000@ha
	.long 0x100040C3
	addis 9, 0, __vmx_41100000411000004110000041100000@ha
	.long 0x112018C3
	addi 6, 6, __vmx_40400000404000004040000040400000@l
	.long 0x11A038C3
	addi 5, 5, __vmx_40800000408000004080000040800000@l
	addi 4, 4, __vmx_40a0000040a0000040a0000040a00000@l
	addi 11, 11, __vmx_40e0000040e0000040e0000040e00000@l
	addis 31, 0, __vmx_41200000412000004120000041200000@ha
	addis 26, 0, __vmx_41700000417000004170000041700000@ha
	.long 0x118030C3
	addi 10, 10, __vmx_41000000410000004100000041000000@l
	.long 0x116028C3
	addis 30, 0, __vmx_41300000413000004130000041300000@ha
	.long 0x114020C3
	addi 9, 9, __vmx_41100000411000004110000041100000@l
	.long 0x110058C3
	addi 8, 31, __vmx_41200000412000004120000041200000@l
	addi 3, 26, __vmx_41700000417000004170000041700000@l
	addi 7, 30, __vmx_41300000413000004130000041300000@l
	.long 0x10E050C3
	addis 29, 0, __vmx_41400000414000004140000041400000@ha
	addis 28, 0, __vmx_41500000415000004150000041500000@ha
	.long 0x10C048C3
	addis 27, 0, __vmx_41600000416000004160000041600000@ha
	.long 0x10A040C3
	addis 25, 0, __vmx_41800000418000004180000041800000@ha
	.long 0x13E018C3
	addis 24, 0, __vmx_41880000418800004188000041880000@ha
	.long 0x108038C3
	addis 23, 0, __vmx_41900000419000004190000041900000@ha
	addi 6, 29, __vmx_41400000414000004140000041400000@l
	addi 5, 28, __vmx_41500000415000004150000041500000@l
	addi 4, 27, __vmx_41600000416000004160000041600000@l
	addi 11, 25, __vmx_41800000418000004180000041800000@l
	addis 22, 0, __vmx_41980000419800004198000041980000@ha
	addis 17, 0, __vmx_41c0000041c0000041c0000041c00000@ha
	.long 0x106030C3
	addi 10, 24, __vmx_41880000418800004188000041880000@l
	.long 0x104028C3
	addis 21, 0, __vmx_41a0000041a0000041a0000041a00000@ha
	.long 0x102020C3
	addi 9, 23, __vmx_41900000419000004190000041900000@l
	.long 0x13C058C3
	addi 8, 22, __vmx_41980000419800004198000041980000@l
	addi 3, 17, __vmx_41c0000041c0000041c0000041c00000@l
	addi 7, 21, __vmx_41a0000041a0000041a0000041a00000@l
	.long 0x13A050C3
	addis 20, 0, __vmx_41a8000041a8000041a8000041a80000@ha
	addis 19, 0, __vmx_41b0000041b0000041b0000041b00000@ha
	.long 0x138048C3
	addis 18, 0, __vmx_41b8000041b8000041b8000041b80000@ha
	.long 0x136040C3
	addi 6, 20, __vmx_41a8000041a8000041a8000041a80000@l
	.long 0x12C018C3
	addi 5, 19, __vmx_41b0000041b0000041b0000041b00000@l
	.long 0x134038C3
	addi 4, 18, __vmx_41b8000041b8000041b8000041b80000@l
	addis 11, 0, __vmx_41c8000041c8000041c8000041c80000@ha
	addis 10, 0, __vmx_41d0000041d0000041d0000041d00000@ha
	addis 9, 0, __vmx_41d8000041d8000041d8000041d80000@ha
	.long 0x132030C3
	addis 8, 0, __vmx_41e0000041e0000041e0000041e00000@ha
	.long 0x130028C3
	addi 3, 11, __vmx_41c8000041c8000041c8000041c80000@l
	.long 0x12E020C3
	addis 7, 0, __vmx_41e8000041e8000041e8000041e80000@ha
	addi 11, 10, __vmx_41d0000041d0000041d0000041d00000@l
	addi 10, 9, __vmx_41d8000041d8000041d8000041d80000@l
	addi 9, 8, __vmx_41e0000041e0000041e0000041e00000@l
	addis 6, 0, __vmx_41f0000041f0000041f0000041f00000@ha
	addis 5, 0, __vmx_41f8000041f8000041f8000041f80000@ha
	addis 4, 0, __vmx_42000000420000004200000042000000@ha
	addi 8, 7, __vmx_41e8000041e8000041e8000041e80000@l
	.long 0x38E10050
	.long 0x12A018C3
	.long 0x128058C3
	addi 6, 6, __vmx_41f0000041f0000041f0000041f00000@l
	.long 0x126050C3
	addi 5, 5, __vmx_41f8000041f8000041f8000041f80000@l
	.long 0x124048C3
	addis 3, 0, __vmx_42040000420400004204000042040000@ha
	addis 11, 0, __vmx_42080000420800004208000042080000@ha
	.long 0x122040C3
	addis 10, 0, __vmx_420c0000420c0000420c0000420c0000@ha
	.long 0x10A038C7
	addis 9, 0, __vmx_42100000421000004210000042100000@ha
	.long 0x120030C3
	addi 4, 4, __vmx_42000000420000004200000042000000@l
	.long 0x11E028C3
	addi 3, 3, __vmx_42040000420400004204000042040000@l
	.long 0x14BF04F4
	addi 11, 11, __vmx_42080000420800004208000042080000@l
	addi 10, 10, __vmx_420c0000420c0000420c0000420c0000@l
	addi 9, 9, __vmx_42100000421000004210000042100000@l
	addis 31, 0, __vmx_421c0000421c0000421c0000421c0000@ha
	.long 0x11C020C3
	addis 8, 0, __vmx_42140000421400004214000042140000@ha
	.long 0x13E018C7
	addis 7, 0, __vmx_42180000421800004218000042180000@ha
	.long 0x100058C3
	addis 30, 0, __vmx_42200000422000004220000042200000@ha
	.long 0x13C050C7
	addi 8, 8, __vmx_42140000421400004214000042140000@l
	.long 0x13A048C7
	addi 7, 7, __vmx_42180000421800004218000042180000@l
	addi 6, 31, __vmx_421c0000421c0000421c0000421c0000@l
	addi 5, 30, __vmx_42200000422000004220000042200000@l
	addis 29, 0, __vmx_42240000422400004224000042240000@ha
	.long 0x14BE6CF4
	addis 28, 0, __vmx_42280000422800004228000042280000@ha
	.long 0x138040C7
	addis 27, 0, __vmx_422c0000422c0000422c0000422c0000@ha
	.long 0x136038C7
	addis 26, 0, __vmx_42300000423000004230000042300000@ha
	.long 0x134030C7
	addis 25, 0, __vmx_42340000423400004234000042340000@ha
	.long 0x132028C7
	addi 4, 29, __vmx_42240000422400004224000042240000@l
	addi 3, 28, __vmx_42280000422800004228000042280000@l
	addi 11, 27, __vmx_422c0000422c0000422c0000422c0000@l
	addi 10, 26, __vmx_42300000423000004230000042300000@l
	addi 9, 25, __vmx_42340000423400004234000042340000@l
	addis 24, 0, __vmx_42380000423800004238000042380000@ha
	.long 0x130020C7
	addis 23, 0, __vmx_423c0000423c0000423c0000423c0000@ha
	.long 0x12E018C7
	addis 22, 0, __vmx_42400000424000004240000042400000@ha
	.long 0x12C058C7
	addis 21, 0, __vmx_42440000424400004244000042440000@ha
	.long 0x12A050C7
	addi 8, 24, __vmx_42380000423800004238000042380000@l
	.long 0x128048C7
	addi 7, 23, __vmx_423c0000423c0000423c0000423c0000@l
	addi 6, 22, __vmx_42400000424000004240000042400000@l
	addi 5, 21, __vmx_42440000424400004244000042440000@l
	addis 4, 0, __vmx_42480000424800004248000042480000@ha
	addis 3, 0, __vmx_424c0000424c0000424c0000424c0000@ha
	.long 0x126040C7
	addis 11, 0, __vmx_42500000425000004250000042500000@ha
	.long 0x124038C7
	addis 10, 0, __vmx_42540000425400004254000042540000@ha
	.long 0x122030C7
	addis 9, 0, __vmx_42580000425800004258000042580000@ha
	.long 0x120028C7
	addis 8, 0, __vmx_425c0000425c0000425c0000425c0000@ha
	addis 7, 0, __vmx_42600000426000004260000042600000@ha
	addis 6, 0, __vmx_42640000426400004264000042640000@ha
	addis 5, 0, __vmx_42680000426800004268000042680000@ha
	addis 31, 0, __vmx_426c0000426c0000426c0000426c0000@ha
	addi 4, 4, __vmx_42480000424800004248000042480000@l
	addi 3, 3, __vmx_424c0000424c0000424c0000424c0000@l
	addi 11, 11, __vmx_42500000425000004250000042500000@l
	addi 10, 10, __vmx_42540000425400004254000042540000@l
	addi 9, 9, __vmx_42580000425800004258000042580000@l
	.long 0x17AC291D
	addi 8, 8, __vmx_425c0000425c0000425c0000425c0000@l
	addis 29, 0, __vmx_42740000427400004274000042740000@ha
	.long 0x11E020C7
	addi 7, 7, __vmx_42600000426000004260000042600000@l
	.long 0x11C018C7
	addi 6, 6, __vmx_42640000426400004264000042640000@l
	.long 0x11A058C7
	addi 5, 5, __vmx_42680000426800004268000042680000@l
	.long 0x118050C7
	.long 0x114040C7
	addi 8, 29, __vmx_42740000427400004274000042740000@l
	.long 0x116048C7
	addis 4, 0, __vmx_42780000427800004278000042780000@ha
	addis 3, 0, __vmx_427c0000427c0000427c0000427c0000@ha
	.long 0x112038C7
	addis 11, 0, __vmx_42800000428000004280000042800000@ha
	.long 0x110030C7
	addis 10, 0, __vmx_42820000428200004282000042820000@ha
	.long 0x10E028C7
	addis 9, 0, __vmx_42840000428400004284000042840000@ha
	.long 0x118040C3
	addis 30, 0, __vmx_42700000427000004270000042700000@ha
	.long 0x17BC5CFC
	addis 28, 0, __vmx_42860000428600004286000042860000@ha
	addis 27, 0, __vmx_42880000428800004288000042880000@ha
	addis 26, 0, __vmx_428a0000428a0000428a0000428a0000@ha
	addis 25, 0, __vmx_428c0000428c0000428c0000428c0000@ha
	addi 7, 4, __vmx_42780000427800004278000042780000@l
	addi 6, 3, __vmx_427c0000427c0000427c0000427c0000@l
	addi 5, 11, __vmx_42800000428000004280000042800000@l
	addi 4, 10, __vmx_42820000428200004282000042820000@l
	addi 3, 9, __vmx_42840000428400004284000042840000@l
	addi 31, 31, __vmx_426c0000426c0000426c0000426c0000@l
	.long 0x10A038C7
	addi 30, 30, __vmx_42700000427000004270000042700000@l
	.long 0x108030C7
	addi 11, 28, __vmx_42860000428600004286000042860000@l
	.long 0x106028C7
	.long 0x155BED33
	addi 10, 27, __vmx_42880000428800004288000042880000@l
	addi 9, 26, __vmx_428a0000428a0000428a0000428a0000@l
	.long 0x104020C7
	addi 8, 25, __vmx_428c0000428c0000428c0000428c0000@l
	.long 0x10C0F8C7
	.long 0x11A0F0C3
	.long 0x102018C7
	.long 0x100058C7
	.long 0x13E050CF
	.long 0x13C048CF
	.long 0x116040C3
	.long 0x155A4CF0
	.long 0x155944F0
	.long 0x15583CF0
	.long 0x155734F0
	.long 0x15562CF0
	.long 0x155524F0
	.long 0x15541CF0
	.long 0x155314F0
	.long 0x15520CF0
	.long 0x1551FCF0
	.long 0x1550F4F0
	.long 0x154FECF0
	.long 0x154EE4F0
	.long 0x154DDCF0
	.long 0x154CD4F0
	.long 0x154BCCF0
	.long 0x154AC4F0
	.long 0x1549BCF0
	.long 0x1548B4F0
	.long 0x1547ACF0
	.long 0x1546A4F0
	.long 0x15459CF0
	.long 0x154494F0
	.long 0x15438CF0
	.long 0x154284F0
	.long 0x15417CF0
	.long 0x154074F0
	.long 0x155FFCD1
	.long 0x141E5510
	.long 0x141DF4D1
	.long 0x141CECD1
	.long 0x38E10060
	.long 0x38C10070
	.long 0x38A10080
	.long 0x38810090
	.long 0x386100A0
	.long 0x13E038C7
	.long 0x396100B0
	.long 0x13C030C7
	.long 0x114028C3
	.long 0x112020C3
	.long 0x110018C3
	.long 0x102058C3
	.long 0x141BE4D1
	.long 0x141ADCD1
	.long 0x1419D4D1
	.long 0x1418CCD1
	.long 0x1417C4D1
	.long 0x1416BCD1
	.long 0x1415B4D1
	.long 0x1414ACD1
	.long 0x1413A4D1
	.long 0x14129CD1
	.long 0x141194D1
	.long 0x14108CD1
	.long 0x140F84D1
	.long 0x140E7CD1
	.long 0x140D74D1
	.long 0x140C6CD1
	.long 0x140B64D1
	.long 0x140A5CD1
	.long 0x140954D1
	.long 0x14084CD1
	.long 0x140744D1
	.long 0x14063CD1
	.long 0x140534D1
	.long 0x14046CD0
	.long 0x140364D0
	.long 0x14022CD1
	.long 0x140124D1
	.long 0x14001CD1
	.long 0x141F10F1
	.long 0x141E08F1
	.long 0x140A00D1
	.long 0x1409F8D3
	.long 0x1408F0D3
	.long 0x142B0110
	.long 0x38210540
	.long 0x3981FF80
	bl __restvmx_64
	b __restgprlr_17
__coffstatic__M3219_215:
	.size cl_keep_across, __coffsec67+1844-cl_keep_across

	.section ".rodata.__vmx@428c0000428c0000428c0000428c0000","aG",@progbits,__vmx_428c0000428c0000428c0000428c0000,comdat
	.p2align 4
__coffsec68:
	.globl __vmx_428c0000428c0000428c0000428c0000
	.type __vmx_428c0000428c0000428c0000428c0000, @object
__vmx_428c0000428c0000428c0000428c0000:
	.long 0x428C0000
	.long 0x428C0000
	.long 0x428C0000
	.long 0x428C0000

	.section ".rodata.__vmx@428a0000428a0000428a0000428a0000","aG",@progbits,__vmx_428a0000428a0000428a0000428a0000,comdat
	.p2align 4
__coffsec69:
	.globl __vmx_428a0000428a0000428a0000428a0000
	.type __vmx_428a0000428a0000428a0000428a0000, @object
__vmx_428a0000428a0000428a0000428a0000:
	.long 0x428A0000
	.long 0x428A0000
	.long 0x428A0000
	.long 0x428A0000

	.section ".rodata.__vmx@42880000428800004288000042880000","aG",@progbits,__vmx_42880000428800004288000042880000,comdat
	.p2align 4
__coffsec70:
	.globl __vmx_42880000428800004288000042880000
	.type __vmx_42880000428800004288000042880000, @object
__vmx_42880000428800004288000042880000:
	.long 0x42880000
	.long 0x42880000
	.long 0x42880000
	.long 0x42880000

	.section ".rodata.__vmx@42860000428600004286000042860000","aG",@progbits,__vmx_42860000428600004286000042860000,comdat
	.p2align 4
__coffsec71:
	.globl __vmx_42860000428600004286000042860000
	.type __vmx_42860000428600004286000042860000, @object
__vmx_42860000428600004286000042860000:
	.long 0x42860000
	.long 0x42860000
	.long 0x42860000
	.long 0x42860000

	.section ".rodata.__vmx@42700000427000004270000042700000","aG",@progbits,__vmx_42700000427000004270000042700000,comdat
	.p2align 4
__coffsec72:
	.globl __vmx_42700000427000004270000042700000
	.type __vmx_42700000427000004270000042700000, @object
__vmx_42700000427000004270000042700000:
	.long 0x42700000
	.long 0x42700000
	.long 0x42700000
	.long 0x42700000

	.section ".rodata.__vmx@42840000428400004284000042840000","aG",@progbits,__vmx_42840000428400004284000042840000,comdat
	.p2align 4
__coffsec73:
	.globl __vmx_42840000428400004284000042840000
	.type __vmx_42840000428400004284000042840000, @object
__vmx_42840000428400004284000042840000:
	.long 0x42840000
	.long 0x42840000
	.long 0x42840000
	.long 0x42840000

	.section ".rodata.__vmx@42820000428200004282000042820000","aG",@progbits,__vmx_42820000428200004282000042820000,comdat
	.p2align 4
__coffsec74:
	.globl __vmx_42820000428200004282000042820000
	.type __vmx_42820000428200004282000042820000, @object
__vmx_42820000428200004282000042820000:
	.long 0x42820000
	.long 0x42820000
	.long 0x42820000
	.long 0x42820000

	.section ".rodata.__vmx@42800000428000004280000042800000","aG",@progbits,__vmx_42800000428000004280000042800000,comdat
	.p2align 4
__coffsec75:
	.globl __vmx_42800000428000004280000042800000
	.type __vmx_42800000428000004280000042800000, @object
__vmx_42800000428000004280000042800000:
	.long 0x42800000
	.long 0x42800000
	.long 0x42800000
	.long 0x42800000

	.section ".rodata.__vmx@427c0000427c0000427c0000427c0000","aG",@progbits,__vmx_427c0000427c0000427c0000427c0000,comdat
	.p2align 4
__coffsec76:
	.globl __vmx_427c0000427c0000427c0000427c0000
	.type __vmx_427c0000427c0000427c0000427c0000, @object
__vmx_427c0000427c0000427c0000427c0000:
	.long 0x427C0000
	.long 0x427C0000
	.long 0x427C0000
	.long 0x427C0000

	.section ".rodata.__vmx@42780000427800004278000042780000","aG",@progbits,__vmx_42780000427800004278000042780000,comdat
	.p2align 4
__coffsec77:
	.globl __vmx_42780000427800004278000042780000
	.type __vmx_42780000427800004278000042780000, @object
__vmx_42780000427800004278000042780000:
	.long 0x42780000
	.long 0x42780000
	.long 0x42780000
	.long 0x42780000

	.section ".rodata.__vmx@42740000427400004274000042740000","aG",@progbits,__vmx_42740000427400004274000042740000,comdat
	.p2align 4
__coffsec78:
	.globl __vmx_42740000427400004274000042740000
	.type __vmx_42740000427400004274000042740000, @object
__vmx_42740000427400004274000042740000:
	.long 0x42740000
	.long 0x42740000
	.long 0x42740000
	.long 0x42740000

	.section ".rodata.__vmx@426c0000426c0000426c0000426c0000","aG",@progbits,__vmx_426c0000426c0000426c0000426c0000,comdat
	.p2align 4
__coffsec79:
	.globl __vmx_426c0000426c0000426c0000426c0000
	.type __vmx_426c0000426c0000426c0000426c0000, @object
__vmx_426c0000426c0000426c0000426c0000:
	.long 0x426C0000
	.long 0x426C0000
	.long 0x426C0000
	.long 0x426C0000

	.section ".rodata.__vmx@42680000426800004268000042680000","aG",@progbits,__vmx_42680000426800004268000042680000,comdat
	.p2align 4
__coffsec80:
	.globl __vmx_42680000426800004268000042680000
	.type __vmx_42680000426800004268000042680000, @object
__vmx_42680000426800004268000042680000:
	.long 0x42680000
	.long 0x42680000
	.long 0x42680000
	.long 0x42680000

	.section ".rodata.__vmx@42640000426400004264000042640000","aG",@progbits,__vmx_42640000426400004264000042640000,comdat
	.p2align 4
__coffsec81:
	.globl __vmx_42640000426400004264000042640000
	.type __vmx_42640000426400004264000042640000, @object
__vmx_42640000426400004264000042640000:
	.long 0x42640000
	.long 0x42640000
	.long 0x42640000
	.long 0x42640000

	.section ".rodata.__vmx@42600000426000004260000042600000","aG",@progbits,__vmx_42600000426000004260000042600000,comdat
	.p2align 4
__coffsec82:
	.globl __vmx_42600000426000004260000042600000
	.type __vmx_42600000426000004260000042600000, @object
__vmx_42600000426000004260000042600000:
	.long 0x42600000
	.long 0x42600000
	.long 0x42600000
	.long 0x42600000

	.section ".rodata.__vmx@425c0000425c0000425c0000425c0000","aG",@progbits,__vmx_425c0000425c0000425c0000425c0000,comdat
	.p2align 4
__coffsec83:
	.globl __vmx_425c0000425c0000425c0000425c0000
	.type __vmx_425c0000425c0000425c0000425c0000, @object
__vmx_425c0000425c0000425c0000425c0000:
	.long 0x425C0000
	.long 0x425C0000
	.long 0x425C0000
	.long 0x425C0000

	.section ".rodata.__vmx@42580000425800004258000042580000","aG",@progbits,__vmx_42580000425800004258000042580000,comdat
	.p2align 4
__coffsec84:
	.globl __vmx_42580000425800004258000042580000
	.type __vmx_42580000425800004258000042580000, @object
__vmx_42580000425800004258000042580000:
	.long 0x42580000
	.long 0x42580000
	.long 0x42580000
	.long 0x42580000

	.section ".rodata.__vmx@42540000425400004254000042540000","aG",@progbits,__vmx_42540000425400004254000042540000,comdat
	.p2align 4
__coffsec85:
	.globl __vmx_42540000425400004254000042540000
	.type __vmx_42540000425400004254000042540000, @object
__vmx_42540000425400004254000042540000:
	.long 0x42540000
	.long 0x42540000
	.long 0x42540000
	.long 0x42540000

	.section ".rodata.__vmx@42500000425000004250000042500000","aG",@progbits,__vmx_42500000425000004250000042500000,comdat
	.p2align 4
__coffsec86:
	.globl __vmx_42500000425000004250000042500000
	.type __vmx_42500000425000004250000042500000, @object
__vmx_42500000425000004250000042500000:
	.long 0x42500000
	.long 0x42500000
	.long 0x42500000
	.long 0x42500000

	.section ".rodata.__vmx@424c0000424c0000424c0000424c0000","aG",@progbits,__vmx_424c0000424c0000424c0000424c0000,comdat
	.p2align 4
__coffsec87:
	.globl __vmx_424c0000424c0000424c0000424c0000
	.type __vmx_424c0000424c0000424c0000424c0000, @object
__vmx_424c0000424c0000424c0000424c0000:
	.long 0x424C0000
	.long 0x424C0000
	.long 0x424C0000
	.long 0x424C0000

	.section ".rodata.__vmx@42480000424800004248000042480000","aG",@progbits,__vmx_42480000424800004248000042480000,comdat
	.p2align 4
__coffsec88:
	.globl __vmx_42480000424800004248000042480000
	.type __vmx_42480000424800004248000042480000, @object
__vmx_42480000424800004248000042480000:
	.long 0x42480000
	.long 0x42480000
	.long 0x42480000
	.long 0x42480000

	.section ".rodata.__vmx@42440000424400004244000042440000","aG",@progbits,__vmx_42440000424400004244000042440000,comdat
	.p2align 4
__coffsec89:
	.globl __vmx_42440000424400004244000042440000
	.type __vmx_42440000424400004244000042440000, @object
__vmx_42440000424400004244000042440000:
	.long 0x42440000
	.long 0x42440000
	.long 0x42440000
	.long 0x42440000

	.section ".rodata.__vmx@42400000424000004240000042400000","aG",@progbits,__vmx_42400000424000004240000042400000,comdat
	.p2align 4
__coffsec90:
	.globl __vmx_42400000424000004240000042400000
	.type __vmx_42400000424000004240000042400000, @object
__vmx_42400000424000004240000042400000:
	.long 0x42400000
	.long 0x42400000
	.long 0x42400000
	.long 0x42400000

	.section ".rodata.__vmx@423c0000423c0000423c0000423c0000","aG",@progbits,__vmx_423c0000423c0000423c0000423c0000,comdat
	.p2align 4
__coffsec91:
	.globl __vmx_423c0000423c0000423c0000423c0000
	.type __vmx_423c0000423c0000423c0000423c0000, @object
__vmx_423c0000423c0000423c0000423c0000:
	.long 0x423C0000
	.long 0x423C0000
	.long 0x423C0000
	.long 0x423C0000

	.section ".rodata.__vmx@42380000423800004238000042380000","aG",@progbits,__vmx_42380000423800004238000042380000,comdat
	.p2align 4
__coffsec92:
	.globl __vmx_42380000423800004238000042380000
	.type __vmx_42380000423800004238000042380000, @object
__vmx_42380000423800004238000042380000:
	.long 0x42380000
	.long 0x42380000
	.long 0x42380000
	.long 0x42380000

	.section ".rodata.__vmx@42340000423400004234000042340000","aG",@progbits,__vmx_42340000423400004234000042340000,comdat
	.p2align 4
__coffsec93:
	.globl __vmx_42340000423400004234000042340000
	.type __vmx_42340000423400004234000042340000, @object
__vmx_42340000423400004234000042340000:
	.long 0x42340000
	.long 0x42340000
	.long 0x42340000
	.long 0x42340000

	.section ".rodata.__vmx@42300000423000004230000042300000","aG",@progbits,__vmx_42300000423000004230000042300000,comdat
	.p2align 4
__coffsec94:
	.globl __vmx_42300000423000004230000042300000
	.type __vmx_42300000423000004230000042300000, @object
__vmx_42300000423000004230000042300000:
	.long 0x42300000
	.long 0x42300000
	.long 0x42300000
	.long 0x42300000

	.section ".rodata.__vmx@422c0000422c0000422c0000422c0000","aG",@progbits,__vmx_422c0000422c0000422c0000422c0000,comdat
	.p2align 4
__coffsec95:
	.globl __vmx_422c0000422c0000422c0000422c0000
	.type __vmx_422c0000422c0000422c0000422c0000, @object
__vmx_422c0000422c0000422c0000422c0000:
	.long 0x422C0000
	.long 0x422C0000
	.long 0x422C0000
	.long 0x422C0000

	.section ".rodata.__vmx@42280000422800004228000042280000","aG",@progbits,__vmx_42280000422800004228000042280000,comdat
	.p2align 4
__coffsec96:
	.globl __vmx_42280000422800004228000042280000
	.type __vmx_42280000422800004228000042280000, @object
__vmx_42280000422800004228000042280000:
	.long 0x42280000
	.long 0x42280000
	.long 0x42280000
	.long 0x42280000

	.section ".rodata.__vmx@42240000422400004224000042240000","aG",@progbits,__vmx_42240000422400004224000042240000,comdat
	.p2align 4
__coffsec97:
	.globl __vmx_42240000422400004224000042240000
	.type __vmx_42240000422400004224000042240000, @object
__vmx_42240000422400004224000042240000:
	.long 0x42240000
	.long 0x42240000
	.long 0x42240000
	.long 0x42240000

	.section ".rodata.__vmx@42200000422000004220000042200000","aG",@progbits,__vmx_42200000422000004220000042200000,comdat
	.p2align 4
__coffsec98:
	.globl __vmx_42200000422000004220000042200000
	.type __vmx_42200000422000004220000042200000, @object
__vmx_42200000422000004220000042200000:
	.long 0x42200000
	.long 0x42200000
	.long 0x42200000
	.long 0x42200000

	.section ".rodata.__vmx@42180000421800004218000042180000","aG",@progbits,__vmx_42180000421800004218000042180000,comdat
	.p2align 4
__coffsec99:
	.globl __vmx_42180000421800004218000042180000
	.type __vmx_42180000421800004218000042180000, @object
__vmx_42180000421800004218000042180000:
	.long 0x42180000
	.long 0x42180000
	.long 0x42180000
	.long 0x42180000

	.section ".rodata.__vmx@42140000421400004214000042140000","aG",@progbits,__vmx_42140000421400004214000042140000,comdat
	.p2align 4
__coffsec100:
	.globl __vmx_42140000421400004214000042140000
	.type __vmx_42140000421400004214000042140000, @object
__vmx_42140000421400004214000042140000:
	.long 0x42140000
	.long 0x42140000
	.long 0x42140000
	.long 0x42140000

	.section ".rodata.__vmx@421c0000421c0000421c0000421c0000","aG",@progbits,__vmx_421c0000421c0000421c0000421c0000,comdat
	.p2align 4
__coffsec101:
	.globl __vmx_421c0000421c0000421c0000421c0000
	.type __vmx_421c0000421c0000421c0000421c0000, @object
__vmx_421c0000421c0000421c0000421c0000:
	.long 0x421C0000
	.long 0x421C0000
	.long 0x421C0000
	.long 0x421C0000

	.section ".rodata.__vmx@42100000421000004210000042100000","aG",@progbits,__vmx_42100000421000004210000042100000,comdat
	.p2align 4
__coffsec102:
	.globl __vmx_42100000421000004210000042100000
	.type __vmx_42100000421000004210000042100000, @object
__vmx_42100000421000004210000042100000:
	.long 0x42100000
	.long 0x42100000
	.long 0x42100000
	.long 0x42100000

	.section ".rodata.__vmx@420c0000420c0000420c0000420c0000","aG",@progbits,__vmx_420c0000420c0000420c0000420c0000,comdat
	.p2align 4
__coffsec103:
	.globl __vmx_420c0000420c0000420c0000420c0000
	.type __vmx_420c0000420c0000420c0000420c0000, @object
__vmx_420c0000420c0000420c0000420c0000:
	.long 0x420C0000
	.long 0x420C0000
	.long 0x420C0000
	.long 0x420C0000

	.section ".rodata.__vmx@42080000420800004208000042080000","aG",@progbits,__vmx_42080000420800004208000042080000,comdat
	.p2align 4
__coffsec104:
	.globl __vmx_42080000420800004208000042080000
	.type __vmx_42080000420800004208000042080000, @object
__vmx_42080000420800004208000042080000:
	.long 0x42080000
	.long 0x42080000
	.long 0x42080000
	.long 0x42080000

	.section ".rodata.__vmx@42040000420400004204000042040000","aG",@progbits,__vmx_42040000420400004204000042040000,comdat
	.p2align 4
__coffsec105:
	.globl __vmx_42040000420400004204000042040000
	.type __vmx_42040000420400004204000042040000, @object
__vmx_42040000420400004204000042040000:
	.long 0x42040000
	.long 0x42040000
	.long 0x42040000
	.long 0x42040000

	.section ".rodata.__vmx@42000000420000004200000042000000","aG",@progbits,__vmx_42000000420000004200000042000000,comdat
	.p2align 4
__coffsec106:
	.globl __vmx_42000000420000004200000042000000
	.type __vmx_42000000420000004200000042000000, @object
__vmx_42000000420000004200000042000000:
	.long 0x42000000
	.long 0x42000000
	.long 0x42000000
	.long 0x42000000

	.section ".rodata.__vmx@41f8000041f8000041f8000041f80000","aG",@progbits,__vmx_41f8000041f8000041f8000041f80000,comdat
	.p2align 4
__coffsec107:
	.globl __vmx_41f8000041f8000041f8000041f80000
	.type __vmx_41f8000041f8000041f8000041f80000, @object
__vmx_41f8000041f8000041f8000041f80000:
	.long 0x41F80000
	.long 0x41F80000
	.long 0x41F80000
	.long 0x41F80000

	.section ".rodata.__vmx@41f0000041f0000041f0000041f00000","aG",@progbits,__vmx_41f0000041f0000041f0000041f00000,comdat
	.p2align 4
__coffsec108:
	.globl __vmx_41f0000041f0000041f0000041f00000
	.type __vmx_41f0000041f0000041f0000041f00000, @object
__vmx_41f0000041f0000041f0000041f00000:
	.long 0x41F00000
	.long 0x41F00000
	.long 0x41F00000
	.long 0x41F00000

	.section ".rodata.__vmx@41e8000041e8000041e8000041e80000","aG",@progbits,__vmx_41e8000041e8000041e8000041e80000,comdat
	.p2align 4
__coffsec109:
	.globl __vmx_41e8000041e8000041e8000041e80000
	.type __vmx_41e8000041e8000041e8000041e80000, @object
__vmx_41e8000041e8000041e8000041e80000:
	.long 0x41E80000
	.long 0x41E80000
	.long 0x41E80000
	.long 0x41E80000

	.section ".rodata.__vmx@41e0000041e0000041e0000041e00000","aG",@progbits,__vmx_41e0000041e0000041e0000041e00000,comdat
	.p2align 4
__coffsec110:
	.globl __vmx_41e0000041e0000041e0000041e00000
	.type __vmx_41e0000041e0000041e0000041e00000, @object
__vmx_41e0000041e0000041e0000041e00000:
	.long 0x41E00000
	.long 0x41E00000
	.long 0x41E00000
	.long 0x41E00000

	.section ".rodata.__vmx@41d8000041d8000041d8000041d80000","aG",@progbits,__vmx_41d8000041d8000041d8000041d80000,comdat
	.p2align 4
__coffsec111:
	.globl __vmx_41d8000041d8000041d8000041d80000
	.type __vmx_41d8000041d8000041d8000041d80000, @object
__vmx_41d8000041d8000041d8000041d80000:
	.long 0x41D80000
	.long 0x41D80000
	.long 0x41D80000
	.long 0x41D80000

	.section ".rodata.__vmx@41d0000041d0000041d0000041d00000","aG",@progbits,__vmx_41d0000041d0000041d0000041d00000,comdat
	.p2align 4
__coffsec112:
	.globl __vmx_41d0000041d0000041d0000041d00000
	.type __vmx_41d0000041d0000041d0000041d00000, @object
__vmx_41d0000041d0000041d0000041d00000:
	.long 0x41D00000
	.long 0x41D00000
	.long 0x41D00000
	.long 0x41D00000

	.section ".rodata.__vmx@41c8000041c8000041c8000041c80000","aG",@progbits,__vmx_41c8000041c8000041c8000041c80000,comdat
	.p2align 4
__coffsec113:
	.globl __vmx_41c8000041c8000041c8000041c80000
	.type __vmx_41c8000041c8000041c8000041c80000, @object
__vmx_41c8000041c8000041c8000041c80000:
	.long 0x41C80000
	.long 0x41C80000
	.long 0x41C80000
	.long 0x41C80000

	.section ".rodata.__vmx@41b8000041b8000041b8000041b80000","aG",@progbits,__vmx_41b8000041b8000041b8000041b80000,comdat
	.p2align 4
__coffsec114:
	.globl __vmx_41b8000041b8000041b8000041b80000
	.type __vmx_41b8000041b8000041b8000041b80000, @object
__vmx_41b8000041b8000041b8000041b80000:
	.long 0x41B80000
	.long 0x41B80000
	.long 0x41B80000
	.long 0x41B80000

	.section ".rodata.__vmx@41b0000041b0000041b0000041b00000","aG",@progbits,__vmx_41b0000041b0000041b0000041b00000,comdat
	.p2align 4
__coffsec115:
	.globl __vmx_41b0000041b0000041b0000041b00000
	.type __vmx_41b0000041b0000041b0000041b00000, @object
__vmx_41b0000041b0000041b0000041b00000:
	.long 0x41B00000
	.long 0x41B00000
	.long 0x41B00000
	.long 0x41B00000

	.section ".rodata.__vmx@41a8000041a8000041a8000041a80000","aG",@progbits,__vmx_41a8000041a8000041a8000041a80000,comdat
	.p2align 4
__coffsec116:
	.globl __vmx_41a8000041a8000041a8000041a80000
	.type __vmx_41a8000041a8000041a8000041a80000, @object
__vmx_41a8000041a8000041a8000041a80000:
	.long 0x41A80000
	.long 0x41A80000
	.long 0x41A80000
	.long 0x41A80000

	.section ".rodata.__vmx@41a0000041a0000041a0000041a00000","aG",@progbits,__vmx_41a0000041a0000041a0000041a00000,comdat
	.p2align 4
__coffsec117:
	.globl __vmx_41a0000041a0000041a0000041a00000
	.type __vmx_41a0000041a0000041a0000041a00000, @object
__vmx_41a0000041a0000041a0000041a00000:
	.long 0x41A00000
	.long 0x41A00000
	.long 0x41A00000
	.long 0x41A00000

	.section ".rodata.__vmx@41c0000041c0000041c0000041c00000","aG",@progbits,__vmx_41c0000041c0000041c0000041c00000,comdat
	.p2align 4
__coffsec118:
	.globl __vmx_41c0000041c0000041c0000041c00000
	.type __vmx_41c0000041c0000041c0000041c00000, @object
__vmx_41c0000041c0000041c0000041c00000:
	.long 0x41C00000
	.long 0x41C00000
	.long 0x41C00000
	.long 0x41C00000

	.section ".rodata.__vmx@41980000419800004198000041980000","aG",@progbits,__vmx_41980000419800004198000041980000,comdat
	.p2align 4
__coffsec119:
	.globl __vmx_41980000419800004198000041980000
	.type __vmx_41980000419800004198000041980000, @object
__vmx_41980000419800004198000041980000:
	.long 0x41980000
	.long 0x41980000
	.long 0x41980000
	.long 0x41980000

	.section ".rodata.__vmx@41900000419000004190000041900000","aG",@progbits,__vmx_41900000419000004190000041900000,comdat
	.p2align 4
__coffsec120:
	.globl __vmx_41900000419000004190000041900000
	.type __vmx_41900000419000004190000041900000, @object
__vmx_41900000419000004190000041900000:
	.long 0x41900000
	.long 0x41900000
	.long 0x41900000
	.long 0x41900000

	.section ".rodata.__vmx@41880000418800004188000041880000","aG",@progbits,__vmx_41880000418800004188000041880000,comdat
	.p2align 4
__coffsec121:
	.globl __vmx_41880000418800004188000041880000
	.type __vmx_41880000418800004188000041880000, @object
__vmx_41880000418800004188000041880000:
	.long 0x41880000
	.long 0x41880000
	.long 0x41880000
	.long 0x41880000

	.section ".text.cl_pressure","ax",@progbits
	.p2align 3
__coffsec123:
	.globl cl_pressure
	.type cl_pressure, @function
cl_pressure:
	.long 0x7D8802A6
	bl __savegprlr_14
	.long 0x3981FF60
	bl __savevmx_85
	.long 0x9421FC90
__coffstatic__M4243_468:
	addis 11, 0, __vmx_00000000000000000000000000000000@ha
	.long 0x90810014
	addis 10, 0, __vmx_42c0000042c0000042c0000042c00000@ha
	.long 0x90610010
	addis 9, 0, __vmx_42be000042be000042be000042be0000@ha
	addis 8, 0, __vmx_42bc000042bc000042bc000042bc0000@ha
	addi 11, 11, __vmx_00000000000000000000000000000000@l
	addi 10, 10, __vmx_42c0000042c0000042c0000042c00000@l
	addi 9, 9, __vmx_42be000042be000042be000042be0000@l
	addi 8, 8, __vmx_42bc000042bc000042bc000042bc0000@l
	addis 7, 0, __vmx_42ba000042ba000042ba000042ba0000@ha
	addis 6, 0, __vmx_42b8000042b8000042b8000042b80000@ha
	.long 0x100058C3
	addis 5, 0, __vmx_42b6000042b6000042b6000042b60000@ha
	.long 0x11A050C3
	addis 31, 0, __vmx_42b4000042b4000042b4000042b40000@ha
	.long 0x118048C3
	addis 30, 0, __vmx_42b2000042b2000042b2000042b20000@ha
	.long 0x112040CF
	addis 29, 0, __vmx_42b0000042b0000042b0000042b00000@ha
	addis 28, 0, __vmx_42ae000042ae000042ae000042ae0000@ha
	addi 7, 7, __vmx_42ba000042ba000042ba000042ba0000@l
	addi 6, 6, __vmx_42b8000042b8000042b8000042b80000@l
	addi 5, 5, __vmx_42b6000042b6000042b6000042b60000@l
	addi 11, 31, __vmx_42b4000042b4000042b4000042b40000@l
	addi 10, 30, __vmx_42b2000042b2000042b2000042b20000@l
	addi 9, 29, __vmx_42b0000042b0000042b0000042b00000@l
	.long 0x106038CF
	addi 8, 28, __vmx_42ae000042ae000042ae000042ae0000@l
	.long 0x112030C3
	addis 27, 0, __vmx_42ac000042ac000042ac000042ac0000@ha
	.long 0x110028C3
	addis 26, 0, __vmx_42aa000042aa000042aa000042aa0000@ha
	.long 0x10E058C3
	addis 25, 0, __vmx_42a8000042a8000042a8000042a80000@ha
	.long 0x10C050C3
	addis 24, 0, __vmx_42a6000042a6000042a6000042a60000@ha
	.long 0x10A048C3
	addis 23, 0, __vmx_42a4000042a4000042a4000042a40000@ha
	.long 0x108040C3
	addis 22, 0, __vmx_42a2000042a2000042a2000042a20000@ha
	addis 21, 0, __vmx_42a0000042a0000042a0000042a00000@ha
	addi 7, 27, __vmx_42ac000042ac000042ac000042ac0000@l
	addi 6, 26, __vmx_42aa000042aa000042aa000042aa0000@l
	addi 5, 25, __vmx_42a8000042a8000042a8000042a80000@l
	addi 11, 24, __vmx_42a6000042a6000042a6000042a60000@l
	addi 10, 23, __vmx_42a4000042a4000042a4000042a40000@l
	addi 9, 22, __vmx_42a2000042a2000042a2000042a20000@l
	.long 0x106038C3
	addi 8, 21, __vmx_42a0000042a0000042a0000042a00000@l
	.long 0x104030C3
	addis 20, 0, __vmx_429e0000429e0000429e0000429e0000@ha
	.long 0x102028C3
	addis 19, 0, __vmx_429c0000429c0000429c0000429c0000@ha
	.long 0x13E058C3
	addis 18, 0, __vmx_429a0000429a0000429a0000429a0000@ha
	.long 0x13C050C3
	addis 17, 0, __vmx_42980000429800004298000042980000@ha
	.long 0x13A048C3
	addis 16, 0, __vmx_42960000429600004296000042960000@ha
	.long 0x138040C3
	addis 15, 0, __vmx_42940000429400004294000042940000@ha
	addis 14, 0, __vmx_42920000429200004292000042920000@ha
	addi 7, 20, __vmx_429e0000429e0000429e0000429e0000@l
	addi 6, 19, __vmx_429c0000429c0000429c0000429c0000@l
	addi 5, 18, __vmx_429a0000429a0000429a0000429a0000@l
	addi 11, 17, __vmx_42980000429800004298000042980000@l
	addi 10, 16, __vmx_42960000429600004296000042960000@l
	addi 9, 15, __vmx_42940000429400004294000042940000@l
	.long 0x136038C3
	addi 8, 14, __vmx_42920000429200004292000042920000@l
	.long 0x134030C3
	addis 4, 0, __vmx_42900000429000004290000042900000@ha
	.long 0x132028C3
	addis 3, 0, __vmx_428e0000428e0000428e0000428e0000@ha
	.long 0x130058C3
	.long 0x12E050C3
	addi 7, 4, __vmx_42900000429000004290000042900000@l
	.long 0x12C048C3
	addi 6, 3, __vmx_428e0000428e0000428e0000428e0000@l
	.long 0x12A040C3
	addis 5, 0, __vmx_428c0000428c0000428c0000428c0000@ha
	.long 0x128038C3
	.long 0x126030C3
	addis 3, 0, __vmx_428a0000428a0000428a0000428a0000@ha
	addi 5, 5, __vmx_428c0000428c0000428c0000428c0000@l
	addis 11, 0, __vmx_42880000428800004288000042880000@ha
	addis 10, 0, __vmx_42860000428600004286000042860000@ha
	addis 9, 0, __vmx_42840000428400004284000042840000@ha
	addis 8, 0, __vmx_42820000428200004282000042820000@ha
	addis 7, 0, __vmx_42800000428000004280000042800000@ha
	.long 0x124028C3
	addis 6, 0, __vmx_427c0000427c0000427c0000427c0000@ha
	addis 31, 0, __vmx_42780000427800004278000042780000@ha
	addi 3, 3, __vmx_428a0000428a0000428a0000428a0000@l
	addi 11, 11, __vmx_42880000428800004288000042880000@l
	addi 10, 10, __vmx_42860000428600004286000042860000@l
	addi 9, 9, __vmx_42840000428400004284000042840000@l
	addi 8, 8, __vmx_42820000428200004282000042820000@l
	addi 7, 7, __vmx_42800000428000004280000042800000@l
	.long 0x122018C3
	addi 6, 6, __vmx_427c0000427c0000427c0000427c0000@l
	.long 0x120058C3
	addi 5, 31, __vmx_42780000427800004278000042780000@l
	.long 0x11E050C3
	addis 30, 0, __vmx_42740000427400004274000042740000@ha
	.long 0x11C048C3
	addis 29, 0, __vmx_42700000427000004270000042700000@ha
	.long 0x13E040C7
	addis 28, 0, __vmx_426c0000426c0000426c0000426c0000@ha
	.long 0x13C038C7
	addis 27, 0, __vmx_42680000426800004268000042680000@ha
	.long 0x13A030C7
	addis 26, 0, __vmx_42640000426400004264000042640000@ha
	.long 0x138028C7
	addis 25, 0, __vmx_42600000426000004260000042600000@ha
	addis 24, 0, __vmx_425c0000425c0000425c0000425c0000@ha
	addis 23, 0, __vmx_42580000425800004258000042580000@ha
	addi 3, 30, __vmx_42740000427400004274000042740000@l
	addi 11, 29, __vmx_42700000427000004270000042700000@l
	addi 10, 28, __vmx_426c0000426c0000426c0000426c0000@l
	addi 9, 27, __vmx_42680000426800004268000042680000@l
	addi 8, 26, __vmx_42640000426400004264000042640000@l
	addi 7, 25, __vmx_42600000426000004260000042600000@l
	.long 0x136018C7
	addi 6, 24, __vmx_425c0000425c0000425c0000425c0000@l
	.long 0x134058C7
	addi 5, 23, __vmx_42580000425800004258000042580000@l
	.long 0x132050C7
	addis 22, 0, __vmx_42540000425400004254000042540000@ha
	.long 0x130048C7
	addis 21, 0, __vmx_42500000425000004250000042500000@ha
	.long 0x12E040C7
	addis 20, 0, __vmx_424c0000424c0000424c0000424c0000@ha
	.long 0x12C038C7
	addis 19, 0, __vmx_42480000424800004248000042480000@ha
	.long 0x12A030C7
	addis 18, 0, __vmx_42440000424400004244000042440000@ha
	.long 0x128028C7
	addis 17, 0, __vmx_42400000424000004240000042400000@ha
	addis 16, 0, __vmx_423c0000423c0000423c0000423c0000@ha
	addis 15, 0, __vmx_42380000423800004238000042380000@ha
	addi 3, 22, __vmx_42540000425400004254000042540000@l
	addi 11, 21, __vmx_42500000425000004250000042500000@l
	addi 10, 20, __vmx_424c0000424c0000424c0000424c0000@l
	addi 9, 19, __vmx_42480000424800004248000042480000@l
	addi 8, 18, __vmx_42440000424400004244000042440000@l
	addi 7, 17, __vmx_42400000424000004240000042400000@l
	.long 0x126018C7
	addi 6, 16, __vmx_423c0000423c0000423c0000423c0000@l
	.long 0x124058C7
	addi 5, 15, __vmx_42380000423800004238000042380000@l
	.long 0x122050C7
	addis 14, 0, __vmx_42340000423400004234000042340000@ha
	.long 0x120048C7
	addis 4, 0, __vmx_42300000423000004230000042300000@ha
	.long 0x11E040C7
	.long 0x11C038C7
	addi 3, 14, __vmx_42340000423400004234000042340000@l
	.long 0x11A030C7
	addi 11, 4, __vmx_42300000423000004230000042300000@l
	.long 0x118028C7
	addis 10, 0, __vmx_422c0000422c0000422c0000422c0000@ha
	.long 0x116018C7
	.long 0x114058C7
	addis 9, 0, __vmx_42280000422800004228000042280000@ha
	addi 10, 10, __vmx_422c0000422c0000422c0000422c0000@l
	addis 8, 0, __vmx_42240000422400004224000042240000@ha
	addis 7, 0, __vmx_42200000422000004220000042200000@ha
	addis 6, 0, __vmx_421c0000421c0000421c0000421c0000@ha
	addis 5, 0, __vmx_42180000421800004218000042180000@ha
	addis 3, 0, __vmx_42140000421400004214000042140000@ha
	.long 0x112050C7
	addis 11, 0, __vmx_42100000421000004210000042100000@ha
	addis 31, 0, __vmx_420c0000420c0000420c0000420c0000@ha
	addi 9, 9, __vmx_42280000422800004228000042280000@l
	addi 8, 8, __vmx_42240000422400004224000042240000@l
	addi 7, 7, __vmx_42200000422000004220000042200000@l
	addi 6, 6, __vmx_421c0000421c0000421c0000421c0000@l
	addi 5, 5, __vmx_42180000421800004218000042180000@l
	addi 3, 3, __vmx_42140000421400004214000042140000@l
	.long 0x110048C7
	addi 11, 11, __vmx_42100000421000004210000042100000@l
	.long 0x10E040C7
	addi 10, 31, __vmx_420c0000420c0000420c0000420c0000@l
	.long 0x10C038C7
	addis 30, 0, __vmx_42080000420800004208000042080000@ha
	.long 0x10A030C7
	addis 29, 0, __vmx_42040000420400004204000042040000@ha
	.long 0x108028C7
	addis 28, 0, __vmx_42000000420000004200000042000000@ha
	.long 0x106018C7
	addis 27, 0, __vmx_41f8000041f8000041f8000041f80000@ha
	.long 0x104058C7
	addis 26, 0, __vmx_41f0000041f0000041f0000041f00000@ha
	.long 0x102050C7
	addis 25, 0, __vmx_41e8000041e8000041e8000041e80000@ha
	addis 24, 0, __vmx_41e0000041e0000041e0000041e00000@ha
	addis 23, 0, __vmx_41d8000041d8000041d8000041d80000@ha
	addi 9, 30, __vmx_42080000420800004208000042080000@l
	addi 8, 29, __vmx_42040000420400004204000042040000@l
	addi 7, 28, __vmx_42000000420000004200000042000000@l
	addi 6, 27, __vmx_41f8000041f8000041f8000041f80000@l
	addi 5, 26, __vmx_41f0000041f0000041f0000041f00000@l
	addi 3, 25, __vmx_41e8000041e8000041e8000041e80000@l
	.long 0x100048C7
	addi 11, 24, __vmx_41e0000041e0000041e0000041e00000@l
	.long 0x13E040CF
	addi 10, 23, __vmx_41d8000041d8000041d8000041d80000@l
	.long 0x13C038CF
	addis 22, 0, __vmx_41d0000041d0000041d0000041d00000@ha
	.long 0x13A030CF
	addis 21, 0, __vmx_41c8000041c8000041c8000041c80000@ha
	.long 0x138028CF
	addis 20, 0, __vmx_41c0000041c0000041c0000041c00000@ha
	.long 0x136018CF
	addis 19, 0, __vmx_41b8000041b8000041b8000041b80000@ha
	.long 0x134058CF
	addis 18, 0, __vmx_41b0000041b0000041b0000041b00000@ha
	.long 0x132050CF
	addis 17, 0, __vmx_41a8000041a8000041a8000041a80000@ha
	addis 16, 0, __vmx_41a0000041a0000041a0000041a00000@ha
	addis 15, 0, __vmx_41980000419800004198000041980000@ha
	addi 9, 22, __vmx_41d0000041d0000041d0000041d00000@l
	addi 8, 21, __vmx_41c8000041c8000041c8000041c80000@l
	addi 7, 20, __vmx_41c0000041c0000041c0000041c00000@l
	addi 6, 19, __vmx_41b8000041b8000041b8000041b80000@l
	addi 5, 18, __vmx_41b0000041b0000041b0000041b00000@l
	addi 3, 17, __vmx_41a8000041a8000041a8000041a80000@l
	.long 0x130048CF
	addi 11, 16, __vmx_41a0000041a0000041a0000041a00000@l
	.long 0x12E040CF
	addi 10, 15, __vmx_41980000419800004198000041980000@l
	.long 0x12C038CF
	addis 14, 0, __vmx_41900000419000004190000041900000@ha
	.long 0x12A030CF
	addis 4, 0, __vmx_41880000418800004188000041880000@ha
	.long 0x128028CF
	.long 0x126018CF
	addi 9, 14, __vmx_41900000419000004190000041900000@l
	.long 0x124058CF
	addi 8, 4, __vmx_41880000418800004188000041880000@l
	.long 0x122050CF
	.long 0x81410010
	.long 0x38E005F0
	.long 0x38C005E0
	.long 0x120048CF
	.long 0x38A005D0
	.long 0x11E040CF
	.long 0x386005C0
	.long 0x396005B0
	.long 0x11CA38CF
	.long 0x392005A0
	.long 0x140E6CF0
	.long 0x11AA30C3
	.long 0x11CA28CF
	.long 0x39000590
	.long 0x110A18CF
	.long 0x38E00580
	.long 0x104A58CF
	addis 6, 0, __vmx_41800000418000004180000041800000@ha
	.long 0x102A48CF
	addis 31, 0, __vmx_41100000411000004110000041100000@ha
	addi 6, 6, __vmx_41800000418000004180000041800000@l
	.long 0x100A40CF
	addis 5, 0, __vmx_41700000417000004170000041700000@ha
	.long 0x13EA38CB
	addis 3, 0, __vmx_41600000416000004160000041600000@ha
	addis 11, 0, __vmx_41500000415000004150000041500000@ha
	addis 9, 0, __vmx_41400000414000004140000041400000@ha
	addis 8, 0, __vmx_41300000413000004130000041300000@ha
	addis 7, 0, __vmx_41200000412000004120000041200000@ha
	.long 0x100D032E
	addi 5, 5, __vmx_41700000417000004170000041700000@l
	addi 3, 3, __vmx_41600000416000004160000041600000@l
	addi 11, 11, __vmx_41500000415000004150000041500000@l
	addi 9, 9, __vmx_41400000414000004140000041400000@l
	addi 8, 8, __vmx_41300000413000004130000041300000@l
	addi 7, 7, __vmx_41200000412000004120000041200000@l
	.long 0x11A028C3
	addis 30, 0, __vmx_41000000410000004100000041000000@ha
	.long 0x118018C3
	addis 29, 0, __vmx_40e0000040e0000040e0000040e00000@ha
	.long 0x116058C3
	addis 28, 0, __vmx_40c0000040c0000040c0000040c00000@ha
	addis 27, 0, __vmx_40a0000040a0000040a0000040a00000@ha
	.long 0x11A040CF
	addis 26, 0, __vmx_40800000408000004080000040800000@ha
	.long 0x118038CF
	addis 25, 0, __vmx_40400000404000004040000040400000@ha
	.long 0x152E053C
	.long 0x100030C3
	addi 6, 31, __vmx_41100000411000004110000041100000@l
	.long 0x11C048CF
	addis 24, 0, __vmx_40000000400000004000000040000000@ha
	addis 23, 0, __vmx_3f8000003f8000003f8000003f800000@ha
	.long 0x3AC00570
	.long 0x3AA00560
	.long 0x116030CF
	.long 0x3A800550
	addi 5, 30, __vmx_41000000410000004100000041000000@l
	addi 3, 29, __vmx_40e0000040e0000040e0000040e00000@l
	addi 11, 28, __vmx_40c0000040c0000040c0000040c00000@l
	.long 0x13CAB0CB
	addi 9, 27, __vmx_40a0000040a0000040a0000040a00000@l
	.long 0x13AAA8CB
	addi 8, 26, __vmx_40800000408000004080000040800000@l
	.long 0x138AA0CB
	.long 0x14684D3F
	addi 7, 25, __vmx_40400000404000004040000040400000@l
	addi 6, 24, __vmx_40000000400000004000000040000000@l
	.long 0x114028CF
	addi 31, 23, __vmx_3f8000003f8000003f8000003f800000@l
	.long 0x114018C3
	.long 0x112058CF
	.long 0x110048CF
	.long 0x10E040CF
	.long 0x10C038CF
	.long 0x10A030CF
	.long 0x1080F8CF
	.long 0x15221D33
	.long 0x152144F0
	.long 0x15203CF0
	.long 0x153F34D0
	.long 0x153E2CD0
	.long 0x38A00540
	.long 0x38600530
	.long 0x39600520
	.long 0x39200510
	.long 0x39000500
	.long 0x110A28C3
	.long 0x38E004F0
	.long 0x38C004E0
	.long 0x10EA58C3
	.long 0x38A004D0
	.long 0x10CA48C3
	.long 0x396004C0
	.long 0x10AA40C3
	.long 0x392004B0
	.long 0x390004A0
	.long 0x3BE00430
	.long 0x153D24D0
	.long 0x108A38C3
	.long 0x106A58CF
	.long 0x38E00490
	.long 0x104A48CF
	.long 0x39600470
	.long 0x102A40CF
	.long 0x39200460
	.long 0x39000450
	.long 0x3BC00420
	.long 0x100A38CF
	.long 0x38E00440
	.long 0x13EA48CB
	.long 0x39200410
	.long 0x13CA40CB
	.long 0x39000400
	.long 0x136AF0CB
	.long 0x13AA38CB
	.long 0x38E003F0
	.long 0x153C1CD0
	.long 0x106A30C3
	.long 0x38C00480
	.long 0x138AF8CB
	.long 0x134A48CB
	.long 0x3BE003E0
	.long 0x132A40CB
	.long 0x392003B0
	.long 0x130A38CB
	.long 0x390003A0
	.long 0x12EAF8CB
	.long 0x12CA48CB
	.long 0x12AA40CB
	.long 0x112848AE
	.long 0x110A18C3
	.long 0x104A28C3
	.long 0x38A003D0
	.long 0x386003C0
	.long 0x1128486E
	.long 0x110A30C3
	.long 0x102A58C3
	.long 0x11274FEE
	.long 0x10EA28C3
	.long 0x13EA18C3
	.long 0x11264FAE
	.long 0x11254F6E
	.long 0x11244F2E
	.long 0x11234EEE
	.long 0x11224EAE
	.long 0x1523CCF0
	.long 0x1522C4F0
	.long 0x1521BCF0
	.long 0x1520B4F0
	.long 0x11284D6E
	.long 0x11214D2E
	.long 0x153F9CD0
	.long 0x153E94D0
	.long 0x153D8CD0
	.long 0x153C84D0
	.long 0x153B7CD0
	.long 0x153A74D0
	.long 0x1539FCD1
	.long 0x1538F4D1
	.long 0x1537ECD1
	.long 0x1527E0D1
	.long 0x153FD8D1
	.long 0x38E00390
	.long 0x38C00380
	.long 0x1736B2F1
	.long 0x38A00370
	.long 0x38600360
	.long 0x39600350
	.long 0x110A38C3
	.long 0x39200340
	.long 0x39000330
	.long 0x10EA28C3
	.long 0x38E00320
	.long 0x130A18C3
	.long 0x38A00310
	.long 0x12EA58C3
	.long 0x38600300
	.long 0x12CA48C3
	.long 0x396002F0
	.long 0x12AA40C3
	.long 0x392002E0
	.long 0x1536D4D1
	.long 0x128A38C3
	.long 0x126A28C3
	.long 0x390002D0
	.long 0x124A18C3
	.long 0x38E002C0
	.long 0x122A58C3
	.long 0x38A002B0
	.long 0x120A48C3
	.long 0x386002A0
	.long 0x39600290
	.long 0x134A40C3
	.long 0x39200280
	.long 0x136A38C3
	.long 0x39000270
	.long 0x138A28C3
	.long 0x38E00260
	.long 0x13AA18C3
	.long 0x38A00250
	.long 0x13CA58C3
	.long 0x38600240
	.long 0x13EA48C3
	.long 0x39600230
	.long 0x1535CCD1
	.long 0x102A40C3
	.long 0x104A38C3
	.long 0x39200220
	.long 0x106A28C3
	.long 0x39000210
	.long 0x108A18C3
	.long 0x38E00200
	.long 0x10AA58C3
	.long 0x10CA48C3
	.long 0x1528C0D1
	.long 0x110A30C3
	.long 0x38C001F0
	.long 0x1528B8D1
	.long 0x110A38C3
	.long 0x17274910
	.long 0x10EA40C3
	.long 0x112A30C3
	.long 0x1738A8D1
	.long 0x1737A0D1
	.long 0x173698D1
	.long 0x173590D1
	.long 0x173488D1
	.long 0x173380D1
	.long 0x173278D1
	.long 0x173170D1
	.long 0x173068D1
	.long 0x174CC930
	.long 0x176BD130
	.long 0x178AD930
	.long 0x17A9E130
	.long 0x17C8E930
	.long 0x17E7F130
	.long 0x1426F930
	.long 0x14450930
	.long 0x14641130
	.long 0x14831930
	.long 0x14A22130
	.long 0x14C12930
	.long 0x38A001E0
	.long 0x386001D0
	.long 0x161BDEF3
	.long 0x396001C0
	.long 0x392001B0
	.long 0x390001A0
	.long 0x38E00190
	.long 0x38C00180
	.long 0x3BE00170
	.long 0x122A48C3
	.long 0x39200160
	.long 0x124A40C3
	.long 0x39000150
	.long 0x126A38C3
	.long 0x38E00140
	.long 0x128A30C3
	.long 0x38C00130
	.long 0x14E03130
	.long 0x12AAF8C3
	.long 0x12CA48C3
	.long 0x3BE00120
	.long 0x12EA40C3
	.long 0x39200110
	.long 0x130A38C3
	.long 0x39000100
	.long 0x132A30C3
	.long 0x38E000F0
	.long 0x38C000E0
	.long 0x134AF8C3
	.long 0x3BE000D0
	.long 0x136A48C3
	.long 0x392000C0
	.long 0x138A40C3
	.long 0x390000B0
	.long 0x13AA38C3
	.long 0x38E000A0
	.long 0x13CA30C3
	.long 0x38C00090
	.long 0x13EAF8C3
	.long 0x3BE00080
	.long 0x151F3D30
	.long 0x14FDEEF3
	.long 0x102A48C3
	.long 0x39200060
	.long 0x104A40C3
	.long 0x39000050
	.long 0x106A38C3
	.long 0x38E00040
	.long 0x108A30C3
	.long 0x10AAF8C3
	.long 0x1509F0D3
	.long 0x112A28C3
	.long 0x38A00070
	.long 0x10CA28C3
	.long 0x14E94110
	.long 0x110A18C3
	.long 0x112A58C3
	.long 0x151C3D30
	.long 0x10EA48C3
	.long 0x16094110
	.long 0x110A40C3
	.long 0x112A38C3
	.long 0x163A8530
	.long 0x16598D30
	.long 0x16789530
	.long 0x16979D30
	.long 0x16B6A530
	.long 0x16D5AD30
	.long 0x16F4B530
	.long 0x1713BD30
	.long 0x1732C530
	.long 0x1751CD30
	.long 0x1770D530
	.long 0x178FDD30
	.long 0x101DE02E
	.long 0x101E036E
	.long 0x101F032E
	.long 0x100102EE
	.long 0x140270D3
	.long 0x140368D3
	.long 0x140460D3
	.long 0x158846F3
	.long 0x38C00030
	.long 0x15673EF3
	.long 0x38A00020
	.long 0x80810014
	.long 0x38600010
	.long 0x11AA30C3
	.long 0x140558D3
	.long 0x140650D3
	.long 0x100702AE
	.long 0x140848D3
	.long 0x15890110
	.long 0x100050C3
	.long 0x156D6110
	.long 0x118A28C3
	.long 0x11AA18C3
	.long 0x15865D30
	.long 0x15A56530
	.long 0x14046D30
	.long 0x100021C3
	.long 0x38210370
	.long 0x3981FF60
	bl __restvmx_85
	b __restgprlr_14
__coffstatic__M4244_389:
	.size cl_pressure, __coffsec123+2388-cl_pressure

	.section ".rodata.__vmx@428e0000428e0000428e0000428e0000","aG",@progbits,__vmx_428e0000428e0000428e0000428e0000,comdat
	.p2align 4
__coffsec124:
	.globl __vmx_428e0000428e0000428e0000428e0000
	.type __vmx_428e0000428e0000428e0000428e0000, @object
__vmx_428e0000428e0000428e0000428e0000:
	.long 0x428E0000
	.long 0x428E0000
	.long 0x428E0000
	.long 0x428E0000

	.section ".rodata.__vmx@42900000429000004290000042900000","aG",@progbits,__vmx_42900000429000004290000042900000,comdat
	.p2align 4
__coffsec125:
	.globl __vmx_42900000429000004290000042900000
	.type __vmx_42900000429000004290000042900000, @object
__vmx_42900000429000004290000042900000:
	.long 0x42900000
	.long 0x42900000
	.long 0x42900000
	.long 0x42900000

	.section ".rodata.__vmx@42920000429200004292000042920000","aG",@progbits,__vmx_42920000429200004292000042920000,comdat
	.p2align 4
__coffsec126:
	.globl __vmx_42920000429200004292000042920000
	.type __vmx_42920000429200004292000042920000, @object
__vmx_42920000429200004292000042920000:
	.long 0x42920000
	.long 0x42920000
	.long 0x42920000
	.long 0x42920000

	.section ".rodata.__vmx@42940000429400004294000042940000","aG",@progbits,__vmx_42940000429400004294000042940000,comdat
	.p2align 4
__coffsec127:
	.globl __vmx_42940000429400004294000042940000
	.type __vmx_42940000429400004294000042940000, @object
__vmx_42940000429400004294000042940000:
	.long 0x42940000
	.long 0x42940000
	.long 0x42940000
	.long 0x42940000

	.section ".rodata.__vmx@42960000429600004296000042960000","aG",@progbits,__vmx_42960000429600004296000042960000,comdat
	.p2align 4
__coffsec128:
	.globl __vmx_42960000429600004296000042960000
	.type __vmx_42960000429600004296000042960000, @object
__vmx_42960000429600004296000042960000:
	.long 0x42960000
	.long 0x42960000
	.long 0x42960000
	.long 0x42960000

	.section ".rodata.__vmx@42980000429800004298000042980000","aG",@progbits,__vmx_42980000429800004298000042980000,comdat
	.p2align 4
__coffsec129:
	.globl __vmx_42980000429800004298000042980000
	.type __vmx_42980000429800004298000042980000, @object
__vmx_42980000429800004298000042980000:
	.long 0x42980000
	.long 0x42980000
	.long 0x42980000
	.long 0x42980000

	.section ".rodata.__vmx@429a0000429a0000429a0000429a0000","aG",@progbits,__vmx_429a0000429a0000429a0000429a0000,comdat
	.p2align 4
__coffsec130:
	.globl __vmx_429a0000429a0000429a0000429a0000
	.type __vmx_429a0000429a0000429a0000429a0000, @object
__vmx_429a0000429a0000429a0000429a0000:
	.long 0x429A0000
	.long 0x429A0000
	.long 0x429A0000
	.long 0x429A0000

	.section ".rodata.__vmx@429c0000429c0000429c0000429c0000","aG",@progbits,__vmx_429c0000429c0000429c0000429c0000,comdat
	.p2align 4
__coffsec131:
	.globl __vmx_429c0000429c0000429c0000429c0000
	.type __vmx_429c0000429c0000429c0000429c0000, @object
__vmx_429c0000429c0000429c0000429c0000:
	.long 0x429C0000
	.long 0x429C0000
	.long 0x429C0000
	.long 0x429C0000

	.section ".rodata.__vmx@429e0000429e0000429e0000429e0000","aG",@progbits,__vmx_429e0000429e0000429e0000429e0000,comdat
	.p2align 4
__coffsec132:
	.globl __vmx_429e0000429e0000429e0000429e0000
	.type __vmx_429e0000429e0000429e0000429e0000, @object
__vmx_429e0000429e0000429e0000429e0000:
	.long 0x429E0000
	.long 0x429E0000
	.long 0x429E0000
	.long 0x429E0000

	.section ".rodata.__vmx@42a0000042a0000042a0000042a00000","aG",@progbits,__vmx_42a0000042a0000042a0000042a00000,comdat
	.p2align 4
__coffsec133:
	.globl __vmx_42a0000042a0000042a0000042a00000
	.type __vmx_42a0000042a0000042a0000042a00000, @object
__vmx_42a0000042a0000042a0000042a00000:
	.long 0x42A00000
	.long 0x42A00000
	.long 0x42A00000
	.long 0x42A00000

	.section ".rodata.__vmx@42a2000042a2000042a2000042a20000","aG",@progbits,__vmx_42a2000042a2000042a2000042a20000,comdat
	.p2align 4
__coffsec134:
	.globl __vmx_42a2000042a2000042a2000042a20000
	.type __vmx_42a2000042a2000042a2000042a20000, @object
__vmx_42a2000042a2000042a2000042a20000:
	.long 0x42A20000
	.long 0x42A20000
	.long 0x42A20000
	.long 0x42A20000

	.section ".rodata.__vmx@42a4000042a4000042a4000042a40000","aG",@progbits,__vmx_42a4000042a4000042a4000042a40000,comdat
	.p2align 4
__coffsec135:
	.globl __vmx_42a4000042a4000042a4000042a40000
	.type __vmx_42a4000042a4000042a4000042a40000, @object
__vmx_42a4000042a4000042a4000042a40000:
	.long 0x42A40000
	.long 0x42A40000
	.long 0x42A40000
	.long 0x42A40000

	.section ".rodata.__vmx@42a6000042a6000042a6000042a60000","aG",@progbits,__vmx_42a6000042a6000042a6000042a60000,comdat
	.p2align 4
__coffsec136:
	.globl __vmx_42a6000042a6000042a6000042a60000
	.type __vmx_42a6000042a6000042a6000042a60000, @object
__vmx_42a6000042a6000042a6000042a60000:
	.long 0x42A60000
	.long 0x42A60000
	.long 0x42A60000
	.long 0x42A60000

	.section ".rodata.__vmx@42a8000042a8000042a8000042a80000","aG",@progbits,__vmx_42a8000042a8000042a8000042a80000,comdat
	.p2align 4
__coffsec137:
	.globl __vmx_42a8000042a8000042a8000042a80000
	.type __vmx_42a8000042a8000042a8000042a80000, @object
__vmx_42a8000042a8000042a8000042a80000:
	.long 0x42A80000
	.long 0x42A80000
	.long 0x42A80000
	.long 0x42A80000

	.section ".rodata.__vmx@42aa000042aa000042aa000042aa0000","aG",@progbits,__vmx_42aa000042aa000042aa000042aa0000,comdat
	.p2align 4
__coffsec138:
	.globl __vmx_42aa000042aa000042aa000042aa0000
	.type __vmx_42aa000042aa000042aa000042aa0000, @object
__vmx_42aa000042aa000042aa000042aa0000:
	.long 0x42AA0000
	.long 0x42AA0000
	.long 0x42AA0000
	.long 0x42AA0000

	.section ".rodata.__vmx@42ac000042ac000042ac000042ac0000","aG",@progbits,__vmx_42ac000042ac000042ac000042ac0000,comdat
	.p2align 4
__coffsec139:
	.globl __vmx_42ac000042ac000042ac000042ac0000
	.type __vmx_42ac000042ac000042ac000042ac0000, @object
__vmx_42ac000042ac000042ac000042ac0000:
	.long 0x42AC0000
	.long 0x42AC0000
	.long 0x42AC0000
	.long 0x42AC0000

	.section ".rodata.__vmx@42ae000042ae000042ae000042ae0000","aG",@progbits,__vmx_42ae000042ae000042ae000042ae0000,comdat
	.p2align 4
__coffsec140:
	.globl __vmx_42ae000042ae000042ae000042ae0000
	.type __vmx_42ae000042ae000042ae000042ae0000, @object
__vmx_42ae000042ae000042ae000042ae0000:
	.long 0x42AE0000
	.long 0x42AE0000
	.long 0x42AE0000
	.long 0x42AE0000

	.section ".rodata.__vmx@42b0000042b0000042b0000042b00000","aG",@progbits,__vmx_42b0000042b0000042b0000042b00000,comdat
	.p2align 4
__coffsec141:
	.globl __vmx_42b0000042b0000042b0000042b00000
	.type __vmx_42b0000042b0000042b0000042b00000, @object
__vmx_42b0000042b0000042b0000042b00000:
	.long 0x42B00000
	.long 0x42B00000
	.long 0x42B00000
	.long 0x42B00000

	.section ".rodata.__vmx@42b2000042b2000042b2000042b20000","aG",@progbits,__vmx_42b2000042b2000042b2000042b20000,comdat
	.p2align 4
__coffsec142:
	.globl __vmx_42b2000042b2000042b2000042b20000
	.type __vmx_42b2000042b2000042b2000042b20000, @object
__vmx_42b2000042b2000042b2000042b20000:
	.long 0x42B20000
	.long 0x42B20000
	.long 0x42B20000
	.long 0x42B20000

	.section ".rodata.__vmx@42b4000042b4000042b4000042b40000","aG",@progbits,__vmx_42b4000042b4000042b4000042b40000,comdat
	.p2align 4
__coffsec143:
	.globl __vmx_42b4000042b4000042b4000042b40000
	.type __vmx_42b4000042b4000042b4000042b40000, @object
__vmx_42b4000042b4000042b4000042b40000:
	.long 0x42B40000
	.long 0x42B40000
	.long 0x42B40000
	.long 0x42B40000

	.section ".rodata.__vmx@42b6000042b6000042b6000042b60000","aG",@progbits,__vmx_42b6000042b6000042b6000042b60000,comdat
	.p2align 4
__coffsec144:
	.globl __vmx_42b6000042b6000042b6000042b60000
	.type __vmx_42b6000042b6000042b6000042b60000, @object
__vmx_42b6000042b6000042b6000042b60000:
	.long 0x42B60000
	.long 0x42B60000
	.long 0x42B60000
	.long 0x42B60000

	.section ".rodata.__vmx@42b8000042b8000042b8000042b80000","aG",@progbits,__vmx_42b8000042b8000042b8000042b80000,comdat
	.p2align 4
__coffsec145:
	.globl __vmx_42b8000042b8000042b8000042b80000
	.type __vmx_42b8000042b8000042b8000042b80000, @object
__vmx_42b8000042b8000042b8000042b80000:
	.long 0x42B80000
	.long 0x42B80000
	.long 0x42B80000
	.long 0x42B80000

	.section ".rodata.__vmx@42ba000042ba000042ba000042ba0000","aG",@progbits,__vmx_42ba000042ba000042ba000042ba0000,comdat
	.p2align 4
__coffsec146:
	.globl __vmx_42ba000042ba000042ba000042ba0000
	.type __vmx_42ba000042ba000042ba000042ba0000, @object
__vmx_42ba000042ba000042ba000042ba0000:
	.long 0x42BA0000
	.long 0x42BA0000
	.long 0x42BA0000
	.long 0x42BA0000

	.section ".rodata.__vmx@42bc000042bc000042bc000042bc0000","aG",@progbits,__vmx_42bc000042bc000042bc000042bc0000,comdat
	.p2align 4
__coffsec147:
	.globl __vmx_42bc000042bc000042bc000042bc0000
	.type __vmx_42bc000042bc000042bc000042bc0000, @object
__vmx_42bc000042bc000042bc000042bc0000:
	.long 0x42BC0000
	.long 0x42BC0000
	.long 0x42BC0000
	.long 0x42BC0000

	.section ".rodata.__vmx@42be000042be000042be000042be0000","aG",@progbits,__vmx_42be000042be000042be000042be0000,comdat
	.p2align 4
__coffsec148:
	.globl __vmx_42be000042be000042be000042be0000
	.type __vmx_42be000042be000042be000042be0000, @object
__vmx_42be000042be000042be000042be0000:
	.long 0x42BE0000
	.long 0x42BE0000
	.long 0x42BE0000
	.long 0x42BE0000

	.section ".rodata.__vmx@42c0000042c0000042c0000042c00000","aG",@progbits,__vmx_42c0000042c0000042c0000042c00000,comdat
	.p2align 4
__coffsec149:
	.globl __vmx_42c0000042c0000042c0000042c00000
	.type __vmx_42c0000042c0000042c0000042c00000, @object
__vmx_42c0000042c0000042c0000042c00000:
	.long 0x42C00000
	.long 0x42C00000
	.long 0x42C00000
	.long 0x42C00000
	.globl __restgprlr_14
	.globl __restgprlr_17
	.globl __restvmx_64
	.globl __restvmx_85
	.globl __savegprlr_14
	.globl __savegprlr_17
	.globl __savevmx_64
	.globl __savevmx_85
