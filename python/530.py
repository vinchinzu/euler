"""Project Euler Problem 530: GCD Sum.

Let f(n) = Σ_{d|n} GCD(d, n/d). Find Σ_{n=1}^N f(n).

Let g = GCD(d, n/d), so d = g*x and n/d = g*y. Then we can rewrite the sum
as:

S = Σ_{g=1}^√N Σ_{(x,y)=1, x*y ≤ N/g²} g.

The relatively prime condition can be removed in the usual way, by summing
over h = GCD(x,y) and multiplying each inner term by µ(h). In the
innermost sum, the number of times g appears is equal to the number of
pairs (x,y) with product at most n = N/g², which is equal to the sum of
floor quotients Σ_i ⌊n/i⌋.
"""

from __future__ import annotations

from math import cbrt, isqrt
from typing import List


def pre_mobius(limit: int) -> List[int]:
    """Precompute Möbius function."""
    mu = [1] * (limit + 1)
    is_prime = [True] * (limit + 1)
    is_prime[0] = is_prime[1] = False

    for i in range(2, limit + 1):
        if is_prime[i]:
            for j in range(i, limit + 1, i):
                is_prime[j] = False
                if j % (i * i) == 0:
                    mu[j] = 0
                else:
                    mu[j] = -mu[j]
    return mu


def sq(n: int) -> int:
    """Square."""
    return n * n


def sum_floor_quotients(n: int) -> int:
    """Sum of floor quotients Σ_i ⌊n/i⌋."""
    result = 0
    i = 1
    while i <= n:
        q = n // i
        j = n // q
        result += q * (j - i + 1)
        i = j + 1
    return result


def sum_powers(n: int, k: int) -> int:
    """Sum of k-th powers from 1 to n."""
    if k == 1:
        return n * (n + 1) // 2
    # For k=1, return triangular number
    result = 0
    for i in range(1, n + 1):
        result += pow(i, k)
    return result


def solve() -> int:
    """Solve Problem 530."""
    N = 10**15
    L = int(cbrt(N))
    sqrt_n = isqrt(N)

    mobius = pre_mobius(sqrt_n)

    # Precompute floor quotient sums
    big = [0] * (L + 1)
    small = [0] * (L + 1)
    for i in range(1, L + 1):
        small[i] = sum_floor_quotients(i)
    for i in range(1, L + 1):
        big[i] = sum_floor_quotients(N // sq(i))

    ans = 0
    for h in range(1, sqrt_n + 1):
        if sq(h) > N:
            break
        n = N // sq(h)
        l = int(cbrt(n)) // 10 + 1
        
        for g in range(1, isqrt(n // l) + 1):
            if g * h < len(big):
                term = big[g * h]
            else:
                term = small[n // sq(g)]
            ans += mobius[h] * term * g
        
        for q in range(1, l):
            sqrt_n_q = isqrt(n // q)
            sqrt_n_q1 = isqrt(n // (q + 1)) if q + 1 <= n else 0
            ans += mobius[h] * small[q] * (
                sum_powers(sqrt_n_q, 1) - sum_powers(sqrt_n_q1, 1)
            )

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
