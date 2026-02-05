#!/usr/bin/env python3
"""
Debug S(200) discrepancy.
"""

from math import gcd, isqrt


def compute_R_int(a, b, c):
    """Compute circumradius if it's an integer, else return None."""
    if a + b <= c or a + c <= b or b + c <= a:
        return None

    s = a + b + c
    p1 = -a + b + c
    p2 = a - b + c
    p3 = a + b - c

    k2_times_16 = s * p1 * p2 * p3

    if k2_times_16 <= 0:
        return None

    k_times_4 = isqrt(k2_times_16)
    if k_times_4 * k_times_4 != k2_times_16:
        return None

    num = a * b * c
    if num % k_times_4 != 0:
        return None

    return num // k_times_4


def compute_R_fraction(a, b, c):
    """Compute circumradius as fraction (p, q) in lowest terms."""
    if a + b <= c or a + c <= b or b + c <= a:
        return None

    s = a + b + c
    p1 = -a + b + c
    p2 = a - b + c
    p3 = a + b - c

    k2_times_16 = s * p1 * p2 * p3

    if k2_times_16 <= 0:
        return None

    k_times_4 = isqrt(k2_times_16)
    if k_times_4 * k_times_4 != k2_times_16:
        return None

    num = a * b * c
    den = k_times_4
    g = gcd(num, den)
    return (num // g, den // g)


def solve_brute(N, max_side=None):
    """Brute force - sum all circumradii."""
    if max_side is None:
        max_side = 2 * N

    triangles = []

    for c in range(1, max_side + 1):
        for b in range(1, c + 1):
            min_a = max(1, c - b + 1)
            for a in range(min_a, b + 1):
                R = compute_R_int(a, b, c)
                if R is not None and R <= N:
                    triangles.append((a, b, c, R))

    return triangles


# Test S(1000)
N = 1000
print(f"Computing brute force for S({N})...")
triangles = solve_brute(N, max_side=2200)
total_brute = sum(t[3] for t in triangles)
count_brute = len(triangles)
print(f"Brute force: total={total_brute}, count={count_brute}")

# Get primitives from brute force
prim_to_instances = {}
for a, b, c, R in triangles:
    g = gcd(gcd(a, b), c)
    prim = (a // g, b // g, c // g)
    if prim not in prim_to_instances:
        prim_to_instances[prim] = []
    prim_to_instances[prim].append((a, b, c, R, g))

print(f"Number of primitives from brute force: {len(prim_to_instances)}")

# Get primitives from parametric approach
def generate_primitive_pythagorean_triples(p_max):
    triples = []
    max_m = isqrt(p_max) + 1

    for m in range(2, max_m + 1):
        for n in range(1, m):
            if (m - n) % 2 == 0:
                continue
            if gcd(m, n) != 1:
                continue

            a = m * m - n * n
            b = 2 * m * n
            c = m * m + n * n

            if c > p_max:
                continue

            if a > b:
                a, b = b, a

            triples.append((a, b, c))

    return triples


def generate_primitives_non_right_via_brahmagupta(p_max):
    primitives = set()

    max_param = max(60, int(p_max ** 0.6) + 20)

    for k in range(1, max_param + 1):
        k2 = k * k

        for m in range(1, max_param + 1):
            m2 = m * m
            m2_k2 = m2 + k2

            for n in range(1, m + 1):
                n2 = n * n
                n2_k2 = n2 + k2

                if m * n <= k2:
                    continue

                a = n * m2_k2
                b = m * n2_k2
                c = (m + n) * (m * n - k2)

                if c <= 0:
                    continue

                g = gcd(gcd(a, b), c)
                a_prim, b_prim, c_prim = a // g, b // g, c // g
                sides = tuple(sorted([a_prim, b_prim, c_prim]))

                if sides[0]**2 + sides[1]**2 == sides[2]**2:
                    continue

                R_frac = compute_R_fraction(*sides)
                if R_frac is None:
                    continue

                p, q = R_frac
                if p <= p_max:
                    primitives.add(sides)

    return primitives


pythagorean = generate_primitive_pythagorean_triples(N)
non_right = generate_primitives_non_right_via_brahmagupta(N)

print(f"Parametric Pythagorean: {len(pythagorean)}")
print(f"Parametric non-right: {len(non_right)}")

# Compute parametric sum
total_param = 0
count_param = 0

for a0, b0, c0 in pythagorean:
    p, q = c0, 2
    max_k = N // p
    sum_R = p * max_k * (max_k + 1) // 2
    total_param += sum_R
    count_param += max_k

for prim in non_right:
    R_frac = compute_R_fraction(*prim)
    if R_frac is None:
        continue
    p, q = R_frac
    if p > N:
        continue
    max_k = N // p
    sum_R = p * max_k * (max_k + 1) // 2
    total_param += sum_R
    count_param += max_k

print(f"Parametric: total={total_param}, count={count_param}")

# Find missing primitives
brute_prims = set(prim_to_instances.keys())
param_prims = set(pythagorean) | non_right

missing = brute_prims - param_prims
extra = param_prims - brute_prims

if missing:
    print(f"\nMissing from parametric ({len(missing)}):")
    for prim in sorted(missing):
        R_frac = compute_R_fraction(*prim)
        is_right = (prim[0]**2 + prim[1]**2 == prim[2]**2)
        instances = prim_to_instances[prim]
        print(f"  {prim}: R={R_frac}, right={is_right}, instances={len(instances)}")

if extra:
    print(f"\nExtra in parametric ({len(extra)}):")
    for prim in sorted(extra)[:10]:
        R_frac = compute_R_fraction(*prim)
        print(f"  {prim}: R={R_frac}")
