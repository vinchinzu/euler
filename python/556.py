"""Project Euler Problem 556: Squarefree Gaussian integers.

Find the number of proper squarefree Gaussian integers a+bi with a^2+b^2<=N.

Let f(n) be the number of proper squarefree Gaussian integers with a^2+b^2<=|n|.
Then sum_z f(n/z^2) over all Gaussian integers z is exactly the total number of
proper Gaussian integers with a^2+b^2<=|n|. The latter can be computed directly
as sum_a floor(sqrt(n-a^2))+1, so this gives a recursive way to compute f(n).

Uses embedded C for performance.
"""
import subprocess, os, tempfile


def solve():
    c_code = r"""
#include <stdio.h>
#include <stdlib.h>
#include <math.h>
#include <string.h>

typedef long long ll;

/* ---- Integer square root ---- */
static ll isqrt_ll(ll n) {
    if (n <= 0) return 0;
    ll r = (ll)sqrt((double)n);
    while (r > 0 && r * r > n) r--;
    while ((r + 1) * (r + 1) <= n) r++;
    return r;
}

/* ---- Hash table for memoization ---- */
/* Open-addressing hash table with linear probing */
#define HT_SIZE (1 << 22)  /* ~4M entries, must be power of 2 */
#define HT_MASK (HT_SIZE - 1)
#define HT_EMPTY (-1LL)

static ll ht_keys[1 << 22];
static ll ht_vals[1 << 22];

static void ht_init(void) {
    memset(ht_keys, 0xFF, sizeof(ht_keys)); /* sets all to -1 (0xFFFF...) */
}

static inline unsigned int ht_hash(ll key) {
    /* Fibonacci hashing */
    unsigned long long h = (unsigned long long)key * 11400714819323198485ULL;
    return (unsigned int)(h >> (64 - 22));
}

static ll ht_get(ll key) {
    unsigned int idx = ht_hash(key);
    for (;;) {
        idx &= HT_MASK;
        if (ht_keys[idx] == key) return ht_vals[idx];
        if (ht_keys[idx] == HT_EMPTY) return HT_EMPTY;
        idx++;
    }
}

static void ht_set(ll key, ll val) {
    unsigned int idx = ht_hash(key);
    for (;;) {
        idx &= HT_MASK;
        if (ht_keys[idx] == HT_EMPTY || ht_keys[idx] == key) {
            ht_keys[idx] = key;
            ht_vals[idx] = val;
            return;
        }
        idx++;
    }
}

/* ---- Count proper Gaussian integers with a^2 + b^2 <= n ---- */
/* These are (a, b) with a >= 1, b >= 0, a^2 + b^2 <= n */
static ll count_gauss(ll n) {
    if (n <= 0) return 0;
    ll total = 0;
    ll sq = isqrt_ll(n);
    for (ll a = 1; a <= sq; a++) {
        ll rem = n - a * a;
        if (rem < 0) break;
        total += isqrt_ll(rem) + 1;
    }
    return total;
}

/* ---- Recursive function f(n) with memoization ---- */
static ll f(ll n) {
    if (n == 0) return 0;

    ll cached = ht_get(n);
    if (cached != HT_EMPTY) return cached;

    ll result = count_gauss(n);

    /* Subtract f(n / |z|^4) for each Gaussian z with |z|^2 >= 2 */
    ll sn = isqrt_ll(n);
    ll fourth = isqrt_ll(sn);

    for (ll a = 1; a <= fourth; a++) {
        ll a_sq = a * a;
        if (a_sq * a_sq > n) break;

        ll max_b_sq = sn - a_sq;
        if (max_b_sq < 0) break;
        ll max_b = isqrt_ll(max_b_sq);

        for (ll b = 0; b <= max_b; b++) {
            ll z_norm_sq = a_sq + b * b;
            if (z_norm_sq <= 1) continue;
            ll z4 = z_norm_sq * z_norm_sq;
            if (z4 > n) break;

            ll arg = n / z4;
            if (arg == 0) continue;

            result -= f(arg);
        }
    }

    ht_set(n, result);
    return result;
}

int main(void) {
    ll N = 100000000000000LL; /* 10^14 */
    ht_init();
    printf("%lld\n", f(N));
    return 0;
}
"""
    tmpdir = tempfile.mkdtemp()
    src = os.path.join(tmpdir, "sol556.c")
    exe = os.path.join(tmpdir, "sol556")
    with open(src, 'w') as f:
        f.write(c_code)
    subprocess.run(["gcc", "-O2", "-o", exe, src, "-lm"], check=True, capture_output=True)
    result = subprocess.run([exe], capture_output=True, text=True, check=True, timeout=280)
    print(result.stdout.strip())


if __name__ == "__main__":
    solve()
