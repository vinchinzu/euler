# <p>Consider the fraction, $\dfrac n d$, where $n$ and $d$ are positive integers.
#  If $n \lt d$ and $\operatorname{HCF}(n,d)=1$, it is called a reduced proper fra
# ction.</p>
# <p>If we list the set of reduced proper fractions for $d \le 8$ in ascending ord
# er of size, we get:
# $$\frac 1 8, \frac 1 7, \frac 1 6, \frac 1 5, \frac 1 4, \frac 2 7, \frac 1 3, \
# frac 3 8, \mathbf{\frac 2 5}, \frac 3 7, \frac 1 2, \frac 4 7, \frac 3 5, \frac
# 5 8, \frac 2 3, \frac 5 7, \frac 3 4, \frac 4 5, \frac 5 6, \frac 6 7, \frac 7 8
# $$</p>
# <p>It can be seen that $\dfrac 2 5$ is the fraction immediately to the left of $
# \dfrac 3 7$.</p>
# <p>By listing the set of reduced proper fractions for $d \le 1\,000\,000$ in asc
# ending order of size, find the numerator of the fraction immediately to the left
#  of $\dfrac 3 7$.</p>

# Solution for Project Euler Problem 71

# We are looking for a reduced proper fraction n/d such that n/d < 3/7,
# d <= 1,000,000, and n/d is the largest such fraction.
# This is equivalent to finding the fraction n/d that is immediately to the
# left of 3/7 in the Farey sequence F_1000000 (extended to include non-reduced fractions initially).

# If n1/d1 and n2/d2 are consecutive terms in a Farey sequence with n1/d1 < n2/d2,
# then n2*d1 - n1*d2 = 1.
# Let n1/d1 = n/d (the fraction we are looking for) and n2/d2 = 3/7.
# So, 3*d - 7*n = 1.
# This equation implies that HCF(n,d) = 1, so the fraction is reduced.
# Also, from 3d - 7n = 1, we have n/d = (3d-1)/(7d) = 3/7 - 1/(7d).
# Since d > 0, 1/(7d) > 0, so n/d < 3/7, as required.

# To maximize n/d (i.e., to make it as close as possible to 3/7),
# we need to minimize 1/(7d). This means we need to maximize d.
# The limit for d is 1,000,000.

# From 3d - 7n = 1, we have 7n = 3d - 1.
# This means 3d - 1 must be divisible by 7.
# So, 3d - 1 ≡ 0 (mod 7)
# 3d ≡ 1 (mod 7)
# To solve for d, we multiply by the modular inverse of 3 mod 7.
# 3 * 1 ≡ 3 (mod 7)
# 3 * 2 ≡ 6 (mod 7)
# 3 * 3 ≡ 9 ≡ 2 (mod 7)
# 3 * 4 ≡ 12 ≡ 5 (mod 7)
# 3 * 5 ≡ 15 ≡ 1 (mod 7)
# The inverse of 3 mod 7 is 5.
# Multiplying by 5: 5 * (3d) ≡ 5 * 1 (mod 7)
# 15d ≡ 5 (mod 7)
# d ≡ 5 (mod 7)

# We need to find the largest d <= 1,000,000 such that d ≡ 5 (mod 7).
limit_d = 1_000_000
target_d = 0

(limit_d).downto(1) do |d_candidate|
  if d_candidate % 7 == 5
    # We need to ensure that d_candidate is not 3/7 itself.
    # If n/d = 3/7, then 7n = 3d, so 3d - 7n = 0. But we need 3d - 7n = 1.
    # The d=0 case is excluded by d being positive.
    # A special case: if d=0, n would be -1/7, not allowed.
    # The problem states d is a positive integer.
    # Smallest d is d=5 (gives n=2, fraction 2/5).
    # If d=0, 3*0 - 7n = 1 => -7n = 1, n=-1/7 (not integer, d not positive).
    # The problem implies d > 0. Also, for 3/7, d cannot be 0.
    # The problem asks for n/d immediately left of 3/7. 3/7 itself is not a candidate for n/d.
    # The question is about the numerator of n/d where d can be 7.
    # If d=7, 3*7 - 7n = 1 => 21 - 7n = 1 => 7n = 20 => n=20/7 (not integer).
    # So d=7 is not a problem. The d_candidate will not be 7 if we are looking for d > 7.
    # Our d_candidate will be large.
    # The only edge case that could lead to n/d = 3/7 is if 3d-7n = 0.
    # But our equation is 3d-7n=1.
    # What if d=7, then 7%7 = 0, not 5. So d=7 is not an issue.
    # What if d_candidate makes n = 0? (3*d_candidate - 1)/7 = 0 => 3*d_candidate=1. No integer d.
    # What if d_candidate makes n = d_candidate? (3d-1)/7 = d => 3d-1=7d => 4d=-1. No positive d.

    # The largest d <= 1,000,000 satisfying d % 7 == 5 is what we need.
    # Once found, this d maximizes n/d = 3/7 - 1/(7d).
    target_d = d_candidate
    break
  end
end

# Calculate n for this target_d
numerator_n = (3 * target_d - 1) / 7

puts "The numerator of the fraction immediately to the left of 3/7 is: #{numerator_n}"
# For verification:
# puts "d = #{target_d}"
# puts "Fraction is #{numerator_n}/#{target_d}"
# puts "Value: #{numerator_n.to_f/target_d}"
# puts "3/7 is #{3.0/7.0}"
