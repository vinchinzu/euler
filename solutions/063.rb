# Powerful digit counts

# <p>The $5$-digit number, $16807=7^5$, is also a fifth power. Similarly, the $9$-digit number, $134217728=8^9$, is a ninth power.</p>
# <p>How many $n$-digit positive integers exist which are also an $n$th power?</p>

# Powerful digit counts
# Project Euler Problem 63
#
# The 5-digit number, 16807=7^5, is also a fifth power.
# Similarly, the 9-digit number, 134217728=8^9, is a ninth power.
# How many n-digit positive integers exist which are also an nth power?

def solve_powerful_digit_counts
  count = 0

  # Determine the range for n (the power and number of digits):
  # We need b^n to be an n-digit number.
  # So, 10^(n-1) <= b^n < 10^n.
  # From b^n < 10^n, it implies b < 10. So, b is in [1, 9].
  # From 10^(n-1) <= b^n, it implies 10^((n-1)/n) <= b.
  # Since b <= 9, we must have 10^((n-1)/n) <= 9.
  # 10^(1 - 1/n) <= 9
  # log10(10^(1 - 1/n)) <= log10(9)
  # 1 - 1/n <= log10(9)
  # 1 - log10(9) <= 1/n
  # n <= 1 / (1 - log10(9))
  # log10(9) is approx 0.95424.
  # n <= 1 / (1 - 0.95424) = 1 / 0.04576 approx 21.85.
  # So, the maximum integer value for n is 21.
  # We loop n from 1 to 21.

  (1..21).each do |n| # n is the power and also the required number of digits
    (1..9).each do |b| # b is the base
      # Calculate b^n. Ruby handles large integers automatically.
      power_val = b**n

      # Calculate the number of digits in power_val.
      # For positive integers, converting to string and getting length is a reliable way.
      num_digits = power_val.to_s.length

      if num_digits == n
        count += 1
        # For verification, you can uncomment the line below:
        # puts "#{b}^#{n} = #{power_val} (Digits: #{num_digits}, Power: #{n})"
      elsif num_digits > n
        # If b^n has more than n digits, then for this n, (b+1)^n will also have too many.
        # However, for b <= 9, b^n < 10^n, so b^n will always have at most n digits.
        # This condition (num_digits > n) will not be met for b <= 9.
        # If it could be met, we could 'break' the inner loop here.
        # (e.g., if b could be >= 10)
      end
      # If num_digits < n, then b^n is too small for the current n and b.
      # We simply continue to the next b or n. The outer loop limit on n (1..21)
      # ensures we stop checking n when it's no longer possible for 9^n
      # (the largest possible base case) to have n digits.
    end
  end

  count
end

# To run the solution and print the result:
result = solve_powerful_digit_counts
puts result
# Expected output: 49