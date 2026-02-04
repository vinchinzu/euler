"""Project Euler Problem 795: Alternating GCD Sum.

Find sum_{n=1}^N g(n), where g(n) = sum_{i=1}^n (-1)^i GCD(n,i^2).

First consider sum_{n=1}^N g'(n), where g'(n) = sum_{i=1}^n GCD(n,i^2). Then g'(n)
is multiplicative where g'(p^e) = sum_{i=1}^{p^e} GCD(p^e,i^2) can be computed
easily by noting that there are p^{e-k}-p^{e-k-1} occurrences of each p^{2k}
(capped at p^e).

For the original g(n), if n is even then for the factor corresponding to p=2,
the 2^{e-k}-2^{e-k-1} occurrences of 1 are negative instead. If n is odd,
then this doesn't work, but g(n) is easy to compute because except for the
last term, each (-1)^k GCD(k,i^2) cancels with (-1)^{n-k} GCD(n-k,i^2). That
means g(n) is equal to its last term, -n.
"""

from __future__ import annotations


def pre_ff(limit):
    """Precompute smallest prime factor."""
    ff = [0] * (limit + 1)
    for i in range(2, limit + 1):
        if ff[i] == 0:
            ff[i] = i
            for j in range(2 * i, limit + 1, i):
                if ff[j] == 0:
                    ff[j] = i
    return ff


def solve():
    """Solve Problem 795."""
    N = 12345678
    ff = pre_ff(N)

    f = [0] * (N + 1)
    for n in range(2, N + 1):
        nn = n
        p = ff[nn]
        e = 0
        while nn % p == 0:
            nn //= p
            e += 1

        if nn > 1:
            f[n] = f[nn] * f[n // nn]
        else:
            for k in range(e + 1):
                pe_k = p ** (e - k)
                count = pe_k - pe_k // p  # p^(e-k) - p^(e-k-1) using integer division
                sign = -1 if (p == 2 and k == 0) else 1
                power = p ** min(2 * k, e)
                f[n] += count * sign * power

    ans = 0
    for n in range(1, N + 1):
        if n % 2 == 0:
            ans += f[n]
        else:
            ans -= n

    return ans


def main():
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
