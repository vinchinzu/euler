"""Project Euler Problem 163: Cross-hatched triangles."""


def count_triangles(n: int) -> int:
    """Count triangles in cross-hatched grid."""
    if not isinstance(n, int) or n < 1:
        raise ValueError("n must be a positive integer (n >= 1)")

    total = 0
    # Upward triangles of size k
    for k in range(1, n + 1):
        rows = n - k + 1
        cols_per_row = k
        total += rows * cols_per_row
    # Downward triangles of size k
    for k in range(1, n):
        rows = n - k
        cols_per_row = k
        total += rows * cols_per_row
    return total


def main() -> int:
    """Main function."""
    return count_triangles(36)


if __name__ == "__main__":
    print(main())
