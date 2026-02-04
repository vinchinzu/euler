"""Project Euler Problem 475: Music festival.

Find the number of ways that N musicians can be rearranged from N/K quartets
to N/3 trios such that no two members of the same quartet are in the same
trio.

Let f(m1, m2, m3) be the number of ways to organize m1 + 2*m2 + 3*m3 musicians
into groups of K such that the m1 musicians were singles, the 2*m2 musicians
were paired in m2 trios, and the 3*m3 musicians were grouped in m3 trios.

We choose d1 singles, d2 pair-members (from d2 different pairs), and d3
trio-members (from d3 different trios) to form one group of K (d1+d2+d3=K).

Answer = f(0, 0, N/3) * (K!)^(N/K) * (1/3!)^(N/3) * (1/(N/3)!).
"""

from __future__ import annotations


def solve() -> int:
    """Solve Problem 475."""
    N = 600
    K = 4
    M = 10**9 + 7

    limit = K * N
    fact = [1] * (limit + 1)
    for i in range(1, limit + 1):
        fact[i] = fact[i - 1] * i % M
    inv_fact = [1] * (limit + 1)
    inv_fact[limit] = pow(fact[limit], M - 2, M)
    for i in range(limit - 1, -1, -1):
        inv_fact[i] = inv_fact[i + 1] * (i + 1) % M

    def nCr(n, r):
        if r < 0 or r > n:
            return 0
        return fact[n] * inv_fact[r] % M * inv_fact[n - r] % M

    cache = {}

    def f(m1, m2, m3):
        if m1 + m2 + m3 == 0:
            return 1
        key = (m1, m2, m3)
        if key in cache:
            return cache[key]

        result = 0
        for d1 in range(min(m1, K) + 1):
            for d2 in range(min(m2, K - d1) + 1):
                d3 = K - d1 - d2
                if d3 < 0 or d3 > m3:
                    continue
                ways = nCr(m1, d1) * pow(2, d2, M) % M * nCr(m2, d2) % M
                ways = ways * pow(3, d3, M) % M * nCr(m3, d3) % M
                sub = f(m1 - d1 + d2, m2 - d2 + d3, m3 - d3)
                result = (result + ways * sub) % M

        cache[key] = result
        return result

    ans = f(0, 0, N // 3)
    ans = ans * pow(fact[K], N // K, M) % M
    ans = ans * pow(inv_fact[3], N // 3, M) % M
    ans = ans * inv_fact[N // 3] % M
    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
