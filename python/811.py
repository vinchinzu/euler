"""Project Euler Problem 811: Recursive Binary Function.

Define a recursive function by A(0)=1, A(2n)=3A(n)+5A(2n-b(n)), and
A(2n+1)=A(n), where b(n) is the largest power of 2 dividing n. Find
H(N,K) = A((2^N+1)^K).

Take A(2n) and consider 2n as a binary string. Then n is the binary
string without the final zero, and 2n-b(n) is the binary string with the
rightmost 1 shifted one more to the right. So we can show by induction
that if the number of zeros between adjacent 1s in n is a_0, a_1, ... a_k,
then A(n) is Π_i (b_i)^(a_i), where b_i = 5b_{i-1} + 3. That's because
we can either remove a zero (and multiply the subtotal by 3) or shift
the rightmost 1 (and multiply the subtotal by 5). The base case is b_0 = 1.

We can use this to compute A(n) for n of reasonable length. To compute
the answer, we note that at some point we are just adding zeros to the
same regions of consecutive zeros, so the answer obeys a simple recursive
function, and we can use standard extrapolation.
"""

from __future__ import annotations

from typing import Callable


def pow_mod(base: int, exp: int, mod: int) -> int:
    """Modular exponentiation."""
    result = 1
    base = base % mod
    while exp > 0:
        if exp & 1:
            result = (result * base) % mod
        base = (base * base) % mod
        exp >>= 1
    return result


def lagrange_extrapolation(
    f: Callable[[int], int], n_points: int, mod: int
) -> Callable[[int], int]:
    """Extrapolate function using Lagrange interpolation."""
    # Generate n_points values
    values = []
    for i in range(1, n_points + 1):
        values.append(f(i) % mod)

    def interpolate(x: int) -> int:
        """Interpolate at point x."""
        result = 0
        for i in range(n_points):
            term = values[i]
            for j in range(n_points):
                if i != j:
                    denom = (i + 1 - (j + 1)) % mod
                    if denom == 0:
                        continue
                    inv = pow(denom, mod - 2, mod)
                    term = (term * (x - (j + 1)) * inv) % mod
            result = (result + term) % mod
        return result

    return interpolate


def H(t: int, r: int, M: int) -> int:
    """Compute H(t, r)."""
    mult = 1
    res = 1
    # Compute (2^t + 1)^r in binary
    num = pow(2, t) + 1
    num_power = pow(num, r)
    binary_str = bin(num_power)[2:]
    # Split by '1' to get consecutive zeros
    parts = binary_str.split("1")
    for s in parts:
        if s:
            res = (res * pow_mod(mult, len(s), M)) % M
        mult = (mult * 5 + 3) % M
    return res


def solve() -> int:
    """Solve Problem 811."""
    N = 10**14 + 31
    K = 62
    M = 1_000_062_031

    # H(K + n, K) satisfies a linear recurrence of order 1 (geometric):
    #   H(K + n + 1, K) = ratio * H(K + n, K)  (mod M)
    # Compute two sample values to determine the ratio.
    v1 = H(K + 1, K, M)
    v2 = H(K + 2, K, M)
    ratio = (v2 * pow(v1, -1, M)) % M

    # Answer = H(K + (N - K), K) = v1 * ratio^(N - K - 1) mod M
    return (v1 * pow(ratio, N - K - 1, M)) % M


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
