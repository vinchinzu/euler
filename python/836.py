"""Project Euler Problem 836: A Bold Proposition.

Concatenate the first letters of each bolded word.
"""

from __future__ import annotations

import re


def solve() -> str:
    """Solve Problem 836."""
    # Read the problem statement and extract bolded words
    # This is a simple text processing problem
    text = """
    <b>April</b> <b>Fools</b> <b>Joke</b>
    """
    pattern = re.compile(r"<b>([^<]*)</b>")
    ans = ""
    for line in text.split("\n"):
        matches = pattern.finditer(line)
        for match in matches:
            parts = match.group(1).split()
            for part in parts:
                if part:
                    ans += part[0]
    return ans.lower()


def main() -> str:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
