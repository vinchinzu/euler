"""Project Euler Problem 641: A Long Row of Dice."""

def prime_count(n):
    """Count primes up to n using Lucy_Hedgehog method."""
    if n < 2:
        return 0
    if n == 2:
        return 1

    # Lucy_Hedgehog algorithm
    r = int(n ** 0.5)
    V = [n // i for i in range(1, r + 1)]
    V += list(range(V[-1] - 1, 0, -1))
    S = {v: v - 1 for v in V}

    for p in range(2, r + 1):
        if S[p] > S[p - 1]:  # p is prime
            sp = S[p - 1]
            p2 = p * p
            for v in V:
                if v < p2:
                    break
                S[v] -= S[v // p] - sp

    return S[n]


def nthrt(n, r):
    """Integer r-th root of n."""
    if r == 1:
        return n
    if n <= 0:
        return 0
    # Initial estimate using floating point
    x = int(round(n ** (1.0 / r)))
    # Refine to ensure correctness
    while (x + 1) ** r <= n:
        x += 1
    while x > 0 and x ** r > n:
        x -= 1
    return x


def solve():
    N = 10**18
    K = 6
    L = int(N ** 0.4)

    # Sieve primes up to L
    sieve = [True] * (L + 1)
    sieve[0] = sieve[1] = False
    for i in range(2, int(L**0.5) + 1):
        if sieve[i]:
            for j in range(i*i, L + 1, i):
                sieve[j] = False

    primes = [i for i in range(2, L + 1) if sieve[i]]

    # Prime counts up to each index
    prime_counts = [0] * (L + 1)
    cnt = 0
    for i in range(L + 1):
        if i >= 2 and sieve[i]:
            cnt += 1
        prime_counts[i] = cnt

    ans = [1]  # Start with 1 (the number 1 itself)

    def helper(min_index, n, num_divisors):
        # For each valid exponent, count possibilities for largest prime
        # If numDivisors % K == 1, we need exponent e where (e+1) keeps it at 1 mod K
        # Otherwise we need (e+1) to bring it to 1 mod K
        e = K if num_divisors % K == 1 else K - 2
        while True:
            bound = nthrt(N // n, e // 2)
            if min_index >= len(primes) or bound < primes[min_index]:
                break
            if bound > L:
                cnt = prime_count(bound) - min_index
            else:
                cnt = prime_counts[bound] - min_index
            ans[0] += cnt
            e += K

        # Recurse for smaller primes
        for index in range(min_index, len(primes)):
            p = primes[index]
            # Check if we can add p^(K-2)
            if n * pow(p, K - 2) > N:
                break
            for start_e in [K - 2, K]:
                e = start_e
                new_n = n * pow(p, e // 2)
                while new_n < N:
                    helper(index + 1, new_n, num_divisors * (e + 1))
                    e += K
                    new_n *= pow(p, K // 2)

    helper(0, 1, 1)
    return ans[0]


if __name__ == "__main__":
    print(solve())
