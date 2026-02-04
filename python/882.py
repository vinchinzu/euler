import math

def solve():
    N = 100000
    G = [0.0] * (N + 1)
    total = 0.0
    for i in range(1, N + 1):
        low = 0.0
        high = float('inf')
        j = 0
        while (1 << j) <= i:
            remaining = (i >> (j + 1) << j) + i % (1 << j)
            if (i & (1 << j)) > 0:
                if G[remaining] > low:
                    low = G[remaining]
            else:
                if G[remaining] < high:
                    high = G[remaining]
            j += 1
        d = 1.0
        G[i] = 0.0
        while G[i] <= low or G[i] >= high:
            G[i] = math.floor(low / d + 1) * d
            d /= 2
        total += i * G[i]
    print(math.ceil(total))

if __name__ == "__main__":
    solve()
