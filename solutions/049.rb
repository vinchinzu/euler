# Prime permutations
# Problem 49
# The arithmetic sequence, 1487, 4817, 8147, in which each of the terms increases by 3330, is unusual in two ways: (i) each of the three terms are prime, and, (ii) each of the 4-digit numbers are permutations of one another.

# There are no arithmetic sequences made up of three 1-, 2-, or 3-digit primes, exhibiting this property, but there is one other 4-digit increasing sequence.

# What 12-digit number do you form by concatenating the three terms in this sequence?


require 'prime'

# Generate all 4-digit primes
primes_4_digit = Prime.each(9999).select { |p| p >= 1000 }
prime_set_4_digit = primes_4_digit.to_set # For quick lookups

primes_4_digit.each do |p1|
  # Calculate the next two terms in the arithmetic sequence
  p2 = p1 + 3330
  p3 = p1 + 2 * 3330

  # Check if p2 and p3 are also 4-digit primes
  next unless p3 < 10000 # Ensure p3 is still a 4-digit number
  next unless prime_set_4_digit.include?(p2) && prime_set_4_digit.include?(p3)

  # Check if they are permutations of each other
  s1_sorted = p1.to_s.chars.sort
  s2_sorted = p2.to_s.chars.sort
  s3_sorted = p3.to_s.chars.sort

  if s1_sorted == s2_sorted && s2_sorted == s3_sorted
    # Skip the example sequence 1487, 4817, 8147
    next if p1 == 1487

    # Found the other sequence, print the concatenated 12-digit number
    puts "#{p1}#{p2}#{p3}"
    break # Assuming there's only one other such sequence as implied by "one other"
  end
end
