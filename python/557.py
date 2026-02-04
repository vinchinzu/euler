"""Project Euler Problem 557: Cutting a Triangle.

A triangle has integer area S. A cevian and a line parallel to one side divide it into
four regions with integer areas a, b, c, d. Find sum of S = a+b+c+d for all valid (a,b,c,d)
with S <= 10000.

From the geometry: bc(2a+b+c+d) = a^2 * d, equivalently bc * (a + S) = a^2 * d where S = a+b+c+d.

For fixed a and S: d must be a multiple of (a+S)/gcd(a^2, a+S).
Then b+c = S-a-d and b*c = a^2*d/(a+S).
The discriminant (b-c)^2 = (b+c)^2 - 4bc must be a non-negative perfect square.
Also need b >= 1 and c >= 1.
"""

import subprocess, tempfile, os

def solve():
    c_code = r"""
#include <stdio.h>
#include <stdlib.h>
#include <math.h>

typedef long long ll;

static ll gcd(ll a, ll b) {
    while (b) { ll t = b; b = a % b; a = t; }
    return a;
}

int main(void) {
    const int N = 10000;
    ll ans = 0;

    for (int a = 1; a < N; a++) {
        ll a2 = (ll)a * a;
        for (int S = a + 3; S <= N; S++) {
            /* Need b >= 1, c >= 1, d >= 1, b+c+d = S-a >= 3 (since a < S and b,c,d >= 1) */
            ll aps = (ll)a + S;
            ll g = gcd(a2, aps);
            ll mult = aps / g;  /* d must be a multiple of mult */

            /* Also need: a^2 * d / (a+S) to be integer (it is, by construction)
               b+c = S - a - d > 0 => d < S - a
               b*c = a^2 * d / (a+S)
               b, c are roots of t^2 - (S-a-d)*t + a^2*d/(a+S) = 0
               discriminant = (S-a-d)^2 - 4*a^2*d/(a+S) >= 0
               Also b >= 1 and c >= 1, so b+c >= 2 => d <= S-a-2
               and b*c >= 1 => a^2*d/(a+S) >= 1 => d >= (a+S)/a^2
            */
            ll sa = S - a;
            for (ll d = mult; d <= sa - 2; d += mult) {
                ll bc = a2 / g * (d / (aps / g));
                /* Hmm, need a2 * d / aps to be integer. Since d is multiple of aps/g
                   and g = gcd(a2, aps), we have a2 * d / aps = a2/g * d/(aps/g) which is integer.
                   But we need to be careful with overflow. */
                /* Actually: bc = a2 * d / aps. Since d = k * mult = k * aps/g,
                   bc = a2 * k * aps / (g * aps) = a2 * k / g = (a2/g) * k.
                   Since g | a2, a2/g is integer. */
                ll k = d / mult;
                bc = (a2 / g) * k;

                ll bpc = sa - d;  /* b + c */
                if (bpc < 2) continue;
                if (bc < 1) continue;

                /* discriminant = bpc^2 - 4*bc */
                ll disc = bpc * bpc - 4 * bc;
                if (disc < 0) continue;

                ll sq = (ll)sqrt((double)disc);
                /* Adjust for floating point errors */
                while (sq * sq > disc) sq--;
                while ((sq+1)*(sq+1) <= disc) sq++;

                if (sq * sq == disc) {
                    /* b and c are (bpc +/- sq) / 2, need both positive integers */
                    if ((bpc + sq) % 2 == 0) {
                        ll b = (bpc + sq) / 2;
                        ll c = (bpc - sq) / 2;
                        if (b >= 1 && c >= 1) {
                            ans += S;
                        }
                    }
                }
            }
        }
    }

    printf("%lld\n", ans);
    return 0;
}
"""
    with tempfile.NamedTemporaryFile(suffix='.c', mode='w', delete=False) as src:
        src.write(c_code)
        src_path = src.name
    bin_path = src_path.replace('.c', '')
    try:
        subprocess.run(['gcc', '-O2', '-o', bin_path, src_path, '-lm'], check=True,
                       capture_output=True, text=True)
        result = subprocess.run([bin_path], capture_output=True, text=True, check=True, timeout=30)
        print(result.stdout.strip())
    except subprocess.CalledProcessError as e:
        print(f"Compile error: {e.stderr}", flush=True)
        raise
    finally:
        os.unlink(src_path)
        if os.path.exists(bin_path):
            os.unlink(bin_path)

if __name__ == "__main__":
    solve()
