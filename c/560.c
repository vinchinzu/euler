/*
 * Project Euler Problem 560: Coprime Nim.
 * Compute the number of losing configurations for the first player.
 * K = N = 10^7, mod 10^9+7.
 *
 * Nimber of pile s: 0 if even, 1 if s=1, pi(p)+1 for smallest prime p|s otherwise.
 * Use Walsh-Hadamard XOR convolution to compute K-fold XOR convolution of counts.
 *
 * Since the number of distinct nimbers can be large (~664K primes below 10^7),
 * we need a power-of-2 transform size. We use a sieve to assign nimber values
 * and then FWHT.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

typedef long long ll;
typedef __int128 i128;

#define N_VAL 10000000
#define K_VAL 10000000LL
#define MOD 1000000007LL

int main(void) {
    /* Sieve to find smallest prime factor */
    int *spf = (int*)calloc(N_VAL + 1, sizeof(int));
    for (int i = 2; i <= N_VAL; i++) {
        if (spf[i] == 0) {
            for (int j = i; j <= N_VAL; j += i) {
                if (spf[j] == 0) spf[j] = i;
            }
        }
    }

    /* Assign prime indices: pi_index[p] = index of p among primes (0-based) */
    int *primes = (int*)malloc((N_VAL / 5) * sizeof(int));
    int nprimes = 0;
    for (int i = 2; i <= N_VAL; i++)
        if (spf[i] == i) primes[nprimes++] = i;

    /* Create mapping: prime -> its 0-based index */
    int *prime_idx = (int*)calloc(N_VAL + 1, sizeof(int));
    for (int i = 0; i < nprimes; i++)
        prime_idx[primes[i]] = i;

    /* Assign nimbers:
     * nimber(s) = 0 if s even
     * nimber(1) = 1
     * nimber(s) = pi(smallest_prime_factor(s)) + 1 for odd s > 1
     * where pi counts the index among all primes (1-based: pi(2)=1, pi(3)=2, ...)
     */
    /* Count nimbers: nimber values range from 0 to nprimes */
    int max_nimber = nprimes;
    ll *counts = (ll*)calloc(max_nimber + 2, sizeof(ll));

    for (int s = 1; s < N_VAL; s++) {
        int nim;
        if (s % 2 == 0) {
            nim = 0;
        } else if (s == 1) {
            nim = 1;
        } else {
            /* s is odd and > 1, so smallest prime factor is odd and >= 3 */
            nim = prime_idx[spf[s]] + 1;
        }
        counts[nim]++;
    }

    free(spf);
    free(prime_idx);

    /* We need XOR convolution power. The nimbers range from 0 to max_nimber.
     * For FWHT we need array size to be a power of 2.
     * max_nimber ~ 664579 (number of primes below 10^7)
     * Next power of 2 is 2^20 = 1048576
     */
    int sz = 1;
    while (sz <= max_nimber) sz <<= 1;

    ll *arr = (ll*)calloc(sz, sizeof(ll));
    for (int i = 0; i <= max_nimber; i++)
        arr[i] = counts[i] % MOD;
    free(counts);

    /* FWHT (forward) */
    for (int len = 1; len < sz; len <<= 1) {
        for (int i = 0; i < sz; i += len << 1) {
            for (int j = 0; j < len; j++) {
                ll u = arr[i + j];
                ll v = arr[i + j + len];
                arr[i + j] = (u + v) % MOD;
                arr[i + j + len] = ((u - v) % MOD + MOD) % MOD;
            }
        }
    }

    /* Raise each element to K-th power */
    for (int i = 0; i < sz; i++) {
        ll base = arr[i], exp = K_VAL, result = 1;
        base %= MOD;
        while (exp > 0) {
            if (exp & 1) result = (i128)result * base % MOD;
            base = (i128)base * base % MOD;
            exp >>= 1;
        }
        arr[i] = result;
    }

    /* FWHT (inverse) */
    for (int len = 1; len < sz; len <<= 1) {
        for (int i = 0; i < sz; i += len << 1) {
            for (int j = 0; j < len; j++) {
                ll u = arr[i + j];
                ll v = arr[i + j + len];
                arr[i + j] = (u + v) % MOD;
                arr[i + j + len] = ((u - v) % MOD + MOD) % MOD;
            }
        }
    }

    /* Divide by sz */
    ll inv_sz = 1;
    {
        ll base = sz, exp = MOD - 2, result = 1;
        while (exp > 0) {
            if (exp & 1) result = (i128)result * base % MOD;
            base = (i128)base * base % MOD;
            exp >>= 1;
        }
        inv_sz = result;
    }
    for (int i = 0; i < sz; i++)
        arr[i] = (i128)arr[i] * inv_sz % MOD;

    printf("%lld\n", (long long)arr[0]);

    free(arr);
    free(primes);
    return 0;
}
