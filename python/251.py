"""Project Euler Problem 251: Cardano Triplets.

Find the number of Cardano triplets (a, b, c) that satisfy
cbrt(a + b*sqrt(c)) + cbrt(a - b*sqrt(c)) = 1 and a+b+c <= N.
"""

from __future__ import annotations

from math import gcd, isqrt


def solve() -> int:
    """Solve Problem 251."""
    N = 110_000_000
    ans = 0

    max_r = isqrt(8 * N // 3)
    if max_r % 2 == 0:
        max_r -= 1

    for r in range(1, max_r + 1, 2):
        r2 = r * r
        # min_t: smallest positive t such that r^2 * t = 5 (mod 8)
        # Since r is odd, r^2 mod 8 is always 1, so 5 * r^2 mod 8 = 5
        min_t = (5 * r2) % 8
        if min_t == 0:
            min_t = 8

        # Upper bound on s: (3*r2/8 + s^2)*min_t <= N => s^2 <= N/min_t - 3*r2/8
        s_limit_sq = N / min_t - 3 * r2 / 8
        if s_limit_sq < 1:
            continue
        max_s = isqrt(int(s_limit_sq))

        for s in range(1, max_s + 1):
            if gcd(r, s) != 1:
                continue

            # Solve 8*s*g - r^2*t = 3 using extended gcd
            # We need g = inverse of 8*s mod r^2, then g = g * 3 mod r^2
            # Similarly t from the other direction
            a8s = 8 * s
            # ext_gcd(8s, r^2) -> since gcd(r,s)=1 and r is odd, gcd(8s, r^2) = 1
            # Use pow for modular inverse (r^2 is odd since r is odd)
            g_inv = pow(a8s, -1, r2)  # modular inverse of 8s mod r^2
            g = (g_inv * 3) % r2
            if g == 0:
                g = r2

            # t from: r^2 * t = 8*s*g - 3
            t_val = (a8s * g - 3) // r2

            s2 = s * s
            start = 3 * g * s - 1 + g * r + s2 * t_val
            if start <= N:
                increment = (3 * s + r) * r2 + 8 * s2 * s
                ans += (N - start) // increment + 1

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
