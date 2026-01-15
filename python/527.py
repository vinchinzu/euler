"""Project Euler Problem 527: Randomized Binary Search.

Let B(n) be the number of expected guesses to find a random number in
[1, N] using binary search (including the final correct guess) and let
R(n) be the number of expected guesses if each guess is a random number
selected in the remaining valid range (also including the final correct
guess). Find R(N) - B(N).
"""

from __future__ import annotations

from typing import Dict


def harmonic(n: int) -> float:
    """Compute harmonic number H_n."""
    result = 0.0
    for i in range(1, n + 1):
        result += 1.0 / i
    return result


def B(n: int, cache: Dict[int, float]) -> float:
    """Compute B(n) using memoization."""
    if n in cache:
        return cache[n]
    if n <= 1:
        return 1.0

    # Binary search: guess middle
    guess = (n + 1) // 2
    # Too high: range [1, guess-1] with (guess-1) numbers
    # Too low: range [guess+1, n] with (n-guess) numbers
    prob_high = (guess - 1) / n
    prob_low = (n - guess) / n

    result = 1.0
    if guess > 1:
        result += prob_high * B(guess - 1, cache)
    if guess < n:
        result += prob_low * B(n - guess, cache)

    cache[n] = result
    return result


def solve() -> str:
    """Solve Problem 527."""
    N = 10**10
    cache: Dict[int, float] = {}
    B_val = B(N, cache)
    R_val = 2 * harmonic(N) * (N + 1) / N - 3
    ans = R_val - B_val
    return f"{ans:.8f}"


def main() -> str:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
