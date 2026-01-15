"""Project Euler Problem 603: Concatenation of Consecutive Primes.

Find the sum of all contiguous integer substrings of the integer C, where C
consists of the first N primes concatenated together, and repeated K times.

The nth digit (0-indexed) of C appears as the unit digit of an integer substring
exactly n+1 times. It also appears as n+1 times as the tenth digit, the
hundredth digit, and so on up to the (10^r)th digit, where r is the number of
digits after it. This means the nth digit is multiplied by (n+1)(111...111) in
the final sum.

Let L be the length of a single copy of the concatenation of N primes. The
above logic gives a sum of sum_{n=0}^{KL-1} d_n (n+1) (10^(KL-n) - 1)/9.
However, since K is large, we need to group the terms n, L+n, ... (K-1)L+n
into an arithmetico-geometric series.
"""

from __future__ import annotations

from sympy import primerange


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


def triangular(n: int, mod: int) -> int:
    """Triangular number n(n+1)/2 modulo mod."""
    return n * (n + 1) // 2 % mod


def S(P: str, K: int, M: int) -> int:
    """Compute sum of contiguous substrings."""
    B = 10
    layered_num = 0
    num = 0
    layered_sum = 0
    sum_val = 0
    L = len(P)

    for n in range(L):
        d = int(P[n])
        layered_num = (layered_num + n * d) % M
        layered_sum = (layered_sum + n * d) % M
        num = (num + d) % M
        sum_val = (sum_val + d) % M
        layered_num = (layered_num * B) % M
        num = (num * B) % M

    piece = pow_mod(B, L, M)
    all_pow = pow_mod(piece, K, M) - 1
    inv_den = mod_inverse(piece - 1, M)
    res = (
        (
            all_pow * inv_den % M * layered_num
            + (all_pow + L * all_pow % M * inv_den - K % M * L) % M
            * inv_den
            % M
            * num
            - K % M * (layered_sum % M)
            - (L * triangular(K - 1, M) + K) % M * sum_val
        )
        % M
        * mod_inverse(B - 1, M)
    ) % M
    return res % M


def solve() -> int:
    """Solve Problem 603."""
    N = 10**6
    K = 10**12
    M = 10**9 + 7
    B = 10

    sb = []
    for p in primerange(2, N + 1000):
        if p > N:
            break
        sb.append(str(p))
    P = "".join(sb)

    return S(P, K, M)


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
