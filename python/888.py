import array
from collections import Counter

def solve():
    N_val = 12491249
    K = 1249
    Ds = [1, 2, 4, 9]
    M = 912491249
    L = 25000

    # Compute nimbers - use array for speed
    nimbers = array.array('i', [0] * L)
    for n in range(L):
        used = set()
        for d in Ds:
            if d <= n:
                used.add(nimbers[n - d])
        # Split: pile n into (i, n-i) for i=1..n-1
        for i in range(1, (n + 1) // 2 + 1):
            v = nimbers[i] ^ nimbers[n - i]
            used.add(v)
        nimber = 0
        while nimber in used:
            nimber += 1
        nimbers[n] = nimber

    # Find period and cap
    max_nimber = max(nimbers)
    cap = 1
    while cap < max_nimber:
        cap *= 2

    # Find period: look for where the second half starts repeating
    half = L // 2
    sub = nimbers[half:L]
    nimbers_list = list(nimbers)
    # Use a smarter search
    index = 0
    for start in range(half):
        match = True
        for j in range(half):
            if nimbers_list[start + j] != nimbers_list[half + j]:
                match = False
                break
        if match:
            index = start
            break
    period = half - index

    # Compute counts
    counts = [0] * cap
    for n in range(1, period):
        counts[nimbers_list[n]] += 1
    for n in range(period):
        counts[nimbers_list[n + period]] += (N_val - n) // period

    # Precompute factorials
    max_val = max(counts) + K + 10
    fact = [1] * (max_val + 1)
    for i in range(1, max_val + 1):
        fact[i] = fact[i - 1] * i % M
    inv_fact = [1] * (max_val + 1)
    inv_fact[max_val] = pow(fact[max_val], M - 2, M)
    for i in range(max_val - 1, -1, -1):
        inv_fact[i] = inv_fact[i + 1] * (i + 1) % M

    def nCr(n, r):
        if r < 0 or r > n:
            return 0
        return fact[n] * inv_fact[r] % M * inv_fact[n - r] % M

    # DP: dp[nimber][xor_val][num_piles]
    # Process one nimber at a time
    prev_dp = [[0] * (K + 1) for _ in range(cap)]
    prev_dp[0][0] = 1

    for nimber in range(cap):
        next_dp = [[0] * (K + 1) for _ in range(cap)]
        for d in range(K + 1):
            mult = 1 if d == 0 else nCr(counts[nimber] + d - 1, d)
            if mult == 0:
                continue
            for n in range(cap):
                if d % 2 == 0:
                    new_n = n
                else:
                    new_n = n ^ nimber
                row = prev_dp[n]
                dst = next_dp[new_n]
                for k in range(K + 1 - d):
                    if row[k]:
                        dst[k + d] = (dst[k + d] + mult * row[k]) % M
        prev_dp = next_dp

    print(prev_dp[0][K])

if __name__ == "__main__":
    solve()
