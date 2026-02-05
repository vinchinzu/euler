#!/usr/bin/env python3
"""
Project Euler 435 - Polynomials of Fibonacci numbers

F_n(x) = sum_{i=0}^n f_i * x^i where f_i is the i-th Fibonacci number.
Find sum_{x=0}^{100} F_n(x) mod 15! where n = 10^15.

Uses matrix exponentiation and CRT.
"""

def solve():
    N = 10**15
    K = 100
    M = 1307674368000  # 15!

    def matrix_mult(A, B, mod):
        """Multiply two 3x3 matrices mod m."""
        result = [[0, 0, 0], [0, 0, 0], [0, 0, 0]]
        for i in range(3):
            for j in range(3):
                for k in range(3):
                    result[i][j] = (result[i][j] + A[i][k] * B[k][j]) % mod
        return result

    def matrix_pow(A, n, mod):
        """Compute A^n mod m using binary exponentiation."""
        result = [[1, 0, 0], [0, 1, 0], [0, 0, 1]]  # Identity
        while n > 0:
            if n & 1:
                result = matrix_mult(result, A, mod)
            A = matrix_mult(A, A, mod)
            n >>= 1
        return result

    def F_n(x, m):
        """Compute F_N(x) mod m using matrix exponentiation."""
        if x == 0:
            return 0
        # Matrix relation:
        # [1, 0, 0]   [  1   ]   [   1    ]
        # [0, 0, 1] * [F_{n-1}] = [ F_n   ]
        # [x, x^2, x] [F_n   ]   [F_{n+1}]
        A = [
            [1, 0, 0],
            [0, 0, 1],
            [x % m, (x * x) % m, x % m]
        ]
        An = matrix_pow(A, N, m)
        # Starting vector: [1, 0, x] (for F_0=0, F_1=x)
        # Result is An * [1, 0, x]^T, middle element
        result = (An[1][0] + x * An[1][2]) % m
        return result

    def extended_gcd(a, b):
        if b == 0:
            return a, 1, 0
        g, x, y = extended_gcd(b, a % b)
        return g, y, x - (a // b) * y

    def crt(remainders, moduli):
        """Chinese Remainder Theorem."""
        result = 0
        M_total = 1
        for m in moduli:
            M_total *= m

        for r, m in zip(remainders, moduli):
            Mi = M_total // m
            _, inv, _ = extended_gcd(Mi, m)
            result += r * Mi * inv

        return result % M_total

    # Factor 15! into prime powers
    # 15! = 2^11 * 3^6 * 5^3 * 7^2 * 11 * 13
    prime_powers = [2**11, 3**6, 5**3, 7**2, 11, 13]
    # 2048, 729, 125, 49, 11, 13

    # Compute F_n(x) for each prime power and combine with CRT
    ans = 0
    for x in range(K + 1):
        remainders = [F_n(x, pp) for pp in prime_powers]
        fx = crt(remainders, prime_powers)
        ans = (ans + fx) % M

    return ans

if __name__ == "__main__":
    print(solve())
