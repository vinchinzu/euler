class Euler117
  def initialize(length)
    @length = length
    # dp[i] will store the number of ways to tile a row of length i.
    # Initialize with 0s. dp[0] will be the base case.
    @dp = Array.new(length + 1, 0)
  end

  def solve
    # Base case: There is one way to tile a row of length 0 (the empty row).
    @dp[0] = 1

    # Iterate from length 1 up to the target length
    (1..@length).each do |i|
      # Initialize current dp_val to 0 for summation
      current_dp_i_val = 0

      # Option 1: The last tile is a grey square (length 1).
      # Builds upon a valid tiling of length i-1.
      if i >= 1
        current_dp_i_val += @dp[i-1]
      end
      
      # Option 2: The last tile is a red oblong (length 2).
      # Builds upon a valid tiling of length i-2.
      if i >= 2
        current_dp_i_val += @dp[i-2]
      end
      
      # Option 3: The last tile is a green oblong (length 3).
      # Builds upon a valid tiling of length i-3.
      if i >= 3
        current_dp_i_val += @dp[i-3]
      end
      
      # Option 4: The last tile is a blue oblong (length 4).
      # Builds upon a valid tiling of length i-4.
      if i >= 4
        current_dp_i_val += @dp[i-4]
      end
      
      @dp[i] = current_dp_i_val
    end
    
    # The final answer is stored in dp[target_length]
    puts @dp[@length]
  end
end

if __FILE__ == $PROGRAM_NAME
  ROW_LENGTH = 50
  solver = Euler117.new(ROW_LENGTH)
  solver.solve
end
