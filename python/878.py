from __future__ import annotations

"""
Project Euler Problem 878
XOR-Equation B

We use x⊕y for the bitwise XOR of x and y.
The XOR-product x⊗y is the carryless (polynomial) multiplication in base 2
where partial products are XORed instead of added; for example 7⊗3 = 9.

For non-negative integers a and b, define
    (a⊗a) ⊕ (2⊗a⊗b) ⊕ (b⊗b) = k.

Let F(N, k) be the number of pairs (a, b) with 0 ≤ a ≤ b ≤ N satisfying
the equation for a given k. Define G(N, m) = Σ_{k=0..m} F(N, k), i.e. the
number of such pairs with 0 ≤ k ≤ m.
We are given G(1000, 100) = 398.

Find G(10^17, 10^6).
"""

from typing import Dict, List

Poly = int
SElement = tuple[Poly, Poly]
FactorMap = Dict[Poly, int]

X: Poly = 2
OMEGA: SElement = (0, 1)
OMEGA_INV: SElement = (X, 1)
MAX_IRRED_DEG = 10


def poly_deg(a: Poly) -> int:
    """Return the degree of polynomial a represented as an integer."""
    return a.bit_length() - 1 if a > 0 else -1


def poly_add(a: Poly, b: Poly) -> Poly:
    return a ^ b


def poly_mul(a: Poly, b: Poly) -> Poly:
    res = 0
    while b > 0:
        if b & 1:
            res ^= a
        a <<= 1
        b >>= 1
    return res


def poly_mod(a: Poly, b: Poly) -> Poly:
    if b == 0:
        raise ValueError("Division by zero")
    da = poly_deg(a)
    db = poly_deg(b)
    if da < db:
        return a

    while da >= db:
        a ^= b << (da - db)
        da = poly_deg(a)
    return a


def poly_divmod(a: Poly, b: Poly) -> tuple[Poly, Poly]:
    if b == 0:
        raise ValueError("Division by zero")
    da = poly_deg(a)
    db = poly_deg(b)
    if da < db:
        return 0, a

    q = 0
    while da >= db:
        shift = da - db
        q ^= 1 << shift
        a ^= b << shift
        da = poly_deg(a)
    return q, a


def poly_gcd(a: Poly, b: Poly) -> Poly:
    while b:
        a, b = b, poly_mod(a, b)
    return a


def poly_sq(a: Poly) -> Poly:
    # a(x)^2 = a(x^2)
    res = 0
    shift = 0
    while a > 0:
        if a & 1:
            res |= 1 << shift
        a >>= 1
        shift += 2
    return res


def s_mul(alpha: SElement, beta: SElement) -> SElement:
    A1, B1 = alpha
    A2, B2 = beta
    a1a2 = poly_mul(A1, A2)
    b1b2 = poly_mul(B1, B2)
    outer = poly_mul(A1, B2) ^ poly_mul(B1, A2)
    new_a = a1a2 ^ b1b2
    new_b = outer ^ poly_mul(X, b1b2)
    return new_a, new_b


def s_norm(alpha: SElement) -> Poly:
    A, B = alpha
    term1 = poly_sq(A)
    term3 = poly_sq(B)
    term2 = poly_mul(poly_mul(X, A), B)
    return term1 ^ term2 ^ term3


def s_conjugate(alpha: SElement) -> SElement:
    A, B = alpha
    return A ^ poly_mul(X, B), B


def s_divmod(alpha: SElement, beta: SElement) -> tuple[SElement, SElement]:
    beta_conj = s_conjugate(beta)
    num = s_mul(alpha, beta_conj)
    den = s_norm(beta)

    if den == 0:
        raise ValueError("Division by zero in S")

    U, V = num
    qU, _ = poly_divmod(U, den)
    qV, _ = poly_divmod(V, den)

    q = (qU, qV)
    prod = s_mul(q, beta)

    rA = alpha[0] ^ prod[0]
    rB = alpha[1] ^ prod[1]
    return q, (rA, rB)


def s_gcd(alpha: SElement, beta: SElement) -> SElement:
    if alpha == (0, 0):
        return beta
    if beta == (0, 0):
        return alpha

    while beta != (0, 0):
        _, r = s_divmod(alpha, beta)
        alpha, beta = beta, r
    return alpha


def get_irreducibles(max_deg: int) -> List[Poly]:
    irreds: List[Poly] = []
    for i in range(2, 1 << (max_deg + 1)):
        if i > 2 and (i & 1) == 0:
            continue

        deg = poly_deg(i)
        is_irred = True
        for p in irreds:
            if poly_deg(p) * 2 > deg:
                break
            if poly_mod(i, p) == 0:
                is_irred = False
                break
        if is_irred:
            irreds.append(i)
    return irreds


PRECOMPUTED_IRREDS: List[Poly] = get_irreducibles(MAX_IRRED_DEG)
prime_gen_cache: dict[Poly, SElement | None] = {}


def factor_poly(k: int) -> FactorMap:
    factors: FactorMap = {}
    if k <= 1:
        return factors

    rem = k
    for p in PRECOMPUTED_IRREDS:
        if poly_deg(p) * 2 > poly_deg(rem):
            break

        while True:
            q, r = poly_divmod(rem, p)
            if r == 0:
                factors[p] = factors.get(p, 0) + 1
                rem = q
                if rem == 1:
                    break
            else:
                break
        if rem == 1:
            break

    if rem > 1:
        factors[rem] = factors.get(rem, 0) + 1

    return factors


def inverse_poly_mod(a: Poly, m: Poly) -> Poly:
    t, newt = 0, 1
    r, newr = m, a
    while newr != 0:
        q, _ = poly_divmod(r, newr)
        t, newt = newt, t ^ poly_mul(q, newt)
        r, newr = newr, r ^ poly_mul(q, newr)

    if poly_deg(r) > 0:
        raise ValueError("Not invertible")
    return t


def solve_root_quadratic(c_val: Poly, p: Poly) -> int | None:
    d = poly_deg(p)
    if d % 2 == 1:
        z = 0
        term = c_val
        for _ in range((d + 1) // 2):
            z ^= term
            term = poly_sq(term)
            term = poly_mod(term, p)
            term = poly_sq(term)
            term = poly_mod(term, p)
        return z

    for z in range(1 << d):
        val = poly_sq(z) ^ z
        if poly_mod(val, p) == c_val:
            return z
    return None


def find_prime_generator(p: Poly) -> SElement | None:
    if p == X:
        return 1, 1

    inv_x2 = inverse_poly_mod(poly_sq(X), p)
    z = solve_root_quadratic(inv_x2, p)

    if z is None:
        return None

    val = poly_sq(z) ^ z
    if poly_mod(val, p) != inv_x2:
        return None

    t0 = poly_mod(poly_mul(z, X), p)
    gen = s_gcd((p, 0), (t0, 1))
    return gen


def get_prime_gen(p: Poly) -> SElement | None:
    if p in prime_gen_cache:
        return prime_gen_cache[p]

    if p == X:
        res: SElement | None = (1, 1)
    else:
        res = find_prime_generator(p)

    prime_gen_cache[p] = res
    return res


def _iterate_orbit(
    start: SElement, step: SElement, n_limit: int, n_deg: int
) -> int:
    count = 0
    curr = start
    while True:
        dA = poly_deg(curr[0])
        dB = poly_deg(curr[1])
        if dA > n_deg + 2 and dB > n_deg + 2:
            break

        valA, valB = curr
        if valA <= n_limit and valB <= n_limit and valA <= valB:
            count += 1
        elif (
            valA > n_limit
            and valB > n_limit
            and dA > n_deg
            and dB > n_deg
        ):
            break
        curr = s_mul(curr, step)
    return count


def count_for_k(k: int, n_limit: int, n_deg: int) -> int:
    if k == 0:
        return 1

    factors = factor_poly(k)
    current_gens: List[SElement] = [(1, 0)]

    for p, e in factors.items():
        if p == X:
            gen = get_prime_gen(p)
            term = (1, 0)
            base = gen if gen is not None else (0, 0)
            exp = e
            while exp > 0:
                if exp & 1:
                    term = s_mul(term, base)
                base = s_mul(base, base)
                exp >>= 1
            current_gens = [s_mul(g, term) for g in current_gens]
            continue

        gen = get_prime_gen(p)
        if gen is None:
            if e % 2 != 0:
                return 0
            scale = 1
            for _ in range(e // 2):
                scale = poly_mul(scale, p)

            current_gens = [
                (poly_mul(g[0], scale), poly_mul(g[1], scale))
                for g in current_gens
            ]
            continue

        gen_conj = s_conjugate(gen)
        pow_pi: List[SElement] = [(1, 0)] * (e + 1)
        pow_bar: List[SElement] = [(1, 0)] * (e + 1)

        curr = (1, 0)
        for i in range(1, e + 1):
            curr = s_mul(curr, gen)
            pow_pi[i] = curr

        curr = (1, 0)
        for i in range(1, e + 1):
            curr = s_mul(curr, gen_conj)
            pow_bar[i] = curr

        next_gens: List[SElement] = []
        for g in current_gens:
            for a in range(e + 1):
                b = e - a
                factor = s_mul(pow_pi[a], pow_bar[b])
                combined = s_mul(g, factor)
                next_gens.append(combined)
        current_gens = next_gens

    total_count = 0
    for g in current_gens:
        total_count += _iterate_orbit(g, OMEGA, n_limit, n_deg)
        total_count += _iterate_orbit(
            s_mul(g, OMEGA_INV), OMEGA_INV, n_limit, n_deg
        )

    return total_count


def solve(N: int = 10**17, m: int = 1_000_000) -> int:
    """
    Return G(N, m), the count of pairs (a, b) with 0 ≤ a ≤ b ≤ N whose XOR
    quadratic lies in [0, m].
    """
    if N < 0 or m < 0:
        raise ValueError("N and m must be non-negative")

    total = 1
    n_deg = N.bit_length()
    for k in range(1, m + 1):
        total += count_for_k(k, N, n_deg)
    return total


if __name__ == "__main__":
    print(solve())
