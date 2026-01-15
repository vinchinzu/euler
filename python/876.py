# Project Euler Problem 876
#
# PROBLEM DESCRIPTION:
# <p>
# Starting with three numbers $a, b, c$, at each step do one of the three operations:</p>
# <ul>
# <li>change $a$ to $2(b + c) - a$;
# </li><li>change $b$ to $2(c + a) - b$;
# </li><li>change $c$ to $2(a + b) - c$;
# </li></ul>
# 
# <p>
# Define $f(a, b, c)$ to be the minimum number of steps required for one number to become zero. If this is not possible then $f(a, b, c)=0$.</p>
# 
# <p>
# For example, $f(6,10,35)=3$:
# $$(6,10,35) \to (6,10,-3) \to (8,10,-3) \to (8,0,-3).$$
# However, $f(6,10,36)=0$ as no series of operations leads to a zero number.</p>
# 
# <p>
# Also define $F(a, b)=\sum_{c=1}^\infty f(a,b,c)$.
# You are given $F(6,10)=17$ and $F(36,100)=179$.</p>
# 
# <p>
# Find $\displaystyle\sum_{k=1}^{18}F(6^k,10^k)$.</p>
#
# ANALYSIS/REVIEW:
# ### Analysis of Ruby Solution for Project Euler Problem 876
#
# #### Correctness (4/10)
# The core logic for `compute_f` using BFS correctly implements the three operations and searches for the minimum steps to reach a state where one number is zero. It handles the example case `f(6,10,35)=3` accurately, as the BFS would explore the path `(6,10,35) -> (6,10,-3) -> (8,10,-3) -> (8,0,-3)` within depth 3. Returning 0 for impossible cases (when queue empties without finding zero) matches the problem definition. Negatives are handled implicitly since Ruby integers support them, and the visited set uses arrays of integers, which works.
#
# However, correctness is compromised by arbitrary limits:
# - `max_depth=20` may miss solutions requiring more steps for larger `a`, `b`, `c`. For instance, if a solution exists at depth 21, it incorrectly returns 0.
# - `max_c=1000` in `compute_F` truncates the infinite sum. While this may suffice for small `k` (e.g., `F(6,10)=17` likely has `f>0` only for small `c`), for larger `k` (e.g., `k=18`, `a=6^18 ≈ 1.5×10^14`, `b=10^18`), `f(a,b,c)>0` could occur for much larger `c`, leading to underestimation.
# - The code computes `F(6,10)` and `F(36,100)` as given (verifiable by running for small `k`), but for the full sum up to `k=18`, it will produce wrong results due to these limits and efficiency failures (see below).
# - Unused `gcd` function suggests an incomplete attempt at optimization (e.g., early termination if `gcd(a,b,c)` doesn't allow zero), but it's not integrated, so no impact.
#
# Overall, correct for toy cases but fails for the problem's scale.
#
# #### Efficiency (1/10)
# The BFS in `compute_f` has time/space complexity O(3^d) in the worst case (d=depth), where each level branches by 3. With `max_depth=20`, the queue/visited set can grow to ~3^20 ≈ 3.5×10^9 states, each storing large integers (Ruby BigInts for `k≥5`). Memory would exceed gigabytes, and time would be seconds to hours per call even for small `c`.
#
# - For `k=1` (`a=6`, `b=10`), 1000 calls to BFS might complete in minutes if state space collapses due to duplicates/invariants, but still slow.
# - For `k=2` (`a=36`, `b=100`), numbers grow quickly (e.g., new values up to ~2×(a+b+c)), exploding visited set size; likely times out or OOMs.
# - For `k≥3`, utterly infeasible: 18 iterations × 1000 `c` × exponential BFS = intractable (days/months on a supercomputer).
# - No optimizations like bounding state space (e.g., via invariants like preserved gcd or modular arithmetic), symmetry pruning, or memoization across `c` values.
# - `compute_F` loops naively over `c=1..1000` without parallelism or early stopping (e.g., if `f=0` for large `c`).
# - The infinite sum requires a closed-form or bound on `c` where `f>0`, but the code ignores this, making it non-scalable.
#
# This brute-force approach is unsuitable for Project Euler's scale, where `k=18` demands O(1) or O(log k) per `F`.
#
# #### Code Quality (7/10)
# The code is clean, readable Ruby: uses standard BFS with a queue and visited set, proper array unpacking, and BigInt support implicitly. Comments explain the problem and functions. Structure is logical (helpers then main loop). Output includes progress prints, which is user-friendly for debugging.
#
# Issues:
# - Unused `gcd` function clutters the code (remove or integrate).
# - Hardcoded limits (`max_depth=20`, `max_c=1000`) without justification or parameters for tuning.
# - No error handling (e.g., for overflow, though Ruby handles it) or input validation (assumes positive integers).
# - Prints intermediates (`puts "Computing..."`), which is fine for debugging but pollutes output for a "complete script"; consider logging or removing.
# - The script is executable and self-contained (`require 'set'` is minimal), with the problem as a comment.
# - Minor style: Inconsistent spacing (e.g., `2*(curr_b + curr_c) - curr_a`), and `state_key = new_state` could be clearer as `state_key = new_state.sort` if order-insensitive, but operations distinguish variables.
#
# Good modularity, but lacks robustness.
#
# #### Edge Cases (6/10)
# - **Initial zero**: If `a=0` or `b=0` or `c=0`, BFS immediately returns 0 steps, correct (though problem starts with positives).
# - **Negatives**: Handled (example path includes `-3`), and visited set works with negative integers.
# - **Impossible cases**: Returns 0 after exhaustion, correct (e.g., `f(6,10,36)=0`).
# - **Small values**: Works for `c=1`, minimal states.
# - **Large `c`**: Truncated at 1000, but if `f>0` beyond, misses (edge for infinity).
# - **Depth limit**: At exactly depth 20 with zero, returns 20 (correct if true min), but >20 incorrectly 0.
# - **Large `k`**: BigInts ok, but efficiency fails, so untested edges like `c` comparable to `a/b`.
# - Untested: All equal (e.g., `a=b=c=1`), or when operations loop indefinitely (BFS avoids via visited).
# - No cases where gcd prevents zero (if any; unclear from problem), since gcd unused.
#
# Handles basics but not extremes due to limits.
#
# #### Bugs (3/10)
# Few outright bugs, but critical flaws act as bugs:
# - **Potential infinite loop/OOM**: BFS without depth limit could loop if cycles, but visited prevents it. However, at depth 20, it skips enqueueing children but continues processing queue, which is correct (BFS levels). Still, for deep solutions, wrongly returns 0.
# - **Visited set collisions**: Arrays as keys work (Ruby hashes arrays by contents), but if states have very large ints, hash computation slows (minor).
# - **Queue shift/unpack**: Correct, but if new_state has non-integers (impossible here), would fail— but inputs are ints.
# - **Sum truncation**: Not a bug per se, but `F` is incomplete, leading to wrong final sum.
# - **No bug in operations**: Verified manually—e.g., op1: `2*(b+c)-a` matches.
# - **Unused gcd**: Dead code, not a bug but smells like incomplete feature.
# - Runtime: For `k=1`, runs; for larger, crashes on memory/time (e.g., Set.new with 10^8+ entries).
#
# Mostly logic sound, but limits introduce "silent bugs" in results.
#
# #### Completeness (5/10)
# The script is a "complete Ruby script" as requested: includes problem comment, computes the required sum `∑_{k=1}^{18} F(6^k, 10^k)`, outputs it, and is directly executable. It verifies given examples implicitly (via computation). However:
# - Doesn't solve the full problem due to efficiency/limits—output will be wrong/incomplete for large `k`.
# - No mathematical insight (e.g., invariants for fast `f` computation), so not a true solution.
# - Lacks bounds for infinite sum or depth; assumes 1000/20 suffice.
# - Outputs progress, which is extra but not harmful.
# - For Project Euler, completeness means correct answer, which it won't produce.
#
# It's a starting point but not a working solution.
#
# #### Specific Recommendations
# 1. **Improve Correctness**:
#    - Make `max_depth` and `max_c` configurable or dynamic (e.g., `max_c = 10 * (a + b)` based on problem patterns; test with given `F(6,10)=17` to find bound where `f=0` for `c > some multiple of a+b`).
#    - Integrate `gcd`: If `gcd(a,b,c) == 0` or doesn't divide potential zeros, early return 0. Explore invariants (e.g., parity of sums, mod 3; test if all reachable states preserve `a + b + c mod 3`—from example, no, but perhaps `a - b mod gcd`).
#    - Verify with examples: Add asserts like `compute_F(6,10) == 17` and `compute_F(36,100) == 179` to catch limit issues.
#
# 2. **Boost Efficiency**:
#    - **Prune BFS**: Sort states `[x,y,z].sort` for visited to reduce duplicates (operations permute roles). Bound numbers (e.g., skip if all |x|,|y|,|z| > 2*(a+b+c)).
#    - **Memoization**: Cache `f(a,b,c)` across calls, but since `c` varies, use a map keyed by normalized state (e.g., divided by gcd).
#    - **Mathematical Shortcut**: Analyze operations as matrix transformations or reflections in 3D space. Notice they preserve the multiset modulo something. Compute `F(a,b)` in closed form: perhaps `f(a,b,c)>0` only if `c` satisfies linear Diophantine conditions, and steps relate to distance to origin. For powers like `6^k,10^k`, find pattern (e.g., `F` scales with k).
#    - **Parallelize**: Use threads for `c` loop in `compute_F`.
#    - **Bound for Infinity**: Prove/run experiments: `f(a,b,c)=0` for `c > a+b` or similar (from example, `c=36` impossible vs `35` possible). Increase `max_c` to 10^5 for small k, but for k=18, need O(1) formula.
#    - Alternative: Simulate backwards from zero (BFS from (0,x,y) states), but state space still huge.
#
# 3. **Enhance Code Quality**:
#    - Remove unused `gcd` or use it (e.g., normalize states by dividing by gcd).
#    - Parameterize limits: `def compute_F(a, b, max_c: 1000, max_depth: 20)`.
#    - Suppress prints or use `$stdout` for clean output.
#    - Add docstrings/tests: E.g., `raise "Test failed" unless compute_f(6,10,35)==3`.
#    - Style: Use `Array#sort` for canonical state keys; consider `Struct` for states instead of arrays.
#
# 4. **Handle Edge Cases/Bugs**:
#    - Test large k small-scale: Run for k=1-3, compare to manual calc.
#    - Add depth warning: If queue empties at max_depth, log "Possible deeper solution missed".
#    - Handle very large ints: Though Ruby ok, monitor with `GC.start` periodically.
#    - Bug fix: In BFS, enqueue only if new steps < max_depth to avoid partial levels.
#
# 5. **Achieve Completeness**:
#    - To solve fully, derive formula for `F(a,b)`: From givens, `F(6,10)=17`, `F(36,100)=179`; pattern might be `F(6^k,10^k) = something like k*constant`. Research invariants (e.g., minimal steps is related to continued fractions or gcd chains).
#    - If brute ok for small k, compute up to k=5 exactly, then extrapolate.
#    - Final output: Ensure only the sum prints, no extras.
#
# With these, efficiency could reach 5/10 for small k, correctness 8/10. But for k=18, need non-BFS approach (e.g., number theory).
#
# RUBY CODE INSIGHTS:
# require 'set'
# require 'thread'
# def extended_gcd(a, b)
#   if a == 0
#     return [b, 0, 1]
#   end
#   gcd, x1, y1 = extended_gcd(b % a, a)
#   x = y1 - (b / a) * x1
#   y = x1
#   [gcd, x, y]
# end
# def gcd(a, b)
#   extended_gcd(a.abs, b.abs)[0]
# end
# def normalize_state(a, b, c)
#   g = gcd([a, b, c].compact.map(&:abs).max, 0)
#   g = 1 if g == 0
#   [a/g, b/g, c/g].sort
# end
# def solvable?(a, b, c)
#   g = gcd(gcd(a, b), c)
#   return false if g == 0
#   sum_mod3 = (a + b + c) % 3
#   sum_mod3 = sum_mod3.abs % 3  # Handle negatives
#   return false if sum_mod3 != 0
#   true
# end
# def compute_f(a, b, c, memo = {})
#   return 0 if a == 0 || b == 0 || c == 0
#   return 0 unless solvable?(a, b, c)
#   key = normalize_state(a, b, c)
#   return memo[key] if memo.key?(key)
#   max_val = [a.abs, b.abs, c.abs].max
#   max_depth = [20, (Math.log(max_val)/Math.log(2)).to_i + 10].min
#   bound = 2 * (a.abs + b.abs + c.abs)
#   queue = [[a, b, c, 0]]  # [a, b, c, steps]
#   visited = Set.new
#   visited.add(normalize_state(a, b, c))
#   while !queue.empty?
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
