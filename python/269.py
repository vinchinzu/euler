"""Project Euler Problem 269: Polynomials with at least one integer root.

Find the number of polynomials of degree N-1 with coefficients in [0, 9] and
at least one integer root.

When evaluating a polynomial at j using Horner's Rule, if the value v from
evaluating a prefix of the polynomial is greater than zero and satisfies (v *
-j + 9) * -j ≥ v, then it is impossible for the value of the entire
polynomial to ever go down to zero. Similarly, it is impossible for the
entire polynomial to go down to zero if v is less than zero and satisfies (v
* -j * -j + 9) ≤ v.

Given these restrictions, we can use dynamic programming to compute for each
10-ple (v0, v1, v2, ... v9) the number of polynomials such that the values
after evaluating i steps of Horner's Rule at 0, 1, 2, ... 9 are v0, v1, v2,
... v9. For efficiency, we treat all v that satisfy the above restrictions as
equivalent (represented by an empty value). Once we have computed the counts
after evaluating all N steps of Horner's Rule, the answer is the sum of the
counts for all 10-ples (v0, v1, v2, ... v9) that have at least one value
equal to zero.
"""

from __future__ import annotations

from collections import defaultdict
from dataclasses import dataclass
from typing import Dict, List, Optional, Tuple


@dataclass(frozen=True)
class Key:
    """Key for DP state."""

    values: Tuple[Optional[int], ...]


def solve() -> int:
    """Solve Problem 269."""
    N = 16
    B = 10

    table: Dict[Key, int] = {
        Key(tuple([0] * B)): 1
    }

    for i in range(1, N + 1):
        new_table: Dict[Key, int] = defaultdict(int)
        for key, count in table.items():
            for d in range(B):
                new_values: List[Optional[int]] = []
                for j in range(B):
                    v = key.values[j]
                    if v is None:
                        new_values.append(None)
                    else:
                        new_v = v * (-j) + d
                        if new_v > 0 and (new_v * (-j) + B - 1) * (-j) >= new_v:
                            new_values.append(None)
                        elif new_v < 0 and (new_v * (-j) * (-j) + B - 1) <= new_v:
                            new_values.append(None)
                        else:
                            new_values.append(new_v)
                new_key = Key(tuple(new_values))
                new_table[new_key] += count
        table = new_table

    ans = 0
    for key, count in table.items():
        if 0 in key.values:
            ans += count

    return ans


def main() -> None:
    """Main entry point."""
    result = solve()
    print(result)


if __name__ == "__main__":
    main()
