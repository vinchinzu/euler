"""Project Euler Problem 682: 5-Smooth Pairs.

Find the number of pairs (p, q) of Hamming numbers (5-smooth numbers) such
that p and q have the same number of prime factors counted with multiplicity,
and the sum of the prime factors of a and b together, counted with multiplicity,
is N.

Let the number of factors of 2, 3, 5 of p be p2, p3, p5 respectively, and
define q2, q3, q5 similarly for q. The number of pairs corresponds to the
number of lattice points in a region defined by planar restrictions, i.e. a
polyhedron. Therefore, the solution obeys a linear recurrence, and we can
extrapolate the final result from small values.
"""

from __future__ import annotations


def berlekamp_massey(s: list[int], mod: int) -> list[int]:
    """Find shortest linear recurrence using Berlekamp-Massey algorithm."""
    n = len(s)
    C = [1]
    B = [1]
    L = 0
    m = 1
    b = 1
    for i in range(n):
        d = s[i]
        for j in range(1, L + 1):
            d = (d + C[j] * s[i - j]) % mod
        if d == 0:
            m += 1
        elif 2 * L <= i:
            T = list(C)
            coeff = d * pow(b, -1, mod) % mod
            while len(C) < len(B) + m:
                C.append(0)
            for j in range(len(B)):
                C[j + m] = (C[j + m] - coeff * B[j]) % mod
            L = i + 1 - L
            B = T
            b = d
            m = 1
        else:
            coeff = d * pow(b, -1, mod) % mod
            while len(C) < len(B) + m:
                C.append(0)
            for j in range(len(B)):
                C[j + m] = (C[j + m] - coeff * B[j]) % mod
            m += 1
    # Return coefficients c1, c2, ... cL such that s[n] = sum c_i * s[n-i]
    return [(-c) % mod for c in C[1:L+1]]


def poly_mult_mod(a: list[int], b: list[int], rec: list[int], mod: int) -> list[int]:
    """Multiply two polynomials modulo the characteristic polynomial of recurrence."""
    L = len(rec)
    result = [0] * (2 * L)
    for i in range(len(a)):
        if a[i] == 0:
            continue
        for j in range(len(b)):
            result[i + j] = (result[i + j] + a[i] * b[j]) % mod
    # Reduce modulo characteristic polynomial: x^L = c1*x^(L-1) + ... + cL
    for i in range(2 * L - 1, L - 1, -1):
        if result[i] == 0:
            continue
        c = result[i]
        result[i] = 0
        for j in range(L):
            result[i - L + j] = (result[i - L + j] + c * rec[L - 1 - j]) % mod
    return result[:L]


def linear_recurrence_nth(rec: list[int], init: list[int], n: int, mod: int) -> int:
    """Compute n-th term of linear recurrence using matrix exponentiation via polynomial.

    rec: coefficients c1..cL where s[n] = c1*s[n-1] + ... + cL*s[n-L]
    init: first L values s[0]..s[L-1]
    """
    L = len(rec)
    if n < L:
        return init[n] % mod

    # We want to compute x^n mod characteristic polynomial
    # Then s[n] = sum of coefficients * init values
    # Using binary exponentiation on polynomials

    # Start with x^1
    base = [0] * L
    if L > 1:
        base[1] = 1
    else:
        base[0] = rec[0]

    result = [0] * L
    result[0] = 1  # x^0 = 1

    exp = n
    while exp > 0:
        if exp & 1:
            result = poly_mult_mod(result, base, rec, mod)
        base = poly_mult_mod(base, base, rec, mod)
        exp >>= 1

    # s[n] = sum result[i] * init[i]
    ans = 0
    for i in range(L):
        ans = (ans + result[i] * init[i]) % mod
    return ans


def solve() -> int:
    """Solve Problem 682."""
    N = 10**7
    M = 10**9 + 7

    def f(n: int) -> int:
        """Count pairs for given n."""
        count = 0
        for p2 in range(n // 2 + 1):
            if 2 * p2 >= n:
                break
            for p3 in range((n - 2 * p2) // 3 + 1):
                if 2 * p2 + 3 * p3 >= n:
                    break
                for p5 in range((n - 2 * p2 - 3 * p3) // 5 + 1):
                    if 2 * p2 + 3 * p3 + 5 * p5 >= n:
                        break
                    for q2 in range((n - 2 * p2 - 3 * p3 - 5 * p5) // 2 + 1):
                        if 2 * p2 + 3 * p3 + 5 * p5 + 2 * q2 > n:
                            break
                        for q3 in range((n - 2 * p2 - 3 * p3 - 5 * p5 - 2 * q2) // 3 + 1):
                            if 2 * p2 + 3 * p3 + 5 * p5 + 2 * q2 + 3 * q3 > n:
                                break
                            q5 = p2 + p3 + p5 - q2 - q3
                            if q5 >= 0 and 2 * p2 + 3 * p3 + 5 * p5 + 2 * q2 + 3 * q3 + 5 * q5 == n:
                                count += 1
        return count

    # Compute enough values to find and verify the recurrence
    num_values = 80
    vals = [f(i) % M for i in range(num_values)]

    # Find recurrence
    rec = berlekamp_massey(vals, M)
    L = len(rec)

    # Verify recurrence on remaining values
    for i in range(L, num_values):
        expected = 0
        for j in range(L):
            expected = (expected + rec[j] * vals[i - 1 - j]) % M
        assert expected == vals[i], f"Recurrence mismatch at {i}: {expected} vs {vals[i]}"

    # Compute the N-th term using polynomial exponentiation
    ans = linear_recurrence_nth(rec, vals[:L], N, M)
    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
