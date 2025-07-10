# Solution for Project Euler Problem 189

L_ROWS = 8
COLORS = [0, 1, 2].freeze

# Generates valid U-row colorings (adjacent U-triangles in a row have different colors)
def generate_valid_u_row_colorings(length, current_coloring = [], &block)
  if current_coloring.length == length
    yield current_coloring.dup
    return
  end

  COLORS.each do |color|
    if current_coloring.empty? || current_coloring.last != color
      current_coloring.push(color)
      generate_valid_u_row_colorings(length, current_coloring, &block)
      current_coloring.pop
    end
  end
end

# Calculates the number of ways to color the D-triangles in row D(h-1, .)
# d_idx is 1-based for D(h-1, d_idx)
# h_of_current_U_row is h, so D-row is h-1. Number of D-triangles is h-1.
# prev_U_colors_array is for U(h-1, .), length h-1
# current_U_colors_array is for U(h, .), length h
# memo_factor is specific to this call context (prev_U, current_U)
def calc_D_factor_recursive(d_idx, prev_d_color, h_of_current_U_row, prev_U_colors_array, current_U_colors_array, memo_factor)
  num_D_triangles_in_row = h_of_current_U_row - 1

  if d_idx == num_D_triangles_in_row + 1
    return 1 # Successfully colored all D-triangles in this segment
  end

  state = [d_idx, prev_d_color]
  return memo_factor[state] if memo_factor.key?(state)

  count = 0

  # Neighbors of D(h-1, d_idx) based on U-triangles
  # Using 0-based indexing for arrays from 1-based d_idx
  # Color of U(h-1, d_idx)
  u_neighbor1_color = prev_U_colors_array[d_idx - 1]
  # Color of U(h-1, d_idx+1)
  u_neighbor2_color = prev_U_colors_array[d_idx]
  # Color of U(h, d_idx+1)
  u_neighbor3_color = current_U_colors_array[d_idx]


  COLORS.each do |d_color|
    next if d_color == u_neighbor1_color || \
            d_color == u_neighbor2_color || \
            d_color == u_neighbor3_color

    # Adjacency to D(h-1, d_idx-1) as per prompt's recursive structure
    # This implies D(h-1, d_idx-1) and D(h-1, d_idx) cannot have the same color.
    next if d_idx > 1 && d_color == prev_d_color

    count += calc_D_factor_recursive(d_idx + 1, d_color, h_of_current_U_row, prev_U_colors_array, current_U_colors_array, memo_factor)
  end

  memo_factor[state] = count
  count
end

def solve
  dp = Array.new(L_ROWS + 1) { Hash.new(0) }

  # Base case for h=1 (first row of U-triangles)
  # U(1,1) can be any of 3 colors.
  generate_valid_u_row_colorings(1) do |u1_colors| # This will yield [0], [1], [2]
    dp[1][u1_colors] = 1
  end

  # DP for h from 2 to L_ROWS
  (2..L_ROWS).each do |h|
    # Iterate over valid colorings of U(h-1,.)
    dp[h-1].each do |prev_U_colors_key, ways_for_prev_U|
      next if ways_for_prev_U == 0 # Should not happen with Hash.new(0) but good practice

      # Iterate over valid colorings for current U-row U(h,.)
      generate_valid_u_row_colorings(h) do |current_U_colors_tuple|
        memo_factor = {} # Fresh cache for this D-row calculation

        factor = calc_D_factor_recursive(
          1,                            # d_idx: start with the first D-triangle D(h-1,1)
          -1,                           # prev_d_color: sentinel for no D-triangle to the left
          h,                            # h_of_current_U_row
          prev_U_colors_key,            # colors of U(h-1,.)
          current_U_colors_tuple,       # colors of U(h,.)
          memo_factor
        )

        if factor > 0
          # Freeze the tuple to ensure it's a valid hash key if it's not already immutable
          # Ruby arrays are mutable; using them as hash keys can be risky if modified.
          # Here, current_U_colors_tuple comes from generate_valid_u_row_colorings which .dup s it.
          # So it should be fine. For safety, one might .freeze it.
          dp[h][current_U_colors_tuple.freeze] += ways_for_prev_U * factor
        end
      end
    end
    # Optional: print progress
    # puts "DP for h=#{h} completed. Number of states: #{dp[h].size}"
  end

  total_ways = dp[L_ROWS].values.sum
  puts total_ways
end

solve if __FILE__ == $PROGRAM_NAME
