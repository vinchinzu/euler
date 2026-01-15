"""Project Euler Problem 444: The Roundtable Lottery.

In a game, the integers from 1 to p are assigned randomly to p players,
but hidden from them. Each player can choose to either reveal his number,
or trade his unknown number with someone else and then leave the game. If
each player tries to maximize his final number, E(p) is the expected number
of players remaining at the end, S_1(N) = Σ_{p=1}^N E(p), and S_k(N) =
Σ_{o=1}^N S_{k-1}(p), then find S_K(N).

First, each player will always choose the highest revealed number and leave,
unless the highest revealed number is smaller than all hidden numbers. This
is clear by induction: if the player reveals a higher number, then a later
player will trade with him, so he might as well have finalized with a
number that is already revealed.

This means that after the first player reveals a number, subsequent players
will always trade with the first player, until the first player reveals a
1. Then, since no one will trade with a 1, the first player can be
considered out of the game, and the remaining players can be considered to
start a new game among just themselves.

There is an equal 1/p probability that any particular number is a 1, so we
have:
E(p) = 1/n (1 + E(n-1)) + 1/n (1 + E(n-2)) + ... + 1/n (1 + E(0))
     = 1 + 1/n (E(n-1) + E(n-2) + ... + E(0))

This is the recurrence for the harmonic sequence, E(p) = 1 + 1/2 + ... +
1/p. This means that S_k(p) are the hyperharmonic numbers of order k+1, and
can be computed by:
S_k(p) = nCr(p+k, k) (E(p+k) - E(k))

We use a Taylor approximation for the nth harmonic number with a
sufficiently small error term.
"""

from __future__ import annotations

import math


def harmonic(n: int) -> float:
    """Compute nth harmonic number using approximation."""
    if n == 0:
        return 0.0
    if n < 2000:
        return sum(1.0 / i for i in range(1, n + 1))
    gamma = 0.57721566490153286060651209  # Euler-Mascheroni constant
    return gamma + math.log(n) + 1 / (2 * n) - 1 / (12 * n * n)


def binomial_coeff(n: int, k: int) -> float:
    """Compute binomial coefficient as float."""
    if k < 0 or k > n:
        return 0.0
    result = 1.0
    for i in range(k):
        result = result * (n - i) / (i + 1)
    return result


def solve() -> float:
    """Solve Problem 444."""
    N = 10**14
    K = 20

    ans = binomial_coeff(N + K, K) * (harmonic(N + K) - harmonic(K))
    return ans


def main() -> float:
    """Main entry point."""
    result = solve()
    print(f"{result:.9e}".replace("+", ""))
    return result


if __name__ == "__main__":
    main()
