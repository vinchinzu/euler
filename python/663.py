"""Project Euler Problem 663: Sums of Subarrays.

Let A be an array of N integers, initially all zero, updated in step i ≥ 1 by
incrementing A[t_{2i-2}] by 2(t_{2i-1}) - n + 1, where t_k are the tribonacci
numbers modulo N. If M(i) is the maximum sum of a contiguous sub-array of A
after step i, find sum_{i=L1 + 1}^L2 M(i).

We use a segment tree to maintain the sum, max prefix, max suffix, and max
subarray sum for each segment.
"""

from __future__ import annotations


def iceil_pow(n: int, base: int) -> int:
    """Smallest power of base >= n."""
    result = 1
    while result < n:
        result *= base
    return result


class SegmentTree:
    """Segment tree for maximum subarray sum."""

    def __init__(self, size: int) -> None:
        """Initialize segment tree."""
        self.L = iceil_pow(size, 2)
        self.sums = [0] * (2 * self.L)
        self.max_prefixes = [0] * (2 * self.L)
        self.max_suffixes = [0] * (2 * self.L)
        self.max_subarrays = [0] * (2 * self.L)

    def update(self, index: int, val: int) -> None:
        """Update value at index."""
        idx = self.L + index
        self.sums[idx] = val
        self.max_prefixes[idx] = val
        self.max_suffixes[idx] = val
        self.max_subarrays[idx] = val

    def merge(self, index: int) -> None:
        """Merge children at index."""
        left = 2 * index
        right = 2 * index + 1
        self.sums[index] = self.sums[left] + self.sums[right]
        self.max_prefixes[index] = max(
            self.max_prefixes[left],
            self.sums[left] + self.max_prefixes[right],
        )
        self.max_suffixes[index] = max(
            self.max_suffixes[right],
            self.max_suffixes[left] + self.sums[right],
        )
        self.max_subarrays[index] = max(
            self.max_subarrays[left],
            self.max_subarrays[right],
            self.max_suffixes[left] + self.max_prefixes[right],
        )

    def build(self) -> None:
        """Build segment tree from leaves."""
        for j in range(self.L - 1, 0, -1):
            self.merge(j)

    def update_and_rebuild(self, index: int, val: int) -> None:
        """Update value and rebuild tree."""
        self.update(index, val)
        j = (self.L + index) // 2
        while j > 0:
            self.merge(j)
            j //= 2


def solve() -> int:
    """Solve Problem 663."""
    N = 10000003
    L1 = 10_000_000
    L2 = 10_200_000

    a, b, c = 0, 0, 1
    A = [0] * N
    tree = SegmentTree(N)

    ans = 0
    for i in range(1, L2 + 1):
        A[a] += 2 * b - N + 1
        if i == L1:
            for j in range(N):
                tree.update(j, A[j])
            tree.build()
        elif i > L1:
            tree.update_and_rebuild(a, A[a])
            ans += tree.max_subarrays[1]

        new_a = c
        new_b = (a + b + new_a) % N
        new_c = (b + c + new_b) % N
        a, b, c = new_a, new_b, new_c

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
