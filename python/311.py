"""Project Euler Problem 311: Biclinic Integral Quadrilaterals

Find count of quadrilaterals ABCD with 1 <= AB < BC < CD < AD,
where AB^2 + BC^2 + CD^2 + AD^2 <= 10^10.

Algorithm (from Java):
This is equivalent to counting ways to choose 3 distinct Pythagorean pairs summing to the same n <= N/4.
The formula involves r2(n) = number of ways to express n as sum of two squares.

For each n that can be expressed as sum of two squares, we choose 3 pairs from r2(n) ways.
The answer is sum of C(r2(n), 3) for all valid n.

r2(n) depends on prime factorization:
- Primes p ≡ 1 (mod 4) contribute (exponent + 1) to the product
- Primes p ≡ 3 (mod 4) must have even exponents
- Factor of 2 contributes differently based on parity
"""

def sieve_primes_mod(limit, residue, modulus):
    """Generate primes up to limit with p ≡ residue (mod modulus)."""
    is_prime = [True] * (limit + 1)
    is_prime[0] = is_prime[1] = False
    for i in range(2, int(limit**0.5) + 1):
        if is_prime[i]:
            for j in range(i*i, limit + 1, i):
                is_prime[j] = False

    primes = []
    for i in range(2, limit + 1):
        if is_prime[i] and i % modulus == residue:
            primes.append(i)
    return primes

def solve():
    N = 10**10
    L = N // 4
    L2 = L // (5 * 5 * 13)

    # Get primes congruent to 1 and 3 mod 4
    primes1mod4 = sieve_primes_mod(int(L // (5 * 5)), 1, 4)
    primes3mod4 = sieve_primes_mod(int(L2**0.5) + 1, 3, 4)

    # Count numbers formed by products of squares of primes ≡ 3 (mod 4)
    num3mod4_prods = [0] * (L2 + 1)

    def helper1(min_idx, n):
        """Generate all products of squares of primes ≡ 3 (mod 4)."""
        num3mod4_prods[n] += 1
        for idx in range(min_idx, len(primes3mod4)):
            p = primes3mod4[idx]
            if n * p * p > L2:
                return
            new_n = n
            while new_n * p * p <= L2:
                new_n *= p * p
                helper1(idx + 1, new_n)

    helper1(0, 1)

    # Compute cumulative counts
    C = [0] * (L2 + 1)
    for i in range(1, L2 + 1):
        C[i] = C[i-1] + num3mod4_prods[i]

    ans = 0

    def nCr(n, r):
        """Compute binomial coefficient."""
        if r < 0 or n < r:
            return 0
        if r == 0 or r == n:
            return 1
        if r > n - r:
            r = n - r
        result = 1
        for i in range(r):
            result = result * (n - i) // (i + 1)
        return result

    def helper2(min_idx, n, a0, b):
        """Generate numbers with primes ≡ 1 (mod 4) and count."""
        nonlocal ans

        # b is the product of (exponent_i + 1) for all primes p ≡ 1 (mod 4)
        # For r2(n), we need to account for the factor of 2 as well
        if b >= 5:
            r2_contrib = (b + (1 if a0 % 2 == 1 else 0)) // 2
            ways = nCr(r2_contrib, 3)
            if ways > 0:
                idx = int(L // n)
                if idx <= L2:
                    ans += C[idx] * ways

        # Calculate limit for next prime based on current product size
        if n > L:
            return

        limit = L / n
        if b == 1:
            limit = limit ** (1/3)
        elif b == 2:
            limit = limit ** (1/2)

        for idx in range(min_idx, len(primes1mod4)):
            p = primes1mod4[idx]
            if p > limit:
                return

            e = 1
            new_n = n
            while new_n * p <= L:
                new_n *= p
                helper2(idx + 1, new_n, a0, b * (e + 1))
                e += 1

    # Handle powers of 2
    a0 = 0
    prod = 1
    while prod <= L:
        helper2(0, prod, a0, 1)
        a0 += 1
        prod *= 2

    return ans

if __name__ == "__main__":
    print(solve())
