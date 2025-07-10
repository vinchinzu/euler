# Project Euler Problem 164: Numbers for which no three consecutive digits have a sum greater than 9

class DigitSumCounter
  def solve
    # dp[length][second_last_digit][last_digit]
    # dimensions: (20 lengths + 1 placeholder) x 10 x 10
    dp = Array.new(21) { Array.new(10) { Array.new(10, 0) } }

    # Base cases for 1-digit numbers (length = 1)
    # second_last_digit is a placeholder (e.g., 0)
    # last_digit (d1) must be from 1 to 9.
    (1..9).each do |d1|
      dp[1][0][d1] = 1
    end

    # Iterate for lengths from 2 to 20
    (2..20).each do |l|
      (0..9).each do |d_last|      # d_l (current last digit)
        (0..9).each do |d_sec_last| # d_{l-1} (current second last digit)

          if l == 2
            # Numbers are d_sec_last d_last.
            # d_sec_last is the first digit, so it cannot be 0.
            if d_sec_last != 0
              # dp[1][0][d_sec_last] is 1 if d_sec_last is in [1,9], else 0.
              # This comes from appending d_last to the 1-digit number d_sec_last.
              dp[l][d_sec_last][d_last] = dp[1][0][d_sec_last]
            end
          else # l >= 3
            # Numbers are ... d_third_last d_sec_last d_last
            # d_third_last is d_{l-2}
            current_sum_val = 0
            (0..9).each do |d_third_last|
              if d_third_last + d_sec_last + d_last <= 9
                current_sum_val += dp[l-1][d_third_last][d_sec_last]
              end
            end
            dp[l][d_sec_last][d_last] = current_sum_val
          end
        end
      end
    end

    # Calculate total sum for length 20
    total_sum = 0
    (0..9).each do |d_sec_last|
      (0..9).each do |d_last|
        total_sum += dp[20][d_sec_last][d_last]
      end
    end

    total_sum
  end
end

if __FILE__ == $PROGRAM_NAME
  counter = DigitSumCounter.new
  result = counter.solve
  # The script should print only the final answer
  puts result
end
```
