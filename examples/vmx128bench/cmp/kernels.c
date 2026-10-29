/* Loop kernels for comparing SDK cl.exe and LLVM auto-vectorization.
 *
 * Conventions (shared with the console timing harness):
 *   - every kernel is an external C function, no static state except the
 *     g_* arrays used by the fixed-size kernels;
 *   - element counts are `int n` (n >= 0); pointers are 32-bit;
 *   - scalar float arguments and float results use f1 (both compilers);
 *   - integer results are int (r3);
 *   - no narrow scalar arguments;
 *   - suffix _r: all pointers are __restrict; no suffix: may alias.
 */

#ifdef _MSC_VER
#define RESTRICT __restrict
#define ALIGN16 __declspec(align(16))
#else
#define RESTRICT __restrict
#define ALIGN16 __attribute__((aligned(16)))
#endif

typedef unsigned char u8;
typedef signed char s8;
typedef unsigned short u16;
typedef short s16;
typedef unsigned int u32;


void k_saxpy(float *y, const float *x, float a, int n)
{
    int i;
    for (i = 0; i < n; i++)
        y[i] = a * x[i] + y[i];
}

void k_saxpy_r(float *RESTRICT y, const float *RESTRICT x, float a, int n)
{
    int i;
    for (i = 0; i < n; i++)
        y[i] = a * x[i] + y[i];
}

void k_scale_inplace(float *y, float a, int n)
{
    int i;
    for (i = 0; i < n; i++)
        y[i] *= a;
}

void k_add_f32_r(float *RESTRICT c, const float *RESTRICT a, const float *RESTRICT b, int n)
{
    int i;
    for (i = 0; i < n; i++)
        c[i] = a[i] + b[i];
}

void k_mul_f32_r(float *RESTRICT c, const float *RESTRICT a, const float *RESTRICT b, int n)
{
    int i;
    for (i = 0; i < n; i++)
        c[i] = a[i] * b[i];
}


float k_sum_f32(const float *x, int n)
{
    int i;
    float s = 0.0f;
    for (i = 0; i < n; i++)
        s += x[i];
    return s;
}

float k_dot_f32(const float *a, const float *b, int n)
{
    int i;
    float s = 0.0f;
    for (i = 0; i < n; i++)
        s += a[i] * b[i];
    return s;
}

int k_sum_i32(const int *x, int n)
{
    int i, s = 0;
    for (i = 0; i < n; i++)
        s += x[i];
    return s;
}

int k_dot_i16(const s16 *a, const s16 *b, int n)
{
    int i, s = 0;
    for (i = 0; i < n; i++)
        s += a[i] * b[i];
    return s;
}

float k_max_f32(const float *x, int n)
{
    int i;
    float m = x[0];
    for (i = 1; i < n; i++)
        m = x[i] > m ? x[i] : m;
    return m;
}


void k_add_i32(int *c, const int *a, const int *b, int n)
{
    int i;
    for (i = 0; i < n; i++)
        c[i] = a[i] + b[i];
}

void k_add_i32_r(int *RESTRICT c, const int *RESTRICT a, const int *RESTRICT b, int n)
{
    int i;
    for (i = 0; i < n; i++)
        c[i] = a[i] + b[i];
}

void k_add_i16_r(s16 *RESTRICT c, const s16 *RESTRICT a, const s16 *RESTRICT b, int n)
{
    int i;
    for (i = 0; i < n; i++)
        c[i] = (s16)(a[i] + b[i]);
}

void k_add_u8_r(u8 *RESTRICT c, const u8 *RESTRICT a, const u8 *RESTRICT b, int n)
{
    int i;
    for (i = 0; i < n; i++)
        c[i] = (u8)(a[i] + b[i]);
}

void k_avg_u8_r(u8 *RESTRICT c, const u8 *RESTRICT a, const u8 *RESTRICT b, int n)
{
    int i;
    for (i = 0; i < n; i++)
        c[i] = (u8)((a[i] + b[i] + 1) >> 1);
}

void k_addsat_u8_r(u8 *RESTRICT c, const u8 *RESTRICT a, const u8 *RESTRICT b, int n)
{
    int i;
    for (i = 0; i < n; i++) {
        int s = a[i] + b[i];
        c[i] = (u8)(s > 255 ? 255 : s);
    }
}

void k_and_u32_r(u32 *RESTRICT c, const u32 *RESTRICT a, const u32 *RESTRICT b, int n)
{
    int i;
    for (i = 0; i < n; i++)
        c[i] = a[i] & b[i];
}

void k_shl_i32_r(int *RESTRICT c, const int *RESTRICT a, int n)
{
    int i;
    for (i = 0; i < n; i++)
        c[i] = a[i] << 3;
}

void k_mul_i32_r(int *RESTRICT c, const int *RESTRICT a, const int *RESTRICT b, int n)
{
    int i;
    for (i = 0; i < n; i++)
        c[i] = a[i] * b[i];
}


void k_min_f32_r(float *RESTRICT c, const float *RESTRICT a, const float *RESTRICT b, int n)
{
    int i;
    for (i = 0; i < n; i++)
        c[i] = a[i] < b[i] ? a[i] : b[i];
}

void k_max_i32_r(int *RESTRICT c, const int *RESTRICT a, const int *RESTRICT b, int n)
{
    int i;
    for (i = 0; i < n; i++)
        c[i] = a[i] > b[i] ? a[i] : b[i];
}

void k_clamp_f32_r(float *RESTRICT y, const float *RESTRICT x, float lo, float hi, int n)
{
    int i;
    for (i = 0; i < n; i++) {
        float v = x[i];
        v = v < lo ? lo : v;
        v = v > hi ? hi : v;
        y[i] = v;
    }
}

void k_clamp_i16_r(s16 *RESTRICT y, const s16 *RESTRICT x, int n)
{
    int i;
    for (i = 0; i < n; i++) {
        int v = x[i];
        v = v < -1000 ? -1000 : v;
        v = v > 1000 ? 1000 : v;
        y[i] = (s16)v;
    }
}

void k_abs_f32_r(float *RESTRICT y, const float *RESTRICT x, int n)
{
    int i;
    for (i = 0; i < n; i++) {
        float v = x[i];
        y[i] = v < 0.0f ? -v : v;
    }
}

void k_abs_i32_r(int *RESTRICT y, const int *RESTRICT x, int n)
{
    int i;
    for (i = 0; i < n; i++) {
        int v = x[i];
        y[i] = v < 0 ? -v : v;
    }
}


void k_f2i_r(int *RESTRICT y, const float *RESTRICT x, int n)
{
    int i;
    for (i = 0; i < n; i++)
        y[i] = (int)x[i];
}

void k_i2f_r(float *RESTRICT y, const int *RESTRICT x, int n)
{
    int i;
    for (i = 0; i < n; i++)
        y[i] = (float)x[i];
}

void k_u8_to_f32_r(float *RESTRICT y, const u8 *RESTRICT x, int n)
{
    int i;
    for (i = 0; i < n; i++)
        y[i] = (float)x[i] * (1.0f / 255.0f);
}


typedef struct { float x, y, z, w; } vec4;

void k_mat4_vec4_r(vec4 *RESTRICT out, const float *RESTRICT m, const vec4 *RESTRICT in, int n)
{
    int i;
    for (i = 0; i < n; i++) {
        float x = in[i].x, y = in[i].y, z = in[i].z, w = in[i].w;
        out[i].x = m[0] * x + m[4] * y + m[8] * z + m[12] * w;
        out[i].y = m[1] * x + m[5] * y + m[9] * z + m[13] * w;
        out[i].z = m[2] * x + m[6] * y + m[10] * z + m[14] * w;
        out[i].w = m[3] * x + m[7] * y + m[11] * z + m[15] * w;
    }
}


void k_copy_u32_r(u32 *RESTRICT d, const u32 *RESTRICT s, int n)
{
    int i;
    for (i = 0; i < n; i++)
        d[i] = s[i];
}

void k_copy_u8(u8 *d, const u8 *s, int n)
{
    int i;
    for (i = 0; i < n; i++)
        d[i] = s[i];
}

void k_set_u32(u32 *d, u32 v, int n)
{
    int i;
    for (i = 0; i < n; i++)
        d[i] = v;
}

void k_set_f32(float *d, float v, int n)
{
    int i;
    for (i = 0; i < n; i++)
        d[i] = v;
}

void k_zero_u8(u8 *d, int n)
{
    int i;
    for (i = 0; i < n; i++)
        d[i] = 0;
}


void k_stride2_f32_r(float *RESTRICT y, const float *RESTRICT x, int n)
{
    int i;
    for (i = 0; i < n; i++)
        y[i] = x[2 * i] + x[2 * i + 1];
}

void k_gather_f32_r(float *RESTRICT y, const float *RESTRICT x, const int *RESTRICT idx, int n)
{
    int i;
    for (i = 0; i < n; i++)
        y[i] = x[idx[i]];
}

void k_offset1_f32_r(float *RESTRICT y, const float *RESTRICT x, int n)
{
    int i;
    for (i = 0; i < n; i++)
        y[i] = x[i] + x[i + 1];
}

void k_fir3_f32_r(float *RESTRICT y, const float *RESTRICT x, int n)
{
    int i;
    for (i = 0; i < n; i++)
        y[i] = 0.25f * x[i] + 0.5f * x[i + 1] + 0.25f * x[i + 2];
}

/* Loop-carried dependence: must stay scalar. */
void k_prefix_f32(float *y, int n)
{
    int i;
    for (i = 1; i < n; i++)
        y[i] += y[i - 1];
}


#define GN 1024
ALIGN16 float g_a[GN];
ALIGN16 float g_b[GN];
ALIGN16 float g_c[GN];
ALIGN16 int g_ia[GN];
ALIGN16 int g_ib[GN];
ALIGN16 int g_ic[GN];

void k_g_add_f32(void)
{
    int i;
    for (i = 0; i < GN; i++)
        g_c[i] = g_a[i] + g_b[i];
}

void k_g_saxpy(float a)
{
    int i;
    for (i = 0; i < GN; i++)
        g_c[i] = a * g_a[i] + g_b[i];
}

void k_g_add_i32(void)
{
    int i;
    for (i = 0; i < GN; i++)
        g_ic[i] = g_ia[i] + g_ib[i];
}

float k_g_dot(void)
{
    int i;
    float s = 0.0f;
    for (i = 0; i < GN; i++)
        s += g_a[i] * g_b[i];
    return s;
}

/* Fixed trip count through pointers. */
void k_add16_f32_r(float *RESTRICT c, const float *RESTRICT a, const float *RESTRICT b)
{
    int i;
    for (i = 0; i < 16; i++)
        c[i] = a[i] + b[i];
}
