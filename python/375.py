#!/usr/bin/env python3
"""
Project Euler Problem 375: Minimum of subsequences

Let S_n be the periodic sequence produced by the Blum Blum Shub RNG
(S_0 = 290797, S_n = S_{n-1}^2 mod 50515093), and let A(i, j) be the minimum
of S_i, ..., S_j. Find M(N) = sum_{1<=i<=j<=N} A(i, j) for N = 2*10^9.

Algorithm (from Java reference):
1. The BBS sequence is periodic. Find the period P.
2. For n = k*P + (N mod P), the function M(n) is a polynomial in k (degree 2
   since the stack-based computation is quadratic in n).
3. Compute M for 3 small multiples of P, then use Lagrange interpolation to
   extrapolate to N.

The stack algorithm: maintain a stack of nondecreasing values. When a new value
is smaller than the top, pop and compute the contribution of each popped element.
"""


def solve():
    N = 2 * 10**9
    MOD = 50515093

    # Generate BBS sequence and find period
    def bbs_generator():
        s = 290797
        while True:
            s = (s * s) % MOD
            yield s

    gen = bbs_generator()
    first = next(gen)
    period = 1
    for s in gen:
        if s == first:
            break
        period += 1

    start = N % period

    def compute_M(n):
        """Compute M(n) = sum_{1<=i<=j<=n} min(S_i, ..., S_j) using stack algorithm."""
        # Stack entries: (position, value)
        # We use a sentinel at position 0 with value -1
        stack_pos = [0]
        stack_val = [-1]
        M = 0

        s = 290797
        for pos in range(1, n + 1):
            s = (s * s) % MOD
            # Pop elements larger than current value
            while stack_val[-1] > s:
                v = stack_val.pop()
                p = stack_pos.pop()
                prev_p = stack_pos[-1]
                M += v * (p - prev_p) * (pos - p)
            stack_pos.append(pos)
            stack_val.append(s)

        # Process remaining stack (sentinel value -1 will catch everything)
        # Actually, process sentinel by treating end as pos = n+1 with value -infinity
        pos = n + 1
        while len(stack_pos) > 1:
            v = stack_val.pop()
            p = stack_pos.pop()
            prev_p = stack_pos[-1]
            M += v * (p - prev_p) * (pos - p)

        return M

    # Compute M at 3 points: start+period, start+2*period, start+3*period
    # These should fit a degree-2 polynomial in the multiplier
    points = []
    for mult in range(1, 4):
        n = start + mult * period
        points.append(compute_M(n))

    # Lagrange interpolation for degree 2 polynomial
    # We have f(1) = points[0], f(2) = points[1], f(3) = points[2]
    # and we want f(k) where k = N // period (since N = k*period + start, assuming start = N%period)
    # Actually: N = k*period + start where k = N // period
    k = N // period

    # f(x) through (1, y0), (2, y1), (3, y2):
    y0, y1, y2 = points
    # Lagrange basis:
    # L0(x) = (x-2)(x-3)/((1-2)(1-3)) = (x-2)(x-3)/2
    # L1(x) = (x-1)(x-3)/((2-1)(2-3)) = -(x-1)(x-3)
    # L2(x) = (x-1)(x-2)/((3-1)(3-2)) = (x-1)(x-2)/2
    # But these involve fractions. For exact integer arithmetic, multiply through.

    # Use the fact that for a degree-2 polynomial through integer points,
    # we can use the Newton forward difference formula.
    # d0 = y0
    # d1 = y1 - y0
    # d2 = y2 - 2*y1 + y0
    # f(x) = d0 + d1*(x-1) + d2*(x-1)*(x-2)/2

    d0 = y0
    d1 = y1 - y0
    d2 = y2 - 2 * y1 + y0

    # f(k) = d0 + d1*(k-1) + d2*(k-1)*(k-2)//2
    # d2*(k-1)*(k-2) must be even for integer result
    result = d0 + d1 * (k - 1) + d2 * (k - 1) * (k - 2) // 2
    return result


if __name__ == "__main__":
    print(solve())
