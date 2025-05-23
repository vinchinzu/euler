# Problem 61: Cyclical figurate numbers
# Find the sum of the only ordered set of six cyclic 4-digit numbers for which each
# polygonal type: triangle, square, pentagonal, hexagonal, heptagonal, and octagonal,
# is represented by a different number in the set.

class CyclicalFigurateNumbersSolver
  def initialize
    # Formulas for P(k,n)
    # Index 0: Triangle (P3), 1: Square (P4), ..., 5: Octagonal (P8)
    @polygonal_formulas = [
      ->(n) { n * (n + 1) / 2 },    # P3
      ->(n) { n * n },                # P4
      ->(n) { n * (3 * n - 1) / 2 },  # P5
      ->(n) { n * (2 * n - 1) },      # P6
      ->(n) { n * (5 * n - 3) / 2 },  # P7
      ->(n) { n * (3 * n - 2) }       # P8
    ]

    # Ranges for n to generate 4-digit numbers (1000-9999)
    @n_ranges = [
      (45..140),  # P3: n=45 (1035) to n=140 (9870)
      (32..99),   # P4: n=32 (1024) to n=99 (9801)
      (26..81),   # P5: n=26 (1001) to n=81 (9921)
      (23..70),   # P6: n=23 (1035) to n=70 (9730)
      (21..63),   # P7: n=21 (1071) to n=63 (9828)
      (19..57)    # P8: n=19 (1045) to n=57 (9633)
    ]

    # @numbers_by_type_and_prefix[type_idx][prefix_val] = [list of full numbers]
    @numbers_by_type_and_prefix = Array.new(6) { Hash.new { |h, k| h[k] = [] } }
    # @all_numbers_of_type[type_idx] = [list of full numbers]
    @all_numbers_of_type = Array.new(6) { [] }

    generate_polygonal_numbers
    @solution_chain = nil
  end

  private

  # Generates all 4-digit polygonal numbers and stores them for quick lookup.
  def generate_polygonal_numbers
    (0..5).each do |type_idx|
      formula = @polygonal_formulas[type_idx]
      @n_ranges[type_idx].each do |n|
        val = formula.call(n)
        # We are only interested in 4-digit numbers
        if val >= 1000 && val <= 9999
          @all_numbers_of_type[type_idx] << val
          prefix = val / 100 # First two digits
          @numbers_by_type_and_prefix[type_idx][prefix] << val
        end
      end
    end
  end

  # Recursive Depth-First Search function to find the cycle.
  # chain: An array of numbers forming the current path.
  # used_types_mask: A bitmask indicating which polygonal types (0-5) are already in the chain.
  def dfs_find_cycle(chain, used_types_mask)
    # If a solution is already found, stop searching.
    return true if @solution_chain 

    current_length = chain.length
    last_number_in_chain = chain.last

    if current_length == 6
      # Chain of 6 numbers formed. Check if it's cyclic with the first number.
      first_number_in_chain = chain.first
      # Last two digits of last number == First two digits of first number
      if (last_number_in_chain % 100) == (first_number_in_chain / 100)
        @solution_chain = chain.dup # Found the solution
        return true
      else
        return false # Not cyclic
      end
    end

    # Determine the required prefix for the next number in the chain.
    # This is the last two digits of the current last_number_in_chain.
    required_prefix = last_number_in_chain % 100

    # Try to add a number from each unused polygonal type.
    (0..5).each do |next_type_idx|
      # Check if this type_idx is NOT already used
      if (used_types_mask & (1 << next_type_idx)) == 0
        # Get candidate numbers of this type that match the required_prefix
        candidate_numbers = @numbers_by_type_and_prefix[next_type_idx][required_prefix] || []
        
        candidate_numbers.each do |candidate_num|
          chain.push(candidate_num)
          # Recursively search with the new number and updated mask
          if dfs_find_cycle(chain, used_types_mask | (1 << next_type_idx))
            return true # Solution found and propagated up
          end
          chain.pop # Backtrack: remove the number if this path didn't lead to a solution
        end
      end
    end
    false # No solution found from this path
  end

  public

  # Solves the problem and returns the sum of the numbers in the unique cyclic set.
  def solve
    # Iterate through each polygonal type to be the type of the first number in the chain
    (0..5).each do |start_type_idx|
      # Iterate through each number of that starting type
      @all_numbers_of_type[start_type_idx].each do |start_num|
        # Start DFS with the first number and its type
        if dfs_find_cycle([start_num], (1 << start_type_idx))
          # Solution is found and stored in @solution_chain
          return @solution_chain.sum
        end
      end
    end
    nil # Should not happen based on problem statement
  end
end

# --- How to run the solver ---
# solver = CyclicalFigurateNumbersSolver.new
# result_sum = solver.solve
#
# Expected output for the sum: 28684
# The set of numbers is [8128, 2882, 8281, 8192, 9216, 1681] (order may vary based on start)
# P3,127=8128
# P5,44=2882
# P4,91=8281
# P8,32=8192  (n(3n-2) = 32 * (96-2) = 32 * 94 = 2908 + 94 = 3008) - Wait, example from problem had P8.
# Let's recheck P8,32: 32 * (3*32 - 2) = 32 * (96 - 2) = 32 * 94 = 3008. This is not in the example set.
# The problem's example set is 3 numbers: 8128 (P3), 2882 (P5), 8281 (P4).
# The actual solution set for 6 numbers is what we are looking for.
# One known solution set is: [1281 (P8), 8128 (P3), 2882 (P5), 8256 (P7), 5625 (P4), 2512 (P6)]
# Sum = 1281+8128+2882+8256+5625+2512 = 28684

puts CyclicalFigurateNumbersSolver.new.solve
