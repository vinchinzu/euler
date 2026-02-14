/*
 * Project Euler Problem 312: Cyclic paths on Sierpinski graphs
 *
 * Find C(C(C(10000))) mod 13^8, where C(n) = 2^(3^(n-2)) * 3^((3^(n-2)-3)/2)
 * is the number of Hamiltonian cycles in Sierpinski graph of order n.
 *
 * Uses Euler's theorem for modular reduction of tower exponents.
 */
#include <stdio.h>

typedef long long ll;
typedef unsigned __int128 u128;

static ll euler_phi(ll n) {
    ll result = n;
    for (ll p = 2; p * p <= n; p++) {
        if (n % p == 0) {
            while (n % p == 0) n /= p;
            result -= result / p;
        }
    }
    if (n > 1) result -= result / n;
    return result;
}

static ll pow_mod(ll base, ll exp, ll mod) {
    if (mod == 1) return 0;
    ll result = 1;
    base %= mod;
    if (base < 0) base += mod;
    while (exp > 0) {
        if (exp & 1) result = (u128)result * base % mod;
        base = (u128)base * base % mod;
        exp >>= 1;
    }
    return result;
}

static int N = 10000;

static ll Ck(ll mod, int k);

static ll Ck(ll mod, int k) {
    ll mod1 = 2 * euler_phi(mod);
    ll mod2 = euler_phi(mod1);

    ll n;
    if (k == 1)
        n = N;
    else
        n = Ck(mod2, k - 1);

    /* C(n) = 2^(3^(n-2)) * 3^((3^(n-2)-3)/2) */
    ll exp1 = pow_mod(3, n - 2, mod1);
    ll term1 = pow_mod(2, exp1, mod);
    ll exp2 = (exp1 - 3) / 2;
    ll term2 = pow_mod(3, exp2, mod);

    return (u128)term1 * term2 % mod;
}

int main(void) {
    /* 13^8 */
    ll M = 1;
    for (int i = 0; i < 8; i++) M *= 13;

    ll result = Ck(M, 3);
    printf("%lld\n", result);
    return 0;
}
