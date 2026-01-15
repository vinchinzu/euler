"""Project Euler Problem 209: Circular Logic.

Find the number of boolean functions τ(a, b, c, d, e, f) such that τ AND τ∘F = 0.
"""

from __future__ import annotations

from itertools import product
from typing import List, Set, Tuple


def solve() -> int:
    """Solve Problem 209."""
    K = 6

    def F(bits: Tuple[bool, ...]) -> Tuple[bool, ...]:
        """Apply function F to bits."""
        a, b, c, d, e, f = bits
        return (b, c, d, e, f, a ^ (b and c))

    # num_good_cycles[n] = number of valid mappings for cycle of length n
    num_good_cycles: List[int] = [-1, 1, 3]
    while len(num_good_cycles) < (1 << K):
        num_good_cycles.append(num_good_cycles[-1] + num_good_cycles[-2])

    seen: Set[Tuple[bool, ...]] = set()
    ans = 1

    for bits_tuple in product([False, True], repeat=K):
        if bits_tuple in seen:
            continue

        cycle_len = 0
        current = bits_tuple
        while current not in seen:
            seen.add(current)
            current = F(current)
            cycle_len += 1

        if cycle_len > 0:
            ans *= num_good_cycles[cycle_len]

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
