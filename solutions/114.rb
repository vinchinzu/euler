class Euler114
  def initialize(length)
    @length = length
    # dp[i] will store the number of ways to fill a row of length i
    # Initialize with 0, dp[0] will be set to 1 as the base case.
    @dp = Array.new(length + 1, 0)
  end

  def solve
    # Base case: There is one way to tile a row of length 0 (the empty tiling).
    @dp[0] = 1

    # Iterate from length 1 up to the target length
    (1..@length).each do |i|
      # Option 1: The i-th unit is a grey square.
      # This means the first i-1 units must form a valid tiling.
      # This is always possible if i >= 1.
      @dp[i] += @dp[i-1]

      # Option 2: The row ends with a red block of length 'block_len'.
      # Minimum block_len is 3.
      # block_len can range from 3 up to i.
      (3..i).each do |block_len|
        if block_len == i
          # Case 2a: The red block fills the entire row of length i.
          # This adds 1 way.
          @dp[i] += 1
        else
          # Case 2b: The red block (length block_len) is preceded by a grey square.
          # The red block is from (i - block_len + 1) to i.
          # The grey square is at position (i - block_len).
          # The subproblem is for the length before this grey square, which is (i - block_len - 1).
          # This requires i - block_len - 1 >= 0.
          if i - block_len - 1 >= 0
            @dp[i] += @dp[i - block_len - 1]
          end
        end
      end
    end

    # The final answer is stored in dp[target_length]
    puts @dp[@length]
  end
end

if __FILE__ == $PROGRAM_NAME
  # Problem 114 asks for a row measuring fifty units in length.
  target_length = 50
  solver = Euler114.new(target_length)
  solver.solve
end
