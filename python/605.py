"""Project Euler Problem 605: Pairwise Coin-Tossing Game.

In an N player game, players (r-1) (mod N) + 1 and r (mod N) + 1 face off in
round r, with each player winning with probability 1/2. Find the probability
that player K is the first to win two rounds in a row.

The probability that player K wins in the first N rounds is equal to the
probability that the first r rounds are won by the first player (for some r
among the first K-1 rounds), the rest of the K-1 rounds are won by the second
player, and the Kth round is won by player K. This probability is (K-1)/2^K.

Continuing this logic, the probability that player K wins in any round is
(K-1)/2^K + (K+N-1)/2^(K+N) + (K+2N-1)/2^(K+2N) + ...

This arithmetico-geometric series evaluates to
((K-1) (2^N - 1) + N) 2^(N-K) / (2^N - 1)².
For (N, 2^N - 1) = 1, this fraction is reduced and the product of the
numerator and denominator is straightforward to compute.
"""

from __future__ import annotations


def pow_mod(base: int, exp: int, mod: int) -> int:
    """Modular exponentiation."""
    result = 1
    base %= mod
    while exp > 0:
        if exp & 1:
            result = (result * base) % mod
        base = (base * base) % mod
        exp >>= 1
    return result


def solve() -> int:
    """Solve Problem 605."""
    N = 10**8 + 7
    K = 10**4 + 7
    M = 10**8

    two_n = pow_mod(2, N, M)
    two_n_minus_one = (two_n - 1) % M
    num = ((K - 1) * two_n_minus_one % M + N) % M * pow_mod(2, N - K, M) % M
    den = pow_mod(two_n_minus_one, 2, M)
    ans = (num * den) % M
    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
