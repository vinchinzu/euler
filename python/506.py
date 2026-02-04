"""Project Euler Problem 506: Clock sequence.

Break the sequence of digits 123432123432... into a sequence v_n such that
the sum of the digits in v_n is n. Find sum_{k=1}^N v_k mod M.

We compute small values of the cumulative sum, find a linear recurrence using
Berlekamp-Massey (mod prime factors via CRT), then evaluate at N using
Kitamasa's method (polynomial exponentiation in the quotient ring).
"""

from __future__ import annotations


def sum_digits_of(s: str) -> int:
    """Sum of digits in string."""
    return sum(int(c) for c in s) if s else 0


def berlekamp_massey(seq, mod):
    """Find the shortest linear recurrence for seq modulo mod (must be prime).

    Returns coefficients [c1, c2, ..., cL] such that
    a[n] = c1*a[n-1] + c2*a[n-2] + ... + cL*a[n-L] for all n >= L.
    """
    n = len(seq)
    C = [1]  # connection polynomial
    B = [1]
    L = 0
    m = 1
    b = 1

    for i in range(n):
        d = seq[i]
        for j in range(1, L + 1):
            d = (d + C[j] * seq[i - j]) % mod
        d %= mod

        if d == 0:
            m += 1
        elif 2 * L <= i:
            T = C[:]
            coeff = d * pow(b, mod - 2, mod) % mod
            while len(C) < len(B) + m:
                C.append(0)
            for j in range(len(B)):
                C[j + m] = (C[j + m] - coeff * B[j]) % mod
            L = i + 1 - L
            B = T
            b = d
            m = 1
        else:
            coeff = d * pow(b, mod - 2, mod) % mod
            while len(C) < len(B) + m:
                C.append(0)
            for j in range(len(B)):
                C[j + m] = (C[j + m] - coeff * B[j]) % mod
            m += 1

    # Return [c1, c2, ..., cL] where a[n] = c1*a[n-1] + ... + cL*a[n-L]
    # Connection polynomial C satisfies: C[0]*a[n] + C[1]*a[n-1] + ... + C[L]*a[n-L] = 0
    # So a[n] = -C[1]*a[n-1] - ... - C[L]*a[n-L]
    return L, [(-C[i]) % mod for i in range(1, L + 1)]


def poly_mult_mod(a, b, rec, mod):
    """Multiply polynomials a, b modulo characteristic polynomial and mod.

    Polynomials: [c0, c1, ...] represents c0 + c1*x + c2*x^2 + ...
    rec: [c1, c2, ..., cL] such that x^L = c1*x^{L-1} + c2*x^{L-2} + ... + cL
    i.e., x^L = sum_{i=1}^{L} c_i * x^{L-i}
    """
    L = len(rec)
    # raw multiply
    raw = [0] * (len(a) + len(b) - 1)
    for i in range(len(a)):
        if a[i] == 0:
            continue
        for j in range(len(b)):
            raw[i + j] = (raw[i + j] + a[i] * b[j]) % mod

    # Reduce: x^L = rec[0]*x^{L-1} + rec[1]*x^{L-2} + ... + rec[L-1]*x^0
    # So the "replacement" polynomial for x^L is:
    # rep[L-1] = rec[0], rep[L-2] = rec[1], ..., rep[0] = rec[L-1]
    rep = [rec[L - 1 - i] % mod for i in range(L)]  # rep[j] is coefficient of x^j

    for i in range(len(raw) - 1, L - 1, -1):
        if raw[i] == 0:
            continue
        c = raw[i]
        raw[i] = 0
        for j in range(L):
            raw[i - L + j] = (raw[i - L + j] + c * rep[j]) % mod

    return raw[:L]


def eval_recurrence(rec, init, n, mod):
    """Evaluate linear recurrence at position n.

    rec: [c1, c2, ..., cL] where a[n] = c1*a[n-1] + ... + cL*a[n-L]
    init: [a[0], a[1], ..., a[L-1]]
    Returns a[n] mod mod.
    """
    L = len(rec)
    if n < L:
        return init[n] % mod

    # Compute x^n mod characteristic polynomial using binary exponentiation
    # Result: r[0] + r[1]*x + ... + r[L-1]*x^{L-1}
    # Then a[n] = r[0]*a[0] + r[1]*a[1] + ... + r[L-1]*a[L-1]

    result = [0] * L
    result[0] = 1  # = x^0 = 1
    base = [0] * L
    if L > 1:
        base[1] = 1  # = x^1
    else:
        # L == 1: x = rec[0] (mod char poly)
        base[0] = rec[0] % mod

    exp = n
    while exp > 0:
        if exp & 1:
            result = poly_mult_mod(result, base, rec, mod)
        base = poly_mult_mod(base, base, rec, mod)
        exp >>= 1

    ans = 0
    for i in range(L):
        ans = (ans + result[i] * init[i]) % mod
    return ans


def solve_linear_mod_prime(A, b, p):
    """Solve Ax = b (mod p) using Gaussian elimination. p must be prime."""
    n = len(A)
    aug = [A[i][:] + [b[i]] for i in range(n)]

    for col in range(n):
        pivot = None
        for row in range(col, n):
            if aug[row][col] % p != 0:
                pivot = row
                break
        if pivot is None:
            return None

        aug[col], aug[pivot] = aug[pivot], aug[col]
        inv = pow(aug[col][col] % p, p - 2, p)
        for j in range(n + 1):
            aug[col][j] = (aug[col][j] * inv) % p
        for row in range(n):
            if row == col:
                continue
            factor = aug[row][col] % p
            for j in range(n + 1):
                aug[row][j] = (aug[row][j] - factor * aug[col][j]) % p

    return [aug[i][n] % p for i in range(n)]


def find_recurrence_mod_pe(seq, L, p, e):
    """Find recurrence coefficients mod p^e given that the order is L.

    seq: sequence values (must have at least 2*L elements)
    L: recurrence order (found via BM mod p)
    p: prime
    e: exponent
    Returns [c1, ..., cL] mod p^e such that a[n] = c1*a[n-1]+...+cL*a[n-L]
    """
    pe = p ** e
    if L == 0:
        return []

    # Build linear system: for i=0..L-1:
    # c1*a[i+L-1] + c2*a[i+L-2] + ... + cL*a[i] = a[i+L]
    # This is L equations in L unknowns c1,...,cL
    # Matrix row i: [a[i+L-1], a[i+L-2], ..., a[i]]
    # RHS: a[i+L]

    # Solve by Hensel lifting: solve mod p, then lift to mod p^e
    seq_pe = [x % pe for x in seq]
    seq_p = [x % p for x in seq]

    # Solve mod p
    A_p = []
    b_p = []
    for i in range(L):
        row = [seq_p[i + L - 1 - j] for j in range(L)]
        A_p.append(row)
        b_p.append(seq_p[i + L])

    coeffs_cur = solve_linear_mod_prime(A_p, b_p, p)
    if coeffs_cur is None:
        return None

    # Lift from mod p to mod p^e iteratively
    cur_mod = p
    for step in range(1, e):
        next_mod = p ** (step + 1)
        # coeffs_cur are correct mod cur_mod = p^step
        # We want coeffs mod p^{step+1}
        # coeffs = coeffs_cur + cur_mod * delta (mod next_mod)
        # Compute residuals: a[i+L] - sum(c_j * a[i+L-j]) should be 0 mod next_mod
        residuals = []
        for i in range(L):
            r = seq_pe[i + L] % next_mod
            for j in range(L):
                r = (r - coeffs_cur[j] * seq_pe[i + L - 1 - j]) % next_mod
            # r should be divisible by cur_mod
            r = r % next_mod
            if r < 0:
                r += next_mod
            residuals.append((r // cur_mod) % p)

        # Solve A_p * delta = residuals (mod p)
        delta = solve_linear_mod_prime(A_p, residuals, p)
        if delta is None:
            return None

        coeffs_cur = [(coeffs_cur[j] + cur_mod * delta[j]) % next_mod for j in range(L)]
        cur_mod = next_mod

    return coeffs_cur


def extended_gcd(a, b):
    """Extended GCD returning (g, x, y) such that a*x + b*y = g."""
    if b == 0:
        return a, 1, 0
    g, x, y = extended_gcd(b, a % b)
    return g, y, x - (a // b) * y


def crt2(r1, m1, r2, m2):
    """Chinese Remainder Theorem for two coprime moduli."""
    g, x, _ = extended_gcd(m1, m2)
    lcm = m1 * m2 // g
    diff = (r2 - r1) // g
    r = (r1 + m1 * (diff * x % (m2 // g))) % lcm
    return r, lcm


def solve() -> int:
    """Solve Problem 506."""
    DIGITS = "123432"
    N = 10**14
    M = 123454321  # = 41^2 * 271^2

    # Compute cumulative sums f(n) = sum_{k=1}^n (v_k mod M), all mod M
    num_values = 500

    values = []
    cum_sum = 0
    digit_idx = 0
    for term_num in range(1, num_values + 1):
        v = ""
        while sum_digits_of(v) < term_num:
            v += DIGITS[digit_idx % len(DIGITS)]
            digit_idx += 1
        val = int(v) % M
        cum_sum = (cum_sum + val) % M
        values.append(cum_sum)

    # M = 41^2 * 271^2. Work mod each prime power separately, then CRT.
    prime_powers = [(41, 2), (271, 2)]

    results = []
    for p, e in prime_powers:
        pe = p ** e
        # Find recurrence order via BM mod p
        seq_p = [v % p for v in values]
        L, _ = berlekamp_massey(seq_p, p)

        # Find recurrence coefficients mod p^e
        coeffs = find_recurrence_mod_pe(values, L, p, e)
        if coeffs is None or L == 0:
            results.append((0, pe))
            continue

        # Verify the recurrence on a few more terms
        seq_pe = [v % pe for v in values]
        for i in range(L, min(L + 10, len(values))):
            expected = 0
            for j in range(L):
                expected = (expected + coeffs[j] * seq_pe[i - 1 - j]) % pe
            assert expected == seq_pe[i], f"Recurrence verification failed at i={i}"

        # Evaluate at N-1 (0-indexed)
        init = seq_pe[:L]
        val = eval_recurrence(coeffs, init, N - 1, pe)
        results.append((val % pe, pe))

    # CRT to combine
    r, m = results[0]
    for val, pe in results[1:]:
        r, m = crt2(r, m, val, pe)

    return r % M


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
