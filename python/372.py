"""
Project Euler Problem 372: Pencils of Rays

R(M, N) = number of lattice points (x, y) with M < x <= N, M < y <= N
where floor(y^2 / x^2) is odd.

R(0, 100) = 3019, R(100, 10000) = 29750422.
Find R(2*10^6, 10^9).
"""

from math import isqrt


def exact_floor_n_alpha(n, d, P, Q):
    """Compute floor(n * (sqrt(d) + P) / Q) exactly.

    Represents alpha = (sqrt(d) + P) / Q as a quadratic irrational.
    """
    if n == 0:
        return 0
    S = isqrt(n * n * d)
    total_int = n * P + S
    q_div, r = divmod(total_int, Q)
    need = Q - r
    if need <= 0:
        return q_div + 1
    rhs = S + need
    if n * n * d >= rhs * rhs:
        return q_div + 1
    return q_div


def sum_floor(n, d):
    """Compute sum_{x=1}^n floor(x * sqrt(d)) using continued fraction recursion.

    For perfect square d = s^2, returns s * n * (n+1) / 2.
    For non-perfect-square d, uses the lattice point counting recursion:
      SF(n, alpha) = n*m - a*m*(m+1)/2 - SF(m, beta)
    where alpha = sqrt(d) - isqrt(d), m = floor(n*alpha), a = floor(1/alpha),
    and beta = 1/alpha - a.
    """
    if n <= 0:
        return 0
    s = isqrt(d)
    if s * s == d:
        return s * n * (n + 1) // 2

    base = s * n * (n + 1) // 2

    P_cur = -s
    Q_cur = 1
    n_cur = n
    sign = 1
    result = base

    while n_cur > 0:
        m = exact_floor_n_alpha(n_cur, d, P_cur, Q_cur)
        if m == 0:
            break

        denom = d - P_cur * P_cur
        Q_new = denom // Q_cur
        P_inv = -P_cur

        a = exact_floor_n_alpha(1, d, P_inv, Q_new)

        result += sign * (n_cur * m - a * m * (m + 1) // 2)

        P_cur = P_inv - a * Q_new
        Q_cur = Q_new
        n_cur = m
        sign = -sign

    return result


def floor_div_sqrt(num, den):
    """Compute floor(num / sqrt(den)) = largest integer x with x^2 * den <= num^2."""
    if den == 0 or num <= 0:
        return 0
    # x^2 * den <= num^2
    val = num * num // den
    z = isqrt(val)
    # Adjust: we need z^2 * den <= num^2
    while (z + 1) * (z + 1) * den <= num * num:
        z += 1
    while z * z * den > num * num:
        z -= 1
    return z


def sum_upper(ll, rr, N, d):
    """Compute sum of min(N, upper(x, d)) for x in [ll, rr].

    upper(x, d) = floor(sqrt(d*x^2 - 1)):
    - If d = s^2: upper(x,d) = s*x - 1
    - If d is not a perfect square: upper(x,d) = floor(x*sqrt(d)) = isqrt(d*x^2)
    """
    if ll > rr:
        return 0

    s = isqrt(d)
    if s * s == d:
        # upper(x,d) = s*x - 1
        # Split at x where s*x - 1 >= N, i.e., s*x >= N+1, i.e., x >= ceil((N+1)/s)
        x_split = (N + s) // s  # ceil((N+1)/s)
        # For x < x_split: min(N, s*x-1) = s*x-1
        # For x >= x_split: min(N, s*x-1) = N
        left_end = min(rr, x_split - 1)
        res = 0
        if ll <= left_end:
            # sum of (s*x - 1) for x = ll to left_end
            cnt = left_end - ll + 1
            sum_x = cnt * (ll + left_end) // 2
            res += s * sum_x - cnt
        right_start = max(ll, x_split)
        if right_start <= rr:
            cnt = rr - right_start + 1
            res += N * cnt
        return res
    else:
        # upper(x,d) = floor(x*sqrt(d))
        # Split at x where floor(x*sqrt(d)) >= N, i.e., x >= ceil(N/sqrt(d))
        # ceil(N/sqrt(d)) = floor_div_sqrt(N, d) + 1 if N/sqrt(d) is not integer
        # Since d is not perfect square, N/sqrt(d) is never integer for integer N.
        x_split = floor_div_sqrt(N, d) + 1  # smallest x with floor(x*sqrt(d)) >= N
        # Actually: floor(x*sqrt(d)) >= N iff x*sqrt(d) >= N iff x >= N/sqrt(d)
        # So x_split = ceil(N/sqrt(d)) = floor_div_sqrt(N, d) + 1

        # For x < x_split: min(N, floor(x*sqrt(d))) = floor(x*sqrt(d))
        # For x >= x_split: min(N, floor(x*sqrt(d))) = N
        left_end = min(rr, x_split - 1)
        res = 0
        if ll <= left_end:
            res += sum_floor(left_end, d) - sum_floor(ll - 1, d)
        right_start = max(ll, x_split)
        if right_start <= rr:
            cnt = rr - right_start + 1
            res += N * cnt
        return res


def sum_lower_max(ll, rr, L, d):
    """Compute sum of max(L, lower(x, d)) for x in [ll, rr].

    lower(x, d) = ceil(x*sqrt(d)):
    - If d = s^2: lower(x,d) = s*x
    - If d is not a perfect square: lower(x,d) = floor(x*sqrt(d)) + 1
    """
    if ll > rr:
        return 0

    s = isqrt(d)
    if s * s == d:
        # lower(x,d) = s*x
        # max(L, s*x). Split at x where s*x >= L, i.e., x >= ceil(L/s)
        x_split = (L + s - 1) // s  # ceil(L/s)
        res = 0
        # For x < x_split: s*x < L, so max(L, s*x) = L
        left_end = min(rr, x_split - 1)
        if ll <= left_end:
            cnt = left_end - ll + 1
            res += L * cnt
        # For x >= x_split: s*x >= L, so max(L, s*x) = s*x
        right_start = max(ll, x_split)
        if right_start <= rr:
            cnt = rr - right_start + 1
            sum_x = cnt * (right_start + rr) // 2
            res += s * sum_x
        return res
    else:
        # lower(x,d) = floor(x*sqrt(d)) + 1
        # max(L, floor(x*sqrt(d)) + 1)
        # Split at x where floor(x*sqrt(d)) + 1 >= L, i.e., floor(x*sqrt(d)) >= L-1
        # floor(x*sqrt(d)) >= L-1 iff x*sqrt(d) >= L-1 (since L-1 is integer and x*sqrt(d) irrational)
        # iff x >= (L-1)/sqrt(d), x >= ceil((L-1)/sqrt(d)) = floor_div_sqrt(L-1, d) + 1
        if L <= 1:
            x_split = 1  # lower is always >= 1 >= L
        else:
            x_split = floor_div_sqrt(L - 1, d) + 1

        res = 0
        # For x < x_split: floor(x*sqrt(d)) + 1 < L, so max = L
        left_end = min(rr, x_split - 1)
        if ll <= left_end:
            cnt = left_end - ll + 1
            res += L * cnt
        # For x >= x_split: floor(x*sqrt(d)) + 1 >= L, so max = floor(x*sqrt(d)) + 1
        right_start = max(ll, x_split)
        if right_start <= rr:
            cnt = rr - right_start + 1
            sf = sum_floor(rr, d) - sum_floor(right_start - 1, d)
            res += sf + cnt  # sum of (floor(x*sqrt(d)) + 1) = sf + count
        return res


def compute_R(M, N):
    L = M + 1
    if L > N:
        return 0
    total = 0
    max_ratio = N // L + 1
    max_k = 2 * max_ratio * max_ratio
    k = 1
    while k <= max_k:
        # x range: x >= ceil(L/sqrt(k+1)), x <= floor(N/sqrt(k))
        # Also x in [L, N].
        x_lo = floor_div_sqrt(L - 1, k + 1) + 1  # ceil(L/sqrt(k+1)): smallest x with x*sqrt(k+1) >= L
        # Actually: we need lower(x, k) <= N, which means ceil(x*sqrt(k)) <= N.
        # And upper(x, k+1) >= L, which means floor(sqrt((k+1)*x^2-1)) >= L.
        # For x range: x >= ceil(L/sqrt(k+1)) and x <= floor(N/sqrt(k)).
        # But also x >= L and x <= N.
        x_start = max(L, x_lo)
        x_end = min(N, floor_div_sqrt(N, k))

        if x_start > x_end:
            k += 2
            continue

        su = sum_upper(x_start, x_end, N, k + 1)
        sl = sum_lower_max(x_start, x_end, L, k)
        num_x = x_end - x_start + 1
        contrib = su - sl + num_x
        if contrib > 0:
            total += contrib
        k += 2
    return total


if __name__ == "__main__":
    print(compute_R(2000000, 1000000000))
