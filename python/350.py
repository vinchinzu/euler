"""Project Euler Problem 350 - Python 3.12 implementation.

This module provides an efficient implementation for counting lists of natural numbers
subject to constraints on their greatest common divisor (gcd) and least common
multiple (lcm).

Core API:
- f(G, L, N, mod): compute the number of lists of size N with gcd >= G and lcm <= L,
  modulo mod.

The implementation mirrors the number-theoretic technique from the Ruby version,
using:
- a sieve to compute the Möbius function mu(n), divisor-counting function tau(n),
  and the Mertens prefix sums S(n) = sum_{k<=n} mu(k), for n up to M = L // G.
- a summation formula leveraging these arithmetic functions.

The module is self-contained and uses only the Python standard library.
"""

from __future__ import annotations

from time import perf_counter
from typing import List, Tuple


MOD: int = 101**4  # 104060401


def pow_mod(base: int, exp: int, mod: int) -> int:
    """Efficient modular exponentiation.

    Handles very large exponents. Returns 0 immediately if base is 0.
    For non-trivial mod, reduces exponent modulo (mod - 1) following
    Euler's theorem when applicable (here used as in the original code).
    """

    if mod <= 0:
        msg = "mod must be positive"
        raise ValueError(msg)

    if base == 0:
        return 0

    base %= mod

    # Euler's theorem-like reduction, mirroring the Ruby implementation.
    if mod > 1:
        exp %= mod - 1

    result = 1
    while exp > 0:
        if exp & 1:
            result = (result * base) % mod
        base = (base * base) % mod
        exp >>= 1
    return result


def sieve_mertens(M: int) -> Tuple[List[int], List[int], List[int]]:
    """Compute mu, tau, and Mertens prefix sums up to M.

    Returns three lists (each of length M + 1):
    - mu[n]: Möbius function values.
    - tau[n]: number of positive divisors of n.
    - S[n]: prefix sums of mu, i.e. S[n] = sum_{k=1..n} mu[k].

    The implementation closely follows the structure of the Ruby version
    while using straightforward Python loops. It is aimed at correctness and
    clarity rather than being a fully optimized analytic number theory sieve.
    """

    if M < 1:
        return [], [], []

    mu: List[int] = [0] * (M + 1)
    tau: List[int] = [0] * (M + 1)
    is_composite: List[bool] = [False] * (M + 1)
    smallest_prime_factor: List[int] = [0] * (M + 1)

    mu[1] = 1
    # tau[1] will be computed by the loop below

    for i in range(2, M + 1):
        if not is_composite[i]:
            smallest_prime_factor[i] = i
            mu[i] = -1
            # tau[i] will be computed by the loop below

            for j in range(i * 2, M + 1, i):
                is_composite[j] = True
                if smallest_prime_factor[j] == 0:
                    smallest_prime_factor[j] = i

                if (j // i) % i == 0:
                    mu[j] = 0
                else:
                    mu[j] = -mu[j // i]

    # Compute tau using the divisor-counting definition.
    for i in range(1, M + 1):
        for j in range(i, M + 1, i):
            tau[j] += 1

    S: List[int] = [0] * (M + 1)
    for i in range(1, M + 1):
        S[i] = S[i - 1] + mu[i]

    return mu, tau, S


def f(G: int, L: int, N: int, mod: int = MOD, verbose: bool = False) -> int:
    """Compute f(G, L, N) modulo mod.

    f(G, L, N) is the number of lists (sequences) of length N consisting of natural
    numbers such that the gcd of all entries is at least G and the lcm of all
    entries is at most L.
    """

    if mod <= 0:
        msg = "mod must be positive"
        raise ValueError(msg)

    if G <= 0 or L <= 0 or N <= 0:
        return 0

    # The empty list is conceptually excluded (N >= 1 for the problem).
    # If needed, this function could be extended to define f for N == 0.

    M = L // G
    if M <= 0:
        return 0

    if verbose:
        print(f"Computing sieve for M = {M}...")

    _, tau, S = sieve_mertens(M)

    result = 0
    for d in range(1, M + 1):
        tau_power = pow_mod(tau[d], N, mod)
        m_div_d = M // d

        mertens = S[m_div_d]
        mertens_mod = (mertens % mod + mod) % mod

        contribution = (tau_power * mertens_mod) % mod
        result = (result + contribution) % mod

    return result


def verify_solution() -> None:
    """Run basic consistency checks against known small cases."""

    mod = MOD

    result1 = f(10, 100, 1, mod)
    print(f"f(10, 100, 1) = {result1} (expected: 10)")
    if result1 == 10:
        print("\u2713 PASS")

    result2 = f(10, 100, 2, mod)
    print(f"f(10, 100, 2) = {result2} (expected: 48)")
    if result2 == 48:
        print("\u2713 PASS")

    result3 = f(10, 100, 3, mod)
    print(f"f(10, 100, 3) = {result3} (expected: 202)")
    if result3 == 202:
        print("\u2713 PASS")


def main() -> None:
    """Execute verification tests and the main Project Euler 350 computation."""

    G = 10**6
    L = 10**12
    N = 10**18
    mod = MOD

    print("Project Euler Problem 350")
    print(f"Computing f({G}, {L}, {N}) mod {mod}...")
    print(f"M = L/G = {L // G}")

    print("\n--- Verification ---")
    verify_solution()

    print("\n--- Main Computation ---")
    start = perf_counter()
    result = f(G, L, N, mod, verbose=True)
    elapsed = perf_counter() - start

    print(f"Result: {result}")
    print(f"Computation time: {elapsed:.2f} seconds")

    print("\n--- Edge Cases ---")
    print(f"f(1, 1, 1) = {f(1, 1, 1, mod)} (expected: 1)")
    print(f"f(1, 3, 2) = {f(1, 3, 2, mod)} (expected: 7)")
    print(f"f(5, 4, 1) = {f(5, 4, 1, mod)} (expected: 0)")


if __name__ == "__main__":  # pragma: no cover - CLI entrypoint
    main()
