"""Project Euler Problem 400: Fibonacci Tree Game.

Consider a Fibonacci tree where T(0) is empty, T(1) consists of one node,
and T(k) consists of a node with T(k-1) and T(k-2) as children. If two
players play a game where each player removes a node and all that node's
children, then find the number of winning first moves for the first player
when playing with T(N).

Define f(T) to be the Nim value of the game consisting of the tree T with
an additional node above the root, i.e. the Nim value of the game with T(k)
is f(T_k) - 1. According to the Colon Principle in Hackenbush theory, if
T has subtrees T1 and T2, then f(T) = (f(T1) ^ f(T2)) + 1.

Define g(k, n) to be the number of moves from T_k that results in a tree T
with f(T) = n. Note there are g(k-1, n) moves that clip the left subtree
into something with Nim value n. This means that the entire clipped tree
will have Nim value (n ^ f(T_{k-2})) + 1, and we can increment g(k,
(n ^ f(T_{k-2})) + 1) by that amount. Similarly, we can increment g(k,
(f(T_{k-1}) ^ n) + 1) by g(k-2, n) for all n. Finally, it is possible to
clip the entire tree into an empty tree with Nim value 0, so g(k, 0) = 1.

The answer is g(N, 1), because a winning move results in a tree T with
f(T) - 1 = 0.
"""

from __future__ import annotations


def iceil_pow(n: int, exp: int) -> int:
    """Return smallest power of 2 >= n."""
    result = 1
    while result < n:
        result <<= 1
    return result


def solve() -> int:
    """Solve Problem 400."""
    N = 10000
    L = iceil_pow(N, 2)
    M = 10**18

    f = [0] * (N + 1)
    f[1] = 1
    g = [[0] * (L + 1) for _ in range(N + 1)]
    g[1][0] = 1

    for k in range(2, N + 1):
        f[k] = (f[k - 1] ^ f[k - 2]) + 1
        for n in range(L):
            g[k][(n ^ f[k - 2]) + 1] += g[k - 1][n]
        for n in range(L):
            g[k][(f[k - 1] ^ n) + 1] += g[k - 2][n]
        g[k][0] = 1
        for i in range(L + 1):
            g[k][i] %= M

    return g[N][1]


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
