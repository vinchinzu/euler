"""Project Euler Problem 796: Expected Cards Drawn.

Find the expected number of cards drawn from D decks of 54 cards (R ranks in
S suits and J jokers) until all ranks, suits, and decks appear at least once.

By linearity of expectation, we can add for each nth card, the probability
that n cards do not contain all ranks, suits, and decks, and then add 1 at
the end for the final card. To obtain each probability, we use Inclusion
Exclusion for r ranks and s suits and d decks showing up over all r,s,d:

E = Σ_{n=1}^∞ nCr(540,n)⁻¹
        Σ_{r=0}^13 Σ_{s=0}^4 Σ_{d=0}^10 (-1)^{r+s+d+1} nCr(13,r) nCr(4,s) nCr(10,d)
                                        nCr(((13-r)(4-s)+2)(10-d),n).

To make this computable with floating point arithmetic, we cancel the n! terms
in the first and final nCr, and multiply the ratios one at a time.
"""

from __future__ import annotations


def nCr(n: int, k: int) -> float:
    """Binomial coefficient C(n, k)."""
    if k < 0 or k > n:
        return 0.0
    if k == 0 or k == n:
        return 1.0
    result = 1.0
    for i in range(min(k, n - k)):
        result = result * (n - i) / (i + 1)
    return result


def parity(n: int) -> int:
    """Return 1 if n is even, -1 if odd."""
    return 1 if n % 2 == 0 else -1


def solve() -> float:
    """Solve Problem 796."""
    R = 13
    S = 4
    J = 2
    D = 10
    L = (R * S + J) * D

    ans = 1.0

    for n in range(1, L + 1):
        for r in range(R + 1):
            for s in range(S + 1):
                for d in range(D + 1):
                    if r + s + d > 0:
                        res = 1.0
                        limit = ((R - r) * (S - s) + J) * (D - d)
                        for k in range(L, limit, -1):
                            res *= 1.0 * (k - n) / k
                        ans -= (
                            parity(r + s + d)
                            * nCr(R, r)
                            * nCr(S, s)
                            * nCr(D, d)
                            * res
                        )

    return ans


def main() -> float:
    """Main entry point."""
    result = solve()
    print(f"{result:.8f}")
    return result


if __name__ == "__main__":
    main()
