"""Project Euler Problem 702: Jumping Flea.

A hexagon with side length N is divided into unit equilateral triangles. Let
J(T) be the minimum number of jumps for a flea starting at the center of the
hexagon to jump into the interior of an equilateral triangle T, if each jump
consists of choosing one of the 6 corners of the hexagon and jumping to the
midpoint of the current location and the corner.

Find ΣJ(T) for all upward facing triangles in the top half of the hexagon.

Construct a coordinate system where the center of the hexagon is at (0,0), D is
at (N,0), C is at (N,N), and B is at (0,N). Note the hexagon is sheared.

It is easy to see that after j jumps, the possible destination points are of
the form (aN/2^j, bN/2^j), for integer a,b, that are within the hexagon.

If N/2^j > 1, then it is impossible for multiple of these points to be in the
same triangle. As long as a≠0, b≠0, and a≠b (mod 2^j), the point (aN/2^j,
bN/2^j) will be within a triangle.

There are nCr(2^j-1,2) such points in every 1/6 triangle of the hexagon. For
the (2^j-1)(2^j-2) points in the first quadrant, exactly half must lie in the
upward-facing triangles, by symmetry. For the second quadrant, there are
nCr(2^j-1,2) total points, but only the ones where aN/2^j (mod 1) > bN/2^j
(mod 1) will lie in an upward-facing triangle.

To count these, we count the number of a,b with a<b in the sequence
(N,2N,3N,...,(2^j-1)N) (mod 2^j). This is equal to nCr(2^j-1,2) minus the
number of inversions a,b with a>b.

To compute numInversions(m,n) in general, which is the number of a,b with a>b
in the sequence (a,2a,3a,...,m*a) (mod m), we split the sequence into
contiguous subsequences, each one consisting of increasing terms and maximal.
Each subsequence consists of either ⌈m/n⌉ or ⌊m/n⌋ terms. There are m%n of the
former "big" subsequences, and n-m%n of the latter "small" subsequences.

By default, the number of inversions between terms in two subsequences is
tr(⌊m/n⌋). However, for two big subsequences, there are tr(⌈m/n⌉) (so ⌈m/n⌉
more) if the first term of the first subsequence is larger, i.e. the two first
terms is an inversion in the sequence of first terms. Similarly, for two small
subsequences, there are tr(⌊m/n⌋-1) (so ⌊m/n⌋ fewer) inversions.

Using these facts, we can compute numInversions(m,n) recursively using the
inversions of the sequences of first terms of the big and small subsequences,
numInversions(n%(m%n),m%n) and numInversions(n%(n-m%n),n-m%n).

This lets us compute the number of triangles with a given J(T) for N/2^j > 1.
However, if 0.5 < N/2^j < 1, then it is possible for three points to be in
the same triangle, resulting in over-counting. This happens if one of the
points satisfies xN/2^j, yN/2^j < 2^j-N. We can count these by recursion on
the smaller grid of points (xK/N, yK/N), where K≡-L (mod N). For each such
point, there are two other points in the same triangle we need to subtract.

Finally, if N/2^j < 0.5, then it is impossible for any triangle to not have a
point, so all 2tr(N) + tr(N-1) upward-facing triangles in the top half of the
hexagon have a point.

We can now easily compute the number of triangles that have a point
(aN/2^j, bN/2^j) but not (aN/2^{j-1}, bN/2^{j-1}) for all j, and get our
answer.
"""

from __future__ import annotations

from math import ceil, isqrt


def tr(n: int) -> int:
    """Triangular number: n*(n+1)//2."""
    return n * (n + 1) // 2


def ncr(n: int, r: int) -> int:
    """Binomial coefficient C(n, r)."""
    if r < 0 or r > n:
        return 0
    if r == 0 or r == n:
        return 1
    result = 1
    for i in range(min(r, n - r)):
        result = result * (n - i) // (i + 1)
    return result


def iceil_pow(n: int, base: int) -> int:
    """Find smallest power of base >= n."""
    if n <= 1:
        return 1
    result = 1
    while result < n:
        result *= base
    return result


def imod(a: int, m: int) -> int:
    """Modulo operation: a mod m."""
    return a % m


def num_inversions(n: int, m: int) -> int:
    """Count inversions in sequence (n, 2n, 3n, ..., m*n) mod m."""
    if n == 0:
        return 0

    big = m % n
    small = n - big

    count = ncr(n, 2) * tr(m // n)
    if big > 0:
        count += num_inversions(n % big, big) * ceil(m / n)
    if small > 0:
        count -= num_inversions(n % small, small) * (m // n)
    return count


def num_points_in_shaded(n: int, point_dist: int) -> int:
    """Count points in shaded triangles."""
    return (point_dist - 2) * (point_dist - 1) - num_inversions(
        n % point_dist, point_dist
    )


def solve() -> int:
    """Solve Problem 702."""
    n = 123456789
    l = iceil_pow(n, 2)

    num_shaded_with_j: list[int] = []
    k = 1
    while k < l:
        num_shaded_with_j.append(num_points_in_shaded(n, k))
        k *= 2

    num_shaded_with_j.append(
        num_points_in_shaded(n, l)
        - 2 * num_points_in_shaded(imod(-l, n), l - n)
    )
    num_shaded_with_j.append(2 * tr(n) + tr(n - 1))

    ans = 0
    for j in range(1, len(num_shaded_with_j)):
        ans += j * (num_shaded_with_j[j] - num_shaded_with_j[j - 1])
    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
