# Project Euler Problem 168: Number Rotations

class NumberRotations
  def initialize(max_digits)
    @max_digits = max_digits # d_max, e.g. 99 for n < 10^100
    @total_sum_of_last_5_digits = 0
  end

  def solve
    # pow10_d_minus_1 will store 10^(d-1)
    # It starts at 10 for d=2 (10^1)
    pow10_d_minus_1 = 10

    (2..@max_digits).each do |d| # d is the number of digits in n
      # Define bounds for A, which has d-1 digits.
      # lower_bound_A is 10^(d-2) for d > 2, or 1 for d=2.
      # upper_bound_A is 10^(d-1) (exclusive).
      lower_bound_A = (d == 2) ? 1 : (pow10_d_minus_1 / 10)
      upper_bound_A = pow10_d_minus_1

      (1..9).each do |b| # b is the last digit of n (1-9)
        (1..9).each do |k| # k is the multiplier (1-9)

          # A = b * (10^(d-1) - k) / (10*k - 1)
          # Numerator for A: b * (10^(d-1) - k)
          # Check for potential negative result if 10^(d-1) < k, though this won't happen here
          # because d>=2 => 10^(d-1) >= 10, and k <= 9. So (10^(d-1) - k) is positive.
          numerator_A = b * (pow10_d_minus_1 - k)
          denominator_A = 10 * k - 1 # This is never zero (k is 1-9)

          # A must be an integer, so numerator_A must be divisible by denominator_A.
          if numerator_A % denominator_A == 0
            a = numerator_A / denominator_A

            # 'a' must be positive. Since b > 0 and (pow10_d_minus_1 - k) > 0, a will be > 0.
            # Check if 'a' has the correct number of digits (d-1 digits).
            if a >= lower_bound_A && a < upper_bound_A
              # Found a valid number n = 10*A + b.
              # We need its last 5 digits. This is effectively (10*A + b) % 100000.
              # To handle potentially very large A, we can use (A % 10000) for calculation.
              a_mod_10000 = a % 10000

              # The value formed by the last 5 digits of n.
              # Example: if n = ...cdeef and we need last 5 digits 'cdeef'.
              # A = ...cde, b = f.
              # last_5_value = 10 * (A % 10000) + b = 10 * cde + f (assuming cde < 10000)
              # This correctly forms the number represented by the sequence of digits.
              # This value is at most 10*9999 + 9 = 99999.
              n_last_digits_value = 10 * a_mod_10000 + b

              @total_sum_of_last_5_digits += n_last_digits_value
            end
          end
        end
      end

      # Prepare power of 10 for the next number of digits d
      # No need to check d < @max_digits before multiplication if @max_digits is the loop limit.
      # Ruby handles large integers, direct multiplication is fine.
      pow10_d_minus_1 *= 10
    end

    # The problem asks for the sum modulo 100000
    @total_sum_of_last_5_digits % 100000
  end
end

if __FILE__ == $PROGRAM_NAME
  # n < 10^100 means n can have at most 99 digits (d_max = 99).
  solver = NumberRotations.new(99)
  result = solver.solve
  puts result
end
```
