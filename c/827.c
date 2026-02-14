/*
 * Project Euler 827: Pythagorean Triple Occurrence
 *
 * Q(n) = smallest number occurring in exactly n Pythagorean triples.
 * Find sum_{k=1}^{18} Q(10^k) mod 409120391.
 *
 * Uses the relationship between number of Pythagorean triples and
 * divisor structure (primes 1 mod 4 and 3 mod 4).
 */
#include <stdio.h>
#include <stdlib.h>
#include <math.h>
#include <string.h>

#define MOD 409120391LL

typedef long long ll;
typedef __int128 i128;

/* Sieve primes up to 500 */
static int primes[100], nprimes = 0;
static int primes_1mod4[50], n1mod4 = 0;
static int primes_3mod4[50], n3mod4 = 0;
static double log_p1[50], log_p3[50];
static double LOG2;

static void init_primes(void) {
    int is_p[501];
    memset(is_p, 1, sizeof(is_p));
    is_p[0] = is_p[1] = 0;
    for (int i = 2; i * i <= 500; i++)
        if (is_p[i])
            for (int j = i * i; j <= 500; j += i)
                is_p[j] = 0;
    for (int i = 2; i <= 500; i++) {
        if (is_p[i]) {
            primes[nprimes++] = i;
            if (i % 4 == 1) {
                log_p1[n1mod4] = log((double)i);
                primes_1mod4[n1mod4++] = i;
            } else if (i % 4 == 3) {
                log_p3[n3mod4] = log((double)i);
                primes_3mod4[n3mod4++] = i;
            }
        }
    }
    LOG2 = log(2.0);
}

static ll pow_mod(ll base, ll exp, ll mod) {
    i128 result = 1;
    i128 b = base % mod;
    while (exp > 0) {
        if (exp & 1) result = result * b % mod;
        b = b * b % mod;
        exp >>= 1;
    }
    return (ll)result;
}

/* Get all odd divisors of n (remove factors of 2 first) */
static int get_odd_divisors(ll n, ll *divs) {
    while (n % 2 == 0) n /= 2;
    if (n == 1) { divs[0] = 1; return 1; }

    /* Factorize n */
    ll ps[30]; int es[30]; int nf = 0;
    ll tmp = n;
    for (int i = 0; i < nprimes && (ll)primes[i] * primes[i] <= tmp; i++) {
        if (tmp % primes[i] == 0) {
            ps[nf] = primes[i]; es[nf] = 0;
            while (tmp % primes[i] == 0) { tmp /= primes[i]; es[nf]++; }
            nf++;
        }
    }
    if (tmp > 1) { ps[nf] = tmp; es[nf] = 1; nf++; }

    int nd = 1;
    divs[0] = 1;
    for (int i = 0; i < nf; i++) {
        int cur = nd;
        ll pk = 1;
        for (int j = 0; j < es[i]; j++) {
            pk *= ps[i];
            for (int k = 0; k < cur; k++)
                divs[nd++] = divs[k] * pk;
        }
    }

    /* Sort */
    for (int i = 0; i < nd - 1; i++)
        for (int j = i + 1; j < nd; j++)
            if (divs[i] > divs[j]) { ll t = divs[i]; divs[i] = divs[j]; divs[j] = t; }
    return nd;
}

static int get_all_divisors(ll n, ll *divs) {
    if (n == 1) { divs[0] = 1; return 1; }
    ll ps[30]; int es[30]; int nf = 0;
    ll tmp = n;
    for (int i = 0; i < nprimes && (ll)primes[i] * primes[i] <= tmp; i++) {
        if (tmp % primes[i] == 0) {
            ps[nf] = primes[i]; es[nf] = 0;
            while (tmp % primes[i] == 0) { tmp /= primes[i]; es[nf]++; }
            nf++;
        }
    }
    if (tmp > 1) { ps[nf] = tmp; es[nf] = 1; nf++; }

    int nd = 1;
    divs[0] = 1;
    for (int i = 0; i < nf; i++) {
        int cur = nd;
        ll pk = 1;
        for (int j = 0; j < es[i]; j++) {
            pk *= ps[i];
            for (int k = 0; k < cur; k++)
                divs[nd++] = divs[k] * pk;
        }
    }

    for (int i = 0; i < nd - 1; i++)
        for (int j = i + 1; j < nd; j++)
            if (divs[i] > divs[j]) { ll t = divs[i]; divs[i] = divs[j]; divs[j] = t; }
    return nd;
}

/* Ordered factorizations of n into factors >= min_val (n must be odd) */
typedef struct { int factors[20]; int len; } Factorization;
static Factorization factor_results[10000];
static int n_factor_results;

static void ordered_factorizations(ll n, int min_val, int *cur_factors, int depth) {
    if (n == 1) {
        Factorization *f = &factor_results[n_factor_results++];
        f->len = depth;
        for (int i = 0; i < depth; i++) f->factors[i] = cur_factors[i];
        return;
    }
    if (n < min_val) return;

    ll all_divs[2000]; int nd = get_all_divisors(n, all_divs);

    for (int i = 0; i < nd; i++) {
        ll d = all_divs[i];
        if (d < min_val) continue;
        if (d == n) {
            cur_factors[depth] = (int)n;
            Factorization *f = &factor_results[n_factor_results++];
            f->len = depth + 1;
            for (int j = 0; j <= depth; j++) f->factors[j] = cur_factors[j];
        } else if (n % d == 0 && d * d <= n) {
            cur_factors[depth] = (int)d;
            ordered_factorizations(n / d, (int)d, cur_factors, depth + 1);
        }
    }
}

/* Find min number for a shape using given primes */
static int min_number_for_shape(ll n, double *log_primes, int *mod_primes, int num_primes,
                                 double budget, double *out_log, ll *out_mod) {
    if (n == 1) { *out_log = 0.0; *out_mod = 1; return 1; }
    if (n < 3 || n % 2 == 0) return 0;

    n_factor_results = 0;
    int cur_factors[20];
    ordered_factorizations(n, 3, cur_factors, 0);

    double best_log = budget;
    ll best_mod_val = 0;
    int found = 0;

    for (int fi = 0; fi < n_factor_results; fi++) {
        Factorization *f = &factor_results[fi];
        int k = f->len;
        if (k > num_primes) continue;

        /* Compute exponents = sorted((f_i-1)/2, descending) */
        int exps[20];
        for (int i = 0; i < k; i++) exps[i] = (f->factors[i] - 1) / 2;
        /* Sort descending */
        for (int i = 0; i < k - 1; i++)
            for (int j = i + 1; j < k; j++)
                if (exps[i] < exps[j]) { int t = exps[i]; exps[i] = exps[j]; exps[j] = t; }

        double log_val = 0;
        for (int i = 0; i < k; i++) log_val += exps[i] * log_primes[i];

        if (log_val < best_log) {
            best_log = log_val;
            ll mod_val = 1;
            for (int i = 0; i < k; i++)
                mod_val = (i128)mod_val * pow_mod(mod_primes[i], exps[i], MOD) % MOD;
            best_mod_val = mod_val;
            found = 1;
        }
    }

    if (found) {
        *out_log = best_log;
        *out_mod = best_mod_val;
        return 1;
    }
    return 0;
}

int main(void) {
    init_primes();

    int N = 18;
    ll total = 0;

    for (int k = 1; k <= N; k++) {
        ll T = 1;
        for (int i = 0; i < k; i++) T *= 10;
        ll target = 2 * T + 2;

        double best_log = 1e30;
        ll best_mod = 0;

        ll odd_divs[20000]; int n_odd = get_odd_divisors(target, odd_divs);

        for (int ai = 0; ai < n_odd; ai++) {
            ll A = odd_divs[ai];
            ll D = target / A;
            ll Dm1 = D - 1;

            double logA; ll modA;
            if (A == 1) { logA = 0.0; modA = 1; }
            else {
                if (!min_number_for_shape(A, log_p1, primes_1mod4, n1mod4, best_log, &logA, &modA))
                    continue;
            }

            if (logA >= best_log) continue;

            /* Case 1: odd m, B = Dm1 */
            if (Dm1 == 1) {
                if (logA < best_log) { best_log = logA; best_mod = modA; }
            } else if (Dm1 % 2 == 1) {
                double remaining = best_log - logA;
                double logB; ll modB;
                if (min_number_for_shape(Dm1, log_p3, primes_3mod4, n3mod4, remaining, &logB, &modB)) {
                    double tl = logA + logB;
                    if (tl < best_log) { best_log = tl; best_mod = (i128)modA * modB % MOD; }
                }
            }

            /* Case 2: even m */
            if (Dm1 >= 1 && Dm1 % 2 == 1) {
                ll c_divs[20000]; int nc = get_all_divisors(Dm1, c_divs);
                for (int ci = 0; ci < nc; ci++) {
                    ll C = c_divs[ci];
                    ll B = Dm1 / C;
                    ll a0 = (C + 1) / 2;
                    double log2_part = a0 * LOG2;

                    if (logA + log2_part >= best_log) continue;

                    double remaining = best_log - logA - log2_part;
                    ll mod2 = pow_mod(2, a0, MOD);

                    if (B == 1) {
                        double tl = logA + log2_part;
                        if (tl < best_log) { best_log = tl; best_mod = (i128)modA * mod2 % MOD; }
                    } else if (B % 2 == 1) {
                        double logB; ll modB;
                        if (min_number_for_shape(B, log_p3, primes_3mod4, n3mod4, remaining, &logB, &modB)) {
                            double tl = logA + log2_part + logB;
                            if (tl < best_log) {
                                best_log = tl;
                                best_mod = (i128)modA * mod2 % MOD;
                                best_mod = (i128)best_mod * modB % MOD;
                            }
                        }
                    }
                }
            }
        }

        total = (total + best_mod) % MOD;
    }

    printf("%lld\n", total);
    return 0;
}
