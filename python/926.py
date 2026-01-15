# Project Euler Problem 926
#
# PROBLEM DESCRIPTION:
# <p>
# A <strong>round number</strong> is a number that ends with one or more zeros in a given base.</p>
# 
# <p>
# Let us define the <dfn>roundness</dfn> of a number $n$ in base $b$ as the number of zeros at the end of the base $b$ representation of $n$.<br>
# For example, $20$ has roundness $2$ in base $2$, because the base $2$ representation of $20$ is $10100$, which ends with $2$ zeros.</p>
# 
# <p>
# Also define $R(n)$, the <dfn>total roundness</dfn> of a number $n$, as the sum of the roundness of $n$ in base $b$ for all $b &gt; 1$.<br>
# For example, $20$ has roundness $2$ in base $2$ and roundness $1$ in base $4$, $5$, $10$, $20$, hence we get $R(20)=6$.<br>
# You are also given $R(10!) = 312$.</p>
# 
# <p>
# Find $R(10\,000\,000!)$. Give your answer modulo $10^9 + 7$.</p>
#

"""
Project Euler Problem 926: Total Roundness

Solution Analysis:
The problem asks for R(N!) mod 10^9+7, where R(n) is the sum of roundness of n over all bases b > 1.
Roundness of n in base b is the largest k such that b^k divides n.

Let n = N!.
Let the prime factorization of b be product(p_i ^ a_i).
The roundness of n in base b is min(floor(v_{p_i}(n) / a_i)).

We need to compute sum_{b > 1} min_i floor(v_{p_i}(n) / a_i).
Instead of summing over b, we sum over possible values of roundness j >= 1.
R(n) = sum_{b>1} sum_{j=1}^{inf} [roundness(n, b) >= j]
     = sum_{j=1}^{inf} sum_{b>1} [roundness(n, b) >= j]

roundness(n, b) >= j means for all prime factors p of b with exponent a in b (so p^a || b),
floor(v_p(n) / a) >= j, which implies v_p(n) >= a * j, or a <= floor(v_p(n) / j).

This means that for a fixed j, a base b contributes to the count if for every prime factor p of b,
the exponent of p in b (call it a_p) satisfies 1 <= a_p <= floor(v_p(n) / j).
Note that if floor(v_p(n) / j) is 0, then p cannot be a factor of b.

So for a fixed j, the number of valid bases b is the number of integers > 1 formed by primes p where
the exponent of p is between 0 and floor(v_p(n) / j).
The number of such integers (including 1) is product_{p} (floor(v_p(n) / j) + 1).
So the number of valid bases b > 1 is (product_{p} (floor(v_p(n) / j) + 1)) - 1.

The maximum possible roundness is max(v_p(n)). The sum over j stops when for all p, floor(v_p(n)/j) = 0.

Algorithm:
1. Sieve primes up to N.
2. Calculate v_p(N!) for all primes using Legendre's formula.
3. Determine max_v = max(v_p(N!)).
4. Iterate j from 1 to max_v.
   For each j, compute P_j = product_{p} (floor(v_p(N!) / j) + 1) mod M.
   Add (P_j - 1) to total sum.
   Optimize the inner product loop by only iterating over primes where v_p(N!) >= j.
   Since v_p(N!) is roughly decreasing with p, we can iterate primes in order and break early.

Time Complexity: O(N log N)
Space Complexity: O(N)
"""

MOD = 1_000_000_007

def exponent_in_factorial(n: int, p: int) -> int:
    """
    Calculate the exponent of prime p in n! using Legendre's formula.
    """
    count = 0
    power = p
    while power <= n:
        count += n // power
        power *= p
    return count

def sieve_of_eratosthenes(n: int):
    """
    Generate all primes up to n using the Sieve of Eratosthenes.
    Returns a list of primes.
    """
    if n < 2:
        return [], [False] * (n + 1)

    is_prime = [True] * (n + 1)
    is_prime[0] = is_prime[1] = False

    for i in range(2, int(n**0.5) + 1):
        if is_prime[i]:
            for j in range(i * i, n + 1, i):
                is_prime[j] = False

    primes = [i for i in range(2, n + 1) if is_prime[i]]
    return primes, is_prime

def compute_total_roundness(n: int) -> int:
    """
    Compute R(n!) mod 10^9 + 7.
    """
    if n < 1:
        return 0

    if n >= 100_000:
        print(f"Generating primes up to {n}...")

    primes, _ = sieve_of_eratosthenes(n)
    num_primes = len(primes)

    if n >= 100_000:
        print(f"Found {num_primes} primes")

    # Calculate the exponent of each prime in n!
    # v_p(n!) is roughly decreasing as p increases, which is good for early breaking
    exponents = [exponent_in_factorial(n, p) for p in primes]

    max_v = max(exponents) if exponents else 0

    if n >= 100_000:
        print(f"Maximum exponent: {max_v}")

    total = 0

    # For each possible roundness value j
    for j in range(1, max_v + 1):
        product = 1
        all_factors_one = True

        # Iterate over primes. Since exponents roughly decrease, start from beginning.
        # Actually, larger primes have smaller exponents.
        # exponents[0] corresponds to prime=2 (largest exponent)
        # exponents[-1] corresponds to largest prime (smallest exponent)
        for i in range(num_primes):
            vp = exponents[i]
            if vp < j:
                # Since we iterate primes in increasing order (2, 3, ...),
                # and vp(n!) generally decreases, we might think we can break.
                # However, vp(n!) is not strictly monotonic (e.g. for n=6, v_2=4, v_3=2, v_5=1).
                # But it is roughly monotonic.
                # Actually, strictly, v_p(n!) <= n/(p-1). So as p increases, upper bound decreases.
                # Can we break?
                # If vp < j, it means floor(vp/j) = 0.
                # If we encounter a prime p such that vp < j, can there be a larger prime q > p with vq >= j?
                # v_p(n!) < j. v_q(n!) < n/(q-1) < n/(p-1).
                # It is possible that v_p is slightly smaller than v_q locally? No, v_p(n!) >= floor(n/p).
                # v_q(n!) < n/(q-1).
                # If p < q, floor(n/p) >= floor(n/q).
                # Generally v_p >= v_q for p < q is true for n!.
                # Proof: v_p(n!) = sum floor(n/p^k). v_q(n!) = sum floor(n/q^k).
                # Since p < q, p^k < q^k, so floor(n/p^k) >= floor(n/q^k).
                # So v_p(n!) >= v_q(n!) for p < q.
                # So the array `exponents` is sorted descending.
                # So if we find vp < j, all subsequent exponents will also be < j.
                break

            factor = 1 + (vp // j)
            product = (product * factor) % MOD
            if factor > 1:
                all_factors_one = False

        if not all_factors_one:
            contribution = (product - 1 + MOD) % MOD
            total = (total + contribution) % MOD
        else:
            # If for a given j, all factors are 1, then product is 1, contribution is 0.
            # And for any k > j, exponents will be even smaller relative to k.
            # So we can break the outer loop?
            # Yes, if max_v < j, we stop. We loop up to max_v, so this is handled.
            pass

    return total

def main():
    n = 10_000_000
    print(f"Computing R({n}!) mod {MOD}...")
    result = compute_total_roundness(n)
    print(result)

if __name__ == "__main__":
    main()
