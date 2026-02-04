"""Project Euler Problem 123.

Let p_n be the nth prime: 2, 3, 5, 7, 11, ..., and let r be the remainder when
(p_n - 1)^n + (p_n + 1)^n is divided by p_n^2.

For n even, remainder is 2.
For n odd, remainder is 2*n*p_n mod p_n^2.
For large enough n, 2*n < p_n, so remainder is simply 2*n*p_n.

We want the smallest n such that remainder > 10^10.
"""

def primes_sieve(limit):
    """Return a list of primes up to limit using Sieve of Eratosthenes."""
    is_prime = bytearray(b'\x01') * (limit + 1)
    is_prime[0] = is_prime[1] = 0
    for i in range(2, int(limit**0.5) + 1):
        if is_prime[i]:
            is_prime[i*i::i] = bytearray(len(is_prime[i*i::i]))
    return [i for i in range(2, limit + 1) if is_prime[i]]

def solve():
    target = 10**10
    # We need roughly the first 25000 primes. The 25000th prime is around
    # 287000, so a sieve up to 300000 is more than enough.
    primes = primes_sieve(300000)

    for n in range(1, len(primes) + 1, 2):  # Only check odd n
        p_n = primes[n - 1]  # 1-indexed: p_1 = 2, p_2 = 3, ...
        remainder = 2 * n * p_n
        if remainder > target:
            return n

    return None

if __name__ == "__main__":
    print(solve())
