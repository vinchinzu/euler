"""
Project Euler Problem 915: Recursive Modulo Sequence
"""

import sys

# Increase recursion depth for deep recursions in S_Phi if necessary
sys.setrecursionlimit(5000)

MOD = 123456789
INV2 = (MOD + 1) // 2  # Modular inverse of 2 mod MOD

def compute_s_phi_table(limit):
    """
    Computes prefix sums of Euler's totient function up to limit.
    Returns the list `S_phi` where S_phi[i] = sum(phi(k) for k in 1..i) % MOD.
    """
    phi = list(range(limit + 1))
    for i in range(2, limit + 1):
        if phi[i] == i:  # i is prime
            for j in range(i, limit + 1, i):
                phi[j] -= phi[j] // i

    # In-place prefix sum modulo MOD
    s_phi = [0] * (limit + 1)
    current = 0
    for i in range(1, limit + 1):
        current = (current + phi[i]) % MOD
        s_phi[i] = current
    return s_phi

# Global memoization for S_Phi
memo_s_phi = {}
PRECOMPUTE_LIMIT = 1000000
precomputed_s_phi = []

def init_s_phi():
    global precomputed_s_phi
    precomputed_s_phi = compute_s_phi_table(PRECOMPUTE_LIMIT)

def S_Phi(n):
    """
    Computes sum_{i=1}^n phi(i) modulo MOD.
    Uses precomputed table for small n, and recursion with memoization for large n.
    Formula: S_Phi(n) = n(n+1)/2 - sum_{k=2}^n S_Phi(floor(n/k))
    """
    if n <= PRECOMPUTE_LIMIT:
        return precomputed_s_phi[n]

    if n in memo_s_phi:
        return memo_s_phi[n]

    # n(n+1)/2 % MOD
    # Since we need exact integer division logic but modulo arithmetic:
    # n*(n+1) is even, so division by 2 is exact.
    # Modulo arithmetic: n % MOD * (n+1) % MOD * INV2 % MOD
    term1 = (n % MOD) * ((n + 1) % MOD) % MOD
    term1 = (term1 * INV2) % MOD

    sub_sum = 0
    l = 2
    while l <= n:
        val = n // l
        # Find largest r such that n // r == val
        if val == 0:
            r = n
        else:
            r = n // val

        count = (r - l + 1) % MOD
        term = (count * S_Phi(val)) % MOD
        sub_sum = (sub_sum + term) % MOD

        l = r + 1

    res = (term1 - sub_sum + MOD) % MOD
    memo_s_phi[n] = res
    return res

# Handling H(g) = s(s(g)) % MOD
# Periodicity analysis from exploration:
# Modulo M sequence period parameters:
# s(n) % M: Preperiod 53, Period 33705
# s(n) % 33705: Preperiod 2, Period 420
# s(n) % 420: Preperiod 2, Period 12
#
# For g >= 5, s(g) is very large (> 53) and s(g) mod 33705 is determined by g mod 420.
# Hence H(g) is periodic with period 420 for g >= 5.

H_vals = []
H_prefix = []

def compute_H_period():
    global H_vals, H_prefix

    # We need to compute s(s(g)) % MOD.
    # Since s(g) grows very fast, we cannot compute s(g) directly.
    # We need s(g) modulo something relevant.
    # Since we want s(X) % MOD, and s(n) % MOD has preperiod 53, period 33705.
    # If X > 53, s(X) % MOD depends on X % 33705 (shifted).
    # Specifically, if X > 53, let X_eff = 53 + (X - 53) % 33705.
    # Then s(X) % MOD == s(X_eff) % MOD.
    
    # We need to compute X = s(g).
    # For small g (1, 2, 3, 4), s(g) is small.
    # s(1)=1, s(2)=2, s(3)=3, s(4)=10. All <= 53.
    # So for g<=4, H(g) = s(s(g)) % MOD.
    
    # For g >= 5, s(g) > 53.
    # We need s(g) % 33705.
    # s(n) % 33705 has preperiod 2, period 420.
    # Since g >= 5 > 2, we represent g in the cycle.
    # s(g) % 33705 depends on g % 420.
    # Specifically, for g > 2:
    # s(g) % 33705 == s(2 + (g - 2) % 420) % 33705 (roughly, checking indices).
    # Actually, simpler to just generate a lookup table for g in range covering the period.
    # Period 420 starts from g=5 (based on s(5)=731 > 53 and g>2).
    # Let's generate H(g) for g up to e.g. 1000.

    # Generate s(n) mod 33705 up to a limit
    # We need s(val) mod MOD.
    # Let's just implement s_step function.

    def s_step(x, m):
        return ((x - 1)**3 + 2) % m

    # Precompute s sequence modulo MOD up to 53 + 33705
    # To lookup s(k) % MOD
    S_mod_M = [0]
    curr = 1
    limit_M = 53 + 33705 + 100
    S_mod_M.append(curr)
    for _ in range(limit_M):
        curr = s_step(curr, MOD)
        S_mod_M.append(curr)

    # Precompute s sequence modulo 33705 up to 2 + 420 + 100
    # To lookup s(g) % 33705
    S_mod_P1 = [0]
    curr = 1
    limit_P1 = 2 + 420 + 100
    S_mod_P1.append(curr)
    for _ in range(limit_P1):
        curr = s_step(curr, 33705)
        S_mod_P1.append(curr)

    H_vals = [0] # 1-based indexing

    # Calculate H(g) for g = 1 to 1000
    for g in range(1, 1001):
        if g <= 4:
            # Direct calculation
            sg = S_mod_M[g] # s(g) for small g is just the value itself since s(4)=10
            # Wait, S_mod_M stores s(k) mod M.
            # s(1)=1, s(2)=2, s(3)=3, s(4)=10.
            # s(s(1)) = s(1) = 1
            # s(s(2)) = s(2) = 2
            # s(s(3)) = s(3) = 3
            # s(s(4)) = s(10)
            # We can look these up in S_mod_M directly.
            val = S_mod_M[sg]
            H_vals.append(val)
        else:
            # g >= 5. s(g) is huge.
            # We need s(g) % 33705 to find index in S_mod_M cycle.
            # s(g) % 33705 is periodic for g > 2.
            # Map g to range [3, 422] (period 420).
            # Effective index for S_mod_P1 lookup:
            # We need index k such that s(g) = s(k) mod 33705.
            # Since g >= 5 > 2, we are in cycle of s mod 33705.
            # Cycle starts at index 3 (since preperiod is 2, indices 1, 2 are preperiod).
            # Index 3 corresponds to start of cycle?
            # Let's rely on S_mod_P1 array being long enough.
            # For g >= 5:
            # eff_g = 3 + (g - 3) % 420
            # This maps 5 -> 5, 422 -> 422, 423 -> 3. Correct?
            # Wait, preperiod 2 means 1, 2 are unique. 3 starts cycle?
            # Let's verify with explicit check.
            # s(3) mod 33705 = 3.
            # s(423) mod 33705 = 3?
            # If period is 420 starting after 2.

            # Use computed S_mod_P1 directly for small g, map for large.
            # But here g is up to 1000, we computed S_mod_P1 up to ~522.
            # Let's extend S_mod_P1 or map g.

            eff_g = g
            if g > 2:
                eff_g = 3 + (g - 3) % 420

            s_g_mod_P1 = S_mod_P1[eff_g]

            # Now we have idx = s(g) % 33705.
            # We need s(s(g)) % MOD.
            # This is s(idx) % MOD IF we account for preperiod of s mod MOD.
            # Preperiod is 53.
            # Since g >= 5, s(g) > 53.
            # So s(s(g)) % MOD is determined by s(g) % 33705 in the cycle.
            # We need to map the index to the cycle of s mod MOD.
            # Cycle of s mod MOD starts at 54 (indices 1..53 preperiod).
            # Length 33705.
            # We need K such that K > 53 and K = s(g) mod 33705.
            # If s_g_mod_P1 <= 53, we add 33705?
            # Or is s_g_mod_P1 already consistent?
            # Let's take K = s_g_mod_P1.
            # While K <= 53: K += 33705.
            # This K should satisfy K = s(g) mod 33705 and K > 53.

            K = s_g_mod_P1
            while K <= 53:
                K += 33705

            # Now lookup S_mod_M[K]
            # But S_mod_M needs to be large enough.
            # We only computed up to 53 + 33705 + 100.
            # If K is within this range, good.
            # K is at most 53 + 33705.
            val = S_mod_M[K]
            H_vals.append(val)

    # Build prefix sums of H
    H_prefix = [0] * (len(H_vals))
    curr = 0
    for i in range(1, len(H_vals)):
        curr = (curr + H_vals[i]) % MOD
        H_prefix[i] = curr

def get_sum_H(n):
    """
    Computes sum_{i=1}^n H(i) % MOD.
    H(i) has period 420 for i >= 5.
    """
    if n < len(H_prefix):
        return H_prefix[n]

    # Range [1, n] split into:
    # [1, 4] -> fixed
    # [5, n] -> periodic with period 420

    # Sum up to 4
    sum_pre = H_prefix[4]

    # Remaining count
    count = n - 4

    # Period length P = 420
    P = 420

    num_full_periods = count // P
    rem_in_period = count % P

    # Sum of one full period starting at 5
    # Range [5, 5 + 420 - 1] = [5, 424]
    sum_period = (H_prefix[4 + P] - H_prefix[4] + MOD) % MOD

    total = sum_pre
    total = (total + num_full_periods * sum_period) % MOD

    # Remainder
    # Range [5, 5 + rem - 1]
    # Length rem.
    # Sum is H_prefix[4 + rem] - H_prefix[4]
    # Note: 4 + rem corresponds to index in H_prefix.
    # Since H_vals has enough precomputed values (1000 > 425), this works.

    term_rem = (H_prefix[4 + rem_in_period] - H_prefix[4] + MOD) % MOD
    total = (total + term_rem) % MOD

    return total

def solve(N):
    init_s_phi()
    compute_H_period()

    total_sum = 0

    # We compute sum_{g=1}^N H(g) * (2 * S_Phi(floor(N/g)) - 1)
    # Rewrite as sum_{v} (2 * S_Phi(v) - 1) * sum_{g where floor(N/g)==v} H(g)

    # Iterate over ranges of g where floor(N/g) is constant
    l = 1
    while l <= N:
        val = N // l
        if val == 0:
            r = N
        else:
            r = N // val

        # Range [l, r]
        # Sum of H(g) in this range
        sum_H_range = (get_sum_H(r) - get_sum_H(l - 1) + MOD) % MOD

        # Weight
        # weight = 2 * S_Phi(val) - 1
        # Careful with -1 mod MOD
        phi_val = S_Phi(val)
        weight = (2 * phi_val - 1) % MOD

        term = (sum_H_range * weight) % MOD
        total_sum = (total_sum + term) % MOD

        l = r + 1

    return total_sum

def main():
    if len(sys.argv) > 1:
        N = int(sys.argv[1])
    else:
        N = 10**8

    result = solve(N)
    print(result)

if __name__ == "__main__":
    main()
