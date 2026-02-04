from sympy import primerange
import sys

sys.setrecursionlimit(10000)

def solve():
    N_target = 10000
    primes = list(primerange(2, 101))  # primes up to 100

    best = float('inf')

    def helper(index, n, sum_e, es):
        nonlocal best
        # Compute g(n) using DP: number of ways to pick sum_e//2 from sum_e total
        # where each prime factor contributes at most es[i] to the count
        half = sum_e // 2
        dp = [[0] * (half + 1) for _ in range(len(es) + 1)]
        dp[0][0] = 1
        for i in range(len(es)):
            for j in range(half + 1):
                for k in range(min(j, es[i]) + 1):
                    dp[i + 1][j] += dp[i][j - k]
        g = dp[len(es)][half]
        if g > N_target and n < best:
            best = n

        p = primes[index]
        e = 1
        while True:
            if len(es) > 0 and e > es[-1]:
                break
            # Check overflow: n * p^e
            power = p ** e
            if n * power >= 10**18:  # rough overflow check
                break
            es.append(e)
            helper(index + 1, n * power, sum_e + e, es)
            es.pop()
            e += 1

    helper(0, 1, 0, [])
    print(best)

if __name__ == "__main__":
    solve()
