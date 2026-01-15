"""Project Euler Problem 154: Exploring Pascal's pyramid."""

import sys

# Increase recursion depth just in case, though we use stack
sys.setrecursionlimit(2000)

def main() -> int:
    """Count ordered triples (a, b, c) with v5(a!b!c!/(a+b+c)!) >= 12."""
    N = 200_000
    REQ = 12
    REQ_V5 = REQ
    LIMIT_V2 = REQ - 1  # We count "bad" cases where v2 < 12, i.e. <= 11

    # Precompute v5_arr
    # v5(n!)
    v5_arr = [0] * (N + 1)
    for i in range(1, N + 1):
        c = 0
        temp = i
        while temp % 5 == 0:
            c += 1
            temp //= 5
        v5_arr[i] = v5_arr[i - 1] + c
    v5_n = v5_arr[N]

    # --- Part 1: Compute Total with High v5 (>= 12) using Base 5 DP ---
    base5 = []
    v = N
    while v > 0:
        base5.append(v % 5)
        v //= 5
    if not base5: base5.append(0)
    len5 = len(base5)
    max_carries_5 = 2 * len5

    # Sum ways
    ways_sum_5 = [0] * 13
    for da in range(5):
        for db in range(5):
            for dc in range(5):
                ways_sum_5[da + db + dc] += 1

    # dp[carry_in][carry_total]
    dp = [[0] * (max_carries_5 + 1) for _ in range(3)]
    dp[0][0] = 1

    for pos in range(len5):
        digit = base5[pos]
        new_dp = [[0] * (max_carries_5 + 1) for _ in range(3)]
        for cin in range(3):
            for ct in range(max_carries_5 + 1):
                w = dp[cin][ct]
                if w == 0: continue
                
                # s + cin = digit + 5 * cout
                # cout ranges 0..2
                for cout in range(3):
                    s = digit + 5 * cout - cin
                    if 0 <= s <= 12:
                        nct = ct + cout
                        if nct <= max_carries_5:
                            new_dp[cout][nct] += w * ways_sum_5[s]
        dp = new_dp

    ordered_total_v5 = sum(dp[0][ct] for ct in range(REQ_V5, max_carries_5 + 1))

    # --- Part 2: Count Bad (v5 >= 12 AND v2 < 12) using Base 2 Iteration ---
    base2 = []
    v = N
    while v > 0:
        base2.append(v % 2)
        v //= 2
    if not base2: base2.append(0)
    len2 = len(base2)
    
    # Precompute powers of 2 for fast 'a' construction
    powers2 = [1 << i for i in range(len2 + 1)]
    
    # Explicit stack for DFS
    
    bad_count = 0
    
    # Optimization: Precompute valid (da, db, dc) tuples for each (digit, cin, cout)
    # transitions[digit][cin][cout] = list of (da, db, dc)
    # digit in 0..1, cin in 0..2, cout in 0..2
    # s = digit + 2*cout - cin
    # if 0 <= s <= 3, valid da,db,dc summing to s
    
    # Precompute da,db,dc sums
    tuples_by_sum = [[] for _ in range(4)]
    for da in range(2):
        for db in range(2):
            for dc in range(2):
                tuples_by_sum[da+db+dc].append((da, db, dc))
                
    transitions = [] 
    for digit in range(2):
        cin_layer = []
        for cin in range(3):
            cout_layer = []
            for cout in range(3):
                s = digit + 2*cout - cin
                match_tuples = []
                if 0 <= s <= 3:
                     match_tuples = tuples_by_sum[s]
                cout_layer.append(match_tuples)
            cin_layer.append(cout_layer)
        transitions.append(cin_layer)

    # Stack: list of tuples (pos, cin, ct, a, b, c)
    stack = [(0, 0, 0, 0, 0, 0)]
    
    while stack:
        pos, cin, ct, a, b, c = stack.pop()
        
        if pos == len2:
            if cin == 0:
                # Leaf check
                # Check v5 >= 12
                # v5 = v5_n - v5_arr[a] - v5_arr[b] - v5_arr[c]
                # Optimization: check directly
                val = v5_n - v5_arr[a] - v5_arr[b] - v5_arr[c]
                if val >= REQ_V5:
                    bad_count += 1
            continue
            
        # Not leaf
        digit = base2[pos]
        # Iterate cout
        # For given pos, digit, cin, we can have possible couts
        # transitions[digit][cin] -> list of 3 elements (for cout 0,1,2)
        
        possibilities = transitions[digit][cin]
        
        # Iterate cout 0, 1, 2
        for cout in range(3):
            valid_tuples = possibilities[cout]
            if not valid_tuples:
                continue
            
            new_ct = ct + cout
            if new_ct > LIMIT_V2:
                continue
                
            p2 = powers2[pos]
            
            # For each tuple, push state
            # Note: We push in reverse order to maintain similar traversal order to recursion (optional)
            for (da, db, dc) in valid_tuples:
                stack.append((
                    pos + 1,
                    cout,
                    new_ct,
                    a + da * p2,
                    b + db * p2,
                    c + dc * p2
                ))

    return ordered_total_v5 - bad_count

if __name__ == "__main__":
    print(main())
