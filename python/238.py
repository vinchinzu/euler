"""Project Euler Problem 238: Infinite string tour.

Create a sequence using the Blum Blum Shub generator:
  s_0 = 14025256, s_{n+1} = s_n^2 mod 20300713
Concatenate to form infinite string w = s_0 s_1 s_2 ...

For positive integer k, p(k) is the starting position of the earliest
substring of w with digit sum equal to k (or 0 if none exists).

Find sum of p(k) for k = 1 to 2*10^15.

Key insight: The BBS sequence is periodic (period P = 2534198 terms).
The digit string repeats with period L digits and digit-sum D per period.
Therefore p(k) depends only on k mod D.

For each residue r mod D, f(r) = min position p such that some substring
starting at p has digit sum congruent to r mod D. This equals:
  f(r) = 1 + min{a : (C[a] + r) mod D in S}
where C[a] = cumulative digit sum at position a (mod D), and
S = {C[b] mod D : b in [0, L)}.

The answer is: Q * sum(f(r) for all r) + sum(f(r) for r in [1, N mod D])
where Q = N // D.
"""

import numpy as np


def solve():
    S0 = 14025256
    M = 20300713
    N = 2 * 10**15

    # Phase 1: Generate BBS sequence and extract digits
    s = S0
    parts = []
    while True:
        parts.append(str(s))
        s = (s * s) % M
        if s == S0:
            break

    w = ''.join(parts)
    L = len(w)
    digits = np.frombuffer(w.encode(), dtype=np.uint8) - ord('0')
    D = int(digits.sum())

    # Phase 2: Compute cumulative digit sums mod D and find first occurrences
    cumsum = np.empty(L, dtype=np.int64)
    cumsum[0] = 0
    np.cumsum(digits[:-1], out=cumsum[1:])
    C_mod = cumsum % D

    # unique_vals: sorted distinct values of C_mod
    # first_indices: position of first occurrence of each unique value
    unique_vals, first_indices = np.unique(C_mod, return_index=True)

    # Sort by first occurrence position (processing order)
    order = np.argsort(first_indices)
    S_vals_ordered = unique_vals[order]
    first_pos_ordered = first_indices[order]
    K = len(unique_vals)

    Q = N // D
    R = N % D

    covered = np.zeros(D, dtype=np.bool_)
    total_F = 0
    partial_G = 0
    total_covered = 0

    # Phase 3a: Full sweeps over all S values for early steps
    # Each step processes |S| ~ 17.2M elements via numpy vectorized ops.
    # After ~10 steps, coverage exceeds 90%, so we switch strategy.
    SWITCH_THRESHOLD = 10
    uv = unique_vals  # sorted by value

    for step in range(min(K, SWITCH_THRESHOLD)):
        pos = int(first_pos_ordered[step])
        v = int(S_vals_ordered[step])

        # Candidate residues: (s - v) mod D for each s in S
        shifted = uv - v
        shifted[shifted < 0] += D

        new_mask = ~covered[shifted]
        new_count = int(new_mask.sum())

        if new_count > 0:
            new_residues = shifted[new_mask]
            covered[new_residues] = True
            f_val = pos + 1
            total_F += f_val * new_count
            total_covered += new_count

            partial_mask = (new_residues >= 1) & (new_residues <= R)
            partial_count = int(partial_mask.sum())
            if partial_count > 0:
                partial_G += f_val * partial_count

        if total_covered >= D:
            break

    # Phase 3b: For remaining uncovered residues, check each against S_bool.
    # This is efficient because the number of uncovered residues decreases rapidly.
    if total_covered < D:
        uncovered_residues = np.where(~covered)[0].astype(np.int64)

        S_bool = np.zeros(D, dtype=np.bool_)
        S_bool[uv] = True

        for step_offset in range(SWITCH_THRESHOLD, K):
            pos = int(first_pos_ordered[step_offset])
            v = int(S_vals_ordered[step_offset])

            targets = (uncovered_residues + v) % D
            hits = S_bool[targets]
            new_count = int(hits.sum())

            if new_count > 0:
                new_residues = uncovered_residues[hits]
                covered[new_residues] = True
                f_val = pos + 1
                total_F += f_val * new_count
                total_covered += new_count

                partial_mask = (new_residues >= 1) & (new_residues <= R)
                partial_count = int(partial_mask.sum())
                if partial_count > 0:
                    partial_G += f_val * partial_count

                uncovered_residues = uncovered_residues[~hits]

            if total_covered >= D:
                break

    return Q * total_F + partial_G


if __name__ == "__main__":
    print(solve())
