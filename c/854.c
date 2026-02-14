/*
 * Project Euler 854 - Pisano Periods 2
 *
 * Compute product of Pisano period related values modulo 1234567891.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

#define MOD 1234567891LL
#define LIMIT 1000000
#define SPF_LIMIT 2000100

static int spf[SPF_LIMIT + 1];
static long long fib_mod[LIMIT + 1]; /* F(n) mod MOD */
static long long G[LIMIT + 1];
static long long M_arr[LIMIT + 1];
static long long L_arr[LIMIT + 1]; /* LCM tracker (actual value) */

/* z_vals: rank of apparition for small primes */
static int z_vals[LIMIT + 1]; /* indexed by prime value. Only set for primes. */

static void build_spf(void) {
    for (int i = 0; i <= SPF_LIMIT; i++) spf[i] = i;
    for (int i = 2; (long long)i * i <= SPF_LIMIT; i++) {
        if (spf[i] == i) {
            for (int j = i * i; j <= SPF_LIMIT; j += i) {
                if (spf[j] == j) spf[j] = i;
            }
        }
    }
}

static long long mod_pow(long long base, long long exp, long long mod) {
    long long result = 1;
    base %= mod;
    if (base < 0) base += mod;
    while (exp > 0) {
        if (exp & 1) result = result * base % mod;
        base = base * base % mod;
        exp >>= 1;
    }
    return result;
}

static long long mod_inv(long long a, long long m) {
    return mod_pow(a, m - 2, m);
}

/* Matrix mul 2x2 mod m */
typedef struct { long long a, b, c, d; } mat2;

static mat2 mat_mul(mat2 A, mat2 B, long long m) {
    mat2 R;
    R.a = (A.a * B.a + A.b * B.c) % m;
    R.b = (A.a * B.b + A.b * B.d) % m;
    R.c = (A.c * B.a + A.d * B.c) % m;
    R.d = (A.c * B.b + A.d * B.d) % m;
    return R;
}

static mat2 mat_pow(mat2 A, long long p, long long m) {
    mat2 res = {1, 0, 0, 1};
    while (p > 0) {
        if (p & 1) res = mat_mul(res, A, m);
        A = mat_mul(A, A, m);
        p >>= 1;
    }
    return res;
}

static long long fib_n_mod_m(long long n, long long m) {
    if (n == 0) return 0;
    if (n == 1) return 1 % m;
    if (m == 1) return 0;
    mat2 M = {1, 1, 1, 0};
    mat2 R = mat_pow(M, n - 1, m);
    return R.a;
}

/* Get divisors from factorization */
static int divs_buf[100000];
static int num_divs;

static void get_divisors(long long D) {
    /* Factor D using spf, then enumerate divisors */
    typedef struct { int p; int e; } pe_t;
    pe_t factors[30];
    int nf = 0;
    long long temp = D;

    while (temp > 1 && temp <= SPF_LIMIT) {
        int p = spf[(int)temp];
        int cnt = 0;
        while (temp % p == 0) { temp /= p; cnt++; }
        factors[nf].p = p; factors[nf].e = cnt; nf++;
    }
    if (temp > 1) {
        /* temp is a large prime */
        factors[nf].p = (int)temp; factors[nf].e = 1; nf++;
    }

    num_divs = 0;
    divs_buf[num_divs++] = 1;
    for (int i = 0; i < nf; i++) {
        int prev_count = num_divs;
        long long pk = 1;
        for (int j = 0; j < factors[i].e; j++) {
            pk *= factors[i].p;
            for (int k = 0; k < prev_count; k++) {
                long long d = (long long)divs_buf[k] * pk;
                if (d <= 2000000000LL)
                    divs_buf[num_divs++] = (int)d;
            }
        }
    }

    /* Sort */
    for (int i = 0; i < num_divs - 1; i++)
        for (int j = i + 1; j < num_divs; j++)
            if (divs_buf[i] > divs_buf[j]) {
                int t = divs_buf[i]; divs_buf[i] = divs_buf[j]; divs_buf[j] = t;
            }
}

static int get_z_rank(int p) {
    if (p == 2) return 3;
    if (p == 5) return 5;

    long long D;
    int rem = p % 5;
    if (rem == 1 || rem == 4)
        D = p - 1;
    else
        D = 2 * ((long long)p + 1);

    get_divisors(D);

    for (int i = 0; i < num_divs; i++) {
        if (fib_n_mod_m(divs_buf[i], p) == 0)
            return divs_buf[i];
    }
    return (int)D;
}

static int get_ratio(int m) {
    if (m == 3) return 1;
    if (m % 2 != 0) return 4;
    if (m % 4 == 2) return 1;
    if (m % 4 == 0) return 2;
    return 0;
}

static int get_v2(int n) {
    int c = 0;
    while (n > 0 && n % 2 == 0) { c++; n /= 2; }
    return c;
}

static int get_vp(int n, int p) {
    int c = 0;
    while (n > 0 && n % p == 0) { c++; n /= p; }
    return c;
}

static int get_v2_F(int m) {
    if (m % 3 != 0) return 0;
    if (m % 6 != 0) return 1;
    return get_v2(m) + 2;
}

static int get_vp_F(int m, int z, int p) {
    return 1 + get_vp(m / z, p);
}

static long long gcd_ll(long long a, long long b) {
    while (b) { long long t = b; b = a % b; a = t; }
    return a;
}

/* Updates by period: sparse map */
#define UBP_SIZE (1 << 20)
#define UBP_MASK (UBP_SIZE - 1)
typedef struct ubp_entry { int period; long long val; struct ubp_entry *next; } ubp_entry;
static ubp_entry *ubp_table[UBP_SIZE];
static ubp_entry ubp_pool[4000000];
static int ubp_pool_idx = 0;

static long long ubp_get(int period) {
    unsigned int h = (unsigned int)period & UBP_MASK;
    for (ubp_entry *e = ubp_table[h]; e; e = e->next)
        if (e->period == period) return e->val;
    return 1;
}

static void ubp_mul(int period, long long val) {
    unsigned int h = (unsigned int)period & UBP_MASK;
    for (ubp_entry *e = ubp_table[h]; e; e = e->next) {
        if (e->period == period) {
            e->val = e->val * val % MOD;
            return;
        }
    }
    ubp_entry *e = &ubp_pool[ubp_pool_idx++];
    e->period = period;
    e->val = val;
    e->next = ubp_table[h];
    ubp_table[h] = e;
}

/* Collect all used periods for sorting */
static int used_periods[2000000];
static int num_used_periods = 0;

static void collect_periods(void) {
    num_used_periods = 0;
    for (int i = 0; i < UBP_SIZE; i++)
        for (ubp_entry *e = ubp_table[i]; e; e = e->next)
            used_periods[num_used_periods++] = e->period;
    /* Sort */
    /* Simple qsort */
    int cmp(const void *a, const void *b) {
        return *(const int *)a - *(const int *)b;
    }
    qsort(used_periods, num_used_periods, sizeof(int), cmp);
}

int main(void) {
    build_spf();

    /* Identify small primes */
    static int small_primes[80000];
    int nsp = 0;
    for (int i = 2; i <= LIMIT; i++)
        if (spf[i] == i)
            small_primes[nsp++] = i;

    /* Compute z_vals */
    for (int i = 0; i < nsp; i++)
        z_vals[small_primes[i]] = get_z_rank(small_primes[i]);

    /* Generate prime power updates */
    memset(ubp_table, 0, sizeof(ubp_table));

    for (int i = 0; i < nsp; i++) {
        int p = small_primes[i];
        if (p == 2) {
            int period = 3;
            while (period <= LIMIT) {
                ubp_mul(period, 2);
                period *= 2;
            }
        } else {
            int z = z_vals[p];
            long long curr_z = z;
            while (1) {
                int ratio = get_ratio((int)curr_z);
                long long period = curr_z * ratio;
                if (period > LIMIT) break;
                ubp_mul((int)period, p);

                curr_z *= p;
                if (curr_z > LIMIT && period * p > LIMIT) break;
            }
        }
    }

    /* Compute G_m (Fibonacci values) */
    {
        long long a = 0, b = 1;
        G[1] = 1;
        for (int i = 2; i <= LIMIT; i++) {
            long long c = (a + b) % MOD;
            a = b; b = c;
            G[i] = b;
        }
    }

    /* Sieve G_m: remove factors at multiples */
    for (int i = 1; i <= LIMIT; i++) {
        if (G[i] <= 1) continue;
        long long inv_g = mod_inv(G[i], MOD);
        for (int j = 2 * i; j <= LIMIT; j += i)
            G[j] = G[j] * inv_g % MOD;
    }

    /* Remove small prime contributions from G_m */
    static int counts_buf[LIMIT + 1];
    for (int pi = 0; pi < nsp; pi++) {
        int p = small_primes[pi];
        int z = z_vals[p];
        int max_k = LIMIT / z;
        if (max_k == 0) continue;

        for (int k = 1; k <= max_k; k++) {
            if (p == 2)
                counts_buf[k] = get_v2_F(k * z);
            else
                counts_buf[k] = get_vp_F(k * z, z, p);
        }

        /* Mobius-like sieve on counts */
        for (int k = 1; k <= max_k; k++) {
            if (counts_buf[k] == 0) continue;
            int c = counts_buf[k];
            for (int m = 2 * k; m <= max_k; m += k)
                counts_buf[m] -= c;
        }

        long long inv_p = mod_inv(p, MOD);
        for (int k = 1; k <= max_k; k++) {
            int c = counts_buf[k];
            if (c > 0) {
                int m = k * z;
                if (G[m] > 0) {
                    long long term = mod_pow(inv_p, c, MOD);
                    G[m] = G[m] * term % MOD;
                }
            }
        }
    }

    /* Add large G_m updates */
    for (int m = 1; m <= LIMIT; m++) {
        if (G[m] > 1) {
            int ratio = get_ratio(m);
            int period = m * ratio;
            if (period <= LIMIT)
                ubp_mul(period, G[m]);
        }
    }

    /* Execute updates */
    collect_periods();

    for (int i = 0; i <= LIMIT; i++) { M_arr[i] = 1; L_arr[i] = 1; }

    for (int ui = 0; ui < num_used_periods; ui++) {
        int w = used_periods[ui];
        long long val = ubp_get(w);
        if (val == 1) continue;
        for (int p = w; p <= LIMIT; p += w) {
            M_arr[p] = M_arr[p] * val % MOD;
            long long x = L_arr[p];
            if (x % w == 0) continue;
            if (w % x == 0)
                L_arr[p] = w;
            else
                L_arr[p] = x / gcd_ll(x, w) * w;
        }
    }

    /* Calculate result */
    long long total_prod = 1;
    for (int p = 1; p <= LIMIT; p++) {
        if (L_arr[p] == p)
            total_prod = total_prod * M_arr[p] % MOD;
    }

    printf("%lld\n", total_prod);
    return 0;
}
