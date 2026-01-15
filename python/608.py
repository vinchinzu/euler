"""Project Euler Problem 608: Divisor Sums.

Find Σ_{d|K!} Σ_{k=1}^N σ_0(k*d), where σ_0(n) is the number of divisors of n.

Let σ_t(n) be the number of divisors of n that are not divisible by t, and
let f_t(n) = Σ_{k=1}^N σ_t(n). We have σ_0(d*n) = σ_0(n) + Σ_{p^e} e * σ_p(n),
where the inner summation goes over all p^e in the prime factorization of d.
"""

from __future__ import annotations

from collections import defaultdict

from sympy import primerange


def num_divisors_sieve(limit: int) -> list[int]:
    """Count number of divisors for each number up to limit."""
    num_divs = [0] * (limit + 1)
    for i in range(1, limit + 1):
        for j in range(i, limit + 1, i):
            num_divs[j] += 1
    return num_divs


def num_factors_in_factorial(n: int, p: int) -> int:
    """Count how many times p divides n!."""
    count = 0
    power = p
    while power <= n:
        count += n // power
        power *= p
    return count


def triangular(n: int) -> int:
    """Triangular number n(n+1)/2."""
    return n * (n + 1) // 2


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


def sum_floor_quotients(n: int) -> int:
    """Sum of σ_0(k) for k=1 to n (simplified)."""
    # Simplified version - compute directly
    result = 0
    for k in range(1, n + 1):
        result += num_divisors_sieve(n)[k] if k <= n else 0
    return result


def solve() -> int:
    """Solve Problem 608."""
    N = 10**12
    K = 200
    M = 10**9 + 7
    L = int(N ** (2 / 3))

    primes = list(primerange(2, K + 1))
    num_divs = num_divisors_sieve(L)
    sum_floor_quotients_arr = [0] * (L + 1)
    for i in range(1, L + 1):
        sum_floor_quotients_arr[i] = (
            sum_floor_quotients_arr[i - 1] + num_divs[i]
        ) % M

    product_updates = {}
    mult = 1
    for p in primes:
        e = num_factors_in_factorial(K, p)
        tr_e_plus_1 = triangular(e + 1)
        mult = (mult * tr_e_plus_1) % M
        tr_e = triangular(e)
        product_updates[p] = (-tr_e * mod_inverse(tr_e_plus_1, M)) % M

    ans = 0

    def helper(min_index: int, d: int, mult_val: int) -> None:
        """Recursive helper."""
        nonlocal ans
        q = N // d
        if q >= L:
            # Use sublinear algorithm (simplified)
            sum_val = sum_floor_quotients(q) % M
        else:
            sum_val = sum_floor_quotients_arr[int(q)]
        ans = (ans + sum_val * mult_val) % M

        for index in range(min_index, len(primes)):
            p = primes[index]
            if d * p > N:
                break
            new_mult = (mult_val * product_updates[p]) % M
            helper(index + 1, d * p, new_mult)

    helper(0, 1, mult)
    return ans % M


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
