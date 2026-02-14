#include <stdio.h>
#include <stdlib.h>

typedef long long ll;
#define MOD 1000000007LL

ll power_mod(ll base, ll exp, ll mod) {
    ll result = 1;
    base %= mod;
    if (base < 0) base += mod;
    while (exp > 0) {
        if (exp & 1) result = result * base % mod;
        base = base * base % mod;
        exp >>= 1;
    }
    return result;
}

int main(void) {
    ll p = MOD;
    int n = 1000000;

    ll inv2 = power_mod(2, p - 2, p);

    /* Precompute factorials */
    ll *f = (ll *)malloc((n + 1) * sizeof(ll));
    f[0] = 1;
    for (int k = 1; k <= n; k++)
        f[k] = f[k - 1] * k % p;

    ll fact = f[n];
    ll total = fact * fact % p;

    /* Compute harmonic-like sum with modular inverses */
    ll h = 0;
    for (int k = 1; k <= n; k++)
        h = (h + power_mod(k, p - 2, p)) % p;

    /* Compute coefficients */
    ll f_n2 = (n >= 2) ? f[n - 2] : 1;
    ll c1 = fact % p * f_n2 % p * n % p * ((h - 1 + p) % p) % p;
    ll c2_inner = (ll)n * ((ll)(n + 1) % p) % p * inv2 % p;
    c2_inner = c2_inner * ((n - h + p) % p) % p;
    ll c2 = fact % p * f_n2 % p * n % p * c2_inner % p;

    /* Wait, the Python code is:
       c1 = fact * f_n2 * n * (h - 1 + p) % p
       c2 = fact * f_n2 * n * ((n + 1) * inv2 % p * (n - h + p) % p) % p

       Python % precedence differs from C. Let me carefully re-read the Python:
       c1 = fact * f_n2 * n * (h - 1 + p) % p
       In Python, * binds tighter than %, so this is (fact * f_n2 * n * (h - 1 + p)) % p
       c2 = fact * f_n2 * n * ((n + 1) * inv2 % p * (n - h + p) % p) % p
       Inner: ((n + 1) * inv2 % p * (n - h + p) % p)
       = ((((n + 1) * inv2) % p) * ((n - h + p) % p)) ... but then the outer % p...
       Actually Python evaluates left to right with equal precedence for * and %:
       (n + 1) * inv2 % p * (n - h + p) % p
       = (((n + 1) * inv2) % p * (n - h + p)) % p
    */

    /* Redo carefully */
    c1 = fact * f_n2 % p;
    c1 = c1 * n % p;
    c1 = c1 * ((h - 1 + p) % p) % p;

    ll inner2 = (ll)(n + 1) % p * inv2 % p;
    inner2 = inner2 * ((n - h + p) % p) % p;
    c2 = fact * f_n2 % p;
    c2 = c2 * n % p;
    c2 = c2 * inner2 % p;

    /* Compute sums */
    ll sum_f = 0;
    for (int k = 0; k < n; k++)
        sum_f = (sum_f + f[k]) % p;
    ll sum_kf = (f[n] - 1 + p) % p;

    ll sum1 = sum_f;
    ll sum2 = ((ll)n % p * sum_f % p - sum_kf + p) % p;
    ll sum3 = ((ll)(n - 1) % p * sum_f % p - sum_kf + p) % p;

    /* Final computation */
    /* q = (total + c1 * sum2 + c2 * sum1 - total * sum1 - inv2 * total * sum3) % p */
    ll q = total;
    q = (q + c1 * sum2 % p) % p;
    q = (q + c2 * sum1 % p) % p;
    q = (q - total * sum1 % p + p) % p;
    q = (q - inv2 * total % p * sum3 % p + p) % p;

    printf("%lld\n", q);

    free(f);
    return 0;
}
