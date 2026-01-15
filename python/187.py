"""Project Euler Problem 187: Semiprimes."""

from typing import Dict, List
import math

LIMIT = 100_000_000
SQRT_LIMIT = int(math.sqrt(LIMIT))
BLOCK_SIZE = 1_000_000


def sieve_primes(limit: int) -> List[int]:
    """Sieve primes up to limit."""
    sieve = [True] * (limit + 1)
    sieve[0] = sieve[1] = False
    for i in range(2, int(math.sqrt(limit)) + 1):
        if sieve[i]:
            for j in range(i * i, limit + 1, i):
                sieve[j] = False
    primes = []
    for n in range(limit + 1):
        if sieve[n]:
            primes.append(n)
    return primes


def main() -> int:
    """Main function."""
    primes = sieve_primes(SQRT_LIMIT)

    limit_map: Dict[int, List[int]] = {}
    for p in primes:
        limit_val = LIMIT // p
        if limit_val not in limit_map:
            limit_map[limit_val] = []
        limit_map[limit_val].append(p)

    pi_values: Dict[int, int] = {}
    sorted_limits = sorted(limit_map.keys())

    # Handle limits less than 2 directly
    while sorted_limits and sorted_limits[0] < 2:
        pi_values[sorted_limits.pop(0)] = 0

    pi_count = 0
    current_limit_index = 0

    low = 2
    while low <= LIMIT:
        high = min(low + BLOCK_SIZE - 1, LIMIT)
        block_size = high - low + 1
        is_prime = [True] * block_size

        for p in primes:
            if p * p > high:
                break
            start = max(p * p, ((low + p - 1) // p) * p)
            for multiple in range(start, high + 1, p):
                is_prime[multiple - low] = False

        if low == 1:
            is_prime[0] = False

        for i in range(block_size):
            n = low + i
            if is_prime[i]:
                pi_count += 1

            while (
                current_limit_index < len(sorted_limits)
                and sorted_limits[current_limit_index] <= n
            ):
                pi_values[sorted_limits[current_limit_index]] = pi_count
                current_limit_index += 1

        low = high + 1

    count = 0
    for idx, p in enumerate(primes):
        limit_val = LIMIT // p
        pi_limit = pi_values[limit_val]
        count += pi_limit - idx

    return count


if __name__ == "__main__":
    print(main())
