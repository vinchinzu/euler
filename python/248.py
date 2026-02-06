"""Project Euler Problem 248: Numbers for which Euler's totient equals 13!.

Find the Nth smallest number n such that ϕ(n) = K!.
"""

from math import isqrt


def sieve(limit):
    """Generate all primes up to limit."""
    if limit < 2:
        return []
    is_prime = [True] * (limit + 1)
    is_prime[0] = is_prime[1] = False
    for i in range(2, isqrt(limit) + 1):
        if is_prime[i]:
            for j in range(i * i, limit + 1, i):
                is_prime[j] = False
    return [i for i in range(limit + 1) if is_prime[i]]


def factorial(n):
    """Return n!."""
    result = 1
    for i in range(1, n + 1):
        result *= i
    return result


def all_divisors(n, primes):
    """Return all divisors of n."""
    divisors = [1]
    temp = n
    for p in primes:
        if temp % p == 0:
            size = len(divisors)
            power = 1
            while temp % p == 0:
                temp //= p
                power *= p
                for i in range(size):
                    divisors.append(divisors[i] * power)
    if temp > 1:
        size = len(divisors)
        for i in range(size):
            divisors.append(divisors[i] * temp)
    return divisors


def is_probable_prime(n):
    """Check if n is probably prime."""
    if n < 2:
        return False
    if n == 2:
        return True
    if n % 2 == 0:
        return False
    for i in range(3, isqrt(n) + 1, 2):
        if n % i == 0:
            return False
    return True


def solve():
    """Solve Problem 248."""
    N = 150000
    K = 13
    Kf = factorial(K)

    # Each entry is (prod, phi) tuple
    nums = [(1, 1)]

    primes_list = sieve(K)
    for d in all_divisors(Kf, primes_list):
        p = Kf // d + 1
        if is_probable_prime(p):
            new_nums = []
            for prod, phi in nums:
                pe = 1
                while Kf % (phi * pe * (p - 1)) == 0:
                    new_nums.append((prod * pe * p, phi * pe * (p - 1)))
                    pe *= p
            nums.extend(new_nums)

    # Filter and sort
    valid_nums = sorted(prod for prod, phi in nums if phi == Kf)
    return valid_nums[N - 1]


if __name__ == "__main__":
    print(solve())
