/*
 * Project Euler Problem 474: Last digits of divisors
 *
 * Count divisors of N! whose last 5 digits equal K=65432.
 * N=10^6, M=10^16+61.
 *
 * For primes coprime to 10, product of (1 + v_p(N!)) gives total count
 * of divisors coprime to {2,5}. Then divide by the number of residue
 * classes modulo lcm(K, 10^ceil_log10(K))/K's Euler totient.
 */
#include <stdio.h>
#include <math.h>

typedef long long ll;
typedef unsigned long long ull;
typedef __int128 i128;

#define NN 1000000
#define K_VAL 65432
#define M_VAL 10000000000000061LL
#define B 10

static int is_prime[NN + 1];

void sieve(void) {
    for (int i = 2; i <= NN; i++) is_prime[i] = 1;
    for (int i = 2; (ll)i * i <= NN; i++) {
        if (is_prime[i]) {
            for (int j = i * i; j <= NN; j += i)
                is_prime[j] = 0;
        }
    }
}

ll num_factors_in_factorial(int n, int p) {
    ll count = 0;
    ll power = p;
    while (power <= n) {
        count += n / power;
        if (power > n / p) break; /* prevent overflow */
        power *= p;
    }
    return count;
}

int gcd_int(int a, int b) {
    while (b) { int t = b; b = a % b; a = t; }
    return a;
}

ll gcd_ll(ll a, ll b) {
    while (b) { ll t = b; b = a % b; a = t; }
    return a;
}

ll lcm_ll(ll a, ll b) {
    return a / gcd_ll(a, b) * b;
}

ll iceil_pow(int n, int base) {
    ll result = 1;
    while (result < n) result *= base;
    return result;
}

ll euler_phi(ll n) {
    ll result = n;
    ll p = 2;
    ll tmp = n;
    while (p * p <= tmp) {
        if (tmp % p == 0) {
            while (tmp % p == 0) tmp /= p;
            result = result / p * (p - 1);
        }
        p++;
    }
    if (tmp > 1) result = result / tmp * (tmp - 1);
    return result;
}

ll mod_inv(ll a, ll m) {
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

int main(void) {
    sieve();

    ll res = 1;
    ll mod = M_VAL;

    for (int p = 2; p <= NN; p++) {
        if (is_prime[p] && gcd_int(p, B) == 1) {
            ll v = num_factors_in_factorial(NN, p);
            ll factor = (1 + v) % mod;
            res = (i128)res * factor % mod;
        }
    }

    ll cp = iceil_pow(K_VAL, B);
    ll lc = lcm_ll((ll)K_VAL, cp);
    ll r = euler_phi(lc / K_VAL);
    ll ans = (i128)res % mod * mod_inv(r, mod) % mod;

    printf("%lld\n", ans);
    return 0;
}
