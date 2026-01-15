"""Project Euler Problem 356 (Python translation).

This module computes the last eight digits of the sum

    sum_{i=1}^{30} floor(a_i ** 987654321),

where a_i is the largest real root of g(x) = x^3 - 2^i * x^2 + i.

Notes
-----
- Implemented for Python 3.12 using only the standard library.
- Uses Decimal for high-precision root finding and exponent handling.
- Public API:
    - compute_solution() -> str
    - verify_small_exponents() -> None
"""

from __future__ import annotations

from dataclasses import dataclass
from decimal import Decimal, getcontext
from typing import Iterable, List, Sequence


# Constants
K: int = 987_654_321
MOD: int = 100_000_000
BIGDEC_PRECISION: int = 100  # decimal digits
EPSILON: Decimal = Decimal("1e-50")
MAX_BINOMIAL_TERMS: int = 20

getcontext().prec = BIGDEC_PRECISION

MOD2: int = 256  # 2**8
MOD5: int = 390_625  # 5**8
PHI_MOD5: int = 312_500  # phi(5**8) = 5**8 * (1 - 1/5)


def _binomial(n: int, k: int) -> int:
    """Return C(n, k) for 0 <= k <= n using an integer-safe multiplicative form."""
    if k < 0 or k > n:
        return 0
    if k == 0 or k == n:
        return 1
    k = min(k, n - k)
    result = 1
    for i in range(1, k + 1):
        result = result * (n - i + 1) // i
    return result


def _int_power_mod(base: int, exp: int, mod: int) -> int:
    """Modular exponentiation for integer base and exponent."""
    if mod == 1:
        return 0
    base %= mod
    result = 1
    e = exp
    while e > 0:
        if e & 1:
            result = (result * base) % mod
        base = (base * base) % mod
        e >>= 1
    return result


def power_mod(base: Decimal | int, exp: int, mod: int) -> int:
    """Compute base**exp modulo mod.

    - If base is effectively integral, use fast integer modular exponentiation.
    - Otherwise, compute the high-precision power with Decimal and reduce mod.

    This mirrors the Ruby behavior: it primarily supports our use cases inside
    this module and is not intended as a fully generic Decimal modular power.
    """

    if isinstance(base, Decimal):
        base_int = int(base)
        if (base - Decimal(base_int)).copy_abs() < Decimal("1e-10"):
            base = base_int
        else:
            # Fallback: direct Decimal exponentiation, then floor and mod.
            # Acceptable here because it is used only in limited contexts.
            result = Decimal(1)
            b = base
            e = exp
            while e > 0:
                if e & 1:
                    result *= b
                b *= b
                e >>= 1
            return int(result.to_integral_value(rounding="ROUND_FLOOR")) % mod

    if isinstance(base, int):
        return _int_power_mod(base, exp, mod)

    # In practice, we should not get here. Keep a minimal safe fallback.
    b_int = int(base)
    return _int_power_mod(b_int, exp, mod)


def _floor_power_via_binomial(m: int, frac: Decimal, k: int) -> int:
    """Approximate floor((m + frac) ** k) using a truncated binomial series.

    This is a translation of the Ruby routine. It is tailored for the
    numeric ranges in Project Euler 356 and not a general-purpose routine.
    """

    if k == 0:
        return 0
    if k == 1:
        return m

    # Precompute powers of m up to k.
    m_powers: List[Decimal] = [Decimal(1)]
    m_dec = Decimal(m)
    for _ in range(1, k + 1):
        m_powers.append(m_powers[-1] * m_dec)

    result = Decimal(0)
    j = 0
    while j <= k and j < MAX_BINOMIAL_TERMS:
        binom_coeff = _binomial(k, j)
        current_term = (
            Decimal(binom_coeff) * m_powers[k - j] * (frac ** j)
        )
        result += current_term
        if current_term.copy_abs() < Decimal("1e-10"):
            break
        j += 1

    return int(result.to_integral_value(rounding="ROUND_FLOOR"))


def _complementary_power(m: int, epsilon: Decimal, k: int) -> int:
    """Handle (m - epsilon) ** k for very small epsilon.

    If epsilon is extremely small, use a first-order correction.
    Otherwise, fall back to the truncated-binomial helper.
    """

    if k == 0:
        return 0

    m_dec = Decimal(m)
    if epsilon.copy_abs() < Decimal("1e-10"):
        base = m_dec ** k
        first_order = Decimal(k) * (m_dec ** (k - 1)) * epsilon
        value = base - first_order
        return int(value.to_integral_value(rounding="ROUND_FLOOR"))

    # Fallback: use the same approximation route.
    return _floor_power_via_binomial(m, -epsilon, k)


def floor_power(x: Decimal, k: int) -> int:
    """Return floor(x ** k) using tailored approximations.

    The original Ruby code represents x = m + f with m = floor(x) and
    applies binomial-based approximations. We preserve that structure. This
    function is specific to the ranges used in this problem.
    """

    if k == 0:
        return 0
    if k == 1:
        return int(x.to_integral_value(rounding="ROUND_FLOOR"))

    m = int(x)  # floor
    frac = x - Decimal(m)

    if m == 0:
        return 0

    # Handle the case where x is extremely close to the next integer.
    if frac > Decimal("0.999999999"):
        eps = Decimal(1) - frac
        return _complementary_power(m + 1, eps, k)

    return _floor_power_via_binomial(m, frac, k)


def _cubic_value(x: Decimal, pow2n: Decimal, n: int) -> Decimal:
    """Evaluate g(x) = x^3 - 2^n x^2 + n at x with given 2^n."""

    return x**3 - pow2n * (x**2) + Decimal(n)


def find_largest_root(n: int) -> Decimal:
    """Find the largest real root of x^3 - 2^n x^2 + n via bisection."""

    pow2n = Decimal(2) ** n
    a = pow2n - Decimal(1)
    b = pow2n + Decimal(1)

    max_iterations = 200
    iterations = 0

    while (b - a).copy_abs() > EPSILON and iterations < max_iterations:
        mid = (a + b) / 2
        poly_val = _cubic_value(mid, pow2n, n)
        if poly_val > 0:
            b = mid
        else:
            a = mid
        iterations += 1

    root = (a + b) / 2
    _validate_root(root, n)
    return root


def _validate_root(root: Decimal, n: int) -> None:
    """Validate that root nearly satisfies the cubic equation.

    Raises ValueError if the root is not within the required tolerance.
    """

    pow2n = Decimal(2) ** n
    poly_val = _cubic_value(root, pow2n, n)
    if poly_val.copy_abs() >= Decimal("1e-40"):
        raise ValueError(f"Root validation failed for n={n}")


def find_root_analytical(n: int) -> Decimal:
    """Return an a_i value using any known analytical / cached forms.

    For n = 1, this defers to numeric root finding.
    For n = 2, we reuse the given approximate root from the Ruby code.
    For all other n, we fall back to numeric root finding.
    """

    if n == 1:
        return find_largest_root(1)
    if n == 2:
        return Decimal("3.86619826")
    return find_largest_root(n)


def _modular_inverse(a: int, m: int) -> int:
    """Return modular inverse of a modulo m using extended Euclidean algo.

    For modulus 5**8, use Euler's theorem with precomputed phi(MOD5).
    """

    a %= m
    if m == MOD5:
        # Assumes gcd(a, m) == 1.
        return _int_power_mod(a, PHI_MOD5 - 1, m)

    t, new_t = 0, 1
    r, new_r = m, a
    while new_r != 0:
        q = r // new_r
        t, new_t = new_t, t - q * new_t
        r, new_r = new_r, r - q * new_r
    if r != 1:
        raise ValueError("Inverse does not exist")
    if t < 0:
        t += m
    return t


def _crt(remainders: Sequence[int], moduli: Sequence[int], target_mod: int) -> int:
    """Chinese remainder theorem for pairwise coprime moduli.

    This implementation is sufficient for MOD2 and MOD5 used here.
    """

    if len(moduli) == 1:
        return remainders[0] % target_mod

    M = 1
    for m in moduli:
        M *= m

    total = 0
    for r_i, m_i in zip(remainders, moduli):
        M_i = M // m_i
        inv = _modular_inverse(M_i, m_i)
        total = (total + r_i * M_i * inv) % M

    return total % target_mod


def power_mod_crt(a: int, k: int, mod: int) -> int:
    """Compute a**k modulo mod, using CRT when mod == MOD.

    For mod == MOD (2**8 * 5**8), compute residues modulo 2**8 and 5**8, then
    reconstruct via CRT. Otherwise, fall back to standard modular exponent.
    """

    if mod == 1:
        return 0

    if mod == MOD:
        a_mod2 = a % MOD2
        a_mod5 = a % MOD5

        pow_mod2 = _int_power_mod(a_mod2, k, MOD2)
        pow_mod5 = _int_power_mod(a_mod5, k, MOD5)

        return _crt([pow_mod2, pow_mod5], [MOD2, MOD5], MOD)

    return _int_power_mod(a, k, mod)


def compute_solution() -> str:
    """Compute the last 8 digits of the Project Euler 356 expression.

    Returns the result as an 8-character zero-padded string.
    """

    total = 0
    for i in range(1, 31):
        # For i <= 2, we optionally use analytical / cached roots.
        if i <= 2:
            a_i = find_root_analytical(i)
        else:
            a_i = find_largest_root(i)

        power_term = floor_power(a_i, K) % MOD
        total = (total + power_term) % MOD

    return f"{total:08d}"


def verify_small_exponents() -> None:
    """Sanity-check floor_power against direct Decimal exponentiation.

    Tests small exponents for a few i to ensure our approximation is
    consistent with direct computation at high precision.
    """

    test_exponents = [1, 2, 3]
    for i in range(1, 6):
        a_i = find_largest_root(i)
        for k in test_exponents:
            approx = floor_power(a_i, k)
            direct = int((a_i**k).to_integral_value(rounding="ROUND_FLOOR"))
            if approx != direct:
                raise AssertionError(
                    f"Verification failed for i={i}, k={k}:"
                    f" approx={approx}, direct={direct}"
                )


if __name__ == "__main__":  # pragma: no cover
    verify_small_exponents()
    print(compute_solution())
