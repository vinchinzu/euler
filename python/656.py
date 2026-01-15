"""Project Euler Problem 656: Palindromic sequences.

Define S_α(n) = ⌊α * n⌋ - ⌊α * (n-1)⌋, and let H_g(α) be the sum of the first
g values of n such that S_α(1), S_α(2), ... S_α(n) is a palindrome. Find
sum_β H_100(√β) for all non-squares β ≤ N.

According to https://www.fq.math.ca/Scanned/39-1/komatsu.pdf, the sequence
S_α(1) ... S_α(n) is a palindrome for n ∈ { 1,2,...q1, q2+q1,2*q2+q1,...q3,
q4+q3,2*q4+q3,...q5, ... }, where q_k are the denominators in the continued
fraction approximants for α.
"""

from __future__ import annotations

from math import isqrt


def is_square(n: int) -> bool:
    """Check if n is a perfect square."""
    root = isqrt(n)
    return root * root == n


def pell_pqa(P0: int, Q0: int, D: int):
    """Generate continued fraction expansion for √D."""
    P = P0
    Q = Q0
    a0 = int((P0 + isqrt(D)) / Q0)
    yield (a0, P, Q)

    seen = set()
    while True:
        P = a0 * Q - P
        Q = (D - P * P) // Q
        a0 = int((P + isqrt(D)) / Q)
        state = (P, Q, a0)
        if state in seen:
            break
        seen.add(state)
        yield (a0, P, Q)


def solve() -> int:
    """Solve Problem 656."""
    N = 1000
    K = 100
    M = 10**15

    ans = 0
    for beta in range(1, N + 1):
        if is_square(beta):
            continue

        qs = [1, 0]
        ns = [0]

        for step in pell_pqa(0, beta, 1):
            a = step[0]
            if len(qs) % 2 == 1:
                for _ in range(a):
                    ns.append((ns[-1] + qs[-1]) % M)
            qs.append((qs[-1] * a + qs[-2]) % M)
            if len(ns) > K:
                break

        H = 0
        for i in range(1, K + 1):
            if i < len(ns):
                H = (H + ns[i]) % M
        ans = (ans + H) % M

    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
