# Project Euler Problem 930
#
# PROBLEM DESCRIPTION:
# <p>Given $n\ge 2$ bowls arranged in a circle, $m\ge 2$ balls are distributed amongst them.</p>
# 
# <p>Initially the balls are distributed randomly: for each ball, a bowl is chosen equiprobably and independently of the other balls. After this is done, we start the following process:</p>
# <ol>
# <li>Choose one of the $m$ balls equiprobably at random.</li>
# <li>Choose a direction to move - either clockwise or anticlockwise - again equiprobably at random.</li>
# <li>Move the chosen ball to the neighbouring bowl in the chosen direction.</li>
# <li>Return to step 1.</li>
# </ol>
# 
# <p>This process stops when all the $m$ balls are located in the same bowl. Note that this may be after zero steps, if the balls happen to have been initially distributed all in the same bowl.</p>
# 
# <p>Let $F(n, m)$ be the expected number of times we move a ball before the process stops. For example, $F(2, 2) = \frac{1}{2}$, $F(3, 2) = \frac{4}{3}$, $F(2, 3) = \frac{9}{4}$, and $F(4, 5) = \frac{6875}{24}$.</p>
# 
# <p>Let $G(N, M) = \sum_{n=2}^N \sum_{m=2}^M F(n, m)$. For example, $G(3, 3) = \frac{137}{12}$ and $G(4, 5) = \frac{6277}{12}$. You are also given that $G(6, 6) \approx 1.681521567954e4$ in scientific format with 12 significant digits after the decimal point.</p>
# 
# <p>Find $G(12, 12)$. Give your answer in scientific format with 12 significant digits after the decimal point.</p>
#
# ANALYSIS/REVIEW:
# ### Analysis Summary
#
# #### Correctness: 2/10
# The core mathematical setup for solving the expected value via linear equations (E[s] = 1 + sum prob * E[s'] for non-absorbing states) is conceptually sound, and the averaging over initial states with probability (1/n)^m is correct. Absorbing states (all balls in one bowl) are handled properly with E=0. However, the transition probabilities and next states are fundamentally incorrect: the code models moves as uniformly random to any bowl (looping over all n targets with prob 1/(m*2*n)), instead of deterministic moves to the actual neighboring bowl in the chosen direction on the circle. This leads to wrong equations and incorrect results (e.g., for F(2,2), it computes ~1.0 instead of 0.5; manual verification shows mixed states get E=2 instead of 1). The state indexing and generation are correct in principle, but the wrong transitions propagate errors. Even if fixed, the full system wouldn't solve the problem correctly for large n/m without symmetry reductions, as the model treats balls as distinguishable (correct) but doesn't exploit circle symmetries.
#
# #### Efficiency: 1/10
# The approach explodes combinatorially: n^m states (12^12 ≈ 8.9e12 for n=12, m=12) require generating/storing a list of all states (memory ~O(n^m * m)), building an n^m x n^m matrix (impossible, >10^25 entries), and solving it via LU decomposition (time/space O((n^m)^3) in worst case, but even storage fails). Recursive state generation will stack-overflow or timeout for m>10. The outer loop computes each F(n,m) independently without memoization or reuse. For small n/m (e.g., G(3,3)), it runs, but for G(12,12), it won't finish in any reasonable time/hardware (e.g., even n=5,m=5 is 3125 states, matrix ~10M entries, solvable but slow; scales exponentially). No optimization like sparse matrices (transitions are sparse: only 2m per state) or state space reduction (e.g., via rotational invariance or occupancy multisets). The verification print for n<=4,m<=5 is a nice touch but doesn't help scalability.
#
# #### Code Quality: 5/10
# The code is readable and well-structured: modular compute_f function, clear helpers (state_index, generate_states), uses Ruby's Matrix library appropriately, and outputs in the required scientific format. Comments are minimal but present; recursion for state generation is clean but inefficient. Issues include: no error handling (e.g., for large n/m, it will crash on memory); redundant loops in transitions (n per direction, unnecessary); global-like vars in loops; no modularity for circle logic (hardcoded assumptions). Ruby idioms are ok (e.g., each_with_index, dup for states), but performance-killers like full matrix (vs. sparse solver) and list storage hurt. The script is complete and executable for tiny inputs, with the problem statement as a comment.
#
# #### Edge Cases: 3/10
# Handles n=2,m=2 partially (computes but wrong value, as neighbors coincide with uniform random in n=2 circle). Absorbing initial states (prob >0) correctly average to 0 contribution. But fails for n=2 (circle wrap-around not implemented, though coincidental). Doesn't handle m=1 or n=1 (but problem n,m>=2). For all balls same initially: ok. For large n/m: crashes, no fallback. No test for examples (e.g., F(4,5)=6875/24≈286.458, but code would compute wrong). Circle modulo not implemented, so anticlockwise from 0 goes to wrong place (code doesn't compute neighbors at all). Balls in same bowl but moving one away: transitions wrong, so edge like almost-gathered states mishandled.
#
# #### Bugs: 2/10
# - **Major bug**: Transition logic ignores current position and direction; loops over arbitrary target_bowl 0..n-1 instead of computing neighbor = (state[ball] + (dir == 0 ? 1 : -1) + n) % n. Probabilities are wrong (1/(m*2*n) * n = 1/(m*2) per direction, but over wrong targets; should be exactly 1/(m*2) per (ball,dir) pair).
# - **Bug in matrix setup**: For self-transitions, it subtracts probs incorrectly because targets include current pos sometimes, but not based on actual move distance (should rarely stay unless n=1, but neighbor != current).
# - **Efficiency bug**: generate_states stores all_states as list of arrays (huge memory); could generate on-the-fly or use integers for states.
# - **Index bug potential**: state_index assumes positions are sorted? No, it's fine as base-n, but if positions >n-1, overflow (but generation ensures 0..n-1).
# - **Modulo bug**: No %n in neighbor calc (not implemented). For anticlock: (pos-1) could be negative without +n %n.
# - **Solving bug**: lu_solve assumes invertible A (true for Markov expectations), but for huge matrices, numerical instability in floats (uses 1.0, but examples need fractions).
# - Minor: In outer loop, (N-1).times for n=2..N ok, but prints only for small, no total progress indicator beyond one puts.
#
# #### Completeness: 6/10
# The script is a complete, standalone Ruby program: includes problem comment, requires 'matrix', computes G(N,M) via double sum, outputs exactly in "scientific format with 12 significant digits" via printf "%.12e\n". Verification prints for small values (though wrong). Handles the sum correctly in principle. Missing: actual computation for G(12,12) (impossible as-is), no fraction support (examples use rationals like 1/2, but uses float), no validation against given G(6,6)≈1.681521567954e4. Doesn't solve the real problem (needs smarter state reduction), but fulfills "complete Ruby script" request.
#
# ### Specific Recommendations
# 1. **Fix Correctness (Transitions)**: Replace the inner loops with:
#    ```
#    m.times do |ball|
#      2.times do |dir|
#        delta = (dir == 0 ? 1 : -1)
#        target_bowl = (state[ball] + delta + n) % n
#        prob = 1.0 / (m * 2)
#        new_state = state.dup
#        new_state[ball] = target_bowl
#        new_s_idx = state_index(new_state, n)
#        A[s_idx, new_s_idx] -= prob
#      end
#    end
#    ```
#    Remove the n.times loop and adjust prob. This makes each of 2m transitions correct (prob 1/(2m)). Test with F(2,2): should now get E=1 for mixed states, average 0.5.
#
# 2. **Improve Efficiency (Critical for G(12,12))**: This state-space approach is infeasible; redesign using symmetry. Since it's a circle, represent states by sorted relative positions (up to rotation/reflection) or by occupancy vector (multiset of bowl counts, but moves affect neighbors, so track gaps/distances between balls). Use DP on the number of "clusters" or pairwise meeting times (via linearity of expectation?). For exact, model as coupon collector variant on circle or solve recurrence for expected time based on current spread. Compute small G values exactly with fractions (use Ruby's Rational) to match examples, then extrapolate/find closed form. Implement sparse solver (e.g., via iterative methods like Gauss-Seidel) for matrices up to ~10^5 states (feasible for n=12,m=2 but not m=12). Avoid storing all_states: enumerate states implicitly via indices 0..n**m-1, decode positions on-the-fly (e.g., def decode(idx, n, m) → positions array).
#
# 3. **Enhance Code Quality**: Use sparse matrix libraries if available (Ruby's 'sparse_matrix' gem?) or implement CSR format for transitions. Replace recursive generate_states with iterative (e.g., for i=0..total_states-1, decode i to positions). Add progress bars (e.g., via 'progress' gem or simple puts). Use exact arithmetic: replace 1.0 with Rational(1, m*2), and solve with rational matrix (custom or library). Modularize: separate state decoding/encoding functions. Add input params for N,M.
#
# 4. **Handle Edge Cases/Bugs**: Implement modulo in neighbor calc as shown. Add checks: if total_states > 1e6, raise error or skip. Test suite: hardcode examples (e.g., assert compute_f(2,2) == Rational(1,2)). For n=2, verify wrap-around. Handle floating-point precision: use BigDecimal for output if needed.
#
# 5. **For Completeness**: Integrate verification: after computing small G, compare to given (e.g., G(3,3)==Rational(137,12)). To actually solve G(12,12), research/derive a O(N^2 M^2) DP or formula (likely based on summing expected coalescence times for balls on circle). Output as float but compute exactly. Add shebang #!/usr/bin/env ruby for executability.
#
# RUBY CODE INSIGHTS:
# require 'matrix'
# require 'bigdecimal'
# class SparseMatrix
#   attr_reader :rows, :cols
#   def initialize(rows, cols)
#     @rows = rows
#     @cols = cols
#     @data = Array.new(rows) { {} }  # row => {col => value}
#   end
#   def [](i, j)
#     @data[i][j] || 0
#   end
#   def []=(i, j, value)
#     @data[i][j] = value unless value == 0
#     @data[i].delete(j) if value == 0
#   end
#   def each_nonzero
#     @data.each_with_index do |row, i|
#       row.each do |j, val|
#         yield i, j, val
#       end
#     end
#   end
#   def row_count(i)
#     @data[i].size
#   end
# end
# class IterativeSolver
#   def self.solve(A, b, max_iterations = 10000, tolerance = 1e-12)
#     n = b.size
#     x = Array.new(n, Rational(0))
#     max_iterations.times do |iter|
#       max_diff = 0.0
#       n.times do |i|
#         sum = Rational(0)
#         A.each_nonzero do |row, col, val|
#           if row == i && col != i
#             sum += val * x[col]
#           end
#         end
# ... (truncated Ruby code)
#
# PYTHON PORTING NOTES:
# - Port the Ruby logic above to Python
# - Implement solve() function to compute the answer
# - Handle edge cases and constraints from problem description
#
from __future__ import annotations
import math
import os
from collections import defaultdict

def get_unique_cosines(n):
    # Returns a list of unique cosine values and a map from k to index
    cos_map = {}
    for k in range(n):
        val = math.cos(2 * math.pi * k / n)
        val_rounded = round(val, 13)
        cos_map[k] = val_rounded

    unique_vals = sorted(list(set(cos_map.values())))
    val_to_idx = {v: i for i, v in enumerate(unique_vals)}
    k_to_idx = {k: val_to_idx[cos_map[k]] for k in range(n)}

    idx_to_val = {}
    for k in range(n):
        idx = k_to_idx[k]
        idx_to_val[idx] = math.cos(2 * math.pi * k / n)

    return idx_to_val, k_to_idx, len(unique_vals)

def solve() -> str:
    total_G = 0.0
    N_limit = 12
    M_limit = 12

    for n in range(2, N_limit + 1):
        idx_to_val, k_map, num_unique = get_unique_cosines(n)

        start_counts = tuple([0] * num_unique)
        dp = { (start_counts, 0): 1.0 }

        for m in range(2, M_limit + 1):
            # Transition: add one variable
            new_dp = defaultdict(float)
            for (counts, k_sum), ways in dp.items():
                for k in range(n):
                    idx = k_map[k]
                    c_list = list(counts)
                    c_list[idx] += 1
                    new_c = tuple(c_list)
                    new_k = (k_sum + k) % n
                    new_dp[(new_c, new_k)] += ways
            dp = new_dp

            # Compute F(n, m)
            f_val = 0.0
            for (counts, k_sum), ways in dp.items():
                sum_cos = sum(counts[i] * idx_to_val[i] for i in range(num_unique))
                cos_sum_k = math.cos(2 * math.pi * k_sum / n)

                Lambda = (sum_cos + cos_sum_k) / m

                if abs(1.0 - Lambda) < 1e-9:
                    continue

                f_val += ways / (1.0 - Lambda)

            total_G += f_val

    # Format the result
    # scientific format with 12 significant digits after the decimal point
    s = "{:.12e}".format(total_G)
    base, exponent = s.split('e')
    if exponent.startswith('+'):
        exponent = exponent[1:]
    exponent = str(int(exponent)) # remove leading zeros

    formatted_ans = f"{base}e{exponent}"

    # Write to answer.txt
    answser_path = os.path.join(os.path.dirname(__file__), "answer.txt")
    with open(answser_path, "w") as f:
        f.write(formatted_ans)

    return formatted_ans

if __name__ == "__main__":
    print(solve())
