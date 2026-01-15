"""Project Euler Problem 713: Turán Graphs.

If there are N fuses and m are working, let T(N, m) be the minimum number of
pairs of fuses to ensure that at least one pair of fuses are both working.
Find Σ_{m=2}^N T(N, m).

T(N, m) is equivalent to constructing a graph over N vertices with the minimum
number of edges such that any subset of m vertices will contain an edge. This
can only be done by splitting the vertices into k = m-1 groups, with an edge
connecting all vertices in a group, because by the Pigeonhole principle at
least two of the m vertices will be in the same group.

To minimize the number of edges, we need to make the sizes of the groups as
similar as possible, i.e. N % k have N/k + 1 vertices, and the other groups
have N/k vertices. The number of edges is then (N % k) nCr(N/k + 1, 2) +
(k - N % k) nCr(N/k, 2). This is the Turan graph and is the minimum by
Turan's theorem.
"""

from __future__ import annotations


def ncr(n: int, r: int) -> int:
    """Binomial coefficient C(n, r)."""
    if r < 0 or r > n:
        return 0
    if r == 0 or r == n:
        return 1
    result = 1
    for i in range(min(r, n - r)):
        result = result * (n - i) // (i + 1)
    return result


def solve() -> int:
    """Solve Problem 713."""
    n = 10**7
    ans = 0

    for k in range(1, n):
        group_size = n // k
        remainder = n % k
        ans += remainder * ncr(group_size + 1, 2) + (k - remainder) * ncr(
            group_size, 2
        )

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
