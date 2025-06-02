require 'set'

# Precompute powers of 10
POWERS_OF_10 = (0..16).map { |i| 10**i }
MAX_X = POWERS_OF_10[16]  # 10^16
numbers = Set.new

# For each d_2 (digits of b)
(1..15).each do |d_2|
  pow = POWERS_OF_10[d_2]
  denom = pow - 1
  max_b = pow - 1

  (1..max_b).each do |b|
    # Quadratic: s^2 - 10^d_2 s + b (10^d_2 - 1) = 0
    c = b * denom
    discriminant = pow * pow - 4 * c
    next unless discriminant >= 0  # Must be non-negative

    sqrt_d = Math.sqrt(discriminant).to_i
    next unless sqrt_d * sqrt_d == discriminant  # Must be perfect square

    # Two possible s values
    [pow + sqrt_d, pow - sqrt_d].each do |sum|
      next unless sum % 2 == 0  # 2s must be integer
      s = sum / 2
      next unless s > b  # Ensure a = s - b > 0

      x = s * s
      next if x >= MAX_X  # x must have <= 16 digits

      # Verify: x = a * 10^d_2 + b
      a = s - b
      if x == a * pow + b
        numbers.add(x)
      end
    end
  end
end

puts numbers.sum
