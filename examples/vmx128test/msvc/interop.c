/* Functions compiled with the console compiler (cl.exe) that vmx128test
   calls, and that call back into Rust, to check the vector calling
   convention between the two compilers. Results are exact for the small
   integer-valued floats the tests pass. */

#include <vectorintrinsics.h>

typedef __vector4 V;

typedef struct { unsigned char a; } S1;
typedef struct { unsigned char a, b; } S2;
typedef struct { unsigned char a, b, c; } S3;
typedef struct { unsigned char a, b, c, d; } S4;
typedef struct { unsigned char a, b, c, d, e; } S5;
typedef struct { unsigned char a, b, c, d, e, f, g, h; } S8;

static V make(float x, float y, float z, float w)
{
    V v;
    v.vector4_f32[0] = x;
    v.vector4_f32[1] = y;
    v.vector4_f32[2] = z;
    v.vector4_f32[3] = w;
    return v;
}

/* sum of (k + 1) * a_k */
static V weigh(V acc, V a, float k)
{
    return __vmaddfp(a, make(k, k, k, k), acc);
}

/* 16 vectors: v1-v13, then three in the parameter area. */
__declspec(noinline) V cl_sum16(V a0, V a1, V a2, V a3, V a4, V a5, V a6, V a7,
    V a8, V a9, V a10, V a11, V a12, V a13, V a14, V a15)
{
    V r = make(0, 0, 0, 0);
    r = weigh(r, a0, 1); r = weigh(r, a1, 2); r = weigh(r, a2, 3); r = weigh(r, a3, 4);
    r = weigh(r, a4, 5); r = weigh(r, a5, 6); r = weigh(r, a6, 7); r = weigh(r, a7, 8);
    r = weigh(r, a8, 9); r = weigh(r, a9, 10); r = weigh(r, a10, 11); r = weigh(r, a11, 12);
    r = weigh(r, a12, 13); r = weigh(r, a13, 14); r = weigh(r, a14, 15); r = weigh(r, a15, 16);
    return r;
}

/* 13 vectors, an int, the 14th vector, an int after it. */
__declspec(noinline) V cl_after13(V a0, V a1, V a2, V a3, V a4, V a5, V a6, V a7,
    V a8, V a9, V a10, V a11, V a12, int i, V a13, int j)
{
    V r = make((float)i, (float)j, 0, 0);
    r = weigh(r, a0, 1); r = weigh(r, a1, 2); r = weigh(r, a2, 3); r = weigh(r, a3, 4);
    r = weigh(r, a4, 5); r = weigh(r, a5, 6); r = weigh(r, a6, 7); r = weigh(r, a7, 8);
    r = weigh(r, a8, 9); r = weigh(r, a9, 10); r = weigh(r, a10, 11); r = weigh(r, a11, 12);
    r = weigh(r, a12, 13); r = weigh(r, a13, 14);
    return r;
}

/* Vectors use no GPR or FPR; floats shadow GPRs. */
__declspec(noinline) V cl_mix(int i0, V a, float f0, V b, int i1, double d1, V c, int i2, float f1)
{
    V r = __vaddfp(a, __vaddfp(b, c));
    r.vector4_f32[0] += (float)i0;
    r.vector4_f32[1] += f0 + f1;
    r.vector4_f32[2] += (float)i1 + (float)i2;
    r.vector4_f32[3] += (float)d1;
    return r;
}

__declspec(noinline) V cl_make(float x, float y, float z, float w)
{
    return make(x, y, z, w);
}

__declspec(noinline) void cl_store(V a, V *out)
{
    *out = a;
}

/* Small structs as arguments: each returns its bytes with the integers
   around it, as a + b*2^8 ... plus k * 2^24 + m (bytes are small). */
__declspec(noinline) unsigned cl_s1_in(int k, S1 s, int m) { return s.a + (k << 24) + m; }
__declspec(noinline) unsigned cl_s2_in(int k, S2 s, int m) { return s.a + (s.b << 8) + (k << 24) + m; }
__declspec(noinline) unsigned cl_s3_in(int k, S3 s, int m) { return s.a + (s.b << 8) + (s.c << 16) + (k << 24) + m; }
__declspec(noinline) unsigned cl_s4_in(int k, S4 s, int m) { return s.a + (s.b << 8) + (s.c << 16) + (s.d << 24) + (k << 4) + m; }
__declspec(noinline) unsigned cl_s5_in(int k, S5 s, int m) { return s.a + (s.b << 8) + (s.c << 16) + (s.d << 24) + s.e * 3 + (k << 4) + m; }
__declspec(noinline) unsigned cl_s8_in(int k, S8 s, int m)
{
    return s.a + (s.b << 8) + (s.c << 16) + (s.d << 24) + s.e * 3 + s.f * 5 + s.g * 7 + s.h * 11 + (k << 4) + m;
}

/* Small structs as returns: byte i is seed + i. */
__declspec(noinline) S1 cl_s1_out(int seed) { S1 s; s.a = (unsigned char)seed; return s; }
__declspec(noinline) S2 cl_s2_out(int seed) { S2 s; s.a = (unsigned char)seed; s.b = (unsigned char)(seed + 1); return s; }
__declspec(noinline) S3 cl_s3_out(int seed)
{
    S3 s; s.a = (unsigned char)seed; s.b = (unsigned char)(seed + 1); s.c = (unsigned char)(seed + 2); return s;
}
__declspec(noinline) S4 cl_s4_out(int seed)
{
    S4 s; s.a = (unsigned char)seed; s.b = (unsigned char)(seed + 1); s.c = (unsigned char)(seed + 2);
    s.d = (unsigned char)(seed + 3); return s;
}
__declspec(noinline) S5 cl_s5_out(int seed)
{
    S5 s; s.a = (unsigned char)seed; s.b = (unsigned char)(seed + 1); s.c = (unsigned char)(seed + 2);
    s.d = (unsigned char)(seed + 3); s.e = (unsigned char)(seed + 4); return s;
}
__declspec(noinline) S8 cl_s8_out(int seed)
{
    S8 s; s.a = (unsigned char)seed; s.b = (unsigned char)(seed + 1); s.c = (unsigned char)(seed + 2);
    s.d = (unsigned char)(seed + 3); s.e = (unsigned char)(seed + 4); s.f = (unsigned char)(seed + 5);
    s.g = (unsigned char)(seed + 6); s.h = (unsigned char)(seed + 7); return s;
}

/* char signedness and extension of narrow arguments and returns. */
__declspec(noinline) int cl_char_value(char c) { return c; }
__declspec(noinline) int cl_schar_value(signed char c) { return c; }
__declspec(noinline) int cl_uchar_value(unsigned char c) { return c; }
__declspec(noinline) int cl_short_value(short c) { return c; }
__declspec(noinline) int cl_char_is_signed(void) { char c = (char)0x80; return c < 0; }
__declspec(noinline) char cl_char_from(int v) { return (char)v; }
__declspec(noinline) unsigned char cl_uchar_from(int v) { return (unsigned char)v; }
__declspec(noinline) short cl_short_from(int v) { return (short)v; }

/* Calls into Rust. */
typedef V (*Cb16)(V, V, V, V, V, V, V, V, V, V, V, V, V, V, V, V);
typedef V (*CbMix)(int, V, float, V, int, double, V, int, float);
typedef unsigned (*CbS3)(int, S3, int);
typedef S3 (*CbS3Out)(int);
typedef S5 (*CbS5Out)(int);
typedef unsigned (*CbS8)(int, S8, int);
typedef int (*CbChar)(char, signed char, unsigned char, short);
typedef void (*CbVoid)(void);

__declspec(noinline) V cl_call16(Cb16 f, const V *p)
{
    return f(p[0], p[1], p[2], p[3], p[4], p[5], p[6], p[7], p[8], p[9], p[10], p[11],
        p[12], p[13], p[14], p[15]);
}

__declspec(noinline) V cl_callmix(CbMix f, const V *p)
{
    return f(7, p[0], 1.5f, p[1], -3, 2.25, p[2], 11, -0.5f);
}

__declspec(noinline) unsigned cl_calls3(CbS3 f, int seed)
{
    S3 s;
    s.a = (unsigned char)seed; s.b = (unsigned char)(seed + 1); s.c = (unsigned char)(seed + 2);
    return f(5, s, 9);
}

__declspec(noinline) unsigned cl_calls3out(CbS3Out f, int seed)
{
    S3 s = f(seed);
    return s.a + (s.b << 8) + (s.c << 16);
}

__declspec(noinline) unsigned cl_calls5out(CbS5Out f, int seed)
{
    S5 s = f(seed);
    return s.a + (s.b << 8) + (s.c << 16) + (s.d << 24) + s.e * 3;
}

__declspec(noinline) unsigned cl_calls8(CbS8 f, int seed)
{
    S8 s;
    s.a = (unsigned char)seed; s.b = (unsigned char)(seed + 1); s.c = (unsigned char)(seed + 2);
    s.d = (unsigned char)(seed + 3); s.e = (unsigned char)(seed + 4); s.f = (unsigned char)(seed + 5);
    s.g = (unsigned char)(seed + 6); s.h = (unsigned char)(seed + 7);
    return f(5, s, 9);
}

__declspec(noinline) int cl_callchar(CbChar f, int v)
{
    return f((char)v, (signed char)v, (unsigned char)v, (short)(v * 300));
}

/* Keeps 70 vectors live across the call: cl.exe holds 64 of them in
   v64-v127 (saved with __savevmx_64) and spills the rest. Returns the sum
   of (k + 1) * p[k]. */
__declspec(noinline) V cl_keep_across(CbVoid f, const V *p)
{
#define L(k) V b##k = p[k];
    L(0) L(1) L(2) L(3) L(4) L(5) L(6) L(7) L(8) L(9) L(10) L(11) L(12) L(13) L(14) L(15) L(16)
    L(17) L(18) L(19) L(20) L(21) L(22) L(23) L(24) L(25) L(26) L(27) L(28) L(29) L(30) L(31) L(32)
    L(33) L(34) L(35) L(36) L(37) L(38) L(39) L(40) L(41) L(42) L(43) L(44) L(45) L(46) L(47) L(48)
    L(49) L(50) L(51) L(52) L(53) L(54) L(55) L(56) L(57) L(58) L(59) L(60) L(61) L(62) L(63) L(64)
    L(65) L(66) L(67) L(68) L(69)
#undef L
    V r = make(0, 0, 0, 0);
    f();
#define W(k) r = weigh(r, b##k, (float)(k + 1));
    W(0) W(1) W(2) W(3) W(4) W(5) W(6) W(7) W(8) W(9) W(10) W(11) W(12) W(13) W(14) W(15) W(16)
    W(17) W(18) W(19) W(20) W(21) W(22) W(23) W(24) W(25) W(26) W(27) W(28) W(29) W(30) W(31) W(32)
    W(33) W(34) W(35) W(36) W(37) W(38) W(39) W(40) W(41) W(42) W(43) W(44) W(45) W(46) W(47) W(48)
    W(49) W(50) W(51) W(52) W(53) W(54) W(55) W(56) W(57) W(58) W(59) W(60) W(61) W(62) W(63) W(64)
    W(65) W(66) W(67) W(68) W(69)
#undef W
    return r;
}

/* A leaf that keeps 96 vectors live at once, so that cl.exe uses
   v0-v127 and saves the nonvolatile ones it touches. Writes the sum of
   (k + 1) * p[k] to out. */
__declspec(noinline) void cl_pressure(const V *p, V *out)
{
#define L(k) V b##k = p[k];
    L(0) L(1) L(2) L(3) L(4) L(5) L(6) L(7) L(8) L(9) L(10) L(11) L(12) L(13) L(14) L(15)
    L(16) L(17) L(18) L(19) L(20) L(21) L(22) L(23) L(24) L(25) L(26) L(27) L(28) L(29) L(30) L(31)
    L(32) L(33) L(34) L(35) L(36) L(37) L(38) L(39) L(40) L(41) L(42) L(43) L(44) L(45) L(46) L(47)
    L(48) L(49) L(50) L(51) L(52) L(53) L(54) L(55) L(56) L(57) L(58) L(59) L(60) L(61) L(62) L(63)
    L(64) L(65) L(66) L(67) L(68) L(69) L(70) L(71) L(72) L(73) L(74) L(75) L(76) L(77) L(78) L(79)
    L(80) L(81) L(82) L(83) L(84) L(85) L(86) L(87) L(88) L(89) L(90) L(91) L(92) L(93) L(94) L(95)
#undef L
    V r = make(0, 0, 0, 0);
#define W(k) r = weigh(r, b##k, (float)(k + 1));
    W(95) W(94) W(93) W(92) W(91) W(90) W(89) W(88) W(87) W(86) W(85) W(84) W(83) W(82) W(81) W(80)
    W(79) W(78) W(77) W(76) W(75) W(74) W(73) W(72) W(71) W(70) W(69) W(68) W(67) W(66) W(65) W(64)
    W(63) W(62) W(61) W(60) W(59) W(58) W(57) W(56) W(55) W(54) W(53) W(52) W(51) W(50) W(49) W(48)
    W(47) W(46) W(45) W(44) W(43) W(42) W(41) W(40) W(39) W(38) W(37) W(36) W(35) W(34) W(33) W(32)
    W(31) W(30) W(29) W(28) W(27) W(26) W(25) W(24) W(23) W(22) W(21) W(20) W(19) W(18) W(17) W(16)
    W(15) W(14) W(13) W(12) W(11) W(10) W(9) W(8) W(7) W(6) W(5) W(4) W(3) W(2) W(1) W(0)
#undef W
    *out = r;
}
