"""Project Euler Problem 880 — Fermat equation with cubes.

Uses embedded C for performance.
"""
import subprocess, os, tempfile


def solve():
    c_code = r"""
#include <stdio.h>
#include <math.h>

typedef long long ll;
typedef unsigned long long ull;
typedef __int128 lll;

/* Check if n is a perfect cube */
static int is_perfect_cube(ll n) {
    if (n <= 0) return n == 0;
    ll c = (ll)round(cbrt((double)n));
    ll c1;
    c1 = c - 1;
    if (c1 >= 0 && c1*c1*c1 == n) return 1;
    if (c*c*c == n) return 1;
    c1 = c + 1;
    if (c1*c1*c1 == n) return 1;
    return 0;
}

static ll gcd(ll a, ll b) {
    while (b) { ll t = b; b = a % b; a = t; }
    return a;
}

static ll llabs_val(ll x) {
    return x < 0 ? -x : x;
}

static ll isqrt_ll(ll n) {
    if (n <= 0) return 0;
    ll x = (ll)sqrt((double)n);
    while ((x+1)*(x+1) <= n) x++;
    while (x*x > n) x--;
    return x;
}

int main(void) {
    ll N = 1000000000000000LL; /* 10^15 */
    ll M = 1095912793LL;       /* 1031^3 + 2 */
    ll ans = 0;
    ll r, s;

    /* sign_a = 1 */
    for (r = 1; r*r*r <= N; r += 2) {
        for (s = 1; ; s++) {
            ll v = s + 2*r;
            /* Check 4*s*v^3 > N without overflow: use __int128 */
            lll check = (lll)4 * s * v * v * v;
            if (check > N) break;
            if (gcd(r, s) != 1) continue;

            ll val1 = r - 4*s;
            ll val2 = s + 2*r;
            ll av1 = llabs_val(val1);
            lll av1_3 = (lll)av1*av1*av1;
            lll v2_3 = (lll)val2*val2*val2;
            ll maybeCube = 2*r*s*s;

            if (!is_perfect_cube(maybeCube)) {
                /* max_g2 = N / (4*s*v2_3), but v2_3 could be large */
                lll denom_y = (lll)4 * s * v2_3;
                ll max_g2;
                if (denom_y == 0 || denom_y > N)
                    max_g2 = 0;
                else
                    max_g2 = (ll)(N / denom_y);

                if (av1_3 > 0) {
                    lll denom_x = (lll)r * av1_3;
                    if (denom_x > 0 && denom_x <= N) {
                        ll max_g2_x = (ll)(N / denom_x);
                        if (max_g2_x < max_g2) max_g2 = max_g2_x;
                    } else if (denom_x > N) {
                        max_g2 = 0;
                    }
                }

                ll max_g = isqrt_ll(max_g2);

                if (max_g >= 1) {
                    /* coeff = r*av1_3 + 4*s*v2_3 - can overflow ll, use __int128 */
                    lll coeff = (lll)r * av1_3 + (lll)4 * s * v2_3;
                    /* sg = max_g*(max_g+1)*(2*max_g+1)/6 - can overflow ll */
                    lll sg = (lll)max_g * (max_g+1) * (2*max_g+1) / 6;
                    ll coeff_mod = (ll)(coeff % M);
                    ll sg_mod = (ll)(sg % M);
                    ans = (ans + coeff_mod * sg_mod) % M;
                }
            }
        }
    }

    /* sign_a = -1 */
    for (r = 1; r*r*r <= N; r += 2) {
        for (s = 1; ; s++) {
            ll v = r + 4*s;
            lll check = (lll)r * v * v * v;
            if (check > N) break;
            if (gcd(r, s) != 1) continue;

            ll val1 = r + 4*s;
            ll val2 = s - 2*r;
            lll v1_3 = (lll)val1*val1*val1;
            ll av2 = llabs_val(val2);
            lll av2_3 = (lll)av2*av2*av2;
            ll maybeCube = 2*r*s*s;

            if (!is_perfect_cube(maybeCube)) {
                lll denom_x = (lll)r * v1_3;
                ll max_g2;
                if (denom_x == 0 || denom_x > N)
                    max_g2 = 0;
                else
                    max_g2 = (ll)(N / denom_x);

                if (av2_3 > 0) {
                    lll denom_y = (lll)4 * s * av2_3;
                    if (denom_y > 0 && denom_y <= N) {
                        ll max_g2_y = (ll)(N / denom_y);
                        if (max_g2_y < max_g2) max_g2 = max_g2_y;
                    } else if (denom_y > N) {
                        max_g2 = 0;
                    }
                }

                ll max_g = isqrt_ll(max_g2);

                if (max_g >= 1) {
                    lll coeff = (lll)r * v1_3 + (lll)4 * s * av2_3;
                    lll sg = (lll)max_g * (max_g+1) * (2*max_g+1) / 6;
                    ll coeff_mod = (ll)(coeff % M);
                    ll sg_mod = (ll)(sg % M);
                    ans = (ans + coeff_mod * sg_mod) % M;
                }
            }
        }
    }

    printf("%lld\n", ans);
    return 0;
}
"""
    tmp = tempfile.NamedTemporaryFile(suffix='.c', delete=False, mode='w')
    tmp.write(c_code)
    tmp.close()
    exe = tmp.name.replace('.c', '')
    try:
        subprocess.run(['gcc', '-O2', '-o', exe, tmp.name, '-lm'],
                       check=True, capture_output=True)
        result = subprocess.run([exe], capture_output=True, text=True, timeout=280)
        print(result.stdout.strip())
    finally:
        os.unlink(tmp.name)
        if os.path.exists(exe):
            os.unlink(exe)


if __name__ == "__main__":
    solve()
