"""Project Euler Problem 327 - Rooms of Doom.

M(C, R) = minimum cards needed to traverse R rooms with capacity C.
Find sum_{C=3}^{40} M(C, 30).

If R < C: M = R + 1
Otherwise: M(C, R) = k + (k-2)//(C-2)*2 + 1 where k = M(C, R-1)
"""

def solve():
    N = 40
    K = 30

    def M(C, R):
        if R < C:
            return R + 1
        k = M(C, R - 1)
        return k + (k - 2) // (C - 2) * 2 + 1

    ans = 0
    for C in range(3, N + 1):
        ans += M(C, K)
    return ans

if __name__ == "__main__":
    print(solve())
