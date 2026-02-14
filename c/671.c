/*
 * Project Euler 671 - Coloured Tiles II
 *
 * Matrix exponentiation for colored tiles on a loop.
 * States encode tile extension + color (reduced by symmetry to 3 colors).
 * 3 + 9*T^2 = 84 states. N=10004003002001, K=10, T=3, M=1000004321.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef long long ll;
typedef __int128 lll;

#define M_VAL 1000004321LL
#define T_VAL 3
#define K_VAL 10
/* States: 3 vertical + 3*T * 3*T = 3 + 81 = 84 */
#define NUM_STATES (3 + T_VAL * 3 * T_VAL * 3)

typedef struct {
    ll m[NUM_STATES][NUM_STATES];
} Mat;

static Mat mat_mult(const Mat *a, const Mat *b) {
    Mat result;
    memset(&result, 0, sizeof(result));
    for (int i = 0; i < NUM_STATES; i++) {
        for (int k = 0; k < NUM_STATES; k++) {
            if (a->m[i][k] == 0) continue;
            for (int j = 0; j < NUM_STATES; j++) {
                result.m[i][j] = (result.m[i][j] + (lll)a->m[i][k] * b->m[k][j]) % M_VAL;
            }
        }
    }
    return result;
}

static Mat mat_pow(const Mat *mat, ll exp) {
    Mat result;
    memset(&result, 0, sizeof(result));
    for (int i = 0; i < NUM_STATES; i++) result.m[i][i] = 1;

    Mat base = *mat;
    while (exp > 0) {
        if (exp & 1) result = mat_mult(&result, &base);
        base = mat_mult(&base, &base);
        exp >>= 1;
    }
    return result;
}

static ll mod_inv(ll a, ll m) {
    ll t = 0, new_t = 1, r = m, new_r = a % m;
    if (new_r < 0) new_r += m;
    while (new_r != 0) {
        ll q = r / new_r;
        ll tmp;
        tmp = new_t; new_t = t - q * new_t; t = tmp;
        tmp = new_r; new_r = r - q * new_r; r = tmp;
    }
    if (t < 0) t += m;
    return t;
}

/* State encoding:
 * Vertical states: index = color (0, 1, 2)
 * Horizontal states: index = 3 + (top * 3 + top_color) * (T*3) + bottom * 3 + bottom_color
 *   where top, bottom in [0, T), top_color, bottom_color in [0, 3)
 */
static int vert_idx(int color) { return color; }
static int horiz_idx(int top, int top_color, int bottom, int bottom_color) {
    return 3 + (top * 3 + top_color) * (T_VAL * 3) + bottom * 3 + bottom_color;
}

static inline int min_c(int c) { return c < 2 ? c : 2; }

int main() {
    ll N = 10004003002001LL;

    Mat *A = (Mat *)calloc(1, sizeof(Mat));

    /* Transitions from vertical states */
    for (int color1 = 0; color1 < 3; color1++) {
        for (int color2 = 0; color2 < K_VAL; color2++) {
            if (color1 != color2) {
                int idx = vert_idx(min_c(color2));
                A->m[idx][vert_idx(color1)] = (A->m[idx][vert_idx(color1)] + 1) % M_VAL;
            }
        }
    }

    /* Transitions from vertical to horizontal and within */
    for (int color1 = 0; color1 < 3; color1++) {
        for (int color2 = 0; color2 < 3; color2++) {
            for (int color3 = 0; color3 < K_VAL; color3++) {
                if ((color1 != color2 || color1 == 2) && color1 != color3 && color2 != color3) {
                    if (color1 == 2 && color2 == 2 && color3 == 3) continue;

                    int mc3 = min_c(color3);
                    int mc2 = min_c(color2);

                    /* vert -> horiz(0, color1, 0, mc2) via color3 */
                    A->m[vert_idx(mc3)][horiz_idx(0, color1, 0, mc2)] =
                        (A->m[vert_idx(mc3)][horiz_idx(0, color1, 0, mc2)] + 1) % M_VAL;

                    for (int i = 1; i < T_VAL; i++) {
                        for (int j = 0; j < T_VAL; j++) {
                            A->m[horiz_idx(i - 1, color1, j, mc3)][horiz_idx(i, color1, 0, mc2)] =
                                (A->m[horiz_idx(i - 1, color1, j, mc3)][horiz_idx(i, color1, 0, mc2)] + 1) % M_VAL;
                        }
                    }

                    for (int i = 0; i < T_VAL; i++) {
                        for (int j = 1; j < T_VAL; j++) {
                            A->m[horiz_idx(i, mc3, j - 1, color1)][horiz_idx(0, mc2, j, color1)] =
                                (A->m[horiz_idx(i, mc3, j - 1, color1)][horiz_idx(0, mc2, j, color1)] + 1) % M_VAL;
                        }
                    }
                }
            }
        }
    }

    /* Transitions from horizontal to vertical */
    for (int color1 = 0; color1 < 3; color1++) {
        for (int color2 = 0; color2 < K_VAL; color2++) {
            for (int color3 = 0; color3 < K_VAL; color3++) {
                int s[3] = {color1, color2, color3};
                /* Check all three are distinct */
                if (s[0] == s[1] || s[0] == s[2] || s[1] == s[2]) continue;
                for (int i = 0; i < T_VAL; i++) {
                    for (int j = 0; j < T_VAL; j++) {
                        A->m[horiz_idx(i, min_c(color2), j, min_c(color3))][vert_idx(color1)] =
                            (A->m[horiz_idx(i, min_c(color2), j, min_c(color3))][vert_idx(color1)] + 1) % M_VAL;
                    }
                }
            }
        }
    }

    /* Transitions within horizontal states */
    for (int color1 = 0; color1 < 3; color1++) {
        for (int color2 = 0; color2 < 3; color2++) {
            for (int i = 1; i < T_VAL; i++) {
                for (int j = 1; j < T_VAL; j++) {
                    A->m[horiz_idx(i - 1, color1, j - 1, min_c(color2))][horiz_idx(i, color1, j, min_c(color2))] =
                        (A->m[horiz_idx(i - 1, color1, j - 1, min_c(color2))][horiz_idx(i, color1, j, min_c(color2))] + 1) % M_VAL;
                }
            }
        }
    }

    /* Compute A^N */
    Mat Ae = mat_pow(A, N);

    ll ans = 0;
    ans = (ans + (ll)K_VAL * Ae.m[vert_idx(0)][vert_idx(0)]) % M_VAL;
    for (int i = 0; i < T_VAL; i++) {
        for (int j = 0; j < T_VAL; j++) {
            ans = (ans + (ll)K_VAL * (K_VAL - 1) % M_VAL * Ae.m[horiz_idx(i, 0, j, 1)][horiz_idx(i, 0, j, 1)]) % M_VAL;
        }
    }

    ans = (lll)ans * mod_inv(N, M_VAL) % M_VAL;
    printf("%lld\n", ans);

    free(A);
    return 0;
}
