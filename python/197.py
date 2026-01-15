"""Project Euler Problem 197: Investigating the behaviour of a recursive sequence."""

CONSTANT = 30.403243784
PRECOMPUTE_STEPS = 1000
START_VALUE = -1.0


def sequence_values(limit: int) -> list[float]:
    """Generate u_n sequence values up to limit."""
    values: list[float] = [0.0] * (limit + 1)
    values[0] = START_VALUE
    for i in range(1, limit + 1):
        prev = values[i - 1]
        power = CONSTANT - prev * prev
        next_val = int(2.0 ** power) * 1e-9
        values[i] = next_val
    return values


VALUES = sequence_values(PRECOMPUTE_STEPS)
CYCLE = [VALUES[-2], VALUES[-1]]
BASE_INDEX = PRECOMPUTE_STEPS - 1


def u_n(n: int) -> float:
    """Fetch u_n using the precomputed sequence and 2-cycle behaviour."""
    if n <= PRECOMPUTE_STEPS:
        return VALUES[n]
    return CYCLE[(n - BASE_INDEX) % 2]


def main() -> str:
    """Main function."""
    target = 1_000_000_000_000
    sum_val = u_n(target) + u_n(target + 1)
    return f"{sum_val:.9f}"


if __name__ == "__main__":
    print(main())
