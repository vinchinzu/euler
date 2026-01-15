"""
Project Euler Problem 903

Note: This solution contains a known incorrect guess assertion.
The algorithm computes a complex combinatorial sum involving factorials
and modular arithmetic.

Time Complexity: O(n * log p) where n = 10^6, p = 10^9 + 7
Space Complexity: O(n) for factorial array
"""

import sys


def main(n, p):
    """
    Compute the main algorithm for given n and modulus p.

    Args:
        n: Problem size parameter
        p: Modulus (10^9 + 7)

    Returns:
        Result modulo p

    Time Complexity: O(n * log p)
    """
    # Modular inverse of 2
    inv2 = pow(2, p - 2, p)

    # Precompute factorials
    f = [0] * (n + 1)
    f[0] = 1
    for k in range(1, n + 1):
        f[k] = f[k - 1] * k % p

    fact = f[n]
    total = fact * fact % p

    # Compute harmonic-like sum with modular inverses
    h = 0
    for k in range(1, n + 1):
        h = (h + pow(k, p - 2, p)) % p

    # Compute coefficients
    f_n2 = f[n - 2] if n >= 2 else 1
    c1 = fact * f_n2 * n * (h - 1 + p) % p
    c2 = fact * f_n2 * n * ((n + 1) * inv2 % p * (n - h + p) % p) % p

    # Compute sums
    sum_f = 0
    for k in range(n):
        sum_f = (sum_f + f[k]) % p
    sum_kf = (f[n] - 1 + p) % p

    sum1 = sum_f
    sum2 = (n * sum_f % p - sum_kf + p) % p
    sum3 = ((n - 1) * sum_f % p - sum_kf + p) % p

    # Final computation
    q = (total + c1 * sum2 % p + c2 * sum1 % p - total * sum1 % p - inv2 * total % p * sum3 % p) % p
    return q


if __name__ == "__main__":
    p = 10**9 + 7
    n = 10**6
    answer = main(n, p)
    print(answer)
