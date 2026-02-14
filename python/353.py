#!/usr/bin/env python3
"""Project Euler Problem 353 - Minimal risk paths on a spherical grid.

Port of the Java reference using:
1. Efficient station enumeration via sum-of-two-squares factorization
2. Spatial regions (LxLxL boxes) for neighbor pruning
3. Heap-based Dijkstra
"""

import math
import heapq
from collections import defaultdict


def factorize(n):
    """Return prime factorization of n as list of (p, e) pairs."""
    factors = []
    d = 2
    while d * d <= n:
        if n % d == 0:
            e = 0
            while n % d == 0:
                n //= d
                e += 1
            factors.append((d, e))
        d += 1
    if n > 1:
        factors.append((n, 1))
    return factors


def sum_of_two_squares_reps(n):
    """Find all representations of n as y^2 + z^2 with y >= 0, z >= 0.

    Uses the factorization of n. A number can be represented as sum of two
    squares iff all prime factors of form 4k+3 appear to even powers.

    We enumerate using Gaussian integers: for each prime p = 4k+1, find a+bi
    with a^2+b^2=p, then combine using multiplication of Gaussian integers.
    """
    if n == 0:
        return [(0, 0)]
    if n < 0:
        return []

    factors = factorize(n)

    # Check feasibility: primes ≡ 3 (mod 4) must have even exponent
    for p, e in factors:
        if p % 4 == 3 and e % 2 == 1:
            return []

    # Start with representations = {(1, 0)}
    # For each prime factor, multiply Gaussian integers
    reps = [(1, 0)]

    for p, e in factors:
        if p == 2:
            # 2 = (1+i)(1-i), so 2 = 1^2 + 1^2
            # Multiply by (1+i)^e
            gauss = (1, 1)
            new_reps = []
            for a, b in reps:
                ca, cb = a, b
                for _ in range(e):
                    # multiply (ca + cb*i) by (1 + i) = (ca - cb) + (ca + cb)*i
                    ca, cb = ca - cb, ca + cb
                new_reps.append((ca, cb))
            reps = new_reps
        elif p % 4 == 1:
            # Find a, b with a^2 + b^2 = p using Cornacchia/trial
            a, b = _find_sq_rep(p)
            # We need to combine with exponent e
            # For each existing rep, multiply by (a+bi)^j * (a-bi)^(e-j) for j=0..e
            new_reps = []
            for ra, rb in reps:
                # Compute all products of (a+bi)^j * (a-bi)^(e-j)
                # Start: powers of (a+bi) from 0 to e
                pow_plus = [(1, 0)]  # (a+bi)^0
                for _ in range(e):
                    x, y = pow_plus[-1]
                    pow_plus.append((x * a - y * b, x * b + y * a))

                pow_minus = [(1, 0)]  # (a-bi)^0
                for _ in range(e):
                    x, y = pow_minus[-1]
                    pow_minus.append((x * a + y * b, -x * b + y * a))

                for j in range(e + 1):
                    # (a+bi)^j * (a-bi)^(e-j)
                    px, py = pow_plus[j]
                    mx, my = pow_minus[e - j]
                    gx = px * mx - py * my
                    gy = px * my + py * mx
                    # Multiply with (ra, rb)
                    fx = ra * gx - rb * gy
                    fy = ra * gy + rb * gx
                    new_reps.append((fx, fy))
            reps = new_reps
        else:
            # p % 4 == 3, e must be even
            # p^e contributes p^(e/2) to both components as real factor
            factor = p ** (e // 2)
            reps = [(a * factor, b * factor) for a, b in reps]

    # Normalize: take abs values and ensure y >= 0, z >= 0, unique pairs
    result = set()
    for a, b in reps:
        a, b = abs(a), abs(b)
        if a * a + b * b == n:
            result.add((min(a, b), max(a, b)))
            result.add((max(a, b), min(a, b)))
            # Actually we want all (y, z) with y >= 0, z >= 0
            # Both (a, b) and (b, a) are valid if different
    # Return unique pairs with y >= 0, z >= 0
    final = set()
    for a, b in reps:
        a, b = abs(a), abs(b)
        if a * a + b * b == n:
            final.add((a, b))
            final.add((b, a))
            # Also add (-a, b) etc but we want y,z >= 0 so only non-negative
    return sorted(final)


def _find_sq_rep(p):
    """Find a, b with a^2 + b^2 = p for prime p ≡ 1 (mod 4)."""
    # Use Cornacchia's algorithm
    # Find r with r^2 ≡ -1 (mod p)
    r = _sqrt_neg1_mod(p)
    # Apply Euclidean algorithm
    a, b = p, r
    limit = math.isqrt(p)
    while b > limit:
        a, b = b, a % b
    return b, math.isqrt(p - b * b)


def _sqrt_neg1_mod(p):
    """Find r with r^2 ≡ -1 (mod p) for prime p ≡ 1 (mod 4)."""
    # Find a quadratic non-residue
    for g in range(2, p):
        r = pow(g, (p - 1) // 4, p)
        if (r * r) % p == p - 1:
            return r
    return None


def enumerate_stations(r):
    """Enumerate all integer points (x,y,z) on sphere of radius r with y>=0, z>=0.

    For each x from 0 to r, compute r^2 - x^2 and find all (y,z) with y^2+z^2 = r^2-x^2
    and y>=0, z>=0.
    """
    r2 = r * r
    stations = []

    for x in range(0, r + 1):
        remainder = r2 - x * x
        reps = sum_of_two_squares_reps(remainder)
        for y, z in reps:
            if y >= 0 and z >= 0:
                stations.append((x, y, z))

    return stations


def compute_m_r(r):
    """Compute M(r) using spatial-region Dijkstra."""
    L = 300  # Region box size

    # Get stations with y >= 0, z >= 0 (matching Java's sumsOfThreeSquares)
    stations = enumerate_stations(r)

    # Add reflections for x != 0 (negative x)
    full_stations = []
    for x, y, z in stations:
        full_stations.append((x, y, z))
        if x != 0:
            full_stations.append((-x, y, z))

    # Sort by x descending (matching Java)
    full_stations.sort(key=lambda p: -p[0])

    n = len(full_stations)
    r2 = r * r

    # Build spatial regions
    regions = defaultdict(list)
    for i, (x, y, z) in enumerate(full_stations):
        rx = (x + r) // L + 1
        ry = (y + r) // L + 1
        rz = (z + r) // L + 1
        regions[(rx, ry, rz)].append(i)

    # Dijkstra from index 0 to index n-1
    INF = float('inf')
    risks = [INF] * n
    risks[0] = 0.0
    visited = [False] * n

    # Min-heap: (risk, index)
    heap = [(0.0, 0)]

    while heap:
        curr_risk, i = heapq.heappop(heap)
        if visited[i]:
            continue
        visited[i] = True

        if i == n - 1:
            return curr_risk

        sx, sy, sz = full_stations[i]
        rx = (sx + r) // L + 1
        ry = (sy + r) // L + 1
        rz = (sz + r) // L + 1

        # Check all 27 neighboring regions
        for dx in range(-1, 2):
            for dy in range(-1, 2):
                for dz in range(-1, 2):
                    key = (rx + dx, ry + dy, rz + dz)
                    if key in regions:
                        for j in regions[key]:
                            if not visited[j]:
                                # Compute risk of edge (i, j)
                                jx, jy, jz = full_stations[j]
                                dot = sx * jx + sy * jy + sz * jz
                                cos_theta = max(min(dot / r2, 1.0), -1.0)
                                theta = math.acos(cos_theta)
                                edge_risk = (theta / math.pi) ** 2
                                new_risk = curr_risk + edge_risk
                                if new_risk < risks[j]:
                                    risks[j] = new_risk
                                    heapq.heappush(heap, (new_risk, j))

    return risks[n - 1]


def main():
    total = 0.0
    for k in range(1, 16):
        r = (1 << k) - 1
        m_r = compute_m_r(r)
        total += m_r
    print(f"{total:.10f}")


if __name__ == "__main__":
    main()
