"""Project Euler Problem 362 - Squarefree factors.

Fsf(n) is the number of ways to factor n into square-free factors > 1.
Find sum of Fsf(k) for k=1 to N=10^10.

Algorithm (from Java reference):
- Precompute numSquareFree for all quotient values N//k
- Use iterative stack-based search over products of square-free factors
- For each partial product, add count of valid last factors using precomputed values
"""

import numpy as np


def solve():
    N = 10**10
    L = int(N**0.5)

    # Compute Mobius function up to sqrt(N) for square-free counting
    mu = np.ones(L + 1, dtype=np.int8)
    mu[0] = 0

    # Sieve for smallest prime factor
    is_prime = np.ones(L + 1, dtype=np.bool_)
    is_prime[0] = is_prime[1] = False

    for i in range(2, int(L**0.5) + 1):
        if is_prime[i]:
            for j in range(i * i, L + 1, i):
                is_prime[j] = False
            # Mark multiples of i^2 as having mu = 0
            sq = i * i
            for j in range(sq, L + 1, sq):
                mu[j] = 0

    # Compute mu values for remaining numbers
    for i in range(2, L + 1):
        if mu[i] == 0:
            continue
        if is_prime[i]:
            for j in range(i, L + 1, i):
                mu[j] = -mu[j]

    # Build list of square-free numbers up to L
    is_square_free = np.ones(L + 1, dtype=np.bool_)
    is_square_free[0] = is_square_free[1] = False

    for p in range(2, int(L**0.5) + 1):
        sq = p * p
        for j in range(sq, L + 1, sq):
            is_square_free[j] = False

    square_frees = np.array([i for i in range(2, L + 1) if is_square_free[i]], dtype=np.int64)
    num_sf = len(square_frees)

    # Precompute cumulative square-free count up to L
    sf_cumul = np.zeros(L + 2, dtype=np.int64)
    for i in range(2, L + 1):
        sf_cumul[i] = sf_cumul[i - 1] + (1 if is_square_free[i] else 0)

    # Count square-free numbers up to x using Mobius function
    # Q(x) = sum_{d=1}^{sqrt(x)} mu(d) * floor(x / d^2)
    def count_square_free(x):
        if x < 1:
            return 0
        total = 0
        d = 1
        while d * d <= x:
            if mu[d] != 0:
                total += int(mu[d]) * (x // (d * d))
            d += 1
        return total

    def num_square_free_up_to(x):
        """Count square-free numbers in [2, x]."""
        if x < 2:
            return 0
        if x <= L:
            return sf_cumul[x]
        return count_square_free(x) - 1  # exclude 1

    # Precompute all quotient values for N
    # There are O(2*sqrt(N)) distinct values of N // k
    quotient_values = {}

    # For k from 1 to sqrt(N), N//k gives large values
    for k in range(1, L + 1):
        q = N // k
        if q not in quotient_values:
            quotient_values[q] = num_square_free_up_to(q)

    # Also need values up to L for small lookups
    small_lookup = sf_cumul.copy()  # Already have [0..L]

    def get_num_sf(x):
        """Get number of square-free numbers in [2, x]."""
        if x < 2:
            return 0
        if x <= L:
            return small_lookup[x]
        if x in quotient_values:
            return quotient_values[x]
        # Should not reach here if we precomputed correctly
        result = num_square_free_up_to(x)
        quotient_values[x] = result
        return result

    # Binary search helper: find largest index where square_frees[index] <= limit
    def find_last_index(start_index, limit):
        if start_index >= num_sf:
            return start_index - 1
        if square_frees[start_index] > limit:
            return start_index - 1
        lo, hi = start_index, num_sf - 1
        while lo < hi:
            mid = (lo + hi + 1) // 2
            if square_frees[mid] <= limit:
                lo = mid
            else:
                hi = mid - 1
        return lo

    # Iterative stack-based DFS
    ans = 0
    # Stack contains (prev_index, prod)
    stack = [(0, 1)]

    while stack:
        prev_index, prod = stack.pop()

        # Add contribution: last factor in [sf[prev_index], N // prod]
        max_last = N // prod
        min_sf = square_frees[prev_index] if prev_index < num_sf else 2

        if max_last >= min_sf:
            contrib = get_num_sf(max_last) - get_num_sf(int(min_sf) - 1)
            ans += contrib

        # Find the range of valid next factors
        # We need prod * sf * sf <= N, so sf <= sqrt(N / prod)
        max_sf_for_next = int((N // prod) ** 0.5)
        if max_sf_for_next < 2:
            continue

        # Find the last valid index
        last_valid = find_last_index(prev_index, max_sf_for_next)

        if last_valid < prev_index:
            continue

        # Push all valid extensions in reverse order (so we process in order)
        for index in range(last_valid, prev_index - 1, -1):
            sf = square_frees[index]
            new_prod = prod * sf
            if new_prod * sf > N:  # Double check
                continue
            stack.append((index, new_prod))

    return ans


if __name__ == "__main__":
    print(solve())
