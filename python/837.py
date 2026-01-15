"""Project Euler Problem 837: Amidakuji.

Find the number of Amidakuji of three vertical lines, with A rungs between
the first two lines and B rungs between the last two lines, which form the
identity permutation.

From top to bottom, there are (A+B)/2 pairs of rungs. Suppose that t of
these pairs consist of two different rungs, i.e. one on the left and the
other on the right. Then t must be the same parity as A and B, because
the remaining rungs come in pairs either both on the left or both on the
right. There are then (A-t)/2 pairs of left rungs and (B-t)/2 pairs of
right rungs.

A pair of rungs both on the left or both on the right doesn't affect the
permutation. A left rung followed by a right rung shifts the permutation
in one direction, and a right rung followed by a left rung shifts the
permutation in the other. If there are r and s of these pairs respectively,
where r+s=t, then we must have 3|r-s.

So for a given t, there are r, s, (A-t)/2, and (B-t)/2) types of each
pair of rungs, and the number of ways to arrange them is
((A+B)/2)! / ( r! s! ((A-t)/2)! ((B-t)/2)! ). Therefore, summing over all
t (of the right parity), the total is:

S = Σ_t Σ_{r,s} ((A+B)/2)! / ( r! s! ((A-t)/2)! ((B-t)/2)! )
  = Σ_t ((A+B)/2)! / (((A-t)/2)! ((B-t)/2)! ) Σ_{r,s} 1/(r!s!)
  = ((A+B)/2)! Σ_t 1 / ( ((A-t)/2)! ((B-t)/2)! t! ) Σ_{r,s} nCr(t,r).

By induction, we can find that for a given t, f(t) = Σ_{r,s} nCr(t,r)
(where r,s are constrained by r+s=t and 3|r-s) satisfies the recurrence
f(t) = 4f(t-2) + 2. We can now sum the terms over all t.
"""

from __future__ import annotations


def factorial(n: int, mod: int) -> int:
    """Compute n! mod mod."""
    result = 1
    for i in range(1, n + 1):
        result = (result * i) % mod
    return result


def mod_inverse(a: int, mod: int) -> int:
    """Modular inverse."""
    return pow(a, -1, mod)


def mod_invs(n: int, mod: int) -> list[int]:
    """Precompute modular inverses for 1..n."""
    invs = [0] * (n + 1)
    invs[1] = 1
    for i in range(2, n + 1):
        invs[i] = (mod - (mod // i) * invs[mod % i] % mod) % mod
    return invs


def solve() -> int:
    """Solve Problem 837."""
    A = 123456789
    B = 987654321
    M = 1234567891

    mod_invs_list = mod_invs(A, M)
    term1 = mod_inverse(
        factorial((A - 1) // 2, M) * factorial((B - 1) // 2, M) % M, M
    )
    term2 = 0
    ans = 0

    for t in range(3, A + 1, 2):
        term1 = (
            term1
            * mod_invs_list[t - 1]
            % M
            * mod_invs_list[t]
            % M
            * ((A - t + 2) // 2)
            % M
            * ((B - t + 2) // 2)
            % M
        )
        term1 %= M
        term2 = (4 * term2 + 2) % M
        ans = (ans + term1 * term2) % M

    ans = ans * factorial((A + B) // 2, M) % M
    return ans


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
