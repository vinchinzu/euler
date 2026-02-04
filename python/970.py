"""Project Euler Problem 970: Kangaroo Hops.

H(n) is the expected number of hops for a kangaroo to pass n.
H(n) ~ 2n + 2/3 + sum_k C_k * exp(lambda_k * n)
where lambda_k are roots of lambda + exp(-lambda) = 1.

We compute delta_n = H(n) - 2n - 2/3 for several n values using exact summation,
fit the first two root pairs, then extrapolate to n=10^6 to extract the first 8
non-six digits after the decimal point.
"""

from mpmath import (mp, mpf, exp, cos, sin, log, log10, fabs, floor, frac,
                    power, findroot, re, im, factorial, binomial, matrix)


def compute_H(n_val, dps_val):
    """Compute H(n) = sum_{k>=0} P(S_k <= n) to high precision."""
    old_dps = mp.dps
    mp.dps = dps_val
    n = mpf(n_val)
    fn = int(n_val)

    H = mpf(0)
    for k in range(0, 20 * fn + 500):
        s = mpf(0)
        for j in range(0, min(k, fn) + 1):
            sign = mpf((-1) ** j)
            binom_val = binomial(k, j)
            base = n - j
            term = sign * binom_val * power(base, k) / factorial(k)
            s += term
        H += s
        if k > 2 * fn + 10 and fabs(s) < power(10, -(dps_val - 20)):
            break

    mp.dps = old_dps
    return H


def solve():
    """Solve Problem 970."""
    mp.dps = 120

    # Find roots of f(l) = l + exp(-l) - 1 = 0
    f = lambda l: l + exp(-l) - mpf(1)

    # First root pair (least negative real part)
    lambda1 = findroot(f, mpf('-2.0888') + mpf('7.4615') * 1j, tol=mpf(10)**(-110))
    p1, q1 = re(lambda1), im(lambda1)

    # Second root pair
    lambda2 = findroot(f, mpf('-2.6641') + mpf('13.879') * 1j, tol=mpf(10)**(-110))
    p2, q2 = re(lambda2), im(lambda2)

    # Compute H(n) for several values and get delta_n = H(n) - 2n - 2/3
    ns_fit = [30, 35, 40, 50]
    deltas = []
    for nv in ns_fit:
        H = compute_H(nv, 120)
        d = H - 2 * mpf(nv) - mpf(2) / 3
        deltas.append(d)

    # Fit 4 parameters: A1, B1, A2, B2
    # delta_n = exp(p1*n)*(A1*cos(q1*n) + B1*sin(q1*n))
    #         + exp(p2*n)*(A2*cos(q2*n) + B2*sin(q2*n))
    M_mat = matrix(4, 4)
    for i in range(4):
        ni = mpf(ns_fit[i])
        e1 = exp(p1 * ni)
        c1 = cos(q1 * ni)
        s1 = sin(q1 * ni)
        e2 = exp(p2 * ni)
        c2 = cos(q2 * ni)
        s2 = sin(q2 * ni)
        M_mat[i, 0] = e1 * c1
        M_mat[i, 1] = e1 * s1
        M_mat[i, 2] = e2 * c2
        M_mat[i, 3] = e2 * s2

    delta_vec = matrix([d for d in deltas])
    x = M_mat ** -1 * delta_vec
    A1 = x[0]
    B1 = x[1]

    # Extrapolate to n = 10^6
    # Only the first root pair matters (second pair has exp((p2-p1)*10^6) ~ 0)
    n = mpf('1000000')
    oscillation = A1 * cos(q1 * n) + B1 * sin(q1 * n)

    # delta = exp(p1*n) * oscillation
    # log10(|delta|) = p1*n/ln(10) + log10(|oscillation|)
    log10_delta = p1 * n / log(10) + log10(fabs(oscillation))

    # |delta| = mantissa * 10^exponent where exponent = floor(log10_delta)
    # mantissa m = 10^frac(log10_delta), in [1, 10)
    frac_log = frac(log10_delta)
    m = power(10, frac_log)  # mantissa in [1, 10)

    # L = -floor(log10_delta) = position where deviation starts
    # Digit extraction:
    # H(n) - 2n = 2/3 + delta
    # For delta < 0: H(n) - 2n = 2/3 - m*10^{-L}
    # 10^L * (H(n) - 2n) = Q + 2/3 - m, where Q = (2*10^L - 2)/3 = 666...6 (L sixes)
    # The digit at position L = last digit of floor(Q + 2/3 - m) = (Q + floor(2/3 - m)) mod 10
    # For delta > 0: similarly with + m

    if oscillation < 0:
        # delta < 0
        g = mpf(2) / 3 - m
    else:
        # delta > 0
        g = mpf(2) / 3 + m

    g_floor = int(floor(g))
    g_frac = g - floor(g)

    # Q mod 10 = 6 (Q = 666...6)
    # (Q + g_floor) mod 10 = (6 + g_floor) mod 10
    last_int_digit = (6 + g_floor) % 10

    # Extract digits: first the integer's last digit, then fractional digits
    frac_digits = floor(g_frac * power(10, 50))
    frac_str = str(int(frac_digits)).zfill(50)

    non_six_digits = ''

    # Check the integer's last digit
    if str(last_int_digit) != '6':
        non_six_digits += str(last_int_digit)

    # Then scan fractional digits
    for digit in frac_str:
        if digit != '6':
            non_six_digits += digit
        if len(non_six_digits) == 8:
            break

    print(non_six_digits)


if __name__ == "__main__":
    solve()
