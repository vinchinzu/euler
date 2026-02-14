/*
 * Project Euler Problem 501: Eight Divisors.
 * Count integers <= N with exactly 8 divisors.
 * Forms: p*q*r (3 distinct primes), p^3*q (p!=q), p^7.
 * Uses Lucy_Hedgehog prime counting for large values.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>
#include <stdint.h>

typedef long long ll;

/*
 * Lucy_Hedgehog prime counting function.
 * Computes pi(n) for n and all floor(N/k) values in O(N^{2/3}) time.
 */

ll N_global;
int sqrtN;

/* For quotient values n/k: if n/k <= sqrtN, index is n/k; otherwise index is sqrtN + N/(n/k) */
/* We use two arrays: small[0..sqrtN] and large[1..sqrtN] */
ll *S_small; /* S_small[v] = pi(v) for v <= sqrtN */
ll *S_large; /* S_large[k] = pi(N/k) for k <= sqrtN */

void lucy(ll N) {
    N_global = N;
    sqrtN = (int)sqrt((double)N);
    while ((ll)sqrtN * sqrtN > N) sqrtN--;
    while ((ll)(sqrtN + 1) * (sqrtN + 1) <= N) sqrtN++;

    S_small = (ll*)malloc((sqrtN + 2) * sizeof(ll));
    S_large = (ll*)malloc((sqrtN + 2) * sizeof(ll));

    /* Initialize: S(v) = v - 1 (count of integers >= 2 up to v) */
    for (int i = 0; i <= sqrtN; i++)
        S_small[i] = i - 1;
    for (int k = 1; k <= sqrtN; k++)
        S_large[k] = N / k - 1;

    /* Sieve */
    char *is_prime_flag = (char*)calloc(sqrtN + 1, 1);
    memset(is_prime_flag, 1, sqrtN + 1);
    is_prime_flag[0] = is_prime_flag[1] = 0;

    for (int p = 2; p <= sqrtN; p++) {
        if (!is_prime_flag[p]) continue;
        ll p2 = (ll)p * p;

        /* Update S_large[k] for k such that N/k >= p^2 */
        for (int k = 1; k <= sqrtN && N / k >= p2; k++) {
            ll v = N / k;
            ll v_over_p = v / p;
            ll sub;
            if (v_over_p <= sqrtN)
                sub = S_small[(int)v_over_p];
            else
                sub = S_large[(int)(N / v_over_p)]; /* k * p might not be right, compute directly */
            S_large[k] -= sub - S_small[p - 1];
        }

        /* Update S_small[v] for v >= p^2 */
        for (int v = sqrtN; v >= p2; v--) {
            S_small[v] -= S_small[v / p] - S_small[p - 1];
        }

        /* Mark composites */
        for (ll j = p2; j <= sqrtN; j += p)
            is_prime_flag[(int)j] = 0;
    }

    free(is_prime_flag);
}

/* Get pi(v) from Lucy tables */
ll pi(ll v) {
    if (v <= 0) return 0;
    if (v <= sqrtN) return S_small[(int)v];
    ll k = N_global / v;
    return S_large[(int)k];
}

/* Sieve small primes for iteration */
int *small_primes;
int num_small_primes;

void sieve_small(int limit) {
    char *is_p = (char*)calloc(limit + 1, 1);
    memset(is_p, 1, limit + 1);
    is_p[0] = is_p[1] = 0;
    for (int i = 2; (ll)i * i <= limit; i++)
        if (is_p[i])
            for (int j = i * i; j <= limit; j += i)
                is_p[j] = 0;
    num_small_primes = 0;
    for (int i = 2; i <= limit; i++)
        if (is_p[i]) num_small_primes++;
    small_primes = (int*)malloc(num_small_primes * sizeof(int));
    int idx = 0;
    for (int i = 2; i <= limit; i++)
        if (is_p[i]) small_primes[idx++] = i;
    free(is_p);
}

int main() {
    ll N = 1000000000000LL;
    int L = (int)pow((double)N, 2.0 / 3.0) + 100;

    /* Sieve primes up to L for iteration */
    sieve_small(L);

    /* Build Lucy tables */
    lucy(N);

    /* Also build small pi array for direct lookup */
    int *pi_small = (int*)calloc(L + 1, sizeof(int));
    {
        int cnt = 0, pidx = 0;
        for (int i = 1; i <= L; i++) {
            if (pidx < num_small_primes && small_primes[pidx] == i) {
                cnt++;
                pidx++;
            }
            pi_small[i] = cnt;
        }
    }

    ll ans = 0;

    /* Count p*q*r with p < q < r, all primes */
    for (int pi_idx = 0; pi_idx < num_small_primes; pi_idx++) {
        ll p = small_primes[pi_idx];
        if (p * p * p > N) break;
        for (int qi_idx = pi_idx + 1; qi_idx < num_small_primes; qi_idx++) {
            ll q = small_primes[qi_idx];
            if (p * q * q > N) break;
            ll limit = N / (p * q);
            ll pi_limit = pi(limit);
            ans += pi_limit - (ll)pi_small[(int)q];
        }
    }

    /* Count p^3 * q */
    for (int pi_idx = 0; pi_idx < num_small_primes; pi_idx++) {
        ll p = small_primes[pi_idx];
        if (p * p * p > N) break;
        ll limit = N / (p * p * p);
        ll pi_limit = pi(limit);
        ans += pi_limit;
        /* Subtract q == p case */
        if (p * p * p * p <= N)
            ans -= 1;
    }

    /* Count p^7 */
    int max_p7 = (int)pow((double)N, 1.0 / 7.0);
    while (1) {
        /* Verify (max_p7+1)^7 <= N */
        double test = 1.0;
        int ok = 1;
        for (int i = 0; i < 7; i++) {
            test *= (max_p7 + 1);
            if (test > (double)N) { ok = 0; break; }
        }
        if (!ok) break;
        max_p7++;
    }
    while (max_p7 > 0) {
        double test = 1.0;
        int ok = 1;
        for (int i = 0; i < 7; i++) {
            test *= max_p7;
            if (test > (double)N) { ok = 0; break; }
        }
        if (ok) break;
        max_p7--;
    }
    ans += pi_small[max_p7];

    printf("%lld\n", ans);

    free(S_small);
    free(S_large);
    free(small_primes);
    free(pi_small);
    return 0;
}
