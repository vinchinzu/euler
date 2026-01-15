"""Project Euler Problem 724: Drone Delivery.

N drones start at x=0 and are stationary. If at each second, a random drone's
speed is increased by 1 unit per second, find the expected number of units a
random drone moves before all drones start moving.

After k seconds, the total distance traveled is tr(k), regardless of the
chosen drones. However, there is some probability P(k) that the last drone is
chosen at the kth second. We can use Inclusion Exclusion to compute this
probability.

The expected distance of a single drone is E/N = N Σ_{i=1}^N Σ_{j=1}^i 1/(ij).
"""

from __future__ import annotations


def solve() -> int:
    """Solve Problem 724."""
    n = 10**8
    harmonics = [0.0] * (n + 1)
    for i in range(1, n + 1):
        harmonics[i] = harmonics[i - 1] + 1.0 / i

    ans = 0.0
    for i in range(1, n + 1):
        ans += harmonics[i] / i
    ans *= n

    return int(ans)


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
