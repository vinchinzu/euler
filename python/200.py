"""Project Euler Problem 200: Find the 200th prime-proof sqube containing substring "200"."""

from typing import Dict, List, Set

SMALL_PRIMES = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31]


def mul_mod(a: int, b: int, mod: int) -> int:
    """Modular multiplication."""
    return (a * b) % mod


def pow_mod(base: int, exp: int, mod: int) -> int:
    """Modular exponentiation."""
    result = 1
    b = base % mod
    e = exp
    while e > 0:
        if (e & 1) == 1:
            result = mul_mod(result, b, mod)
        b = mul_mod(b, b, mod)
        e >>= 1
    return result


def miller_rabin_witness(a: int, n: int, d: int, r: int) -> bool:
    """Miller-Rabin witness test."""
    x = pow_mod(a, d, n)
    if x == 1 or x == n - 1:
        return False
    for _ in range(r - 1):
        x = mul_mod(x, x, n)
        if x == n - 1:
            return False
    return True


def is_prime(n: int) -> bool:
    """Check if n is prime using Miller-Rabin."""
    if n < 2:
        return False
    for p in SMALL_PRIMES:
        if n == p:
            return True
        if n % p == 0:
            return False
    # write n-1 = d * 2^r with d odd
    d = n - 1
    r = 0
    while (d & 1) == 0:
        d >>= 1
        r += 1
    # Deterministic bases for 64-bit integers
    bases = [2, 3, 5, 7, 11, 13, 17]
    for a in bases:
        if a % n == 0:
            continue
        if miller_rabin_witness(a, n, d, r):
            return False
    return True


def prime_proof(n: int) -> bool:
    """Check if n is prime-proof."""
    s = str(n)
    length = len(s)
    for i in range(length):
        orig = s[i]
        for ch in "0123456789":
            if ch == orig:
                continue
            if i == 0 and ch == "0":  # avoid leading zeros
                continue
            # Skip trivial non-primes quickly
            if ch == "0" and i == length - 1:  # ending with 0 => composite
                continue
            if ch == "5" and i == length - 1 and length > 1:  # ending with 5 => composite
                continue
            t = s[:i] + ch + s[i + 1 :]
            m = int(t)
            if m < 2:
                continue
            if is_prime(m):
                return False
    return True


def primes_up_to(limit: int) -> List[int]:
    """Sieve for generating primes up to limit."""
    if limit < 2:
        return []
    sieve = [True] * (limit + 1)
    sieve[0] = sieve[1] = False
    p = 2
    while p * p <= limit:
        if sieve[p]:
            start = p * p
            step = p
            while start <= limit:
                sieve[start] = False
                start += step
        p += 1
    arr = []
    for i, is_prime_val in enumerate(sieve):
        if is_prime_val:
            arr.append(i)
    return arr


def cbrt_floor(x: int) -> int:
    """Cube root floor."""
    a = int(x ** (1.0 / 3))
    while (a + 1) ** 3 <= x:
        a += 1
    while a ** 3 > x:
        a -= 1
    return a


def enumerate_squbes(limit: int, primes: List[int]) -> List[int]:
    """Iterate squbes up to limit."""
    results: List[int] = []
    for p in primes:
        p2 = p * p
        if p2 > limit:
            break
        q_max = cbrt_floor(limit // p2)
        if q_max < 2:
            break
        for q in primes:
            if q > q_max:
                break
            if q == p:
                continue
            v = p2 * q * q * q
            results.append(v)
    return results


def main() -> int:
    """Main function."""
    import math

    target_index = 200
    limit = 1_000_000_000  # start at 1e9; grow as needed

    known_candidates: Dict[int, bool] = {}
    cached_primes: List[int] = []
    cached_bound = 1

    while True:
        # ensure we have enough primes up to max(sqrt(limit), cbrt(limit))
        bound = max(int(math.sqrt(limit)), cbrt_floor(limit))
        if bound > cached_bound:
            cached_primes = primes_up_to(bound)
            cached_bound = bound

        squbes = enumerate_squbes(limit, cached_primes)
        # filter those containing "200"
        for v in squbes:
            if "200" not in str(v):
                continue
            if v not in known_candidates:
                known_candidates[v] = prime_proof(v)

        good = sorted([k for k, v in known_candidates.items() if v])
        if len(good) >= target_index:
            return good[target_index - 1]

        # not enough; grow limit and continue
        limit *= 2


if __name__ == "__main__":
    print(main())
