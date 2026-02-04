"""Project Euler Problem 264: Triangle Centres.

Find the sum of the perimeters of all triangles with lattice point vertices,
circumcenter at the origin, orthocenter at (5, 0), and perimeter at most N.

Key optimization: the divisibility condition (D + Ay^2) | 2*Ay^2*R2 simplifies to
(D + Ay^2) | 2*D*W where D = (5-Ax)^2 and W = 10*Ax - 25.
So product = |2*D*W| = 10*(5-Ax)^2*|2*Ax-5|.
We enumerate divisors of product to find valid Ay values.
"""

from __future__ import annotations

from math import hypot, isqrt
from collections import Counter


def sq(n: int) -> int:
    return n * n


def is_square(n: int) -> bool:
    if n < 0:
        return False
    r = isqrt(n)
    return r * r == n


def factorize_with_spf(n: int, spf: list[int]) -> Counter:
    """Factorize n using smallest prime factor table."""
    factors = Counter()
    while n > 1:
        p = spf[n]
        while n % p == 0:
            factors[p] += 1
            n //= p
    return factors


def divisors_from_factors(factors: Counter) -> list[int]:
    """Generate all divisors from prime factorization."""
    divs = [1]
    for p, e in factors.items():
        new_divs = []
        pe = 1
        for _ in range(e + 1):
            for d in divs:
                new_divs.append(d * pe)
            pe *= p
        divs = new_divs
    return divs


def solve() -> float:
    """Solve Problem 264."""
    N = 100000
    max_ax = N // 4  # 25000

    # Precompute smallest prime factor up to 2*max_ax + 5 = 50005
    # We need to factorize |5-Ax| and |2*Ax-5| for Ax up to 25000
    # |5-Ax| up to 24995, |2*Ax-5| up to 49995
    SPF_LIMIT = 50010
    spf = list(range(SPF_LIMIT + 1))
    for i in range(2, isqrt(SPF_LIMIT) + 1):
        if spf[i] == i:  # prime
            for j in range(i * i, SPF_LIMIT + 1, i):
                if spf[j] == j:
                    spf[j] = i

    triangles: dict[frozenset, float] = {}

    for Ax in range(max_ax + 1):
        D = sq(5 - Ax)
        W = 10 * Ax - 25

        parity_start = Ax % 2 + 1
        ay_parity = parity_start % 2

        if D != 0 and W != 0:
            # product = |2 * D * W| = 2 * (5-Ax)^2 * |10*Ax - 25|
            # = 10 * (5-Ax)^2 * |2*Ax - 5|
            u = abs(5 - Ax)
            v = abs(2 * Ax - 5)

            # Factorize u and v using SPF
            factors = Counter()
            factors[2] = 1
            factors[5] = 1
            # u^2
            uf = factorize_with_spf(u, spf)
            for p, e in uf.items():
                factors[p] += 2 * e
            # v
            vf = factorize_with_spf(v, spf)
            for p, e in vf.items():
                factors[p] += e

            for d in divisors_from_factors(factors):
                ay_sq = d - D
                if ay_sq <= 0:
                    continue
                if not is_square(ay_sq):
                    continue
                Ay = isqrt(ay_sq)
                if Ay % 2 != ay_parity:
                    continue
                if Ay > max_ax:
                    continue

                _process_point(Ax, Ay, N, triangles)

        elif D == 0:
            # Ax = 5
            for Ay in range(parity_start, max_ax + 1, 2):
                _process_point(Ax, Ay, N, triangles)

        # W == 0 means Ax = 5/2 which is never integer, so skip

        # Handle Ay = 0 case
        disc_val = 4 * sq(Ax) - sq(5 - Ax)
        if is_square(disc_val):
            By = isqrt(disc_val) // 2
            if By > 0:
                p1 = (Ax, 0)
                p2 = ((5 - Ax) // 2, By)
                p3 = ((5 - Ax) // 2, -By)
                perim = (
                    hypot(p2[0]-p1[0], p2[1]-p1[1]) +
                    hypot(p3[0]-p2[0], p3[1]-p2[1]) +
                    hypot(p1[0]-p3[0], p1[1]-p3[1])
                )
                if perim <= N:
                    key = frozenset([p1, p2, p3])
                    triangles[key] = perim

    return sum(triangles.values())


def _process_point(Ax: int, Ay: int, N: int, triangles: dict) -> None:
    """Process a candidate (Ax, Ay) point and add valid triangles."""
    R2 = sq(Ax) + sq(Ay)
    num = 2 * sq(Ay) * R2
    den = sq(5 - Ax) + sq(Ay)
    if den == 0 or num % den != 0:
        return
    disc = 2 * num // den - sq(Ay)
    if disc < 0 or not is_square(disc):
        return

    sqrt_disc = isqrt(disc)
    Bx = ((5 - Ax) + sqrt_disc) // 2
    Cx = 5 - Ax - Bx
    By_sq = R2 - sq(Bx)
    if By_sq < 0:
        return
    if not is_square(By_sq):
        return
    By = isqrt(By_sq)
    if sq(Cx) + sq(Ay + By) != R2:
        By = -By

    p1 = (Ax, Ay)
    p2 = (Bx, By)
    p3 = (Cx, -Ay - By)
    perim = (
        hypot(p2[0]-p1[0], p2[1]-p1[1]) +
        hypot(p3[0]-p2[0], p3[1]-p2[1]) +
        hypot(p1[0]-p3[0], p1[1]-p3[1])
    )

    area2 = abs(p1[0]*(p2[1]-p3[1]) + p2[0]*(p3[1]-p1[1]) + p3[0]*(p1[1]-p2[1]))
    if perim <= N and area2 > 0:
        key = frozenset([p1, p2, p3])
        triangles[key] = perim
        rp1 = (Ax, -Ay)
        rp2 = (Bx, -By)
        rp3 = (Cx, Ay + By)
        rkey = frozenset([rp1, rp2, rp3])
        triangles[rkey] = perim


def main() -> None:
    """Main entry point."""
    result = solve()
    print(f"{result:.4f}")


if __name__ == "__main__":
    main()
