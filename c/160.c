/*
 * Project Euler 160 - Factorial trailing digits
 *
 * Compute last 5 non-zero digits of (10^12)!
 * Uses CRT: compute mod 32 and mod 3125 separately.
 */
#include <stdio.h>

typedef long long ll;
typedef unsigned long long ull;

static ll mod_pow(ll base, ll exp, ll mod) {
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

static ll extended_gcd(ll a, ll b, ll *x, ll *y) {
    if (a == 0) { *x = 0; *y = 1; return b; }
    ll x1, y1;
    ll g = extended_gcd(b % a, a, &x1, &y1);
    *x = y1 - (b / a) * x1;
    *y = x1;
    return g;
}

static ll mod_inverse(ll a, ll m) {
    ll x, y;
    extended_gcd(a, m, &x, &y);
    return ((x % m) + m) % m;
}

static ll prime_exponent(ll p, ll n) {
    ll exp = 0;
    ll power = p;
    while (power <= n) {
        exp += n / power;
        if (power > n / p) break;
        power *= p;
    }
    return exp;
}

/* Compute (n! / p^{v_p(n!)}) mod p^k recursively */
static ll factorial_p_free(ll n, ll p, ll pk) {
    if (n == 0) return 1;

    /* Product of numbers in [1, pk) not divisible by p */
    ll prod_cycle = 1;
    for (ll i = 1; i < pk; i++) {
        if (i % p != 0) {
            prod_cycle = prod_cycle * i % pk;
        }
    }

    ll res = mod_pow(prod_cycle, n / pk, pk);

    /* Remaining terms */
    ll remainder = n % pk;
    for (ll i = 1; i <= remainder; i++) {
        if (i % p != 0) {
            res = res * i % pk;
        }
    }

    /* Recurse for terms divisible by p */
    res = res * factorial_p_free(n / p, p, pk) % pk;
    return res;
}

static ll crt(ll a1, ll m1, ll a2, ll m2) {
    ll inv = mod_inverse(m1, m2);
    ll u = ((a2 - a1) % m2 + m2) % m2;
    u = u * inv % m2;
    return (a1 + m1 * u) % (m1 * m2);
}

int main(void) {
    ll N = 1000000000000LL; /* 10^12 */
    ll MOD2 = 32;   /* 2^5 */
    ll MOD5 = 3125; /* 5^5 */

    ll exp2 = prime_exponent(2, N);
    ll exp5 = prime_exponent(5, N);

    /* Compute mod 3125 */
    ll term_five_free = factorial_p_free(N, 5, 3125);
    ll term_two_inv = mod_inverse(mod_pow(2, exp2, 3125), 3125);
    ll m_mod_3125 = term_five_free * term_two_inv % 3125;
    ll res_mod_3125 = m_mod_3125 * mod_pow(2, exp2 - exp5, 3125) % 3125;

    /* Compute mod 32: the Python code sets res_mod_32 = 0, which seems to be an oversight.
     * Actually looking carefully, the factorial_nonzero_last_five returns crt(0, 32, res_mod_3125, 3125).
     * This is because for N >= 32, the product of all odd numbers not divisible by 5 mod 32
     * cycles, and the net result with the excess 2s works out.
     * Let's replicate: res_mod_32 = 0 as in Python. */
    ll res_mod_32 = 0;

    ll result = crt(res_mod_32, MOD2, res_mod_3125, MOD5);

    printf("%05lld\n", result);
    return 0;
}
