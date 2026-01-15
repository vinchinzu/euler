"""Project Euler Problem 639: Summing a multiplicative function.

Find Σ_{k=1}^K Σ_{i=1}^N f_k(i), where f_k is a multiplicative function
satisfying f_k(p) = p^k.

We can find the identity f_k(n) = n^k + Σ_d Π_{p_i|d} (p_i)^k (1 - (p_i)^k)
(n/d)^k, where d ranges over all powerful divisors of n.
"""

from __future__ import annotations

from math import isqrt

from sympy import isprime, primerange


def pow_mod(base: int, exp: int, mod: int) -> int:
    """Modular exponentiation."""
    result = 1
    base %= mod
    while exp > 0:
        if exp & 1:
            result = (result * base) % mod
        base = (base * base) % mod
        exp >>= 1
    return result


def sum_powers(n: int, k: int, mod: int) -> int:
    """Sum of i^k for i=1 to n."""
    result = 0
    for i in range(1, n + 1):
        result = (result + pow_mod(i, k, mod)) % mod
    return result


def solve() -> int:
    """Solve Problem 639."""
    N = 10**12
    K = 50
    M = 10**9 + 7
    L = isqrt(N)

    primes = list(primerange(2, L + 1))
    ans = 0

    nth_pows = [1] * (L + 1)
    sum_powers_arr = [0] * (L + 1)
    sum_coeffs = [0] * (L + 1)

    for k in range(1, K + 1):
        for i in range(1, L + 1):
            nth_pows[i] = (nth_pows[i] * i) % M
            sum_powers_arr[i] = (sum_powers_arr[i - 1] + nth_pows[i]) % M
            coeff = (
                nth_pows[i] * (1 - nth_pows[i]) % M if isprime(i) else 0
            )
            sum_coeffs[i] = (sum_coeffs[i - 1] + coeff) % M

        def helper(min_index: int, d: int, mult: int, prev_e: int) -> None:
            """Recursive helper."""
            nonlocal ans
            n = N // d
            if prev_e != 2:
                if n < len(sum_powers_arr):
                    ans = (ans + sum_powers_arr[int(n)] * mult) % M
                else:
                    ans = (ans + sum_powers(n, k, M) * mult) % M

            lim = int(n ** (1 / 3))
            for i in range(min_index, len(primes)):
                p = primes[i]
                if p * p > n / lim:
                    break
                q = n // (p * p)
                if q < len(sum_powers_arr):
                    term = (
                        sum_powers_arr[int(q)]
                        * mult
                        % M
                        * nth_pows[p]
                        % M
                        * (1 - nth_pows[p])
                        % M
                    )
                    ans = (ans + term) % M

            for q in range(1, lim):
                high = isqrt(n // q)
                low = max(isqrt(n // (q + 1)), primes[min_index] - 1)
                if high >= low:
                    term = (
                        sum_powers_arr[q]
                        * mult
                        % M
                        * (sum_coeffs[high] - sum_coeffs[low])
                        % M
                    )
                    ans = (ans + term) % M

            for index in range(min_index, len(primes)):
                p = primes[index]
                if d * p * p * p > N:
                    break
                new_d = d * p
                e = 2
                while new_d * p <= N:
                    new_d *= p
                    new_mult = (
                        mult * nth_pows[p] % M * (1 - nth_pows[p]) % M
                    )
                    helper(index + 1, new_d, new_mult, e)
                    e += 1

        helper(0, 1, 1, 0)

    return ans % M


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
