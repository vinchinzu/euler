"""Project Euler Problem 600: Integer sided equi-angular hexagons.

Find the number of distinct equi-angular convex hexagons (all angles are
120°) with perimeter at most N.

An equi-angular hexagon must have sides that satisfy a+b=d+e and b+c=e+f.
We then use Burnside's Lemma over the 12 symmetries of the hexagon to
compute the number of distinct hexagons.
"""

from __future__ import annotations

from typing import Callable


def extrapolation(f: Callable[[int], int], n_points: int, mod: int) -> Callable[[int], int]:
    """Extrapolate function using Lagrange interpolation."""
    # Generate n_points values
    values = []
    for i in range(1, n_points + 1):
        values.append(f(i) % mod)

    def interpolate(x: int) -> int:
        """Interpolate at point x."""
        result = 0
        for i in range(n_points):
            term = values[i]
            for j in range(n_points):
                if i != j:
                    denom = (i + 1 - (j + 1)) % mod
                    if denom == 0:
                        continue
                    # Compute modular inverse
                    inv = pow(denom, mod - 2, mod)
                    term = (term * (x - (j + 1)) * inv) % mod
            result = (result + term) % mod
        return result

    return interpolate


def compute_exact(f: Callable[[int], int], mod: int) -> int:
    """Compute exact value using extrapolation."""
    interp_func = extrapolation(f, 8, mod)

    def compute_mod(n: int) -> int:
        """Compute modulo mod."""
        id_count = 0
        rot60 = 0
        rot120 = 0
        rot180 = 0
        flip_corner = 0
        flip_side = 0

        for a in range(1, n + 1):
            for b in range(1, n - a + 1):
                for c in range(1, n - a - b + 1):
                    for d in range(1, n - a - b - c + 1):
                        for e in range(1, n - a - b - c - d + 1):
                            f_val = n - a - b - c - d - e
                            if f_val <= 0:
                                continue
                            if a + b == d + e and b + c == e + f_val:
                                id_count += 1
                                if a == b == c == d == e == f_val:
                                    rot60 += 1
                                if a == c == e and b == d == f_val:
                                    rot120 += 1
                                if a == d and b == e and c == f_val:
                                    rot180 += 1
                                if a == d and b == c and e == f_val:
                                    flip_corner += 1
                                if b == f_val and c == e:
                                    flip_side += 1

        return (
            id_count + rot180 + 2 * (rot60 + rot120) + 3 * (flip_corner + flip_side)
        ) // 12

    return compute_mod


def solve() -> int:
    """Solve Problem 600."""
    N = 55106
    M = 10**9 + 7

    def f(n: int) -> int:
        """Count hexagons with perimeter <= n."""
        count = 0
        for a in range(1, n + 1):
            for b in range(1, n - a + 1):
                for c in range(1, n - a - b + 1):
                    for d in range(1, n - a - b - c + 1):
                        for e in range(1, n - a - b - c - d + 1):
                            f_val = n - a - b - c - d - e
                            if f_val > 0 and a + b == d + e and b + c == e + f_val:
                                count += 1
        return count % M

    result_func = compute_exact(f, M)
    ans = result_func(N) % M

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
