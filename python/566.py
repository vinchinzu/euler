"""Project Euler Problem 566: Cake Cutting.

Given a circular cake with icing on top, if we repeatedly cut slices of cake of lengths
360º/a, 360º/b, 360º/√c in that order, each time flipping over the cut slice, let F(a,b,c)
be the minimum number of flips until all the icing returns to the top. Find
Σ_{9≤a<b<c≤N} F(a,b,c).

For convenience, denote 0=0º and 1=360º and interpolate linearly. We start by considering
the point at the top of the cake at 0. Simulate the cuts, and keep track of the positions
(relative to the knife) of this point as we make cuts, until the point returns to the top
of the cake at 0. While performing this simulation, a small region after this point was
never cut, so this entire region always moves together.

This gives the period for this region, but there may be smaller orders at which the region
may be elsewhere along the cake, but the sequence of whether it is above or below the cake
is the same as the sequence if starting at 0. So we determine all of these orders.

After processing this region, we move on to the next region, and so on, until we've
processed the entire cake. We use the generalized Chinese Remainder Theorem to find the
orders at which all the regions have the same sequence of being above or below the cake.

Finally, as an optimization, when we process a contiguous region of cake, we can also
process any region of cake that the region moved to during the simulation after a full
multiple of 3 flips, because they would all have the same period. A tree map data structure
containing (start, end) intervals is used to efficiently handle these queries.
"""

from __future__ import annotations

from dataclasses import dataclass
from math import gcd, isqrt, sqrt
from typing import List, Set


N = 53

sqrt_c: float = 0.0
d: int = 0


@dataclass(frozen=True)
class Rad:
    """Represents (a+b√c)/d in the current context."""

    a: int
    b: int

    def add(self, other: "Rad") -> "Rad":
        """Add two Rad values."""
        return Rad(self.a + other.a, self.b + other.b)

    def subtract(self, other: "Rad") -> "Rad":
        """Subtract two Rad values."""
        return Rad(self.a - other.a, self.b - other.b)

    def min(self, other: "Rad") -> "Rad":
        """Minimum of two Rad values."""
        return self if self.compare_to(other) <= 0 else other

    def max(self, other: "Rad") -> "Rad":
        """Maximum of two Rad values."""
        return self if self.compare_to(other) >= 0 else other

    def __lt__(self, other: "Rad") -> bool:
        """Less than comparison."""
        return (self.a + self.b * sqrt_c) < (other.a + other.b * sqrt_c)

    def __le__(self, other: "Rad") -> bool:
        """Less than or equal comparison."""
        return (self.a + self.b * sqrt_c) <= (other.a + other.b * sqrt_c)

    def compare_to(self, other: "Rad") -> int:
        """Compare two Rad values. Returns -1, 0, or 1."""
        val1 = self.a + self.b * sqrt_c
        val2 = other.a + other.b * sqrt_c
        if val1 < val2:
            return -1
        if val1 > val2:
            return 1
        return 0

    @staticmethod
    def ONE() -> "Rad":
        """Return Rad representing 1."""
        return Rad(d, 0)


@dataclass(frozen=True)
class Pos:
    """Position on cake."""

    angle: Rad
    flipped: bool


def is_sq(n: int) -> bool:
    """Check if n is a perfect square."""
    root = isqrt(n)
    return root * root == n


def lcm(a: int, b: int) -> int:
    """Least common multiple."""
    return a * b // gcd(a, b)


def lin_comb(a: int, b: int) -> Rad:
    """Find solution to ax + by = gcd(a,b) using extended Euclidean algorithm."""
    if b == 0:
        return Rad(1, 0)
    x1 = lin_comb(b, a % b)
    q = a // b
    return Rad(x1.b, x1.a - q * x1.b)


def mod(a: int, m: int) -> int:
    """Modular reduction."""
    return ((a % m) + m) % m


def pow_int(base: int, exp: int) -> int:
    """Integer power."""
    return base**exp


def general_crt(a1: int, m1: int, a2: int, m2: int) -> int:
    """Generalized Chinese Remainder Theorem.

    Returns x such that x≡a1 (mod m1) and x≡a2 (mod m2), where m1 and m2 need not be co-prime.
    Returns -1 if no solution exists.
    """
    g = gcd(m1, m2)
    if a1 % g != a2 % g:
        return -1
    lin = lin_comb(m1, m2)
    return mod(a1 - m1 * lin.a * (a1 - a2) // g, m1 * m2 // g)


def identical_shifts(vals: List[int], k: int) -> Set[int]:
    """Returns all i such that the values at i, i+k, i+2k, ..., i-k (wrapping around) equal the
    values at 0, k, 2k, ...
    """
    H = 3
    pow_h = pow_int(H, len(vals) // k)
    target_hash = 0
    for i in range(0, len(vals), k):
        target_hash = target_hash * H + vals[i]

    shifts: Set[int] = set()
    for initial_shift in range(k):
        hash_val = 0
        for i in range(initial_shift, len(vals), k):
            hash_val = hash_val * H + vals[i]
        for i in range(initial_shift, len(vals), k):
            if hash_val == target_hash:
                shifts.add(i)
            hash_val = hash_val * H + (1 - pow_h) * vals[i]
    return shifts


class ProcessedRegions:
    """Contains an entry (start, end) for each interval. No two entries intersect."""

    def __init__(self) -> None:
        """Initialize processed regions."""
        self.intervals: List[tuple[Rad, Rad]] = []
        self.point = Rad(0, 0)

    def advance_to_next_unprocessed(self) -> None:
        """Advance to next unprocessed point."""
        # Find interval containing point
        for start, end in self.intervals:
            if start.compare_to(self.point) <= 0 and self.point.compare_to(end) < 0:
                self.point = end
                return
        # If not found, point is at end of last interval or at 0
        if self.intervals:
            self.point = self.intervals[-1][1]
        else:
            self.point = Rad(0, 0)

    def process_interval(self, start: Rad, end: Rad) -> None:
        """Process an interval, merging with existing intervals."""
        # Check if start is within an existing interval
        merged_start = start
        merged_end = end
        new_intervals: List[tuple[Rad, Rad]] = []
        for s, e in self.intervals:
            # Check for overlap: intervals overlap if not (e < start or end < s)
            if not (e.compare_to(start) < 0 or end.compare_to(s) < 0):
                # Overlap - merge
                merged_start = merged_start.min(s)
                merged_end = merged_end.max(e)
            else:
                # No overlap - keep interval
                new_intervals.append((s, e))
        new_intervals.append((merged_start, merged_end))
        # Sort by start value
        new_intervals.sort(key=lambda x: (x[0].a, x[0].b))
        self.intervals = new_intervals

    def done(self) -> bool:
        """Check if all regions processed."""
        return self.point.compare_to(Rad.ONE()) >= 0


def order(flip_sizes: List[Rad]) -> int:
    """Compute order for given flip sizes."""
    orders: Set[int] = {0}
    period = 1
    regions = ProcessedRegions()
    while not regions.done():
        regions.advance_to_next_unprocessed()
        start_pos = Pos(regions.point, False)
        positions: List[Pos] = []
        uncut_region = Rad.ONE()
        pos = start_pos
        first = True
        while first or not (
            pos.angle.compare_to(start_pos.angle) == 0 and pos.flipped == start_pos.flipped
        ):
            first = False
            for size in flip_sizes:
                positions.append(pos)
                remaining = Rad.ONE().subtract(pos.angle)
                if pos.angle.compare_to(size) < (1 if pos.flipped else 0):
                    uncut_region = uncut_region.min(
                        pos.angle if pos.flipped else size.subtract(pos.angle)
                    )
                    pos = Pos(remaining, not pos.flipped)
                else:
                    uncut_region = uncut_region.min(
                        pos.angle.subtract(size) if pos.flipped else remaining
                    )
                    pos = Pos(pos.angle.subtract(size), pos.flipped)

        curr_region_orders = identical_shifts(
            [1 if p.flipped else 0 for p in positions], len(flip_sizes)
        )
        curr_period = len(positions)
        new_orders: Set[int] = set()
        for order_val in orders:
            for curr_order in curr_region_orders:
                new_order = general_crt(order_val, period, curr_order, curr_period)
                if new_order != -1:
                    new_orders.add(new_order)
        period = lcm(period, curr_period)
        orders = new_orders

        for i in range(0, len(positions), len(flip_sizes)):
            pos = positions[i]
            if not pos.flipped:
                regions.process_interval(pos.angle, pos.angle.add(uncut_region))

    orders.discard(0)
    return min(orders) if orders else period


def F(a: int, b: int, c: int) -> int:
    """Compute F(a, b, c)."""
    global sqrt_c, d
    if is_sq(c):
        sqrt_c = 1.0
        d = a * b * isqrt(c)
        return order([Rad(d // a, 0), Rad(d // b, 0), Rad(d // isqrt(c), 0)])
    else:
        sqrt_c = sqrt(c)
        d = a * b * c
        return order([Rad(d // a, 0), Rad(d // b, 0), Rad(0, d // c)])


def solve() -> int:
    """Solve Problem 566."""
    ans = 0
    for a in range(9, N):
        for b in range(a + 1, N):
            for c in range(b + 1, N + 1):
                ans += F(a, b, c)
    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
