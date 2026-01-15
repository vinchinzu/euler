
import sys

# Increase recursion depth for deep recursion if necessary, though 9 is small.
sys.setrecursionlimit(2000)

def solve():
    MOD = 10**9 + 7
    N = 800
    
    # For testing with G(5)
    # N = 5
    
    # Primes and classification
    primes = []
    is_prime = [True] * (N + 1)
    is_prime[0] = is_prime[1] = False
    for p in range(2, N + 1):
        if is_prime[p]:
            primes.append(p)
            for i in range(p * p, N + 1, p):
                is_prime[i] = False
                
    sqrt_N = int(N**0.5)
    small_primes = [p for p in primes if p <= sqrt_N]
    large_primes = [p for p in primes if p > sqrt_N]
    
    # Generate smooth numbers (only small prime factors)
    smooths = []
    def generate_smooth(idx, current):
        if idx == len(small_primes):
            if current <= N:
                smooths.append(current)
            return

        p = small_primes[idx]
        # Try all powers of p
        pe = 1
        while current * pe <= N:
            generate_smooth(idx + 1, current * pe)
            pe *= p
    
    generate_smooth(0, 1)
    smooths.sort()
    num_smooths = len(smooths)
    
    # Map from value to index in smooths
    # Actually we only need to look up counts.
    # We need to find index i such that smooths[i] <= K < smooths[i+1]
    # bisect_right
    from bisect import bisect_right
    
    # Constants W_p for all primes
    # W_p = p^k_max where p^k_max <= N < p^(k_max+1)
    W_vals = {}
    for p in primes:
        pe = 1
        while pe * p <= N:
            pe *= p
        W_vals[p] = pe
        
    # Precompute constant C = product of W_p
    C = 1
    for p in primes:
        C = (C * W_vals[p]) % MOD
        
    # Modular Inverse
    def modinv(a):
        return pow(a, MOD - 2, MOD)
        
    # Weight function w(p, k)
    # if k=0: 1
    # if k>0: -phi(p^k) / W_p
    def get_weight(p, k):
        if k == 0:
            return 1
        # phi(p^k) = p^k - p^(k-1)
        phi = (pow(p, k, MOD) - pow(p, k-1, MOD)) % MOD
        num = (-phi) % MOD
        den = W_vals[p]
        return (num * modinv(den)) % MOD

    # Groups for large primes
    # Map K -> List of large primes P such that floor(N/P) = K
    # Only need K where list is non-empty.
    # K = N // P.
    large_groups = {}
    for P in large_primes:
        K = N // P
        if K not in large_groups:
            large_groups[K] = []
        large_groups[K].append(P)
        
    # Precompute polynomials for each group
    # Poly_K[cnt] = Product_{P in group} (2^cnt + w(P, 1))
    # w(P, 1) since P is large, max exponent is 1.
    # w(P, 1) = -(P-1)/P (mod MOD) since W_P = P for P > sqrt(N) (mostly)
    # Wait, if P > sqrt(N), then P^2 > N, so W_P = P. Correct.
    
    # We need table Lookup_K[cnt] for cnt in 0..idx_K
    # idx_K is count of smooths <= K.
    
    group_lookups = [] # List of (idx_mask, lookup_table)
    
    for K, P_list in large_groups.items():
        limit_idx = bisect_right(smooths, K)
        # Lookup table size limit_idx + 1
        table = [1] * (limit_idx + 1)
        
        # Compute product
        # For each P, compute term = (2^cnt + w(P))
        # w(P) = -(P-1)/P
        # But we can sum log-probs? No, sum over field.
        
        # Calculate w_P list
        ws = []
        for P in P_list:
            w = get_weight(P, 1)
            ws.append(w)
            
        # Populate table
        # For each possible cnt
        # This might be slow if group is large and table is large?
        # Group size can be large (e.g. K=1, many P).
        # But table is small (limit_idx for K=1 is 1, smooths <= 1 is {1}).
        # For K=27, limit_idx is larger.
        
        for cnt in range(limit_idx + 1):
            val = 1
            pow2 = pow(2, cnt, MOD)
            for w in ws:
                term = (pow2 + w) % MOD
                val = (val * term) % MOD
            table[cnt] = val
            
        # Determine mask for this K
        # We need mask to extract bits 0..limit_idx-1
        # mask = (1 << limit_idx) - 1
        # Actually we'll store limit_idx
        group_lookups.append({
            'limit': limit_idx,
            'table': table
        })
        
    # Also need the term for the smooth numbers themselves (K=N)
    # This corresponds to the factor 2^M(cS) * W(cS)
    # M(cS) is bit_count of full mask.
    # We handle W(cS) in the recursion.
    # The 2^M(cS) is simply pow(2, bit_count, MOD).
    # We can precompute powers of 2.
    pow2_all = [pow(2, i, MOD) for i in range(num_smooths + 1)]

    # Precompute bitmasks for small primes
    # masks[p][k] = integer bitmask
    masks = {}
    
    for p in small_primes:
        masks[p] = {}
        # Possible exponents: 0 to max_e
        max_e = 0
        pe = 1
        while pe <= N:
            pe *= p
            max_e += 1
        max_e -= 1 # Correct
        
        # k=0
        # fits if v_p(y) < infinity (always)
        masks[p][0] = (1 << num_smooths) - 1
        
        for k in range(1, max_e + 1):
            m = 0
            for i, y in enumerate(smooths):
                # Check v_p(y)
                v = 0
                temp = y
                while temp > 0 and temp % p == 0:
                    v += 1
                    temp //= p
                
                if v < k:
                    m |= (1 << i)
            masks[p][k] = m

    # Recursion
    total_sum = 0
    
    # Prepare list of small primes and their options
    small_options = []
    for p in small_primes:
        opts = []
        # k=0
        opts.append((0, get_weight(p, 0)))
        # k=1..max_e
        pe = p
        k = 1
        while pe <= N:
            opts.append((k, get_weight(p, k)))
            pe *= p
            k += 1
        small_options.append((p, opts))
        
    def recurse(idx, current_mask, current_weight):
        nonlocal total_sum
        
        if idx == len(small_options):
            # Leaf
            term = current_weight
            
            # Factor from smooth numbers (2^M)
            M = current_mask.bit_count()
            term = (term * pow2_all[M]) % MOD
            
            # Factors from large groups
            for grp in group_lookups:
                limit = grp['limit']
                # extract bits relevant to this group
                # The first 'limit' bits of current_mask
                # mask is (1 << limit) - 1
                sub_mask = current_mask & ((1 << limit) - 1)
                cnt = sub_mask.bit_count()
                term = (term * grp['table'][cnt]) % MOD
                
            total_sum = (total_sum + term) % MOD
            return

        p, opts = small_options[idx]
        p_masks = masks[p]
        
        for k, w in opts:
            # Update mask
            new_mask = current_mask & p_masks[k]
            # Update weight
            new_weight = (current_weight * w) % MOD
            
            recurse(idx + 1, new_mask, new_weight)
            
    # Start recursion
    initial_mask = (1 << num_smooths) - 1
    recurse(0, initial_mask, 1)
    
    final_ans = (total_sum * C) % MOD
    return final_ans

if __name__ == "__main__":
    print(solve())
