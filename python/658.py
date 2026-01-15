"""Project Euler Problem 658: Incomplete Words II.

Find the number of words of length ≤ N that contain some but not all of k
letters, summed over all 1 ≤ k ≤ K.

For a given k, we have from p657 that the number of words is
I(k) = sum_{t=0}^k (-1)^(k-t) nCr(k, t) f(t),
where f(t) is 1 if t=0, N+1 if t=1, and (t^{N+1} - 1) / (t-1) otherwise.
We can sum this over all k and swap the summations.
"""

from __future__ import annotations

from sympy import isprime, primerange


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


def preff(n: int) -> list[int]:
    """Precompute smallest prime factor."""
    ff = list(range(n + 1))
    for i in range(2, int(n**0.5) + 1):
        if ff[i] == i:
            for j in range(i * i, n + 1, i):
                if ff[j] == j:
                    ff[j] = i
    return ff


def parity(n: int) -> int:
    """Return (-1)^n."""
    return 1 if n % 2 == 0 else -1


def solve() -> int:
    """Solve Problem 658."""
    N = 10**12
    K = 10**7
    M = 10**9 + 7

    ff = preff(K)
    pows = [0] * K
    for i in range(1, K):
        if i == 1 or isprime(i):
            pows[i] = pow_mod(i, N + 1, M)
        else:
            pows[i] = (pows[ff[i]] * pows[i // ff[i]]) % M

    mod_invs_list = mod_invs(K, M)

    ans = 0
    nCr = 1
    inner_sum = K % 2

    for t in range(K):
        if t == 0:
            num_words = 1
        elif t == 1:
            num_words = (N + 1) % M
        else:
            num_words = ((pows[t] - 1) * mod_invs_list[t - 1]) % M

        ans = (ans + num_words * inner_sum) % M

        if t < K - 1:
            new_nCr = nCr * (K - t) % M * mod_invs_list[t + 1] % M
            inner_sum = (
                (inner_sum + 1 + parity(K - t) * (nCr + new_nCr))
                * mod_inverse(2, M)
                % M
            )
            nCr = new_nCr

    return ans % M


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
