"""Project Euler Problem 672: One More One.

Define a process where we start with n and repeatedly divide by 7 if the current
number is divisible by 7, and add one otherwise, ending at 1. Let g(n) be the
number of times when we add one, S(N) = sum_{n=1}^N g(n), and H(K) = S((7^K - 1) / 11).
Find H(N).

We compute H(k) for small k values, find the linear recurrence using
Berlekamp-Massey, and then use polynomial exponentiation to evaluate at N.
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
    for i in range(2 * L - 1, L - 1, -1):
        if result[i] == 0:
            continue
        c = result[i]
        result[i] = 0
        for j in range(L):
            result[i - L + j] = (result[i - L + j] + c * rec[L - 1 - j]) % mod
    return result[:L]


def linear_recurrence_nth(rec: list[int], init: list[int], n: int, mod: int) -> int:
    """Compute n-th term of linear recurrence using polynomial exponentiation."""
    L = len(rec)
    if n < L:
        return init[n] % mod

    base = [0] * L
    if L > 1:
        base[1] = 1
    else:
        base[0] = rec[0]

    result = [0] * L
    result[0] = 1

    exp = n
    while exp > 0:
        if exp & 1:
            result = poly_mult_mod(result, base, rec, mod)
        base = poly_mult_mod(base, base, rec, mod)
        exp >>= 1

    ans = 0
    for i in range(L):
        ans = (ans + result[i] * init[i]) % mod
    return ans


def solve() -> int:
    """Solve Problem 672."""
    N = 10**9
    K = 11
    M = 1117117717
    B = 7

    def tr(n: int) -> int:
        return n * (n + 1) // 2

    def H(k: int) -> int:
        """Compute H(k) = S((7^k - 1) / 11)."""
        n = B - 1
        n_div_b = 0
        g = 0
        H_val = 0
        for i in range(1, k):
            n = n * B + (B - 1)
            digit = n // K
            H_val = (
                B * H_val
                + n_div_b * tr(B - 1)
                + digit * g
                + tr(B - 2)
                - tr(B - 1 - digit)
            ) % M
            n -= digit * K
            n_div_b = (B * n_div_b + digit) % M
            g += B - 1 - digit
        return H_val

    # Compute enough values (H is 1-indexed: H(1), H(2), ...)
    num_values = 60
    vals = [H(k) for k in range(1, num_values + 1)]

    # Find recurrence
    rec = berlekamp_massey(vals, M)
    L = len(rec)

    # Compute H(N) using the recurrence (0-indexed in vals, so H(N) = vals[N-1])
    ans = linear_recurrence_nth(rec, vals[:L], N - 1, M)
    return ans % M


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
