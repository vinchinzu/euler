"""Project Euler Problem 761: Runner and Swimmer.

Find the critical speed V for a runner running around an N-sided pool to
always be able to prevent a swimmer in the pool swimming with speed 1 from
escaping at the edge before the runner gets there.

See section 4 of the referenced paper. Let θ = π/N and K be the largest
k for which sin(kθ) - (k+N)tan(θ)cos(kθ) is negative. Then if we let
α = 1/2 (Kθ + cos⁻¹( 2sin(Kθ) / (K+N)tan(θ) - cos(Kθ) )), the answer
is V=1/cos(α).
"""

from __future__ import annotations

import math


def solve() -> str:
    """Solve Problem 761."""
    N = 6
    t = math.pi / N
    K = 0

    # Find largest K for which sin(kθ) - (k+N)tan(θ)cos(kθ) < 0
    while math.sin(K * t) - (K + N) * math.tan(t) * math.cos(K * t) < 0:
        K += 1
    K -= 1

    # Compute α
    numerator = 2 * math.sin(K * t)
    denominator = (K + N) * math.tan(t)
    a = (K * t + math.acos(numerator / denominator - math.cos(K * t))) / 2

    ans = 1 / math.cos(a)
    return f"{ans:.8f}"


def main() -> str:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
