class Euler116
  # Calculates the number of ways to tile a row of 'row_length'
  # using grey tiles (length 1) and one type of colored tile (length 'tile_k_length').
  # At least one colored tile must be used.
  def count_ways_for_one_color(row_length, tile_k_length)
    # dp[i] stores the number of ways to tile a row of length i,
    # including the all-grey case.
    dp = Array.new(row_length + 1, 0)
    
    # Base case: There is one way to tile a row of length 0 (empty row).
    dp[0] = 1

    (1..row_length).each do |i|
      # Option 1: The i-th tile is a grey square.
      # This builds upon a valid tiling of length i-1.
      dp[i] = dp[i-1]
      
      # Option 2: The row ends with a colored tile of length 'tile_k_length'.
      # This builds upon a valid tiling of length i - tile_k_length.
      # This is only possible if i is greater than or equal to tile_k_length.
      if i >= tile_k_length
        dp[i] += dp[i - tile_k_length]
      end
    end
    
    # dp[row_length] now contains all possible tilings, including the one
    # consisting purely of grey tiles. Since at least one colored tile
    # must be used, we subtract 1 for this all-grey case.
    return dp[row_length] - 1
  end

  def solve(n)
    # Calculate ways for red tiles (length 2)
    ways_red = count_ways_for_one_color(n, 2)
    
    # Calculate ways for green tiles (length 3)
    ways_green = count_ways_for_one_color(n, 3)
    
    # Calculate ways for blue tiles (length 4)
    ways_blue = count_ways_for_one_color(n, 4)
    
    # Total number of ways is the sum of ways for each color,
    # as colors cannot be mixed.
    total_ways = ways_red + ways_green + ways_blue
    
    puts total_ways
  end
end

if __FILE__ == $PROGRAM_NAME
  ROW_LENGTH = 50
  solver = Euler116.new
  solver.solve(ROW_LENGTH)
end
