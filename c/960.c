/* Project Euler 960 - Stone Pile Game
 * F(n) = (n-1)!/2 * sum_{k=1}^{n-1} [C(n,k) * k^(k-1) * (n-k)^(n-k-1) * min(k, n-k)]
 * Find F(100) mod 10^9+7
 */
#include <stdio.h>

typedef long long ll;
#define MOD 1000000007LL

ll power(ll base, ll exp, ll mod) {
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
    int n = 100;
    ll fact[101], inv_fact[101];
    fact[0] = 1;
    for (int i = 1; i <= n; i++)
        fact[i] = fact[i-1] * i % MOD;
    inv_fact[n] = power(fact[n], MOD - 2, MOD);
    for (int i = n - 1; i >= 0; i--)
        inv_fact[i] = inv_fact[i+1] * (i+1) % MOD;

    ll total_sum = 0;
    for (int k = 1; k < n; k++) {
        ll term = fact[n] % MOD;
        term = term * inv_fact[k] % MOD;
        term = term * inv_fact[n-k] % MOD;
        term = term * power(k, k-1, MOD) % MOD;
        term = term * power(n-k, n-k-1, MOD) % MOD;
        int mk = k < n-k ? k : n-k;
        term = term * mk % MOD;
        total_sum = (total_sum + term) % MOD;
    }

    ll result = fact[n-1] % MOD;
    result = result * power(2, MOD - 2, MOD) % MOD;
    result = result * total_sum % MOD;

    printf("%lld\n", result);
    return 0;
}
