"""Project Euler Problem 344 - Silver Dollar Game.

Count winning configurations in a 1xN board with C coins and 1 silver dollar.
Uses Nim theory with XOR-zero DP and Chinese Remainder Theorem.
"""

def solve():
    N = 1000000
    C = 100
    M1 = 1000003
    M2 = 1000033

    def W(n, c, mod):
        k = c - c // 2  # = c/2 + 1 for even c (51 for c=100)

        # Precompute nCr(i, j) for i <= k+1, j <= k
        # Using Zp-like modular arithmetic
        # First precompute factorials and inverse factorials
        max_val = max(n, k + 1) + 1
        # We need nCr(n, c+1) and nCr(big, k) etc.
        # Use modular arithmetic

        # Precompute factorial and inv_factorial up to n
        fact = [1] * (n + 1)
        for i in range(1, n + 1):
            fact[i] = fact[i - 1] * i % mod

        inv_fact = [1] * (n + 1)
        inv_fact[n] = pow(fact[n], mod - 2, mod)
        for i in range(n - 1, -1, -1):
            inv_fact[i] = inv_fact[i + 1] * (i + 1) % mod

        def nCr(a, b):
            if b < 0 or b > a:
                return 0
            return fact[a] * inv_fact[b] % mod * inv_fact[a - b] % mod

        # Small nCr table for XOR zero DP
        nCrs = [[0] * (k + 1) for _ in range(k + 2)]
        for i in range(k + 2):
            for j in range(k + 1):
                nCrs[i][j] = nCr(i, j)

        def num_xor_zero_sets(max_sum, num_piles):
            """Count ways that num_piles non-negative integers XOR to 0 and sum to each value."""
            xz = [0] * (max_sum + 1)
            xz[0] = 1
            for i in range(2, max_sum + 1, 2):
                for num_one_pairs in range(min(num_piles // 2, i // 2) + 1):
                    xz[i] = (xz[i] + xz[i // 2 - num_one_pairs] * nCrs[num_piles][2 * num_one_pairs]) % mod
            return xz

        xz = num_xor_zero_sets(n, c // 2 + 1)
        xz_minus_one = num_xor_zero_sets(n, c // 2)

        res = (c + 1) * nCr(n, c + 1) % mod
        for i in range(n - c):
            res = (res - xz[i] * nCr(n - c - 1 - i + k, k)) % mod

        num_losing_late = 0
        for i in range(n - c + 1):
            num_losing_late = (num_losing_late + xz[i] * nCr(n - c - i + k, k)) % mod
        for i in range(n - c + 1):
            num_losing_late = (num_losing_late - xz_minus_one[i] * nCr(n - c - i + k, k)) % mod

        return (res - (c - 1) * num_losing_late) % mod

    # CRT: combine results mod M1 and M2
    w1 = W(N, C, M1)
    w2 = W(N, C, M2)

    # CRT: find x such that x = w1 mod M1 and x = w2 mod M2
    M = M1 * M2
    x = (w1 * M2 % M * pow(M2, M1 - 2, M1) % M + w2 * M1 % M * pow(M1, M2 - 2, M2) % M) % M
    return x

if __name__ == "__main__":
    print(solve())
