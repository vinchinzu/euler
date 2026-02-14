/*
 * Project Euler Problem 557: Cutting a Triangle.
 * A triangle has integer area S. A cevian and a line parallel to one side
 * divide it into four regions with integer areas a, b, c, d.
 * Find sum of S for all valid (a,b,c,d) with S <= 10000.
 *
 * Extracted from embedded C in Python solution.
 */
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
            ll aps = (ll)a + S;
            ll g = gcd(a2, aps);
            ll mult = aps / g;

            ll sa = S - a;
            for (ll d = mult; d <= sa - 2; d += mult) {
                ll k = d / mult;
                ll bc = (a2 / g) * k;

                ll bpc = sa - d;
                if (bpc < 2) continue;
                if (bc < 1) continue;

                ll disc = bpc * bpc - 4 * bc;
                if (disc < 0) continue;

                ll sq = (ll)sqrt((double)disc);
                while (sq * sq > disc) sq--;
                while ((sq+1)*(sq+1) <= disc) sq++;

                if (sq * sq == disc) {
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
