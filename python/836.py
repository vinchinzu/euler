"""Project Euler Problem 836: A Bold Proposition.

Concatenate the first letters of each bolded word.
"""

from __future__ import annotations

import re


def solve() -> str:
    """Solve Problem 836.

    The problem is an April Fools joke (published 2023-04-01).
    It presents mathematical jargon with specific bolded words/phrases:
      - affine plane
      - radically integral local field
      - open oriented line section
      - jacobian
      - orthogonal kernel embedding
    The answer is the concatenation of the first letters of each bolded word.
    """
    # The problem statement contains these bolded phrases in order.
    # We extract the first letter of each individual word within them.
    text = (
        '<b>affine plane</b> over a <b>radically integral local field</b> '
        '<b>open oriented line section</b> '
        '<b>jacobian</b> associated to the <b>orthogonal kernel embedding</b>'
    )
    pattern = re.compile(r"<b>([^<]*)</b>")
    ans = ""
    for match in pattern.finditer(text):
        for word in match.group(1).split():
            if word:
                ans += word[0]
    return ans


def main() -> str:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
