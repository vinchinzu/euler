/* Project Euler 731: A Stoneham Number.
 * Compute digits of Stoneham number sum 1/(3^i * 10^(3^i)).
 * Uses modular arithmetic with mod = 10^12 (K+2 digits).
 *
 * We need arbitrary precision modular exponentiation with very large moduli
 * (3^i can be huge, up to 3^34 ~ 10^16). We use __int128 for intermediate results.
 */
#include <stdio.h>
#include <stdint.h>

typedef long long ll;
typedef unsigned long long ull;
typedef __int128 lll;

/* We need modular exponentiation where mod can be up to ~10^16 */
ull pow_mod_ull(ull base, ull exp, ull mod) {
    if (mod == 1) return 0;
    ull result = 1;
    base %= mod;
    while (exp > 0) {
        if (exp & 1) result = (unsigned __int128)result * base % mod;
        base = (unsigned __int128)base * base % mod;
        exp >>= 1;
    }
    return result;
}

/* Extended GCD */
ll ext_gcd(ll a, ll b, ll *x, ll *y) {
    if (a == 0) { *x = 0; *y = 1; return b; }
    ll x1, y1;
    ll g = ext_gcd(b % a, a, &x1, &y1);
    *x = y1 - (b / a) * x1;
    *y = x1;
    return g;
}

ll mod_inv_ll(ll a, ll m) {
    ll x, y;
    ext_gcd(a % m, m, &x, &y);
    return ((x % m) + m) % m;
}

int main() {
    ull n = 10000000000000000ULL; /* 10^16 */
    int k = 10;
    ull m = 1000000000000ULL; /* 10^(k+2) = 10^12 */

    ull ans = 0;

    /* Compute 3^i iteratively and check 3^i < n */
    ull three_power = 3; /* 3^1 */
    int i = 1;
    while (three_power < n) {
        ull exp = n + k + 1 - three_power;
        /* term = (10^exp mod m - 10^exp mod 3^i) * inv(3^i, m) mod m */
        /* But we need 3^i for modular ops */
        ull term1 = pow_mod_ull(10, exp, m);
        ull term2 = pow_mod_ull(10, exp, three_power);

        /* inv_three_i = modular inverse of three_power mod m */
        ll inv_three_i = mod_inv_ll((ll)(three_power % m), (ll)m);

        /* (term1 - term2) might be negative in unsigned, handle carefully */
        ll diff = (ll)term1 - (ll)term2;
        ll contribution = (lll)diff * inv_three_i % (ll)m;
        contribution = ((contribution % (ll)m) + (ll)m) % (ll)m;

        ans = (ans + (ull)contribution) % m;

        /* Next 3^i -- check for overflow */
        if (three_power > n / 3) break; /* would overflow */
        three_power *= 3;
        i++;
    }

    /* Extract first k digits from ans (which has k+2 digits) */
    /* ans is mod 10^12, we want first 10 digits = ans / 100 */
    ull result = ans / 100; /* divide by 10^2 to get first k digits */

    printf("%llu\n", result);
    return 0;
}
