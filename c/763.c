/*
 * Project Euler 763 - Amoebas in a 3D Grid
 *
 * D(N) = number of distinct arrangements after N divisions.
 * a2[m] = D(m+1). Compute a2[9999] = D(10000) mod 10^9.
 *
 * Uses a DP recurrence with arrays u[n][k][idx] and v[n][k][idx],
 * plus f0[m] and a2[m]. The active n values grow as sqrt(2*m).
 *
 * Translated from cirosantilli's Python solution.
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define M_VAL 9999
#define MOD 1000000000U

typedef unsigned int uint;

/* offset[n] = (n+1)*(n+2)/2 */
/* lens[n] = max(M_VAL - offset[n] + 1, 0) */

static int offset_arr[200];
static int lens_arr[200];
static int N_val;  /* max_n + 3 */

/* u[n] and v[n] are flat arrays of size n * lens[n] */
/* u[n][k * lens[n] + idx] for k=0..n-1, idx=0..lens[n]-1 */
static uint *u_arr[200];
static uint *v_arr[200];

static uint f0[M_VAL + 1];
static uint a2[M_VAL + 1];

int main(void) {
    /* Determine max_n: (n+1)*(n+2)/2 <= M_VAL */
    int n_tmp = 0;
    while ((n_tmp + 1) * (n_tmp + 2) / 2 <= M_VAL) n_tmp++;
    int max_n = n_tmp - 1;
    N_val = max_n + 3;

    for (int n = 0; n < N_val + 2; n++) {
        offset_arr[n] = (n + 1) * (n + 2) / 2;
        int ln = M_VAL - offset_arr[n] + 1;
        lens_arr[n] = ln > 0 ? ln : 0;
    }

    /* Allocate u and v arrays */
    for (int n = 1; n < N_val + 2; n++) {
        int ln = lens_arr[n];
        if (ln > 0) {
            size_t sz = (size_t)n * ln;
            u_arr[n] = (uint *)calloc(sz, sizeof(uint));
            v_arr[n] = (uint *)calloc(sz, sizeof(uint));
        }
    }

    memset(f0, 0, sizeof(f0));
    memset(a2, 0, sizeof(a2));
    a2[0] = 1;

    int n_active = 0;

    for (int m = 0; m <= M_VAL; m++) {
        while (n_active + 1 < N_val + 1 && offset_arr[n_active + 1] <= m)
            n_active++;

        for (int n = 1; n <= n_active; n++) {
            int off = offset_arr[n];
            int ln = lens_arr[n];
            int idx_cur = m - off;

            int mp1 = m - n - 2;
            int idx1 = mp1 - off;

            int mp2 = m - n - 3;
            int idx2 = mp2 - offset_arr[n + 1];
            int lnp = lens_arr[n + 1];

            int mp3 = m - n - 1;
            int idx3 = mp3 - offset_arr[n > 0 ? n - 1 : 0];
            int lnm = n > 1 ? lens_arr[n - 1] : 0;

            if (n == 1) {
                unsigned long long val_u = 0;
                if (idx1 >= 0) {
                    val_u += 2ULL * u_arr[1][idx1] + v_arr[1][idx1];
                }
                if (idx2 >= 0 && lnp > 0) {
                    val_u += v_arr[2][idx2] + u_arr[2][lnp + idx2];
                }
                if (mp3 >= 0) {
                    val_u += f0[mp3];
                }
                u_arr[1][idx_cur] = (uint)(val_u % MOD);

                unsigned long long val_v = 0;
                if (idx1 >= 0) {
                    val_v += 2ULL * v_arr[1][idx1] + 2ULL * u_arr[1][idx1];
                }
                if (idx2 >= 0 && lnp > 0) {
                    val_v += v_arr[2][lnp + idx2] + 2ULL * u_arr[2][idx2];
                }
                if (mp3 >= 0) {
                    val_v += f0[mp3];
                }
                v_arr[1][idx_cur] = (uint)(val_v % MOD);
                continue;
            }

            uint *u_n = u_arr[n];
            uint *v_n = v_arr[n];
            uint *u_p = u_arr[n + 1];
            uint *v_p = v_arr[n + 1];
            uint *u_m_arr = (n > 1) ? u_arr[n - 1] : NULL;
            uint *v_m_arr = (n > 1) ? v_arr[n - 1] : NULL;

            uint u_n1 = (idx1 >= 0) ? u_n[idx1] : 0;
            uint v_n1 = (idx1 >= 0) ? v_n[idx1] : 0;
            uint u_p1 = (idx2 >= 0 && lnp > 0) ? u_p[idx2] : 0;
            uint v_p1 = (idx2 >= 0 && lnp > 0) ? v_p[idx2] : 0;

            int base = 0;
            int base_next = ln;
            int base_p = lnp;
            int base_m = 0;

            if (idx1 < 0) {
                /* Only n-1 term survives */
                for (int k = 1; k < n; k++) {
                    u_n[base + idx_cur] = u_m_arr[base_m + idx3];
                    v_n[base + idx_cur] = v_m_arr[base_m + idx3];
                    base = base_next;
                    base_next += ln;
                    base_m += lnm;
                }
                /* k = n */
                u_n[(n - 1) * ln + idx_cur] = u_m_arr[(n - 2) * lnm + idx3];
                v_n[(n - 1) * ln + idx_cur] = v_m_arr[(n - 2) * lnm + idx3];
                continue;
            }

            if (idx2 >= 0 && lnp > 0) {
                /* Full recurrence */
                for (int k = 1; k < n; k++) {
                    unsigned long long uval = (unsigned long long)u_n[base + idx1]
                        + v_p1
                        + u_p[base_p + idx2]
                        + u_m_arr[base_m + idx3]
                        + v_n1
                        + u_n[base_next + idx1];
                    u_n[base + idx_cur] = (uint)(uval % MOD);

                    unsigned long long vval = (unsigned long long)v_n[base + idx1]
                        + v_p[base_p + idx2]
                        + u_p1
                        + v_m_arr[base_m + idx3]
                        + v_n[base_next + idx1]
                        + u_n1;
                    v_n[base + idx_cur] = (uint)(vval % MOD);

                    base = base_next;
                    base_next += ln;
                    base_p += lnp;
                    base_m += lnm;
                }
                int base_last = (n - 1) * ln;
                unsigned long long uval = 2ULL * u_n[base_last + idx1]
                    + v_n1
                    + v_p1
                    + u_p[base_p + idx2]
                    + u_m_arr[(n - 2) * lnm + idx3];
                u_n[base_last + idx_cur] = (uint)(uval % MOD);

                unsigned long long vval = 2ULL * v_n[base_last + idx1]
                    + 2ULL * u_n1
                    + v_p[base_p + idx2]
                    + 2ULL * u_p1
                    + v_m_arr[(n - 2) * lnm + idx3];
                v_n[base_last + idx_cur] = (uint)(vval % MOD);
            } else {
                /* No n+1 contribution */
                for (int k = 1; k < n; k++) {
                    unsigned long long uval = (unsigned long long)u_n[base + idx1]
                        + u_m_arr[base_m + idx3]
                        + v_n1
                        + u_n[base_next + idx1];
                    u_n[base + idx_cur] = (uint)(uval % MOD);

                    unsigned long long vval = (unsigned long long)v_n[base + idx1]
                        + v_m_arr[base_m + idx3]
                        + v_n[base_next + idx1]
                        + u_n1;
                    v_n[base + idx_cur] = (uint)(vval % MOD);

                    base = base_next;
                    base_next += ln;
                    base_m += lnm;
                }
                int base_last = (n - 1) * ln;
                unsigned long long uval = 2ULL * u_n[base_last + idx1] + v_n1 + u_m_arr[(n - 2) * lnm + idx3];
                u_n[base_last + idx_cur] = (uint)(uval % MOD);

                unsigned long long vval = 2ULL * v_n[base_last + idx1] + 2ULL * u_n1 + v_m_arr[(n - 2) * lnm + idx3];
                v_n[base_last + idx_cur] = (uint)(vval % MOD);
            }
        }

        /* f0 and a2 */
        unsigned long long val_f = 0;
        if (m - 1 >= 0) val_f += a2[m - 1];
        if (m - 2 >= 0) val_f += 4ULL * f0[m - 2];
        int mp = m - 3;
        if (mp >= offset_arr[1] && lens_arr[1] > 0) {
            int id1 = mp - offset_arr[1];
            val_f += 2ULL * u_arr[1][id1] + v_arr[1][id1];
        }
        f0[m] = (uint)(val_f % MOD);

        if (m >= 1) {
            unsigned long long val_a = 3ULL * a2[m - 1];
            if (m - 2 >= 0) val_a += 3ULL * f0[m - 2];
            a2[m] = (uint)(val_a % MOD);
        }
    }

    /* a2[9999] = D(10000) last 9 digits */
    printf("%u\n", a2[9999]);

    /* Cleanup */
    for (int n = 1; n < N_val + 2; n++) {
        if (u_arr[n]) free(u_arr[n]);
        if (v_arr[n]) free(v_arr[n]);
    }

    return 0;
}
