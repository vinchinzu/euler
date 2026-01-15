"""Project Euler Problem 212: Combined Volume of Cuboids.

Find the total volume contained in the union of the given N cuboids.
"""

from __future__ import annotations

from collections import defaultdict
from dataclasses import dataclass
from typing import DefaultDict, List, Tuple


@dataclass(frozen=True)
class Section:
    """Represents a section of space."""

    x: int
    y: int
    z: int


@dataclass
class Cuboid:
    """Represents a cuboid."""

    x: int
    y: int
    z: int
    dx: int
    dy: int
    dz: int


def lagged_fibonacci(n: int) -> List[int]:
    """Generate lagged Fibonacci sequence."""
    s = [0] * 55
    for k in range(1, 56):
        s[k - 1] = (100003 - 200003 * k + 300007 * k * k * k) % 1000000
    for k in range(55, n):
        s.append((s[k - 24] + s[k - 55]) % 1000000)
    return s


def iround_down(n: int, k: int) -> int:
    """Round n down to nearest multiple of k."""
    return n // k * k


def parity(n: int) -> int:
    """Return 1 if n is even, -1 if odd."""
    return 1 if n % 2 == 0 else -1


def solve() -> int:
    """Solve Problem 212."""
    N = 50000
    L = 130

    # Generate cuboids using lagged Fibonacci
    s = lagged_fibonacci(6 * N)
    cuboids: List[Cuboid] = []
    for i in range(N):
        idx = 6 * i
        cuboids.append(
            Cuboid(
                s[idx] % 10000,
                s[idx + 1] % 10000,
                s[idx + 2] % 10000,
                s[idx + 3] % 399 + 1,
                s[idx + 4] % 399 + 1,
                s[idx + 5] % 399 + 1,
            )
        )

    # Assign cuboids to sections
    sections: DefaultDict[Section, List[Cuboid]] = defaultdict(list)
    for cuboid in cuboids:
        for dx in range(0, cuboid.dx + L, L):
            for dy in range(0, cuboid.dy + L, L):
                for dz in range(0, cuboid.dz + L, L):
                    section = Section(
                        iround_down(cuboid.x + dx, L),
                        iround_down(cuboid.y + dy, L),
                        iround_down(cuboid.z + dz, L),
                    )
                    sections[section].append(cuboid)

    def helper(
        index: int,
        cuboids_list: List[Cuboid],
        min_x: int,
        min_y: int,
        min_z: int,
        max_x: int,
        max_y: int,
        max_z: int,
        num_cuboids: int,
    ) -> int:
        """Helper function for inclusion-exclusion."""
        if min_x >= max_x or min_y >= max_y or min_z >= max_z:
            return 0
        if index == len(cuboids_list):
            if num_cuboids == 0:
                return 0
            return parity(num_cuboids) * (max_x - min_x) * (max_y - min_y) * (max_z - min_z)

        c = cuboids_list[index]
        return helper(
            index + 1,
            cuboids_list,
            min_x,
            min_y,
            min_z,
            max_x,
            max_y,
            max_z,
            num_cuboids,
        ) + helper(
            index + 1,
            cuboids_list,
            max(min_x, c.x),
            max(min_y, c.y),
            max(min_z, c.z),
            min(max_x, c.x + c.dx),
            min(max_y, c.y + c.dy),
            min(max_z, c.z + c.dz),
            num_cuboids + 1,
        )

    ans = 0
    for section, cuboids_list in sections.items():
        ans -= helper(
            0,
            cuboids_list,
            section.x,
            section.y,
            section.z,
            section.x + L,
            section.y + L,
            section.z + L,
            0,
        )

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
