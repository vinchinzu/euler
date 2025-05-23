require 'prime'

# Problem 60: Prime pair sets
# Find the lowest sum for a set of five primes for which any two primes
# concatenate to produce another prime.

class PrimePairSetsSolver
  # Initializes the solver.
  # prime_limit: The upper bound for primes to consider initially.
  # The largest prime in the actual solution set is 8389.
  # Setting a limit around 9000 to 10000 is reasonable.
  def initialize(prime_limit = 9500) # Increased slightly for safety margin
    @prime_limit = prime_limit
    
    # Generate primes up to the specified limit.
    # We can exclude 2 and 5 as they generally don't form such pairs
    # with other primes (e.g., concatenating with 2 makes an even number,
    # concatenating with 5 makes a number ending in 5).
    # This is a small optimization.
    @primes = Prime.each(@prime_limit).to_a
    @primes.reject! { |p| p == 2 || p == 5 } if @prime_limit > 5 # Avoid issues if limit is very small

    # Memoization caches
    @concat_pair_check_cache = {} # Stores results of check_concat_pair(p1, p2)
    @primality_cache = {}         # Stores primality test results for concatenated numbers
  end

  # Checks if a number n is prime.
  # Uses Ruby's Prime.prime? and memoizes the result.
  private def is_concatenated_num_prime?(n)
    return false if n <= 1 # Should not happen with positive prime concatenations
    return @primality_cache[n] if @primality_cache.key?(n)

    # Quick checks for large concatenated numbers (won't be 2 or 5 themselves)
    # If n is even (and not 2), it's not prime.
    # If n ends in 5 (and not 5), it's not prime.
    # These are mostly for clarity; Prime.prime? handles these.
    s_n = n.to_s # Convert to string once
    return (@primality_cache[n] = false) if n.even? && n != 2 
    return (@primality_cache[n] = false) if s_n.length > 1 && s_n.end_with?('5') && n != 5
    
    @primality_cache[n] = Prime.prime?(n)
    @primality_cache[n]
  end

  # Checks if two primes, p1 and p2, form a "concatenatable prime pair".
  # This means p1p2 (concatenation) and p2p1 are both prime.
  # Uses memoization.
  private def check_concat_pair(p1, p2)
    # Order primes for consistent cache key
    key = p1 < p2 ? [p1, p2] : [p2, p1]
    return @concat_pair_check_cache[key] if @concat_pair_check_cache.key?(key)

    num1_str = p1.to_s + p2.to_s
    num2_str = p2.to_s + p1.to_s

    # Numbers formed by concatenation can be large, check for overflow if not using Bignums.
    # Ruby handles large integers automatically.
    num1 = num1_str.to_i
    num2 = num2_str.to_i

    result = is_concatenated_num_prime?(num1) && is_concatenated_num_prime?(num2)
    @concat_pair_check_cache[key] = result
    result
  end

  # Finds the set of five primes with the specified property that has the lowest sum.
  public def find_lowest_sum_set
    min_sum = Float::INFINITY
    result_set = []
    num_primes = @primes.length

    # Iterate to find the first prime (p1)
    (0...num_primes).each do |i1|
      p1 = @primes[i1]
      # Pruning: If p1 * 5 (smallest possible sum with p1) is already >= current min_sum, stop.
      break if p1 * 5 >= min_sum

      # Iterate to find the second prime (p2)
      (i1 + 1...num_primes).each do |i2|
        p2 = @primes[i2]
        # Pruning: p1 + p2*4 is the smallest sum if p2,p3,p4,p5 are at least p2
        break if p1 + p2 * 4 >= min_sum
        next unless check_concat_pair(p1, p2)

        # Iterate to find the third prime (p3)
        (i2 + 1...num_primes).each do |i3|
          p3 = @primes[i3]
          break if p1 + p2 + p3 * 3 >= min_sum
          next unless check_concat_pair(p1, p3) && \
                      check_concat_pair(p2, p3)

          # Iterate to find the fourth prime (p4)
          (i3 + 1...num_primes).each do |i4|
            p4 = @primes[i4]
            break if p1 + p2 + p3 + p4 * 2 >= min_sum
            next unless check_concat_pair(p1, p4) && \
                        check_concat_pair(p2, p4) && \
                        check_concat_pair(p3, p4)
            
            # Iterate to find the fifth prime (p5)
            (i4 + 1...num_primes).each do |i5|
              p5 = @primes[i5]
              current_sum = p1 + p2 + p3 + p4 + p5
              # Pruning: If current_sum is already too large
              break if current_sum >= min_sum # Note: break, not next, as p5 increases

              if check_concat_pair(p1, p5) && \
                 check_concat_pair(p2, p5) && \
                 check_concat_pair(p3, p5) && \
                 check_concat_pair(p4, p5)
                
                # Found a valid set of five primes
                min_sum = current_sum
                result_set = [p1, p2, p3, p4, p5]
                # Optional: Print progress
                # puts "Found candidate set: #{result_set.sort.inspect}, Sum: #{min_sum}"
              end
            end # p5 loop
          end # p4 loop
        end # p3 loop
      end # p2 loop
    end # p1 loop

    if result_set.empty?
      puts "No solution found with prime limit #{@prime_limit}. Consider increasing it."
      return nil
    else
      puts "Lowest sum for a set of five primes: #{min_sum}"
      puts "Set: #{result_set.sort.inspect}"
      return min_sum
    end
  end
end

# --- How to run the solver ---
# This problem is computationally intensive and may take some time to run.
#
# start_time = Time.now
# solver = PrimePairSetsSolver.new(9500) # Max prime in known solution is 8389.
#                                        # Primes up to 9500 should be sufficient.
# lowest_sum = solver.find_lowest_sum_set
# end_time = Time.now
#
# puts "Calculation took #{end_time - start_time} seconds."
#
# Expected output for sum: 26033
# Expected set: [13, 5197, 5701, 6733, 8389] (order may vary before sort)
