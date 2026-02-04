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

from sympy import nextprime


def mod_inverse(a: int, m: int) -> int:
    """Modular inverse using extended Euclidean algorithm."""
    return pow(a, m - 2, m)


def S(P: str, K: int, M: int) -> int:
    """Compute sum of contiguous substrings.

    Matches the Java implementation exactly:
      layeredNum = sum_{n} d_n * n * B^(L-1-n)   (mod M)
      num        = sum_{n} d_n     * B^(L-1-n)   (mod M)
      layeredSum = sum_{n} d_n * n                (mod M)
      sum        = sum_{n} d_n                    (mod M)
    """
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
        layered_num = layered_num * B % M
        num = num * B % M

    piece = pow(B, L, M)
    all_pow = (pow(piece, K, M) - 1) % M
    inv_den = mod_inverse((piece - 1) % M, M)

    # tr(K-1, M) = (K-1)*K/2 mod M
    tr_km1 = (K - 1) % M * (K % M) % M * mod_inverse(2, M) % M

    # Match Java expression with explicit parenthesization:
    # res = (all * invDen % M * layeredNum
    #      + (all + L * all % M * invDen - K % M * L) % M * invDen % M * num
    #      - K % M * (layeredSum % M)
    #      - (L * tr(K-1,M) + K) % M * sum
    #      ) % M * modInv(B-1, M)
    #
    # Java operator precedence (left-to-right for * and %):
    # term1 = ((all * invDen) % M) * layeredNum
    # term2_inner = (all + ((L * all) % M) * invDen - ((K % M) * L)) % M
    # term2 = ((term2_inner * invDen) % M) * num
    # term3 = (K % M) * (layeredSum % M)
    # term4 = ((L * tr(K-1,M) + K) % M) * sum

    term1 = (all_pow * inv_den % M) * layered_num % M
    term2_inner = (all_pow + (L % M * all_pow % M) * inv_den % M - (K % M) * (L % M) % M) % M
    term2 = (term2_inner * inv_den % M) * num % M
    term3 = (K % M) * (layered_sum % M) % M
    term4 = ((L % M) * tr_km1 % M + K % M) % M * sum_val % M

    res = (term1 + term2 - term3 - term4) % M
    res = res * mod_inverse(B - 1, M) % M

    return res % M


def solve() -> int:
    """Solve Problem 603."""
    N = 10**6
    K = 10**12
    M = 10**9 + 7

    # Generate first N primes
    sb = []
    p = 2
    for _ in range(N):
        sb.append(str(p))
        p = nextprime(p)
    P = "".join(sb)

    return S(P, K, M)


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
