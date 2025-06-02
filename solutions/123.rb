# Problem 123: Prime Square Remainders
#
# Problem Statement:
# Let $p_n$ be the $n$th prime: $2, 3, 5, 7, 11, \dots$, and let $r$ be the
# remainder when $(p_n - 1)^n + (p_n + 1)^n$ is divided by $p_n^2$.
#
# For example, when $n = 3$, $p_3 = 5$, and $4^3 + 6^3 = 280 \equiv 5 \mod 25$.
#
# The least value of $n$ for which the remainder first exceeds $10^9$ is $7037$.
#
# Find the least value of $n$ for which the remainder first exceeds $10^{10}$.
#
# Notes:
# The problem asks for the least value of n for which the remainder r when ((p_n-1)^n + (p_n+1)^n) is divided by p_n^2 first exceeds 10^10.
# The remainder simplifies to 2np_n for odd n (and 2 for even n, which is too small).
# The script implements a Sieve of Eratosthenes to generate primes (up to around 400,000).
# It then iterates through odd values of n, finds the n-th prime p_n, calculates R = 2*n*p_n,
# and stops when R first exceeds 10^10, outputting that n.
# The solution found is 21035.

# Full Ruby script content from temp_problem_123.rb:

class Problem123Solver
  TARGET_REMAINDER_THRESHOLD = 10**10 # Target is 10^10
  # Based on n ~ 21035, p_n ~ 2.1e5 * ln(2.1e5) ~ 2.1e5 * 9.95 ~ 2.1e5.
  # More accurately, p_21035 ~ 237803 (from online sources).
  # A sieve limit of 300,000 was estimated. Using 400,000 as in previous successful runs for safety.
  PRIME_SIEVE_UPPER_BOUND = 400_000

  def initialize
    @primes = sieve(PRIME_SIEVE_UPPER_BOUND)
  end

  # Sieve of Eratosthenes to generate primes up to a given limit.
  def sieve(limit)
    is_prime = Array.new(limit + 1, true)
    is_prime[0] = is_prime[1] = false # 0 and 1 are not prime.
    (2..Math.sqrt(limit).to_i).each do |p|
      if is_prime[p]
        (p * p..limit).step(p) do |i|
          is_prime[i] = false # Mark multiples of p as not prime.
        end
      end
    end
    primes_list = []
    (2..limit).each { |p| primes_list << p if is_prime[p] }
    primes_list
  end

  def solve
    # Iterate using n_val as the 1-based index for the n-th prime (problem's definition of n)
    (1..@primes.length).each do |n_val|
      # The problem states that for odd n, the remainder r = 2*n*p_n (mod p_n^2).
      # For even n, r = 2. We are interested when r > 10^10, so n must be odd.
      next if n_val.even? # Skip even n_val, as their remainder (2) is too small.

      # Get the n-th prime (p_n). @primes array is 0-indexed.
      # Ensure n_val-1 is a valid index for @primes.
      if n_val - 1 >= @primes.length
        # This block should ideally not be reached if PRIME_SIEVE_UPPER_BOUND is sufficient.
        $stderr.puts "Error: Attempting to access prime out of sieve bounds."
        $stderr.puts "n_val = #{n_val}, but only #{@primes.length} primes generated."
        $stderr.puts "Increase PRIME_SIEVE_UPPER_BOUND."
        return # Or raise an error
      end
      p_n = @primes[n_val - 1]
      
      # Calculate the term 2 * n * p_n.
      # The problem simplifies to checking when this term (not its value mod p_n^2) exceeds the threshold,
      # because for the large n where this occurs, 2*n < p_n, so 2*n*p_n mod p_n^2 = 2*n*p_n.
      remainder_term = 2 * n_val * p_n

      if remainder_term > TARGET_REMAINDER_THRESHOLD
        puts n_val # Output the least value of n_val satisfying the condition.
        return     # Terminate after finding the first such n_val.
      end
    end
    
    # If the loop completes, it means no n_val satisfied the condition within the generated primes range.
    $stderr.puts "Error: Solution not found within the current prime sieve limit."
    $stderr.puts "Max n_val explored: #{@primes.length}. Max p_n considered: #{@primes.last}."
    $stderr.puts "Consider increasing PRIME_SIEVE_UPPER_BOUND if the known answer wasn't found."
  end
end

# Create an instance of the solver and run the solve method.
solver = Problem123Solver.new
solver.solve

