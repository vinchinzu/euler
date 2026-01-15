"""Project Euler Problem 679: Free Farea.

Find the number of strings of length N, using characters in S, that has
exactly one of each of the given KEYWORDS.

We use straightforward memoization. At each point, only the last few
characters in the string matter. We track any keywords that are found, and
if any keywords are discovered twice, we exit that branch.
"""

from __future__ import annotations

from functools import lru_cache
from typing import FrozenSet


def solve() -> int:
    """Solve Problem 679."""
    N = 30
    S = "AEFR"
    KEYWORDS = frozenset(["FREE", "FARE", "AREA", "REEF"])

    @lru_cache(maxsize=None)
    def num_words(
        index: int, prefix: str, found_keywords: FrozenSet[str]
    ) -> int:
        """Count words with given prefix and found keywords."""
        if prefix in found_keywords:
            return 0
        new_found = set(found_keywords)
        if prefix in KEYWORDS:
            new_found.add(prefix)
        new_found = frozenset(new_found)

        if index == N:
            return 1 if new_found == KEYWORDS else 0

        result = 0
        for c in S:
            new_prefix = prefix + c
            # Trim prefix if all keywords are shorter
            max_keyword_len = max(len(kw) for kw in KEYWORDS)
            if len(new_prefix) > max_keyword_len:
                new_prefix = new_prefix[1:]
            result += num_words(index + 1, new_prefix, new_found)
        return result

    return num_words(0, "", frozenset())


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
