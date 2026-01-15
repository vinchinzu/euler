"""Project Euler Problem 640: Shut the Box.

Bob starts with cards (1, 2, ... 2N) face up on a table. At every turn, Bob
rolls two N-sided dice to get (x, y) and must flip either card x, y, or x+y.
Find the expected number of turns to flip all cards face down given an optimal
strategy.

We iteratively improve the expected number of turns starting from each of the
2^(2N) states of cards, until the probabilities do not change.
"""

from __future__ import annotations


def solve() -> float:
    """Solve Problem 640."""
    N = 6
    L = 1 << (2 * N)

    X = [0.0] * L

    while True:
        new_X = [0.0] * L
        for i in range(L - 1):
            new_X[i] = 1.0
            for x in range(1, N + 1):
                for y in range(x, N + 1):
                    card_to_flip = x - 1
                    best_prob = X[i ^ (1 << card_to_flip)]

                    prob = X[i ^ (1 << (y - 1))]
                    if prob < best_prob:
                        card_to_flip = y - 1
                        best_prob = prob

                    prob_xy = X[i ^ (1 << (x + y - 1))]
                    if prob_xy < best_prob:
                        card_to_flip = x + y - 1

                    multiplier = 1.0 if x == y else 2.0
                    new_X[i] += X[i ^ (1 << card_to_flip)] / (N * N) * multiplier

        if all(abs(X[i] - new_X[i]) < 1e-10 for i in range(L)):
            break
        X = new_X

    return X[0]


def main() -> float:
    """Main entry point."""
    result = solve()
    print(f"{result:.6f}")
    return result


if __name__ == "__main__":
    main()
