/*
 * Project Euler 851 - R_6(10000!) mod 10^9+7
 *
 * R_k(n) relates to divisor sums. R_6 can be expressed as a linear combination
 * of tau(n) and sigma_k(n) * n^l terms (quasimodular forms).
 *
 * Algorithm:
 * 1. Compute Ramanujan tau(n) for small n via prod(1-q^n)^24
 * 2. Compute R_6(n) for small n via convolution of R_1 = 2*sigma_1
 * 3. Solve linear system to get coefficients
 * 4. Evaluate at n = 10000! using multiplicativity
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define MOD 1000000007LL
#define BIG_N 10000
#define K 6

static long long mod_pow(long long base, long long exp, long long mod) {
    long long r = 1;
    base %= mod; if (base < 0) base += mod;
    while (exp > 0) {
        if (exp & 1) r = r * base % mod;
        base = base * base % mod;
        exp >>= 1;
    }
    return r;
}

static long long mod_inv(long long a) { return mod_pow(a % MOD, MOD - 2, MOD); }

/* Sieve primes up to BIG_N */
static int primes[2000], nprimes;
static char is_prime_arr[BIG_N + 1];

static void sieve_primes(void) {
    memset(is_prime_arr, 1, sizeof(is_prime_arr));
    is_prime_arr[0] = is_prime_arr[1] = 0;
    for (int i = 2; i * i <= BIG_N; i++)
        if (is_prime_arr[i])
            for (int j = i * i; j <= BIG_N; j += i)
                is_prime_arr[j] = 0;
    nprimes = 0;
    for (int i = 2; i <= BIG_N; i++)
        if (is_prime_arr[i]) primes[nprimes++] = i;
}

/* Compute sigma_1(n) for n=1..L (exact, not modular, for small L) */
#define L_SIZE 23 /* We need 22 unknowns, so L = 22 equations from n=1..22 */

static long long sigma1[L_SIZE + 1];
static void compute_sigma1(int limit) {
    memset(sigma1, 0, sizeof(sigma1));
    for (int d = 1; d <= limit; d++)
        for (int m = d; m <= limit; m += d)
            sigma1[m] += d;
}

/* Compute sigma_k(n) mod MOD for n=1..L */
static long long sigma_k_mod[L_SIZE + 1];
static void compute_sigma_k_mod(int limit, int k) {
    memset(sigma_k_mod, 0, sizeof(sigma_k_mod));
    for (int d = 1; d <= limit; d++) {
        long long dk = mod_pow(d, k, MOD);
        for (int m = d; m <= limit; m += d)
            sigma_k_mod[m] = (sigma_k_mod[m] + dk) % MOD;
    }
}

/* Compute R_6(n) for n=1..L mod MOD via convolution */
static long long R6_vals[L_SIZE + 1];
static void compute_R6(int limit) {
    /* R_1(n) = 2 * sigma_1(n) */
    compute_sigma1(limit);

    long long R1[L_SIZE + 1];
    for (int i = 0; i <= limit; i++) R1[i] = (2 * sigma1[i]) % MOD;

    /* R = R_1 */
    long long R[L_SIZE + 1];
    memcpy(R, R1, sizeof(long long) * (limit + 1));

    /* Convolve 5 more times (total 6 convolutions of R_1) */
    for (int iter = 0; iter < 5; iter++) {
        long long new_R[L_SIZE + 1];
        memset(new_R, 0, sizeof(new_R));
        for (int i = 1; i <= limit; i++) {
            if (R[i] == 0) continue;
            for (int j = 1; j <= limit - i; j++) {
                if (R1[j])
                    new_R[i + j] = (new_R[i + j] + R[i] * R1[j]) % MOD;
            }
        }
        memcpy(R, new_R, sizeof(long long) * (limit + 1));
    }

    memcpy(R6_vals, R, sizeof(long long) * (limit + 1));
}

/* Compute tau(n) for n=1..BIG_N mod MOD */
static long long tau_arr[BIG_N + 1];

static void poly_mul_trunc(long long *a, long long *b, long long *out, int limit) {
    /* out = a * b truncated to degree limit, mod MOD */
    memset(out, 0, (limit + 1) * sizeof(long long));
    for (int i = 0; i <= limit; i++) {
        if (a[i] == 0) continue;
        long long ai = a[i];
        int jmax = limit - i;
        for (int j = 0; j <= jmax; j++) {
            if (b[j])
                out[i + j] = (out[i + j] + ai * b[j]) % MOD;
        }
    }
}

static void compute_tau(void) {
    /* Step 1: Compute a = prod_{n=1}^{N} (1 - q^n) mod MOD */
    static long long a[BIG_N + 1];
    memset(a, 0, sizeof(a));
    a[0] = 1;
    for (int n = 1; n <= BIG_N; n++) {
        for (int j = BIG_N; j >= n; j--)
            a[j] = (a[j] - a[j - n] + MOD) % MOD;
    }

    /* Step 2: Compute a^24 using binary exponentiation */
    static long long result[BIG_N + 1];
    static long long base_p[BIG_N + 1];
    static long long temp[BIG_N + 1];

    memcpy(base_p, a, sizeof(a));
    memset(result, 0, sizeof(result));
    result[0] = 1;

    int exp = 24;
    while (exp > 0) {
        if (exp & 1) {
            poly_mul_trunc(result, base_p, temp, BIG_N);
            memcpy(result, temp, sizeof(result));
        }
        if (exp > 1) {
            poly_mul_trunc(base_p, base_p, temp, BIG_N);
            memcpy(base_p, temp, sizeof(base_p));
        }
        exp >>= 1;
    }

    /* tau(n) = result[n-1] (shift by q) */
    memset(tau_arr, 0, sizeof(tau_arr));
    for (int n = 1; n <= BIG_N; n++)
        tau_arr[n] = result[n - 1];
}

/* Basis terms: (k, l) pairs for quasimodular expansion */
typedef struct { int k; int l; } basis_term;
static basis_term basis[30];
static int nbasis;

static void get_basis_terms(void) {
    nbasis = 0;
    for (int k = 1; k < 2 * K; k += 2) {
        int l = 0;
        while (k + 2 * l < 2 * K) {
            basis[nbasis].k = k;
            basis[nbasis].l = l;
            nbasis++;
            l++;
        }
    }
}

/* Solve linear system mod MOD */
/* L unknowns (nbasis + 1 for tau) */
static long long A_mat[24][24];
static long long b_vec[24];
static long long X_sol[24];

static void solve_linear_system(int n) {
    long long aug[24][25];
    for (int i = 0; i < n; i++) {
        for (int j = 0; j < n; j++) aug[i][j] = A_mat[i][j];
        aug[i][n] = b_vec[i];
    }

    for (int col = 0; col < n; col++) {
        /* Find pivot */
        int pivot = -1;
        for (int row = col; row < n; row++)
            if (aug[row][col] % MOD != 0) { pivot = row; break; }

        if (pivot != col) {
            for (int j = 0; j <= n; j++) {
                long long t = aug[col][j]; aug[col][j] = aug[pivot][j]; aug[pivot][j] = t;
            }
        }

        long long inv = mod_inv(aug[col][col]);
        for (int j = col; j <= n; j++) aug[col][j] = aug[col][j] * inv % MOD;

        for (int r = 0; r < n; r++) {
            if (r == col) continue;
            long long factor = aug[r][col] % MOD;
            if (factor == 0) continue;
            for (int j = col; j <= n; j++)
                aug[r][j] = (aug[r][j] - factor * aug[col][j] % MOD + MOD) % MOD;
        }
    }

    for (int i = 0; i < n; i++) X_sol[i] = aug[i][n] % MOD;
}

/* Factorial prime exponents */
static int fac_exp[BIG_N + 1]; /* fac_exp[p] = exponent of p in N! */

int main(void) {
    sieve_primes();
    get_basis_terms();

    int L = 1 + nbasis; /* Number of unknowns: 1 for tau + nbasis for sigma terms */

    /* Step 1: Compute tau */
    compute_tau();

    /* Step 2: Compute R_6(n) for n=1..L */
    compute_R6(L);

    /* Step 3: Compute sigma_k values for n=1..L */
    /* We need sigma_k for each unique k in basis */
    int needed_ks[20], n_needed_ks = 0;
    for (int i = 0; i < nbasis; i++) {
        int k = basis[i].k;
        int found = 0;
        for (int j = 0; j < n_needed_ks; j++)
            if (needed_ks[j] == k) { found = 1; break; }
        if (!found) needed_ks[n_needed_ks++] = k;
    }

    /* Store sigma values: sigma_k_vals[k_idx][n] */
    long long sigma_k_vals[10][L_SIZE + 1];
    for (int ki = 0; ki < n_needed_ks; ki++) {
        compute_sigma_k_mod(L, needed_ks[ki]);
        for (int n = 0; n <= L; n++) sigma_k_vals[ki][n] = sigma_k_mod[n];
    }

    /* Step 4: Build linear system */
    /* R_6(n) = X[0]*tau(n) + sum X[i+1]*sigma_{k_i}(n)*n^{l_i} */
    for (int i = 0; i < L; i++) {
        int n = i + 1;
        A_mat[i][0] = tau_arr[n];
        for (int j = 0; j < nbasis; j++) {
            int ki_idx = -1;
            for (int kk = 0; kk < n_needed_ks; kk++)
                if (needed_ks[kk] == basis[j].k) { ki_idx = kk; break; }
            A_mat[i][j + 1] = sigma_k_vals[ki_idx][n] * mod_pow(n, basis[j].l, MOD) % MOD;
        }
        b_vec[i] = R6_vals[n];
    }

    solve_linear_system(L);

    /* Step 5: Evaluate at n = 10000! */
    /* Compute factorial prime exponents */
    for (int i = 0; i < nprimes; i++) {
        int p = primes[i];
        int count = 0, m = BIG_N;
        while (m) { m /= p; count += m; }
        fac_exp[p] = count;
    }

    /* tau(N!) = prod_p tau(p^{e_p}) */
    long long big_tau = 1;
    for (int i = 0; i < nprimes; i++) {
        int p = primes[i];
        int e = fac_exp[p];
        if (e == 0) continue;

        /* Compute tau(p^e) using recurrence: tau(p^{r+1}) = tau(p)*tau(p^r) - p^11*tau(p^{r-1}) */
        long long *tp = (long long *)calloc(e + 1, sizeof(long long));
        tp[0] = 1;
        tp[1] = tau_arr[p];
        long long p11 = mod_pow(p, 11, MOD);
        for (int r = 1; r < e; r++)
            tp[r + 1] = (tp[1] * tp[r] % MOD - p11 * tp[r - 1] % MOD + MOD) % MOD;
        big_tau = big_tau * tp[e] % MOD;
        free(tp);
    }

    /* sigma_k(N!) = prod_p (p^{k(e+1)} - 1) / (p^k - 1) */
    long long big_sigmas[10];
    for (int ki = 0; ki < n_needed_ks; ki++) {
        int k = needed_ks[ki];
        long long val = 1;
        for (int i = 0; i < nprimes; i++) {
            int p = primes[i];
            int e = fac_exp[p];
            if (e == 0) continue;
            long long pk = mod_pow(p, k, MOD);
            long long num = (mod_pow(p, (long long)k * (e + 1), MOD) - 1 + MOD) % MOD;
            long long den = (pk - 1 + MOD) % MOD;
            val = val * (num % MOD * mod_inv(den) % MOD) % MOD;
        }
        big_sigmas[ki] = val;
    }

    /* N! mod MOD */
    /* Since 10000 > MOD is false (10000 < 10^9+7), so N! mod MOD is just 10000! mod MOD */
    long long factorial_mod = 1;
    for (int x = 1; x <= BIG_N; x++)
        factorial_mod = factorial_mod * x % MOD;

    /* Powers of (N! mod MOD) */
    int max_l = 0;
    for (int i = 0; i < nbasis; i++)
        if (basis[i].l > max_l) max_l = basis[i].l;

    long long fac_powers[20];
    fac_powers[0] = 1;
    for (int l = 1; l <= max_l; l++)
        fac_powers[l] = fac_powers[l - 1] * factorial_mod % MOD;

    /* Final answer */
    long long ans = big_tau * X_sol[0] % MOD;
    for (int i = 0; i < nbasis; i++) {
        int ki_idx = -1;
        for (int kk = 0; kk < n_needed_ks; kk++)
            if (needed_ks[kk] == basis[i].k) { ki_idx = kk; break; }
        ans = (ans + big_sigmas[ki_idx] * fac_powers[basis[i].l] % MOD * X_sol[i + 1]) % MOD;
    }
    ans = (ans % MOD + MOD) % MOD;

    printf("%lld\n", ans);
    return 0;
}
