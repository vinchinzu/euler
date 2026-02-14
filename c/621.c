/*
 * Project Euler 621: Sum of three triangular numbers
 *
 * G(n) = number of ordered triples of triangular numbers summing to n.
 * G(n) = sum_{k=0}^{L} (d_1(Q_k) - d_3(Q_k))
 * where Q_k = 4n+1 - 2k(k+1), and d_1/d_3 count divisors mod 4.
 *
 * Uses sieve-based factorization over ~6M values of k.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

typedef long long ll;
typedef unsigned long long ull;

int main() {
    ll N_val = 17526000000000LL;  /* 17526 * 10^9 */
    ll Q0_val = 4 * N_val + 1;   /* 70104000000001 */
    int L_val = ((int)(sqrt(8.0 * N_val + 1.0)) - 1) / 2;  /* ~5920685 */
    /* Verify L_val */
    while ((ll)L_val * (L_val + 1) / 2 > N_val) L_val--;
    while ((ll)(L_val + 1) * (L_val + 2) / 2 <= N_val) L_val++;

    int sz = L_val + 1;

    ll *remaining = (ll *)malloc(sz * sizeof(ll));
    int *result = (int *)malloc(sz * sizeof(int));

    for (int k = 0; k < sz; k++) {
        remaining[k] = Q0_val - 2LL * k * (k + 1);
        result[k] = 1;
    }

    int sieve_limit = (int)(sqrt((double)Q0_val)) + 2;
    char *is_prime = (char *)calloc(sieve_limit + 1, 1);
    memset(is_prime, 1, sieve_limit + 1);
    is_prime[0] = is_prime[1] = 0;
    for (int i = 2; (ll)i * i <= sieve_limit; i++)
        if (is_prime[i])
            for (int j = i * i; j <= sieve_limit; j += i)
                is_prime[j] = 0;

    ll M_val = 2LL * Q0_val + 1; /* = 8N + 3 */

    for (int p = 3; p <= sieve_limit; p += 2) {
        if (!is_prime[p]) continue;

        ll m_mod_p = M_val % p;
        ll euler = 1;
        {
            ll base = m_mod_p % p;
            ll exp = (p - 1) / 2;
            ll r = 1;
            while (exp > 0) {
                if (exp & 1) r = r * base % p;
                base = base * base % p;
                exp >>= 1;
            }
            euler = r;
        }

        if (euler != 1 && m_mod_p != 0) continue;

        ll sq;
        if (m_mod_p == 0) {
            sq = 0;
        } else if (p % 4 == 3) {
            ll base = m_mod_p;
            ll exp = (p + 1) / 4;
            ll r = 1;
            while (exp > 0) {
                if (exp & 1) r = r * base % p;
                base = base * base % p;
                exp >>= 1;
            }
            sq = r;
        } else {
            /* Tonelli-Shanks */
            ll Q = p - 1;
            int S = 0;
            while (Q % 2 == 0) { Q /= 2; S++; }
            ll z = 2;
            while (1) {
                ll base = z % p, exp2 = (p-1)/2, r2 = 1;
                while (exp2 > 0) { if(exp2&1) r2=r2*base%p; base=base*base%p; exp2>>=1; }
                if (r2 != 1) break;
                z++;
            }
            ll MM = S;
            ll c = 1; { ll base=z%p, exp2=Q; while(exp2>0){if(exp2&1)c=c*base%p;base=base*base%p;exp2>>=1;} }
            ll t = 1; { ll base=m_mod_p%p, exp2=Q; while(exp2>0){if(exp2&1)t=t*base%p;base=base*base%p;exp2>>=1;} }
            ll R = 1; { ll base=m_mod_p%p, exp2=(Q+1)/2; while(exp2>0){if(exp2&1)R=R*base%p;base=base*base%p;exp2>>=1;} }
            while (t != 1) {
                ll tt = t;
                int ii = 0;
                while (tt != 1) { tt = tt * tt % p; ii++; }
                ll b2 = c;
                for (int jj = 0; jj < MM - ii - 1; jj++) b2 = b2 * b2 % p;
                MM = ii;
                c = b2 * b2 % p;
                t = t * c % p;
                R = R * b2 % p;
            }
            sq = R;
        }

        ll inv2 = (p + 1) / 2;
        ll roots[2];
        int nroots;
        if (m_mod_p == 0) {
            roots[0] = (p - 1) * inv2 % p;
            nroots = 1;
        } else {
            roots[0] = (sq - 1 + p) % p * inv2 % p;
            roots[1] = (p - sq - 1 + p) % p * inv2 % p;
            nroots = (roots[0] == roots[1]) ? 1 : 2;
        }

        int p_mod4 = p % 4;

        for (int ri = 0; ri < nroots; ri++) {
            ll k_start = roots[ri];
            for (ll k = k_start; k < sz; k += p) {
                int e = 0;
                while (remaining[k] % p == 0) {
                    remaining[k] /= p;
                    e++;
                }
                if (e > 0) {
                    if (p_mod4 == 1) {
                        result[k] *= (e + 1);
                    } else {
                        if (e % 2 == 1) {
                            result[k] = 0;
                        }
                    }
                }
            }
        }
    }

    ll answer = 0;
    for (int k = 0; k < sz; k++) {
        if (result[k] == 0) continue;
        ll r = remaining[k];
        if (r == 1) {
            answer += result[k];
        } else {
            if (r % 4 == 1) {
                answer += result[k] * 2;
            }
        }
    }

    printf("%lld\n", answer);

    free(remaining);
    free(result);
    free(is_prime);
    return 0;
}
