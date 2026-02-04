"""Project Euler Problem 773: Numbers Relatively Prime to Primes Ending in 7.

Let L_N = 10 times the product of the first N primes that end in 7. Find the
sum of all numbers less than L_N that end in 7 and are relatively prime to
these first N primes.

We use Inclusion Exclusion. For each subset of primes {p_i} ending in 7, we
need to add or subtract the series k*P(p_i) + (k+10)P(p_i) + (k+20)P(p_i) +
..., where k is the single digit so that each term ends in 7, and can be
easily computed by k = 7^{1-|p_i|} (mod 10).

If t = L_N / (10*P(p_i)), then the closed form of the series is
t(t-1)/2 P(p_i) + k*P(p_i)*t = (L_N/10)(5(t-1) + k). The sum of all 5(t-1)
terms for all t can be factored as 5*P(p_i - 1). For the k term, since k only
depends on the cardinality |p_i|, we can iterate over each cardinality i and
add/subtract the nCr(N,i) values of k.
"""

from __future__ import annotations

from sympy import primerange


def pow_mod(base, exp, mod):
    """Modular exponentiation."""
    result = 1
    base %= mod
    while exp > 0:
        if exp & 1:
            result = (result * base) % mod
        base = (base * base) % mod
        exp >>= 1
    return result


def mod_inv_prime(a, m):
    """Modular inverse for prime modulus m using Fermat's little theorem."""
    return pow_mod(a, m - 2, m)


def mod_inv_general(a, m):
    """Modular inverse using extended GCD (works for any coprime a, m)."""
    # Python 3.8+ supports pow(a, -1, m)
    return pow(a, -1, m)


def nCr(n, k, mod):
    """Binomial coefficient C(n, k) mod mod (mod must be prime)."""
    if k < 0 or k > n:
        return 0
    if k == 0 or k == n:
        return 1
    result = 1
    for i in range(min(k, n - k)):
        result = (result * (n - i) % mod * mod_inv_prime(i + 1, mod)) % mod
    return result


def parity(n):
    """Return 1 if n is even, -1 if odd."""
    return 1 if n % 2 == 0 else -1


def primes_mod(limit, remainder, mod):
    """Find primes up to limit that are = remainder (mod mod)."""
    return [p for p in primerange(2, limit + 1) if p % mod == remainder]


def solve():
    """Solve Problem 773."""
    N = 97
    K = 7
    M = 10**9 + 7
    B = 10

    # Find first N primes ending in 7
    primes = primes_mod(2 * B * 1000, K, B)
    primes = primes[:N]

    # ans = B/2 * prod(p_i - 1) mod M
    ans = B // 2
    for i in range(N):
        ans = (ans * (primes[i] - 1)) % M

    # Add inclusion-exclusion correction for k values
    for i in range(N + 1):
        # k_val = 7 * (7^i)^{-1} mod 10
        ki = pow_mod(K, i, B)  # 7^i mod 10
        ki_inv = mod_inv_general(ki, B)  # modular inverse mod 10
        k_val = (K * ki_inv) % B
        ans = (ans + parity(i) * k_val * nCr(N, i, M)) % M

    # Multiply by prod(primes)
    for i in range(N):
        ans = (ans * primes[i]) % M

    return ans % M


def main():
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
