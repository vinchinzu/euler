"""Project Euler Problem 741: Binary Grid Colourings.

Find g(n) be the number of ways each cell of an n x n grid can be colored black
or white, such that each row and column has exactly two black cells, and are
unique up to rotations and reflections. Find g(N1) + g(N2).

We use Burnside's Lemma, with the 8 symmetries of the square grid.

First we count the number of grids f(n), without the uniqueness constraint.
Each grid corresponds 1:1 with a bipartite graph where every vertex has 2
edges. Let f'(n) be the number of such bipartite graphs where a particular
vertex v and some other vertex w in the other bipartite set from v only has 1
edge, and there is no edge directly from v to w. Then there are edges v-v1 and
v1-v2, of which there are n ways to choose v1 and n-1 ways to choose v2. Then,
either (1) there is an edge directly from v2 to w (in which case there are n-1
ways to choose w, and f(n-2) ways to complete the rest of the graph, or (2)
there is no direct edge, and thus by definition there are f'(n-1) ways to
complete the rest of the graph.

Next, it's clear that f(n) = f'(n)/2, because each graph counted in f(n) can
be broken in two places at any vertex v to get a graph in f'(n):

f'(n) = n(n-1) ((n-1)f(n-2) + f'(n-1))
f(n)  = f'(n) / 2

Next we consider grids f(n) with 90º rotational symmetry. If n is odd, then
the number of black squares 2n ≡ 2 (mod 4), but the grid is composed of 4
symmetric regions and one square, so the number of black squares must be 0 or
1 (mod 4), a contradiction.

Otherwise, n is even, and we can consider the rows 1, 2, ... n/2. A black
square (x,y) in the top left quadrant is included in the count of both column
x and row y. So the number of grids is equal to the number of graphs on n/2
vertices, where each vertex has 0 or 2 edges, but each edge (x,y) is labeled
0 or 1: 0 for the grid square (x,y), and 1 for the grid square (y,x). There
can be two edges (x,y), but they must be labeled differently.

Let f'(n) be the number of such graphs where a particular vertex v has 1 edge.
Then there are n-2 ways to choose an edge (v,w) from v. Now we either (1) let
w have 1 edge, in which case there are f(n-4) ways to complete the graph, or
(2) we choose one of the f'(n-2) ways to complete the graph starting from w.

Next, to count f(n), we choose a vertex v. If v has no edges, there are
f(n-2) ways to complete the graph. Or, we connect v with two edges to one of
the remaining n/2 - 1 vertices; then there are f(n-4) ways to complete the
graph. Finally, we can draw one edge (v,w) from v (there are n-2 ways to do
this and label it), and choose one of the f'(n-2) ways to complete the graph
from w.

r90:
f'(n) = (n-2) (f(n-4) + f'(n-2))
f(n)  = f(n-2) + (n/2 - 1)f(n-4) + (n-2)f'(n-2)

Next we consider grids f(n) with 180º rotational symmetry. If n is even, then
choose a vertex v. Define f'(n) similarly, where v has one edge, so
f(n) = f'(n)/2. There are n ways to choose an edge (v,w). If w is the 180º
image of v, this completes a four-edge path, and there are f(n-2) ways to
complete the rest of the graph. Otherwise, there are n-2 ways to choose an
edge (w,x), and (n-2)f(n-4) + f'(n-2) ways to complete the graph.

If n is odd, let the center right vertex be t. There are n/2 ways to choose
the vertices v and v' connected to t. Let f'(n) be the number of ways to
complete the graph. There are n-1 ways to choose the other vertex w connected
to v. Either w is connected to the center left vertex, in which case there are
f(n-3) ways to complete the graph, or w is connected to one of the n-3
remaining vertices, and there are f'(n-2) ways to complete the graph.

r180 even:
f'(n) = n ( f(n-2) + (n-2) ((n-2)f(n-4) + f'(n-2)) )
f(n) = f'(n) / 2

r180 odd:
f'(n) = (n-1) (f(n-3) + (n-3)f'(n-2))
f(n)  = ⌊n/2⌋ f'(n)

Next we consider grids f(n) with vertical reflection symmetry. If n is odd,
there can't be any black cells in the middle column, so f(n) = 0. If n is even,
then each row of the left half must have one black cell. So f(n) is the number
of ways to assign a column to each row such that each column is assigned twice,
which is n!/2^{n/2}.

flipY:
f(n) = n! / 2^{n/2}

Finally we consider grids f(n) with diagonal reflection symmetry. Let f'(n) be
the number of grids such that one pair of vertices have only 1 edge each, and
let f''(n) be the number of grids such that two pairs of vertices have only 1
edge each. For f'(n), we either connect the two special vertices, and then
there are f(n-1) ways to complete the graph, or connect each one to one of
the n-1 other vertices, and then there are f'(n-1) ways to complete the graph.

For f''(n), let v be one the special vertices. We either connect it to the
other vertex, and then there are f(n-2) ways to complete the graph, or we
connect it to its corresponding vertex, and then there are f'(n-1) ways to
complete the graph, or we connect it to one of the n-2 other vertices, and
then there are f''(n-1) ways to complete the graph.

Finally, for f(n), we choose a vertex v. If an edge connects v to its
corresponding vertex, then there are n-1 choices for the other edge from v,
and f'(n-1) ways to complete the graph. Otherwise, there are nCr(n-1,2) ways
to choose the two edges from v, and f''(n-1) ways to complete the graph.

flipDiagonal:
f'(n)  = f(n-1) + (n-1)f'(n-1)
f''(n) = f(n-2) + f'(n-1) + (n-2)f''(n-1)
f(n)   = (n-1)f'(n-1) + nCr(n-1,2)f''(n-1)

The answer is the average of these values (with multiplicity of 2 for
rotations of 90º or reflections, since there are 2 of each of those symmetries).
"""

from __future__ import annotations

from typing import List


def pow_mod(base: int, exp: int, mod: int) -> int:
    """Modular exponentiation."""
    if mod <= 0:
        msg = "mod must be positive"
        raise ValueError(msg)
    if base == 0:
        return 0
    base %= mod
    result = 1
    while exp > 0:
        if exp & 1:
            result = (result * base) % mod
        base = (base * base) % mod
        exp >>= 1
    return result


def mod_inv(a: int, m: int) -> int:
    """Modular inverse using extended Euclidean algorithm."""
    if m == 1:
        return 0
    t, new_t = 0, 1
    r, new_r = m, a % m
    while new_r != 0:
        q = r // new_r
        t, new_t = new_t, t - q * new_t
        r, new_r = new_r, r - q * new_r
    if r != 1:
        raise ValueError("Modular inverse does not exist")
    if t < 0:
        t += m
    return t


def nCr(n: int, r: int, mod: int) -> int:
    """Binomial coefficient modulo mod."""
    if r < 0 or r > n:
        return 0
    if r == 0 or r == n:
        return 1
    r = min(r, n - r)
    result = 1
    for i in range(r):
        result = (result * (n - i) * mod_inv(i + 1, mod)) % mod
    return result


def factorials(n: int, mod: int) -> List[int]:
    """Precompute factorials up to n."""
    fact = [1] * (n + 1)
    for i in range(1, n + 1):
        fact[i] = (fact[i - 1] * i) % mod
    return fact


def f(n: int, M: int) -> int:
    """Count grids without symmetry constraint."""
    fp = [0] * (n + 1)
    f_arr = [0] * (n + 1)
    f_arr[0] = 1

    for k in range(2, n + 1):
        fp[k] = (
            k
            * (k - 1)
            % M
            * ((k - 1) * f_arr[k - 2] % M + fp[k - 1])
            % M
        )
        f_arr[k] = ((M + 1) // 2) * fp[k] % M

    return f_arr[n]


def rotate90(n: int, M: int) -> int:
    """Count grids with 90º rotational symmetry."""
    if n % 2 == 1:
        return 0
    fp = [0] * (n + 1)
    f_arr = [0] * (n + 1)
    f_arr[0] = f_arr[2] = 1

    for k in range(4, n + 1, 2):
        fp[k] = (k - 2) * (f_arr[k - 4] + fp[k - 2]) % M
        f_arr[k] = (
            f_arr[k - 2]
            + (k // 2 - 1) * f_arr[k - 4]
            + (k - 2) * fp[k - 2]
        ) % M

    return f_arr[n]


def rotate180(n: int, M: int) -> int:
    """Count grids with 180º rotational symmetry."""
    fp = [0] * (n + 1)
    f_arr = [0] * (n + 1)
    f_arr[0] = f_arr[2] = 1
    fp[2] = 2

    for k in range(3, n + 1):
        if k % 2 == 0:
            fp[k] = (
                (((k - 2) * f_arr[k - 4] % M + fp[k - 2]) * (k - 2) % M)
                + f_arr[k - 2]
            ) * k % M
            f_arr[k] = ((M + 1) // 2) * fp[k] % M
        else:
            fp[k] = (
                f_arr[k - 3] + (k - 3) * fp[k - 2] % M
            ) * (k - 1) % M
            f_arr[k] = (k // 2) * fp[k] % M

    return f_arr[n]


def flip_y(n: int, M: int) -> int:
    """Count grids with vertical reflection symmetry."""
    if n % 2 == 1:
        return 0
    fact = factorials(n, M)
    return fact[n] * mod_inv(pow_mod(2, n // 2, M), M) % M


def flip_diagonal(n: int, M: int) -> int:
    """Count grids with diagonal reflection symmetry."""
    fp = [0] * (n + 1)
    fpp = [0] * (n + 1)
    f_arr = [0] * (n + 1)
    f_arr[0] = fp[1] = 1

    for k in range(2, n + 1):
        fp[k] = (f_arr[k - 1] + (k - 1) * fp[k - 1]) % M
        fpp[k] = (f_arr[k - 2] + fp[k - 1] + (k - 2) * fpp[k - 1]) % M
        f_arr[k] = (
            (k - 1) * fp[k - 1]
            + nCr(k - 1, 2, M) * fpp[k - 1] % M
        ) % M

    return f_arr[n]


def g(n: int, M: int) -> int:
    """Count grids up to symmetry."""
    return (
        (
            f(n, M)
            + 2 * rotate90(n, M)
            + rotate180(n, M)
            + 2 * flip_y(n, M)
            + 2 * flip_diagonal(n, M)
        )
        * mod_inv(8, M)
        % M
    )


def solve() -> int:
    """Solve Problem 741."""
    N1 = 7**7
    N2 = 8**8
    M = 10**9 + 7

    ans = (g(N1, M) + g(N2, M)) % M
    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
