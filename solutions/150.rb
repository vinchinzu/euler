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
