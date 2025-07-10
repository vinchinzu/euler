# Project Euler Problem 167: Investigating Ulam sequences
require 'set'

class UlamSequenceSum
  def initialize(a, b, k_target)
    @a = a
    @b = b
    @k_target = k_target

    @u_list = [] # Stores the generated Ulam numbers
    # @u_set = Set.new # Not strictly needed if u_list is sorted and checked,
                       # but Ulam definition implies smallest integer *not already in sequence*.
                       # However, new terms are always > last term, so u_set for that check is not needed.
    @representations = Hash.new(0) # Stores sum_value => count_of_ways_to_form_sum
  end

  # Generates the next Ulam term and updates internal state
  def generate_next_term_and_update_state
    # Smallest candidate for the next term is the last term + 1
    # (or 'a' if list is empty, 'b' if list has 'a')
    # This method is called when @u_list has at least two terms.
    candidate = @u_list.last + 1

    loop do
      # A Ulam number must be formed by exactly one sum of two distinct earlier terms.
      if @representations.fetch(candidate, 0) == 1 # .fetch avoids creating new keys for misses
        new_term = candidate

        # Add sums involving the new_term and all previous distinct terms
        # Previous terms are already in @u_list
        @u_list.each do |existing_term|
          # new_term is guaranteed to be distinct from existing_term as it's larger
          @representations[existing_term + new_term] += 1
        end

        @u_list << new_term
        # @u_set.add(new_term) # Not needed due to increasing nature
        return # new_term is implicitly returned by modifying @u_list.last
      end
      candidate += 1
      # Safety break for unexpected behavior, though Ulam sequences are infinite
      raise "Candidate too large, something wrong" if candidate > (@u_list.last + 1_000_000) && @u_list.size < 100
    end
  end

  def calculate_sum_for_sequence
    # Initialize first two terms
    @u_list = [@a, @b]
    # @u_set.add(@a); @u_set.add(@b)
    @representations[@a + @b] = 1

    # Number of initial "irregular" terms to generate: L = 4n
    # Given b = 2n+1 => n = (b-1)/2. So L = 4 * (b-1)/2 = 2*(b-1).
    num_initial_terms_L = 2 * (@b - 1)

    # Handle cases where k_target is very small
    if @k_target == 0
      return 0
    elsif @k_target == 1
      return @a
    elsif @k_target == 2
      return @a + @b
    end
    # Ensure num_initial_terms_L is at least 2 if k_target > 2
    # because the AP logic relies on u_{L-1} and u_L.
    # Smallest n=2 => b=5. L = 2*(5-1) = 8. This is > 2.
    # So num_initial_terms_L will be >= 8.

    # Generate initial L terms (or k_target terms if k_target < L)
    # We already have 2 terms, so generate L-2 more (or k_target-2 more)
    num_to_generate_for_init_part = [@k_target, num_initial_terms_L].min - 2

    (0...num_to_generate_for_init_part).each do
      generate_next_term_and_update_state
    end

    initial_sum = @u_list.sum

    # If k_target <= num_initial_terms_L, we have summed all needed terms
    return initial_sum if @k_target <= num_initial_terms_L

    # --- Sum remaining terms using arithmetic progression properties ---
    # Remaining terms to sum: R = k_target - num_initial_terms_L
    r_terms_count = @k_target - num_initial_terms_L

    # Last two terms of the initial sequence (0-indexed list)
    u_L_minus_1 = @u_list[num_initial_terms_L - 2]
    u_L = @u_list[num_initial_terms_L - 1]

    # AP1: terms with odd indices relative to L+1. Starts with u_{L+1} = u_{L-1} + 4
    # AP2: terms with even indices relative to L+1. Starts with u_{L+2} = u_L + 4
    first_term_ap1 = u_L_minus_1 + 4
    first_term_ap2 = u_L + 4
    common_diff = 4

    num_terms_ap1 = (r_terms_count / 2.0).ceil.to_i
    num_terms_ap2 = (r_terms_count / 2.0).floor.to_i

    sum_ap1 = 0
    if num_terms_ap1 > 0
      # Sum = N * (2A + (N-1)D) / 2
      # All terms in this calculation are integers, result must be integer.
      sum_ap1 = num_terms_ap1 * (2 * first_term_ap1 + (num_terms_ap1 - 1) * common_diff) / 2
    end

    sum_ap2 = 0
    if num_terms_ap2 > 0
      sum_ap2 = num_terms_ap2 * (2 * first_term_ap2 + (num_terms_ap2 - 1) * common_diff) / 2
    end

    initial_sum + sum_ap1 + sum_ap2
  end
end

if __FILE__ == $PROGRAM_NAME
  grand_total_sum = 0
  k_target = 10**11

  # n from 2 to 10
  (2..10).each do |n|
    a = 2
    b = 2 * n + 1

    calculator = UlamSequenceSum.new(a, b, k_target)
    sum_for_sequence = calculator.calculate_sum_for_sequence
    grand_total_sum += sum_for_sequence
  end

  # The script should print only the final answer
  puts grand_total_sum
end
```
