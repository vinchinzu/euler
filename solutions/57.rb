# Square root convergents
# Problem 57
# It is possible to show that the square root of two can be expressed as an infinite continued fraction.
#
# √ 2 = 1 + 1/(2 + 1/(2 + 1/(2 + ... ))) = 1.414213...
#
# By expanding this for the first four iterations, we get:
#
# 1 + 1/2 = 3/2 = 1.5
# 1 + 1/(2 + 1/2) = 7/5 = 1.4
# 1 + 1/(2 + 1/(2 + 1/2)) = 17/12 = 1.41666...
# 1 + 1/(2 + 1/(2 + 1/(2 + 1/2))) = 41/29 = 1.41379...
#
# The next three expansions are 99/70, 239/169, and 577/408, but the eighth expansion, 1393/985,
# is the first example where the number of digits in the numerator exceeds the number of digits in the denominator.
#
# In the first one-thousand expansions, how many fractions contain a numerator with more digits than the denominator?

def solve_square_root_convergents
  # We are looking for the first 1000 expansions.
  limit = 1000
  count = 0

  # Initialize numerator (n) and denominator (d) for the state *before* the first expansion.
  # This corresponds to N_0 = 1, D_0 = 1, which helps generate N_1/D_1 = 3/2 correctly.
  # N_k = N_{k-1} + 2 * D_{k-1}
  # D_k = N_{k-1} + D_{k-1}
  n = 1 # Represents N_{k-1}
  d = 1 # Represents D_{k-1}

  # Iterate for each of the 1000 expansions.
  (1..limit).each do |_|
    # Calculate the numerator and denominator for the current expansion (k-th expansion).
    current_expansion_n = n + 2 * d
    current_expansion_d = n + d

    # Check if the current numerator has more digits than the current denominator.
    # Ruby's integers handle arbitrary precision, so we don't need BigDecimal.
    if current_expansion_n.to_s.length > current_expansion_d.to_s.length
      count += 1
    end

    # Update n and d to be the current expansion's values,
    # so they become the "previous" values for the next iteration.
    n = current_expansion_n
    d = current_expansion_d
  end

  count
end

puts solve_square_root_convergents
