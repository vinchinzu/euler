from __future__ import annotations
import sys

# Increase recursion depth just in case
sys.setrecursionlimit(2000)

MOD = 1405695061
CUTOFF = 40000

def solve_compressed(k, N):
    """
    Computes M_k(N) using a compressed state BFS.
    State: (ones_count, non_ones_tuple)
    non_ones_tuple is sorted.
    """
    # State: (ones_count, non_ones_tuple)
    initial = (k, ())
    visited = set()
    visited.add(initial)
    queue = [initial]
    found_numbers = {1}
    
    while queue:
        ones, non_ones = queue.pop(0)
        
        P = 1
        for x in non_ones:
            P *= x
            
        # 1. Try replacing a 1 (if any)
        if ones > 0:
            # x = 1, P_others = P
            # val = k * P - 1
            # Check bounds carefully to avoid huge number creation
            if k * P <= N + 1:
                val = k * P - 1
                if val <= N and val > 1:
                    new_non_ones = list(non_ones)
                    new_non_ones.append(val)
                    new_non_ones.sort()
                    new_state = (ones - 1, tuple(new_non_ones))
                    
                    if new_state not in visited:
                        visited.add(new_state)
                        found_numbers.add(val)
                        queue.append(new_state)
        
        # 2. Try replacing each non-one
        for i, x in enumerate(non_ones):
            # P_others = P // x
            # val = k * P_others - x
            
            P_others = P // x
            
            if k * P_others > N + x:
                continue
                
            val = k * P_others - x
            
            if val <= N:
                if val > x: # Only move up
                    new_non_ones = list(non_ones)
                    new_non_ones[i] = val
                    new_non_ones.sort()
                    new_state = (ones, tuple(new_non_ones))
                    
                    if new_state not in visited:
                        visited.add(new_state)
                        found_numbers.add(val)
                        queue.append(new_state)
    
    return sum(found_numbers) % MOD

def sum_pow(p, n, mod):
    """Computes sum_{i=1}^n i^p mod mod."""
    if n < 1: return 0
    n_val = n # Keep original n for calculation
    
    if p == 0: return n_val % mod
    
    if p == 1:
        # n(n+1)/2
        val = n_val * (n_val+1)
        inv2 = (mod + 1) // 2
        return (val % mod) * inv2 % mod
        
    if p == 2:
        # n(n+1)(2n+1)/6
        val = n_val * (n_val+1) % mod
        val = val * (2*n_val+1) % mod
        inv6 = pow(6, -1, mod)
        return val * inv6 % mod
        
    if p == 3:
        # (n(n+1)/2)^2
        val = n_val * (n_val+1)
        inv2 = (mod + 1) // 2
        val = (val % mod) * inv2 % mod
        return val * val % mod
        
    if p == 4:
        # n(n+1)(2n+1)(3n^2+3n-1)/30
        t1 = n_val * (n_val+1) % mod
        t2 = (2*n_val+1) % mod
        t3 = (3*n_val*n_val + 3*n_val - 1) % mod
        val = t1 * t2 % mod
        val = val * t3 % mod
        inv30 = pow(30, -1, mod)
        return val * inv30 % mod
        
    return 0

def poly_eval_sum(coeffs, limit, mod):
    """
    Computes sum_{k=1}^limit P(k) mod mod, where P(k) = sum coeffs[i] * k^i.
    coeffs is list [c0, c1, c2, ...] for c0 + c1*k + c2*k^2...
    """
    total = 0
    for p, c in enumerate(coeffs):
        s_p = sum_pow(p, limit, mod)
        term = (c % mod) * s_p % mod
        total = (total + term) % mod
    return total

def poly_sum_range(coeffs, start_k, end_k, mod):
    """Computes sum_{k=start_k}^end_k P(k) mod mod."""
    if start_k > end_k: return 0
    s_end = poly_eval_sum(coeffs, end_k, mod)
    s_start = poly_eval_sum(coeffs, start_k - 1, mod)
    return (s_end - s_start + mod) % mod

def poly_shift(P):
    # Multiply by k: [c0, c1, ...] -> [0, c0, c1, ...]
    return [0] + P

def poly_sub(P, Q):
    # P - Q
    l = max(len(P), len(Q))
    res = [0]*l
    for i in range(l):
        a = P[i] if i < len(P) else 0
        b = Q[i] if i < len(Q) else 0
        res[i] = a - b
    return res

def get_trunk_polys(max_n):
    # v1 = [1]
    # v2 = [-1, 1] (k-1)
    # v3 = [-1, -1, 1] (k^2 - k - 1)
    # vn = k * v_{n-1} - v_{n-2}
    polys = []
    v1 = [1]
    v2 = [-1, 1]
    polys.append(v1)
    polys.append(v2)
    
    for i in range(2, max_n):
        prev = polys[-1]
        prev2 = polys[-2]
        # next = k * prev - prev2
        term1 = poly_shift(prev)
        v_next = poly_sub(term1, prev2)
        polys.append(v_next)
        
    return polys

def eval_poly(poly, k):
    res = 0
    kp = 1
    for c in poly:
        res += c * kp
        kp *= k
    return res

def find_limit(poly, N, start_k, max_k):
    # Find max k in [start_k, max_k] such that poly(k) <= N.
    # Poly is monotonic increasing for k >= 3.
    # Binary search.
    low = start_k
    high = max_k
    ans = start_k - 1
    
    # If poly(start_k) > N, return start_k - 1
    if eval_poly(poly, start_k) > N:
        return start_k - 1
        
    while low <= high:
        mid = (low + high) // 2
        val = eval_poly(poly, mid)
        if val <= N:
            ans = mid
            low = mid + 1
        else:
            high = mid - 1
    return ans

def solve() -> int:
    K_VAL = 10**18
    N_VAL = 10**18
    
    if len(sys.argv) > 2:
        K_VAL = int(sys.argv[1])
        N_VAL = int(sys.argv[2])
        
    total_sum = 0
    
    # 1. Small k: use Compressed BFS
    limit_small = min(CUTOFF, K_VAL)
    
    for k in range(3, limit_small + 1):
        mk = solve_compressed(k, N_VAL)
        total_sum = (total_sum + mk) % MOD
        
    # 2. Large k: use polynomial summation
    if K_VAL > CUTOFF:
        # We need v1, v2, v3, v4.
        # v5(40000) > 10^18, so v5 is not needed for k > 40000.
        
        polys = get_trunk_polys(5) # Get v1 to v5
        
        start_k_large = CUTOFF + 1
        
        for m in range(4): # v1, v2, v3, v4 (indices 0..3)
            poly = polys[m]
            limit = find_limit(poly, N_VAL, start_k_large, K_VAL)
            
            if limit >= start_k_large:
                term_sum = poly_sum_range(poly, start_k_large, limit, MOD)
                total_sum = (total_sum + term_sum) % MOD
            
    return total_sum

if __name__ == "__main__":
    print(solve())
