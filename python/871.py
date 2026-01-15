import sys

# Increase recursion depth just in case, though we aim for iterative
sys.setrecursionlimit(200000)

def solve():
    total_D = 0
    # Problem asks for sum of D(f_n) for n = 10^5 + 1 to 10^5 + 100
    start_n = 10**5 + 1
    end_n = 10**5 + 100
    
    for n in range(start_n, end_n + 1):
        d = compute_D(n)
        total_D += d
        
    return total_D

def compute_D(n):
    # 1. Compute f(x) and build graph
    # f(x) = (x^3 + x + 1) % n
    f = [(x*x*x + x + 1) % n for x in range(n)]
    
    # 2. Compute in-degrees
    in_degree = [0] * n
    for x in range(n):
        in_degree[f[x]] += 1
        
    # 3. Topological sort to process trees
    # Queue of leaves
    queue = [x for x in range(n) if in_degree[x] == 0]
    
    # P[u]: Max size in subtree of u (including u) given u is IN A
    # S[u]: Max size in subtree of u given u is NOT IN A
    # Initialize for leaves (and all nodes initially)
    # P starts at 1 (u itself), S starts at 0
    # We will accumulate children contributions into these
    
    # To process efficiently, we track accumulated sums and max_diff for each node
    # sum_S[v]: sum of S[child] for all processed children
    # max_diff[v]: max(P[child] - S[child]) for all processed children
    
    sum_S = [0] * n
    max_diff = [0] * n
    
    # Process queue
    head = 0
    while head < len(queue):
        u = queue[head]
        head += 1
        
        # Calculate final P and S for u based on its processed children
        # For a leaf, sum_S=0, max_diff=0 -> P=1, S=0. Correct.
        
        # P[u] = 1 + sum S(children)
        # S[u] = sum S(children) + max(0, max(P(child) - S(child)))
        
        p_u = 1 + sum_S[u]
        s_u = sum_S[u] + max_diff[u]
        
        # Propagate to parent v = f(u)
        v = f[u]
        
        # Update parent's accumulators
        sum_S[v] += s_u
        diff = p_u - s_u
        if diff > max_diff[v]:
            max_diff[v] = diff
            
        # Decrement in-degree
        in_degree[v] -= 1
        if in_degree[v] == 0:
            queue.append(v)
            
    # 4. Process Cycles
    # Nodes with in_degree > 0 are part of cycles (or reachable from cycles?? 
    # No, in a functional graph, only cycles remain after stripping trees)
    
    total_max = 0
    visited = [False] * n
    
    for i in range(n):
        if in_degree[i] > 0 and not visited[i]:
            # Found a new cycle component
            cycle = []
            curr = i
            while not visited[curr]:
                visited[curr] = True
                cycle.append(curr)
                curr = f[curr]
            
            # Although we found the cycle, the 'curr' might not be 'i' if we entered midway?
            # Actually, after topo sort, only pure cycles remain. 
            # Any node with in_degree > 0 MUST be on a cycle.
            # So 'cycle' is exactly the cycle elements in order.
            
            # Calculate P' and S' for cycle nodes (contributions from tree children)
            # These are already in sum_S and max_diff!
            # Just need to compute the final P' and S' values.
            
            cycle_P = []
            cycle_S = []
            for node in cycle:
                p_node = 1 + sum_S[node]
                s_node = sum_S[node] + max_diff[node]
                cycle_P.append(p_node)
                cycle_S.append(s_node)
                
            total_max += solve_cycle(cycle_P, cycle_S)
            
    return total_max

def solve_cycle(P, S):
    k = len(P)
    if k == 0:
        return 0
    
    # Case 1: c0 is NOT in A (state 0)
    # Initial DP state at c0
    # dp[0] = S[0] (Value if c0 not in A, and no cycle child picked)
    # dp[1] = -infinity
    
    dp0 = S[0]
    dp1 = -float('inf')
    
    for i in range(1, k):
        # Transition from i-1 to i
        # New dp0: c_i not in A
        #  from old_dp0 (c_{i-1} not in A): gain S[i]
        #  from old_dp1 (c_{i-1} in A): gain S[i] (Wait, check logic)
        
        # Logic Recap:
        # 0 -> 0: c_{i-1} not in A, c_i not in A. Value += S[i]
        # 1 -> 0: c_{i-1} in A, c_i not in A. Value += P[i] - 1
        # 0 -> 1: c_{i-1} not in A, c_i in A. Value += P[i]
        # 1 -> 1: Forbidden
        
        next_dp0 = max(dp0 + S[i], dp1 + P[i] - 1)
        next_dp1 = dp0 + P[i]
        
        dp0, dp1 = next_dp0, next_dp1
        
    # End of cycle. Check consistency with c0=0
    # If end in 0 (c_{k-1} not in A): Transition 0 -> 0 valid.
    # If end in 1 (c_{k-1} in A): Transition 1 -> 0 valid.
    #   But c0 was treated as having NO cycle child (S[0]).
    #   If c_{k-1} in A, c0 HAS a cycle child.
    #   So value should have been P[0] - 1.
    #   Correction: Subtract S[0], add P[0] - 1.
    
    res0 = dp0 # Valid 0->0
    res1 = dp1 - S[0] + (P[0] - 1) # Valid 1->0 with correction
    
    ans_case0 = max(res0, res1)
    
    # Case 2: c0 is IN A (state 1)
    # dp[0] = -infinity
    # dp[1] = P[0]
    
    dp0 = -float('inf')
    dp1 = P[0]
    
    for i in range(1, k):
        next_dp0 = max(dp0 + S[i], dp1 + P[i] - 1)
        next_dp1 = dp0 + P[i]
        dp0, dp1 = next_dp0, next_dp1
        
    # End of cycle. Check consistency with c0=1
    # Must end in 0 (c_{k-1} not in A) because c0 in A implies f(c_{k-1})=c0 in A -> impossible?
    # Wait. If c0 in A, then f(c_{k-1}) is in A.
    # Constraint: x in A -> f(x) not in A.
    # So c_{k-1} in A -> f(c_{k-1}) not in A -> c0 not in A.
    # This contradicts c0 in A.
    # So c_{k-1} CANNOT be in A.
    # So end state must be 0.
    
    # Transition 0 -> 1.
    # c_{k-1} not in A. c0 in A.
    # c0 takes no cycle child (since c_{k-1} not in A).
    # P[0] assumes no cycle child?
    # P[0] = 1 + sum S(tree). Yes. Correct.
    
    ans_case1 = dp0
    
    return max(ans_case0, ans_case1)

if __name__ == "__main__":
    print(solve())
