"""Project Euler Problem 712: Exponent difference."""

from math import isqrt

def prime_count_table(N):
    """
    Lucy_Hedgehog algorithm to compute pi(n/k) for all distinct quotients.
    Returns a function get(v) that gives pi(v) for any v that is either <= sqrt(N) or N//k for some k.
    """
    r = isqrt(N)
    # V contains all distinct values of N // i
    V = [N // i for i in range(1, r + 1)]
    V += list(range(V[-1] - 1, 0, -1))

    # S[v] = count of integers 2..v (initially v - 1)
    S = {v: v - 1 for v in V}

    for p in range(2, r + 1):
        if S[p] > S[p - 1]:  # p is prime
            sp = S[p - 1]
            p2 = p * p
            for v in V:
                if v < p2:
                    break
                S[v] -= S[v // p] - sp

    return S


def solve():
    N = 10**12
    M = 10**9 + 7
    L = isqrt(N)

    # Sieve primes up to N/L ~ sqrt(N)
    limit = N // L + 1
    sieve = [True] * (limit + 1)
    sieve[0] = sieve[1] = False
    for i in range(2, isqrt(limit) + 1):
        if sieve[i]:
            for j in range(i * i, limit + 1, i):
                sieve[j] = False
    primes_list = [i for i in range(2, limit + 1) if sieve[i]]

    ans = 0

    # Process small primes (p <= N/L ~ sqrt(N))
    for p in primes_list:
        counts = []
        e = 0
        pe = 1
        while pe <= N:
            pe_next = pe * p
            cnt = (N // pe - N // pe_next) % M
            counts.append(cnt)
            pe = pe_next
            e += 1

        # Sum over all pairs (vn, vm)
        for vn in range(len(counts)):
            for vm in range(len(counts)):
                diff = abs(vn - vm)
                contribution = diff * counts[vn] % M * counts[vm] % M
                ans = (ans + contribution) % M

    # Process large primes using prime counting
    # For large p (p > sqrt(N)), v_p(n) <= 1, so only contributions are
    # when v_p(n) = 1, v_p(m) = 0 or vice versa.
    # Number of n with v_p(n) = 0: N - N//p
    # Number of n with v_p(n) = 1: N//p - N//p^2 ~ N//p for large p
    # Contribution: 2 * (N - N//p) * (N//p) per prime

    # Sum over primes p where N//p takes value q
    # Number of such primes = pi(N/q) - pi(N/(q+1))
    S = prime_count_table(N)

    # We need to exclude small primes we already processed
    num_small_primes = len(primes_list)

    for q in range(1, L):
        # pi(N/q) counts primes p with p <= N/q, i.e., N/p >= q, i.e., floor(N/p) >= q
        # pi(N/(q+1)) counts primes p with p <= N/(q+1)
        # Difference = primes with q <= floor(N/p) < q+1, i.e., floor(N/p) = q
        pi_q = S.get(N // q, 0) if N // q > 0 else 0
        pi_q_plus_1 = S.get(N // (q + 1), 0) if N // (q + 1) > 0 else 0

        # But we need primes > limit (already handled small primes)
        if N // q <= limit:
            pi_q = min(pi_q, num_small_primes)
        if N // (q + 1) <= limit:
            pi_q_plus_1 = min(pi_q_plus_1, num_small_primes)

        num_primes_in_range = (pi_q - pi_q_plus_1) % M

        # For each prime p with floor(N/p) = q:
        # v_p(n) = 0 for N - q numbers, v_p(n) >= 1 for q numbers
        # Actually for large p, v_p(n) = 1 for exactly q numbers
        # Contribution: 2 * (N - q) * q per prime
        contribution = 2 * (N - q) % M * q % M * num_primes_in_range % M
        ans = (ans + contribution) % M

    return ans


if __name__ == "__main__":
    print(solve())
