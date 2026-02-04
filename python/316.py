"""Project Euler Problem 316 - Numbers in decimal expansions.

Find sum_{n=2}^{999999} g(floor(10^16/n)), where g(s) is the expected
position of the first occurrence of s's digits in a random decimal sequence.

g(s) = sum of 10^i for all i >= 0 where prefix of length i equals suffix of
length i, minus len(s).
"""

def solve():
    N = 10**16
    K = 999999
    B = 10
    total = 0
    for n in range(2, K + 1):
        s = str(N // n)
        L = len(s)
        for i in range(L + 1):
            if s[:i] == s[L - i:]:
                total += B ** i
        total -= L
    return total

if __name__ == "__main__":
    print(solve())
