"""Project Euler Problem 326.

a_n is defined by a_1=1, a_n = sum_{k=1}^{n-1} k*a_k mod n.
The sequence has closed form:
  a_{6i}   = 3i
  a_{6i+1} = 4i+1
  a_{6i+2} = 3i+1
  a_{6i+3} = i
  a_{6i+4} = 6i+3
  a_{6i+5} = i

Sum of first 6M terms (a_0..a_{6M-1}) is 0 mod M, so prefix sums S(n) mod M
are periodic with period 6M.
Count pairs (p,q) with 1<=p<=q<=N where sum_{i=p}^{q} a_i = 0 mod M.
"""

def solve():
    N = 10**12
    M = 10**6

    # Build a[0..6M-1] using closed form
    # a[k] = a_k in problem terms, with a_0 = 0
    a = [0] * (6 * M)
    for i in range(M):
        a[6 * i] = 3 * i
        a[6 * i + 1] = 4 * i + 1
        a[6 * i + 2] = 3 * i + 1
        a[6 * i + 3] = i
        a[6 * i + 4] = 6 * i + 3
        a[6 * i + 5] = i

    period = 6 * M

    # Compute prefix sums S(i) = sum_{k=0}^{i-1} a[k] = sum_{k=1}^{i} a_k
    # S(i) mod M is periodic with period 6M
    # After processing index i in the loop, sum = S(i) = a[0]+...+a[i-1]+a[i]
    # Wait: sum starts at 0, adds a[i], so after i: sum = a[0]+a[1]+...+a[i]
    # Since a[0]=0, sum = a[1]+...+a[i] = S(i) where S(i) = sum_{k=1}^{i} a_k
    # and S(0) = 0.
    # Index i in [0, 6M) represents S(i).
    # S(i) repeats at: i, i+6M, i+12M, ... up to N
    # Count of such indices <= N: (N - i) // (6M) + 1 if i <= N

    counts = [0] * M
    s = 0
    for i in range(period):
        s = (s + a[i]) % M
        # s = S(i) now
        # This S value appears at indices i, i+period, i+2*period, ... up to N
        # Number of occurrences: (N - i + period) // period  [Java formula]
        counts[s] += (N - i + period) // period

    ans = 0
    for c in counts:
        ans += c * (c - 1) // 2

    return ans

if __name__ == "__main__":
    print(solve())
