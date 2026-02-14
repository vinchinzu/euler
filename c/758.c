/* Project Euler Problem 758: Buckets of Water.
 * Translated from python/758.py
 *
 * For primes p < q < 1000, compute P(2^{p^5}-1, 2^{q^5}-1)
 * using extended Euclidean algorithm on exponents.
 */
#include <stdio.h>
#include <math.h>

typedef long long ll;
typedef __int128 lll;

#define MOD 1000000007LL
#define MAXN 1001

static int primes[200];
static int nprimes;

void sieve(void) {
    char is_prime[MAXN];
    for (int i = 0; i < MAXN; i++) is_prime[i] = 1;
    is_prime[0] = is_prime[1] = 0;
    int sq = (int)sqrt((double)MAXN);
    for (int i = 2; i <= sq; i++) {
        if (is_prime[i]) {
            for (int j = i * i; j < MAXN; j += i)
                is_prime[j] = 0;
        }
    }
    nprimes = 0;
    for (int i = 2; i < MAXN; i++) {
        if (is_prime[i]) primes[nprimes++] = i;
    }
}

ll pow_mod(ll base, ll exp, ll mod) {
    ll result = 1;
    base %= mod;
    if (base < 0) base += mod;
    while (exp > 0) {
        if (exp & 1) result = (lll)result * base % mod;
        exp >>= 1;
        base = (lll)base * base % mod;
    }
    return result;
}

ll mod_inv(ll a, ll mod) {
    return pow_mod(a, mod - 2, mod);
}

/* Extended Euclidean algorithm for near powers of 2.
 * Returns (x, y, sign) such that (2^e - 1)*x + (2^f - 1)*y = 1
 * with x, y computed mod M.
 * sign indicates which of x-y-1 or y-x-1 is the answer. */
typedef struct {
    ll x, y;
    int sign;
} Result;

Result lin_comb(ll e, ll f, ll M) {
    if (f == 0) {
        return (Result){1, 0, 1};
    }

    Result prev = lin_comb(f, e % f, M);

    /* quotient = (2^e - 2^{e%f}) / (2^f - 1) mod M */
    ll num = (pow_mod(2, e, M) - pow_mod(2, e % f, M) + M) % M;
    ll den = (pow_mod(2, f, M) - 1 + M) % M;
    ll quotient = (lll)num * mod_inv(den, M) % M;

    ll new_y = (prev.x - (lll)quotient * prev.y % M + M) % M;
    return (Result){prev.y, new_y, -prev.sign};
}

int main(void) {
    sieve();
    ll ans = 0;

    for (int i = 0; i < nprimes; i++) {
        ll p = primes[i];
        ll e = p * p * p * p * p; /* p^5 */
        for (int j = i + 1; j < nprimes; j++) {
            ll q = primes[j];
            ll f = q * q * q * q * q; /* q^5 */
            Result r = lin_comb(e, f, MOD);
            ll k = ((lll)r.sign * (r.x - r.y + MOD) % MOD - 1 + MOD) % MOD;
            ans = (ans + 2 * k) % MOD;
        }
    }

    printf("%lld\n", ans);
    return 0;
}
