"""Project Euler Problem 340 - Crazy Function.

F(n) = n - c for n > b
F(n) = F(a + F(a + F(a + F(a + n)))) for n <= b

Find sum_{n=0}^{b} F(n) mod 10^9 with a=21^7, b=7^21, c=12^7.

Closed form:
F(n) = n + 4(a-c) + floor((b-n)/a) * (4a - 3c)

Sum = tr(b) + (b+1)*4*(a-c) + tr(b//a)*(b%a+1)*(4a-3c) + tr(b//a-1)*(a-b%a-1)*(4a-3c)
where tr(n) = n*(n+1)/2
"""

def solve():
    A = 21**7
    B = 7**21
    C = 12**7
    M = 10**9

    def tr(n):
        """Triangular number n*(n+1)/2 mod M."""
        # n*(n+1) is always even, so we can divide before taking mod
        if n % 2 == 0:
            return (n // 2 % M) * ((n + 1) % M) % M
        else:
            return (n % M) * ((n + 1) // 2 % M) % M

    ans = 0
    ans += tr(B)
    ans += (B + 1) % M * (4 % M) % M * ((A - C) % M) % M
    ans += tr(B // A) * ((B % A + 1) % M) % M * ((4 * A - 3 * C) % M) % M
    ans += tr(B // A - 1) * ((A - B % A - 1) % M) % M * ((4 * A - 3 * C) % M) % M
    ans %= M

    return ans

if __name__ == "__main__":
    print(solve())
