"""Project Euler Problem 664: An Infinite Game of Tag.

On one side of a dividing line on an infinite checker-board, all squares on
the d'th column have d^N tokens. In each move, select a token, move it to an
orthogonally adjacent square, and remove an orthogonally adjacent token.
Determine the farthest number of squares that a token can be moved past the
dividing line.

This is a variant of Conway's soldiers, with the same scoring function. The
tokens on the d'th column have a total score of d^N ϕ^(4-d). The answer is
the largest power of ϕ smaller than the total score of all tokens.
"""

from __future__ import annotations

import math


def log_sum(log_a: float, log_b: float) -> float:
    """Compute log(a + b) given log(a) and log(b), assuming a >= b."""
    if log_a == float("-inf"):
        return log_b
    if log_b == float("-inf"):
        return log_a
    if log_a >= log_b:
        return log_a + math.log1p(math.exp(log_b - log_a))
    else:
        return log_b + math.log1p(math.exp(log_a - log_b))


def feq(a: float, b: float, eps: float = 1e-10) -> bool:
    """Check if two floats are approximately equal."""
    return abs(a - b) < eps


def solve() -> int:
    """Solve Problem 664."""
    N = 1234567
    PHI = (1 + math.sqrt(5)) / 2

    log_sum_val = float("-inf")
    d = 1
    while True:
        log_val = N * math.log(d) - (d - 4) * math.log(PHI)
        new_log_sum = log_sum(log_sum_val, log_val)
        if feq(log_sum_val, new_log_sum):
            break
        log_sum_val = new_log_sum
        d += 1

    log_sum_val /= math.log(PHI)
    return int(log_sum_val)


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
