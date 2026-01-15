
# Project Euler Problem 850
#
# PROBLEM DESCRIPTION:
# <p>Any positive real number $x$ can be decomposed into integer and fractional parts $\lfloor x \rfloor + \{x\}$, where $\lfloor x \rfloor$ (the floor function) is an integer, and $0\le \{x\} &lt; 1$.</p>
# 
# <p>For positive integers $k$ and $n$, define the function
# $$\begin{align}
# f_k(n) = \sum_{i=1}^{n}\left\{ \frac{i^k}{n} \right\}
# \end{align}$$
# For example, $f_5(10)=4.5$ and $f_7(1234)=616.5$.</p>
# 
# <p>Let
# $$\begin{align}
# S(N) = \sum_{\substack{k=1 \\ k\text{ odd}}}^{N} \sum_{n=1}^{N}  f_k(n)
# \end{align}$$
# You are given that $S(10)=100.5$ and $S(10^3)=123687804$.</p>
# 
# <p>Find $\lfloor S(33557799775533) \rfloor$. Give your answer modulo 977676779.</p>

import sys
from typing import List, Dict, Union, Any

# Increase recursion depth just in case
sys.setrecursionlimit(5000)

def solve() -> int:
    N: int = 33557799775533
    MOD: int = 977676779
    MOD2: int = 2 * MOD
    
    # Max prime needed is sqrt(N)
    SQRT_N: int = int(N**0.5) + 100
    # Sieve primes up to SQRT_N
    # We need P_SUM[x] = sum (p-1) for p <= x
    
    # Using bytearray for sieve is memory efficient
    is_prime = bytearray([1]) * (SQRT_N + 1)
    is_prime[0] = 0
    is_prime[1] = 0
    
    for i in range(2, int(SQRT_N**0.5) + 1):
        if is_prime[i]:
            is_prime[i*i : SQRT_N + 1 : i] = bytearray([0]) * len(range(i*i, SQRT_N + 1, i))
            
    # Build P_SUM array
    # P_SUM[x] stores sum_{p <= x} (p-1)
    p_sum: List[int] = [0] * (SQRT_N + 1)
    
    current_sum: int = 0
    small_primes: List[int] = []
    
    SMALL_PRIME_LIMIT: int = 32000
    
    for i in range(2, SQRT_N + 1):
        if is_prime[i]:
            current_sum += (i - 1)
            if i <= SMALL_PRIME_LIMIT:
                small_primes.append(i)
        p_sum[i] = current_sum

    # Define small odd Ks
    # Max exponent for N is ~45 (2^45 > N)
    # We need K such that ceil(e/K) > 1 is possible, i.e., K < e
    MAX_SMALL_K: int = 45
    small_odd_ks: List[int] = list(range(3, MAX_SMALL_K + 2, 2))
    
    # We will accumulate sums for each K in small_odd_ks and one for "inf" (large K)
    total_sums: Dict[Union[int, str], int] = {k: 0 for k in small_odd_ks}
    total_sums['inf'] = 0
    
    # Precompute powers for C_K calculation
    def get_c_k(p: int, e: int, k: Union[int, str]) -> int:
        # C_K(p^e) = p^(e - ceil(e/k))
        # For 'inf', ceil(e/k) = 1
        if k == 'inf':
            ceil_val = 1
        else:
            # k is guaranteed to be int here due to logic
            assert isinstance(k, int)
            ceil_val = (e + k - 1) // k
        
        exp = e - ceil_val
        return pow(p, exp)

    def calc_tail_sum(M: int) -> int:
        res: int = M
        
        limit_p = int(M**0.5)
        if limit_p <= SMALL_PRIME_LIMIT:
            return res
            
        min_p_bound = SMALL_PRIME_LIMIT
        
        # k goes from 1 up to M / (min_p_bound^2)
        
        k = 1
        while True:
            upper_bound_val = M // k
            lower_bound_val = M // (k + 1)
            
            # p range: (sqrt(lower), sqrt(upper)]
            upper_p = int(upper_bound_val**0.5)
            lower_p = int(lower_bound_val**0.5)
            
            eff_upper = upper_p
            eff_lower = max(lower_p, min_p_bound)
            
            if eff_upper > eff_lower:
                # Sum range
                term_sum = p_sum[eff_upper] - p_sum[eff_lower]
                res += term_sum * k
            
            if eff_upper <= min_p_bound:
                break
            
            k += 1
            
        # Part 2: p^3 terms
        # Correct limit is M^(1/3)
        limit_p3 = int(pow(M, 1/3))
        
        if limit_p3 > SMALL_PRIME_LIMIT:
            for p in range(SMALL_PRIME_LIMIT + 1, limit_p3 + 1):
                if is_prime[p]:
                    val = p*p - p
                    term = M // (p*p*p)
                    res += val * term
                    
        return res

    def dfs(idx: int, current_d: int, current_vals: Dict[Union[int, str], int]) -> None:
        M = N // current_d
        tail_mult = calc_tail_sum(M)
        
        for k in small_odd_ks:
            total_sums[k] = (total_sums[k] + current_vals[k] * tail_mult) % MOD2
            
        total_sums['inf'] = (total_sums['inf'] + current_vals['inf'] * tail_mult) % MOD2
        
        for i in range(idx, len(small_primes)):
            p = small_primes[i]
            
            if current_d * p * p > N:
                break
                
            pe = p * p
            e = 2
            
            while True:
                new_d = current_d * pe
                if new_d > N:
                    break
                
                new_vals: Dict[Union[int, str], int] = {}
                
                for k in small_odd_ks:
                    term = get_c_k(p, e, k) - get_c_k(p, e-1, k)
                    new_vals[k] = (current_vals[k] * term) % MOD2
                    
                term_inf = get_c_k(p, e, 'inf') - get_c_k(p, e-1, 'inf')
                new_vals['inf'] = (current_vals['inf'] * term_inf) % MOD2
                
                dfs(i + 1, new_d, new_vals)
                
                pe *= p
                e += 1
    
    # Initial call
    init_vals: Dict[Union[int, str], int] = {k: 1 for k in small_odd_ks}
    init_vals['inf'] = 1
    dfs(0, 1, init_vals)
    
    # Calculate Final S
    # S = num_odd * N(N+1)/4 - 1/2 * Sum(Sum C_K)
    
    num_odd = (N + 1) // 2
    # N * (N + 1) / 4 might be fractional (X.5), so compute 2*Term1 directly
    # 2 * (num_odd * N(N+1)/4) = num_odd * N(N+1)/2
    term_doubled = (num_odd * (N * (N + 1) // 2)) % MOD2
    
    # Sum of Sigma C_K
    # K=1 => N
    sum_sigma_ck = N
    
    for k in small_odd_ks:
        sum_sigma_ck = (sum_sigma_ck + total_sums[k]) % MOD2
        
    num_small = len(small_odd_ks) + 1 # +1 for K=1
    num_large = num_odd - num_small
    sum_sigma_ck = (sum_sigma_ck + num_large * total_sums['inf']) % MOD2
    
    two_S = (term_doubled - sum_sigma_ck) % MOD2
    
    ans = (two_S // 2) % MOD
    
    return ans

if __name__ == "__main__":
    print(solve())
