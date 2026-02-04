"""Project Euler Problem 500: Problem 500!!!

Find the smallest number with 2^500500 divisors, modulo 500500507.
Use a priority queue: start with all primes up to ~8M.
Each time we pick the smallest value from the queue, multiply it into the answer,
and push its square back (representing doubling that prime's exponent contribution).
"""

import heapq
from math import isqrt

def solve():
    N = 500_500
    M = 500_500_507

    # We need enough primes. The N-th prime is roughly N*ln(N) ~ 500500*13 ~ 7.7M
    limit = 7_800_000
    is_prime = bytearray(b'\x01') * (limit + 1)
    is_prime[0] = is_prime[1] = 0
    for i in range(2, isqrt(limit) + 1):
        if is_prime[i]:
            is_prime[i*i::i] = bytearray(len(is_prime[i*i::i]))
    primes = [i for i in range(2, limit + 1) if is_prime[i]]

    # Priority queue: each entry is a value we could multiply in.
    # Initially all primes. When we use value v, push v*v (next power-of-2 exponent).
    heap = list(primes[:N])  # We need at most N primes
    heapq.heapify(heap)

    ans = 1
    for _ in range(N):
        v = heapq.heappop(heap)
        ans = (ans * v) % M
        heapq.heappush(heap, v * v)

    return ans

if __name__ == "__main__":
    print(solve())
