"""Project Euler Problem 585: Nested square roots.

Find the number of distinct values equal to the sum and/or difference of
a finite number of square roots of integers, which can be written in the
form √(x + √y + √z) for positive integers x,y,z, 1≤x≤N, and y,z are
not perfect squares.

There are two possibilities:
1. Values of the form √a + √b, where a<b and a*b is not a perfect square
2. Values of the form √a + √b + √c - √d, where a<b<c<d, ad=bc, etc.
"""

from __future__ import annotations

from math import gcd, isqrt

from sympy import primerange


def is_square(n: int) -> bool:
    """Check if n is a perfect square."""
    if n < 0:
        return False
    root = isqrt(n)
    return root * root == n


def sieve_with_smallest_factor(limit: int) -> list[int]:
    """Sieve that stores smallest prime factor."""
    ff = [0] * (limit + 1)
    for i in range(2, limit + 1):
        if ff[i] == 0:
            ff[i] = i
            for j in range(i * i, limit + 1, i):
                if ff[j] == 0:
                    ff[j] = i
    return ff


def euler_totient(n: int, ff: list[int]) -> int:
    """Compute Euler's totient using smallest factor array."""
    phi = n
    temp = n
    while temp > 1:
        p = ff[temp]
        phi = phi // p * (p - 1)
        while temp % p == 0:
            temp //= p
    return phi


def is_square_free(n: int, ff: list[int]) -> bool:
    """Check if n is square-free."""
    temp = n
    while temp > 1:
        p = ff[temp]
        count = 0
        while temp % p == 0:
            temp //= p
            count += 1
        if count >= 2:
            return False
    return True


def multiplicative_function(limit: int, f) -> list[int]:
    """Compute multiplicative function."""
    ff = sieve_with_smallest_factor(limit)
    result = [0] * (limit + 1)
    result[1] = 1

    for i in range(2, limit + 1):
        if ff[i] == i:  # i is prime
            result[i] = f(i, 1)
        else:
            p = ff[i]
            e = 0
            temp = i
            while temp % p == 0:
                temp //= p
                e += 1
            result[i] = result[temp] * f(p, e)

    return result


def solve() -> int:
    """Solve Problem 585."""
    N = 5_000_000

    # Precompute utilities
    ff = sieve_with_smallest_factor(N)
    phi = [0] * (N + 1)
    for i in range(1, N + 1):
        phi[i] = euler_totient(i, ff)

    # Compute f(n) and f'(n)
    f = [0] * (N + 1)
    fp = [0] * (N + 1)
    for n in range(1, N + 1):
        f[n] = (n - 1) // 2
        fp[n] = phi[n] // 2

    # Subtract cases where a*b is a perfect square
    for k in range(1, N + 1):
        if not is_square_free(k, ff):
            continue
        s = 1
        while s * s * k <= N:
            t = s + 1
            while (s * s + t * t) * k <= N:
                sum_val = (s * s + t * t) * k
                f[sum_val] -= 1
                if k == 1 and gcd(s, t) == 1:
                    fp[sum_val] -= 1
                t += 1
            s += 1

    # First case: sum of f[n]
    ans = sum(f[1 : N + 1])

    # Second case: compute res
    res = 0
    for g_plus_h in range(1, N + 1):
        for ap_plus_bp in range(1, N + 1):
            if g_plus_h * ap_plus_bp > N:
                break
            res += f[g_plus_h] * fp[ap_plus_bp]

    # Subtract cases where g*h*a'*b' is a perfect square
    sizes = multiplicative_function(N, lambda p, e: 2 * e + 1)
    start_indices = [0] * (N + 2)
    for i in range(1, N + 1):
        start_indices[i + 1] = start_indices[i] + (sizes[i] + 1) // 2

    curr_indices = start_indices[:]
    total_size = start_indices[N + 1]
    smalls = [0] * total_size
    bigs = [0] * total_size

    for k in range(1, N + 1):
        if not is_square_free(k, ff):
            continue
        s = 1
        while s * s * k <= N:
            t = s
            while (s * s + t * t) * k <= N:
                idx = s * t * k
                if idx < len(curr_indices):
                    smalls[curr_indices[idx]] = s * s * k
                    bigs[curr_indices[idx]] = t * t * k
                    curr_indices[idx] += 1
                t += 1
            s += 1

    for i in range(N):
        for ad in range(start_indices[i], curr_indices[i]):
            for bc in range(start_indices[i], curr_indices[i]):
                a = smalls[ad]
                b = smalls[bc]
                c = bigs[bc]
                d = bigs[ad]
                if a < b and a + b + c + d <= N and not is_square(a * b):
                    res -= 2 if b != c else 1

    ans += res // 2
    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
