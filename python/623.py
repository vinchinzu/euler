"""Project Euler Problem 623: Lambda Terms.

Consider lambda terms built by the following rules:
- A single letter variable is a lambda term.
- If M and N are lambda terms, then so is the application of M onto N,
  denoted (MN).
- If x is a variable and M is a term, then so is the abstraction (λx.M).

Find the number of lambda terms with at most N characters, assuming that all
variables are bound in an abstraction, and renaming of variables is not
considered distinct.

We use dynamic programming on the number of characters and the number of bound
variables in that scope. If there is only a single character, then it must be
one of the bounded variables. Otherwise, if the number of characters is at
least 6, then we can use 5 of those characters on the abstraction boilerplate
(λx. ) and recurse on the remaining number of characters, remembering that we
now have an additional bound variable. Also, if the number of characters is at
least 4, then we can use 2 characters on the application boilerplate ( ) and
split the remaining characters into 2 terms to recurse on.
"""

from __future__ import annotations


def solve() -> int:
    """Solve Problem 623."""
    N = 2000
    M = 10**9 + 7

    cache: dict[tuple[int, int], int] = {}

    def num_terms(num_chars: int, num_bound: int) -> int:
        """Compute number of lambda terms."""
        if num_chars == 1:
            return num_bound
        key = (num_chars, num_bound)
        if key in cache:
            return cache[key]

        num_terms_val = 0
        if num_chars >= 6:
            num_terms_val = (num_terms_val + num_terms(num_chars - 5, num_bound + 1)) % M
        if num_chars >= 4:
            for num_left in range(1, (num_chars - 1) // 2 + 1):
                num_right = num_chars - 2 - num_left
                if num_left == num_right:
                    term_val = num_terms(num_left, num_bound)
                    num_terms_val = (num_terms_val + term_val * term_val) % M
                else:
                    num_terms_val = (
                        num_terms_val
                        + 2 * num_terms(num_left, num_bound) * num_terms(num_right, num_bound)
                    ) % M
        cache[key] = num_terms_val
        return num_terms_val

    ans = 0
    for num_chars in range(1, N + 1):
        ans = (ans + num_terms(num_chars, 0)) % M
    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
