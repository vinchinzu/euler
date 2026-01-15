# Project Euler Problem 934
#
# PROBLEM DESCRIPTION:
# <p>We define the <i>unlucky prime</i> of a number $n$, denoted $u(n)$, as the smallest prime number $p$ such that the remainder of $n$ divided by $p$ (i.e. $n \bmod p$) is not a multiple of seven.<br>
# For example, $u(14) = 3$, $u(147) = 2$ and $u(1470) = 13$.</p>
# 
# <p>Let $U(N)$ be the sum $\sum_{n = 1}^N u(n)$.<br>
# You are given $U(1470) = 4293$.</p>
# 
# <p>Find $U(10^{17})$.</p>
#
# RUBY CODE INSIGHTS:
# # NOTE: Placeholder runner added to keep the file executable.
# # The original solution draft from solutions/sky_solutions is preserved below __END__ for reference.
# puts "Problem 934 placeholder implementation."
# __END__
# require 'prime'
# # Precompute primes up to 10^6 for efficient factorization
# MAX_PRIME_LIMIT = 1_000_000
# primes = Prime.each(MAX_PRIME_LIMIT).to_a
# def factorize(n, primes)
#   factors = {}
#   for p in primes
#     if p * p > n
#       break
#     end
#     if n % p == 0
#       count = 0
#       while n % p == 0
#         n /= p
#         count += 1
#       end
#       factors[p] = count
#     end
#   end
#   if n > 1
#     factors[n] = 1
#   end
#   factors
# end
# def find_u(n, primes)
#   if n % 7 == 0
#     # Check each prime starting from 2
#     for p in primes
#       if n % p != 0
#         return p
#       end
#     end
#     # If all primes up to sqrt(n) divide n, then n is a prime power
#     # But since n is multiple of 7, u(n)=2 (smallest prime not dividing)
#     return 2
#   end
#   # Find the smallest prime factor of n
#   for p in primes
#     if p * p > n
#       return 2  # n is prime itself
#     end
# ... (truncated Ruby code)
#
# PYTHON PORTING NOTES:
# - Port the Ruby logic above to Python
# - Implement solve() function to compute the answer
# - Handle edge cases and constraints from problem description
#
from __future__ import annotations
from typing import Optional

def solve() -> int:
    limit = 1000
    primes = []
    is_prime = [True] * (limit + 1)
    for p in range(2, limit + 1):
        if is_prime[p]:
            primes.append(p)
            for i in range(p*p, limit + 1, p):
                is_prime[i] = False

    N = 10**17

    total = 0
    # k=1: P1=2. Term 2 * S1.
    s1 = N
    total += 2 * s1

    # k=2: P2=3. Term (3-2) * S2.
    s2 = N // 2
    total += 1 * s2

    # k=3: P3=5. Term (5-3) * S3.
    s3 = N // 6
    total += 2 * s3

    # k=4: P4=7. Term (7-5) * S4.
    s4 = N // 30
    total += 2 * s4

    # k=5: P5=11. Term (11-7) * S5.
    s5 = N // 210
    total += 4 * s5

    M = N // 210

    residues = [0] # valid residues modulo mod_val
    mod_val = 1

    i = 4 # Index for P5=11
    while i + 1 < len(primes):
        p = primes[i]   # Current prime P_k
        next_p = primes[i+1] # P_{k+1}
        diff = next_p - p

        # Expand residues
        next_mod = mod_val * p

        limit_s = (p - 1) // 7
        inv30 = pow(30, -1, p)
        valid_digits = []
        for s in range(limit_s + 1):
            val = (s * inv30) % p
            valid_digits.append(val)

        inv_mod = pow(mod_val, -1, p)

        new_residues = []
        is_next_mod_large = (next_mod > M)

        for r in residues:
            for d in valid_digits:
                k = ((d - r) * inv_mod) % p
                x = r + k * mod_val
                if is_next_mod_large:
                    if x <= M:
                        new_residues.append(x)
                else:
                    new_residues.append(x)

        residues = new_residues
        mod_val = next_mod

        # Calculate S_{k+1}
        if mod_val > M:
            count = len(residues)
        else:
            full_cycles = M // mod_val
            rem = M % mod_val
            partial = 0
            for r in residues:
                if r <= rem:
                    partial += 1
            count = full_cycles * len(residues) + partial

        count -= 1 # Exclude 0

        if count == 0:
            break

        total += diff * count
        i += 1

    return total

if __name__ == "__main__":
    print(solve())
