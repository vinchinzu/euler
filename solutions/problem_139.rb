# Problem 139: Pythagorean Tiling
#
# We are looking for Pythagorean triangles (a,b,c) such that their perimeter
# a+b+c is less than 100,000,000.
# Additionally, these triangles must satisfy the condition that c is divisible
# by |a-b|. This relates to tiling a square hole.
#
# We can generate primitive Pythagorean triples (a0, b0, c0) using Euclid's formula:
# a0 = m^2 - k^2
# b0 = 2mk
# c0 = m^2 + k^2
# where m > k > 0, m and k are coprime, and m and k have opposite parity.
#
# The perimeter of such a primitive is P0 = a0 + b0 + c0 = 2m(m+k).
#
# If a primitive triple (a0,b0,c0) satisfies the condition c0 % |a0-b0| == 0,
# then any multiple (d*a0, d*b0, d*c0) will also satisfy it, because
# (d*c0) % |d*a0 - d*b0| == (d*c0) % (d*|a0-b0|) == 0.
# (This is true if d*|a0-b0| is not zero, which it won't be.
# And c0 % |a0-b0| == 0 implies k*|a0-b0| = c0 for some integer k.
# Then d*k*|a0-b0| = d*c0. So (d*c0)/(d*|a0-b0|) = k, an integer.)

# GCD function
def gcd(a, b)
  while b != 0
    a, b = b, a % b
  end
  a
end

perimeter_limit = 100_000_000
total_tiling_triangles = 0

# Determine m_limit for the loop:
# P0 = 2m(m+k). Smallest k is 1. So P0_min approx 2m(m+1) approx 2m^2.
# 2m^2 < perimeter_limit
# m^2 < perimeter_limit / 2
# m < sqrt(perimeter_limit / 2)
m_limit = Math.sqrt(perimeter_limit / 2).to_i

(2..m_limit).each do |m|
  # k runs from 1 up to m-1
  (1...m).each do |k|
    # Condition for primitive triples:
    # 1. m and k must have opposite parity. (m-k) must be odd.
    next if ((m - k) % 2 == 0) # same as (m.even? == k.even?) or (m.odd? == k.odd?)
    
    # 2. m and k must be coprime.
    next if gcd(m, k) != 1
    
    # Generate primitive triple (a0, b0, c0)
    # By convention, a can be m^2-k^2 and b can be 2mk or vice-versa.
    # The condition uses |a-b|, so the specific assignment does not matter.
    a0 = m**2 - k**2
    b0 = 2 * m * k
    c0 = m**2 + k**2
    
    # Calculate the side of the "hole" for tiling.
    hole_side = (a0 - b0).abs
    
    # hole_side cannot be 0 because that would imply a0=b0, i.e., m^2-k^2 = 2mk.
    # Dividing by k^2: (m/k)^2 - 1 = 2(m/k).
    # Let x = m/k. x^2 - 2x - 1 = 0.
    # x = (2 +/- sqrt(4 - 4*1*(-1))) / 2 = (2 +/- sqrt(8)) / 2 = 1 +/- sqrt(2).
    # Since m/k must be rational, this equality is not possible for integers m, k.
    # Thus, hole_side is never zero.

    if c0 % hole_side == 0
      # This primitive (a0,b0,c0) allows tiling.
      # Its perimeter is P0 = a0 + b0 + c0 = (m^2-k^2) + (2mk) + (m^2+k^2) = 2m^2 + 2mk = 2m(m+k).
      p0 = 2 * m * (m + k)
      
      # All multiples d*P0 < perimeter_limit are also solutions.
      # d < perimeter_limit / P0.
      # Number of such multiples is floor((perimeter_limit - 1) / P0).
      # (Using perimeter_limit - 1 because perimeter must be strictly less than limit)
      num_multiples = (perimeter_limit - 1) / p0
      total_tiling_triangles += num_multiples
    end
  end
end

puts total_tiling_triangles
