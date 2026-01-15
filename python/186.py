"""Project Euler Problem 186: Connectedness of a network."""

from typing import List

MOD = 1_000_000
TOTAL_USERS = MOD
TARGET = (TOTAL_USERS * 99) // 100
PRIME_MINISTER = 524_287


class LaggedFibonacci:
    """Lagged Fibonacci generator."""

    MOD = 1_000_000

    def __init__(self) -> None:
        """Initialize generator."""
        self.buffer: List[int] = [0] * 55
        for k in range(1, 56):
            self.buffer[k - 1] = (100_003 - 200_003 * k + 300_007 * k * k * k) % self.MOD
        self.k = 1

    def next_value(self) -> int:
        """Generate next value."""
        if self.k <= 55:
            value = self.buffer[self.k - 1]
        else:
            value = (
                self.buffer[(self.k - 24 - 1) % 55]
                + self.buffer[(self.k - 55 - 1) % 55]
            ) % self.MOD
            self.buffer[(self.k - 1) % 55] = value
        self.k += 1
        return value


class UnionFind:
    """Union-Find data structure."""

    def __init__(self, n: int) -> None:
        """Initialize with n elements."""
        self.parent: List[int] = list(range(n))
        self.size: List[int] = [1] * n

    def find(self, x: int) -> int:
        """Find root with path compression."""
        while self.parent[x] != x:
            self.parent[x] = self.parent[self.parent[x]]
            x = self.parent[x]
        return x

    def union(self, a: int, b: int) -> int:
        """Union two sets."""
        root_a = self.find(a)
        root_b = self.find(b)
        if root_a == root_b:
            return root_a

        if self.size[root_a] < self.size[root_b]:
            self.parent[root_a] = root_b
            self.size[root_b] += self.size[root_a]
            return root_b
        else:
            self.parent[root_b] = root_a
            self.size[root_a] += self.size[root_b]
            return root_a


def main() -> int:
    """Main function."""
    generator = LaggedFibonacci()
    uf = UnionFind(TOTAL_USERS)

    successful_calls = 0
    pm_root = uf.find(PRIME_MINISTER)

    while True:
        caller = generator.next_value()
        called = generator.next_value()
        if caller == called:
            continue

        successful_calls += 1
        uf.union(caller, called)

        pm_root = uf.find(PRIME_MINISTER)
        if uf.size[pm_root] >= TARGET:
            break

    return successful_calls


if __name__ == "__main__":
    print(main())
