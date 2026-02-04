"""Project Euler Problem 406: Guessing Game.

C(n, a, b) = minimum worst-case cost to guess a number in [1, n].
f(c) = max n guessable with budget c = 1 + f(c-a) + f(c-b).

Compute sum_{k=1}^{30} C(10^12, sqrt(k), sqrt(F_k)).
"""
from math import sqrt
import bisect

def fibonacci(k):
    """Compute k-th Fibonacci number (F_1=F_2=1)."""
    if k <= 2:
        return 1
    a, b = 1, 1
    for _ in range(k - 2):
        a, b = b, a + b
    return b

def C_func(n, a_val, b_val):
    """Compute C(n, a, b) where a and b are float values.

    Build sorted list of cost values c = i*a + j*b for non-negative i, j.
    For each c (in order), compute f(c) = 1 + f(c-a) + f(c-b).
    f(c-a) is the largest f-value for some c' <= c-a.
    Return smallest c where f(c) >= n.
    """
    # Generate all candidate cost values c = i*a + j*b up to a budget limit
    # We need f(c) >= n = 10^12. f grows roughly exponentially.
    # Max c needed is approximately log_phi(n) * max(a, b) ~ 60 * max(a, b)
    max_budget = 80.0 * max(a_val, b_val)

    # Generate sorted list of (cost, i, j) tuples
    costs = set()
    i = 0
    while i * a_val <= max_budget:
        j = 0
        while i * a_val + j * b_val <= max_budget:
            costs.add(i * a_val + j * b_val)
            j += 1
        i += 1

    sorted_costs = sorted(costs)

    # For each cost value, compute f using binary search on previous values
    # f_vals[k] = f(sorted_costs[k])
    f_vals = []
    eps = 1e-9

    for idx, c in enumerate(sorted_costs):
        if c < -eps:
            f_vals.append(0)
            continue

        # f(c-a): find largest c' <= c - a in sorted_costs
        target_a = c - a_val + eps
        pos_a = bisect.bisect_right(sorted_costs, target_a, 0, idx + 1) - 1
        fa = f_vals[pos_a] if pos_a >= 0 else 0

        # f(c-b): find largest c' <= c - b in sorted_costs
        target_b = c - b_val + eps
        pos_b = bisect.bisect_right(sorted_costs, target_b, 0, idx + 1) - 1
        fb = f_vals[pos_b] if pos_b >= 0 else 0

        f_c = 1 + fa + fb

        # Early termination check
        if f_c >= n:
            return c

        f_vals.append(f_c)

    # Should not reach here if max_budget is large enough
    return sorted_costs[-1]

def solve():
    n = 10**12
    total = 0.0

    for k in range(1, 31):
        a = sqrt(k)
        b = sqrt(fibonacci(k))
        c = C_func(n, a, b)
        total += c

    return total

if __name__ == "__main__":
    result = solve()
    print(f"{result:.8f}")
