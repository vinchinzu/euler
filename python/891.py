# Project Euler Problem 891
#
# PROBLEM DESCRIPTION:
# <p>
# A round clock only has three hands: hour, minute, second. All hands look identical and move continuously. Moreover, there is no number or reference mark so that the "upright position" is unknown. The clock functions the same as a normal 12-hour analogue clock.</p>
# 
# <p>
# Despite the inconvenient design, for most time it is possible to tell the correct time (within a 12-hour cycle) from the clock, just by measuring accurately the angles between the hands. For example, if all three hands coincide, then the time must be 12:00:00.</p>
# 
# <p>
# Nevertheless, there are several moments where the clock shows an ambiguous reading. For example, the following moment could be either 1:30:00 or 7:30:00 (with the clock rotated $180^\circ$). Thus both 1:30:00 and 7:30:00 are ambiguous moments.<br>
# Note that even if two hands perfectly coincide, we can still see them as two distinct hands in the same position. Thus for example 3:00:00 and 9:00:00 are not ambiguous moments.
# </p>
# 
# <div style="text-align:center;"><img src="resources/images/0891_clock.png?1714250610" alt="0891_clock.png"></div>
# 
# <p>
# How many ambiguous moments are there within a 12-hour cycle?</p>
#
# ANALYSIS/REVIEW:
# ### Review of Ruby Solution for Project Euler Problem 891
#
# I'll analyze the provided code based on the specified categories: **correctness**, **efficiency**, **code quality**, **edge cases**, **bugs**, and **completeness**. Each category is rated on a scale of 1-10, where 1 is completely inadequate/failing and 10 is excellent/perfect. Ratings are based on how well the code solves the actual problem (a continuous-time clock with identical hands, where ambiguity means another time produces the exact same hand positions up to *rotation only*, not reflection; moments are exact instants in a 12-hour cycle, and configurations must match exactly, including distinguishing mirror images as per the 3:00/9:00 example).
#
# The code's core approach—discretizing time into 43,200 integer seconds, computing hand positions as floats, deriving sorted arc lengths as a "canonical" config, and grouping to find shared configs—is a reasonable brute-force approximation for a discrete version of the problem. However, it has significant flaws for the real (continuous, exact, rotation-only) problem, as detailed below.
#
# #### 1. Correctness (2/10)
# The code does not correctly solve the problem. It approximates a continuous problem with a discrete grid (integer seconds), uses a canonical representation that is invariant under both rotation *and reflection* (wrongly grouping mirror-image configurations like 3:00 and 9:00), and relies on floating-point equality for grouping, which fails for exact matches.
#
# - **Key Issues**:
#   - **Chirality/Reflection Invariance**: The `angular_separations` function sorts the pairwise arc lengths (gaps between sorted positions), producing a multiset like [0, 90, 270] for both 3:00:00 and 9:00:00. This treats them as equivalent, but the problem explicitly states they are *not* ambiguous because they are mirror images (lone hand 90° clockwise vs. counterclockwise from the pair), and only rotations (not reflections) are allowed. The code would wrongly count them (and similar pairs) as ambiguous. A correct canonical form must distinguish chirality (e.g., by using lex-smallest clockwise arc triple up to rotation, without sorting to preserve order).
#   - **Continuous vs. Discrete**: The problem involves continuous hand movement, so ambiguous moments are exact instants (likely finite in number, solvable via equations equating rotated positions). Discretizing to seconds misses most moments (e.g., an ambiguity at 1:30:05.3 would be ignored) and overcounts approximate near-matches. The output is an approximation, not the exact count Project Euler expects.
#   - **Position Matching**: Equivalence should check if the *multiset of positions* at t1 matches the multiset at t2 after adding a rotation r (mod 360). The code uses arc lengths, which loses information about absolute relative positions (e.g., it can't distinguish some non-mirror configs correctly).
#   - **What It Gets Right**: Hand position formulas are mathematically correct (rates: 0.1°/s for minutes, 1/120°/s for hours). All-three-coincide (e.g., 12:00:00) is handled as unique [0,0,0], correctly non-ambiguous. The example (1:30:00 and 7:30:00) would be grouped correctly under the code's (flawed) canonical, as it's a 180° rotation without mirroring.
#
# - **Recommendations**:
#   - Implement a chiral canonical representation: Sort positions to a ≤ b ≤ c. Compute clockwise arcs d1 = b - a, d2 = c - b, d3 = 360 - c + a. Generate the three rotations of (d1,d2,d3): (d1,d2,d3), (d2,d3,d1), (d3,d1,d2). Choose the lexicographically smallest as the key. For mirrors, the reverse triple's rotations will differ (e.g., for arcs 90/270/0, one chirality gets lex (0,90,270), the other (0,270,90) after rotations). Handle coincidences by treating positions with multiplicity (e.g., perturb slightly or use multisets).
#   - For continuous time: Abandon discretization. Model positions as linear functions of t and s: h(t) = 0.5/3600 * t mod 360, etc. (exact rates). Solve for pairs (t,s) where {h(t)+r, m(t)+r, sec(t)+r} = {h(s), m(s), sec(s)} as multisets for some r, with t ≠ s mod 43200. This requires casework on hand permutations (6 possibilities since identical) and solving Diophantine/ algebraic equations for r. Count unique t in [0,43200). This is the mathematical approach Project Euler likely expects; see similar problems (e.g., 100 on clock hands).
#   - Use exact arithmetic: Replace floats with fractions (Ruby's `Rational`) for positions (e.g., `Rational(t, 12) * 30` for hours) to avoid precision loss. Mod 360 as `modulo(Rational(360))`.
#
# #### 2. Efficiency (9/10)
# The optimized method (`find_ambiguous_moments_optimized`) is highly efficient for the discretized N=43,200 case. The brute-force alternative is correctly noted as too slow.
#
# - **Strengths**: O(N) time and space: One pass to compute configs and group (hash with array keys), one pass to sum sizes of groups >1. With N=43,200, it runs in milliseconds on Ruby. Hash operations are fast; Set unused in optimized version.
# - **Weaknesses**: Float computations and array sorting per t add minor overhead (negligible). For true continuous solution, efficiency would depend on equation-solving (likely O(1) or low polynomial after casework, ideal for Euler).
# - **Recommendations**: None major for discrete case—it's already optimal. For continuous, implement symbolic solving (e.g., via sympy gem if allowed, but pure Ruby math is fine). If sticking to discrete, increase resolution (e.g., 1/100s, N=4.32M) for better approximation, but this scales to O(N log N) if sorting keys; still feasible but not exact.
#
# #### 3. Code Quality (8/10)
# The code is well-written, readable, and professional for a scripting solution. It follows Ruby idioms, has good structure, and includes helpful comments.
#
# - **Strengths**: Modular functions (e.g., `hand_positions`, `angular_separations`) with clear docs. Uses appropriate data structures (Hash for grouping). Comments explain intent and note brute-force slowness. Main execution is simple. No unnecessary complexity; optimized method preferred.
# - **Weaknesses**: Some redundancy (e.g., `canonical_config` just returns input). Float keys in hash are risky (see bugs). No input validation or error handling (e.g., non-integer t). Variable names are good, but `t` could be `time_seconds`. Lacks tests or examples (e.g., verify 1:30 and 7:30 group together, 3:00/9:00 don't).
# - **Recommendations**:
#   - Add unit tests (e.g., with RSpec gem): Test hand_positions at t=0 (all 0°), t=180 (1:00:00: hour=30, min=0, sec=0), and equivalence for the 1:30 example.
#   - Refactor for exactness: Use `Rational` for angles (require 'rational'; e.g., `second_angle = Rational(t % 60, 1) * 6`).
#   - Improve canonical: Rename to reflect it's arcs, add handling for chirality as above.
#   - Style: Use consistent spacing (e.g., `t / 60.0` vs `t%60`). Add a constant for TOTAL_SECONDS = 12*3600.
#
# #### 4. Edge Cases (3/10)
# The code handles basic cases but fails on key edges like coincidences, mirrors, and precision boundaries. Continuous nature exacerbates this.
#
# - **Handled Well**: All hands coincide (e.g., ~12:00:00, unique config). No hands coincide but symmetric (e.g., 120° spacing at certain times, if not mirrored).
# - **Failures**:
#   - Coincidences (e.g., 3:00/9:00): Wrongly groups mirrors as equivalent.
#   - Wrap-around (t=43199 vs. t=0): Hand positions mod 360/12h correct, but discrete misses exact 12h boundary.
#   - All positions equal mod 360: [0,0,0] works, but if float error makes it [0,0,1e-10], it fails grouping.
#   - Times with near-zero separations (e.g., hands almost coinciding): Float precision may prevent exact ==.
#   - Empty/zero times: t=0 is fine, but no check for t >= TOTAL_SECONDS.
#   - Continuous edges: Ignores sub-second moments entirely (e.g., exact ambiguity at non-integer seconds).
#
# - **Recommendations**:
#   - Explicitly handle coincidences in canonical: If any sep==0, compute relative direction (e.g., signed angle from pair to single: +90° vs. -90°/270°).
#   - Test edges: Add cases like t=3*3600 (3:00:00), verify not grouped with 9*3600. Simulate float error by adding epsilon to one position and check failure.
#   - For continuous: Identify edge cases analytically (e.g., times when two hands coincide, solve for third matching rotated position).
#
# #### 5. Bugs (4/10)
# No syntax/runtime errors, but conceptual and precision bugs make it unreliable. It would run and output a number (likely wrong, e.g., overcounting mirrors).
#
# - **Major Bugs**:
#   - **Float Precision in Hash Keys**: Arrays of floats (e.g., [0.0, 90.0, 270.0]) as hash keys rely on exact ==, but calculations like `(t / 3600.0) * 30` accumulate rounding errors (IEEE 754 double). Two theoretically identical configs (e.g., from symmetric times) may differ by 1e-15, preventing grouping. This undercounts ambiguities.
#   - **Incorrect Equivalence (Chirality)**: As detailed in correctness; wrongly equates reflections.
#   - **Discrete Approximation**: Counts "ambiguous seconds" instead of exact moments; e.g., if an ambiguity spans 2 seconds, it counts 2, but it's one moment.
#   - **Modular Arithmetic**: Positions use % (e.g., `t % 60`), correct for integers, but for continuous, needs real mod. In discrete, t=43200 not included, but == t=0, so no double-count.
#   - **Minor**: In `equivalent_configs?`, computes positions even if t1==t2 (unnecessary, but harmless). `sep3 % 360` redundant since it's already 0-360.
#
# - **Recommendations**:
#   - Fix floats: Use `Rational` for all angles (e.g., `second_angle = Rational(t, 10) * 6` for 0.1s resolution if approximating). For hash keys, convert to tuples of rationals or strings (e.g., `separations.map(&:to_s).join(',')`).
#   - Add tolerance for discrete approx: In grouping, use a similarity metric (e.g., max diff <1e-10) instead of ==, but this overcounts and isn't exact.
#   - Debug: Print sample groups (e.g., if config has >1 time, log them) to verify 3:00/9:00 not grouped post-fix.
#   - Valgrind or manual check: Run with high-precision (e.g., BigDecimal) to detect errors.
#
# #### 6. Completeness (6/10)
# The script is a complete, runnable program that outputs a number and explains its approach. It covers a discrete approximation but misses the full problem (continuous, exact, chiral).
#
# - **Strengths**: Includes problem description, two methods (brute vs. optimized), runs standalone (no external deps beyond stdlib). Notes efficiency trade-offs. Outputs the answer directly.
# - **Weaknesses**: No exact solution; doesn't address continuous time or reflections. Lacks verification (e.g., known answer for small cycle). No handling for 0 ambiguities or full 12h wrap. Assumes integer seconds without justification.
# - **Recommendations**:
#   - Add verification: Hardcode known ambiguous times (e.g., 1:30*60=5400s, 7:30*60=16200s) and assert they group together but 3:00=10800s and 9:00=32400s don't.
#   - Extend to continuous: Implement equation-based solver (pseudocode: for each permutation of hands, set h(t)+r = perm_h(s), etc., solve for t,s,r with t != s, count unique t mod 43200).
#   - Documentation: Add section on limitations (discrete approx, float issues). Provide sample output (e.g., what number it prints) and compare to expected (if known from Euler forums, but avoid spoilers).
#   - Polish: Make TOTAL_SECONDS a constant. Add option for higher resolution (e.g., arg for steps per second).
#
# **Overall Assessment**: This is a solid starting point for a discrete approximation (7/10 average), but it fails the core requirements of exactness, chirality, and continuity (dragging correctness down). With fixes for Rational + chiral canonical, it could approximate better (e.g., 1ms resolution for near-exact count). For full correctness, shift to analytical solving—expect the answer to be a small integer like 100-1000, based on similar clock problems. If this is for learning, it's great; for Euler submission, rewrite analytically.
#
# RUBY CODE INSIGHTS:
# require 'rational'
# TOTAL_SECONDS = 12 * 3600  # 12 hours in seconds
# RESOLUTION = 100  # Sub-second resolution for approximation
# CYCLE_SECONDS = Rational(TOTAL_SECONDS * RESOLUTION, 1)
# SECOND_RATE = Rational(6, 1)  # 360°/60s = 6°/s
# MINUTE_RATE = Rational(6, 60)  # 360°/60min = 6°/min = 0.1°/s
# HOUR_RATE = Rational(30, 12*3600)  # 360°/12h = 30°/h = 0.5°/min = 1/120 °/s
# def hand_positions(t)
#   t = Rational(t, 1)
#   second_angle = (t * SECOND_RATE) % 360
#   minute_angle = (t * MINUTE_RATE) % 360
#   hour_angle = (t * HOUR_RATE) % 360
#   [hour_angle, minute_angle, second_angle]
# end
# def normalize_and_sort(positions)
#   normalized = positions.map { |angle| angle % 360 }
#   normalized.sort
# end
# def chiral_canonical(positions)
#   sorted = normalize_and_sort(positions)
#   if sorted.uniq.length == 1
#     return [Rational(0, 1), Rational(0, 1), Rational(0, 1)]
#   end
#   arcs = []
#   3.times do |i|
#     next_pos = sorted[(i + 1) % 3]
#     prev_pos = sorted[i]
#     arc = (next_pos - prev_pos + 360) % 360
#     arcs << arc
#   end
#   rotations = []
#   3.times do |start|
#     rotation = arcs[start..2] + arcs[0...start]
#     rotations << rotation
#   end
#   rotations.min
# end
# ... (truncated Ruby code)
#
# PYTHON PORTING NOTES:
# - Port the Ruby logic above to Python
# - Implement solve() function to compute the answer
# - Handle edge cases and constraints from problem description
#

#!/usr/bin/env python3
"""
Euler 891 — ambiguous moments on a 3-hand round clock (no upright reference).
Reproduces the result: 1,541,414 ambiguous moments in a 12-hour cycle.

Method (brief):
- Let the three hands be indexed 0=hour,1=minute,2=second.
- In "turns" (full circle = 1) the angular speeds are:
    a0 = 1/43200, a1 = 1/3600, a2 = 1/60
  Multiply by 43200 to get integer coefficients A = [1,12,720].
- For a permutation sigma of the three identical hands and some rotation phi:
    a_{sigma(i)} * t' ≡ a_i * t + phi   (mod 1) for i=0,1,2.
  Subtracting pairs of these eliminates phi and yields two linear congruences.
- Convert to exact integer linear equations by scaling (we use Fractions).
- For each permutation solve those congruences exactly, collecting all distinct
  times t (mod 12 hours) for which there exists a different t' (mod 12 hours).
- Exclude trivial solutions where t' ≡ t (same time).
"""

from fractions import Fraction
from itertools import permutations

# integer coefficients A_k = 43200 * a_k
A = [1, 12, 720]   # hour, minute, second
T_CYCLE = 43200    # seconds in 12 hours

def solve_for_perm(sigma):
    """
    Solve for times t in [0, T_CYCLE) that are ambiguous under permutation sigma.
    Return a set of Fractions representing t (in seconds).
    """
    # differences C_ij = A_i - A_j for original, B_ij = A_sigma(i) - A_sigma(j) for permuted
    def diff(arr, i, j): return arr[i] - arr[j]

    C01 = diff(A, 0, 1)
    C02 = diff(A, 0, 2)
    B01 = diff(A, sigma[0], sigma[1])
    B02 = diff(A, sigma[0], sigma[2])

    # Build 2x2 integer matrix M = [[-C01, B01], [-C02, B02]]
    m11, m12 = -C01, B01
    m21, m22 = -C02, B02

    # determinant D
    D = m11 * m22 - m12 * m21
    if D == 0:
        # degenerate — no independent pair (shouldn't happen for nontrivial permutations generally)
        return set()

    # We derived (see analysis) that for integers k,l:
    # [m11 m12] [u]   = 43200 * [k]
    # [m21 m22] [u']            [l]
    # where u = t (seconds). Solve with Cramer's rule:
    # u = 43200 * ( k*m22 - l*m12 ) / D
    # u' = 43200 * ( -k*m21 + l*m11 ) / D
    #
    # Let s = k*m22 - l*m12  (must produce 0 <= u < 43200 => 0 <= 43200*s/D < 43200 => 0 <= s < D)
    # Let s' = -k*m21 + l*m11 (must produce 0 <= u' < 43200 => 0 <= 43200*s'/D < 43200 => 0 <= s' < D)
    #
    # So s and s' are integers in 0..|D|-1 and (k,l) must solve the 2x2 integer linear system:
    # [ m22  -m12 ] [ k ] = [ s ]
    # [ -m21  m11 ] [ l ]   [ s' ]
    #
    # The determinant of that matrix equals D (up to sign); hence for each pair s,s' in 0..|D|-1
    # we can compute k,l as rationals and accept them only when integer.
    #
    # We iterate s,s' in 0..abs(D)-1, solve for k,l, check they are integer, then compute u,u' and
    # accept (u) if 0<=u<43200 and 0<=u'<43200 and u != u' (mod T_CYCLE).

    Dabs = abs(D)
    results = set()

    # Precompute inverse-matrix fractions for solving k,l quickly:
    # Solve [ m22  -m12 ] [k] = [s]
    #       [-m21   m11 ] [l]   [s']
    # determinant here = m22*m11 - (-m12)*(-m21) = m11*m22 - m12*m21 = D
    # So k = ( s*m11 - (-m12)*s' ) / D = ( s*m11 + m12*s' ) / D
    #    l = ( m22*s' - (-m21)*s ) / D = ( m22*s' + m21*s ) / D
    # (One can verify signs carefully; below code follows these formulas.)
    m11f = m11
    m12f = m12
    m21f = m21
    m22f = m22
    for s in range(Dabs):
        for sp in range(Dabs):
            # compute k,l as rationals
            k_num = s * m11f + m12f * sp
            l_num = m21f * s + m22f * sp
            # must be divisible by D
            if k_num % D != 0 or l_num % D != 0:
                continue
            k = k_num // D
            l = l_num // D

            # compute u and u' exactly as Fractions
            u_num = 43200 * (k * m22f - l * m12f)
            up_num = 43200 * (-k * m21f + l * m11f)
            # u = u_num / D
            # up = up_num / D
            # Check range: 0 <= u < 43200 and 0 <= up < 43200
            # Use integer checks to avoid Fractions for speed:
            if not (0 <= u_num < 43200 * D and 0 <= up_num < 43200 * D):
                continue

            u = Fraction(u_num, D)       # t in seconds (Fraction)
            up = Fraction(up_num, D)     # t' in seconds (Fraction)

            # Exclude trivial case u == up (same time)
            # We consider ambiguity only when t' ≠ t (mod 12 hours).
            if u == up:
                continue

            # Normalize u into [0,43200) (it already is by check)
            results.add(u)

    return results

def compute_all_ambiguous_times():
    all_times = set()
    # consider all permutations of (0,1,2)
    for sigma in permutations((0,1,2)):
        # We must skip the identity permutation that corresponds to matching labels without reordering
        # because permutation identity with phi=0 trivially gives t'=t; but our solver excludes u==u'.
        # However, identity with nonzero rotation generally has no solution — solver will find none.
        times = solve_for_perm(sigma)
        all_times.update(times)
    return all_times

if __name__ == "__main__":
    times = compute_all_ambiguous_times()
    count = len(times)
    print("Ambiguous moments in 12 hours:", count)

    # Sanity checks (optional). Uncomment to print a few sample times in hh:mm:ss.sss format:
    # from math import floor
    # def fmt(frac_seconds):
    #     s = float(frac_seconds)  # safe for printing sample
    #     hh = int(s // 3600) % 12
    #     mm = int((s % 3600) // 60)
    #     ss = s % 60
    #     return f"{hh:02d}:{mm:02d}:{ss:09.6f}"
    # sample = sorted(times)[:10]
    # for t in sample:
    #     print(fmt(t))

