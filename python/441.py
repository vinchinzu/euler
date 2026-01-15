"""Project Euler Problem 441: The inverse summation of coprime pairs.

Find Σ_{i=2}^N R(i), where R(i) is the sum of all 1/(p*q) for 1 ≤ p < q ≤
M, p+q ≥ M, and (p,q) = 1.

By induction, the sum is 1/2 (N - 3 + Σ_{g=1}^N μ(g) ( Σ_{i=1}^{⌊N/g⌋}
1/(g*i) )²).
"""

from __future__ import annotations


def mobius_sieve(n: int) -> list[int]:
    """Compute Mobius function for all numbers up to n."""
    mu = [1] * (n + 1)
    is_prime = [True] * (n + 1)
    is_prime[0] = is_prime[1] = False

    for i in range(2, n + 1):
        if is_prime[i]:
            for j in range(i, n + 1, i):
                is_prime[j] = False
                mu[j] *= -1
            for j in range(i * i, n + 1, i * i):
                mu[j] = 0

    return mu


def solve() -> float:
    """Solve Problem 441."""
    N = 10**7

    mu = mobius_sieve(N)
    harmonics = [0.0] * (N + 1)
    for i in range(1, N + 1):
        harmonics[i] = harmonics[i - 1] + 1.0 / i

    ans = 0.0
    for g in range(1, N + 1):
        if mu[g] != 0:
            h_val = harmonics[N // g] / g
            ans += mu[g] * h_val * h_val

    ans = (ans + N - 3) / 2
    return ans


def main() -> float:
    """Main entry point."""
    result = solve()
    print(f"{result:.4f}")
    return result


if __name__ == "__main__":
    main()
