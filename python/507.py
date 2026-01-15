"""Project Euler Problem 507: Shortest Lattice Vector.

For 1 ≤ n ≤ N, define the pair of vectors V_n and W_n as the difference,
sum, and product of adjacent Tribonacci numbers (mod M). Let S(n) be the
minimal Manhattan length / L1 norm of a nonzero linear combination D =
k V_n + l W_n. Find Σ_{n=1}^N S(n).
"""

from __future__ import annotations

import math


def L1(v1: int, v2: int, v3: int) -> int:
    """L1 norm."""
    return abs(v1) + abs(v2) + abs(v3)


def gauss(
    u1: int, u2: int, u3: int, v1: int, v2: int, v3: int
) -> int:
    """Gauss algorithm for shortest vector."""
    u_l1 = L1(u1, u2, u3)
    ms = [
        int(math.floor(v1 / u1)) if u1 != 0 else 0,
        int(math.ceil(v1 / u1)) if u1 != 0 else 0,
        int(math.floor(v2 / u2)) if u2 != 0 else 0,
        int(math.ceil(v2 / u2)) if u2 != 0 else 0,
        int(math.floor(v3 / u3)) if u3 != 0 else 0,
        int(math.ceil(v3 / u3)) if u3 != 0 else 0,
    ]

    min_w1 = -1
    min_w2 = -1
    min_w3 = -1
    min_d = float("inf")

    for m in ms:
        if abs(m) < (2**63 - 1) // max(u_l1, 1):
            w1 = v1 - m * u1
            w2 = v2 - m * u2
            w3 = v3 - m * u3
            d = L1(w1, w2, w3)
            if d < min_d:
                min_w1 = w1
                min_w2 = w2
                min_w3 = w3
                min_d = d

    if min_d == float("inf"):
        return u_l1

    if L1(min_w1, min_w2, min_w3) < u_l1:
        return gauss(min_w1, min_w2, min_w3, u1, u2, u3)
    return u_l1


def solve() -> int:
    """Solve Problem 507."""
    N = 2 * 10**7
    M = 10**7

    # Compute Tribonacci sequence
    r = [0] * (12 * N + 1)
    r[2] = 1
    for n in range(3, len(r)):
        r[n] = (r[n - 1] + r[n - 2] + r[n - 3]) % M

    ans = 0
    for n in range(1, N + 1):
        v1 = (r[12 * n - 11] - r[12 * n - 10]) % M
        v2 = (r[12 * n - 9] + r[12 * n - 8]) % M
        v3 = (r[12 * n - 7] * r[12 * n - 6]) % M
        w1 = (r[12 * n - 5] - r[12 * n - 4]) % M
        w2 = (r[12 * n - 3] + r[12 * n - 2]) % M
        w3 = (r[12 * n - 1] * r[12 * n]) % M

        if L1(v1, v2, v3) < L1(w1, w2, w3):
            ans += gauss(v1, v2, v3, w1, w2, w3)
        else:
            ans += gauss(w1, w2, w3, v1, v2, v3)

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
