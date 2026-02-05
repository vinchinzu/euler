#!/usr/bin/env python3
"""
Project Euler 433 - Steps in Euclid's Algorithm

Let E(x,y) be the number of steps in Euclid's algorithm for GCD(x,y).
Find S(N) = sum of E(x,y) for 1 <= x,y <= N where N = 5*10^6.
"""

import subprocess
import tempfile
import os

def solve():
    c_code = r'''
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

// Extended GCD: returns gcd, sets *x, *y such that a*x + b*y = gcd
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

// numLatticePoints: count lattice points on line segment from (x0, y0) stepping by (-A, B)
// for dx+1 steps (i.e., x from x0 to x0 - dx*A)
// Returns sum_{i=0}^{dx} floor((y0 + i*B) / something)
// Actually, this counts points (x,y) with y >= 1 below the line
ll numLatticePoints(ll dx, ll B, ll A) {
    // Java: floorSums.numLatticePoints(x - x1 - 1, B, A)
    // This is sum_{i=0}^{dx} (y0 + i) where y decreases by A/B per step
    // More precisely: number of lattice points with y >= 1 in a right triangle
    // with vertices at (0,0), (dx+1, 0), and (dx+1, (dx+1)*A/B) approximately
    if (dx < 0) return 0;
    ll n = dx + 1;
    // sum_{x=1}^{n} floor(A*x/B)
    ll ans = 0;
    ll a = A, b = B;
    while (n > 0 && a > 0) {
        if (a >= b) {
            ans += (a / b) * n * (n + 1) / 2;
            a %= b;
        }
        if (a == 0) break;
        ll y_max = a * n / b;
        if (y_max == 0) break;
        ans += y_max * n;
        n = y_max;
        ll tmp = a;
        a = b;
        b = tmp;
        ans -= n;  // adjust for the reflection
    }
    return ans;
}

// GCD function
ll gcd_ll(ll a, ll b) {
    while (b) {
        ll t = b;
        b = a % b;
        a = t;
    }
    return a;
}

// Correct floor sum implementation
ll floor_sum(ll n, ll a, ll b) {
    // sum_{x=1}^{n} floor(a*x/b)
    if (n <= 0 || a == 0) return 0;
    ll ans = 0;
    if (a >= b) {
        ans += (a / b) * n * (n + 1) / 2;
        a %= b;
    }
    if (a == 0) return ans;
    ll m = a * n / b;
    ll g = gcd_ll(a, b);
    // f(n,a,b) = m*n - f(m,b,a) + floor(m*g/a)
    ans += m * n - floor_sum(m, b, a) + (m * g) / a;
    return ans;
}

int main() {
    sieve();

    ll ans = 0;

    // Part 1: sum of floor(N/sum) * phi[sum] / 2 for sum from 3 to N
    for (int sum = 3; sum <= N; sum++) {
        ans += (ll)(N / sum) * (phi[sum] / 2);
    }

    // Part 2: Mobius sum over quotients C = N/g
    for (int g = 1; g <= N; g++) {
        if (mobius[g] == 0) continue;

        ll C = N / g;
        ll sqrtC = isqrt(C);
        ll res = 0;

        for (ll B = 1; B * B <= C; B++) {
            for (ll A = 1; A < B; A++) {
                // Find a point (x, y) on B*x + A*y = C
                ll x_coef, y_coef;
                ll gcd = extgcd(B, A, &x_coef, &y_coef);
                // B*x_coef + A*y_coef = gcd
                // Scale to get B*x + A*y = C
                ll scale = C / gcd;
                // Java uses: y = linComb.y * (C / gcd) % B - B
                // Java's % returns result with sign of dividend
                // We want y in range (-B, 0] (non-positive, > -B)
                ll temp = y_coef * scale;
                // Java's % behavior: temp % B has same sign as temp
                ll y_mod = temp % B;  // C's behavior is same as Java for this
                ll y = y_mod - B;
                ll x = (C - A * y) / B;

                ll x1 = C / (A + B);  // intersection of B*x + A*y = C with y = x
                ll x2 = C / B;        // intersection with y = 0

                if (sqrtC > x1) {
                    // Case 1: sqrtC > x1
                    // Triangle part
                    res += nCr2(x1);

                    // Lattice points along the line - note: use (B, A) not (A, B)
                    ll pts1 = floor_sum(x - x1 - 1, B, A);      // from x1+1 to x-1
                    ll pts2 = floor_sum(x - sqrtC - 1, B, A);   // from sqrtC+1 to x-1
                    ll pts3 = floor_sum(x - x2 - 1, B, A);      // from x2+1 to x-1

                    res += pts1 + pts2 - 2 * pts3;
                    res += (2 * x2 - x1 - sqrtC) * y;
                } else {
                    // Case 2: sqrtC <= x1
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
'''

    with tempfile.NamedTemporaryFile(suffix='.c', delete=False, mode='w') as f:
        f.write(c_code)
        c_file = f.name

    exe_file = c_file[:-2]
    try:
        subprocess.run(['gcc', '-O3', '-lm', '-o', exe_file, c_file], check=True, capture_output=True)
        result = subprocess.check_output([exe_file], timeout=600).decode().strip()
        return int(result)
    finally:
        os.unlink(c_file)
        if os.path.exists(exe_file):
            os.unlink(exe_file)

if __name__ == "__main__":
    print(solve())
