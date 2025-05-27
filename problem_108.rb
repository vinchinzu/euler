# Problem 108: Diophantine reciprocals I

class DiophantineReciprocalsI
  PRIMES = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29] # First 10 primes
  TARGET_EXP_PRODUCT = 2000 # Since d(n^2) > 1999 means d(n^2) >= 2000

  def initialize
    @min_n_found = Float::INFINITY
  end

  def solve
    # Max exponent for the first prime (2).
    # Based on n=180180 = 2^3 * 3^3 * 5 * 7 * 11 * 13, exponents are small.
    # Max exponent for 2^a for n around 10^5 - 10^6 would be around log2(10^6) approx 20.
    # Initial call with a generous limit for the first prime's exponent.
    find_min_n_recursive(0, 1, 1, 20) 
    puts @min_n_found.to_i
  end

  private

  def find_min_n_recursive(prime_idx, product_so_far, n_so_far, prev_exponent_limit)
    current_prime = PRIMES[prime_idx]
    
    # Loop for exponent 'a' for the current prime
    # current_prime_power_val = 1 # p^0
    
    (1..prev_exponent_limit).each do |exponent|
      # current_prime_power_val *= current_prime # p^exponent
      # Using ** for clarity, though iterative multiplication is often more efficient for large exponents
      # However, exponents here are expected to be small.
      
      # Calculate p^exponent safely
      # Check if current_prime is nil (should not happen if prime_idx is managed)
      # or if exponent is too large leading to huge numbers before multiplication
      term_val = current_prime ** exponent # Ruby handles large integers

      # Prospective n value if this prime with this exponent is included
      # Check for potential overflow before multiplication if n_so_far is already huge.
      # Ruby's Bignums handle large numbers, so direct overflow to Float::INFINITY is not typical
      # unless numbers become astronomically large, exceeding memory.
      # The primary check is against @min_n_found.
      if n_so_far > @min_n_found / term_val # More robust check to prevent large intermediate n
                                          # if term_val is small but n_so_far is already near @min_n_found limit.
        current_n_with_prime = @min_n_found # Effectively makes it too large for next check
      else
        current_n_with_prime = n_so_far * term_val
      end
      
      # Pruning 1: If current_n_with_prime is already worse than or equal to best_n found,
      # then increasing this exponent or adding more primes from this path won't help.
      break if current_n_with_prime >= @min_n_found

      new_exp_product = product_so_far * (2 * exponent + 1)

      if new_exp_product >= TARGET_EXP_PRODUCT
        # This combination of exponents yields enough divisors.
        # current_n_with_prime is a candidate for the smallest n.
        # No need to check against @min_n_found here, as the earlier break handles it.
        # If it passed the break, it must be < @min_n_found.
         @min_n_found = current_n_with_prime
      end
      
      # Recursive call for the next prime, if available
      if prime_idx + 1 < PRIMES.length
        # Pass `exponent` as the `prev_exponent_limit` for the next prime,
        # ensuring that exponents are non-increasing.
        find_min_n_recursive(prime_idx + 1, new_exp_product, current_n_with_prime, exponent)
      end
    end
  end
end

# --- Main execution ---
solver = DiophantineReciprocalsI.new
solver.solve
```
