#  <p>It is possible to write ten as the sum of primes in exactly five different wa
# ys:</p>
# \begin{align}
# &amp;7 + 3\\
# &amp;5 + 5\\
# &amp;5 + 3 + 2\\
# &amp;3 + 3 + 2 + 2\\
# &amp;2 + 2 + 2 + 2 + 2
# \end{align}
# <p>What is the first value which can be written as the sum of primes in over fiv
# e thousand different ways?</p>

# Solution for Project Euler Problem 77

# This problem is similar to integer partitioning (Problem 76), but the parts
# must be prime numbers. We use dynamic programming.

TARGET_WAYS = 5000
# N_LIMIT is an estimate for the maximum sum we need to check.
# If the answer is larger than N_LIMIT, this value would need to be increased.
# Based on typical problem scales and OEIS A000607 (number of partitions of n into prime parts),
# the answer is expected to be relatively small (e.g., < 100).
N_LIMIT = 100 # Initial guess, problem constraints usually keep this manageable.

# Step 1: Generate primes up to N_LIMIT using a sieve
primes_list = []
is_prime_sieve = Array.new(N_LIMIT + 1, true)
is_prime_sieve[0] = is_prime_sieve[1] = false

(2..Math.sqrt(N_LIMIT).floor).each do |p|
  if is_prime_sieve[p]
    (p * p).step(N_LIMIT, p) do |multiple|
      is_prime_sieve[multiple] = false
    end
  end
end

(2..N_LIMIT).each do |num|
  primes_list << num if is_prime_sieve[num]
end

# Step 2: Dynamic Programming to count ways to sum to n using primes
# ways[i] will store the number of ways to write 'i' as a sum of primes.
ways = Array.new(N_LIMIT + 1, 0)
ways[0] = 1 # Base case: one way to make sum 0 (by using no primes)

# For each prime number available as a "part"
primes_list.each do |prime|
  # Update the ways array for sums from 'prime' up to N_LIMIT
  (prime..N_LIMIT).each do |current_sum|
    # The number of ways to make 'current_sum' can be increased by using the current 'prime'.
    # This is done by adding the number of ways to make 'current_sum - prime'.
    ways[current_sum] += ways[current_sum - prime]
  end
end

# Step 3: Find the first value with over TARGET_WAYS
result_value = -1
(2..N_LIMIT).each do |i| # Start checking from sum = 2
  if ways[i] > TARGET_WAYS
    result_value = i
    break
  end
end

if result_value != -1
  puts "The first value that can be written as the sum of primes in over #{TARGET_WAYS} ways is: #{result_value}"
else
  puts "No value found up to #{N_LIMIT} with over #{TARGET_WAYS} ways. Consider increasing N_LIMIT."
end
