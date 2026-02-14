#!/usr/bin/env python3
"""Project Euler Problem 791 - Average and Variance.

S(n) = sum of (a+b+c+d) over ordered quadruples 1 <= a <= b <= c <= d <= n
where average equals twice the variance.

Embedded C implementation for speed. O(sqrt(N)) algorithm.
"""

import subprocess, tempfile, os, sys

C_CODE = r"""
#include <stdio.h>
#include <math.h>

typedef long long ll;
typedef unsigned long long ull;

static const ll MOD = 433494437LL;
static const ll N = 100000000LL;

static ll inv3, inv6;

static ll mod(ll x) {
    x %= MOD;
    if (x < 0) x += MOD;
    return x;
}

static ll mulmod(ll a, ll b) {
    return (a % MOD) * (b % MOD) % MOD;
}

static ll powmod(ll base, ll exp, ll m) {
    ll result = 1;
    base %= m;
    while (exp > 0) {
        if (exp & 1) result = result * base % m;
        base = base * base % m;
        exp >>= 1;
    }
    return result;
}

static ll isqrt_ll(ll n) {
    if (n < 0) return -1;
    ll r = (ll)sqrt((double)n);
    /* correct rounding errors */
    while (r * r > n) r--;
    while ((r + 1) * (r + 1) <= n) r++;
    return r;
}

static ll closed_sum(ll g, ll H) {
    if (H < 0) return 0;
    ll gm = g % MOD;
    ll Hm = H % MOD;
    /* t1 = 2 * g^2 * (H+1)^2 */
    ll t1 = 2 * gm % MOD * gm % MOD;
    ll hp1 = (Hm + 1) % MOD;
    t1 = t1 * hp1 % MOD * hp1 % MOD;
    /* t2 = H*(H+1)*(2H+1)*(2H+3)/3 */
    ll t2 = Hm * (Hm + 1) % MOD;
    t2 = t2 * ((2 * Hm + 1) % MOD) % MOD;
    t2 = t2 * ((2 * Hm + 3) % MOD) % MOD;
    t2 = t2 % MOD * inv3 % MOD;
    return (t1 + t2) % MOD;
}

static ll sum_sq_to(ll n) {
    if (n < 0) return 0;
    ll nm = n % MOD;
    return nm * ((nm + 1) % MOD) % MOD * ((2 * nm + 1) % MOD) % MOD * inv6 % MOD;
}

static ll sum_sq_range(ll a, ll b) {
    if (a > b) return 0;
    if (a >= 0) {
        return mod(sum_sq_to(b) - sum_sq_to(a - 1));
    } else if (b < 0) {
        return mod(sum_sq_to(-a) - sum_sq_to(-b - 1));
    } else {
        return (sum_sq_to(b) + sum_sq_to(-a)) % MOD;
    }
}

int main(void) {
    inv3 = powmod(3, MOD - 2, MOD);
    inv6 = powmod(6, MOD - 2, MOD);

    ll ans = 0;

    /* G: max g where g*(g+1) <= 2*N */
    ll G = isqrt_ll(2 * N);
    while (G * (G + 1) > 2 * N) G--;

    for (ll g = 0; g <= G; g++) {
        ll g2 = g * g;
        ll T_full = 2 * N - g2 - g;
        if (T_full < 0) break;

        /* H_full: max h where 2h^2 + 2h <= T_full, i.e. h = (-1+isqrt(1+2*T_full))/2 */
        ll H_full = (-1 + isqrt_ll(1 + 2 * T_full)) / 2;
        if (H_full > g) H_full = g;

        ans = (ans + closed_sum(g, H_full)) % MOD;

        /* H_any: max h where h^2+h <= T_full (since T_any == T_full) */
        ll T_any = T_full;
        ll H_any = (-1 + isqrt_ll(1 + 4 * T_any)) / 2;
        if (H_any > g) H_any = g;

        for (ll h = H_full + 1; h <= H_any; h++) {
            ll h2 = h * h;
            ll T = 2 * N - g2 - h2 - g - h;
            if (T < 0) break;

            /* r_hi: max positive r with r(r+1) <= T */
            ll r_hi = (-1 + isqrt_ll(1 + 4 * T)) / 2;
            if (r_hi > h) r_hi = h;

            /* r_neg_max: max |r| with |r|(|r|-1) <= T */
            ll r_neg_max = (1 + isqrt_ll(1 + 4 * T)) / 2;
            ll r_lo = -h;
            if (r_lo < -r_neg_max) r_lo = -r_neg_max;

            if (r_lo > r_hi) continue;

            ll cnt = r_hi - r_lo + 1;
            ll sr = sum_sq_range(r_lo, r_hi);
            ll gh2 = (g2 + h2) % MOD;
            ll contrib = (2 * sr % MOD + 2 * (cnt % MOD) % MOD * gh2 % MOD) % MOD;
            ans = (ans + contrib) % MOD;
        }
    }

    /* Correction for a >= 1 constraint */
    ans = (ans - 12 + MOD) % MOD;

    printf("%lld\n", ans);
    return 0;
}
"""

def solve():
    with tempfile.TemporaryDirectory() as tmpdir:
        src = os.path.join(tmpdir, "p791.c")
        exe = os.path.join(tmpdir, "p791")
        with open(src, "w") as f:
            f.write(C_CODE)
        # Compile with optimizations
        comp = subprocess.run(
            ["gcc", "-O2", "-o", exe, src, "-lm"],
            capture_output=True, text=True
        )
        if comp.returncode != 0:
            print("Compile error:", comp.stderr, file=sys.stderr)
            sys.exit(1)
        # Run
        result = subprocess.run(
            [exe], capture_output=True, text=True, timeout=280
        )
        if result.returncode != 0:
            print("Runtime error:", result.stderr, file=sys.stderr)
            sys.exit(1)
        return int(result.stdout.strip())

if __name__ == "__main__":
    print(solve())
