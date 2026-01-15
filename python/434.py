"""Project Euler Problem 434: Rigid graphs.

Given a grid graph consisting of m x n squares, let R(m, n) be the number
of ways to add diagonal edges to a subset of the squares such that the
entire graph is rigid, i.e. there is no way to move part of the graph
while keeping the lengths of all edges constant. Find Σ_{m=N} Σ_{n=N}
R(m, n).

Moving part of the graph will make some of the squares into rhombi.
However, since opposite sides of a rhombus are parallel, all vertical
edges of a row are always parallel to each other; the same goes for all
horizontal edges in a column. If a diagonal edge is placed in row i and
column j, then that adds the constraint that the vertical edges in row i
are all perpendicular to the horizontal edges in column j.

Consider a bipartite graph G. One component G_m consists of m vertices,
each one representing a row in the original graph; the other component G_n
consists of n vertices representing the columns. Placing a diagonal edge on
the grid graph represents connecting the row vertex and column vertex in
the bipartite graph G. If we take any connected component of vertices in
G, and take the edges in the grid graph that they represent, then the
slopes of those edges must all be fixed relative to each other. If a grid
graph is rigid, then its bipartite graph G must be connected.

To compute the number of connected bipartite graphs R(m, n), first note
that the total number of bipartite graphs is 2^{m*n}. Now choose some
vertex v in G_m and look at the largest connected component containing v.
Suppose this component contains a vertices in G_m (including v) and b
vertices in G_n. We need to subtract the number of all graphs where either
a < m or b < n, i.e. the graph is not connected. For a given (a, b), there
are nCr(m-1, a-1) ways to choose the remaining a-1 vertices in G_m, nCr(n,
b) ways to choose the b vertices in G_n, R(a, b) ways to place the edges
in the connected component, and 2^{(m-a)(n-b)} ways to place the remaining
edges. So:

R(m, n) = 2^{m*n} - Σ_{a=1}^m Σ_{b=0}^n nCr(m-1, a-1) nCr(n, b) R(a, b)
         2^{(m-a)(n-b)}.
"""

from __future__ import annotations


def ncr(n: int, k: int, mod: int) -> int:
    """Compute binomial coefficient n choose k modulo mod."""
    if k < 0 or k > n:
        return 0
    if k == 0 or k == n:
        return 1
    k = min(k, n - k)
    result = 1
    for i in range(k):
        result = result * (n - i) % mod
        result = result * pow(i + 1, mod - 2, mod) % mod
    return result


def ncr_table(n: int, mod: int) -> list[list[int]]:
    """Precompute binomial coefficients up to n."""
    table = [[0] * (n + 1) for _ in range(n + 1)]
    for i in range(n + 1):
        table[i][0] = 1
        for j in range(1, i + 1):
            table[i][j] = (table[i - 1][j - 1] + table[i - 1][j]) % mod
    return table


def pow_mod(base: int, exp: int, mod: int) -> int:
    """Compute base^exp mod mod."""
    result = 1
    base %= mod
    while exp:
        if exp & 1:
            result = (result * base) % mod
        base = (base * base) % mod
        exp >>= 1
    return result


def solve() -> int:
    """Solve Problem 434."""
    N = 100
    M = 10**9 + 33

    ncr_table_vals = ncr_table(N, M)
    max_pow = N * N
    pow2s = [pow_mod(2, i, M) for i in range(max_pow + 1)]

    R = [[0] * (N + 1) for _ in range(N + 1)]
    for m in range(1, N + 1):
        for n in range(N + 1):
            R[m][n] = pow2s[m * n]
            for a in range(1, m + 1):
                for b in range(n + 1):
                    if a < m or b < n:
                        R[m][n] = (
                            R[m][n]
                            - ncr_table_vals[m - 1][a - 1]
                            * ncr_table_vals[n][b]
                            % M
                            * R[a][b]
                            % M
                            * pow2s[(m - a) * (n - b)]
                            % M
                        ) % M

    ans = 0
    for m in range(1, N + 1):
        for n in range(1, N + 1):
            ans = (ans + R[m][n]) % M

    return (ans + M) % M


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
