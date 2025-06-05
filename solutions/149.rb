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
