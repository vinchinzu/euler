# Project Euler Problem 860
#
# PROBLEM DESCRIPTION:
# <p>
# Gary and Sally play a game using gold and silver coins arranged into a number of vertical stacks, alternating turns. On Gary's turn he chooses a gold coin and removes it from the game along with any other coins sitting on top. Sally does the same on her turn by removing a silver coin. The first player unable to make a move loses.</p>
# 
# <p>
# An arrangement is called <dfn>fair</dfn> if the person moving first, whether it be Gary or Sally, will lose the game if both play optimally.</p>
# 
# <p>
# Define $F(n)$ to be the number of fair arrangements of $n$ stacks, all of size $2$. Different orderings of the stacks are to be counted separately, so $F(2) = 4$ due to the following four arrangements:</p>
# 
# <div class="center">
# <img src="resources/images/0860_diag3.jpg?1696883006" alt="0860_diag3.jpg">
# </div>
# 
# <p>
# You are also given $F(10) = 63594$.</p>
# 
# <p>
# Find $F(9898)$. Give your answer modulo $989898989$</p>
#
# ANALYSIS/REVIEW:
# ### Analysis of the Ruby Solution for Project Euler Problem 860
#
# I'll analyze the provided code based on the specified categories: correctness, efficiency, code quality, edge cases, bugs, and completeness. The analysis is informed by a deep dive into the problem (based on the description and verification against given values like F(2)=4 and F(10)=63594). Project Euler 860 involves counting "fair" arrangements of n stacks, each of height 2, where each stack is one of 4 types (GG, GS, SG, SS) with game values 2, +1/2, -1/2, -2 respectively (in combinatorial game theory terms). A fair arrangement has total game value exactly 0 (second-player win under optimal play, assuming Gary starts). This equates to counting sequences where the scaled sum (multiply values by 2: +4 for GG, +1 for GS, -1 for SG, -4 for SS) is exactly 0, modulo 989898989.
#
# The code attempts matrix exponentiation for a linear recurrence but uses an incorrect model (3 stack types instead of 4, wrong transitions, extraneous binomial coefficients). It does not solve the problem correctly. Ratings are on a 1-10 scale (1=poor, 10=excellent). I'll explain each rating, highlight issues, and provide specific recommendations.
#
# #### 1. Correctness (Rating: 2/10)
# The code does not correctly solve the problem. It models only 3 stack types (GG, GS, SS) with 1 way each, leading to a total of 3^n arrangements (wrong; should be 4^n). The "fair" condition is mishandled via an invalid sum involving binomial coefficients and parity signs, which computes something like ∑ C(n,k)^2 * 2^{n-k} * (-1)^k (assuming fixed transitions)—unrelated to the actual fair count (sequences with scaled sum 0). 
#
# - For n=2, the code's matrix would (after buggy execution) produce a "fair" value not equal to 4 (manual simulation yields something like 1 or -3 mod MOD, not 4).
# - For n=10, it would yield a value inconsistent with 63594 (total arrangements 3^10=59049 < 63594, impossible).
# - The core issue: Ignores SG stack type and miscomputes the game-theoretic condition (sum of values =0). The binomial C(n,k) multiplier is spurious (perhaps a confused attempt at inclusion-exclusion or generating functions but applied wrongly). The sign alternation might intend to filter even/odd parity of GG stacks, but this doesn't capture the ±1/2 and ±2 values needed for sum=0.
# - Positive: Modular arithmetic and inverse are correct (MOD appears prime, suitable for inverses).
#
# The code runs without crashing for small n but produces wrong results. It would "solve" F(9898) incorrectly and slowly.
#
# #### 2. Efficiency (Rating: 1/10)
# Extremely inefficient for n=9898. The matrix is (n+1) × (n+1) ≈ 10^4 × 10^4. Each multiplication is O(n^3) ≈ 10^{12} operations (with MOD reductions). Exponentiation requires O(log n) ≈ 14 multiplications, totaling ~1.4 × 10^{13} operations—impossible (would take years on standard hardware; Ruby ~10^7-10^8 ops/sec). Building the matrix is O(n^2), minor. The final loop for binomials is O(n^2) (each of n binomials takes O(n) time), negligible.
#
# - Even for n=10, it's overkill but runnable; for n=9898, it fails practically.
# - No optimizations (e.g., sparse matrix, band structure, or FFT for generating functions).
# - A correct DP solution (see recommendations) would be O(n^2) ≈ 10^8 operations, feasible in Ruby (~1-10 seconds).
#
# #### 3. Code Quality (Rating: 6/10)
# Moderately good structure and style, but undermined by logical errors and Ruby-specific issues. It's a self-contained script with clear comments, a custom Matrix class, and modular functions. Uses Ruby idioms well (e.g., array initialization, % MOD). However, the code is brittle due to bugs, and the model is ad-hoc without justification (e.g., why only 3 types? Why the "if k > 0" for GS? Why binom(n,k)?).
#
# - Strengths: Clean Matrix class with multiply/power (standard exponentiation-by-squaring). Modular inverse via extended Euclid is correct and efficient. Main computation is simple.
# - Weaknesses: Hardcoded assumptions (e.g., types as GG/GS/SS without problem alignment). No input validation. Verbose binom computation could be precomputed. Lacks comments explaining the state (k = #GG stacks?). Ruby-specific: `self.clone` in power() shallow-copies, which works for ints but could fail if @data had mutable elements. No error handling (e.g., for n=0).
# - Readability: Good (indentation, short methods). But misleading variable names (e.g., "type 2 stack (GS)" without defining game impact).
#
# Overall, it's professional-looking but fundamentally flawed in design.
#
# #### 4. Edge Cases (Rating: 4/10)
# Handles some basics but fails others due to model errors. Tested mentally/simulated for small n.
#
# - n=0: Returns 1 (empty matrix power, sum=1). Arguably correct if F(0)=1 (empty arrangement fair), but problem implies n≥1; doesn't crash.
# - n=1: Computes sum result_vec=2 (wrong; should be 0 fair arrangements, as no single stack sums to 0: 2, 0.5, -0.5, -2 ≠0). Binom loop works (C(1,0)=1, C(1,1)=1).
# - n=2: Matrix extension happens (trans[2,3] sets beyond cols), but powering ignores it; "fair" sum wrong (not 4).
# - Large n: Crashes implicitly via time/memory (matrix ~10^4×10^4 = 10^8 entries, ~800MB RAM ok, but time no).
# - k=0 or k=n in binom: Handled (loop empty, binom=1).
# - MOD=1 or non-prime: Inverse assumes coprime, but MOD prime ok; untested.
# - Negative/zero values: Power handles exp=0 (identity), but n=0 special-cased.
#
# Fails key edges like n=2 (given F(2)=4) due to wrong model. No explicit tests.
#
# #### 5. Bugs (Rating: 3/10)
# Several critical bugs prevent correct execution or cause wrong results. Some are runtime (e.g., array extension), others logical.
#
# - **Major Bugs**:
#   - Wrong number of stack types (3 vs. 4); ignores SG, leading to undercount.
#   - Transition matrix incorrect: `if k > 0` for GS addition is spurious (prevents GS at k=0, making total ways 2^n-ish instead of 3^n even in flawed model). Remove it for partial fix, but still wrong.
#   - Out-of-bounds write: For k=n, `trans[k, k+1]` extends @data[n] to size n+2, making matrix irregular (inner arrays unequal length). Accesses in multiply/power use @cols=n+1, so it "works" but ignores the extra entry; corrupts structure for n≥1.
#   - Extraneous `binom(n,k)` in fair sum: Makes computation ∑ sign(k) * C(n,k) * ways(k), which is incorrect (should not have C(n,k) multiplier; see correctness).
#   - Matrix power for n=0: Special-cased to 1, but build_transition(n=0) creates 1×1 zero matrix; power(0) should be identity, but special case overrides correctly by luck.
#   - Clone in power(): Shallow copy; if @data had mutable objects, it could mutate during squaring. Harmless here (ints immutable).
#
# - **Minor Bugs**:
#   - In binom: `min_k = [k, n-k].min`; if k>n (impossible), crashes, but loop is 0 to n.
#   - mod_inverse: Handles a=0 or a=1 edge, but if gcd(a,M)>1, returns wrong (undefined behavior); assumes coprime.
#   - No handling if exp<0 in power() (though not called).
#
# - **Runtime Issues**: For n=9898, out-of-memory/time error likely (matrix alloc ~800MB ok, but mult loops too slow).
#
# The code runs for small n but produces garbage; fixes needed for any correctness.
#
# #### 6. Completeness (Rating: 8/10)
# A complete, standalone script: Shebang, problem description in comments, MOD defined, computes and prints F(9898). Includes all needed components (Matrix class, transitions, computation, inverse). No external deps. Runs with `ruby script.rb`. However, lacks tests, validation, or explanation of the (flawed) approach. Missing: Precomputed factorials for binoms (minor), sparse handling for efficiency.
#
# #### Specific Recommendations
# 1. **Fix Model and Correctness**:
#    - Use 4 stack types with scaled values: deltas = [4, 1, -1, -4] (for GG, GS, SG, SS).
#    - Implement DP for number of ways to reach scaled sum=0 after n steps:
#      ```ruby
#      def compute_f(n)
#        if n == 0
#          return 1 % MOD  # Or 0, depending on problem; adjust if needed
#        end
#        offset = 4 * n
#        size = 8 * n + 1
#        dp = Array.new(size, 0)
#        dp[offset] = 1
#        deltas = [4, 1, -1, -4]
#        (1..n).each do |step|
#          new_dp = Array.new(size, 0)
#          (0...size).each do |i|
#            next if dp[i] == 0
#            deltas.each do |delta|
#              j = i + delta
#              if j >= 0 && j < size
#                new_dp[j] = (new_dp[j] + dp[i]) % MOD
#              end
#            end
#          end
#          dp = new_dp
#        end
#        dp[offset]
#      end
#      ```
#      - This is O(n^2) time (8×10^8 iterations worst-case, ~5-20s in Ruby; optimize by tracking min/max sum bounds or using a Hash for sparse dp if needed).
#      - Verify: For n=2, dp[offset]=4; for n=10, should yield 63594 % MOD.
#      - Remove binom loop and sign entirely—DP directly counts fair arrangements.
#      - Remove `if k > 0` in transitions (irrelevant now).
#
# 2. **Improve Efficiency**:
#    - Replace matrix expo with above DP (O(n^2) vs. O(n^3 log n)).
#    - For faster: Use two arrays and bound loops to current min/max sum (reduces to ~O(n^2 / something)). Or implement polynomial exponentiation with NTT/FFT (advanced; use gem like 'ntt' if allowed, but pure Ruby hard).
#    - Precompute nothing needed here, but if binom kept (don't), precompute facts/invfacts in O(n log MOD).
#    - For n=9898, test on smaller n first; if too slow, switch to C++ or optimize with `dp.map!` or parallel (but Ruby GIL limits).
#
# 3. **Enhance Code Quality**:
#    - Add comments explaining the correct model (e.g., "Scaled sums: GG=4, GS=1, SG=-1, SS=-4; count ways to sum=0").
#    - Define clone in Matrix: `def clone; self.dup; end` (ensures deep-ish copy if needed).
#    - Make Matrix stricter: In []=, raise if i>=@rows or j>=@cols.
#    - Remove special n=0 case if not needed; add `raise ArgumentError, "n >=1"` for problem constraints.
#    - Use constants: `DELTAS = [4,1,-1,-4]`.
#    - Refactor: Move DP into compute_f; add a test function (e.g., `assert compute_f(2) == 4`).
#
# 4. **Handle Edge Cases and Bugs**:
#    - Test suite: Add `if __FILE__ == $0; puts compute_f(2); end` and verify against given values.
#    - Fix out-of-bounds: In build_transition, change `if k < n` (not n+1) for trans[k,k+1].
#    - Mod inverse: Add `return nil if gcd(a,m) !=1` or handle explicitly.
#    - Edges: Handle n=1 (should return 0); negative n raise error.
#
# 5. **General Improvements for Completeness**:
#    - Add ARGV for n: `n = ARGV[0].to_i || 9898`.
#    - Output format: `puts "F(#{n}) = #{result}"`.
#    - Profile with `ruby -r profile script.rb` to check slowdowns.
#    - If DP too slow in Ruby, note: "For production, port to faster lang like C++ with 64-bit ints for MOD".
#
# With these fixes (especially DP), the code would score 9-10 across categories and solve the problem correctly. As-is, it's a good starting point for learning matrix expo but not a solution.
#
# RUBY CODE INSIGHTS:
# MOD = 989898989
# DELTAS = [4, 1, -1, -4].freeze  # Scaled game values for GG, GS, SG, SS stacks
# def compute_f(n)
#   raise ArgumentError, "n must be non-negative integer" unless n.is_a?(Integer) && n >= 0
#   if n == 0
#     return 1  # Empty arrangement is fair
#   end
#   offset = 4 * n
#   size = 8 * n + 1  # Range from -4n to +4n inclusive
#   dp = Array.new(size, 0)
#   dp[offset] = 1
#   (1..n).each do |step|
#     new_dp = Array.new(size, 0)
#     (0...size).each do |i|
#       next if dp[i] == 0  # Skip impossible sums
#       DELTAS.each do |delta|
#         j = i + delta
#         if j >= 0 && j < size
#           new_dp[j] = (new_dp[j] + dp[i]) % MOD
#         end
#       end
#     end
#     dp = new_dp
#   end
#   dp[offset]
# end
# def test_compute_f
#   puts "Testing compute_f:"
#   result = compute_f(0)
#   expected = 1
#   puts "F(0) = result}, expected #{expected}"
#   result = compute_f(2)
#   expected = 4
#   puts "F(2) = result}, expected #{expected}"
#   result = compute_f(10)
#   expected = 63594
#   puts "F(10) = result}, expected #{expected}"
# ... (truncated Ruby code)
#
# PYTHON PORTING NOTES:
# - Port the Ruby logic above to Python
# - Implement solve() function to compute the answer
# - Handle edge cases and constraints from problem description
#

mod = 989898989
n = 9898
fact = [1] * (n+1)
for i in range(1,n+1):
    fact[i] = fact[i-1] * i % mod
invfact = [0] * (n+1)
invfact[n] = pow(fact[n], mod-2, mod)
for i in range(n-1, -1, -1):
    invfact[i] = invfact[i+1] * (i+1) % mod
total = 0
for j in range(0, n//5 +1, 2):
    s = (n + 3*j) // 2
    low_c = 4 * j
    if s < low_c: continue
    sum_contrib = 0
    for c in range(low_c, s+1):
        a = s - c
        b = a + j
        d = c - 4*j
        if a<0 or b<0 or c<0 or d<0: continue
        term = fact[n] * invfact[a] * invfact[b] * invfact[c] * invfact[d] % mod
        sum_contrib = (sum_contrib + term) % mod
    if j == 0:
        total = (total + sum_contrib) % mod
    else:
        total = (total + 2 * sum_contrib) % mod
print(total)
