#!/usr/bin/env python3
"""Project Euler Problem 791 - Average and Variance.

S(n) = sum of (a+b+c+d) over ordered quadruples 1 <= a <= b <= c <= d <= n
where average equals twice the variance.

Constraint: S^2 + 2S = 4Q where S=a+b+c+d, Q=a^2+b^2+c^2+d^2.
Substitution v=d-a, w=c-b, then g=(v+w)/2, h=(v-w)/2, r=(a+d-b-c)/2
reduces to: r^2 + g^2 + h^2 = m = S/2, with g >= h >= |r| >= 0.
S(n) = sum of 2(r^2 + g^2 + h^2) over valid (r, g, h).

Closed form per g: C(g,H) = 2g^2(H+1)^2 + H(H+1)(2H+1)(2H+3)/3
for h=0..H with r in [-h,h]. Total complexity: O(sqrt(N)).
"""

import math


def solve(N=10**8, MOD=433494437):
    inv3 = pow(3, MOD - 2, MOD)
    inv6 = pow(6, MOD - 2, MOD)

    def closed_sum(g, H):
        """Sum over h=0..H, r=-h..h of 2(r^2+g^2+h^2) mod MOD."""
        if H < 0:
            return 0
        gm = g % MOD
        Hm = H % MOD
        t1 = 2 * gm * gm % MOD * ((Hm + 1) * (Hm + 1) % MOD) % MOD
        t2 = Hm * (Hm + 1) % MOD * (2 * Hm + 1) % MOD * (2 * Hm + 3) % MOD % MOD * inv3 % MOD
        return (t1 + t2) % MOD

    def sum_sq_to(n):
        """sum_{i=0}^{n} i^2 mod MOD."""
        if n < 0:
            return 0
        nm = n % MOD
        return nm * (nm + 1) % MOD * (2 * nm + 1) % MOD * inv6 % MOD

    def sum_sq_range(a, b):
        """sum_{i=a}^{b} i^2 mod MOD, works for negative a."""
        if a > b:
            return 0
        if a >= 0:
            return (sum_sq_to(b) - sum_sq_to(a - 1)) % MOD
        elif b < 0:
            return (sum_sq_to(-a) - sum_sq_to(-b - 1)) % MOD
        else:
            return (sum_sq_to(b) + sum_sq_to(-a)) % MOD

    ans = 0

    # g ranges from 0 to G where g^2+g <= 2N (from h=r=0, d=(g^2+g)/2<=N)
    G = math.isqrt(2 * N)
    while G * (G + 1) > 2 * N:
        G -= 1

    for g in range(G + 1):
        g2 = g * g
        # H_full: max h where ALL r in [-h,h] satisfy d<=N
        # Condition: g^2 + 2h^2 + g + 2h <= 2N
        T_full = 2 * N - g2 - g
        if T_full < 0:
            break
        H_full = (-1 + math.isqrt(1 + 2 * T_full)) // 2
        H_full = min(H_full, g)

        # Add closed form for h=0..H_full
        ans = (ans + closed_sum(g, H_full)) % MOD

        # Boundary: h from H_full+1 to min(g, H_any)
        # H_any: max h where ANY r has d<=N (use r=0: g^2+h^2+g+h <= 2N)
        T_any = 2 * N - g2 - g
        H_any = (-1 + math.isqrt(1 + 4 * T_any)) // 2  # h^2+h <= T_any
        H_any = min(H_any, g)

        for h in range(H_full + 1, H_any + 1):
            h2 = h * h
            T = 2 * N - g2 - h2 - g - h
            if T < 0:
                break
            # Positive r: r(r+1) <= T => r_hi
            r_hi = (-1 + math.isqrt(1 + 4 * T)) // 2
            r_hi = min(r_hi, h)
            # Negative r: |r|(|r|-1) <= T => |r|_max
            r_neg_max = (1 + math.isqrt(1 + 4 * T)) // 2
            r_lo = max(-h, -r_neg_max)

            if r_lo > r_hi:
                continue

            cnt = r_hi - r_lo + 1
            sr = sum_sq_range(r_lo, r_hi)
            gh2 = (g2 + h2) % MOD
            contrib = (2 * sr + 2 * cnt % MOD * gh2) % MOD
            ans = (ans + contrib) % MOD

    # Corrections for a >= 1 constraint (only affects small g, h):
    # (g,h)=(1,0): r=0 gives a=0. Subtract 2*(0+1+0) = 2.
    # (g,h)=(1,1): r=0 gives a=0, r=-1 gives a=0.
    #   Subtract 2*(0+1+1) + 2*(1+1+1) = 4 + 6 = 10.
    ans = (ans - 12) % MOD

    return ans % MOD


if __name__ == "__main__":
    print(solve())
