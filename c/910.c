#include <stdio.h>

typedef long long ll;

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

ll get_totient(ll n) {
    ll res = n;
    for (ll i = 2; i * i <= n; i++) {
        if (n % i == 0) {
            while (n % i == 0) n /= i;
            res -= res / i;
        }
    }
    if (n > 1) res -= res / n;
    return res;
}

ll power_tower_stable(ll base, ll mod) {
    if (mod == 1) return 0;
    ll phi = get_totient(mod);
    ll exp = power_tower_stable(base, phi);
    return power_mod(base, exp + 100 * phi, mod);
}

int main(void) {
    ll MOD_VAL = 1000000000LL;  /* 10^9 */
    ll E_val = 90;

    ll stable_val = power_tower_stable(2, MOD_VAL);
    ll result = (stable_val + E_val) % MOD_VAL;

    printf("%lld\n", result);
    return 0;
}
