"""Project Euler Problem 677: Coloured Graphs.

Find the number of trees with N nodes, each RED, BLUE, or YELLOW, such that RED
nodes have no more than 4 neighbors, BLUE and YELLOW nodes have no more than 3
neighbors, and no two YELLOW nodes may be connected.

Let:
- f_{n,y}(s) = the number of trees of s nodes such that the root has n ORDERED
  children, i.e. the tree with two children A and B is distinct from the tree
  with children B and A, and y=1 if the root is YELLOW.
- g_{n,y}(s) = the number of trees of s nodes such that the root has n children,
  and y=1 if the root is YELLOW.
- h_c(s) = the number of trees of s nodes such that the root has color c.
- H_y(s) = the number of trees of s nodes, where if y=1, then the root cannot
  be YELLOW.

We can write these recursive functions:
f_{0,y}(s) = 1 if s=1, otherwise 0
f_{n,y}(s) = Σ_{s'} H_y(s') f_{n-1,y}(s-s')  (choose one child with s' nodes,
                                              then fill the remaining children)

h_'r'(s) = g_{3,0}(s) + g_{2,0}(s) + g_{1,0}(s) + g_{0,0}(s)
h_'b'(s) = g_{2,0}(s) + g_{1,0}(s) + g_{0,0}(s)
h_'y'(s) = g_{2,1}(s) + g_{1,1}(s) + g_{0,1}(s)

H_0(s) = h_'r'(s) + h_'b'(s) + h_'y'(s)
H_1(s) = h_'r'(s) + h_'b'(s)

To compute g_{n,y}(s), we use Burnside's Lemma. f_{n,y}(s) is the number of
trees with ORDERED children, but we can also count the number of trees such
that various subsets of children are the same. Taking the average of these
values gives g_{n,y}(s).

A further issue is that each tree would be counted once for each node as the
root. To avoid this, we use the fact that every tree with an odd number N of
nodes has exactly one "core", i.e. if the tree is rooted at the core, then
every subtree has no more than N/2 nodes. So if we limit each subtree size to
N/2, then we count each tree once.

For even N, there is a single "core" edge, and both endpoints are core vertices.
To deduplicate these, we apply Burnside's Lemma again. We count the number of
trees with two vertices with subtree sizes N/2, and average that count with the
number of trees with two vertices with IDENTICAL subtrees (with size N/2 each).
"""

from __future__ import annotations


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


def solve() -> int:
    """Solve Problem 677."""
    N = 10000
    K = 4
    M = 10**9 + 7

    colors = ['r', 'b', 'y']

    # Precompute factorials
    fact = [1] * (K + 1)
    for i in range(1, K + 1):
        fact[i] = (fact[i - 1] * i) % M

    # Precompute nCr for small values
    def nCr(n: int, r: int) -> int:
        """Binomial coefficient."""
        if r < 0 or r > n:
            return 0
        if r == 0 or r == n:
            return 1
        result = 1
        for i in range(min(r, n - r)):
            result = (result * (n - i) * mod_inv(i + 1, M)) % M
        return result

    f = [[[0] * (N + 1) for _ in range(2)] for _ in range(K + 1)]
    g = [[[0] * (N + 1) for _ in range(2)] for _ in range(K + 1)]
    h: dict[str, list[int]] = {c: [0] * (N + 1) for c in colors}
    H = [[0] * (N + 1) for _ in range(2)]

    for size in range(N + 1):
        for yellow_root in range(2):
            for num_children in range(K + 1):
                if num_children == K and size != N:
                    continue
                if num_children == 0:
                    f[num_children][yellow_root][size] = 1 if size == 1 else 0
                else:
                    count = 0
                    for child_size in range(1, size):
                        if 2 * child_size <= N:
                            count = (
                                count
                                + H[yellow_root][child_size]
                                * f[num_children - 1][yellow_root][size - child_size]
                            ) % M
                    f[num_children][yellow_root][size] = count % M

                if size > N // 2 and size != N:
                    continue
                count = f[num_children][yellow_root][size]
                for k in range(2, num_children + 1):
                    multiplier = (fact[k - 1] * nCr(num_children, k)) % M
                    for child_size in range(1, size):
                        if k * child_size < size and 2 * child_size <= N:
                            count = (
                                count
                                + multiplier
                                * H[yellow_root][child_size]
                                * f[num_children - k][yellow_root][size - k * child_size]
                            ) % M
                g[num_children][yellow_root][size] = (
                    count * mod_inv(fact[num_children], M)
                ) % M

        for root in colors:
            max_children = K if root == 'r' else K - 1
            for num_children in range(max_children):
                yellow_flag = 1 if root == 'y' else 0
                h[root][size] = (h[root][size] + g[num_children][yellow_flag][size]) % M

        H[0][size] = (h['r'][size] + h['b'][size] + h['y'][size]) % M
        H[1][size] = (h['r'][size] + h['b'][size]) % M

    ans = 0
    for root in colors:
        max_children = K if root == 'r' else K - 1
        for num_children in range(max_children + 1):
            yellow_flag = 1 if root == 'y' else 0
            ans = (ans + g[num_children][yellow_flag][N]) % M

    if N % 2 == 0:
        ans = (ans * 2) % M
        for root in colors:
            for child in colors:
                if root != 'y' or child != 'y':
                    ans = (ans - h[root][N // 2] * h[child][N // 2]) % M
        ans = (ans + H[1][N // 2]) % M
        ans = (ans * mod_inv(2, M)) % M

    return ans % M


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
