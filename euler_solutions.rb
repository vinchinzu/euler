#!/usr/bin/env ruby

# This script will contain solutions to Project Euler problems.

# Helper to count even and odd numbers in a given range (inclusive)
def count_even_odd_in_range(range_start, range_end)
  even_count = 0
  odd_count = 0
  (range_start..range_end).each do |val|
    if val.even?
      even_count += 1
    else
      odd_count += 1
    end
  end
  [even_count, odd_count]
end

# Calculates the number of "diagonal" rectangles in a single m x n grid
def count_diagonal_rectangles_single_grid(m, n)
  # u = x+y ranges from 0 to m+n. Number of points is m+n+1.
  # x and y here are 0-indexed vertex coordinates.
  # m and n are cell counts (width and height).
  u_range_size = m + n + 1

  # Number of even/odd u-coordinates
  # If u_range_size is 5 (e.g., 0,1,2,3,4), u_even is 3 (0,2,4), u_odd is 2 (1,3)
  # (5+1)/2 = 3 for even; 5/2 = 2 for odd.
  u_even_count = (u_range_size + 1) / 2
  u_odd_count = u_range_size / 2

  # v = x-y ranges from -n to m. Number of points is m-(-n)+1 = m+n+1.
  v_even_count, v_odd_count = count_even_odd_in_range(-n, m)

  # Number of ways to choose 2 distinct coordinates C(N,2) = N*(N-1)/2
  rects_from_even_uv = 0
  if u_even_count >= 2 && v_even_count >= 2
    rects_from_even_uv = (u_even_count * (u_even_count - 1) / 2) * \
                         (v_even_count * (v_even_count - 1) / 2)
  end

  rects_from_odd_uv = 0
  if u_odd_count >= 2 && v_odd_count >= 2
    rects_from_odd_uv = (u_odd_count * (u_odd_count - 1) / 2) * \
                        (v_odd_count * (v_odd_count - 1) / 2)
  end

  rects_from_even_uv + rects_from_odd_uv
end

def problem_147(max_m, max_n)
  total_rectangles = 0

  # Iterate through all possible grid dimensions (width m, height n)
  (1..max_m).each do |m|
    (1..max_n).each do |n|
      # Count grid-aligned rectangles
      grid_aligned_rects = (m * (m + 1) / 2) * (n * (n + 1) / 2)
      total_rectangles += grid_aligned_rects

      # Count diagonal rectangles
      diagonal_rects = count_diagonal_rectangles_single_grid(m, n)
      total_rectangles += diagonal_rects
    end
  end

  total_rectangles
end

# Example calls (optional, for testing)
# puts "S(1,1) diag: #{count_diagonal_rectangles_single_grid(1,1)}" # Expected based on analysis: 0
# puts "S(2,1) diag: #{count_diagonal_rectangles_single_grid(2,1)}" # Expected based on analysis: 2
# puts "S(2,2) diag: #{count_diagonal_rectangles_single_grid(2,2)}" # Expected based on analysis: 10

# puts "Problem 147 (1x1): #{problem_147(1,1)}" # Expected C(1)C(1)+S(1,1) = 1+0=1
# puts "Problem 147 (2,1): #{problem_147(2,1)}" # Expected C(1)C(1)+S(1,1) + C(2)C(1)+S(2,1) = 1+0 + 3+2 = 6
                                                 # This interpretation of sum is: sum over all subgrids.
                                                 # The problem_147 function already sums up.
                                                 # So problem_147(1,1) is just for 1x1 grid. Total 1.
                                                 # problem_147(2,1) is for 1x1, 1x2 (no), 2x1.
                                                 # It means sum over m from 1 to max_m, n from 1 to max_n.
                                                 # So problem_147(2,1) means:
                                                 # grid 1x1: C(1)C(1)+S(1,1) = 1+0=1
                                                 # grid 2x1: C(2)C(1)+S(2,1) = 3+2=5
                                                 # Total for problem_147(2,1) = 1+5 = 6

# puts "Problem 147 (2,2): #{problem_147(2,2)}"
# Expected for problem_147(2,2):
# 1x1: 1
# 1x2: C(1)C(2)+S(1,2) = 1*3 + S(1,2)
#      S(1,2): u_len=1+2+1=4. u_e=2,u_o=2. v_coords(-2..1)=[-2,-1,0,1]. v_e=2,v_o=2.
#      S(1,2) = C(2)C(2)+C(2)C(2) = 1*1+1*1=2.
#      1x2 total = 3+2=5
# 2x1: C(2)C(1)+S(2,1) = 3+2=5
# 2x2: C(2)C(2)+S(2,2) = 9+10=19
# Total = 1+5+5+19 = 30

# Problem 148: Exploring Pascal's triangle
# Counts the number of entries not divisible by p in the first n_rows of Pascal's triangle.
# Entries C(n,k) are not divisible by p if digits of k in base p are <= digits of n in base p.
# Number of such k for a given n (digits n_i) is product(n_i + 1).
# This function sums product(n_i + 1) for n from 0 to n_rows - 1.
def problem_148
  n_rows = 10**9
  p = 7

  # Convert n_rows to base p digits
  # For n_rows = 10^9, p=7, this is [3,3,5,3,1,6,0,0,6,1,6]
  n_digits_base_p = n_rows.to_s(p).chars.map(&:to_i)
  num_digits = n_digits_base_p.length

  # sum_factor is sum_{j=0}^{p-1} (j+1) = p*(p+1)/2
  # For p=7, this is 7*8/2 = 28
  sum_factor = p * (p + 1) / 2

  total_count = 0
  # product_of_prefix_digits_plus_one represents (L_k+1)*(L_{k-1}+1)*... for the current prefix of N
  # where N is n_rows, and L_i are its digits in base p.
  product_of_prefix_digits_plus_one = 1

  n_digits_base_p.each_with_index do |current_digit_val_of_n_rows, index|
    remaining_len = num_digits - 1 - index

    # Iterate for current digit position `d` from 0 up to `current_digit_val_of_n_rows - 1`.
    # These are cases where the number formed is guaranteed to be less than n_rows.
    # For each such `d`:
    # The contribution to total_count is:
    # (product from higher-order digits of n_rows) * (d+1) * (sum_factor ^ remaining_len)
    # where sum_factor ^ remaining_len accounts for all possible combinations of lower-order digits.
    (0...current_digit_val_of_n_rows).each do |d|
      total_count += product_of_prefix_digits_plus_one * (d + 1) * (sum_factor ** remaining_len)
    end

    # Update product_of_prefix_digits_plus_one to include the term from the current digit of n_rows.
    # This will be used for the next iteration (next lower-order digit) or if this is the most significant part.
    product_of_prefix_digits_plus_one *= (current_digit_val_of_n_rows + 1)
  end

  # The loop iterates through all prefixes of N (n_rows).
  # `total_count` accumulates the sum for all numbers `n` from `0` to `n_rows - 1`.
  # The `product_of_prefix_digits_plus_one` at the end of the loop is for `n = n_rows` itself.
  # This term should NOT be added if we are counting for rows 0 to n_rows-1.
  # "first 10^9 rows" typically means rows 0, 1, ..., 10^9 - 1.

  total_count
end

# Problem 149: Maximum-sum subsequence
def problem_149
  size = 2000 # Grid size N x N
  total_numbers = size * size
  mod_val = 1_000_000

  s = Array.new(total_numbers + 1) # 1-indexed for k

  # Generate s_k values
  (1..55).each do |k|
    # Expression is always positive for k in [1,55], so standard % is fine.
    val = (100003 - 200003 * k + 300007 * (k**3))
    s[k] = (val % mod_val) - 500000
  end

  (56..total_numbers).each do |k|
    # s[k-24] + s[k-55] is min -1_000_000. Adding mod_val makes it non-negative.
    # So standard % is fine here too.
    val = (s[k-24] + s[k-55] + mod_val)
    s[k] = (val % mod_val) - 500000
  end

  grid = Array.new(size) { Array.new(size) }
  (0...size).each do |r|
    (0...size).each do |c|
      k_idx = size * r + c + 1 # Convert (r,c) (0-indexed) to k (1-indexed)
      grid[r][c] = s[k_idx]
    end
  end
  s = nil # Free memory for s array

  max_overall_sum = -Float::INFINITY

  kadane_non_empty = lambda do |arr|
    # This lambda should not be called with an empty arr by the logic below.
    # If it were, this check would be important.
    # return -Float::INFINITY if arr.empty?

    current_max_for_subarray_ending_here = arr[0]
    global_max_for_array = arr[0]
    (1...arr.length).each do |i|
      current_max_for_subarray_ending_here = [arr[i], current_max_for_subarray_ending_here + arr[i]].max
      global_max_for_array = [global_max_for_array, current_max_for_subarray_ending_here].max
    end
    global_max_for_array
  end

  # Rows
  (0...size).each do |r|
    row_arr = grid[r]
    max_overall_sum = [max_overall_sum, kadane_non_empty.call(row_arr)].max
  end

  # Columns
  (0...size).each do |c|
    col_arr = (0...size).map { |r| grid[r][c] }
    max_overall_sum = [max_overall_sum, kadane_non_empty.call(col_arr)].max
  end

  # Diagonals (top-left to bottom-right: \)
  (0...size).each do |c_start| # Starting from top row grid[0][c_start]
    diag_arr = []
    r, c = 0, c_start
    while r < size && c < size
      diag_arr << grid[r][c]
      r += 1; c += 1
    end
    max_overall_sum = [max_overall_sum, kadane_non_empty.call(diag_arr)].max
  end
  (1...size).each do |r_start| # Starting from left col grid[r_start][0] (excl. grid[0][0])
    diag_arr = []
    r, c = r_start, 0
    while r < size && c < size
      diag_arr << grid[r][c]
      r += 1; c += 1
    end
    max_overall_sum = [max_overall_sum, kadane_non_empty.call(diag_arr)].max
  end

  # Anti-diagonals (top-right to bottom-left: /)
  (0...size).each do |c_start| # Starting from top row grid[0][c_start]
    anti_diag_arr = []
    r, c = 0, c_start
    while r < size && c >= 0
      anti_diag_arr << grid[r][c]
      r += 1; c -= 1
    end
    max_overall_sum = [max_overall_sum, kadane_non_empty.call(anti_diag_arr)].max
  end
  (1...size).each do |r_start| # Starting from right col grid[r_start][size-1] (excl. grid[0][size-1])
    anti_diag_arr = []
    r, c = r_start, size - 1
    while r < size && c >= 0
      anti_diag_arr << grid[r][c]
      r += 1; c -= 1
    end
    max_overall_sum = [max_overall_sum, kadane_non_empty.call(anti_diag_arr)].max
  end

  max_overall_sum
end

# Problem 150: Searching for the smallest sum in a sub-triangle
def problem_150
  rows = 1000

  num_elements = rows * (rows + 1) / 2
  modulus = 1 << 20    # 2^20 = 1048576
  subtract_val = 1 << 19 # 2^19 = 524288

  s_values = Array.new(num_elements) # 0-indexed for s_values array
  t = 0 # t_0 = 0

  # Generate s_k values (0-indexed: s_values[k] is (k+1)th number in problem statement's 1-indexed s_k)
  (0...num_elements).each do |k_idx|
    t = (615949 * t + 797807) % modulus
    s_values[k_idx] = t - subtract_val
  end

  # Populate triangle (0-indexed: triangle[r][c])
  # triangle[r] has r+1 elements
  triangle_grid = Array.new(rows) { |r| Array.new(r + 1) }
  current_s_idx = 0
  (0...rows).each do |r|
    (0..r).each do |c|
      triangle_grid[r][c] = s_values[current_s_idx]
      current_s_idx += 1
    end
  end
  s_values = nil # Free memory

  # Precompute prefix sums for each row of the triangle_grid
  # row_prefix_sums[r][c_idx_in_row] = sum(triangle_grid[r][0..c_idx_in_row])
  row_prefix_sums = Array.new(rows) { |r| Array.new(r + 1) }
  (0...rows).each do |r|
    current_row_total = 0
    (0..r).each do |c_idx_in_row|
      current_row_total += triangle_grid[r][c_idx_in_row]
      row_prefix_sums[r][c_idx_in_row] = current_row_total
    end
  end

  min_overall_sum = Float::INFINITY

  # Iterate over all possible apexes (r_apex, c_apex)
  (0...rows).each do |r_apex|
    (0..r_apex).each do |c_apex|
      current_sum_for_this_apex_sub_triangles = 0
      # Iterate over height of sub-triangle by adding one row segment at a time
      # h_offset = actual_height - 1. Max h_offset is rows - 1 - r_apex.
      (0...(rows - r_apex)).each do |h_offset| # h_offset is 0 for height 1, 1 for height 2, etc.
        main_triangle_row_idx_of_segment = r_apex + h_offset

        col_start_of_segment_in_main_row = c_apex
        col_end_of_segment_in_main_row = c_apex + h_offset

        segment_sum = row_prefix_sums[main_triangle_row_idx_of_segment][col_end_of_segment_in_main_row]
        if col_start_of_segment_in_main_row > 0
          segment_sum -= row_prefix_sums[main_triangle_row_idx_of_segment][col_start_of_segment_in_main_row - 1]
        end

        current_sum_for_this_apex_sub_triangles += segment_sum
        min_overall_sum = [min_overall_sum, current_sum_for_this_apex_sub_triangles].min
      end
    end
  end

  min_overall_sum
end

# Memoization cache for Problem 151
$memo_151 = {}

# Recursive solver for Problem 151
# counts: an array [c1, c2, c3, c4, c5] representing counts of A1 to A5 sheets
def solve_151(counts)
  state_tuple = counts.dup.freeze # Use a frozen copy of the array as a hash key
  return $memo_151[state_tuple] if $memo_151.key?(state_tuple)

  num_sheets = counts.sum
  if num_sheets == 0
    return 0.0 # Base case: envelope empty, no more events.
  end

  # If only one A5 sheet remains, this is a "last batch" scenario.
  # This specific "single sheet" event is not counted towards the expectation.
  if num_sheets == 1 && counts[4] == 1 # counts[4] is c5 (A5 sheets)
    return 0.0
  end

  # Value added by the current state, if it's a countable single sheet event.
  # A "single sheet event" occurs if num_sheets == 1.
  # It's countable if it's not the very first A1 sheet (which is handled by final subtraction)
  # and not the "single A5 sheet remaining" case (which is handled by returning 0.0 above).
  current_event_contribution = 0.0
  if num_sheets == 1
    # This implies it's a single A1, A2, A3, or A4 sheet.
    current_event_contribution = 1.0
  end

  # Expected value from future states
  future_expected_value = 0.0

  if num_sheets == 1
    # This state has a single sheet (A1, A2, A3, or A4). It will be cut.
    # Find which sheet it is (0 for A1, 1 for A2, etc.)
    idx_of_single_sheet = counts.find_index(1)

    new_counts = counts.clone
    new_counts[idx_of_single_sheet] = 0
    new_counts[idx_of_single_sheet + 1] = 2 # Cut into two of the next size

    future_expected_value = solve_151(new_counts)
  else # num_sheets > 1
    # Multiple sheets in the envelope. One is chosen randomly.
    counts.each_with_index do |count_of_ax, idx_ax| # idx_ax: 0 for A1, ..., 4 for A5
      if count_of_ax > 0
        prob_pick_ax = count_of_ax.to_f / num_sheets

        new_counts = counts.clone
        new_counts[idx_ax] -= 1 # Remove the picked sheet

        if idx_ax < 4 # If A1, A2, A3, or A4 was picked (index 0-3)
          new_counts[idx_ax + 1] += 2 # It's cut and two smaller sheets are added
        end
        # If A5 was picked (idx_ax == 4), it's just removed. No new sheets are added from cutting.

        future_expected_value += prob_pick_ax * solve_151(new_counts)
      end
    end
  end

  result = current_event_contribution + future_expected_value
  $memo_151[state_tuple] = result
  return result
end

# Problem 151: A Preference for A5
def problem_151
  initial_counts = [1, 0, 0, 0, 0] # Starts with one A1 sheet

  # The result from solve_151 includes the count for the initial single A1 sheet.
  # The problem asks to exclude this first one.
  # $memo_151 must be cleared for multiple runs if this function were called multiple times.
  # For a single call per script execution, it's fine.
  $memo_151.clear
  raw_expected_value = solve_151(initial_counts)
  final_expected_value = raw_expected_value - 1.0

  final_expected_value # The main block will format it.
end

if __FILE__ == $0
  puts "Euler Problem Solutions:"
  puts "------------------------"

  # Problem 147: Rectangles in Cross-hatched Grids
  max_m_147 = 47
  max_n_147 = 43
  total_count_147 = problem_147(max_m_147, max_n_147)
  puts "Problem 147: Total rectangles in grids up to #{max_m_147}x#{max_n_147}: #{total_count_147}"

  # Problem 148: Exploring Pascal's Triangle
  count_148 = problem_148
  puts "Problem 148: Entries not divisible by 7 in first 10^9 rows: #{count_148}"

  # Problem 149: Maximum-sum Subsequence
  max_sum_149 = problem_149
  puts "Problem 149: Max sum in LFG table (2000x2000): #{max_sum_149}"

  # Problem 150: Sub-triangle Sums
  min_sum_150 = problem_150
  puts "Problem 150: Smallest sub-triangle sum (1000 rows): #{min_sum_150}"

  # Problem 151: A Preference for A5
  expected_value_151 = problem_151
  puts "Problem 151: Expected single sheets (A5 preference): #{'%.6f' % expected_value_151}"
  puts "------------------------"
end
