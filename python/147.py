"""Project Euler Problem 147."""


def upright(m: int, n: int) -> int:
    """Count upright rectangles."""
    return (m * (m + 1) // 2) * (n * (n + 1) // 2)


def diagonal(m: int, n: int) -> int:
    """Count diagonal rectangles."""
    if m < n:
        return diagonal(n, m)
    return (n * ((2 * m - n) * (4 * n * n - 1) - 3)) // 6


def total_rectangles(m: int, n: int) -> int:
    """Total rectangles."""
    return upright(m, n) + diagonal(m, n)


def main() -> int:
    """Main function."""
    total_sum = 0
    for a in range(1, 48):
        for b in range(1, 44):
            total_sum += total_rectangles(a, b)
    return total_sum


if __name__ == "__main__":
    print(main())
