"""Project Euler Problem 680: Yarra Gnisrever.

Given an array A of n integers from 0 to n-1, and K operations of the form
"reverse the sub-array from index F_{2n-1} and F_{2n} inclusive (where F_n are
the Fibonacci numbers), find R = Σ_{i=0}^{N-1} i A[i] for the final array.

We maintain a segment tree, where each node represents an entire interval
(represented by the length of the intervals, the first value, and a "diff"
value which is ±1 to represent whether the next value is one higher or one
lower. We also lazily reverse nodes by only tagging a node as "reversed" until
we need to reverse its children (either when we need to reverse only part of
the node, or if we need to compute R).
"""

from __future__ import annotations


def sum_powers(n: int, exp: int, mod: int) -> int:
    """Sum of powers: sum_{i=0}^{n} i^exp mod mod."""
    if exp == 1:
        return (n * (n + 1) // 2) % mod
    if exp == 2:
        return (n * (n + 1) * (2 * n + 1) // 6) % mod
    # General case (simplified)
    result = 0
    for i in range(n + 1):
        result = (result + pow(i, exp, mod)) % mod
    return result


class Node:
    """Segment tree node."""

    def __init__(
        self, len_val: int, first: int, diff: int, left: Node | None = None, right: Node | None = None
    ) -> None:
        """Initialize node."""
        self.len = len_val
        self.first = first
        self.diff = diff
        self.left = left
        self.right = right

    def canonicalize(self) -> None:
        """Canonicalize node structure."""
        if self.left is not None and self.diff == -1:
            left = self.left
            right = self.right
            assert right is not None
            self.left = right.reverse()
            self.right = left.reverse()
            self.diff = 1

    def reverse(self) -> Node:
        """Reverse this node."""
        if self.left is None:
            self.first += self.diff * (self.len - 1)
        self.diff *= -1
        return self

    def reverse_range(self, start: int, end: int) -> None:
        """Reverse range [start, end)."""
        self.canonicalize()
        if self.left is None:
            self.left = Node(start, self.first, self.diff)
            self.right = Node(
                Node(end - start, self.first + self.diff * (end - 1), -self.diff),
                Node(self.len - end, self.first + self.diff * end, self.diff),
            )
            self.diff = 1
        elif end <= self.left.len:
            self.left.reverse_range(start, end)
        elif start >= self.left.len:
            assert self.right is not None
            self.right.reverse_range(start - self.left.len, end - self.left.len)
        else:
            assert self.right is not None
            self.left.ensure_cut_at(start)
            self.right.ensure_cut_at(end - self.left.len)
            left_right = self.left.right
            right_left = self.right.left
            assert left_right is not None
            assert right_left is not None
            self.left.right = right_left.reverse()
            self.right.left = left_right.reverse()
            self.left.len = (self.left.left.len if self.left.left else 0) + (
                self.left.right.len if self.left.right else 0
            )
            self.right.len = (self.right.left.len if self.right.left else 0) + (
                self.right.right.len if self.right.right else 0
            )

    def ensure_cut_at(self, index: int) -> None:
        """Ensure node is cut at given index."""
        self.canonicalize()
        if self.left is None:
            self.left = Node(index, self.first, self.diff)
            self.right = Node(self.len - index, self.first + self.diff * index, self.diff)
            self.diff = 1
        elif index < self.left.len:
            assert self.left is not None
            self.left.ensure_cut_at(index)
            left_left = self.left.left
            left_right = self.left.right
            assert left_left is not None
            assert left_right is not None
            self.left = left_left
            self.right = Node(left_right, self.right)
        elif index > self.left.len:
            assert self.right is not None
            self.right.ensure_cut_at(index - self.left.len)
            right_left = self.right.left
            right_right = self.right.right
            assert right_left is not None
            assert right_right is not None
            self.right = right_right
            self.left = Node(self.left, right_left)

    def compute_R(self, start: int, mod: int) -> int:
        """Compute R value starting from given index."""
        self.canonicalize()
        if self.len == 0:
            return 0
        if self.left is None:
            term1 = (start % mod) * (self.first % mod) % mod * (self.len % mod) % mod
            term2 = (
                ((start * self.diff + self.first) % mod)
                * sum_powers(self.len - 1, 1, mod)
            ) % mod
            term3 = (self.diff * sum_powers(self.len - 1, 2, mod)) % mod
            return (term1 + term2 + term3) % mod
        assert self.left is not None
        assert self.right is not None
        return (
            self.left.compute_R(start, mod)
            + self.right.compute_R(start + self.left.len, mod)
        ) % mod


def solve() -> int:
    """Solve Problem 680."""
    N = 10**18
    K = 10**6
    M = 10**9

    # Generate Fibonacci numbers
    F = [0] * (2 * K + 1)
    F[1] = F[2] = 1
    for i in range(3, len(F)):
        F[i] = (F[i - 2] + F[i - 1]) % N

    tree = Node(N, 0, 1)
    for i in range(1, K + 1):
        s = F[2 * i - 1]
        t = F[2 * i]
        if s < t:
            tree.reverse_range(s, t + 1)
        else:
            tree.reverse_range(t, s + 1)

    ans = tree.compute_R(0, M)
    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
