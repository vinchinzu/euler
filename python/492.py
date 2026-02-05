"""Project Euler Problem 492: Exploding sequence.

Define the sequence a_1 = 1 and a_{n+1} = 6(a_n)^2 + 10(a_n) + 3. Find sum of
a_N (mod p) for all primes X <= p <= X+Y.
"""

from math import isqrt


def sieve_primes(limit):
    """Sieve of Eratosthenes."""
    is_prime = [True] * (limit + 1)
    is_prime[0] = is_prime[1] = False
    for i in range(2, isqrt(limit) + 1):
        if is_prime[i]:
            for j in range(i * i, limit + 1, i):
                is_prime[j] = False
    return [i for i in range(limit + 1) if is_prime[i]]


def matrix_multiply_2x2(a, b, mod):
    """Multiply two 2x2 matrices mod p."""
    return [
        (a[0] * b[0] + a[1] * b[2]) % mod,
        (a[0] * b[1] + a[1] * b[3]) % mod,
        (a[2] * b[0] + a[3] * b[2]) % mod,
        (a[2] * b[1] + a[3] * b[3]) % mod,
    ]


def pow_2x2(matrix, exp, mod):
    """Matrix exponentiation."""
    result = [1, 0, 0, 1]
    base = [x % mod for x in matrix]
    while exp > 0:
        if exp & 1:
            result = matrix_multiply_2x2(result, base, mod)
        base = matrix_multiply_2x2(base, base, mod)
        exp >>= 1
    return result


def solve():
    """Solve Problem 492."""
    N = 10**15
    X = 10**9
    Y = 10**7

    # Sieve primes in range [X, X+Y] using segmented sieve
    # is_prime[i] = True means X + i is prime
    is_prime = [True] * (Y + 1)
    small_primes = sieve_primes(isqrt(X + Y))

    for p in small_primes:
        # Find first multiple of p that is >= X
        # i.e., find smallest i >= 0 such that (X + i) % p == 0
        start = (-X) % p  # = (p - X % p) % p
        for i in range(start, Y + 1, p):
            is_prime[i] = False

    A = [0, 1, -1, 11]
    ans = 0

    for i in range(Y + 1):
        if is_prime[i]:
            p = X + i
            # Identity matrix is pow_2x2(A, 0, p) = [1, 0, 0, 1]
            identity = [1, 0, 0, 1]
            # Check if period is p-1 or p+1
            if pow_2x2(A, p - 1, p) == identity:
                period = p - 1
            else:
                period = p + 1
            exp = pow(2, N - 1, period)
            mat = pow_2x2(A, exp, p)
            # x_n = 2 * mat[0] + 11 * mat[1]
            # a_n = (x_n - 5) / 6 mod p
            x_n = (2 * mat[0] + 11 * mat[1]) % p
            a_n = ((x_n - 5) % p * pow(6, p - 2, p)) % p
            ans += a_n

    return ans


if __name__ == "__main__":
    print(solve())
