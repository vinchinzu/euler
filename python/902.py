"""
Project Euler Problem 902: Sum of Permutation Ranks

Given permutations sigma and tau on triangular numbers,
compute the sum of ranks of powers of pi = tau^(-1) * sigma * tau.

P(m) = sum of ranks of pi^k for k = 1 to m!
Answer requested: P(100)

Examples: P(2) = 4, P(3) = 780, P(4) = 38810300

Time Complexity: O(d * n^2) where d is order of permutation, n = m*(m+1)/2
Space Complexity: O(n)
"""

import math

MOD = 1000000007


def triangular(k):
    """Compute k-th triangular number: k*(k+1)/2"""
    return k * (k + 1) // 2


def build_sigma(m):
    """Build sigma permutation for given m"""
    n = triangular(m)
    sigma = list(range(1, n + 1))
    for k in range(1, m + 1):
        pos = triangular(k)
        sigma[pos - 1] = triangular(k - 1) + 1
    return sigma


def build_tau(m):
    """Build tau permutation using modular arithmetic"""
    n = triangular(m)
    a = 1000000007
    tau = [(a * i % n) + 1 for i in range(1, n + 1)]
    return tau


def build_tau_inv(tau):
    """Compute inverse of tau permutation"""
    n = len(tau)
    inv = [0] * (n + 1)
    for i in range(n):
        inv[tau[i]] = i + 1
    return inv[1:]


def compose(p1, p2):
    """Compose two permutations: (p1 ∘ p2)(x) = p1[p2[x]]"""
    return [p1[x - 1] for x in p2]


def rank_perm(perm):
    """Compute rank of permutation using Lehmer code (1-indexed)"""
    n = len(perm)
    fact = [1] * (n + 1)
    for i in range(1, n + 1):
        fact[i] = fact[i - 1] * i
    lehmer = [0] * n
    used = [False] * n
    for i in range(n):
        count = 0
        for j in range(perm[i] - 1):
            if not used[j]:
                count += 1
        lehmer[i] = count
        used[perm[i] - 1] = True
    rank = 0
    for i in range(n):
        rank += lehmer[i] * fact[n - 1 - i]
    return (rank % MOD + 1) % MOD


def compute_p(m):
    """
    Compute P(m) = sum of ranks of pi^k for k = 1 to m!

    Time Complexity: O(d * n^2) where d is order of pi, n = m*(m+1)/2
    """
    n = triangular(m)
    sigma = build_sigma(m)
    tau = build_tau(m)
    tau_inv = build_tau_inv(tau)
    temp = compose(sigma, tau)
    pi = compose(tau_inv, temp)

    # Find order of permutation and collect ranks
    identity = list(range(1, n + 1))
    current = pi[:]
    d = 1
    ranks = [rank_perm(current)]
    while current != identity:
        current = compose(current, pi)
        d += 1
        ranks.append(rank_perm(current))

    # Compute sum efficiently
    fact_m = math.factorial(m)
    q = fact_m // d
    r = fact_m % d
    sum_r = sum(ranks) % MOD
    sum_first_r = sum(ranks[:r]) % MOD
    total = (q * sum_r + sum_first_r) % MOD
    return total


def main():
    """Main entry point"""
    result = compute_p(100)
    print(result)
    return result


if __name__ == "__main__":
    main()
