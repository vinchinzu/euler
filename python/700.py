"""Project Euler Problem 700: Eulercoin.

Find the sum of all elements in the sequence N*n (mod M) that are strictly
smaller than all previous elements.

We brute force for small n. Once the minimum element is small enough, we
can go through all smaller values and find their positions in the sequence
instead, only taking a larger value if its position is earlier in the
sequence.
"""

from __future__ import annotations

from math import isqrt


def mod_inverse(a: int, m: int) -> int:
    """Modular inverse using extended Euclidean algorithm."""
    if m == 1:
        return 0
    t, new_t = 0, 1
    r, new_r = m, a % m
    while new_r != 0:
        q = r // new_r
        t, new_t = new_t, t - q * new_t
        r, new_r = new_r, r - q * new_r
    if r != 1:
        raise ValueError("Modular inverse does not exist")
    if t < 0:
        t += m
    return t


def solve() -> int:
    """Solve Problem 700."""
    N = 1504170715041707
    M = 4503599627370517
    L = isqrt(M)

    min_el = M
    ans = 0
    el = N
    for n in range(1, L + 1):
        if el < min_el:
            min_el = el
            ans += el
        el = (el + N) % M

    mod_inv = mod_inverse(N, M)
    min_n = M
    n = mod_inv
    for el in range(1, min_el):
        if n < min_n:
            min_n = n
            ans += el
        n = (n + mod_inv) % M

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
