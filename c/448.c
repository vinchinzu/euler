/*
 * Project Euler 448 - Average least common multiple
 *
 * Extracted from embedded C in python/448.py.
 * S(N) = (N + sum_{k=1}^N floor(N/k) * k * phi(k)) / 2
 */
#include <stdio.h>
#include <stdint.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

typedef long long ll;
typedef unsigned long long ull;
typedef __int128 lll;

const ll N = 99999999019LL;
const ll MOD = 999999017LL;
int L;
int threshold;

int *phi;
ll *k_phi_sum_small;

ll mod_inv(ll a, ll m) {
    ll g = m, x = 0, y = 1;
    while (a != 0) {
        ll q = g / a;
        ll t = g - q * a; g = a; a = t;
        t = x - q * y; x = y; y = t;
    }
    return (x % m + m) % m;
}

ll inv6;

ll sum_sq(ll m) {
    lll mm = m % MOD;
    return (lll)mm * (mm + 1) % MOD * (2 * mm + 1) % MOD * inv6 % MOD;
}

#define HASH_SIZE 1000003
typedef struct {
    ll key;
    ll val;
    int used;
} HashEntry;
HashEntry *cache;

ll hash_get(ll key, int *found) {
    ull h = (ull)key % HASH_SIZE;
    while (cache[h].used) {
        if (cache[h].key == key) {
            *found = 1;
            return cache[h].val;
        }
        h = (h + 1) % HASH_SIZE;
    }
    *found = 0;
    return 0;
}

void hash_set(ll key, ll val) {
    ull h = (ull)key % HASH_SIZE;
    while (cache[h].used) {
        if (cache[h].key == key) {
            cache[h].val = val;
            return;
        }
        h = (h + 1) % HASH_SIZE;
    }
    cache[h].key = key;
    cache[h].val = val;
    cache[h].used = 1;
}

ll k_phi_sum(ll n) {
    if (n <= 0) return 0;
    if (n <= L) return k_phi_sum_small[n];

    int found;
    ll cached = hash_get(n, &found);
    if (found) return cached;

    ll result = sum_sq(n);
    ll sqrt_n = (ll)sqrtl((long double)n);
    while ((sqrt_n + 1) * (sqrt_n + 1) <= n) sqrt_n++;
    while (sqrt_n * sqrt_n > n) sqrt_n--;

    for (ll d = 2; d <= sqrt_n; d++) {
        result = (result - k_phi_sum(n / d) * (d % MOD)) % MOD;
    }

    for (ll q = 1; q <= sqrt_n; q++) {
        if (n / q > sqrt_n) {
            ll d_lo = n / (q + 1) + 1;
            ll d_hi = n / q;
            lll sum_d = ((lll)d_hi * (d_hi + 1) / 2 - (lll)d_lo * (d_lo - 1) / 2) % MOD;
            if (sum_d < 0) sum_d += MOD;
            result = (result - k_phi_sum(q) * (ll)sum_d) % MOD;
        }
    }

    if (result < 0) result += MOD;
    hash_set(n, result);
    return result;
}

int main() {
    L = (int)sqrtl((long double)N) + 10;
    threshold = N / L;

    inv6 = mod_inv(6, MOD);

    phi = (int *)malloc((L + 1) * sizeof(int));
    for (int i = 0; i <= L; i++) phi[i] = i;
    for (int i = 2; i <= L; i++) {
        if (phi[i] == i) {
            for (int j = i; j <= L; j += i) {
                phi[j] -= phi[j] / i;
            }
        }
    }

    k_phi_sum_small = (ll *)malloc((L + 1) * sizeof(ll));
    k_phi_sum_small[0] = 0;
    for (int k = 1; k <= L; k++) {
        k_phi_sum_small[k] = (k_phi_sum_small[k - 1] + (ll)k * phi[k]) % MOD;
    }

    cache = (HashEntry *)calloc(HASH_SIZE, sizeof(HashEntry));

    ll ans = 0;

    for (int k = 1; k <= threshold; k++) {
        ll term = (ll)((N / k) % MOD) * (k % MOD) % MOD * phi[k] % MOD;
        ans = (ans + term) % MOD;
    }

    for (int q = 1; q < L; q++) {
        ll t1 = k_phi_sum(N / q);
        ll t2 = k_phi_sum(N / (q + 1));
        ll diff = (t1 - t2) % MOD;
        if (diff < 0) diff += MOD;
        ans = (ans + diff * (q % MOD)) % MOD;
    }

    ans = (ans + N % MOD) % MOD;
    ll inv2 = mod_inv(2, MOD);
    ans = ans * inv2 % MOD;

    printf("%lld\n", ans);

    free(phi);
    free(k_phi_sum_small);
    free(cache);
    return 0;
}
