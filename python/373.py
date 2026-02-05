#!/usr/bin/env python3
"""
Project Euler Problem 373: Circumscribed Circles

S(n) = sum of circumradii of ALL integer-sided triangles with integer circumradius <= n.

Algorithm:
1. Generate all primitive Heronian triangles:
   a) Primitive Pythagorean triples (right triangles)
   b) Primitive non-right Heronian triangles (Brahmagupta parametrization)
2. For each primitive with R_prim = p/q, sum the circumradii of scaled triangles:
   R = p, 2p, 3p, ..., floor(R_max/p) * p

The key is ensuring we search wide enough parameter ranges in Brahmagupta.
"""

from math import gcd, isqrt


def compute_R_fraction(a, b, c):
    """Compute circumradius as fraction (numerator, denominator) in lowest terms."""
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


def generate_primitive_pythagorean_triples(p_max):
    """Generate all primitive Pythagorean triples with c <= p_max."""
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
    """Generate primitive non-right Heronian triangles where R_prim = p/q has p <= p_max."""
    primitives = set()

    # The raw R = (m^2+k^2)(n^2+k^2)/(4k)
    # For primitive, we divide by gcd of sides
    # The primitive's R can be significantly smaller than raw R

    # Need wider search to find all primitives
    # Empirically: for p_max = 100, we need m up to at least 15
    # Scale: max_m ~ O(p_max^0.5) or higher

    # Let's be generous with bounds
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

                # Get primitive form
                g = gcd(gcd(a, b), c)
                a_prim, b_prim, c_prim = a // g, b // g, c // g
                sides = tuple(sorted([a_prim, b_prim, c_prim]))

                # Skip right triangles
                if sides[0]**2 + sides[1]**2 == sides[2]**2:
                    continue

                R_frac = compute_R_fraction(*sides)
                if R_frac is None:
                    continue

                p, q = R_frac
                if p <= p_max:
                    primitives.add(sides)

    return primitives


def solve(R_max, verbose=False):
    """Compute S(R_max)."""

    total = 0
    count = 0

    # 1. Primitive Pythagorean triples: R_prim = c/2, p = c, q = 2
    pythagorean = generate_primitive_pythagorean_triples(R_max)
    if verbose:
        print(f"Found {len(pythagorean)} primitive Pythagorean triples with c <= {R_max}")

    for a0, b0, c0 in pythagorean:
        p, q = c0, 2

        max_k = R_max // p
        sum_R = p * max_k * (max_k + 1) // 2
        total += sum_R
        count += max_k

    # 2. Non-right primitive Heronian triangles
    non_right = generate_primitives_non_right_via_brahmagupta(R_max)
    if verbose:
        print(f"Found {len(non_right)} primitive non-right Heronian triangles with p <= {R_max}")

    for prim in non_right:
        R_frac = compute_R_fraction(*prim)
        if R_frac is None:
            continue

        p, q = R_frac

        if p > R_max:
            continue

        max_k = R_max // p
        sum_R = p * max_k * (max_k + 1) // 2
        total += sum_R
        count += max_k

    return total, count


# Test
print("Testing S(100):")
total, count = solve(100, verbose=True)
print(f"  S(100) = {total}, count = {count}, expected S(100) = 4950, count = 80")

print("\nTesting S(1200):")
total, count = solve(1200, verbose=True)
print(f"  S(1200) = {total}, count = {count}, expected S(1200) = 1653605")
