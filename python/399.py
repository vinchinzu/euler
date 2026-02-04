"""Approximate solver for Project Euler Problem 399 in Python 3.12.

This is a direct, idiomatic translation of the provided Ruby script.

Notes and limitations
---------------------
- Relies on Wall's conjecture as in the original description: for every prime p,
  the first Fibonacci number divisible by p is not divisible by p**2.
- Uses a truncated product over primes up to MAX_PRIME to approximate the
  density of squarefree Fibonacci numbers. This is heuristic and mirrors the
  Ruby code; it is not a proven algorithm.
- The rank_of_apparition implementation is a pragmatic search adapted from the
  Ruby version and is not guaranteed optimal.

The module exposes a small public API:
- solve(n: int) -> tuple[str, str]
    Compute an approximation to the nth squarefree Fibonacci number, returning
    (last_16_digits, scientific_notation_string).

The main() function runs the computation for the configured N when executed as
"__main__".
"""

from __future__ import annotations

import math
from dataclasses import dataclass
from typing import List, Sequence, Tuple

M: int = 10**16
N: int = 100_000_000
MAX_PRIME: int = 100_000  # Sufficient for good density approximation (heuristic)
PHI: float = (1.0 + math.sqrt(5.0)) / 2.0
LOG10_PHI: float = math.log10(PHI)
LOG10_SQRT5: float = math.log10(math.sqrt(5.0))


@dataclass(frozen=True)
class Matrix2x2:
    """Simple immutable 2x2 matrix for Fibonacci exponentiation modulo m."""

    a11: int
    a12: int
    a21: int
    a22: int

    def mult(self, other: "Matrix2x2", mod: int) -> "Matrix2x2":
        """Return (self * other) modulo mod."""

        return Matrix2x2(
            (self.a11 * other.a11 + self.a12 * other.a21) % mod,
            (self.a11 * other.a12 + self.a12 * other.a22) % mod,
            (self.a21 * other.a11 + self.a22 * other.a21) % mod,
            (self.a21 * other.a12 + self.a22 * other.a22) % mod,
        )

    @staticmethod
    def identity() -> "Matrix2x2":
        """Return 2x2 identity matrix."""

        return Matrix2x2(1, 0, 0, 1)


def matrix_pow(mat: Matrix2x2, exp: int, mod: int) -> Matrix2x2:
    """Fast exponentiation of a 2x2 matrix modulo mod."""

    result = Matrix2x2.identity()
    base = mat
    e = exp
    while e > 0:
        if e & 1:
            result = result.mult(base, mod)
        base = base.mult(base, mod)
        e >>= 1
    return result


def fib_mod(k: int, mod: int) -> int:
    """Return F_k modulo mod using fast doubling via matrix exponentiation.

    F_0 = 0, F_1 = 1.
    """

    if k == 0:
        return 0
    if k == 1:
        return 1 % mod
    pow_matrix = matrix_pow(Matrix2x2(1, 1, 1, 0), k - 1, mod)
    return pow_matrix.a11 % mod


def _prime_sieve(limit: int) -> List[int]:
    """Return list of primes up to and including limit using a sieve.

    This replaces Ruby's `Prime.each` from the standard library.
    """

    if limit < 2:
        return []
    sieve = bytearray(b"\x01") * (limit + 1)
    sieve[0:2] = b"\x00\x00"
    for p in range(2, int(limit**0.5) + 1):
        if sieve[p]:
            step = p
            start = p * p
            sieve[start : limit + 1 : step] = b"\x00" * ((limit - start) // step + 1)
    return [i for i, is_prime in enumerate(sieve) if is_prime]


def rank_of_apparition(p: int) -> int:
    """Compute the rank of apparition of prime p in the Fibonacci sequence.

    The rank of apparition z(p) is the smallest positive integer k such that p
    divides F_k.

    Notes:
    - For p=2 and p=5, values are hard-coded as in the original script.
    - For other primes, this mirrors the Ruby approach, trying a small set of
      plausible divisors followed by a bounded linear search.
    - This is not mathematically tight but is adequate within the heuristic
      nature of the original code.
    """

    if p == 2:
        return 3  # F_3 = 2
    if p == 5:
        return 5  # F_5 = 5

    candidates: List[int] = []
    if (p - 1) % 2 == 0:
        candidates.append(p - 1)
    if (p + 1) % 2 == 0:
        candidates.append(p + 1)
    if 2 * (p + 1) <= 4 * p:
        candidates.append(2 * (p + 1))

    for d in sorted(set(candidates)):
        if fib_mod(d, p) == 0:
            return d

    d = 1
    limit = 4 * p
    while d <= limit:
        if fib_mod(d, p) == 0:
            return d
        d += 1

    # In practice this should not occur for our parameter range. If it does,
    # caller should handle the error; it means our heuristic failed.
    raise ValueError(f"Could not find rank of apparition for prime {p}")


def compute_density(max_prime: int = MAX_PRIME, verbose: bool = False) -> float:
    """Estimate density of squarefree Fibonacci numbers via heuristic product.

    This computes:
        density ≈ ∏_p (1 - 1 / z(p)),
    over primes up to ``max_prime``.
    """

    primes = _prime_sieve(max_prime)
    density = 1.0

    if verbose:
        print(f"Computing density using {len(primes)} primes up to {max_prime}...")

    for i, p in enumerate(primes):
        if verbose and i % 10_000 == 0:
            print(f"Processed {i}/{len(primes)} primes...")
        z_p = rank_of_apparition(p)
        density *= 1.0 - 1.0 / float(z_p)

    if verbose:
        print(f"Computed density: {density:.6f}")
    return density


def estimate_index(n: int, density: float) -> int:
    """Return a loose estimate for index k of the nth squarefree Fibonacci.

    This reproduces the Ruby behavior of slightly inflating the estimate.
    """

    k_estimate = math.ceil(n / density)
    return int(k_estimate * 1.05)


def find_exact_index(n: int, density: float, verbose: bool = False) -> int:
    """Heuristically find index k with approximately n squarefree Fibonacci.

    The original Ruby code used density * k as an approximation and a binary
    search. Here we replicate that. Note this does not check actual
    squarefreeness; it is only consistent with the original heuristic.
    """

    if n < 1:
        raise ValueError("n must be positive")

    low = 1
    high = int(n / density * 1.2)

    if verbose:
        print(f"Binary searching for index k where count(k) >= {n}...")

    while low < high:
        mid = (low + high) // 2
        approx_count = int(mid * density)
        if approx_count < n:
            low = mid + 1
        else:
            high = mid

    k = low
    while int(k * density) < n:
        k += 1

    if verbose:
        print(f"Estimated index k = {k}")

    return k


def compute_last_16_digits(k: int) -> str:
    """Return the last 16 digits of F_k as zero-padded decimal string."""
    f_k_mod = fib_mod(k, M)
    return f"{f_k_mod:016d}"


def compute_scientific_notation(k: int) -> str:
    """Return scientific-notation approximation for F_k as '<mantissa>e<exp>'."""

    log10_f = k * LOG10_PHI - LOG10_SQRT5
    exponent = int(math.floor(log10_f))
    fractional_part = log10_f - exponent
    mantissa = 10 ** fractional_part

    mantissa_rounded = round(mantissa * 10) / 10.0
    if mantissa_rounded >= 10.0:
        mantissa_rounded = 1.0
        exponent += 1

    return f"{mantissa_rounded}e{exponent}"


def _small_case_solution(n: int) -> Tuple[str, str]:
    """Handle very small n using hard-coded values as in the Ruby script."""

    if not (1 <= n <= 13):  # The Ruby logic only covers up to 13.
        raise ValueError("n out of supported range for small-case handler")

    small_cases = [
        None,
        "0000000000000001",  # F_1 = 1
        "0000000000000001",  # F_2 = 1
        "0000000000000002",  # F_3 = 2
        "0000000000000003",  # F_4 = 3
        "0000000000000005",  # F_5 = 5
        "0000000000000013",  # F_7 = 13
        "0000000000000021",  # F_8 = 21
        "0000000000000034",  # F_9 = 34
        "0000000000000055",  # F_10 = 55
        "0000000000000089",  # F_11 = 89
        "0000000000000233",  # F_13 = 233
        "0000000000000377",  # F_14 = 377
    ]

    # Map index n (nth squarefree Fibonacci) to Fibonacci value F_k.
    if n in (1, 2):
        f_n = 1
    elif n == 3:
        f_n = 2
    elif n == 4:
        f_n = 3
    elif n == 5:
        f_n = 5
    elif n == 6:
        f_n = 13
    elif n == 7:
        f_n = 21
    elif n == 8:
        f_n = 34
    elif n == 9:
        f_n = 55
    elif n == 10:
        f_n = 89
    elif n == 11:
        f_n = 233
    elif n == 12:
        f_n = 377
    elif n == 13:
        f_n = 610
    else:  # pragma: no cover - defensive; logically unreachable
        raise ValueError("Invalid small case")

    if f_n < 10:
        sci = f"{f_n}.0e0"
    else:
        log10_f = math.log10(float(f_n))
        exponent = int(math.floor(log10_f))
        mantissa = 10 ** (log10_f - exponent)
        mantissa_rounded = round(mantissa * 10) / 10.0
        sci = f"{mantissa_rounded}e{exponent}"

    return small_cases[n], sci


def solve(n: int) -> Tuple[str, str]:
    """Compute an approximation for the nth squarefree Fibonacci number.

    Returns a pair:
    - last 16 digits as zero-padded decimal string
    - scientific notation string (mantissa rounded to 1 decimal place)

    This matches the behavior and assumptions of the provided Ruby script,
    including its reliance on heuristic density estimation and Wall's
    conjecture; it does NOT rigorously verify squarefreeness.
    """

    if n < 1:
        raise ValueError("n must be positive")

    if n <= 13:
        return _small_case_solution(n)

    density = compute_density()
    k = find_exact_index(n, density)
    last_digits = compute_last_16_digits(k)
    sci_notation = compute_scientific_notation(k)
    return last_digits, sci_notation


if __name__ == "__main__":
    last_digits, sci = solve(N)
    print(f"{last_digits},{sci}")
