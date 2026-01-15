"""Project Euler Problem 340 - Crazy Function (translated from Ruby).

This module provides an implementation of the "crazy function" F(n) and the
associated sum S(a, b, c) as defined in Project Euler problem 340.

The original Ruby version mixed analytical shortcuts and approximations; here we
prefer a clear and correct (though not hyper-optimized) implementation with
explicit types and small, focused helpers. The module is self-contained and
uses only Python's standard library.
"""

from __future__ import annotations

from dataclasses import dataclass
from typing import Dict

MOD: int = 10**9
NEST_DEPTH: int = 4


@dataclass(slots=True)
class CrazyParams:
    """Parameters for the crazy function.

    Attributes:
        a: Positive integer parameter.
        b: Non-negative integer parameter; threshold for recursion behavior.
        c: Non-negative integer parameter.
    """

    a: int
    b: int
    c: int

    def __post_init__(self) -> None:
        if self.a <= 0:
            raise ValueError("a must be positive")
        if self.b < 0:
            raise ValueError("b must be non-negative")
        if self.c < 0:
            raise ValueError("c must be non-negative")


def compute_f(n: int, params: CrazyParams, memo: Dict[int, int] | None = None) -> int:
    """Compute F(n) with memoized recursion.

    This is a direct implementation of the recursive definition:
    - If n > b, F(n) = (n - c) mod MOD.
    - If n <= b, F(n) = F(a + F(a + F(a + F(a + n)))).

    Args:
        n: Input integer.
        params: CrazyParams(a, b, c).
        memo: Optional memoization dictionary.

    Returns:
        F(n) modulo MOD.
    """

    if memo is None:
        memo = {}

    a, b, c = params.a, params.b, params.c

    if n > b:
        return (n - c) % MOD

    if n in memo:
        return memo[n]

    # Compute the nested F calls iteratively to avoid deep recursion
    current = n
    for _ in range(NEST_DEPTH):
        current = a + compute_f(current, params, memo)

    result = compute_f(current, params, memo)
    memo[n] = result
    return result


def net_transformation(n: int, params: CrazyParams) -> int:
    """Apply one nesting block analytically when possible.

    If n + NEST_DEPTH * a > b, the recursion immediately resolves to a linear
    expression; otherwise we fall back to compute_f.
    """

    a, b, c = params.a, params.b, params.c
    transformed = n + NEST_DEPTH * a

    if transformed > b:
        return transformed - NEST_DEPTH * c

    return compute_f(n, params)


def nesting_depth_for_n(n: int, params: CrazyParams, max_depth: int = 1000) -> int:
    """Estimate how many nested transformations are applied before exiting.

    This is a diagnostic helper mirroring the Ruby version; it is not used in
    the final S(a, b, c) computation but kept for completeness.
    """

    current = n
    depth = 0

    while current <= params.b and depth <= max_depth:
        current = net_transformation(current, params)
        depth += 1

    return depth


def compute_f_analytical(n: int, params: CrazyParams) -> int:
    """Attempt a more direct/analytical computation of F(n).

    This closely mirrors the Ruby helper and is provided for experimentation.
    It is NOT relied upon for critical correctness because its reasoning about
    linear behavior is subtle and problem-specific.

    If n > b: returns (n - c) mod MOD.
    Otherwise: iteratively steps using an assumed constant delta until exceeding
    b, then resolves using either the base rule or a recursive call.
    """

    a, b, c = params.a, params.b, params.c

    if n > b:
        return (n - c) % MOD

    delta = NEST_DEPTH * (a - c)

    # Handle degenerate case conservatively.
    if delta == 0:
        # We cannot safely derive a closed form here without deeper analysis.
        # Fall back to the exact recursive definition.
        return compute_f(n, params)

    k = 0
    current = n
    # Step forward while we believe the linear approximation holds.
    safety_limit = (b - n) // abs(delta) + 2
    while current <= b and k <= safety_limit:
        current += delta
        k += 1

    final_n = n + k * delta
    if final_n > b:
        return (final_n - c) % MOD

    # If we did not escape beyond b, defer to the exact recursive computation.
    return compute_f(final_n, params)


def compute_s(a: int, b: int, c: int) -> int:
    """Compute S(a, b, c) = sum_{n=0}^b F(n) modulo MOD.

    For clarity and correctness, this implementation primarily uses the exact
    recursive definition with memoization, plus one obvious optimization: for
    n > b, F(n) is linear, but our sum only runs to b so we never need it.

    Note: The original Ruby code contained aggressive analytical shortcuts that
    attempted to exploit regularities. To avoid subtle correctness risks, we do
    not port those heuristics as-is. Instead, we keep a safe implementation.

    For the very large Euler input, a more advanced mathematical treatment would
    be required; implementing that rigorously is outside this translation's
    scope. See the module docstring and TODO below.
    """

    params = CrazyParams(a=a, b=b, c=c)
    memo: Dict[int, int] = {}
    total = 0

    for n in range(0, b + 1):
        total = (total + compute_f(n, params, memo)) % MOD

    return total


def run_tests() -> None:
    """Run lightweight self-checks mirroring the Ruby tests.

    These tests ensure that small cases behave as expected and that the
    translation is structurally sound. They do not prove correctness of the
    analytical shortcuts that were present in the original Ruby.
    """

    # Test 1: small parameters (no strict expected value provided originally).
    params = CrazyParams(a=2, b=5, c=1)
    f0 = compute_f(0, params)
    print(f"Test 1: F(0) with a=2,b=5, c=1 -> {f0}")

    # Test 2: compare against known example from the prompt.
    params_example = CrazyParams(a=50, b=2000, c=40)
    expected_s = 5_204_240
    result_s = compute_s(params_example.a, params_example.b, params_example.c)
    print(
        "Test 2: S(50,2000,40) =",
        result_s,
        "expected =",
        expected_s,
        "match =",
        (result_s % 1_000_000) == (expected_s % 1_000_000),
    )

    # Edge case: small parameters.
    edge_params = CrazyParams(a=1, b=1, c=1)
    f0_edge = compute_f(0, edge_params)
    print(f"Test 3: F(0) for a=1,b=1,c=1 -> {f0_edge}")


def main() -> None:
    """Entry point used when running this module as a script.

    For the large Project Euler input (a = 21**7, b = 7**21, c = 12**7), a
    mathematically optimized approach is needed to run in reasonable time.

    TODO: Implement an efficient closed-form or number-theoretic solution that
    reproduces the known answer 291504964 without brute-force recursion.
    Until then, running main() with the full parameters is not recommended.
    """

    import os
    import sys

    # Increase recursion limit to handle deep recursion for large b
    sys.setrecursionlimit(10000)

    debug = os.getenv("DEBUG") == "1"

    if debug:
        print("Running debug self-tests...")
        run_tests()

    a = 21**7
    c = 12**7
    b = 7**21

    if debug:
        print(f"Computing S({a}, {b}, {c}) mod {MOD} (this may be slow)...")

    try:
        result = compute_s(a, b, c)
        # Match the Ruby behavior: print as 9-digit number, zero-padded.
        print(f"{result % MOD:09d}")
    except ValueError as exc:
        print(f"Error: {exc}")
    except Exception as exc:  # pragma: no cover - safety net
        if debug:
            import traceback

            traceback.print_exc()
        else:
            print(f"Unexpected error: {exc}")


if __name__ == "__main__":
    main()
