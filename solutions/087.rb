#  <p>The smallest number expressible as the sum of a prime square, prime cube, and
#  prime fourth power is $28$. In fact, there are exactly four numbers below fifty
#  that can be expressed in such a way:</p>
# \begin{align}
# 28 &amp;= 2^2 + 2^3 + 2^4\\
# 33 &amp;= 3^2 + 2^3 + 2^4\\
# 49 &amp;= 5^2 + 2^3 + 2^4\\
# 47 &amp;= 2^2 + 3^3 + 2^4
# \end{align}
# <p>How many numbers below fifty million can be expressed as the sum of a prime s
# quare, prime cube, and prime fourth power?</p>

# Solution for Project Euler Problem 87
require 'set'

LIMIT = 50_000_000

# Step 1: Generate Prime Numbers
# Determine maximum possible prime needed for each power type.
# p1^2 < LIMIT => p1 < sqrt(LIMIT)
p1_max_val = Math.sqrt(LIMIT).floor # approx 7071

# p2^3 < LIMIT => p2 < cbrt(LIMIT)
p2_max_val = (LIMIT**(1.0/3.0)).floor # approx 368

# p3^4 < LIMIT => p3 < LIMIT^(1/4)
p3_max_val = (LIMIT**(1.0/4.0)).floor # approx 84

# We need primes up to the largest of these individual maximums.
# However, for the loops, we'll use specific lists or iterate primes up to these specific limits.
# The sieve should go up to the overall maximum prime we might need, which is p1_max_val.
sieve_limit = p1_max_val
primes_list = []
is_prime_sieve = Array.new(sieve_limit + 1, true)
is_prime_sieve[0] = is_prime_sieve[1] = false

(2..Math.sqrt(sieve_limit).floor).each do |p|
  if is_prime_sieve[p]
    (p * p).step(sieve_limit, p) do |multiple|
      is_prime_sieve[multiple] = false
    end
  end
end

(2..sieve_limit).each do |num|
  primes_list << num if is_prime_sieve[num]
end

# Filter primes for each specific power based on their individual limits
# This is more efficient for the loops later.
primes_for_square = primes_list.select { |p| p <= p1_max_val } # Actually, this is just primes_list
primes_for_cube   = primes_list.select { |p| p <= p2_max_val }
primes_for_fourth = primes_list.select { |p| p <= p3_max_val }

# Step 2: Store unique sums found
found_sums = Set.new

# Step 3: Iterate through combinations
primes_for_fourth.each do |p3|
  p3_fourth = p3**4
  break if p3_fourth >= LIMIT # Optimization

  primes_for_cube.each do |p2|
    p2_cube = p2**3
    sum_p3_p2 = p3_fourth + p2_cube
    break if sum_p3_p2 >= LIMIT # Optimization

    primes_for_square.each do |p1| # Use the full primes_list for p1 or the filtered one
      p1_square = p1**2
      current_total_sum = sum_p3_p2 + p1_square

      if current_total_sum < LIMIT
        found_sums.add(current_total_sum)
      else
        # Since primes_for_square is sorted, further p1 values will also exceed the LIMIT.
        break # Optimization for the innermost loop
      end
    end
  end
end

# The result is the number of unique sums found
result_count = found_sums.size
puts "The number of values below #{LIMIT} expressible as such a sum is: #{result_count}"
