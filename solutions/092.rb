#  <p>A number chain is created by continuously adding the square of the digits in
# a number to form a new number until it has been seen before.</p>
# <p>For example,
# \begin{align}
# &amp;44 \to 32 \to 13 \to 10 \to \mathbf 1 \to \mathbf 1\\
# &amp;85 \to \mathbf{89} \to 145 \to 42 \to 20 \to 4 \to 16 \to 37 \to 58 \to \ma
# thbf{89}
# \end{align}
# </p><p>Therefore any chain that arrives at $1$ or $89$ will become stuck in an e
# ndless loop. What is most amazing is that EVERY starting number will eventually
# arrive at $1$ or $89$.</p>
# <p>How many starting numbers below ten million will arrive at $89$?</p>

def sum_of_squares_of_digits(n)
  sum = 0
  while n > 0
    digit = n % 10
    sum += digit * digit
    n /= 10
  end
  sum
end

def arrives_at_89?(n)
  while n != 1 && n != 89
    n = sum_of_squares_of_digits(n)
  end
  n == 89
end

# Fast solution using digit sum counting
def solve_092_fast
  # For 7-digit numbers, max sum of squares is 7 * 9^2 = 567
  max_sum = 7 * 81
  
  # Precompute which sums lead to 89
  leads_to_89 = Array.new(max_sum + 1, false)
  (1..max_sum).each do |i|
    leads_to_89[i] = arrives_at_89?(i)
  end
  
  count = 0
  
  # Count numbers with 1 to 7 digits
  (1..7).each do |num_digits|
    count += count_numbers_with_digits(num_digits, leads_to_89)
  end
  
  count
end

def count_numbers_with_digits(num_digits, leads_to_89)
  # Use dynamic programming to count numbers with given digit count
  # that produce each possible sum of squares
  
  # dp[i][s] = number of i-digit numbers that produce sum s
  dp = Array.new(num_digits + 1) { Array.new(568, 0) }
  
  # Base case: 0 digits produces sum 0
  dp[0][0] = 1
  
  # Fill the DP table
  (0...num_digits).each do |i|
    (0..567).each do |sum|
      next if dp[i][sum] == 0
      
      # Try each digit 0-9
      (0..9).each do |digit|
        # Skip leading zeros for multi-digit numbers
        next if i == 0 && num_digits > 1 && digit == 0
        
        new_sum = sum + digit * digit
        next if new_sum > 567
        
        dp[i + 1][new_sum] += dp[i][sum]
      end
    end
  end
  
  # Count numbers that lead to 89
  total = 0
  (1..567).each do |sum|
    total += dp[num_digits][sum] if leads_to_89[sum]
  end
  
  total
end

# Original slower solution for comparison
def solve_092_original
  memo = {}
  count = 0
  limit = 10_000_000
  
  (1...limit).each do |i|
    count += 1 if arrives_at_89_memo?(i, memo)
  end
  
  count
end

def arrives_at_89_memo?(n, memo = {})
  original = n
  path = []
  
  while n != 1 && n != 89
    return memo[n] if memo.key?(n)
    path << n
    n = sum_of_squares_of_digits(n)
  end
  
  result = (n == 89)
  
  # Memoize all numbers in the path
  path.each { |num| memo[num] = result }
  
  result
end

# Run both solutions for comparison
start_time = Time.now
result_fast = solve_092_fast
end_time = Time.now
puts "Numbers below 10 million that arrive at 89: #{result_fast}"
