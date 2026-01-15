# Project Euler Problem 885
#
# PROBLEM DESCRIPTION:
# <p>
# For a positive integer $d$, let $f(d)$ be the number created by sorting the digits of $d$ in ascending order, removing any zeros. For example, $f(3403) = 334$.</p>
# 
# <p>
# Let $S(n)$ be the sum of $f(d)$ for all positive integers $d$ of $n$ digits or less. You are given $S(1) = 45$ and $S(5) = 1543545675$.</p>
# 
# <p>
# Find $S(18)$. Give your answer modulo $1123455689$.</p>
#
# ANALYSIS/REVIEW:
# ### Overall Assessment
# This Ruby solution attempts to solve Project Euler Problem 885 by computing \(S(n)\), the sum of \(f(d)\) for all positive integers \(d\) with at most \(n\) digits, modulo 1123455689. However, the approach fundamentally misunderstands \(f(d)\): \(f(d)\) is the number formed by **sorting the non-zero digits of \(d\) in ascending order and concatenating them**, which depends only on the multiset of non-zero digits (ignoring positions and zeros). The code incorrectly computes contributions using place values from the original number \(d\) (e.g., \(10^{\text{len-pos}}\)), treating it like a sum involving the structure of \(d\) itself rather than the sorted non-zero digits. This leads to incorrect results for \(n \geq 2\).
#
# The code verifies \(S(1) = 45\) correctly (by coincidence) but fails for \(S(2)\) (code outputs 1485, actual is 3465). It is syntactically valid Ruby, runs quickly, and handles modular arithmetic properly (assuming the modulus is prime, which it appears to be). However, it includes unused code (factorials and inverse factorials), flawed logic, and no handling for leading zeros in multi-digit numbers. A correct solution requires grouping by multisets of non-zero digits and counting valid placements (excluding leading zeros) using multinomial coefficients.
#
# Below, I rate each category 1-10 (1 = severely deficient, 10 = excellent) based on the criteria. Ratings consider the code's intent to solve the problem but penalize fundamental errors.
#
# #### Correctness: 1/10
# - **Strengths**: Correctly computes \(S(1) = 45\). Modular arithmetic (e.g., `power` function for exponentiation) is implemented correctly using binary exponentiation. Binomial coefficients are computed accurately.
# - **Weaknesses**: The core logic is wrong. It misinterprets \(f(d)\) by using place values from \(d\)'s digit positions (e.g., `digit * power(10, len - pos, MOD)`) instead of the place values in the sorted non-zero digits (which form a \(k\)-digit number independent of zeros and positions in \(d\)). This treats the sum as if it were related to the value of \(d\) itself, not \(f(d)\). No enforcement of no-leading-zero for multi-digit numbers (e.g., `binom[len][k]` allows leading zeros). Fails for \(n=2\) (as manually verified: actual \(S(2) = 3465\), code gives 1485). The inner loops (e.g., `pos` for "smallest digit position") do not align with sorting non-zero digits.
# - **Verification**: For \(n=5\), the given \(S(5) = 1543545675\) cannot be checked here, but given the \(n=2\) failure, it would be wrong.
#
# #### Efficiency: 10/10
# - **Strengths**: For \(n=18\), time complexity is \(O(n^3 \cdot \log n)\) due to loops over `len` (1-18), `k` (1-len), `pos` (1 to len-k+1 ≤18), and digits (9), with ~50k modular exponentiations (each \(O(\log 18)\)). Precomputations (binomials, factorials) are \(O(n^2)\). Runs in milliseconds on modern hardware. Space is \(O(n^2)\) for binomials, fine.
# - **Weaknesses**: None—it's overkill for a wrong algorithm but scales perfectly for \(n=18\).
#
# #### Code Quality: 4/10
# - **Strengths**: Readable structure with comments (e.g., explaining loops). Uses Ruby idioms (e.g., `each` loops). Modular arithmetic is clean and consistent. No syntax errors; runs without crashing.
# - **Weaknesses**: 
#   - Unused code: Factorials (`fact`, `inv_fact`) and their inverses are precomputed but never used (binomials are computed directly). This bloats the code unnecessarily.
#   - Poor variable naming and comments: `pos` is called "position of the smallest digit" but doesn't compute contributions correctly for sorted digits. Comments don't explain the flawed logic.
#   - Magic numbers: Hardcodes loops without clear justification (e.g., why `len - k + 1` for `pos`?).
#   - Redundancy: Binomial computation sets `binom[i][i] = 1` inside a conditional that's always true.
#   - Style: Inconsistent spacing; no error handling or input validation (e.g., assumes \(n > 0\)).
#   - Overall, it's maintainable but misleading due to incorrect math.
#
# #### Edge Cases: 3/10
# - **Strengths**: Handles \(n=0\) (returns 0, correct as no positive integers). \(n=1\) works by coincidence. Binomials handle base cases like `C(0,0)=1`.
# - **Weaknesses**: 
#   - Fails for \(n=2\) (as above).
#   - No-leading-zero not enforced: For `len=2`, `k=1`, it overcounts invalid numbers like "01" (though not explicitly, the position choice allows it implicitly).
#   - Doesn't handle cases where `remaining_digits > remaining_positions` well (skips with `next`, but this is part of the flawed logic).
#   - Large \(n=18\): Would compute a wrong answer quickly, but no overflow issues (Ruby ints are arbitrary-precision, mod keeps numbers small).
#   - Zero-digit numbers or all-zero \(d\): Implicitly excluded, but not explicitly tested.
#
# #### Bugs: 2/10
# - **Strengths**: No runtime crashes, infinite loops, or arithmetic overflows. Modular operations are safe (e.g., `% MOD` after multiplications).
# - **Weaknesses**: 
#   - **Major logical bugs**: As described in correctness—wrong place values and no multiset grouping lead to incorrect sums. The `contrib` formula doesn't compute \(f(d)\).
#   - **Overcounting bug**: `ways = binom[len][k]` counts placements allowing leading zeros, but subtracts nothing for them.
#   - **Index errors?** None, but `binom[remaining_positions][remaining_digits]` assumes `remaining_positions <= n` (true here).
#   - **Unused code bug**: `inv_fact` is computed but ignored, wasting cycles (minor).
#   - **Modular subtraction**: Not an issue here, but if negative intermediates occurred, it could wrap incorrectly (doesn't happen).
#   - No unit tests or assertions (e.g., no check for given \(S(5)\)).
#
# #### Completeness: 5/10
# - **Strengths**: Covers the full problem: Computes \(S(n)\) for \(n=18\), applies modulo, outputs the result. Includes precomputations and a main call. Handles positive integers up to \(n\) digits.
# - **Weaknesses**: 
#   - Misses core requirement: Doesn't correctly implement \(f(d)\) or sum over valid \(d\) (ignores multiset nature and leading zeros).
#   - No verification against given values (\(S(1)\), \(S(5)\)).
#   - Lacks helper functions for key computations (e.g., no `f(d)` simulator for testing).
#   - Doesn't enumerate multisets or use multinomials, which are needed for correctness.
#   - Assumes MOD is prime for inverses (it is, but unstated).
#
# ### Specific Recommendations
# 1. **Fix Correctness with Multiset Approach (High Priority)**:
#    - Rewrite `compute_s(n)` to group by multisets of non-zero digits. For each `len` (1 to `n`), `k` (1 to `len`):
#      - Enumerate all multiplicity arrays `c[1..9]` where \(\sum c_d = k\) (use recursion, as sketched below).
#      - Compute `c0 = len - k`.
#      - Compute \(v\) (the value of \(f(d)\)): Build as a big integer in Ruby (e.g., `v = 0; current_power = 10**(k-1); for d=1 to 9: for i=1 to c[d]: v += d * current_power; current_power /= 10; end`), then `v % MOD`.
#      - Compute total placements: Multinomial `len! / (c0! \prod c_d!) % MOD` using precomputed `fact` and `inv_fact`.
#      - Compute invalid (leading zero) placements: If `c0 >= 1`, `(len-1)! / ((c0-1)! \prod c_d!) % MOD`; else 0.
#      - Valid ways = `(total - invalid + MOD) % MOD`.
#      - Add `(v * valid) % MOD` to sum.
#    - This correctly handles \(f(d)\), leading zeros, and multiplicities. Total operations ~4.7 million (feasible).
#    - Remove all `pos`, `ways_distribute`, and binomial loops—they're irrelevant.
#    - Verify: Implement a test for \(n=2\) (should give 3465 % MOD = 3465) and \(n=5\).
#
# 2. **Implement Multiset Enumeration**:
#    - Add a recursive helper:
#      ```ruby
#      def enumerate_multisets(d, rem_k, c, len, fact, inv_fact, MOD, sum_for_len)
#        if d == 10
#          if rem_k == 0
#            # Compute v, c0, total_ways, leading_ways, valid as above
#            # sum_for_len = (sum_for_len + (v % MOD * valid % MOD)) % MOD
#            # But pass sum_for_len by reference or return it
#          end
#          return
#        end
#        (0..rem_k).each do |count|
#          c[d] = count
#          enumerate_multisets(d+1, rem_k - count, c, len, fact, inv_fact, MOD, sum_for_len)
#        end
#      end
#      ```
#    - Call inside `k` loop: `c = Array.new(10, 0); enumerate_multisets(1, k, c, len, ...)`.
#
# 3. **Clean Up Code Quality**:
#    - Remove unused `fact` and `inv_fact` precomputations—re-add them only for multinomials (compute `inv_fact` using `power(fact[i], MOD-2, MOD)` as you have).
#    - Remove binomial precomputation entirely.
#    - Improve naming: E.g., `sum_for_length` instead of `sum_for_len`; add params/docs for helpers.
#    - Add comments explaining the multiset logic and why multinomials are used.
#    - Use Ruby's `BigDecimal` or stick to integers for \(v\) (unnecessary, as Ruby handles it).
#    - Add a constant for digits (1..9).
#
# 4. **Improve Efficiency (Minor)**:
#    - Precompute `inv_fact[0] = 1` explicitly.
#    - Cache `10**exp % MOD` if needed, but not necessary.
#    - For \(v\), use modular multiplication from the start: `place = power(10, k-1, MOD); inv10 = power(10, MOD-2, MOD);` then `place = (place * inv10) % MOD` after each digit.
#
# 5. **Handle Edge Cases and Bugs**:
#    - Add tests: E.g., assert `compute_s(1) == 45`; compute \(S(2)\) manually and assert.
#    - In valid ways: Always add `+ MOD` before `% MOD` to handle negative (though rare).
#    - For `k=0`: Explicitly skip (no positive \(d\)).
#    - For `c0=0`: Leading ways = 0 (already handled).
#    - Add input validation: `raise` if `n < 1` or `n > 18` (to avoid huge factorials).
#
# 6. **Enhance Completeness**:
#    - Add a `f(d)` function for testing: E.g., sort non-zero digits, join, to_i.
#    - Verify against \(S(5)\): Run and assert `== 1543545675 % MOD`.
#    - Output format: Already good (`puts result`), but add `p "S(18) mod #{MOD} = #{result}"`.
#    - Modular prime assumption: Add a comment confirming MOD is prime (it is, via quick check).
#
# Implementing these (especially the multiset rewrite) will yield a correct, efficient solution running in <1 second for \(n=18\). The current code is a good skeleton for modular math but needs a full algorithmic overhaul.
#
# RUBY CODE INSIGHTS:
# MOD = 1123455689
# def power(base, exp, mod)
#   result = 1
#   base %= mod
#   while exp > 0
#     if exp.odd?
#       result = (result * base) % mod
#     end
#     base = (base * base) % mod
#     exp /= 2
#   end
#   result
# end
# def compute_factorials(n, mod)
#   fact = Array.new(n + 1)
#   fact[0] = 1
#   (1..n).each { |i| fact[i] = (fact[i-1] * i) % mod }
#   fact
# end
# def compute_inv_factorials(fact, n, mod)
#   inv_fact = Array.new(n + 1)
#   inv_fact[n] = power(fact[n], mod - 2, mod)
#   (n-1).downto(0) do |i|
#     inv_fact[i] = (inv_fact[i+1] * (i + 1)) % mod
#   end
#   inv_fact
# end
# def multinomial_coefficient(total, counts, fact, inv_fact, mod)
#   result = fact[total]
#   (0..9).each { |d| result = (result * inv_fact[counts[d]]) % mod }
#   result
# end
# def compute_f_value(counts, k)
#   result = 0
#   current_power = 10**(k - 1)
#   (1..9).each do |d|
#     counts[d].times do
#       result += d * current_power
#       current_power /= 10
#     end
#   end
#   result
# end
# ... (truncated Ruby code)
#
# PYTHON PORTING NOTES:
# - Port the Ruby logic above to Python
# - Implement solve() function to compute the answer
# - Handle edge cases and constraints from problem description
#
from __future__ import annotations
from typing import Optional

def solve() -> int:
    # TODO: Implement solution
    return 0

if __name__ == "__main__":
    print(solve())
