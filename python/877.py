# Project Euler Problem 877
#
# PROBLEM DESCRIPTION:
# <p>
# We use $x\oplus y$ for the bitwise XOR of $x$ and $y$.<br>
# Define the <dfn>XOR-product</dfn> of $x$ and $y$, denoted by $x \otimes y$, similar to a long multiplication in base $2$, except that the intermediate results are XORed instead of the usual integer addition.
# </p>
# <p>
# For example, $7 \otimes 3 = 9$, or in base $2$, $111_2 \otimes 11_2 = 1001_2$:
# </p><center>
# $$\begin{align*}
# \phantom{\otimes 111} 111_2 \\
# \otimes \phantom{1111} 11_2 \\
# \hline
# \phantom{\otimes 111} 111_2 \\
# \oplus \phantom{11} 111_2  \phantom{9} \\
# \hline
# \phantom{\otimes 11} 1001_2 \\
# \end{align*}$$
# </center>
# We consider the equation:
# <center>
# $$\begin{align}
# (a \otimes a) \oplus (2 \otimes a \otimes b) \oplus (b \otimes b) = 5
# \end{align}$$
# </center>
# 
# For example, $(a, b) = (3, 6)$ is a solution.
# 
# <p>
# Let $X(N)$ be the XOR of the $b$ values for all solutions to this equation satisfying $0 \le a \le b \le N$.<br> You are given $X(10)=5$.
# </p>
# <p>
# Find $X(10^{18})$.
# </p>
#
from __future__ import annotations

from typing import Iterator, Tuple

# Over GF(2)[x], the equation is A(x)^2 + x A(x) B(x) + B(x)^2 = 1 + x^2
# with x representing the bit-value 2. Multiplying any solution by the unit
# \omega satisfying \omega^2 = x \omega + 1 preserves the norm and therefore
# generates every solution. In integers this yields the recurrence
#   a_{n+1} = b_n
#   b_{n+1} = (b_n << 1) ^ a_n
# seeded with (a_0, b_0) = (0, 3). The number of pairs up to N is O(log N).

TARGET_N = 10**18


def solution_pairs(limit: int) -> Iterator[Tuple[int, int]]:
    """Yield all (a, b) solving the equation with 0 <= a <= b <= limit."""
    if limit < 0:
        raise ValueError("limit must be non-negative")

    a, b = 0, 3
    while b <= limit:
        yield a, b
        a, b = b, (b << 1) ^ a


def x_value(limit: int) -> int:
    """Compute X(limit), the XOR of b across all valid pairs up to limit."""
    total = 0
    for _, b in solution_pairs(limit):
        total ^= b
    return total


def solve() -> int:
    """Return X(10^18) derived via the closed-form recurrence."""
    return x_value(TARGET_N)


if __name__ == "__main__":
    print(solve())
