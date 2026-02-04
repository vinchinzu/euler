"""Project Euler Problem 488: Unbalanced Nim.

Find the sum of a+b+c for all losing positions (a,b,c) with 0 < a < b < c < N in a
variant of three-heap Nim where a player can never make two heaps the same size.

Losing positions satisfy (a+1)^(b+1)^(c+1) = 0. We compute the count and sum of
triples (a,b,c) with 0 <= a,b,c <= N and a^b^c = 0 using digit DP on binary
representation, then adjust for the constraints.
"""

from __future__ import annotations

from itertools import product


def solve() -> int:
    """Solve Problem 488."""
    N = 10**18
    M = 10**9
    MOD = 6 * M  # work modulo 6M to allow exact division by 6

    cache = {}

    def f(n, g):
        """Count and sum of triples (a,b,c) with 0<=a,b,c<=n and a^b^c=0.

        g is a tuple of 3 booleans indicating whether the current prefix
        of each value exceeds the corresponding prefix of n.
        Returns (count % MOD, sum % MOD).
        """
        if n == 0:
            if any(g):
                return (0, 0)
            return (1, 0)

        key = (n, g)
        if key in cache:
            return cache[key]

        count = 0
        total = 0
        nbit = n % 2

        for bits in product(range(2), repeat=3):
            # XOR constraint: sum of bits must be even
            if sum(bits) % 2 != 0:
                continue

            new_g = tuple(
                bits[i] > nbit or (bits[i] == nbit and g[i])
                for i in range(3)
            )

            sub_count, sub_total = f(n // 2, new_g)
            count = (count + sub_count) % MOD
            total = (total + 2 * sub_total + sum(bits) * sub_count) % MOD

        cache[key] = (count % MOD, total % MOD)
        return cache[key]

    res = f(N, (False, False, False))
    f0 = res[0]
    f1 = res[1]

    # f0 = count of (a,b,c) with 0<=a,b,c<=N, a^b^c=0
    # f1 = sum of a+b+c over these
    # Need to subtract (0,0,0), permutations of (0,k,k), and permutations of (0,1,k) for even N
    # Formula from Java: 6S = f1 - 3*f0 - 6*N^2 + 15*N - 3
    # where N^2 is taken mod M

    Nsq = pow(N, 2, MOD)
    Nmod = N % MOD

    numerator = (f1 - 3 * f0 - 6 * Nsq + 15 * Nmod - 3) % MOD
    ans = (numerator // 6) % M
    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
