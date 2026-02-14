/* Project Euler Problem 936 - Peerless Trees
 * P(n) = number of peerless trees on n unlabelled vertices.
 * S(N) = sum P(n) for n=3..N.
 * Uses polynomial (2D generating function) approach.
 * MAX_N = 50.
 */
#include <stdio.h>
#include <string.h>
#include <stdlib.h>

typedef long long ll;

#define MAX_N 50

ll binom(int n, int k) {
    if (k < 0 || k > n) return 0;
    if (k == 0 || k == n) return 1;
    if (k > n / 2) k = n - k;
    ll res = 1;
    for (int i = 0; i < k; i++)
        res = res * (n - i) / (i + 1);
    return res;
}

/* 2D polynomial: coeffs[x][y], x=0..MAX_N, y=0..MAX_N */
typedef struct {
    ll coeffs[MAX_N + 1][MAX_N + 1];
} Poly;

void poly_init(Poly *p) {
    memset(p->coeffs, 0, sizeof(p->coeffs));
    p->coeffs[0][0] = 1;
}

void poly_copy(Poly *dst, const Poly *src) {
    memcpy(dst, src, sizeof(Poly));
}

/* Multiply by (1 - x^s * y)^(-count) */
void poly_multiply_by_inv_factor(Poly *p, int s, ll count) {
    int max_j = MAX_N / s;
    if (max_j > MAX_N) max_j = MAX_N;

    for (int x = MAX_N; x >= 0; x--) {
        for (int y = MAX_N; y >= 0; y--) {
            ll term = 0;
            for (int j = 1; j <= max_j; j++) {
                int px = x - j * s;
                int py = y - j;
                if (px < 0 || py < 0) break;
                ll c = binom((int)(count + j - 1), j);
                term += p->coeffs[px][py] * c;
            }
            p->coeffs[x][y] += term;
        }
    }
}

/* Multiply by (1 - x^s * y)^count */
void poly_multiply_by_factor(Poly *p, int s, ll count) {
    int max_j = MAX_N / s;
    if (max_j > MAX_N) max_j = MAX_N;
    if (max_j > (int)count) max_j = (int)count;

    for (int x = MAX_N; x >= 0; x--) {
        for (int y = MAX_N; y >= 0; y--) {
            ll term = 0;
            for (int j = 1; j <= max_j; j++) {
                int px = x - j * s;
                int py = y - j;
                if (px < 0 || py < 0) break;
                ll c = binom((int)count, j);
                if (j % 2 == 1)
                    term -= p->coeffs[px][py] * c;
                else
                    term += p->coeffs[px][py] * c;
            }
            p->coeffs[x][y] += term;
        }
    }
}

static ll A[MAX_N + 1][MAX_N + 1]; /* A[n][k] = rooted trees of size n, root degree k */
static Poly G_total;

int main(void) {
    memset(A, 0, sizeof(A));
    poly_init(&G_total);

    ll S_total = 0;

    for (int n = 1; n <= MAX_N; n++) {
        /* Step 1: Compute A[n][K] for all K */
        for (int K = 0; K < n; K++) {
            /* Find forbidden items: trees of size s < n with root degree K */
            int nforbid = 0;
            int forb_s[MAX_N + 1];
            ll forb_count[MAX_N + 1];

            for (int s = 1; s < n; s++) {
                if (A[s][K] > 0) {
                    forb_s[nforbid] = s;
                    forb_count[nforbid] = A[s][K];
                    nforbid++;
                }
            }

            if (nforbid == 0) {
                A[n][K] = G_total.coeffs[n - 1][K];
            } else {
                Poly temp;
                poly_copy(&temp, &G_total);
                for (int i = 0; i < nforbid; i++)
                    poly_multiply_by_factor(&temp, forb_s[i], forb_count[i]);
                A[n][K] = temp.coeffs[n - 1][K];
            }
        }

        /* Step 2: Add newly computed A[n][K] trees to G_total */
        for (int K = 0; K < n; K++) {
            if (A[n][K] > 0)
                poly_multiply_by_inv_factor(&G_total, n, A[n][K]);
        }

        /* Step 3: Compute P(n) */
        if (n < 3) continue;

        ll p_n = 0;

        /* Case 1: Single Centroid */
        int limit_s = (n - 1) / 2;
        Poly G_small;
        poly_init(&G_small);

        for (int s = 1; s <= limit_s; s++) {
            for (int k = 0; k < s; k++) {
                if (A[s][k] > 0)
                    poly_multiply_by_inv_factor(&G_small, s, A[s][k]);
            }
        }

        for (int D = 0; D < n; D++) {
            int forbidden_k = D - 1;
            int nf2 = 0;
            int f2_s[MAX_N + 1];
            ll f2_count[MAX_N + 1];

            if (forbidden_k >= 0) {
                for (int s = 1; s <= limit_s; s++) {
                    if (A[s][forbidden_k] > 0) {
                        f2_s[nf2] = s;
                        f2_count[nf2] = A[s][forbidden_k];
                        nf2++;
                    }
                }
            }

            ll val;
            if (nf2 == 0) {
                val = G_small.coeffs[n - 1][D];
            } else {
                Poly temp2;
                poly_copy(&temp2, &G_small);
                for (int i = 0; i < nf2; i++)
                    poly_multiply_by_factor(&temp2, f2_s[i], f2_count[i]);
                val = temp2.coeffs[n - 1][D];
            }
            p_n += val;
        }

        /* Case 2: Bicentroid (only if n is even) */
        if (n % 2 == 0) {
            int half = n / 2;
            for (int k1 = 0; k1 < MAX_N + 1; k1++) {
                if (A[half][k1] == 0) continue;
                for (int k2 = k1 + 1; k2 < MAX_N + 1; k2++) {
                    if (A[half][k2] == 0) continue;
                    p_n += A[half][k1] * A[half][k2];
                }
            }
        }

        S_total += p_n;
    }

    printf("%lld\n", S_total);
    return 0;
}
