/* Kernels for comparing VMX128, classic AltiVec and scalar code generated
 * by clang. Each operation has a scalar source form (`_s`, left to the
 * auto-vectorizer) and a vector source form (`_v`, clang vector
 * extensions, which compile to scalar code without AltiVec). `_x` forms use
 * VMX128 builtins and exist only when __VMX128__ is defined.
 *
 * Conventions as in kernels.c: `int n` element counts, restrict pointers.
 * Vector forms need 16-byte aligned pointers and n a multiple of 16.
 * An element is one f32, one u32 or one byte channel.
 */

#define R __restrict

typedef float v4f __attribute__((vector_size(16)));
typedef unsigned int v4u __attribute__((vector_size(16)));
typedef int v4i __attribute__((vector_size(16)));
typedef unsigned char v16u8 __attribute__((vector_size(16)));

#define SPLAT(v, i) __builtin_shufflevector((v), (v), i, i, i, i)


void sp_saxpy_s(float *R y, const float *R x, float a, int n)
{
    for (int i = 0; i < n; i++)
        y[i] = a * x[i] + y[i];
}

void sp_saxpy_v(v4f *R y, const v4f *R x, float a, int n)
{
    v4f av = {a, a, a, a};
    for (int i = 0; i < n / 4; i++)
        y[i] = av * x[i] + y[i];
}


float sp_dot_s(const float *R a, const float *R b, int n)
{
    float s = 0.0f;
    for (int i = 0; i < n; i++)
        s += a[i] * b[i];
    return s;
}

float sp_dot_v(const v4f *R a, const v4f *R b, int n)
{
    v4f s0 = {0}, s1 = {0}, s2 = {0}, s3 = {0};
    for (int i = 0; i < n / 4; i += 4) {
        s0 += a[i] * b[i];
        s1 += a[i + 1] * b[i + 1];
        s2 += a[i + 2] * b[i + 2];
        s3 += a[i + 3] * b[i + 3];
    }
    v4f s = (s0 + s1) + (s2 + s3);
    return (s[0] + s[1]) + (s[2] + s[3]);
}

/* Column-major matrix: m[4k..4k+3] is column k. */

void sp_mat4_s(float *R out, const float *R m, const float *R in, int n)
{
    for (int i = 0; i < n; i++) {
        float x = in[4 * i], y = in[4 * i + 1], z = in[4 * i + 2], w = in[4 * i + 3];
        out[4 * i] = m[0] * x + m[4] * y + m[8] * z + m[12] * w;
        out[4 * i + 1] = m[1] * x + m[5] * y + m[9] * z + m[13] * w;
        out[4 * i + 2] = m[2] * x + m[6] * y + m[10] * z + m[14] * w;
        out[4 * i + 3] = m[3] * x + m[7] * y + m[11] * z + m[15] * w;
    }
}

static inline v4f mat4_one(v4f c0, v4f c1, v4f c2, v4f c3, v4f v)
{
    return c0 * SPLAT(v, 0) + c1 * SPLAT(v, 1) + c2 * SPLAT(v, 2) + c3 * SPLAT(v, 3);
}

void sp_mat4_v(v4f *R out, const v4f *R m, const v4f *R in, int n)
{
    v4f c0 = m[0], c1 = m[1], c2 = m[2], c3 = m[3];
    for (int i = 0; i < n; i++)
        out[i] = mat4_one(c0, c1, c2, c3, in[i]);
}

/* 8 and 16 vectors per iteration, every splat computed before the
 * multiplies, so that the source asks for 6 live registers per vector. */
#define MAT4_BLOCK(U)                                                        \
    void sp_mat4_v##U(v4f *R out, const v4f *R m, const v4f *R in, int n)    \
    {                                                                        \
        v4f c0 = m[0], c1 = m[1], c2 = m[2], c3 = m[3];                      \
        for (int i = 0; i < n; i += U) {                                     \
            v4f sx[U], sy[U], sz[U], sw[U];                                  \
            for (int j = 0; j < U; j++) {                                    \
                v4f v = in[i + j];                                           \
                sx[j] = SPLAT(v, 0);                                         \
                sy[j] = SPLAT(v, 1);                                         \
                sz[j] = SPLAT(v, 2);                                         \
                sw[j] = SPLAT(v, 3);                                         \
            }                                                                \
            for (int j = 0; j < U; j++)                                      \
                out[i + j] = c0 * sx[j] + c1 * sy[j] + c2 * sz[j] + c3 * sw[j]; \
        }                                                                    \
    }
MAT4_BLOCK(8)
MAT4_BLOCK(16)

#ifdef __VMX128__
/* Dot-product form: rows from a transpose, vmsum4fp128 per output word,
 * vrlimi128 to collect the words. vmsum4fp128 is not exact for mixed
 * signs, so the results differ from the other forms in the last bits. */
void sp_mat4_x(v4f *R out, const v4f *R m, const v4f *R in, int n)
{
    v4f t0 = __builtin_shufflevector(m[0], m[2], 0, 4, 1, 5);
    v4f t1 = __builtin_shufflevector(m[1], m[3], 0, 4, 1, 5);
    v4f t2 = __builtin_shufflevector(m[0], m[2], 2, 6, 3, 7);
    v4f t3 = __builtin_shufflevector(m[1], m[3], 2, 6, 3, 7);
    v4f r0 = __builtin_shufflevector(t0, t1, 0, 4, 1, 5);
    v4f r1 = __builtin_shufflevector(t0, t1, 2, 6, 3, 7);
    v4f r2 = __builtin_shufflevector(t2, t3, 0, 4, 1, 5);
    v4f r3 = __builtin_shufflevector(t2, t3, 2, 6, 3, 7);
    for (int i = 0; i < n; i++) {
        v4f v = in[i];
        v4f o = __builtin_vmx128_vmsum4fp(r0, v);
        o = __builtin_vmx128_vrlimi(o, __builtin_vmx128_vmsum4fp(r1, v), 4, 0);
        o = __builtin_vmx128_vrlimi(o, __builtin_vmx128_vmsum4fp(r2, v), 2, 0);
        o = __builtin_vmx128_vrlimi(o, __builtin_vmx128_vmsum4fp(r3, v), 1, 0);
        out[i] = o;
    }
}
#endif


void sp_copy_s(unsigned *R d, const unsigned *R s, int n)
{
    for (int i = 0; i < n; i++)
        d[i] = s[i];
}

void sp_copy_v(v4u *R d, const v4u *R s, int n)
{
    for (int i = 0; i < n / 4; i++)
        d[i] = s[i];
}


void sp_bright_s(unsigned char *R d, const unsigned char *R s, int n)
{
    for (int i = 0; i < n; i++) {
        int v = s[i] + 40;
        d[i] = (unsigned char)(v > 255 ? 255 : v);
    }
}

void sp_bright_v(v16u8 *R d, const v16u8 *R s, int n)
{
    v16u8 k = {40, 40, 40, 40, 40, 40, 40, 40, 40, 40, 40, 40, 40, 40, 40, 40};
    for (int i = 0; i < n / 16; i++)
        d[i] = __builtin_elementwise_add_sat(s[i], k);
}


void sp_u8f_s(float *R d, const unsigned char *R s, int n)
{
    for (int i = 0; i < n; i++)
        d[i] = (float)s[i] * (1.0f / 255.0f);
}

void sp_u8f_v(v4f *R d, const v16u8 *R s, int n)
{
    const v4f k = {1.0f / 255.0f, 1.0f / 255.0f, 1.0f / 255.0f, 1.0f / 255.0f};
    const v16u8 z = {0};
    for (int i = 0; i < n / 16; i++) {
        v16u8 b = s[i];
        /* big-endian: a zero-extended u32 is three zero bytes then the byte */
        v4u w0 = (v4u)__builtin_shufflevector(z, b, 0, 1, 2, 16, 0, 1, 2, 17, 0, 1, 2, 18, 0, 1, 2, 19);
        v4u w1 = (v4u)__builtin_shufflevector(z, b, 0, 1, 2, 20, 0, 1, 2, 21, 0, 1, 2, 22, 0, 1, 2, 23);
        v4u w2 = (v4u)__builtin_shufflevector(z, b, 0, 1, 2, 24, 0, 1, 2, 25, 0, 1, 2, 26, 0, 1, 2, 27);
        v4u w3 = (v4u)__builtin_shufflevector(z, b, 0, 1, 2, 28, 0, 1, 2, 29, 0, 1, 2, 30, 0, 1, 2, 31);
        d[4 * i] = __builtin_convertvector(w0, v4f) * k;
        d[4 * i + 1] = __builtin_convertvector(w1, v4f) * k;
        d[4 * i + 2] = __builtin_convertvector(w2, v4f) * k;
        d[4 * i + 3] = __builtin_convertvector(w3, v4f) * k;
    }
}


void sp_f2u8_s(unsigned char *R d, const float *R s, int n)
{
    for (int i = 0; i < n; i++) {
        float v = s[i] * 255.0f + 0.5f;
        v = v < 0.0f ? 0.0f : v;
        v = v > 255.0f ? 255.0f : v;
        d[i] = (unsigned char)(unsigned)v;
    }
}

void sp_f2u8_v(v16u8 *R d, const v4f *R s, int n)
{
    const v4f k = {255.0f, 255.0f, 255.0f, 255.0f};
    const v4f h = {0.5f, 0.5f, 0.5f, 0.5f};
    const v4f lo = {0.0f, 0.0f, 0.0f, 0.0f};
    for (int i = 0; i < n / 16; i++) {
        v16u8 b[4];
        for (int j = 0; j < 4; j++) {
            v4f v = s[4 * i + j] * k + h;
            v = __builtin_elementwise_minnum(__builtin_elementwise_maxnum(v, lo), k);
            b[j] = (v16u8)__builtin_convertvector(v, v4u);
        }
        v16u8 p01 = __builtin_shufflevector(b[0], b[1], 3, 7, 11, 15, 19, 23, 27, 31, 0, 0, 0, 0, 0, 0, 0, 0);
        v16u8 p23 = __builtin_shufflevector(b[2], b[3], 3, 7, 11, 15, 19, 23, 27, 31, 0, 0, 0, 0, 0, 0, 0, 0);
        d[i] = __builtin_shufflevector(p01, p23, 0, 1, 2, 3, 4, 5, 6, 7, 16, 17, 18, 19, 20, 21, 22, 23);
    }
}
