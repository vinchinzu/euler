"""Project Euler Problem 792: 2-Valuation of Binomial Sums.

Let v2(n) be the 2-valuation of n, let S(n) = Σ_{k=1}^n (-2)^k nCr(2k,k),
and let u(n) = v2(3S(n)+4). Find Σ_{n=1}^N u(n³).

We can find by induction that 3S(n)+4 = 2^{n+2} * Σ_{k=0}^n (-2)^k
nCr(2n+1,n+k+1): the latter term is https://oeis.org/A026641, and has a
relatively small number of factors of 2. So we can compute the latter term mod
a power of 2 and find the number of factors of 2 it has.

To compute nCr(2n+1,n+k+1) mod a power of 2, we precompute the double
factorials n!!, and express n! as a product of these double factorials,
multiplied by a power of 2.
"""

from __future__ import annotations

from typing import List


def mod_inv(a: int, m: int) -> int:
    """Modular inverse."""
    return pow(a, m - 2, m)


def num_factors_in_factorial(n: int, p: int) -> int:
    """Count number of factors of p in n!."""
    count = 0
    power = p
    while power <= n:
        count += n // power
        power *= p
    return count


def cb(n: int) -> int:
    """Cube of n."""
    return n * n * n


def solve() -> int:
    """Solve Problem 792."""
    N = 10000
    L = 1 << 27

    # Precompute double factorials
    double_factorials = [1] * L
    for n in range(1, L):
        if n % 2 == 0:
            double_factorials[n] = double_factorials[n - 1]
        else:
            double_factorials[n] = (double_factorials[n - 1] * n) % L

    def nCr_mod_L(a: int, b: int) -> int:
        """Compute nCr(a, b) mod L."""
        nCr = 1
        n = a
        while n > 0:
            nCr = (nCr * double_factorials[n % L]) % L
            n //= 2
        n = b
        while n > 0:
            nCr = (nCr * mod_inv(double_factorials[n % L], L)) % L
            n //= 2
        n = a - b
        while n > 0:
            nCr = (nCr * mod_inv(double_factorials[n % L], L)) % L
            n //= 2

        # Add power of 2
        exp_2 = (
            num_factors_in_factorial(a, 2)
            - num_factors_in_factorial(b, 2)
            - num_factors_in_factorial(a - b, 2)
        )
        nCr = (nCr << exp_2) % L
        return nCr

    def u(n: int) -> int:
        """Compute u(n) = v2(3S(n)+4)."""
        res = 0
        k = 0
        while True:
            term = nCr_mod_L(2 * n + 1, n + k + 1)
            if k % 2 == 0:
                res = (res + term) % L
            else:
                res = (res - term) % L

            if res % 2 != 0:
                return n + 2 + k

            res //= 2
            k += 1

    ans = 0
    for n in range(1, N + 1):
        ans += u(cb(n))

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
