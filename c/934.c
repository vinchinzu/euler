/* Project Euler Problem 934 - Unlucky Prime
 * U(N) = sum_{n=1}^N u(n) where u(n) is smallest prime p with n%p not divisible by 7.
 * Uses sieving over prime products with CRT-like residue expansion.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef long long ll;
typedef unsigned long long ull;
typedef __int128 lll;

int main(void) {
    /* Sieve primes up to 1000 */
    int limit = 1000;
    int *primes = NULL;
    int nprimes = 0;
    {
        char *is_prime = (char *)malloc(limit + 1);
        memset(is_prime, 1, limit + 1);
        is_prime[0] = is_prime[1] = 0;
        for (int i = 2; i * i <= limit; i++)
            if (is_prime[i])
                for (int j = i * i; j <= limit; j += i)
                    is_prime[j] = 0;
        for (int i = 2; i <= limit; i++)
            if (is_prime[i]) nprimes++;
        primes = (int *)malloc(nprimes * sizeof(int));
        int idx = 0;
        for (int i = 2; i <= limit; i++)
            if (is_prime[i]) primes[idx++] = i;
        free(is_prime);
    }

    ll N = 100000000000000000LL; /* 10^17 */
    ll total = 0;

    /* k=1: P1=2. Term 2 * S1. S1=N */
    total += 2 * N;

    /* k=2: P2=3. Term (3-2) * S2. S2=N/2 */
    total += 1 * (N / 2);

    /* k=3: P3=5. Term (5-3) * S3. S3=N/6 */
    total += 2 * (N / 6);

    /* k=4: P4=7. Term (7-5) * S4. S4=N/30 */
    total += 2 * (N / 30);

    /* k=5: P5=11. Term (11-7) * S5. S5=N/210 */
    total += 4 * (N / 210);

    ll M = N / 210;

    /* Dynamic residue expansion */
    /* residues mod mod_val: numbers n <= M such that 30*n is divisible only by
       primes whose remainder mod 7 is 0. */
    ll *residues = (ll *)malloc(sizeof(ll) * 20000000);
    int nres = 1;
    residues[0] = 0;
    ll mod_val = 1;

    /* Start from index 4 (P5=11) */
    int i = 4; /* primes[4] = 11 */
    while (i + 1 < nprimes) {
        int p = primes[i];
        int next_p = primes[i + 1];
        int diff = next_p - p;

        ll next_mod = mod_val * p;

        /* Compute valid digits: s in 0..(p-1)/7, val = (s * inv30) % p */
        int limit_s = (p - 1) / 7;

        /* Compute inv30 mod p */
        ll inv30 = 1;
        {
            ll base = 30 % p;
            ll exp = p - 2;
            ll res = 1;
            while (exp > 0) {
                if (exp & 1) res = res * base % p;
                base = base * base % p;
                exp >>= 1;
            }
            inv30 = res;
        }

        int *valid_digits = (int *)malloc((limit_s + 1) * sizeof(int));
        int nvalid = 0;
        for (int s = 0; s <= limit_s; s++) {
            valid_digits[nvalid++] = (int)((s * inv30) % p);
        }

        /* inv_mod = mod_val^(-1) mod p */
        ll inv_mod;
        {
            ll base = mod_val % p;
            ll exp = p - 2;
            ll res = 1;
            while (exp > 0) {
                if (exp & 1) res = res * base % p;
                base = base * base % p;
                exp >>= 1;
            }
            inv_mod = res;
        }

        int is_next_mod_large = (next_mod > M);

        ll *new_residues = (ll *)malloc(sizeof(ll) * (ll)nres * nvalid + sizeof(ll));
        int new_nres = 0;

        for (int ri = 0; ri < nres; ri++) {
            ll r = residues[ri];
            for (int di = 0; di < nvalid; di++) {
                int d = valid_digits[di];
                ll k_val = ((d - (r % p) + p) % p * inv_mod) % p;
                ll x = r + k_val * mod_val;
                if (is_next_mod_large) {
                    if (x <= M) new_residues[new_nres++] = x;
                } else {
                    new_residues[new_nres++] = x;
                }
            }
        }

        free(residues);
        free(valid_digits);
        residues = new_residues;
        nres = new_nres;
        mod_val = next_mod;

        /* Calculate count */
        ll count;
        if (mod_val > M) {
            count = nres;
        } else {
            ll full_cycles = M / mod_val;
            ll rem = M % mod_val;
            ll partial = 0;
            for (int ri = 0; ri < nres; ri++) {
                if (residues[ri] <= rem) partial++;
            }
            count = full_cycles * nres + partial;
        }
        count -= 1; /* Exclude 0 */

        if (count == 0) break;

        total += (ll)diff * count;
        i++;
    }

    printf("%lld\n", total);
    free(residues);
    free(primes);
    return 0;
}
