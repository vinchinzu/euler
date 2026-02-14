/*
 * Project Euler 639: Summing a multiplicative function
 *
 * Find sum_{k=1}^{K} sum_{i=1}^{N} f_k(i), where f_k is multiplicative
 * with f_k(p) = p^k. N=10^12, K=50, mod 10^9+7.
 *
 * Uses powerful number iteration with Lagrange interpolation for power sums.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

typedef long long ll;
typedef __int128 lll;

#define N_VAL 1000000000000LL
#define K_VAL 50
#define MOD_VAL 1000000007LL

ll powmod(ll base, ll exp, ll mod) {
    ll result = 1;
    base = ((base % mod) + mod) % mod;
    while (exp > 0) {
        if (exp & 1) result = (lll)result * base % mod;
        base = (lll)base * base % mod;
        exp >>= 1;
    }
    return result;
}

int L; /* sqrt(N) */
int *primes;
int nprimes;
char *is_prime;

ll *nth_pows;     /* nth_pows[i] = i^k mod MOD */
ll *sum_powers;   /* sum_powers[i] = sum_{j=1}^i j^k */
ll *sum_coeffs;   /* sum_coeffs[i] = sum over primes p<=i of p^k*(1-p^k) */

/* Factorial inverses for Lagrange interpolation */
ll fact[K_VAL + 3];
ll inv_fact[K_VAL + 3];

void init_lagrange() {
    int max_deg = K_VAL + 2;
    fact[0] = 1;
    for (int i = 1; i <= max_deg; i++)
        fact[i] = (lll)fact[i-1] * i % MOD_VAL;
    inv_fact[max_deg] = powmod(fact[max_deg], MOD_VAL - 2, MOD_VAL);
    for (int i = max_deg - 1; i >= 0; i--)
        inv_fact[i] = (lll)inv_fact[i+1] * (i+1) % MOD_VAL;
}

/* Compute sum_{i=1}^{n} i^k mod MOD using Lagrange interpolation */
ll sum_kth_powers(ll n, int k) {
    if (n <= L) return sum_powers[n];
    int pts = k + 2;
    ll prefix[K_VAL + 3], suffix[K_VAL + 3];
    prefix[0] = 1;
    for (int j = 0; j < pts; j++)
        prefix[j+1] = (lll)prefix[j] * ((n - j) % MOD_VAL + MOD_VAL) % MOD_VAL;
    suffix[pts] = 1;
    for (int j = pts - 1; j >= 0; j--)
        suffix[j] = (lll)suffix[j+1] * ((n - j) % MOD_VAL + MOD_VAL) % MOD_VAL;
    ll result = 0;
    int m = k + 1; /* degree */
    for (int i = 0; i < pts; i++) {
        ll numer = (lll)prefix[i] * suffix[i+1] % MOD_VAL;
        numer = (lll)numer * sum_powers[i] % MOD_VAL;
        ll denom = (lll)inv_fact[i] * inv_fact[m - i] % MOD_VAL;
        if ((m - i) % 2 == 1) denom = MOD_VAL - denom;
        result = (result + (lll)numer * denom) % MOD_VAL;
    }
    return result;
}

/* Stack for powerful number iteration */
typedef struct {
    int min_idx;
    ll d;
    ll mult;
    int prev_e;
} StackEntry;

#define MAX_STACK 2000000
StackEntry *stack;
int stack_top;

int main() {
    L = (int)sqrt((double)N_VAL);
    while ((ll)L * L > N_VAL) L--;
    while ((ll)(L+1) * (L+1) <= N_VAL) L++;

    /* Prime sieve up to L */
    is_prime = (char *)calloc(L + 1, 1);
    is_prime[2] = 1;
    for (int i = 3; i <= L; i += 2) is_prime[i] = 1;
    for (int i = 3; i * i <= L; i += 2)
        if (is_prime[i])
            for (int j = i * i; j <= L; j += 2 * i)
                is_prime[j] = 0;

    nprimes = 0;
    for (int i = 2; i <= L; i++) if (is_prime[i]) nprimes++;
    primes = (int *)malloc(nprimes * sizeof(int));
    int idx = 0;
    for (int i = 2; i <= L; i++) if (is_prime[i]) primes[idx++] = i;

    init_lagrange();

    nth_pows = (ll *)malloc((L + 1) * sizeof(ll));
    sum_powers = (ll *)malloc((L + 1) * sizeof(ll));
    sum_coeffs = (ll *)malloc((L + 1) * sizeof(ll));
    stack = (StackEntry *)malloc(MAX_STACK * sizeof(StackEntry));

    for (int i = 0; i <= L; i++) nth_pows[i] = 1;

    ll ans = 0;

    for (int k = 1; k <= K_VAL; k++) {
        sum_powers[0] = 0;
        sum_coeffs[0] = 0;
        for (int i = 1; i <= L; i++) {
            nth_pows[i] = (lll)nth_pows[i] * i % MOD_VAL;
            sum_powers[i] = (sum_powers[i-1] + nth_pows[i]) % MOD_VAL;
            ll coeff = 0;
            if (is_prime[i]) {
                coeff = (lll)nth_pows[i] * (1 - nth_pows[i] + MOD_VAL) % MOD_VAL;
            }
            sum_coeffs[i] = (sum_coeffs[i-1] + coeff) % MOD_VAL;
        }

        /* Powerful number iteration */
        stack_top = 0;
        stack[stack_top].min_idx = 0;
        stack[stack_top].d = 1;
        stack[stack_top].mult = 1;
        stack[stack_top].prev_e = 0;
        stack_top++;

        while (stack_top > 0) {
            stack_top--;
            int min_idx = stack[stack_top].min_idx;
            ll d = stack[stack_top].d;
            ll mult = stack[stack_top].mult;
            int prev_e = stack[stack_top].prev_e;

            ll n = N_VAL / d;

            /* Part 1: direct contribution */
            if (prev_e != 2) {
                ll sp = sum_kth_powers(n, k);
                ans = (ans + (lll)sp * mult) % MOD_VAL;
            }

            /* Compute lim = n^{1/3} */
            ll lim = (ll)cbrt((double)n);
            while (lim > 0 && lim * lim * lim > n) lim--;
            while ((lim + 1) * (lim + 1) * (lim + 1) <= n) lim++;

            /* Part 2: individual primes (exponent 2) */
            for (int i = min_idx; i < nprimes; i++) {
                ll p = primes[i];
                ll pp = p * p;
                ll threshold = (lim > 0) ? n / lim : n;
                if (pp > threshold) break;
                ll q = n / pp;
                ll sp_q = sum_kth_powers(q, k);
                ll coeff = (lll)nth_pows[(int)p] * (1 - nth_pows[(int)p] + MOD_VAL) % MOD_VAL;
                ans = (ans + (lll)sp_q % MOD_VAL * mult % MOD_VAL * coeff) % MOD_VAL;
            }

            /* Part 3: ranges of primes using sum_coeffs */
            ll p_min = (min_idx < nprimes) ? primes[min_idx] : (ll)L + 1;
            for (ll q = 1; q < lim; q++) {
                ll high = (ll)sqrt((double)(n / q));
                while (high > 0 && high * high > n / q) high--;
                while ((high + 1) * (high + 1) <= n / q) high++;

                ll low_sq = n / (q + 1);
                ll low = (ll)sqrt((double)low_sq);
                while (low > 0 && low * low > low_sq) low--;
                while ((low + 1) * (low + 1) <= low_sq) low++;
                if (low < p_min - 1) low = p_min - 1;

                if (high > L) high = L;
                if (high >= low && high >= 0 && low >= 0) {
                    ll coeff_sum = (sum_coeffs[high] - sum_coeffs[low] + MOD_VAL) % MOD_VAL;
                    ans = (ans + (lll)sum_powers[q] * mult % MOD_VAL * coeff_sum) % MOD_VAL;
                } else {
                    break;
                }
            }

            /* Part 4: recurse with higher prime powers (exponent >= 3) */
            for (int i = min_idx; i < nprimes; i++) {
                ll p = primes[i];
                if (d * p * p * p > N_VAL) break;
                ll new_d = d * p;
                ll new_mult = (lll)mult * nth_pows[(int)p] % MOD_VAL;
                new_mult = (lll)new_mult * (1 - nth_pows[(int)p] + MOD_VAL) % MOD_VAL;
                int e = 1;
                while (new_d * p <= N_VAL) {
                    new_d *= p;
                    e++;
                    if (stack_top >= MAX_STACK) {
                        fprintf(stderr, "Stack overflow\n");
                        return 1;
                    }
                    stack[stack_top].min_idx = i + 1;
                    stack[stack_top].d = new_d;
                    stack[stack_top].mult = new_mult;
                    stack[stack_top].prev_e = e;
                    stack_top++;
                }
            }
        }
    }

    printf("%lld\n", ans % MOD_VAL);

    free(nth_pows);
    free(sum_powers);
    free(sum_coeffs);
    free(primes);
    free(is_prime);
    free(stack);
    return 0;
}
