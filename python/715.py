"""Project Euler Problem 715: Sextuplet Norms.

Let f(n) be the number of 6-tuples of non-negative integers (x1, x2, x3, x4,
x5, x6) where all x_i < n and gcd(x1²+x2²+x3²+x4²+x5²+x6², n²) = 1.
Find Σ_{k=1}^N f(k) / (k²ϕ(k)).

From https://oeis.org/A238534, the values of f are multiplicative with
f(p^e) = (p-1) p^{6e-4} (p³-{d|4}), where {d|4} is the Legendre symbol. This
means f(k) / (k²ϕ(k)) is multiplicative with f(p^e) = p^{3e-3} (p³-{d|4}).
So the sum can be written as Σ_{k=1}^N (Σ_{i=1}^⌊N/k⌋ i³) µ'(k), where µ'(k)
is 0 if k is even or a square, otherwise 1 if k has an even number of 1 (mod 4)
prime factors, otherwise -1; it is similar to the Mobius function.

To compute this sum more efficiently, we can use the standard trick of adding
all terms with the same ⌊N/k⌋ at once. This requires efficiently summing µ'(k).
To do this, we use the identity Σ_{k=1}^n {4|k} (Σ_{i=1}^⌊N/k⌋ µ'(i)) = 1 to
compute the Σ µ' in O(n^{2/3}) time, similar to the algorithm for computing
the Mertens function.
"""

from __future__ import annotations

from math import isqrt


def preff(n: int) -> list[int]:
    """Precompute smallest prime factor."""
    ff = list(range(n + 1))
    for i in range(2, int(n**0.5) + 1):
        if ff[i] == i:
            for j in range(i * i, n + 1, i):
                if ff[j] == j:
                    ff[j] = i
    return ff


def imod(a: int, m: int) -> int:
    """Integer modulo (handles negative)."""
    return ((a % m) + m) % m


def sum_powers(n: int, k: int, mod: int) -> int:
    """Sum of k-th powers from 1 to n mod mod."""
    if k == 0:
        return n % mod
    if k == 1:
        return (n * (n + 1) // 2) % mod
    if k == 2:
        return (n * (n + 1) * (2 * n + 1) // 6) % mod
    if k == 3:
        # Sum of cubes: (n(n+1)/2)^2
        s = (n * (n + 1) // 2) % mod
        return (s * s) % mod
    # For higher powers, compute directly
    result = 0
    for i in range(1, n + 1):
        result = (result + pow(i, k, mod)) % mod
    return result


def legendre_symbol(a: int, p: int) -> int:
    """Compute Legendre symbol (a/p)."""
    if a % p == 0:
        return 0
    result = pow(a, (p - 1) // 2, p)
    if result == p - 1:
        return -1
    return result


def solve() -> int:
    """Solve Problem 715."""
    N = 10**12
    M = 10**9 + 7
    L1 = int(N ** (1 / 3))
    L2 = int(N / L1)

    ff = preff(L2)

    # Compute µ'(k) = num1mod4PrimesParity[k]
    # µ'(k) = 0 if k is even or a square
    # Otherwise, (-1)^(number of 1 mod 4 prime factors)
    num1mod4_primes_parity = [0] * (L2 + 1)
    num1mod4_primes_parity[1] = 1
    for i in range(2, L2 + 1):
        d = ff[i]
        if d == 2:
            # Even numbers have µ' = 0
            num1mod4_primes_parity[i] = 0
        elif (i // d) % d == 0:
            # Square factor, so µ' = 0
            num1mod4_primes_parity[i] = 0
        else:
            # Check if d ≡ 1 (mod 4)
            if d % 4 == 1:
                num1mod4_primes_parity[i] = -num1mod4_primes_parity[i // d]
            else:
                num1mod4_primes_parity[i] = num1mod4_primes_parity[i // d]

    # Helper arrays for mod 4 square detection
    is_mod4_sq = [0, 1, 0, -1]  # is_mod4_sq[k % 4] = 1 if k is square mod 4
    cum_is_mod4_sq = [0, 1, 1, 0]  # Cumulative sum

    # Compute small[i] = Σ_{j=1}^i µ'(j)
    small = [0] * (L2 + 1)
    for i in range(1, L2 + 1):
        small[i] = (small[i - 1] + num1mod4_primes_parity[i]) % M

    # Compute big[i] = Σ_{j=1}^{⌊N/i⌋} µ'(j) for i ≤ L1
    big = [0] * (L1 + 1)
    for i in range(L1, 0, -1):
        big[i] = 1
        sqrt_n_i = isqrt(N // i)
        # Sum over k where k^2 divides something
        for k in range(2, sqrt_n_i):
            k_mod4 = k % 4
            if i * k < L1:
                big[i] = (big[i] - is_mod4_sq[k_mod4] * big[i * k]) % M
            else:
                idx = N // i // k
                if idx <= L2:
                    big[i] = (big[i] - is_mod4_sq[k_mod4] * small[idx]) % M
        # Sum over t
        for t in range(1, N // i // sqrt_n_i + 1):
            n_i_t = N // i // t
            n_i_t_plus_1 = N // i // (t + 1)
            n_i_t_mod4 = imod(n_i_t, 4)
            n_i_t_plus_1_mod4 = imod(n_i_t_plus_1, 4)
            diff = (
                cum_is_mod4_sq[n_i_t_mod4] - cum_is_mod4_sq[n_i_t_plus_1_mod4]
            ) % M
            big[i] = (big[i] - diff * small[t]) % M
        big[i] = imod(big[i], M)

    ans = 0
    # Sum over i ≤ L2
    for i in range(1, L2 + 1):
        sum_cubes = sum_powers(N // i, 3, M)
        ans = (ans + sum_cubes * num1mod4_primes_parity[i]) % M

    # Sum over t < L1
    for t in range(1, L1):
        sum_cubes = sum_powers(t, 3, M)
        diff = (big[t] - big[t + 1]) % M
        ans = (ans + sum_cubes * diff) % M

    return imod(ans, M)


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
