/*
 * Project Euler Problem 712: Exponent Difference.
 *
 * For each prime p, sum |v_p(n) - v_p(m)| over all 1 <= n, m <= N.
 * Small primes: enumerate all exponent counts directly.
 * Large primes (p > sqrt(N)): v_p(n) is 0 or 1, contribution = 2*(N - N/p)*(N/p).
 * Use Lucy DP for prime counting.
 */
#include <stdio.h>
#include <stdlib.h>
#include <math.h>

typedef long long ll;
typedef __int128 lll;

#define MOD 1000000007LL

int main() {
    ll N = 1000000000000LL;
    ll L = (ll)sqrt((double)N);
    while ((L + 1) * (L + 1) <= N) L++;
    while (L * L > N) L--;

    int limit = (int)(N / L) + 1;

    /* Sieve primes up to limit */
    char *sieve = calloc(limit + 1, 1);
    for (int i = 2; i * i <= limit; i++) {
        if (!sieve[i]) {
            for (int j = i * i; j <= limit; j += i)
                sieve[j] = 1;
        }
    }
    int *primes_list = malloc((limit + 1) * sizeof(int));
    int num_small_primes = 0;
    for (int i = 2; i <= limit; i++) {
        if (!sieve[i]) primes_list[num_small_primes++] = i;
    }
    free(sieve);

    ll ans = 0;

    /* Process small primes */
    for (int pi = 0; pi < num_small_primes; pi++) {
        ll p = primes_list[pi];
        ll counts[60];
        int num_e = 0;
        ll pe = 1;
        while (pe <= N) {
            ll pe_next;
            if (pe > N / p) pe_next = N + 1; else pe_next = pe * p;
            ll cnt = ((N / pe) - (pe_next <= N ? N / pe_next : 0)) % MOD;
            counts[num_e++] = cnt;
            if (pe > N / p) break;
            pe *= p;
        }

        for (int vn = 0; vn < num_e; vn++) {
            for (int vm = 0; vm < num_e; vm++) {
                int diff = vn > vm ? vn - vm : vm - vn;
                ll contribution = (ll)diff % MOD * (counts[vn] % MOD) % MOD * (counts[vm] % MOD) % MOD;
                ans = (ans + contribution) % MOD;
            }
        }
    }

    /* Lucy DP for prime counting */
    int r = (int)L;
    int vlen = 2 * r + 2;
    ll *V = malloc(vlen * sizeof(ll));
    ll *S = malloc(vlen * sizeof(ll));
    /* Map: for v > r, index = N/v; for v <= r, index = r + v (approximately) */
    /* Actually use direct mapping via sorted array */

    int vcnt = 0;
    for (ll i = 1; i <= r; i++) {
        V[vcnt++] = N / i;
    }
    ll last = V[vcnt - 1];
    for (ll v = last - 1; v >= 1; v--) {
        V[vcnt++] = v;
    }

    /* S[i] = number of "primes" (actually integers >= 2) up to V[i], initially V[i] - 1 */
    /* We need a way to look up S[v]. For v > r, index = N/v - 1 (since V[0] = N/1, V[1] = N/2, etc.)
       For v <= r, index = vcnt - v. Actually let's use a function. */

    /* Better: use two arrays. small[v] for v <= r, big[i] for N/i, i <= r */
    ll *small_s = calloc(r + 2, sizeof(ll));
    ll *big_s = calloc(r + 2, sizeof(ll));

    for (int v = 1; v <= r; v++) small_s[v] = v - 1;
    for (int i = 1; i <= r; i++) big_s[i] = N / i - 1;

    for (int p = 2; p <= r; p++) {
        if (small_s[p] == small_s[p - 1]) continue; /* p is not prime */
        ll sp = small_s[p - 1];
        ll p2 = (ll)p * p;
        for (int i = 1; i <= r && N / i >= p2; i++) {
            ll v_over_p = N / i / p;
            ll sv;
            if (v_over_p <= r) sv = small_s[v_over_p];
            else sv = big_s[i * p]; /* N/(i*p) is the index */
            /* Wait, we need S[N/(i*p)]. If i*p <= r, that's big_s[i*p].
               Otherwise if N/(i*p) <= r, it's small_s[N/(i*p)]. */
            if ((ll)i * p <= r) sv = big_s[i * p];
            else sv = small_s[(int)(N / ((ll)i * p))];
            big_s[i] -= sv - sp;
        }
        for (int v = r; v >= p2; v--) {
            small_s[v] -= small_s[v / p] - sp;
        }
    }

    /* Now small_s[v] = pi(v) for v <= r, big_s[i] = pi(N/i) for i <= r */
    /* We need pi(N/q) and pi(N/(q+1)) for q = 1..L-1 */

    for (int q = 1; q < (int)L; q++) {
        /* pi(N/q) */
        ll pi_q;
        ll nq = N / q;
        if (nq <= r) pi_q = small_s[(int)nq]; else pi_q = big_s[q];

        ll pi_q1;
        ll nq1 = N / (q + 1);
        if (nq1 <= r) pi_q1 = small_s[(int)nq1]; else pi_q1 = big_s[q + 1];

        /* Exclude small primes we already processed */
        if (nq <= limit) {
            if (pi_q > num_small_primes) pi_q = num_small_primes;
        }
        if (nq1 <= limit) {
            if (pi_q1 > num_small_primes) pi_q1 = num_small_primes;
        }

        ll num_primes_in_range = (pi_q - pi_q1) % MOD;
        if (num_primes_in_range < 0) num_primes_in_range += MOD;

        /* Contribution: 2 * (N - q) * q * num_primes_in_range */
        ll contribution = 2 * ((N - q) % MOD) % MOD * (q % MOD) % MOD * num_primes_in_range % MOD;
        ans = (ans + contribution) % MOD;
    }

    printf("%lld\n", ans);

    free(primes_list);
    free(small_s);
    free(big_s);
    free(V);
    free(S);
    return 0;
}
