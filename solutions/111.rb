require 'prime'

class Euler111
  def initialize(n)
    @n = n
  end

  def is_prime?(num)
    # Prime.prime? is efficient and handles num <= 1 correctly.
    Prime.prime?(num)
  end

  def solve
    total_s_sum = 0

    (0..9).each do |d| # For each digit d
      s_n_d = 0
      # M(n,d) is the largest k_repeats for which primes exist.
      # We iterate k_repeats from @n down to 1.
      @n.downto(1) do |k_repeats|
        num_other_digits = @n - k_repeats
        
        current_sum_for_this_k = 0
        current_count_for_this_k = 0

        # Get all combinations of positions for the 'd' digit
        # (0...@n).to_a gives [0, 1, ..., @n-1]
        (0...@n).to_a.combination(k_repeats).each do |d_positions|
          # d_positions is an array of indices where 'd' will be placed.
          
          if num_other_digits == 0
            # All digits are 'd'
            num_str = d.to_s * @n
            # If d=0, num_str is "00...0", num is 0. is_prime?(0) is false.
            # No explicit 'leading zero' check needed here as d=0 and @n > 1 makes it non-prime.
            # Or, if d!=0, it's a repdigit like 11...1.
            num = num_str.to_i 
            if is_prime?(num)
              current_sum_for_this_k += num
              current_count_for_this_k += 1
            end
          else
            # There are 'num_other_digits' to fill.
            other_positions = (0...@n).to_a - d_positions
            
            candidate_other_digits = (0..9).to_a.reject { |digit| digit == d }

            # Generate all sequences of length num_other_digits from candidate_other_digits
            candidate_other_digits.repeated_permutation(num_other_digits).each do |other_digits_sequence|
              num_arr = Array.new(@n)
              
              d_positions.each { |pos| num_arr[pos] = d.to_s }
              
              other_positions.each_with_index do |pos, i|
                num_arr[pos] = other_digits_sequence[i].to_s
              end

              # Critical check: n-digit numbers cannot start with '0' (unless n=1)
              next if @n > 1 && num_arr[0] == '0'

              num_str = num_arr.join
              num = num_str.to_i
              
              if is_prime?(num)
                current_sum_for_this_k += num
                current_count_for_this_k += 1
              end
            end # repeated_permutation loop
          end # else (num_other_digits > 0)
        end # d_positions.combination loop

        if current_count_for_this_k > 0
          # These are the primes for M(n,d) = k_repeats.
          # S(n,d) is the sum of these primes.
          s_n_d = current_sum_for_this_k
          # N(n,d) = current_count_for_this_k (not strictly needed for final sum)
          break # Found M(n,d) and S(n,d) for this d, so break from k_repeats loop
        end
      end # k_repeats loop (from @n down to 1)

      total_s_sum += s_n_d
    end # d loop (0 to 9)

    puts total_s_sum
  end
end

if __FILE__ == $PROGRAM_NAME
  # Project Euler Problem 111 specifies n=10
  solver = Euler111.new(10)
  solver.solve
end
