# Prime digit replacements
# Problem 51
# By replacing the 1st digit of the 2-digit number *3, it turns out that six of the nine possible values: 13, 23, 43, 53, 73, and 83, are all prime.

# By replacing the 3rd and 4th digits of 56**3 with the same digit, this 5-digit number is the first example having seven primes among the ten generated numbers, yielding the family: 56003, 56113, 56333, 56443, 56663, 56773, and 56993. Consequently 56003, being the first member of this family, is the smallest prime with this property.

# Find the smallest prime which, by replacing part of the number (not necessarily adjacent digits) with the same digit, is part of an eight prime value family.

require 'prime'

# Returns the smallest prime which, by replacing part of the number (not necessarily adjacent digits)
# with the same digit, is part of an eight prime value family.
def solve_problem_51
  Prime.each do |p|
    s_p = p.to_s
    num_digits = s_p.length
    next if num_digits < 2

    (1...num_digits).each do |num_to_replace| # This is the number of digits to be replaced, e.g., 1, 2, or 3 asterisks
      (0...num_digits).to_a.combination(num_to_replace) do |indices_to_replace| # Choose which specific digit positions to replace
        # The original digits at indices_to_replace do NOT need to be the same.
        # For example, in 56xx3, the 'xx' can be different original digits.
        # The line `next unless digits_at_indices.uniq.length == 1` was an incorrect interpretation.
        # It has been removed.

        prime_family = []
        ('0'..'9').each do |replacement_char| # Try replacing with '0' through '9'
          # Skip leading zero for multi-digit numbers
          if indices_to_replace.include?(0) && replacement_char == '0' && num_digits > 1
            next
          end

          candidate_chars = s_p.chars
          indices_to_replace.each { |idx| candidate_chars[idx] = replacement_char }
          candidate_str = candidate_chars.join
          next if candidate_str.length != num_digits

          candidate_num = candidate_str.to_i
          if Prime.prime?(candidate_num)
            prime_family << candidate_num
          end
        end

        if prime_family.count == 8
          return prime_family.min
        end
      end
    end
  end
end

puts solve_problem_51
