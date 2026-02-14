/*
 * Project Euler 433 - Steps in Euclid's Algorithm
 *
 * S(N) = sum of E(x,y) for 1 <= x,y <= N where N = 5*10^6.
 * Extracted from embedded C in python/433.py.
 */
#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <math.h>

typedef long long ll;

#define N 5000000

int *phi;
signed char *mobius;

void sieve() {
    phi = (int*)malloc((N + 1) * sizeof(int));
    mobius = (signed char*)malloc((N + 1) * sizeof(signed char));
    int *spf = (int*)malloc((N + 1) * sizeof(int));

    for (int i = 0; i <= N; i++) {
        phi[i] = i;
        spf[i] = i;
        mobius[i] = 1;
    }

    for (int i = 2; i <= N; i++) {
        if (spf[i] == i) {
            phi[i] = i - 1;
            mobius[i] = -1;
            for (ll j = (ll)i * i; j <= N; j += i) {
                if (spf[j] == j) spf[j] = i;
            }
        } else {
            int p = spf[i];
            int q = i / p;
            if (q % p == 0) {
                phi[i] = phi[q] * p;
                mobius[i] = 0;
            } else {
                phi[i] = phi[q] * (p - 1);
                mobius[i] = -mobius[q];
            }
        }
    }
    free(spf);
}

static inline ll isqrt(ll n) {
    ll x = (ll)sqrtl((long double)n);
    while (x > 0 && x * x > n) x--;
    while ((x + 1) * (x + 1) <= n) x++;
    return x;
}

static inline ll nCr2(ll n) {
    return n < 2 ? 0 : n * (n - 1) / 2;
}

ll gcd_ll(ll a, ll b) {
    while (b) {
        ll t = b;
        b = a % b;
        a = t;
    }
    return a;
}

ll floor_sum(ll n, ll a, ll b) {
    if (n <= 0 || a == 0) return 0;
    ll ans = 0;
    if (a >= b) {
        ans += (a / b) * n * (n + 1) / 2;
        a %= b;
    }
    if (a == 0) return ans;
    ll m = a * n / b;
    ll g = gcd_ll(a, b);
    ans += m * n - floor_sum(m, b, a) + (m * g) / a;
    return ans;
}

ll extgcd(ll a, ll b, ll *x, ll *y) {
    if (b == 0) {
        *x = 1;
        *y = 0;
        return a;
    }
    ll x1, y1;
    ll g = extgcd(b, a % b, &x1, &y1);
    *x = y1;
    *y = x1 - (a / b) * y1;
    return g;
}

int main() {
    sieve();

    ll ans = 0;

    for (int sum = 3; sum <= N; sum++) {
        ans += (ll)(N / sum) * (phi[sum] / 2);
    }

    for (int g = 1; g <= N; g++) {
        if (mobius[g] == 0) continue;

        ll C = N / g;
        ll sqrtC = isqrt(C);
        ll res = 0;

        for (ll B = 1; B * B <= C; B++) {
            for (ll A = 1; A < B; A++) {
                ll x_coef, y_coef;
                ll gcd = extgcd(B, A, &x_coef, &y_coef);
                ll scale = C / gcd;
                ll temp = y_coef * scale;
                ll y_mod = temp % B;
                ll y = y_mod - B;
                ll x = (C - A * y) / B;

                ll x1 = C / (A + B);
                ll x2 = C / B;

                if (sqrtC > x1) {
                    res += nCr2(x1);

                    ll pts1 = floor_sum(x - x1 - 1, B, A);
                    ll pts2 = floor_sum(x - sqrtC - 1, B, A);
                    ll pts3 = floor_sum(x - x2 - 1, B, A);

                    res += pts1 + pts2 - 2 * pts3;
                    res += (2 * x2 - x1 - sqrtC) * y;
                } else {
                    ll pts1 = floor_sum(x - x1 - 1, B, A);
                    ll pts3 = floor_sum(x - x2 - 1, B, A);

                    res += 2 * (nCr2(x1) + pts1 - pts3 + (x2 - x1) * y);
                    res -= nCr2(sqrtC);
                }
            }
        }
        ans += mobius[g] * res;
    }

    ans *= 4;
    ans += (ll)N * N + N / 2;

    printf("%lld\n", ans);

    free(phi);
    free(mobius);
    return 0;
}
