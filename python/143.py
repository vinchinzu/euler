"""Project Euler Problem 143: Investigating the Torricelli point of a triangle.

Find the sum of all distinct values of p + q + r <= 120000 for Torricelli triangles.
"""

import math
from typing import Dict, Set

L = 120000
max_m = int(math.sqrt(L) + 10)
pairs_map: Dict[int, Set[int]] = {}


def main() -> int:
    """Main function."""
    from math import gcd

    for m in range(2, max_m + 1):
        for n in range(1, m):
            if gcd(m, n) != 1:
                continue
            if (m - n) % 3 == 0:
                continue
            a = 2 * m * n + n * n
            b = m * m - n * n
            k = 1
            while True:
                pk = k * a
                qk = k * b
                if pk + qk > L:
                    break
                min_pq = min(pk, qk)
                max_pq = max(pk, qk)
                if min_pq not in pairs_map:
                    pairs_map[min_pq] = set()
                pairs_map[min_pq].add(max_pq)
                k += 1

    sums: Set[int] = set()
    for p, qs in pairs_map.items():
        for q in qs:
            if q not in pairs_map:
                continue
            common = pairs_map[p] & pairs_map[q]
            for r in common:
                if r <= q:
                    continue
                s = p + q + r
                if s > L:
                    continue
                sums.add(s)

    return sum(sums)


if __name__ == "__main__":
    print(main())
