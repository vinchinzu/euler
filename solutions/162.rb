# Project Euler Problem 162: Hexadecimal numbers

class HexadecimalCounter
  def initialize
    # Sizes of character sets
    @size_U = 16
    @size_U_minus_one_char = 15 # e.g., U-{0}, U-{1}, U-{A}
    @size_U_minus_two_chars = 14 # e.g., U-{0,1}, U-{0,A}, U-{1,A}
    @size_U_minus_three_chars = 13 # e.g., U-{0,1,A}
  end

  # Helper to calculate base^exp
  # Ruby's ** operator handles large integers, so this is mostly for clarity if needed,
  # but direct use of ** is fine.
  def power(base, exp)
    return 1 if exp == 0
    base**exp
  end

  # Calculates the count of L-digit numbers formed from a character set of a given size.
  # - length: The number of digits (L).
  # - set_size: The number of available unique characters in the allowed set.
  # - zero_is_in_set: Boolean, true if '0' is one of the allowed characters.
  def count_l_digits_from_set(length, set_size, zero_is_in_set)
    # If length is 0, or no characters are allowed to form the number (and length > 0).
    return 0 if length == 0
    return 0 if set_size == 0 && length > 0

    # If length is 1 digit
    if length == 1
      # Any character from the set can form a 1-digit number.
      # This includes "0" if zero_is_in_set is true and '0' is part of the set.
      return set_size
    end

    # If length > 1 digit
    if zero_is_in_set
      # The first digit cannot be '0'.
      # (set_size - 1) choices for the first digit.
      # set_size choices for each of the remaining (length - 1) digits.
      # If set_size is 1 (meaning the set is just {'0'}), then (1-1) = 0 choices for the first digit. Correct.
      return (set_size - 1) * power(set_size, length - 1)
    else
      # '0' is not in the allowed character set. No restriction on the first digit other than being in the set.
      # set_size choices for each of the 'length' digits.
      return power(set_size, length)
    end
  end

  def solve
    grand_total_count = 0

    (1..16).each do |l| # Iterate for each length L from 1 to 16
      # Calculate terms for the Inclusion-Exclusion Principle for length L

      # All valid L-digit numbers from U = {0-9, A-F}
      # |U| = 16, '0' is in U.
      term_s_all = count_l_digits_from_set(l, @size_U, true)

      # Numbers missing '0' (digits from U-{0})
      # |U-{0}| = 15, '0' is NOT in U-{0}.
      term_s_no0 = count_l_digits_from_set(l, @size_U_minus_one_char, false)

      # Numbers missing '1' (digits from U-{1})
      # |U-{1}| = 15, '0' IS in U-{1}.
      term_s_no1 = count_l_digits_from_set(l, @size_U_minus_one_char, true)

      # Numbers missing 'A' (digits from U-{A})
      # |U-{A}| = 15, '0' IS in U-{A}.
      term_s_noA = count_l_digits_from_set(l, @size_U_minus_one_char, true)

      # Sum of terms missing one required character
      sum_missing_one = term_s_no0 + term_s_no1 + term_s_noA

      # Numbers missing '0' and '1' (digits from U-{0,1})
      # |U-{0,1}| = 14, '0' is NOT in U-{0,1}.
      term_s_no01 = count_l_digits_from_set(l, @size_U_minus_two_chars, false)

      # Numbers missing '0' and 'A' (digits from U-{0,A})
      # |U-{0,A}| = 14, '0' is NOT in U-{0,A}.
      term_s_no0A = count_l_digits_from_set(l, @size_U_minus_two_chars, false)

      # Numbers missing '1' and 'A' (digits from U-{1,A})
      # |U-{1,A}| = 14, '0' IS in U-{1,A}.
      term_s_no1A = count_l_digits_from_set(l, @size_U_minus_two_chars, true)

      # Sum of terms missing two required characters
      sum_missing_two = term_s_no01 + term_s_no0A + term_s_no1A

      # Numbers missing '0', '1', and 'A' (digits from U-{0,1,A})
      # |U-{0,1,A}| = 13, '0' is NOT in U-{0,1,A}.
      term_s_no01A = count_l_digits_from_set(l, @size_U_minus_three_chars, false)

      # Apply Inclusion-Exclusion Principle for length L
      count_for_l = term_s_all - sum_missing_one + sum_missing_two - term_s_no01A

      grand_total_count += count_for_l
    end

    grand_total_count
  end
end

if __FILE__ == $PROGRAM_NAME
  counter = HexadecimalCounter.new
  result = counter.solve
  puts result
end
```
