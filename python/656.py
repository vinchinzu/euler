"""Project Euler Problem 656: Palindromic sequences.

Define S_alpha(n) = floor(alpha * n) - floor(alpha * (n-1)), and let H_g(alpha) be the sum of the first
g values of n such that S_alpha(1), S_alpha(2), ... S_alpha(n) is a palindrome. Find
sum_beta H_100(sqrt(beta)) for all non-squares beta <= N.

According to https://www.fq.math.ca/Scanned/39-1/komatsu.pdf, the sequence
S_alpha(1) ... S_alpha(n) is a palindrome for n in { 1,2,...q1, q2+q1,2*q2+q1,...q3,
q4+q3,2*q4+q3,...q5, ... }, where q_k are the denominators in the continued
fraction approximants for alpha.
"""

from __future__ import annotations

from math import isqrt


def is_square(n: int) -> bool:
    """Check if n is a perfect square."""
    root = isqrt(n)
    return root * root == n


def cf_sqrt(D: int):
    """Generate continued fraction coefficients for sqrt(D).

    Yields a0, a1, a2, ... (the periodic part repeats).
    """
    a0 = isqrt(D)
    yield a0

    if a0 * a0 == D:
        return

    P = a0
    Q = D - a0 * a0

    while True:
        a = (a0 + P) // Q
        yield a
        P = a * Q - P
        Q = (D - P * P) // Q
        if Q == 0:
            break


def H(beta: int, K: int, M: int) -> int:
    """Compute H_K(sqrt(beta))."""
    qs = [1, 0]
    ns = [0]

    for a in cf_sqrt(beta):
        if len(qs) % 2 == 1:
            for _ in range(a):
                ns.append((ns[-1] + qs[-1]) % M)
                if len(ns) > K:
                    break
        qs.append((qs[-1] * a + qs[-2]) % M)
        if len(ns) > K:
            break

    result = 0
    for i in range(1, min(K + 1, len(ns))):
        result = (result + ns[i]) % M
    return result


def solve() -> int:
    """Solve Problem 656."""
    N = 1000
    K = 100
    M = 10**15

    ans = 0
    for beta in range(1, N + 1):
        if is_square(beta):
            continue
        ans = (ans + H(beta, K, M)) % M

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
