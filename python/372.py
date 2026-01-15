"""
Project Euler Problem 372: Pencils of Rays

Let R(M, N) be the number of lattice points (x, y) which satisfy M < x ≤ N, M < y ≤ N and floor(y^2 / x^2) is odd.
We can verify that R(0, 100) = 3019 and R(100, 10000) = 29750422.
Find R(2·10^6, 10^9).

Note: floor x represents the floor function.
"""

from __future__ import annotations
from math import isqrt


def is_perfect_square(d: int) -> bool:
    s: int = isqrt(d)
    return s * s == d


def sum_floor(n: int, d: int) -> int:
    if n <= 0:
        return 0
    if is_perfect_square(d):
        s: int = isqrt(d)
        return s * n * (n + 1) // 2
    p: int = n * (n + 1) // 2
    pp: int = p * p
    dp: int = d * pp
    m0: int = isqrt(dp)
    left: int = 4 * dp
    right: int = (2 * m0 + 1) ** 2
    f_ge_05: bool = left >= right
    i: int = n // 2
    if n % 2 == 1:
        return m0 - i
    else:
        return m0 - i + (1 if f_ge_05 else 0)


def floor_div_sqrt(num: int, den: int) -> int:
    if den == 0 or num == 0:
        return 0
    temp: int = (num * num) // den
    z: int = isqrt(temp)
    if (z + 1) ** 2 * den <= num * num:
        z += 1
    while z ** 2 * den > num * num:
        z -= 1
    return z


def sum_upper(ll: int, rr: int, N: int, d: int) -> int:
    if ll > rr:
        return 0
    res: int = 0
    if is_perfect_square(d):
        s: int = isqrt(d)
        x_split: int = N // s
        left_end: int = min(rr, x_split)
        if ll <= left_end:
            num1: int = left_end - ll + 1
            sum_x1: int = num1 * (ll + left_end) // 2
            res += s * sum_x1
        right_start: int = max(ll, x_split + 1)
        if right_start <= rr:
            num2: int = rr - right_start + 1
            res += N * num2
    else:
        x1: int = floor_div_sqrt(N + 1, d)
        r1: int = min(rr, x1)
        if ll <= r1:
            res += sum_floor(r1, d) - sum_floor(ll - 1, d)
        l2: int = max(ll, x1 + 1)
        if l2 <= rr:
            res += N * (rr - l2 + 1)
    return res


def sum_lower_max(ll: int, rr: int, L: int, d: int) -> int:
    if ll > rr:
        return 0
    res: int = 0
    if is_perfect_square(d):
        s: int = isqrt(d)
        x_split: int = L // s
        left_end: int = min(rr, x_split)
        if ll <= left_end:
            num1: int = left_end - ll + 1
            res += L * num1
        right_start: int = max(ll, x_split + 1)
        if right_start <= rr:
            num2: int = rr - right_start + 1
            sum_x2: int = num2 * (right_start + rr) // 2
            res += s * sum_x2
    else:
        x_low_local: int = floor_div_sqrt(L, d) + 1
        r1: int = min(rr, x_low_local - 1)
        if ll <= r1:
            res += L * (r1 - ll + 1)
        l2: int = max(ll, x_low_local)
        if l2 <= rr:
            res += sum_floor(rr, d) - sum_floor(l2 - 1, d)
            res += (rr - l2 + 1)
    return res


def compute_R(M: int, N: int) -> int:
    L: int = M + 1
    if L > N:
        return 0
    total: int = 0
    max_ratio: int = (N // L) + 1
    max_k: int = 2 * max_ratio * max_ratio
    k: int = 1
    while k <= max_k:
        x_start: int = max(L, floor_div_sqrt(L, k + 1) + 1)
        x_end: int = min(N, floor_div_sqrt(N, k))
        if x_start > x_end:
            k += 2
            continue
        su: int = sum_upper(x_start, x_end, N, k + 1)
        sl: int = sum_lower_max(x_start, x_end, L, k)
        num_x: int = x_end - x_start + 1
        contrib: int = su - sl + num_x
        if contrib > 0:
            total += contrib
        k += 2
    return total


if __name__ == "__main__":
    print(compute_R(2000000, 1000000000))
