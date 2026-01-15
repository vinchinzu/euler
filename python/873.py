# Project Euler Problem 873
#
# PROBLEM DESCRIPTION:
# Let W(p,q,r) be the number of words that can be formed using the letter A p times, 
# the letter B q times and the letter C r times with the condition that every A is 
# separated from every B by at least two Cs.
#
# Find W(10^6, 10^7, 10^8) modulo 1_000_000_007.

import sys

# Increase recursion depth just in case, though we use iterative approach
sys.setrecursionlimit(2000)

MOD = 1_000_000_007

def mod_pow(base, exp, mod):
    return pow(base, exp, mod)

def mod_inv(a, mod):
    return pow(a, mod - 2, mod)

def solve_w(p, q, r):
    """
    Computes W(p, q, r) modulo 10^9 + 7.
    """
    if p == 0:
        # If no A's, any arrangement of B's and C's is valid.
        # Number of ways is binom(q+r, q)
        # We can compute this simply.
        # But for large numbers we need a loop or just reuse the logic below if it handles p=0.
        # The logic below assumes p >= 1 and q >= 1.
        # Let's handle p=0 or q=0 separately.
        if q == 0: return 1 # Only C's
        # binom(q+r, q)
        N = q + r
        K = q
        if K < 0 or K > N: return 0
        if K > N // 2: K = N - K
        
        num = 1
        den = 1
        for i in range(K):
            num = (num * (N - i)) % MOD
            den = (den * (i + 1)) % MOD
        return (num * mod_inv(den, MOD)) % MOD

    if q == 0:
        # Same as p=0 case
        N = p + r
        K = p
        if K < 0 or K > N: return 0
        if K > N // 2: K = N - K
        num = 1
        den = 1
        for i in range(K):
            num = (num * (N - i)) % MOD
            den = (den * (i + 1)) % MOD
        return (num * mod_inv(den, MOD)) % MOD

    # General case: p >= 1, q >= 1
    # K_sb = p + q (Stars and bars buckets count - 1 is implicit, wait)
    # Stars and bars: distribute FreeCs into p+q+1 slots.
    # Formula: binom(FreeCs + (p+q+1) - 1, (p+q+1) - 1) = binom(FreeCs + p + q, p + q)
    
    K_sb = p + q
    
    # Initial N for k=2 (m=1). 
    # Transitions = k - 1 = 1.
    # Mandatory Cs = 2 * 1 = 2.
    # Free Cs = r - 2.
    # N_sb = FreeCs + p + q = r - 2 + p + q.
    
    # Check if r is large enough for minimal separation
    # Min transitions is 1 (since p>=1, q>=1, at least one A-B or B-A).
    if r < 2:
        return 0
        
    curr_N_sb = r - 2 + p + q
    
    # Compute initial Stars and Bars term: binom(curr_N_sb, K_sb)
    # This is O(K_sb)
    # Since K_sb ~ 1.1e7, this is heavy but doable once.
    
    sb_num = 1
    sb_den = 1
    # Optimization: binom(N, K) = N! / (K! (N-K)!)
    # If N is huge, we use the loop form.
    # range(K_sb) is 1e7. 
    
    for i in range(K_sb):
        sb_num = (sb_num * (curr_N_sb - i)) % MOD
        sb_den = (sb_den * (i + 1)) % MOD
        
    curr_sb_val = (sb_num * mod_inv(sb_den, MOD)) % MOD
    
    # Initialize combinations for runs
    # m=1. binom(p-1, 0) = 1, binom(q-1, 0) = 1
    comb_p = 1
    comb_q = 1
    
    ans = 0
    
    # Loop m from 1 upwards
    m = 1
    max_m = min(p, q) # Actually can go up to min(p, q) + 1 in some branches?
    # logic: m is number of runs of A (or B).
    # if k=2m, A-runs=m, B-runs=m.
    # if k=2m+1, A-runs=m+1, B-runs=m OR A-runs=m, B-runs=m+1.
    
    # We need to iterate until we can't form valid run counts.
    # max runs_A is p. max runs_B is q.
    
    while True:
        # Case k = 2m
        # Runs: (m, m)
        # Valid if m <= p and m <= q
        
        if m <= p and m <= q:
            # Skeletons: 2 * binom(p-1, m-1) * binom(q-1, m-1)
            # We have comb_p = binom(p-1, m-1)
            # We have comb_q = binom(q-1, m-1)
            
            term = (2 * comb_p * comb_q) % MOD
            term = (term * curr_sb_val) % MOD
            ans = (ans + term) % MOD
        
        # Before going to k=2m+1, update SB value
        # N decreases by 2
        # binom(N-2, K) = binom(N, K) * ...
        # Let's do it in two steps of -1
        
        # N -> N-1
        # binom(N-1, K) = binom(N, K) * (N-K) / N
        if curr_N_sb - K_sb < 0: # Should verify logic, but effectively 0 if N < K
             curr_sb_val = 0
        else:
             # Step 1
             factor = ((curr_N_sb - K_sb) * mod_inv(curr_N_sb, MOD)) % MOD
             curr_sb_val = (curr_sb_val * factor) % MOD
             curr_N_sb -= 1
             
             # Step 2
             if curr_N_sb == 0 and K_sb > 0: # avoid inv(0)
                 curr_sb_val = 0
             else:
                 factor = ((curr_N_sb - K_sb) * mod_inv(curr_N_sb, MOD)) % MOD
                 curr_sb_val = (curr_sb_val * factor) % MOD
                 curr_N_sb -= 1
        
        if curr_sb_val == 0:
            # If we ran out of C's, we can stop early
            break

        # Case k = 2m+1
        # Two subcases:
        # 1. A-runs = m+1, B-runs = m (Starts A)
        #    Valid if m+1 <= p and m <= q
        # 2. B-runs = m+1, A-runs = m (Starts B)
        #    Valid if m <= p and m+1 <= q
        
        # We need binom(p-1, m) and binom(q-1, m)
        # Update combinations
        # comb(n, k) = comb(n, k-1) * (n - (k-1)) / k
        # Here we want comb(p-1, m) from comb(p-1, m-1).
        
        inv_m = mod_inv(m, MOD)
        
        next_comb_p = 0
        if m <= p - 1:
            next_comb_p = (comb_p * (p - m)) % MOD
            next_comb_p = (next_comb_p * inv_m) % MOD
        
        next_comb_q = 0
        if m <= q - 1:
            next_comb_q = (comb_q * (q - m)) % MOD
            next_comb_q = (next_comb_q * inv_m) % MOD
            
        term_odd = 0
        
        # Subcase 1
        if m + 1 <= p and m <= q:
            # binom(p-1, m) * binom(q-1, m-1)
            t = (next_comb_p * comb_q) % MOD
            term_odd = (term_odd + t) % MOD
            
        # Subcase 2
        if m <= p and m + 1 <= q:
            # binom(p-1, m-1) * binom(q-1, m)
            t = (comb_p * next_comb_q) % MOD
            term_odd = (term_odd + t) % MOD
            
        term_odd = (term_odd * curr_sb_val) % MOD
        ans = (ans + term_odd) % MOD
        
        # Update SB value for next iteration (k goes to 2(m+1) = 2m+2)
        # Again decrease N by 2
        
        # Step 1
        if curr_N_sb <= 0: # Safety
             curr_sb_val = 0
        else:
             if curr_N_sb - K_sb < 0:
                 curr_sb_val = 0
             else:
                 factor = ((curr_N_sb - K_sb) * mod_inv(curr_N_sb, MOD)) % MOD
                 curr_sb_val = (curr_sb_val * factor) % MOD
             curr_N_sb -= 1
             
             # Step 2
             if curr_N_sb <= 0:
                 curr_sb_val = 0
             else:
                 if curr_N_sb - K_sb < 0:
                     curr_sb_val = 0
                 else:
                     factor = ((curr_N_sb - K_sb) * mod_inv(curr_N_sb, MOD)) % MOD
                     curr_sb_val = (curr_sb_val * factor) % MOD
                 curr_N_sb -= 1

        if curr_sb_val == 0 and m > max_m:
            # Stop if no ways and m exceeded bounds
            break
            
        # Move to next m
        comb_p = next_comb_p
        comb_q = next_comb_q
        m += 1
        
        if comb_p == 0 and comb_q == 0:
             # No more skeletons possible
             break
             
    return ans

def solve():
    # Sample check
    # W(2,2,4) = 32
    sample = solve_w(2, 2, 4)
    if sample != 32:
        print(f"Sample W(2,2,4) failed: got {sample}, expected 32")
    
    # Another sample
    # W(4,4,44) = 13908607644
    # 13908607644 % (10^9+7) = 13908607644 % 1000000007 
    # 13908607644 = 13 * 1000000007 + 908607553
    expected_large = 13908607644 % MOD
    sample2 = solve_w(4, 4, 44)
    if sample2 != expected_large:
        print(f"Sample W(4,4,44) failed: got {sample2}, expected {expected_large}")
        
    # If samples pass, compute final
    result = solve_w(10**6, 10**7, 10**8)
    return result

if __name__ == "__main__":
    print(solve())
