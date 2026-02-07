#!/usr/bin/env python3
"""Project Euler Problem 339 - Peredur fab Efrawg.

Uses tridiagonal system approach from the Java reference.
Memory: O(N) instead of O(N^2).
"""


def tridiagonal_solve(a, b, c, d):
    """Solve tridiagonal system using Thomas algorithm.

    System: a[i]*x[i-1] + b[i]*x[i] + c[i]*x[i+1] = d[i]
    """
    n = len(d)
    if n == 0:
        return []
    c_ = [0.0] * n
    d_ = [0.0] * n
    c_[0] = c[0] / b[0]
    d_[0] = d[0] / b[0]
    for i in range(1, n):
        m = b[i] - a[i] * c_[i - 1]
        c_[i] = c[i] / m
        d_[i] = (d[i] - a[i] * d_[i - 1]) / m
    x = [0.0] * n
    x[n - 1] = d_[n - 1]
    for i in range(n - 2, -1, -1):
        x[i] = d_[i] - c_[i] * x[i + 1]
    return x


def solve(N=10000):
    e = [0.0] * (2 * N + 1)
    e[1] = 1.0
    e[2] = 1.0
    for k in range(3, 2 * N + 1):
        n = (k - 1) // 2
        a = [0.0] * n
        b = [0.0] * n
        c = [0.0] * n
        d = [0.0] * n
        for i in range(n):
            a[i] = (i + 1.0) / k - 1.0
            b[i] = 1.0
            c[i] = -(i + 1.0) / k
        d[0] = 1.0 - 1.0 / k
        x = tridiagonal_solve(a, b, c, d)
        e[k] = x[n - 1] * k + (1.0 - x[n - 1]) * e[(k // 2) * 2 - 1]
    ans = (e[2 * N] + e[2 * N - 3]) / 2.0
    return f"{ans:.6f}"


if __name__ == "__main__":
    print(solve())
