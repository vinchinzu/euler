"""Project Euler Problem 295: Lenticular Holes.

Find L(100000), the number of distinct lenticular pairs (r1, r2) with
0 < r1 <= r2 <= N, where two circles of radii r1 and r2 centered at
lattice points intersect at two lattice points with no lattice points
in the interior of their intersection.

Algorithm:
- Enumerate primitive chord directions (p, q) via Stern-Brocot tree
- For each chord, compute valid r^2 values
- Count pairs per chord group, then apply inclusion-exclusion to
  remove overcounting from r^2 values shared across chord groups
"""

from collections import Counter, defaultdict
from itertools import combinations


def solve():
    N = 100000
    N2 = N * N

    # Build chord groups using iterative Stern-Brocot tree
    chords = defaultdict(list)
    stack = [(0, 1, 1, 0)]
    while stack:
        p1, q1, p2, q2 = stack.pop()
        p = p1 + p2
        q = q1 + q2
        if p * p + q * q > 4 * N:
            continue
        if (p + q) % 2 == 0:
            len2 = p * p + q * q
            min_r2 = (
                (p1 * p1 + q1 * q1)
                * (p2 * p2 + q2 * q2)
                * len2
                // 4
            )
            chords[len2].append((p, q, min_r2))
        stack.append((p1, q1, p, q))
        stack.append((p, q, p2, q2))

    # For each chord group, compute r^2 values and track multiplicities
    r2_count = defaultdict(int)
    group_data = {}

    for len2, chord_list in chords.items():
        p, q, min_r2 = min(chord_list, key=lambda c: c[2])
        half_len2 = (p * p + q * q) // 2
        radii = []
        k = 0
        while True:
            rv = (2 * k * k + 2 * k + 1) * half_len2
            if rv > N2:
                break
            if rv >= min_r2:
                radii.append(rv)
            k += 1
        group_data[len2] = radii
        for rv in radii:
            r2_count[rv] += 1

    # Find r^2 values shared by multiple chord groups
    shared_r2 = frozenset(rv for rv, c in r2_count.items() if c > 1)
    del r2_count

    # Map shared r^2 values to their chord groups
    r2_to_groups = defaultdict(list)
    for len2, radii in group_data.items():
        for rv in radii:
            if rv in shared_r2:
                r2_to_groups[rv].append(len2)

    # Naive count: sum of C(n+1, 2) per chord group
    naive = sum(len(r) * (len(r) + 1) // 2 for r in group_data.values())

    # Inclusion-exclusion corrections for overcounting
    # For each subset size k >= 2, compute intersection sizes
    inter = [None, None, Counter(), Counter(), Counter()]
    for rv, gs in r2_to_groups.items():
        gs_sorted = tuple(sorted(gs))
        m = len(gs_sorted)
        for k in range(2, min(m, 4) + 1):
            for combo in combinations(gs_sorted, k):
                inter[k][combo] += 1

    corrections = [0, 0, 0, 0, 0]
    for k in range(2, 5):
        corrections[k] = sum(n * (n + 1) // 2 for n in inter[k].values())

    # PIE: alternating signs starting with - for k=2
    answer = naive - corrections[2] + corrections[3] - corrections[4]
    return answer


if __name__ == "__main__":
    print(solve())
