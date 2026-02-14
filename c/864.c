/*
 * Project Euler 864 - C(n) = count of squarefree x^2+1 for 1 <= x <= n
 * Find C(123567101113).
 *
 * sum_{d sqfree, primes 1mod4} mu(d) * #{x<=N : d^2 | x^2+1}
 * Part A: DFS over d <= D with CRT solutions.
 * Part B: For d > D, iterate k such that x^2+1 = k*d^2, solve Pell x^2 - k*y^2 = -1.
 * With D = 10^8, K_LIMIT is small (~15267).
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

#define N_VAL 123567101113LL
#define D_LIM 100000000LL  /* Threshold for Part A = 10^8 */
#define SIEVE_LIM 100000001

/* Packed bit sieve for odd numbers: bit i represents 2*i+1 */
static unsigned char *sieve_bits;
static int *primes_1mod4;
static int np1m4;
static int *all_primes;
static int nall_primes;

static void do_sieve(void) {
    long long half = SIEVE_LIM / 2 + 1;
    sieve_bits = calloc(half, 1); /* 0 = prime, 1 = composite for odd numbers */
    sieve_bits[0] = 1; /* 1 is not prime */

    int lim = (int)sqrt((double)SIEVE_LIM);
    for (int i = 1; 2 * i + 1 <= lim; i++) {
        if (!sieve_bits[i]) {
            int p = 2 * i + 1;
            for (long long j = (long long)p * p; j <= SIEVE_LIM; j += 2 * p) {
                sieve_bits[(int)((j - 1) / 2)] = 1;
            }
        }
    }

    /* Count primes to allocate */
    int cnt = 1; /* for 2 */
    for (long long i = 1; 2 * i + 1 <= SIEVE_LIM; i++)
        if (!sieve_bits[i]) cnt++;

    all_primes = malloc(cnt * sizeof(int));
    primes_1mod4 = malloc(cnt * sizeof(int));

    nall_primes = 0;
    np1m4 = 0;
    all_primes[nall_primes++] = 2;
    for (long long i = 1; 2 * i + 1 <= SIEVE_LIM; i++) {
        if (sieve_bits[i]) continue;
        int p = (int)(2 * i + 1);
        all_primes[nall_primes++] = p;
        if (p % 4 == 1)
            primes_1mod4[np1m4++] = p;
    }
}

static inline int is_prime(int n) {
    if (n < 2) return 0;
    if (n == 2) return 1;
    if (n % 2 == 0) return 0;
    return !sieve_bits[(n - 1) / 2];
}

/* Extended GCD for modular inverse */
static long long mod_inv_gen(long long a, long long m) {
    long long m0 = m, x0 = 0, x1 = 1;
    if (m == 1) return 0;
    a = a % m;
    if (a < 0) a += m;
    while (a > 1 && m0 > 0) {
        long long q = a / m0;
        long long t = m0;
        m0 = a - q * m0;
        a = t;
        t = x0;
        x0 = x1 - q * x0;
        x1 = t;
    }
    return (x1 % m + m) % m;
}

/* sqrt(-1) mod p^2 using Hensel lift. Returns r with r < p^2/2. */
static long long mod_sqrt_neg1_p2(int p) {
    /* Find r: r^2 = -1 mod p */
    long long r = 0;
    for (int g = 2; g < p; g++) {
        long long t = 1, base = g;
        int e = (p - 1) / 4;
        while (e > 0) {
            if (e & 1) t = t * base % p;
            base = base * base % p;
            e >>= 1;
        }
        if ((t * t % p) == (long long)(p - 1)) { r = t; break; }
    }

    /* Hensel lift to mod p^2 */
    long long p2 = (long long)p * p;
    long long inv2r = mod_inv_gen(2 * r, p2);
    long long val = r * r + 1;
    /* val % p2 * inv2r can overflow long long, use __int128 */
    long long correction = (long long)((__int128)(val % p2) * inv2r % p2);
    r = ((r - correction) % p2 + p2) % p2;

    return (r < p2 - r) ? r : p2 - r;
}

/* Count x in [1,N] with x in sols (mod m_sq) */
static long long count_solutions(long long n, long long m_sq, long long *sols, int nsols) {
    long long total = 0;
    for (int i = 0; i < nsols; i++) {
        long long a = sols[i];
        if (a == 0) {
            if (n >= m_sq) total += n / m_sq;
        } else {
            if (a <= n) total += (n - a) / m_sq + 1;
        }
    }
    return total;
}

/* Part A: recursive DFS over squarefree d composed of primes_1mod4, d <= D_LIM */
static long long part_a_result;

#define MAX_SOLS 1024

static void part_a_dfs(int idx, long long d, long long *sols, int nsols, int mu) {
    /* Process current d */
    long long d_sq = d * d;
    long long cnt = count_solutions(N_VAL, d_sq, sols, nsols);
    if (cnt > 0)
        part_a_result += mu * cnt;

    /* Try adding more primes */
    for (int i = idx; i < np1m4; i++) {
        long long p = primes_1mod4[i];
        long long new_d = d * p;
        if (new_d > D_LIM) break;

        long long p2 = p * p;
        long long r = mod_sqrt_neg1_p2((int)p);
        long long roots_p[2] = {r, p2 - r};

        /* Need inverse of d_sq mod p_sq. Since d is coprime to p (squarefree, new prime),
           and d_sq < D_LIM^2 ~ 10^16, p2 < 10^16, this fits in long long with mod_inv_gen. */
        long long d_sq_mod_p2 = d_sq % p2;
        long long inv_d_sq = mod_inv_gen(d_sq_mod_p2, p2);

        long long new_sols[MAX_SOLS];
        int new_nsols = 0;

        for (int si = 0; si < nsols && new_nsols < MAX_SOLS - 2; si++) {
            long long s = sols[si];
            for (int ri = 0; ri < 2; ri++) {
                long long rp = roots_p[ri];
                long long s_mod_p2 = s % p2;
                long long diff = ((rp - s_mod_p2) % p2 + p2) % p2;
                /* diff * inv_d_sq can overflow long long, use __int128 */
                long long kv = (long long)((__int128)diff * inv_d_sq % p2);
                /* s + d_sq * kv can also overflow, use __int128 */
                __int128 x128 = (__int128)s + (__int128)d_sq * kv;
                new_sols[new_nsols++] = (long long)x128;
            }
        }

        part_a_dfs(i + 1, new_d, new_sols, new_nsols, -mu);
    }
}

/* Part B: Pell equation x^2 - k*y^2 = -1 */
static int is_squarefree_with_mu(long long n, int *mu_out) {
    int mu = 1;
    long long temp = n;
    for (int i = 0; i < nall_primes; i++) {
        long long p = all_primes[i];
        if (p * p > temp) break;
        if (temp % p == 0) {
            temp /= p;
            mu = -mu;
            if (temp % p == 0) { *mu_out = 0; return 0; }
        }
    }
    if (temp > 1) mu = -mu;
    *mu_out = mu;
    return 1;
}

static long long part_b_result;

static void solve_pell_and_count(long long k) {
    long long a0 = (long long)sqrt((double)k);
    while ((a0+1)*(a0+1) <= k) a0++;
    while (a0*a0 > k) a0--;
    if (a0 * a0 == k) return;

    long long m = 0, d = 1, a = a0;
    long long num2 = 0, num1 = 1;
    long long den2 = 1, den1 = 0;
    long long fund_x = 0, fund_y = 0;

    for (int iter = 0; iter < 10000; iter++) {
        long long num = a * num1 + num2;
        long long den = a * den1 + den2;

        __int128 val = (__int128)num * num - (__int128)k * den * den;

        if (val == -1) { fund_x = num; fund_y = den; break; }
        if (val == 1 && iter > 0) return;

        num2 = num1; num1 = num;
        den2 = den1; den1 = den;
        m = d * a - m;
        d = (k - m * m) / d;
        if (d == 0) return;
        a = (a0 + m) / d;
    }

    if (fund_x == 0) return;

    /* Generate all solutions with x <= N_VAL */
    __int128 mul_x = (__int128)2 * fund_x * fund_x + 1;
    __int128 mul_y = (__int128)2 * fund_x * fund_y;

    __int128 cx = fund_x, cy = fund_y;
    while (cx <= N_VAL) {
        long long y = (long long)cy;
        if (y > D_LIM) {
            int mu;
            if (is_squarefree_with_mu(y, &mu))
                part_b_result += mu;
        }

        __int128 nx = cx * mul_x + (__int128)cy * mul_y * k;
        __int128 ny = cx * mul_y + cy * mul_x;
        if (nx > (__int128)N_VAL) break;
        cx = nx; cy = ny;
    }
}

static void part_b_dfs(int idx, long long k, long long K_LIMIT) {
    solve_pell_and_count(k);

    for (int i = idx; i < np1m4; i++) {
        long long p = primes_1mod4[i];
        long long nk = k * p;
        if (nk > K_LIMIT) break;
        part_b_dfs(i, nk, K_LIMIT); /* Allow repeated factors */
    }
}

int main(void) {
    do_sieve();

    /* Part A */
    part_a_result = 0;
    long long init_sols[1] = {0};
    part_a_dfs(0, 1, init_sols, 1, 1);

    /* Part B */
    part_b_result = 0;
    /* K_LIMIT = (N^2+1)/D^2 + 1. With D=10^8, this is ~15267 */
    __int128 n128 = N_VAL;
    long long K_LIMIT = (long long)((n128 * n128 + 1) / ((__int128)D_LIM * D_LIM)) + 1;

    /* k=1 and k=2 */
    part_b_dfs(0, 1, K_LIMIT);
    part_b_dfs(0, 2, K_LIMIT);

    long long total = part_a_result + part_b_result;
    printf("%lld\n", total);

    free(sieve_bits);
    free(all_primes);
    free(primes_1mod4);
    return 0;
}
