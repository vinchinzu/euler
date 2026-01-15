"""Project Euler Problem 572: Idempotent Matrices.

Find the number of 3x3 idempotent matrices (A²=A) with integer elements with
absolute value at most N.

First we consider the rank of the matrix A:
- There is only one rank zero matrix: the matrix of all zeros. It is idempotent.
- If the rank is 1, then let A = [r s t]ᵀ [x y z]. Note that the equations
  equating A²=A element-wise all reduce to the requirement that the trace
  r*x + s*y + t*z = 1. So we iterate over all possible triplets (r*x, s*y, t*z)
  summing to 1, and iterate over all possible ways to split each product into
  two factors. Note that at the end we need to divide by 2, because changing
  the sign of (r,s,t) is the same as changing the sign of (x,y,z).
- If the rank is 2, note that I-A is idempotent because (I-A)² = I²-2A+A² =
  I-A, and I-A has rank 1. So for each rank 1 idempotent matrix we find above,
  we also get a rank 2 idempotent matrix. The only issue is that the bounds are
  slightly changed; the rank 1 matrix cannot have -N in the diagonal, but can
  have N+1 in the diagonal.
- If the rank is 3, then A is invertible, which means A²=A => A=I, the identity
  matrix.

So we count the valid rank 1 and rank 2 matrices, divide by 2, and add 2 for
the all-zero matrix and identity matrix.
"""

from __future__ import annotations

from collections import defaultdict
from math import gcd
from typing import Dict, List, Tuple


def precompute_gcds(limit: int) -> List[List[int]]:
    """Precompute GCD table."""
    gcds = [[0] * (limit + 1) for _ in range(limit + 1)]
    for i in range(limit + 1):
        for j in range(limit + 1):
            gcds[i][j] = gcd(i, j)
    return gcds


def solve() -> int:
    """Solve Problem 572."""
    N = 200

    # Build multimap of divisors: product -> list of (a, b) pairs
    divisors: Dict[int, List[Tuple[int, int]]] = defaultdict(list)
    for a in range(-N, N + 1):
        for b in range(-N, N + 1):
            divisors[a * b].append((a, b))

    gcds = precompute_gcds(N)

    ans = 0
    for a in range(-N, N + 2):
        for e in range(-N, N + 2):
            i = 1 - a - e
            if i < -N or i > N + 1:
                continue
            for rx in divisors.get(a, []):
                r, x = rx
                for sy in divisors.get(e, []):
                    s, y = sy
                    if abs(r * y) > N or abs(s * x) > N:
                        continue
                    for tz in divisors.get(i, []):
                        t, z = tz
                        if (
                            abs(r * z) > N
                            or abs(s * z) > N
                            or abs(t * x) > N
                            or abs(t * y) > N
                        ):
                            continue
                        g = gcds[gcds[abs(r)][abs(s)]][abs(t)]
                        if g != 1:
                            continue
                        # Rank 1 case: a <= N, e <= N, i <= N
                        if a <= N and e <= N and i <= N:
                            ans += 1
                        # Rank 2 case: a > -N, e > -N, i > -N
                        if a > -N and e > -N and i > -N:
                            ans += 1

    ans //= 2
    ans += 2  # All-zero matrix and identity matrix
    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
