"""Project Euler Problem 690: Tom and Jerry.

Jerry starts on a vertex in a simple graph. Every turn, Jerry must move along
an edge to an adjacent vertex, and Tom checks one of the vertices. Find the
number of graphs with N vertices such that Tom can guarantee to find Jerry
within a finite number of turns.

This is the Hunter vs. Mole problem, and Tom can guarantee to find Jerry if
every connected component of the graph is a "lobster", i.e. there is a
path such that every vertex is at most distance 2 from the path.

We compute the number of lobster graphs using generating functions, then
use DP to count the number of graphs whose components are all lobsters.
"""

from __future__ import annotations


def solve() -> int:
    """Solve Problem 690."""
    N = 2019
    M = 10**9 + 7

    # Generating function operations mod M, truncated to degree N
    def gf_add(f1: list[int], f2: list[int]) -> list[int]:
        r = [0] * max(len(f1), len(f2))
        for i in range(len(f1)):
            r[i] = (r[i] + f1[i]) % M
        for i in range(len(f2)):
            r[i] = (r[i] + f2[i]) % M
        return r

    def gf_sub(f1: list[int], f2: list[int]) -> list[int]:
        r = [0] * max(len(f1), len(f2))
        for i in range(len(f1)):
            r[i] = (r[i] + f1[i]) % M
        for i in range(len(f2)):
            r[i] = (r[i] - f2[i]) % M
        return r

    def gf_neg(f1: list[int]) -> list[int]:
        return [(-x) % M for x in f1]

    def gf_mul(f1: list[int], f2: list[int]) -> list[int]:
        r = [0] * min(len(f1) + len(f2), N + 1)
        for i in range(min(len(f1), N + 1)):
            if f1[i] == 0:
                continue
            for j in range(min(len(f2), N + 1 - i)):
                r[i + j] = (r[i + j] + f1[i] * f2[j]) % M
        return r

    def gf_mul3(f1, f2, f3):
        return gf_mul(gf_mul(f1, f2), f3)

    def gf_recip(f1: list[int]) -> list[int]:
        """Compute 1/f1 as a power series, truncated to degree N."""
        r = [0] * (N + 1)
        inv_f0 = pow(f1[0], M - 2, M)
        for i in range(N + 1):
            if i == 0:
                r[0] = inv_f0
            else:
                s = 0
                for j in range(1, min(i + 1, len(f1))):
                    s = (s + f1[j] * r[i - j]) % M
                r[i] = (-s * inv_f0) % M
        return r

    def gf_div(f1: list[int], f2: list[int]) -> list[int]:
        return gf_mul(f1, gf_recip(f2))

    def gf_pow(f: list[int], e: int) -> list[int]:
        if e == 1:
            return list(f)
        r = gf_pow(f, e // 2)
        r = gf_mul(r, r)
        if e % 2 == 1:
            r = gf_mul(r, f)
        return r

    # Partition number generating function: prod_{k>=1} 1/(1-x^k)
    def num_partitions() -> list[int]:
        P = [0] * (N + 1)
        P[0] = 1
        for k in range(1, N + 1):
            for i in range(k, N + 1):
                P[i] = (P[i] + P[i - k]) % M
        return P

    # Build the lobster generating function
    ONE = [1]
    TWO = [2]
    X = [0, 1]

    P = num_partitions()

    # P2[2i] = P[i] (partition function evaluated at x^2)
    P2 = [0] * (N + 1)
    for i in range(N // 2 + 1):
        P2[2 * i] = P[i]

    # 1/(1-x)
    recip_1mx = gf_recip(gf_sub(ONE, X))

    # 1/(1-x^2)
    X2 = [0, 0, 1]
    recip_1mx2 = gf_recip(gf_sub(ONE, X2))

    # P - 1/(1-x)
    term1 = gf_sub(P, recip_1mx)

    # (P - 1/(1-x))^2
    term1_sq = gf_pow(term1, 2)

    # 1 - x*P
    one_minus_xP = gf_sub(ONE, gf_mul(X, P))

    # (P - 1/(1-x))^2 / (1 - x*P)
    part_a = gf_div(term1_sq, one_minus_xP)

    # P2 - 1/(1-x^2)
    term2 = gf_sub(P2, recip_1mx2)

    # 1 + x*P
    one_plus_xP = gf_add(ONE, gf_mul(X, P))

    # 1 - x^2 * P2
    one_minus_x2P2 = gf_sub(ONE, gf_mul(X2, P2))

    # (P2 - 1/(1-x^2)) * (1 + x*P) / (1 - x^2*P2)
    part_b = gf_mul3(term2, one_plus_xP, gf_recip(one_minus_x2P2))

    # x^2 * (part_a + part_b) / 2
    inner = gf_mul3(gf_pow(X, 2), gf_add(part_a, part_b), gf_recip(TWO))

    # x * P
    xP = gf_mul(X, P)

    # x^3 / ((1-x)^2 * (1+x))
    one_mx = gf_sub(ONE, X)
    one_px = gf_add(ONE, X)
    denom = gf_mul3(gf_pow(one_mx, 2), one_px, [1])  # just the three factors
    denom = gf_mul(gf_pow(one_mx, 2), one_px)
    correction = gf_div(gf_pow(X, 3), denom)

    # num_lobsters = inner + x*P - correction
    num_lobsters_gf = gf_sub(gf_add(inner, xP), correction)

    # Extract coefficients
    num_lobsters = [0] * (N + 1)
    for i in range(min(len(num_lobsters_gf), N + 1)):
        num_lobsters[i] = num_lobsters_gf[i] % M

    # DP for counting lobster family graphs
    mod_invs = [0] * (N + 2)
    mod_invs[1] = 1
    for i in range(2, N + 2):
        mod_invs[i] = (M - (M // i) * mod_invs[M % i] % M) % M

    dp = [[0] * (N + 1) for _ in range(N + 1)]
    dp[0][0] = 1
    for i in range(N + 1):
        for j in range(1, i + 1):
            nCr = 1
            for k in range(i // j + 1):
                dp[i][j] = (dp[i][j] + nCr * dp[i - j * k][j - 1]) % M
                if k * j < i:
                    nCr = nCr * (num_lobsters[j] + k) % M * mod_invs[k + 1] % M
        for j in range(i + 1, N + 1):
            dp[i][j] = dp[i][i]

    return dp[N][N]


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
