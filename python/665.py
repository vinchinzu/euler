"""Project Euler Problem 665: Proportionate Nim.

In a game with two piles of stones, players take turns either (1) removing
some n stones from one pile, (2) removing n stones from both piles, or (3)
removing n stones from one pile and 2n stones from the other pile, and the
player removing the last stone wins. Find the sum of n+m for all n≤m, n+m≤N
such that the starting piles (n, m) is a losing position.

We use a union-find-like data structure to track which positions are losing.
"""

from __future__ import annotations


class NextFree:
    """Data structure to find next free position."""

    def __init__(self, n: int) -> None:
        """Initialize NextFree."""
        self.next_free = list(range(n))

    def get(self, n: int) -> int:
        """Get next free position >= n."""
        if n < 0 or n >= len(self.next_free):
            return n
        if self.next_free[n] == n:
            return n
        self.next_free[n] = self.get(self.next_free[n])
        return self.next_free[n]

    def use(self, n: int, jump: int) -> None:
        """Mark position n as used."""
        if n >= 0 and n < len(self.next_free):
            self.next_free[n] = self.get(self.next_free[n + jump])


def solve() -> int:
    """Solve Problem 665."""
    N = 10**7

    next_free1 = NextFree(3 * N)
    next_free2 = NextFree(2 * N)
    next_free3 = NextFree(2 * N)
    next_free4 = NextFree(4 * N)

    ans = 0
    for n in range(N):
        if next_free1.get(n) != n:
            continue
        m = n
        while True:
            old_m = m
            m = next_free1.get(m)
            m = next_free2.get(m - n) + n
            m = next_free3.get(m - 2 * n + N) + 2 * n - N
            m = (next_free4.get(2 * m - n) + n) // 2
            if m == old_m:
                break

        if n + m <= N:
            ans += n + m

        next_free1.use(m, 1)
        next_free2.use(m - n, 1)
        next_free3.use(m - 2 * n + N, 1)
        next_free3.use(n - 2 * m + N, 1)
        next_free4.use(2 * m - n, 2)
        next_free4.use(2 * n - m, 2)

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
