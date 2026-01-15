"""
Project Euler Problem 847
"""
import sys

# Increase recursion depth for deep DP
sys.setrecursionlimit(5000)

memo = {}
MOD = 1_000_000_007

def solve_dp(limit, k, checks):
    limit_bits = limit.bit_length()
    num_bits = max(k, limit_bits)
    
    L_bits = [(limit >> i) & 1 for i in range(num_bits)]
    
    memo.clear()
    
    def dp(j, R, carries, c_states):
        # R is the "excess" value. Range [-3, 2].
        state = (j, R, carries, c_states)
        if state in memo:
            return memo[state]
        
        if j == -1:
            if all(carries) and R <= 0:
                return 1
            return 0
        
        res = 0
        limit_bit = L_bits[j]
        
        for a in range(2):
            for b in range(2):
                for c in range(2):
                    # 1. Update R
                    sum_val = a + b + c
                    new_R = 2 * R + sum_val - limit_bit
                    
                    if new_R >= 2: continue # Prune
                    if new_R <= -3: new_R = -3
                    
                    # 2. Update Carries
                    # A
                    cA = carries[0]
                    valid_ncA = []
                    if cA: 
                        if a == 1: valid_ncA = [True]
                        else: valid_ncA = []
                    else:
                        if a == 0: valid_ncA = [False, True]
                        else: valid_ncA = [False]
                    if not valid_ncA: continue
                    
                    # B
                    cB = carries[1]
                    valid_ncB = []
                    if cB:
                        if b == 1: valid_ncB = [True]
                        else: valid_ncB = []
                    else:
                        if b == 0: valid_ncB = [False, True]
                        else: valid_ncB = [False]
                    if not valid_ncB: continue

                    # C
                    cC = carries[2]
                    valid_ncC = []
                    if cC:
                        if c == 1: valid_ncC = [True]
                        else: valid_ncC = []
                    else:
                        if c == 0: valid_ncC = [False, True]
                        else: valid_ncC = [False]
                    if not valid_ncC: continue
                    
                    for ncA in valid_ncA:
                        for ncB in valid_ncB:
                            for ncC in valid_ncC:
                                next_carries = (ncA, ncB, ncC)
                                
                                bitA = a
                                bitA1 = a + int(ncA) - 2*int(cA)
                                bitB = b
                                bitB1 = b + int(ncB) - 2*int(cB)
                                bitC = c
                                bitC1 = c + int(ncC) - 2*int(cC)
                                
                                # 3. Update Check States
                                new_c_states = []
                                possible = True
                                has_source = (j < k)
                                
                                for idx, check in enumerate(checks):
                                    bA = bitA1 if check[0] else bitA
                                    bB = bitB1 if check[1] else bitB
                                    bC = bitC1 if check[2] else bitC
                                    
                                    prev_states = c_states[idx]
                                    current_possible = set()
                                    
                                    for p_state in prev_states:
                                        psA, psB, psC = p_state
                                        owners = [0, 1, 2] if has_source else [None]
                                        
                                        for owner in owners:
                                            valid_owner = True
                                            nsA, nsB, nsC = psA, psB, psC
                                            
                                            if owner == 0:
                                                if psA == 0:
                                                    if bA == 0: nsA = 1
                                            else:
                                                if psA == 0 and bA == 1: valid_owner = False
                                            if not valid_owner: continue
                                            
                                            if owner == 1:
                                                if psB == 0:
                                                    if bB == 0: nsB = 1
                                            else:
                                                if psB == 0 and bB == 1: valid_owner = False
                                            if not valid_owner: continue
                                            
                                            if owner == 2:
                                                if psC == 0:
                                                    if bC == 0: nsC = 1
                                            else:
                                                if psC == 0 and bC == 1: valid_owner = False
                                            if not valid_owner: continue
                                            
                                            current_possible.add((nsA, nsB, nsC))
                                    
                                    if not current_possible:
                                        possible = False
                                        break
                                    new_c_states.append(frozenset(current_possible))
                                
                                if possible:
                                    res += dp(j - 1, new_R, next_carries, tuple(new_c_states))
        
        memo[state] = res
        return res

    initial_check_states = tuple([frozenset([(0,0,0)]) for _ in checks])
    return dp(num_bits - 1, 0, (False, False, False), initial_check_states)

def C(N):
    if N < 0: return 0
    # Use integer arithmetic for precision, then mod at end if needed
    # But here we need exact value for subtraction
    val = (N + 3) * (N + 2) * (N + 1) // 6 - 1
    return val

def solve() -> int:
    N = int("1" * 19)
    total_H = 0
    k = 0
    
    while True:
        t1 = solve_dp(N - 1, k, [(0,0,0)])
        t2 = solve_dp(N - 2, k, [(0,1,0), (1,0,0)])
        t3 = solve_dp(N - 3, k, [(0,1,1), (1,0,1), (1,1,0)])
        
        size_Sk = 3 * t1 - 3 * t2 + t3
        term = C(N) - size_Sk
        
        if term == 0:
            break
            
        total_H = (total_H + term) % MOD
        k += 1
        if k > 100: break
        
    return total_H

if __name__ == "__main__":
    print(solve())
