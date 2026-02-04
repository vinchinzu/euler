"""Project Euler Problem 335 - Gathering the beans.

M(2^k + 1) = 4^k + 2*2^k - 3^k

Find sum_{k=0}^{10^18} M(2^k + 1) mod 7^9.

Sum of geometric series:
sum 4^k = (4^{N+1} - 1)/3
sum 2*2^k = 2*(2^{N+1} - 1)
sum 3^k = (3^{N+1} - 1)/2
"""

def solve():
    N = 10**18
    M = 7**9

    # sum_{k=0}^{N} 4^k = (4^{N+1} - 1) / 3
    # Compute in mod 3*M to get exact division by 3
    s1 = (pow(4, N + 1, 3 * M) - 1) // 3

    # sum_{k=0}^{N} 2*2^k = 2*(2^{N+1} - 1)
    s2 = 2 * (pow(2, N + 1, M) - 1)

    # sum_{k=0}^{N} 3^k = (3^{N+1} - 1) / 2
    # Compute in mod 2*M to get exact division by 2
    s3 = (pow(3, N + 1, 2 * M) - 1) // 2

    ans = (s1 + s2 - s3) % M
    return ans

if __name__ == "__main__":
    print(solve())
