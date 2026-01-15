"""Project Euler Problem 657: Incomplete Words.

Find the number of words of length ≤ N that contain some but not all of K
letters.

We use Inclusion-Exclusion on the number of distinct letters in a word. For
any t < K, there are nCr(K, t) ways to choose t letters, and the number of
words with exactly those t letters is 1 + t + t² + t³ + ... + t^N. This is
usually (t^{N+1} - 1) / (t-1), but is equal to 1 when t=0 and equal to N+1
when t=1. The coefficient of the Inclusion-Exclusion is (-1)^(K-t).
"""

from __future__ import annotations


def mod_inverse(a: int, m: int) -> int:
    """Modular inverse using extended Euclidean algorithm."""
    if m == 1:
        return 0
    t, new_t = 0, 1
    r, new_r = m, a % m
    while new_r != 0:
        q = r // new_r
        t, new_t = new_t, t - q * new_t
        r, new_r = new_r, r - q * new_r
    if r != 1:
        raise ValueError("Modular inverse does not exist")
    if t < 0:
        t += m
    return t


def mod_invs(n: int, m: int) -> list[int]:
    """Precompute modular inverses for 1..n modulo m."""
    invs = [0] * (n + 1)
    invs[1] = 1
    for i in range(2, n + 1):
        invs[i] = (m - (m // i) * invs[m % i] % m) % m
    return invs


def pow_mod(base: int, exp: int, mod: int) -> int:
    """Modular exponentiation."""
    result = 1
    base %= mod
    while exp > 0:
        if exp & 1:
            result = (result * base) % mod
        base = (base * base) % mod
        exp >>= 1
    return result


def nth_pows(n: int, exp: int, mod: int) -> list[int]:
    """Compute i^exp for i from 0 to n modulo mod."""
    pows = [0] * (n + 1)
    pows[0] = 0 if exp > 0 else 1
    for i in range(1, n + 1):
        pows[i] = pow_mod(i, exp, mod)
    return pows


def parity(n: int) -> int:
    """Return (-1)^n."""
    return 1 if n % 2 == 0 else -1


def solve() -> int:
    """Solve Problem 657."""
    N = 10**12
    K = 10**7
    M = 10**9 + 7

    pows = nth_pows(K, N + 1, M)
    mod_invs_list = mod_invs(K, M)

    ans = 0
    num_choices = 1
    for t in range(K):
        if t == 0:
            num_words = 1
        elif t == 1:
            num_words = (N + 1) % M
        else:
            num_words = ((pows[t] - 1) * mod_invs_list[t - 1]) % M

        ans = (
            ans - parity(K - t) * num_words % M * num_choices % M
        ) % M
        if t < K - 1:
            num_choices = (
                num_choices * (K - t) % M * mod_invs_list[t + 1] % M
            )

    return ans % M


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
