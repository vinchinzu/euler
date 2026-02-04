"""Project Euler Problem 527: Randomized Binary Search.

R(n) = 2*(n+1)/n * H(n) - 3, where H(n) is the n-th harmonic number.
B(n) is computed via recursion (O(log n) depth since binary search halves each time).
H(n) for large n uses the Euler-Maclaurin asymptotic expansion.
"""

from mpmath import mp, mpf, log, euler, bernoulli


def harmonic_large(n):
    """Compute H(n) using Euler-Maclaurin asymptotic expansion."""
    mp.dps = 30
    n = mpf(n)
    gamma = euler
    Hn = log(n) + gamma + 1 / (2 * n)
    for k in range(1, 20):
        Hn -= bernoulli(2 * k) / (2 * k * n ** (2 * k))
    return Hn


def B(n, cache={}):
    """Expected guesses for standard binary search on range of size n."""
    if n in cache:
        return cache[n]
    if n <= 1:
        return mpf(1)
    mid = (n + 1) // 2
    left = mid - 1
    right = n - mid
    res = mpf(1) + (mpf(left) * B(left) + mpf(right) * B(right)) / mpf(n)
    cache[n] = res
    return res


def solve():
    mp.dps = 30
    N = 10 ** 10
    Hn = harmonic_large(N)
    R_val = 2 * mpf(N + 1) / mpf(N) * Hn - 3
    B_val = B(N)
    ans = R_val - B_val
    return f"{float(ans):.8f}"


if __name__ == "__main__":
    print(solve())
