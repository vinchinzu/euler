class Euler115
  def initialize(m_min_block_length, limit)
    @m = m_min_block_length
    @limit = limit
    # @dp stores F(m, i) at index i.
    # Initialize with dp[0] = 1 for the empty row.
    @dp = [1] 
  end

  def solve
    n = 0 # Current length of the row
    loop do
      n += 1
      current_dp_n_val = 0

      # Option 1: The n-th unit is a black square.
      # Requires a valid tiling of length n-1.
      # @dp[n-1] is the last element pushed in the previous iteration (if n > 0).
      current_dp_n_val += @dp[n-1]
      
      # Option 2: The row ends with a red block of length 'block_len'.
      # 'block_len' must be at least @m.
      # This loop runs only if n >= @m.
      (@m..n).each do |block_len|
        if block_len == n
          # Case 2a: The red block fills the entire row of length n.
          # This adds 1 way. (e.g., RRRRR for n=5, m=5)
          current_dp_n_val += 1
        else
          # Case 2b: The red block (length block_len) is preceded by a black square.
          # The red block is from (n - block_len + 1) to n.
          # The black square is at position (n - block_len).
          # The subproblem is for the length before this black square: (n - block_len - 1).
          # This index must be non-negative.
          sub_problem_idx = n - block_len - 1
          # Since block_len >= @m, and block_len < n here.
          # Smallest block_len is @m. Smallest sub_problem_idx is n - @m - 1.
          # This index is guaranteed to be < n.
          # dp values for these indices are already in @dp array.
          current_dp_n_val += @dp[sub_problem_idx]
        end
      end
      
      @dp.push(current_dp_n_val) # Store F(m, n)

      # Check if the fill-count function first exceeds one million.
      if current_dp_n_val > @limit
        puts n # Print the least value of n
        break
      end
    end
  end
end

if __FILE__ == $PROGRAM_NAME
  MIN_M_VALUE = 50
  TARGET_LIMIT = 1_000_000

  solver = Euler115.new(MIN_M_VALUE, TARGET_LIMIT)
  solver.solve
end
