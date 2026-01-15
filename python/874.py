# Project Euler Problem 874
#
# PROBLEM DESCRIPTION:
# Let p(t) denote the (t+1)th prime number. So that p(0) = 2, p(1) = 3, etc.
# We define the prime score of a list of nonnegative integers [a_1, ..., a_n] as the sum of p(a_i).
# Let M(k, n) be the maximal prime score among all lists such that:
# - 0 <= a_i < k for each i
# - the sum of a_i is a multiple of k.
#
# Find M(7000, p(7000)).

from typing import List

def sieve(limit: int) -> List[int]:
    """
    Generate all primes up to the given limit using the Sieve of Eratosthenes.
    """
    if limit < 2:
        return []
    sieve_arr = [True] * (limit + 1)
    sieve_arr[0] = sieve_arr[1] = False
    for i in range(2, int(limit**0.5) + 1):
        if sieve_arr[i]:
            for j in range(i*i, limit + 1, i):
                sieve_arr[j] = False
    return [i for i in range(2, limit + 1) if sieve_arr[i]]

def get_primes_up_to_n_count(count: int) -> List[int]:
    """
    Get the first `count` prime numbers.
    """
    # Approximation: n log n + n log log n
    # For 7000, 7000 * 10 = 70000. Safe limit 100000.
    limit = 100000
    while True:
        primes = sieve(limit)
        if len(primes) >= count:
            return primes[:count]
        limit *= 2

def solve():
    k = 7000
    # We need p(0) to p(k-1) to calculate costs.
    # We also need n = p(7000), so we need the 7001st prime.
    # So we need at least 7001 primes.
    
    primes = get_primes_up_to_n_count(k + 1)
    
    n = primes[k] # p(k) = p(7000) = 7001st prime
    
    # Primes array p(0) ... p(k-1)
    # We will use these to compute costs.
    # p(t) = primes[t]
    
    p_max = primes[k-1] # p(k-1)
    
    # Initial maximal sum (if all a_i = k-1)
    # Sum indices: n * (k-1)
    current_sum_indices = n * (k - 1)
    remainder = current_sum_indices % k
    
    # We need sum_indices % k == 0.
    # We can subtract R from the sum indices.
    # current_sum_indices - R = 0 mod k
    # R = current_sum_indices % k
    
    target_R = remainder
    
    if target_R == 0:
        print(n * p_max)
        return

    # We want to achieve reduction R in the sum of indices with MINIMAL reduction in prime score.
    # Cost to reduce index by d (from k-1 to k-1-d) is:
    # C(d) = p(k-1) - p(k-1-d)
    # We need sum(d_i) = R.
    # We minimize sum(C(d_i)).
    # 0 <= d_i <= k-1 (since a_i >= 0).
    # Since R < k, the constraint d_i <= k-1 is always satisfied if d_i <= R.
    
    # DP for Unbounded Knapsack (or bounded by R, effectively unbounded item count)
    # dp[w] = min cost to get reduction w
    
    # Precompute costs
    # We only need reductions up to R.
    cost = [0] * (target_R + 1)
    for d in range(1, target_R + 1):
        cost[d] = primes[k-1] - primes[k-1-d]
        
    # Initialize DP
    dp = [float('inf')] * (target_R + 1)
    dp[0] = 0
    
    # Since we can use multiple items of any size, and we have plenty of items (n ~ 70000 >> R ~ 7000),
    # we can treat this as unbounded knapsack where we can pick any reduction 'd' any number of times.
    # Iterate through weights
    for w in range(1, target_R + 1):
        # Try all possible last steps 'd'
        # dp[w] = min(cost[d] + dp[w-d]) for 1 <= d <= w
        # Optimizations?
        # Just iterate.
        # For R=7000, inner loop 7000. Total 49e6. 
        # In Python this might be 10-20 seconds.
        # Optimization: We only need to check d such that w-d is valid.
        # Also, because of the nature of primes, maybe we only need to check small d?
        # Or large d?
        # Let's just run it.
        
        min_c = cost[w] # Using a single reduction of size w
        
        # Try splitting
        # Symmetry: only check j up to w/2
        for j in range(1, w // 2 + 1):
            c = dp[j] + dp[w-j]
            if c < min_c:
                min_c = c
        
        dp[w] = min_c
        
    min_cost = dp[target_R]
    
    max_score = n * p_max - min_cost
    print(max_score)

if __name__ == "__main__":
    solve()
