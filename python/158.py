"""Project Euler Problem 158: Exploring strings for which only one character comes lexicographically after its neighbour to the left."""

ALPHABET_SIZE = 26


def binom(n: int, k: int) -> int:
    """Compute binomial coefficient C(n, k)."""
    if k < 0 or k > n:
        return 0
    k = min(k, n - k)
    num = 1
    den = 1
    for i in range(1, k + 1):
        num *= n - k + i
        den *= i
    return num // den


def eulerian_one(n: int) -> int:
    """Compute Eulerian number A(n, 1) = 2^n - n - 1."""
    if n < 2:
        return 0
    return (1 << n) - n - 1


def p_of_n(n: int) -> int:
    """Count permutations with exactly one ascent."""
    if n < 2:
        return 0
    return binom(ALPHABET_SIZE, n) * eulerian_one(n)


def main() -> int:
    """Main function."""
    max_val = 0
    for n in range(1, ALPHABET_SIZE + 1):
        val = p_of_n(n)
        if val > max_val:
            max_val = val
    return max_val


if __name__ == "__main__":
    print(main())
