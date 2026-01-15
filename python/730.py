"""Project Euler Problem 730: Shifted Pythagorean Triples.

Find the number of k-shifted Pythagorean triples, which are triples (p,q,r)
where p²+q²+k=r², GCD(p,q,r)=1, 1≤p≤q≤r, p+q+r≤N, and 0≤k≤K.

From https://en.wikipedia.org/wiki/Tree_of_primitive_Pythagorean_triples, we
can generate all primitive Pythagorean triples by applying the three Barning
matrices repeatedly to the base triple (3,4,5). These matrices when applied
to (p,q,r) keep p²+q²-r² constant, so given base solutions to p²+q²+k=r² for
any k, we can generate other solutions. So we use brute force to compute the
base solutions for each k, and for each one recursively apply the Barning
matrices to get all solutions for k.
"""

from __future__ import annotations

from math import gcd, isqrt


def is_sq(n: int) -> bool:
    """Check if n is a perfect square."""
    root = isqrt(n)
    return root * root == n


def sq(n: int) -> int:
    """Square of n."""
    return n * n


def solve() -> int:
    """Solve Problem 730."""
    n = 10**8
    k_max = 100
    l = 200

    used = [[[False] * l for _ in range(l)] for _ in range(k_max + 1)]
    ans = 0

    def helper(k: int, a: int, b: int, c: int) -> None:
        """Recursively generate triples using Barning matrices."""
        nonlocal ans
        if a + b + c > n:
            return
        if a > b:
            helper(k, b, a, c)
            return
        if a < l and b < l and k <= k_max:
            if used[k][a][b]:
                return
            used[k][a][b] = True
        ans += 1
        # Apply three Barning matrices
        helper(k, a - 2 * b + 2 * c, 2 * a - b + 2 * c, 2 * a - 2 * b + 3 * c)
        helper(k, a + 2 * b + 2 * c, 2 * a + b + 2 * c, 2 * a + 2 * b + 3 * c)
        if a != b:
            helper(
                k,
                -a + 2 * b + 2 * c,
                -2 * a + b + 2 * c,
                -2 * a + 2 * b + 3 * c,
            )

    # Find base solutions
    for k in range(k_max + 1):
        for p in range(1, l):
            for q in range(p, l):
                r2 = sq(p) + sq(q) + k
                if is_sq(r2):
                    r = isqrt(r2)
                    if p + q + r <= n and gcd(gcd(p, q), r) == 1:
                        helper(k, p, q, r)

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
