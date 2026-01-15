import sys
import time

# Increase recursion depth
sys.setrecursionlimit(200000)

def solve_n(N):
    print(f"Solving for N={N}...", flush=True)
    start_total = time.time()
    
    # 1. Build Graph
    min_prime = [0] * (N + 1)
    primes = []
    for i in range(2, N + 1):
        if min_prime[i] == 0:
            min_prime[i] = i
            primes.append(i)
            for j in range(i * i, N + 1, i):
                if min_prime[j] == 0:
                    min_prime[j] = i
                    
    allowed = [1, 2]
    for p in primes:
        if p == 2: continue
        pk = p
        while pk <= N:
            allowed.append(pk)
            if 2 * pk <= N:
                allowed.append(2 * pk)
            pk *= p
            
    allowed.sort()
    num_set = set(allowed)
    print(f"Allowed numbers: {len(allowed)}", flush=True)
    
    adj = {}
    for u in allowed:
        adj[u] = set()
    
    memo_roots = {}
    def get_sqrt_neg1(p):
        if p in memo_roots: return memo_roots[p]
        if p == 2: return 1
        if p % 4 != 1: return None
        g = 2
        while pow(g, (p-1)//2, p) != p-1:
            g += 1
        res = pow(g, (p-1)//4, p)
        memo_roots[p] = res
        return res

    print("Building graph edges...", flush=True)
    count_edges = 0
    processed_nodes = 0
    checkpoint_build = max(1, len(allowed) // 10)
    
    for u in allowed:
        processed_nodes += 1
        if processed_nodes % checkpoint_build == 0:
             print(f"Building edges: {processed_nodes}/{len(allowed)} processed...", flush=True)

        roots = []
        if u == 1:
            roots = [0] 
        elif u == 2:
            roots = [1]
        else:
            temp = u
            if temp % 2 == 0: temp //= 2
            p = min_prime[temp]
            if p % 4 == 3: continue
            r = get_sqrt_neg1(p)
            if r is None: continue
            
            cur_r = r
            cur_mod = p
            target = temp
            while cur_mod < target:
                inv_deriv = pow(2*cur_r, -1, p)
                val = (cur_r*cur_r + 1) // cur_mod
                diff = (val * inv_deriv) % p
                cur_r = cur_r - diff * cur_mod
                cur_mod *= p
                cur_r %= cur_mod
                if cur_r < 0: cur_r += cur_mod
            
            if u % 2 == 0:
                if cur_r % 2 == 0: roots.append(cur_r + temp)
                else: roots.append(cur_r)
                if roots[0] * 2 != u: roots.append(u - roots[0])
            else:
                roots.append(cur_r)
                roots.append(u - cur_r)
        
        limit = int((u * N)**0.5)
        if u == 1:
            for x in range(limit + 1):
                v = x*x + 1
                if v > u and v in num_set:
                    adj[u].add(v); adj[v].add(u); count_edges += 1
        else:
            for r in roots:
                start = r
                if start == 0: start += u
                for x in range(start, limit + 1, u):
                    val = x*x + 1
                    v = val // u
                    if v > u and v in num_set:
                        adj[u].add(v); adj[v].add(u); count_edges += 1
                        
    print(f"Graph built. Nodes: {len(allowed)}, Edges: {count_edges}", flush=True)
    
    # 2. Prune to 2-Core
    print("Pruning to 2-Core...", flush=True)
    active_nodes = {u for u in adj if len(adj[u]) >= 2}
    current_adj = {u: set() for u in active_nodes}
    for u in active_nodes:
        for v in adj[u]:
            if v in active_nodes:
                current_adj[u].add(v)
    
    while True:
        to_remove = []
        for u in current_adj:
            if len(current_adj[u]) < 2: to_remove.append(u)
        if not to_remove: break
        for u in to_remove:
            for v in current_adj[u]:
                if v in current_adj: current_adj[v].remove(u)
            del current_adj[u]
            
    core_nodes = sorted(list(current_adj.keys()))
    print(f"2-Core Size: {len(core_nodes)}", flush=True)
    
    if not core_nodes:
        print("No cycles.")
        return 0

    # 3. Find Biconnected Components (Blocks)
    print("Finding Biconnected Components...", flush=True)
    visited = set()
    depth = {}
    low = {}
    parent = {}
    stack = []
    blocks = []
    
    def dfs_bcc(u, d, p):
        visited.add(u)
        depth[u] = d
        low[u] = d
        parent[u] = p
        children = 0
        
        for v in current_adj[u]:
            if v == p: continue
            if v in visited:
                low[u] = min(low[u], depth[v])
                if depth[v] < depth[u]:
                    stack.append((u, v))
            else:
                stack.append((u, v))
                children += 1
                dfs_bcc(v, d + 1, u)
                low[u] = min(low[u], low[v])
                if (p is not None and low[v] >= depth[u]) or (p is None and children > 1):
                    blk = set()
                    while stack:
                        edge = stack.pop()
                        blk.add(edge[0])
                        blk.add(edge[1])
                        if edge == (u, v): break
                    blocks.append(blk)
                    
    for u in core_nodes:
        if u not in visited:
            dfs_bcc(u, 0, None)
            if stack:
                blk = set()
                while stack:
                    edge = stack.pop()
                    blk.add(edge[0])
                    blk.add(edge[1])
                blocks.append(blk)
                
    print(f"Found {len(blocks)} blocks.", flush=True)
    block_sizes = [len(b) for b in blocks]
    print(f"Largest block size: {max(block_sizes) if block_sizes else 0}", flush=True)
    
    # 4. Process each block
    total_potency = 0
    
    blocks.sort(key=len)
    
    processed_count = 0
    for blk in blocks:
        if len(blk) < 3: continue 
        
        nodes = sorted(list(blk))
        sub_adj = {u: [] for u in nodes}
        node_map_blk = {u: i for i, u in enumerate(nodes)}
        
        for u in nodes:
            for v in current_adj[u]:
                if v in blk:
                    sub_adj[u].append(v)
        
        blk_potency = 0
        blk_cycles = 0
        path_vis = [False] * len(nodes)
        
        is_large = len(nodes) > 50
        if is_large:
            print(f"Processing large block size {len(nodes)}...", flush=True)
            
        def dfs_blk(start_idx, u_idx, current_sum, length):
            nonlocal blk_potency, blk_cycles
            path_vis[u_idx] = True
            u_val = nodes[u_idx]
            
            for v in sub_adj[u_val]:
                v_idx = node_map_blk[v]
                if v_idx == start_idx:
                    if length >= 3:
                        blk_potency += current_sum
                        blk_cycles += 1
                elif v_idx > start_idx:
                    if not path_vis[v_idx]:
                        dfs_blk(start_idx, v_idx, current_sum + nodes[v_idx], length + 1)
            path_vis[u_idx] = False
            
        for i in range(len(nodes)):
            dfs_blk(i, i, nodes[i], 1)
            
        total_potency += blk_potency // 2
        
        processed_count += 1
        if processed_count % 100 == 0:
            print(f"Blocks processed: {processed_count}/{len(blocks)}", flush=True)
            
    print(f"F(N) = {total_potency}", flush=True)
    print(f"Time: {time.time() - start_total:.2f}s", flush=True)
    return total_potency

if __name__ == "__main__":
    # Validation
    res_100 = solve_n(100)
    print(f"F(100) = {res_100} (Expected 538768)")
    solve_n(1000000)
