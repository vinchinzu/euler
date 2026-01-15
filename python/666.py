"""Project Euler Problem 666: Polymorphic Bacteria.

Given a species of bacteria with N types, starting with a single bacterium of
type 0, and such that a bacterium of type i has a probability of either (1)
dying, (2) cloning, (3) changing into type 2i (mod N), (4) splitting into 3
of type (i²+1) (mod N), and (5) spawning a new type i+1 (mod N) (depending on
the values of the random sequence R), find the probability that the bacterium
eventually dies out.

Let P_i be the probability that a single bacterium of type i eventually dies
out. We can write an equation for each P_i, based on each possible action. We
solve the equations by setting arbitrary initial values and repeatedly
evaluating better estimates until they converge.
"""

from __future__ import annotations


def feq(a: float, b: float, eps: float = 1e-10) -> bool:
    """Check if two floats are approximately equal."""
    return abs(a - b) < eps


def solve() -> float:
    """Solve Problem 666."""
    N = 500
    K = 10

    # Generate random sequence R
    R = []
    r = 306
    for _ in range(N * K):
        R.append(r)
        r = (r * r) % 10007

    probs = [0.5] * N
    while True:
        new_probs = [0.0] * N
        for i in range(N):
            for j in range(K):
                q = R[i * K + j] % 5
                if q == 0:
                    new_probs[i] += 1.0 / K
                elif q == 1:
                    new_probs[i] += (probs[i] ** 2) / K
                elif q == 2:
                    new_probs[i] += probs[(2 * i) % N] / K
                elif q == 3:
                    new_probs[i] += (probs[(i * i + 1) % N] ** 3) / K
                elif q == 4:
                    new_probs[i] += (
                        probs[i] * probs[(i + 1) % N] / K
                    )

        if feq(probs[0], new_probs[0]):
            break
        probs = new_probs

    return probs[0]


def main() -> float:
    """Main entry point."""
    result = solve()
    print(f"{result:.8f}")
    return result


if __name__ == "__main__":
    main()
