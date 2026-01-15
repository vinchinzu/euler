"""Project Euler Problem 574: Verifying Primes.

Let V(p) be the smallest A in a triplet (A,B,q) such that A≥B>0, GCD(A,B) = 1,
AB is divisible by every prime less than q, p<q², and p=A+B or p=A-B. Find the
sum of V(p) for all primes p less than N.

First, we can choose q to be as small as possible such that p<q², since the
other requirement is strictly more exclusive for higher q. Then we can test all
A<p for whether A and B=p-A have a product divisible by all primes less than q.

If not, then A>p and we must have p=A-B. Consider a subset P of the primes
less than q. Some of those primes (say with product c0) must divide A, and the
rest (with product c1) must divide B. Let A=c0*x and B=c1*y, then we can
solve for (x,y) such that c0*x - c1*y = p, this gives x up to (mod c1), or
A=c0*x up to (mod c0*c1 = Π P). So we need to only check all possible values
of x (mod Π P) for different (c0, c1). The smallest one such that AB also
divides the primes other than P (call them Q) is V(p).
"""

from __future__ import annotations

from math import gcd, isqrt
from typing import List, Set, Tuple


def sieve(limit: int) -> List[int]:
    """Generate all primes up to limit using Sieve of Eratosthenes."""
    if limit < 2:
        return []
    is_prime = [True] * (limit + 1)
    is_prime[0] = is_prime[1] = False
    for i in range(2, isqrt(limit) + 1):
        if is_prime[i]:
            for j in range(i * i, limit + 1, i):
                is_prime[j] = False
    return [i for i in range(limit + 1) if is_prime[i]]


def extended_gcd(a: int, b: int) -> Tuple[int, int, int]:
    """Extended Euclidean algorithm.

    Returns (gcd, x, y) such that a*x + b*y = gcd(a, b).
    """
    if b == 0:
        return (a, 1, 0)
    g, x1, y1 = extended_gcd(b, a % b)
    return (g, y1, x1 - (a // b) * y1)


def linear_combination(a: int, b: int, c: int) -> Tuple[int, int]:
    """Find (x, y) such that a*x + b*y = c."""
    g, x0, y0 = extended_gcd(a, b)
    if c % g != 0:
        return (0, 0)
    factor = c // g
    return (x0 * factor, y0 * factor)


def solve() -> int:
    """Solve Problem 574."""
    N = 3800

    primes = sieve(N)
    ans = 0

    def V(p: int) -> int:
        """Compute V(p)."""
        # Find q such that p < q²
        q_primes = sieve(isqrt(p))
        if not q_primes:
            return 0

        # Split primes into P and Q
        P = 1
        Q = 1
        cutoff = 0
        while Q < (1 << len(q_primes)):
            Q *= q_primes[cutoff]
            cutoff += 1
            if cutoff >= len(q_primes):
                break

        for i in range(cutoff, len(q_primes)):
            P *= q_primes[i]

        # Try A < p case: p = A + B
        for A in range((p + 1) // 2, p):
            B = p - A
            if A * B % (P * Q) == 0:
                return A

        # Try A > p case: p = A - B
        # Generate all residues mod P
        residues: Set[int] = set()
        num_subsets = 1 << (len(q_primes) - cutoff)
        for subset in range(num_subsets):
            c0 = 1
            c1 = 1
            for i in range(len(q_primes) - cutoff):
                if (subset >> i) & 1:
                    c0 *= q_primes[i + cutoff]
                else:
                    c1 *= q_primes[i + cutoff]

            # Solve c0*x - c1*y = p
            # This is equivalent to c0*x ≡ p (mod c1)
            g, x0, _y0 = extended_gcd(c0, c1)
            if p % g != 0:
                continue
            x_sol = (x0 * (p // g)) % c1
            residue = (c0 * x_sol) % P
            residues.add(residue)

        # Search for smallest A
        k = 0
        while True:
            for residue in sorted(residues):
                A = k * P + residue
                if A > p:
                    # Check if Q divides A and A-p properly
                    if gcd(Q, A) * gcd(Q, A - p) == Q:
                        return A
            k += 1

    for p in primes:
        if p >= N:
            break
        ans += V(p)

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
