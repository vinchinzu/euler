def solve():
    N = 18
    B = 10
    M = 1123455689

    # Precompute factorials and inverse factorials mod M
    max_fact = N + 1
    fact = [1] * (max_fact + 1)
    for i in range(1, max_fact + 1):
        fact[i] = fact[i - 1] * i % M
    inv_fact = [1] * (max_fact + 1)
    inv_fact[max_fact] = pow(fact[max_fact], M - 2, M)
    for i in range(max_fact - 1, -1, -1):
        inv_fact[i] = inv_fact[i + 1] * (i + 1) % M

    ans = 0
    counts = [0] * B

    def gnCr(counts):
        total = sum(counts)
        result = fact[total]
        for c in counts:
            result = result * inv_fact[c] % M
        return result

    def helper(index, min_d, n):
        nonlocal ans
        if index == N:
            ans = (ans + n % M * (gnCr(counts) % M)) % M
            return
        for d in range(min_d, B):
            counts[d] += 1
            helper(index + 1, d, n * B + d)
            counts[d] -= 1

    helper(0, 0, 0)
    ans %= M
    print(ans)

if __name__ == "__main__":
    solve()
