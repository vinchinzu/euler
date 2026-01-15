"""Project Euler Problem 703: Circular Logic III.

Let f be a function from N-tuples of booleans (b_1, ... b_N) to N-tuples of
booleans (c_1, ... c_N) where c_i = b_{i+1} for i<N and c_N = b_1 AND
(b_2 XOR b_3). Find the number of functions T from N-tuples of booleans to
booleans such that T(x) AND T(f(x)) = false for all N-tuples x.

We can construct a directed graph with edges from each x to f(x). Each vertex
has a single outgoing edge, so each connected component is a tree with a
single additional edge from the root to some vertex. The problem is then to
find the number of ways to color each vertex of this graph either false (f)
or true (t) such that no two true vertices are connected.

Each connected component is independent. For each connected component, the
root can be either false or true. If it's false, then we can ignore the
constraint on the additional edge, and we can recurse on the tree. Let f_v
and t_v be the number of colorings of a subtree rooted at v if the root v is
colored false and true, respectively. Then we can compute f_v and t_v by
computing f_u and t_u for all children u; we have f_v = Π_u (f_u + t_u) and
t_v = Π_u f_u.

If the root is true, then the vertex at the other end of the additional edge
(call it n) must be false. So we can cut the tree at n, and compute the
number of colorings of the two subtree pieces.
"""

from __future__ import annotations

from dataclasses import dataclass
from typing import List


@dataclass
class Result:
    """Result for subtree coloring."""

    f: int  # Number of colorings with root false
    t: int  # Number of colorings with root true


def solve() -> int:
    """Solve Problem 703."""
    n = 20
    m = 1_001_001_011

    # Build next array: next[i] = f(i)
    next_arr: List[int] = []
    for i in range(1 << n):
        # c_i = b_{i+1} for i < N
        # c_N = b_1 AND (b_2 XOR b_3)
        # So: shift right by 1, then set highest bit to b_1 AND (b_2 XOR b_3)
        shifted = i >> 1
        b1 = i & 1
        b2 = (i >> 1) & 1
        b3 = (i >> 2) & 1
        highest_bit = b1 & (b2 ^ b3)
        next_val = shifted + (highest_bit << (n - 1))
        next_arr.append(next_val)

    # Build reverse graph (prevs[i] = list of nodes that map to i)
    prevs: List[List[int]] = [[] for _ in range(1 << n)]
    for i in range(1 << n):
        prevs[next_arr[i]].append(i)

    used = [False] * (1 << n)
    ans = 1

    def helper(ptr: int, avoid: int) -> Result:
        """Compute colorings for subtree rooted at ptr, avoiding avoid."""
        used[ptr] = True
        if ptr == avoid:
            return Result(1, 0)

        f_val = 1
        t_val = 1
        for prev in prevs[ptr]:
            result = helper(prev, avoid)
            f_val = (f_val * (result.f + result.t)) % m
            t_val = (t_val * result.f) % m

        return Result(f_val % m, t_val % m)

    for i in range(1 << n):
        if not used[i]:
            # Find root of cycle
            root = i
            while not used[root]:
                used[root] = True
                root = next_arr[root]

            # Root is false: all subtrees independent
            f_val = 1
            for prev in prevs[root]:
                res = helper(prev, root)
                f_val = (f_val * (res.f + res.t)) % m

            # Root is true: next[root] must be false
            if next_arr[root] == root:
                t_val = 0
            else:
                t_val = (
                    helper(next_arr[root], root).f
                    * helper(root, next_arr[root]).t
                ) % m

            ans = (ans * (f_val + t_val)) % m

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
