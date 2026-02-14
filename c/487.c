/* Project Euler 487 - Sums of power sums
 * Extracted from embedded C in python/487.py
 * Lagrange interpolation for sum of powers, iterated over primes in range.
 */
#include <stdio.h>
#include <math.h>

typedef unsigned long long ull;
typedef unsigned __int128 u128;

static int is_prime(ull n) {
    if (n < 2) return 0;
    if (n < 4) return 1;
    if (n % 2 == 0 || n % 3 == 0) return 0;
    for (ull i = 5; i * i <= n; i += 6) {
        if (n % i == 0 || n % (i + 2) == 0) return 0;
    }
    return 1;
}

static ull mulmod(ull a, ull b, ull m) {
    return (u128)a * b % m;
}

static ull powmod(ull base, ull exp, ull m) {
    ull result = 1;
    base %= m;
    while (exp > 0) {
        if (exp & 1) result = mulmod(result, base, m);
        base = mulmod(base, base, m);
        exp >>= 1;
    }
    return result;
}

#define MAXM 10003

static ull sum_powers(ull n, int k, ull p) {
    int m = k + 1;
    ull x = n % p;

    static ull y[MAXM + 1];
    y[0] = 0;
    for (int i = 1; i <= m; i++) {
        y[i] = (y[i-1] + powmod((ull)i, (ull)k, p)) % p;
    }

    if (n <= (ull)m) return y[n];

    static ull fact[MAXM + 1], inv_fact[MAXM + 1];
    fact[0] = 1;
    for (int i = 1; i <= m; i++)
        fact[i] = mulmod(fact[i-1], (ull)i, p);
    inv_fact[m] = powmod(fact[m], p - 2, p);
    for (int i = m; i > 0; i--)
        inv_fact[i-1] = mulmod(inv_fact[i], (ull)i, p);

    static ull pre[MAXM + 2], suf[MAXM + 2];
    pre[0] = 1;
    for (int i = 0; i <= m; i++) {
        ull diff = (x >= (ull)i) ? (x - (ull)i) : (p - ((ull)i - x) % p) % p;
        pre[i+1] = mulmod(pre[i], diff, p);
    }
    suf[m+1] = 1;
    for (int i = m; i >= 0; i--) {
        ull diff = (x >= (ull)i) ? (x - (ull)i) : (p - ((ull)i - x) % p) % p;
        suf[i] = mulmod(suf[i+1], diff, p);
    }

    ull result = 0;
    for (int i = 0; i <= m; i++) {
        ull num = mulmod(pre[i], suf[i+1], p);
        ull den = mulmod(inv_fact[i], inv_fact[m - i], p);
        ull term = mulmod(mulmod(y[i], num, p), den, p);
        if ((m - i) & 1)
            result = (result + p - term) % p;
        else
            result = (result + term) % p;
    }
    return result;
}

int main(void) {
    ull N_val = 1000000000000ULL;  /* 10^12 */
    int K = 10000;
    ull L = 2000000000ULL;
    ull H = 2000002000ULL;

    ull ans = 0;
    for (ull p = L; p < H; p++) {
        if (!is_prime(p)) continue;
        ull s_k = sum_powers(N_val, K, p);
        ull s_k1 = sum_powers(N_val, K + 1, p);
        ull np1 = (N_val + 1) % p;
        ull term = (mulmod(np1, s_k, p) + p - s_k1) % p;
        ans += term;
    }

    printf("%llu\n", ans);
    return 0;
}
