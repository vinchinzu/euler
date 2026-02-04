"""Project Euler Problem 233: Lattice points on a circle.

Find the sum of all n <= 10^11 such that the circle through (0,0), (n,0),
(0,n), (n,n) has exactly 420 lattice points.

The number of lattice points on this circle equals r_2(2n^2) = 4 * prod(2*a_i+1)
where a_i are the exponents of primes ≡ 1 mod 4 in the factorization of n. We need
this product to equal 105. The valid exponent tuples (for primes ≡ 1 mod 4) are:
  (1,2,3), (2,10), (3,7)
(The tuples (1,17) and (52,) always produce cores exceeding 10^11.)

For each exponent pattern, enumerate all valid cores (products of primes ≡ 1 mod 4
raised to their assigned exponents), then for each core P, the valid n are P*m where
m has no prime factor ≡ 1 mod 4 and P*m <= N.
"""

from math import isqrt
import bisect


def solve():
    N = 10**11

    # Sieve primes
    def sieve_primes(limit):
        is_prime = bytearray(b'\x01') * (limit + 1)
        is_prime[0] = is_prime[1] = 0
        for i in range(2, isqrt(limit) + 1):
            if is_prime[i]:
                is_prime[i*i::i] = bytearray(len(is_prime[i*i::i]))
        return [p for p in range(2, limit + 1) if is_prime[p]]

    # Maximum multiplier: N / min_core for (1,2,3) = N / (5^3 * 13^2 * 17)
    MAX_MULT = N // (5**3 * 13**2 * 17) + 10

    # Build prefix sum of valid multipliers (numbers with no prime factor ≡ 1 mod 4)
    primes_small = sieve_primes(MAX_MULT)
    is_valid = bytearray(b'\x01') * (MAX_MULT + 1)
    is_valid[0] = 0
    for p in primes_small:
        if p % 4 == 1:
            for j in range(p, MAX_MULT + 1, p):
                is_valid[j] = 0

    valid_sum = [0] * (MAX_MULT + 1)
    v = 0
    for m in range(MAX_MULT + 1):
        v += m * is_valid[m]
        valid_sum[m] = v

    def S(L):
        if L <= 0:
            return 0
        return valid_sum[min(L, MAX_MULT)]

    # Primes ≡ 1 mod 4 up to limit needed for cores
    PLIMIT = 5_000_000
    P1 = [p for p in sieve_primes(PLIMIT) if p % 4 == 1]

    total = 0

    # Pattern (1, 2, 3): three distinct primes, all 6 permutations of exponent assignment
    for (ep, eq, er) in [(1,2,3),(1,3,2),(2,1,3),(2,3,1),(3,1,2),(3,2,1)]:
        for ip, p in enumerate(P1):
            pp = p ** ep
            if pp >= N:
                break
            remain_p = N // pp
            for iq in range(ip + 1, len(P1)):
                q = P1[iq]
                qq = q ** eq
                if qq > remain_p:
                    break
                remain_pq = remain_p // qq
                if er == 1:
                    max_r = remain_pq
                elif er == 2:
                    max_r = isqrt(remain_pq)
                else:
                    max_r = int(round(remain_pq ** (1.0/er)))
                    while (max_r + 1) ** er <= remain_pq:
                        max_r += 1
                    while max_r ** er > remain_pq:
                        max_r -= 1
                if max_r <= q:
                    continue
                end_ir = bisect.bisect_right(P1, max_r)
                for ir in range(iq + 1, end_ir):
                    r = P1[ir]
                    core = pp * qq * r ** er
                    L = N // core
                    if L <= 0:
                        break
                    total += core * S(L)

    # Pattern (2, 10): two distinct primes, both permutations
    for (ep, eq) in [(2, 10), (10, 2)]:
        for ip, p in enumerate(P1):
            pp = p ** ep
            if pp >= N:
                break
            remain = N // pp
            for iq in range(ip + 1, len(P1)):
                q = P1[iq]
                qq = q ** eq
                if qq > remain:
                    break
                core = pp * qq
                total += core * S(N // core)

    # Pattern (3, 7): two distinct primes, both permutations
    for (ep, eq) in [(3, 7), (7, 3)]:
        for ip, p in enumerate(P1):
            pp = p ** ep
            if pp >= N:
                break
            remain = N // pp
            for iq in range(ip + 1, len(P1)):
                q = P1[iq]
                qq = q ** eq
                if qq > remain:
                    break
                core = pp * qq
                total += core * S(N // core)

    return total


if __name__ == "__main__":
    print(solve())
