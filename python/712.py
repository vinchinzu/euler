"""Project Euler Problem 712: Exponent difference.

Let ν_p(n) be the largest r where p^r | n, and let
D(n, m) = Σ_{p prime} |ν_p(n) - ν_p(m)|.
Find S(N) = Σ_{1 ≤ n,m ≤ N} D(n, m).

We can write
S(N) = Σ_{n=1}^N Σ_{m=1}^N Σ_{p prime} |ν_p(n) - ν_p(m)|
     = Σ_{p prime} Σ_{n=1}^N Σ_{m=1}^N |ν_p(n) - ν_p(m)|.

For each prime p, ν = ν_p(n) can range from 0 to the highest power of p less
than N. There are ⌊N / p^ν⌋ - ⌊N / p^{ν+1}⌋ numbers with a given ν. So for
small p, we can iterate over all possible ν_p(n) and ν_p(m) to compute the sum.

For large p, ν_p(n) is at most 1. So the only contribution to the sum is
ν_p(n) = 1 and ν_p(m) = 0, or vice versa. These terms simplify to
Σ_{n=1}^N Σ_{m=1}^N (N - ⌊N/p⌋) (⌊N/p⌋). We can use the standard trick to sum
over all terms with the same ⌊N/p⌋.
"""

from __future__ import annotations

from math import isqrt

from sympy import primerange


class QuotientValues:
    """Helper for quotient-based prime counting."""

    def __init__(self, n: int, big: list[int], small: list[int]):
        """Initialize."""
        self.n = n
        self.big = big
        self.small = small
        self.L = len(small) - 1

    def div(self, x: int) -> int:
        """Get number of primes for quotient n/x."""
        if x <= self.L:
            return self.small[x]
        idx = self.n // x
        return self.big[idx] if idx < len(self.big) else 0


def sum_prime_powers(n: int, power: int, mod: int) -> QuotientValues:
    """Sum of prime powers (for power=0, counts primes)."""
    L = isqrt(n)
    big = [0] * (L + 1)
    small = [0] * (L + 1)

    # Compute π(x) for x ≤ L
    primes_list = list(primerange(2, L + 1))
    count = 0
    prime_idx = 0
    for i in range(1, L + 1):
        if prime_idx < len(primes_list) and primes_list[prime_idx] == i:
            count += 1
            prime_idx += 1
        small[i] = count

    # For large values, compute π(n/i) for i ≤ L
    # We need to count primes ≤ n/i
    for i in range(1, L + 1):
        q = n // i
        if q <= L:
            big[i] = small[q]
        else:
            # Count primes up to q using segmented approach
            count = 0
            for p in primes_list:
                if p > q:
                    break
                count += 1
            # Also count primes in [L+1, q] if q > L
            if q > L:
                # Use segmented sieve
                segment_size = max(L, 10000)
                low = L + 1
                while low <= q:
                    high = min(low + segment_size - 1, q)
                    is_prime_seg = [True] * (high - low + 1)
                    for p in primes_list:
                        if p * p > high:
                            break
                        start = max(p * p, ((low + p - 1) // p) * p)
                        for j in range(start, high + 1, p):
                            if j >= low:
                                is_prime_seg[j - low] = False
                    for idx in range(high - low + 1):
                        if is_prime_seg[idx]:
                            count += 1
                    low = high + 1
            big[i] = count

    return QuotientValues(n, big, small)


def pow_int(base: int, exp: int) -> int:
    """Compute base^exp (for small values)."""
    result = 1
    for _ in range(exp):
        result *= base
    return result


def solve() -> int:
    """Solve Problem 712."""
    N = 10**12
    M = 10**9 + 7
    L = isqrt(N)

    ans = 0

    # Process small primes
    primes_list = list(primerange(2, int(N // L) + 1))
    for p in primes_list:
        counts: list[int] = []
        e = 0
        while pow_int(p, e) <= N:
            p_e = pow_int(p, e)
            p_e_plus_1 = pow_int(p, e + 1)
            count = (N // p_e - N // p_e_plus_1) % M
            counts.append(count)
            e += 1

        # Sum over all pairs (vn, vm)
        for vn in range(len(counts)):
            for vm in range(len(counts)):
                diff = abs(vn - vm)
                contribution = diff * counts[vn] % M * counts[vm] % M
                ans = (ans + contribution) % M

    # Process large primes
    num_primes = sum_prime_powers(N, 0, M)
    for q in range(1, L):
        num_primes_q = num_primes.div(q)
        num_primes_q_plus_1 = num_primes.div(q + 1)
        num_primes_in_range = (num_primes_q - num_primes_q_plus_1) % M
        contribution = 2 * (N - q) % M * q % M * num_primes_in_range % M
        ans = (ans + contribution) % M

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
