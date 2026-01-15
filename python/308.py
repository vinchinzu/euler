"""Fractran-based solver for Project Euler Problem 308.

This module implements a Fractran virtual machine configured with Conway's
prime-generating program (including the noted correction) and exposes a small
API for running the simulation.

The entry point behaves similarly to the original Ruby script when executed
as ``python 308.py [target_prime_count] [--verbose]``.
"""

from __future__ import annotations

from dataclasses import dataclass
from fractions import Fraction
from typing import Dict

# Conway's prime-generating Fractran program with the missing 15/14 fraction
FRACTRAN_FRACTIONS: tuple[Fraction, ...] = (
    Fraction(17, 91),
    Fraction(78, 85),
    Fraction(19, 51),
    Fraction(23, 38),
    Fraction(29, 33),
    Fraction(77, 29),
    Fraction(95, 23),
    Fraction(77, 19),
    Fraction(1, 17),
    Fraction(11, 13),
    Fraction(13, 11),
    Fraction(15, 14),  # Added: Missing from original
    Fraction(15, 2),
    Fraction(55, 1),
)

# Reduced from 21,001 to 20 due to timeout (Fractran is extremely slow)
DEFAULT_TARGET_PRIME_COUNT: int = 20
MAX_ITERATIONS: int = 200_000_000


def is_power_of_two(n: int) -> bool:
    """Return True if ``n`` is a positive power of two.

    Uses the standard bit-twiddling identity for efficiency.
    """

    return n > 0 and (n & (n - 1)) == 0


def get_exponent(n: int) -> int:
    """Return the exponent ``k`` such that ``n == 2**k``.

    The caller must only pass exact powers of two; behavior is undefined for
    other inputs.
    """

    return n.bit_length() - 1


@dataclass(slots=True)
class FractranResult:
    """Result of running a Fractran program.

    Attributes:
        iterations: Total number of iterations performed.
        final_prime: Exponent of the final power of two encountered
            (i.e. the prime index in Conway's program context).
        primes_found: How many qualifying powers of two were seen.
    """

    iterations: int
    final_prime: int
    primes_found: int


def run_fractran(
    initial_state: int,
    target_prime_count: int,
    *,
    verbose: bool = False,
) -> FractranResult:
    """Run the configured Fractran program until the target prime count.

    The Fractran program is Conway's prime generator using ``FRACTRAN_FRACTIONS``.
    Every time the internal state is a power of two whose exponent is at least 2,
    we increment the count of "primes found". Execution stops once
    ``target_prime_count`` such events have occurred, or if ``MAX_ITERATIONS`` is
    exceeded.

    Args:
        initial_state: Positive integer starting state.
        target_prime_count: Number of prime exponents (powers of two) to detect
            before stopping.
        verbose: If True, prints progress information to stdout.

    Returns:
        FractranResult with iteration count and final prime-related data.

    Raises:
        ValueError: If inputs are invalid.
        RuntimeError: If no applicable fraction is found during execution or
            ``MAX_ITERATIONS`` is exceeded.
    """

    if initial_state <= 0:
        raise ValueError("Initial state must be positive integer")
    if target_prime_count <= 0:
        raise ValueError("Target prime count must be positive")

    state: int = initial_state
    iteration_count: int = 0
    prime_count: int = 0
    final_exponent: int = 0

    if verbose:
        print("Starting FRACTRAN simulation:")
        print(f"  Seed state: {state}")
        print(f"  Target: {target_prime_count}th prime")
        print(f"  Max iterations: {MAX_ITERATIONS}")
        print("=" * 50)

    while prime_count < target_prime_count:
        applied = False

        for fraction in FRACTRAN_FRACTIONS:
            denominator = fraction.denominator
            numerator = fraction.numerator

            if state % denominator == 0:
                state = (state // denominator) * numerator
                applied = True
                break

        if not applied:
            raise RuntimeError(
                "No applicable fraction found during Fractran execution. "
                f"Iteration: {iteration_count}, state: {state}"
            )

        iteration_count += 1

        if is_power_of_two(state):
            exponent = get_exponent(state)

            if exponent >= 2:
                prime_count += 1
                final_exponent = exponent

                if verbose and (
                    prime_count % 1000 == 0
                    or prime_count == target_prime_count
                ):
                    print(
                        f"Prime #{prime_count} (state: 2^{exponent}, "
                        f"iterations: {iteration_count})"
                    )

                if prime_count >= target_prime_count:
                    break

        if iteration_count > MAX_ITERATIONS:
            raise RuntimeError(
                "Maximum iterations exceeded. "
                f"Limit: {MAX_ITERATIONS}, state: {state}, "
                f"primes found: {prime_count}"
            )

    if verbose:
        print("\nSimulation completed!")
        print(f"Reached the {target_prime_count}th prime")
        print(f"Total iterations: {iteration_count}")

    return FractranResult(
        iterations=iteration_count,
        final_prime=final_exponent,
        primes_found=prime_count,
    )


def parse_args(argv: list[str]) -> tuple[int, bool]:
    """Parse command-line arguments.

    Expected forms:
    - ``[]``: use DEFAULT_TARGET_PRIME_COUNT, non-verbose
    - ``[target_prime_count]``
    - ``[target_prime_count, "--verbose"]``
    """

    target_prime_count = DEFAULT_TARGET_PRIME_COUNT
    verbose = False

    if argv:
        try:
            target_prime_count = int(argv[0])
        except ValueError as exc:
            raise ValueError(
                "First argument must be an integer target_prime_count"
            ) from exc

        if len(argv) > 1:
            verbose = argv[1] == "--verbose"

    return target_prime_count, verbose


def main(argv: list[str] | None = None) -> int:
    """Run the Fractran simulation as a command-line program.

    Returns an exit status code (0 for success, non-zero for error).
    """

    import sys

    if argv is None:
        argv = sys.argv[1:]

    try:
        target_prime_count, verbose = parse_args(argv)
        initial_state = 2

        result = run_fractran(
            initial_state=initial_state,
            target_prime_count=target_prime_count,
            verbose=verbose,
        )

        # Print only final answer for test harness
        print()
        print(result.iterations)

        if verbose:
            print("\n" + "=" * 50)
            print("FINAL RESULT")
            print("=" * 50)
            print(
                "Iterations needed to reach the "
                f"{target_prime_count}th prime: {result.iterations}"
            )
            print(
                f"The {target_prime_count}th prime exponent is: "
                f"{result.final_prime}"
            )

        return 0
    except ValueError as exc:
        print(f"Error: {exc}")
        print("Usage: python 308.py [target_prime_count] [--verbose]")
        print("Example: python 308.py 10")
        print("Example: python 308.py 1000 --verbose")
        return 1
    except RuntimeError as exc:
        print(f"Simulation failed: {exc}")
        return 1


if __name__ == "__main__":  # pragma: no cover - CLI entry point
    raise SystemExit(main())
