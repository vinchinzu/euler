/*
 * Project Euler 457 - A square on the hypotenuse
 *
 * f(n) = n^2 - 3n - 1, R(p) = smallest positive n with f(n) = 0 (mod p^2).
 * Sum R(p) for all primes p <= N = 10^7.
 *
 * Uses Tonelli-Shanks and Hensel lifting.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>
#include <math.h>

typedef long long ll;
typedef __int128 lll;

#define MAXN 10000001

static char is_prime[MAXN];

static ll pow_mod(ll base, ll exp, ll mod) {
    ll result = 1;
    base %= mod;
    if (base < 0) base += mod;
    while (exp > 0) {
        if (exp & 1) result = (lll)result * base % mod;
        base = (lll)base * base % mod;
        exp >>= 1;
    }
    return result;
}

static ll sqrt_mod(ll a, ll p) {
    a %= p;
    if (a < 0) a += p;
    if (a == 0) return 0;
    if (p == 2) return a % 2;

    /* Check if QR */
    if (pow_mod(a, (p - 1) / 2, p) != 1) return -1;

    ll Q = p - 1;
    int S = 0;
    while (Q % 2 == 0) { Q /= 2; S++; }

    ll z = 2;
    while (pow_mod(z, (p - 1) / 2, p) != p - 1) z++;

    int M = S;
    ll c = pow_mod(z, Q, p);
    ll t = pow_mod(a, Q, p);
    ll R = pow_mod(a, (Q + 1) / 2, p);

    while (t != 1) {
        ll tt = t;
        int i = 0;
        while (i < M && tt != 1) {
            tt = (lll)tt * tt % p;
            i++;
        }
        ll b = pow_mod(c, 1LL << (M - i - 1), p);
        M = i;
        c = (lll)b * b % p;
        t = (lll)t * c % p;
        R = (lll)R * b % p;
    }

    return R;
}

static ll mod_inv(ll a, ll m) {
    a %= m;
    if (a < 0) a += m;
    ll t = 0, new_t = 1;
    ll r = m, new_r = a;
    while (new_r != 0) {
        ll q = r / new_r;
        ll tmp;
        tmp = new_t; new_t = t - q * new_t; t = tmp;
        tmp = new_r; new_r = r - q * new_r; r = tmp;
    }
    if (t < 0) t += m;
    return t;
}

static ll compute_R(ll p) {
    if (p <= 13) {
        ll p2 = p * p;
        for (ll n = 1; n <= p2; n++) {
            ll fn = n * n - 3 * n - 1;
            fn = ((fn % p2) + p2) % p2;
            if (fn == 0) return n;
        }
        return 0;
    }

    ll sv = sqrt_mod(13, p);
    if (sv < 0) return 0;

    /* inv2 = (p+1)/2 */
    ll inv2 = (p + 1) / 2;
    ll R_min = (ll)9e18;

    for (int sign = -1; sign <= 1; sign += 2) {
        ll n_val = ((3 + sign * sv) % p + p) % p * inv2 % p;
        /* f(n_val) should be 0 mod p; now Hensel lift to mod p^2 */
        ll fn = ((lll)n_val * n_val - 3 * n_val - 1) % p;
        fn = (fn + p) % p;
        /* fn should be 0 */
        ll fn_full = (lll)n_val * n_val - 3 * n_val - 1;
        ll k_num = fn_full / p;  /* this might be negative */
        ll deriv = (2 * n_val - 3) % p;
        deriv = (deriv + p) % p;
        ll inv_deriv = mod_inv(deriv, p);
        ll k = ((-k_num % p + p) % p * inv_deriv) % p;
        ll candidate = k * p + n_val;
        if (candidate > 0 && candidate < R_min)
            R_min = candidate;
    }

    return (R_min < (ll)9e18) ? R_min : 0;
}

int main(void) {
    int N = 10000000;

    memset(is_prime, 1, sizeof(is_prime));
    is_prime[0] = is_prime[1] = 0;
    for (int i = 2; (ll)i * i <= N; i++) {
        if (is_prime[i]) {
            for (int j = i * i; j <= N; j += i)
                is_prime[j] = 0;
        }
    }

    ll ans = 0;
    for (int p = 2; p <= N; p++) {
        if (is_prime[p]) {
            ans += compute_R(p);
        }
    }

    printf("%lld\n", ans);
    return 0;
}
