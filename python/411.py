"""Project Euler Problem 411: Station paths.

Let S(n) be the maximum number of stations that a path from (0, 0) to (n, n)
can pass through, given that there is a station at (x, y) if (x, y) ≡ (2^i,
3^i) for some 0 ≤ i ≤ 2n, and the x and y coordinates of the path can never
decrease. Find sum_{k=1}^N S(k^K).

To compute S(n), we can find all stations, sort them by x-coordinate (followed
by y-coordinate), and then find the longest increasing subsequence of the
y-coordinates using the LIS algorithm.

Optimizations:
- Consider just the x-coordinate. If n has e_2 factors of 2, then the first
  e_2 stations (2^i, 3^i) for 0 ≤ i < e_2 are not duplicated. If we define n_2
  to equal n with all factors of 2 divided out, then the station for i ≥ e_2 has
  the same x-coordinate as the station for i ≥ e_2 + order(2, n_2), where the
  order is the smallest o such that 2^o ≡ 1.
- Using similar logic for the y-coordinate to compute e_3 and n_3, we find that
  the station for i = max(e_2, e_3) + lcm(order(2, n_2), order(3, n_3)) is the
  same as the station for i = max(e_2, e_3), and we can stop generating stations
  after that amount.
- When sorting, we encode each station (x, y) as a 64-bit integer where the most
  significant 32 bits are the x-coordinate and the least significant 32 bits are
  the y-coordinate, to avoid creating objects.
"""

from __future__ import annotations

from math import gcd
from typing import List


def pow_mod(base: int, exp: int, mod: int) -> int:
    """Modular exponentiation."""
    result = 1
    base = base % mod
    while exp > 0:
        if exp & 1:
            result = (result * base) % mod
        base = (base * base) % mod
        exp >>= 1
    return result


def lcm(a: int, b: int) -> int:
    """Least common multiple."""
    return a * b // gcd(a, b)


def order(base: int, mod: int) -> int:
    """Find the order of base modulo mod (smallest o such that base^o ≡ 1)."""
    if gcd(base, mod) != 1:
        return 0
    phi = mod
    result = phi
    p = 2
    while p * p <= phi:
        if phi % p == 0:
            while phi % p == 0:
                phi //= p
            while result % p == 0:
                if pow_mod(base, result // p, mod) == 1:
                    result //= p
                else:
                    break
        p += 1
    if phi > 1:
        while result % phi == 0:
            if pow_mod(base, result // phi, mod) == 1:
                result //= phi
            else:
                break
    return result


def longest_increasing_subsequence(arr: List[int]) -> int:
    """Find the length of the longest increasing subsequence."""
    if not arr:
        return 0

    lowest_with_len: List[int] = []
    max_len = 0

    for val in arr:
        # Binary search for the position to insert
        left, right = 0, len(lowest_with_len)
        while left < right:
            mid = (left + right) // 2
            if lowest_with_len[mid] <= val:
                left = mid + 1
            else:
                right = mid

        if right == len(lowest_with_len):
            lowest_with_len.append(val)
        else:
            lowest_with_len[right] = val
        max_len = max(max_len, right + 1)

    return max_len


def S(n: int) -> int:
    """Compute S(n): maximum stations in a path from (0,0) to (n,n)."""
    # Find e_2 and n_2
    e_2 = 0
    n_2 = n
    while n_2 % 2 == 0:
        e_2 += 1
        n_2 //= 2

    # Find e_3 and n_3
    e_3 = 0
    n_3 = n
    while n_3 % 3 == 0:
        e_3 += 1
        n_3 //= 3

    # Compute number of stations needed
    ord_2 = order(2, n_2) if n_2 > 1 else 1
    ord_3 = order(3, n_3) if n_3 > 1 else 1
    num_stations = max(e_2, e_3) + lcm(ord_2, ord_3)

    # Generate stations encoded as 64-bit integers
    stations: List[int] = []
    x = 1 % n
    y = 1 % n
    for i in range(num_stations):
        # Encode (x, y) as (x << 32) | y
        encoded = (x << 32) | y
        stations.append(encoded)
        x = (x * 2) % n
        y = (y * 3) % n

    # Sort by x-coordinate (most significant 32 bits), then y-coordinate
    stations.sort()

    # Extract y-coordinates (least significant 32 bits)
    y_coords = [s & 0xFFFFFFFF for s in stations]

    # Find longest increasing subsequence of y-coordinates
    return longest_increasing_subsequence(y_coords)


def solve() -> int:
    """Solve Problem 411."""
    N = 30
    K = 5
    ans = 0
    for k in range(1, N + 1):
        n = pow(k, K)
        ans += S(n)
    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
