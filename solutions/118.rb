require 'prime' # For Prime.prime?
require 'set'   # For Set data structure

class Euler118
  def initialize
    @solutions_set = Set.new
  end

  def is_prime?(n)
    # Single digit 1 is not prime.
    # Digits are 1-9, so n > 0 always.
    return false if n == 1 
    Prime.prime?(n)
  end

  # Generates partitions of 's' into primes, ensuring primes are in increasing order.
  # s: the string of digits (a permutation of '1'..'9')
  # start_index: current position in 's' to start forming the next number
  # current_primes: list of primes formed so far in this partition
  def find_partitions(s, start_index, current_primes)
    if start_index == s.length
      # All digits used, a valid partition is formed.
      # Store a frozen version of the sorted list to ensure hashability for the Set.
      @solutions_set.add(current_primes.sort.freeze)
      return
    end

    (start_index...s.length).each do |i|
      num_str = s[start_index..i]
      num = num_str.to_i

      # Optimization: If current_primes list is not empty,
      # the new number must be greater than the last prime added.
      # This ensures that for any given permutation 's', each set of primes
      # is generated in a canonical (sorted) order, preventing duplicates from
      # different orderings within that permutation's partitioning.
      if !current_primes.empty? && num <= current_primes.last
        next
      end

      if is_prime?(num)
        # If num is prime, recurse with the remaining digits
        # current_primes + [num] creates a new array for the next call
        find_partitions(s, i + 1, current_primes + [num])
      end
    end
  end

  def solve
    digits = ('1'..'9').to_a
    
    # Iterate through all permutations of the digits
    digits.permutation.each do |p_array|
      perm_string = p_array.join
      find_partitions(perm_string, 0, [])
    end
    
    puts @solutions_set.size
  end
end

if __FILE__ == $PROGRAM_NAME
  solver = Euler118.new
  solver.solve
end
