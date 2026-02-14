/*
 * Project Euler 478 - Mixtures
 *
 * E(n) = number of subsets of M(n) that can produce mixture (1:1:1).
 * M(n) = set of all (a:b:c) with 0 <= a,b,c <= n and gcd(a,b,c) = 1.
 *
 * Algorithm based on roosephu's approach:
 * - Count total = |M(n)| using Mobius inversion
 * - Use half-plane decomposition to subtract bad subsets
 * - Each direction parameterized by b, with 6*phi(b) symmetries
 * - F(b,n) counts points in each half-plane
 *
 * MOD = 11^8 = 214358881
 * PHI_MOD = phi(MOD) * 2 = 11^7 * 10 * 2 = 389743420
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef long long ll;
typedef __int128 lll;

#define N_MAX 10000000
#define MOD 214358881LL
#define PHI_MOD 389743420LL   /* 2 * phi(11^8) = 2 * 11^7 * 10 */

static int *primes;
static int nprimes;
static int mu[N_MAX + 1];
static int phi_arr[N_MAX + 1];
static ll mertens_arr[N_MAX + 1];
static char composite[N_MAX + 1];

void sieve(void) {
    /* Linear sieve for primes, mu, phi */
    memset(composite, 0, sizeof(composite));
    primes = (int *)malloc((size_t)(N_MAX + 1) * sizeof(int));
    nprimes = 0;

    mu[1] = 1;
    phi_arr[1] = 1;

    for (int i = 2; i <= N_MAX; i++) {
        if (!composite[i]) {
            primes[nprimes++] = i;
            mu[i] = -1;
            phi_arr[i] = i - 1;
        }
        for (int j = 0; j < nprimes; j++) {
            ll t = (ll)i * primes[j];
            if (t > N_MAX) break;
            composite[(int)t] = 1;
            if (i % primes[j] == 0) {
                mu[(int)t] = 0;
                phi_arr[(int)t] = phi_arr[i] * primes[j];
                break;
            } else {
                mu[(int)t] = -mu[i];
                phi_arr[(int)t] = phi_arr[i] * (primes[j] - 1);
            }
        }
    }

    /* Mertens function (prefix sums of mu) */
    mertens_arr[0] = 0;
    for (int i = 1; i <= N_MAX; i++)
        mertens_arr[i] = mertens_arr[i - 1] + mu[i];
}

ll pow_mod(ll base, ll exp, ll mod) {
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

/*
 * F(s, n): count lattice points in a half-plane for direction parameterized by s.
 * This computes sum_{d coprime to something} floor(n/(s*d)) * ... using Mobius.
 * The formula from roosephu:
 *   For each i, compute quotient blocks where n/(i*s) is constant.
 *   Within each block, sum (2*t + 2 - s*(1+d)) * d / 2 where t = n/i, d = t/s.
 *   Weight by mertens difference.
 */
ll F(int s, int n) {
    ll ret = 0;
    for (ll i = 1; i * s <= n; ) {
        ll j = (ll)n / ((ll)n / i) + 1;
        ll t = (ll)n / i;
        ll d = t / s;
        ll g = ((2 * t % PHI_MOD + 2 - (ll)s * (1 + d) % PHI_MOD + 2 * PHI_MOD) % PHI_MOD) * (d % PHI_MOD) % PHI_MOD;
        /* divide by 2 */
        if (g % 2 == 0) g /= 2;
        else g = (g + PHI_MOD) / 2;
        ret += g * ((mertens_arr[j - 1 > N_MAX ? N_MAX : (int)(j - 1)] - mertens_arr[(int)(i - 1)]) % PHI_MOD) % PHI_MOD;
        ret %= PHI_MOD;
        i = j;
    }
    return (ret % PHI_MOD + PHI_MOD) % PHI_MOD;
}

int main(void) {
    int n = N_MAX;

    sieve();

    /* Compute total = |M(n)| = sum_{d=1}^n mu(d) * ((floor(n/d)+1)^3 - 1) mod PHI_MOD */
    ll total = 0;
    for (int i = 1; i <= n; i++) {
        if (mu[i] == 0) continue;
        ll x = (ll)n / i + 1;
        ll cube = (x % PHI_MOD) * (x % PHI_MOD) % PHI_MOD * (x % PHI_MOD) % PHI_MOD;
        cube = (cube - 1 + PHI_MOD) % PHI_MOD;
        total = (total + mu[i] * cube % PHI_MOD + PHI_MOD) % PHI_MOD;
    }
    total = (total % PHI_MOD + PHI_MOD) % PHI_MOD;

    ll ans = pow_mod(2, total, MOD);

    /* Subtract bad subsets: for each direction b, there are 6*phi(b) half-planes */
    for (int b = 1; b <= n; b++) {
        ll M = (6LL * phi_arr[b]) % MOD;
        ll f = F(b, n);
        /* Number of bad subsets in this direction:
         * 2^((total-1)/2) - 2^((total-1)/2 - f)
         * This counts subsets entirely on one side of the separating line */
        ll half = (total - 1 + PHI_MOD) % PHI_MOD;
        /* Since total is odd (|M(n)| includes (1,1,1) point and its complement),
         * (total-1)/2 is exact */
        half = half % 2 == 0 ? half / 2 : (half + PHI_MOD) / 2;
        ll term1 = pow_mod(2, half, MOD);
        ll term2 = pow_mod(2, (half - f + PHI_MOD) % PHI_MOD, MOD);
        ll diff = (term1 - term2 + MOD) % MOD;
        ans = (ans - M * diff % MOD + MOD) % MOD;
    }

    ans = (ans - 1 + MOD) % MOD;
    printf("%lld\n", ans);

    free(primes);
    return 0;
}
