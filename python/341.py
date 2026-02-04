"""Project Euler Problem 341 - Golomb's self-describing sequence.

Find sum_{n=1}^{10^6} G(n^3).

Build Golomb's sequence iteratively up to N^{6/5}, then use
prefix sums to locate G(n^3) values efficiently.
"""

def solve():
    N = 10**6
    L = int(N ** 1.2)

    # Build Golomb's sequence
    G = [0] * (2 * L)
    size_G = 1
    G[1] = 1
    k = 1
    while size_G < L:
        G[size_G] = k
        size_G += 1
        for t in range(1, G[k]):
            if size_G >= L:
                break
            G[size_G] = k
            size_G += 1
        k += 1

    # Now compute answer using prefix sums
    sum_G = 0
    sum_KG = 0
    ans = 0
    n = 1
    for k in range(1, size_G):
        sum_G += G[k]
        sum_KG += k * G[k]
        while n < N and n * n * n <= sum_KG:
            ans += sum_G - (sum_KG - n * n * n) // k
            n += 1
        if n >= N:
            break

    return ans

if __name__ == "__main__":
    print(solve())
