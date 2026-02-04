def solve():
    N = 7 ** 10
    K = 7
    # tr(N-1) = (N-1)*N//2
    ans = (N - 1) * N // 2
    for d in range(1, K + 1):
        prev_k = 1
        t = 1
        while prev_k < N:
            k = 2 ** t
            if t > d:
                k += t + 1 - d - 2 ** (t - d)
            if k > N:
                k = N
            ans += (k - prev_k) * t
            prev_k = k
            t += 1
    print(ans)

if __name__ == "__main__":
    solve()
