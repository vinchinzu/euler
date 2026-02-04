
import sys
import heapq

# Set recursion limit just in case
sys.setrecursionlimit(20000)

def solve(N):
    MOD = 998388889
    s1 = 102022661
    
    # Generate s sequence
    # s[1] ... s[2N]
    # We use a list. 2*10^7 ints is ~160MB.
    # We need to optimize the loop slightly to run fast in Python.
    
    # Pre-allocate
    s = [0] * (2 * N + 1)
    s[1] = s1
    
    # Unrolling or using local vars for speed
    curr = s1
    for i in range(2, 2 * N + 1):
        curr = (curr * curr) % MOD
        s[i] = curr

    a = [0] * (N + 1)
    b = [0] * (N + 1)
    
    # Fill a and b
    # a[n] = s[2n-1], b[n] = s[2n]
    for i in range(1, N + 1):
        a[i] = s[2 * i - 1]
        b[i] = s[2 * i]

    # Release s to free memory
    del s

    # Prefix sums
    pa = [0] * (N + 1)
    pb = [0] * (N + 1)
    curr_pa = 0
    curr_pb = 0
    for i in range(1, N + 1):
        curr_pa += a[i]
        curr_pb += b[i]
        pa[i] = curr_pa
        pb[i] = curr_pb

    def get_interesting_indices(arr, N, K):
        indices = {1, N}

        # Prefix Minima
        curr_min = float('inf')
        for i in range(1, N + 1):
            val = arr[i]
            if val < curr_min:
                curr_min = val
                indices.add(i)

        # Suffix Minima
        curr_min = float('inf')
        for i in range(N, 0, -1):
            val = arr[i]
            if val < curr_min:
                curr_min = val
                indices.add(i)

        # Top K smallest values
        # Optimization: Filter by threshold first
        # Expected value is 5*10^8. Min is ~0.
        # We want approx K values.
        # Uniform distribution. Threshold T = K * MOD / N * Factor
        # T = 2000 * 10^9 / 10^7 * 2 = 200 * 1000 = 200,000
        # Let's use T = 1,000,000 to be safe.
        threshold = 1000000
        candidates = [i for i, x in enumerate(arr) if i > 0 and x < threshold]

        # If candidates fewer than K, we might miss some, but unlikely given distribution.
        # If too many, take K smallest.
        if len(candidates) > K:
            # Take K smallest
            # Using heapq.nsmallest
            candidates = heapq.nsmallest(K, candidates, key=lambda i: arr[i])

        indices.update(candidates)

        return sorted(list(indices))

    # K=2000 is generous enough
    K = 2000
    rows = get_interesting_indices(a, N, K)
    cols = get_interesting_indices(b, N, K)

    NR = len(rows)
    NC = len(cols)

    # DP on compressed grid
    # dp[r][c] is min cost to reach cell (rows[r], cols[c])

    # Initialize DP table
    # Only store current row and next row to save memory?
    # Actually NR=2500. Table size 6.25M. 6M doubles is 48MB. Fine.
    dp = [[float('inf')] * NC for _ in range(NR)]

    # Start at (1,1) which corresponds to (rows[0], cols[0])
    dp[0][0] = a[1] + b[1]

    # Iterate
    for r in range(NR):
        real_r = rows[r]
        val_a_r = a[real_r]

        # Precompute next_r logic
        has_down = (r + 1 < NR)
        if has_down:
            next_r = rows[r+1]
            diff_r = next_r - real_r
            cost_a_segment = pa[next_r] - pa[real_r]

        for c in range(NC):
            curr = dp[r][c]
            if curr == float('inf'):
                continue

            real_c = cols[c]

            # Move Right
            if c + 1 < NC:
                next_c = cols[c+1]
                # Cost to move from (real_r, real_c) to (real_r, next_c)
                # path: (real_r, real_c) -> (real_r, real_c+1) ... -> (real_r, next_c)
                # Cells added: (real_r, k) for k in real_c+1 ... next_c
                # Sum (a[real_r] + b[k])
                cost_move = (next_c - real_c) * val_a_r + (pb[next_c] - pb[real_c])
                if curr + cost_move < dp[r][c+1]:
                    dp[r][c+1] = curr + cost_move

            # Move Down
            if has_down:
                # Cost to move from (real_r, real_c) to (next_r, real_c)
                # path: (real_r, real_c) -> (real_r+1, real_c) ... -> (next_r, real_c)
                # Cells added: (k, real_c) for k in real_r+1 ... next_r
                # Sum (a[k] + b[real_c])
                cost_move = cost_a_segment + diff_r * b[real_c]
                if curr + cost_move < dp[r+1][c]:
                    dp[r+1][c] = curr + cost_move

    return int(dp[NR-1][NC-1])

if __name__ == "__main__":
    result = solve(10**7)
    print(result)
