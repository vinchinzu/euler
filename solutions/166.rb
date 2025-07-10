# Project Euler Problem 166: Criss Cross

class CrissCrossCounter
  def initialize
    @grid = Array.new(4) { Array.new(4) }
    @count = 0
    @target_sum = nil # Will be set when the first row is filled
  end

  def solve
    fill_cell_k(0)
    @count
  end

  private

  def fill_cell_k(k)
    if k == 16 # All 16 cells are filled
      # All row sums, column sums, and the main diagonal sum have been
      # implicitly or explicitly checked and match @target_sum due to the
      # logic in get_possible_digits and the first row sum setting @target_sum.
      # The only sum that needs explicit final verification is the anti-diagonal.
      current_anti_diag_sum = 0
      (0..3).each { |i| current_anti_diag_sum += @grid[i][3 - i] }

      if current_anti_diag_sum == @target_sum
        @count += 1
      end
      return
    end

    r = k / 4
    c = k % 4

    possible_digits = get_possible_digits(r, c)

    possible_digits.each do |digit|
      @grid[r][c] = digit

      if r == 0 && c == 3 # First row just completed
        current_row0_sum = @grid[0][0] + @grid[0][1] + @grid[0][2] + @grid[0][3]

        # Optimization: if sum is too high, no other line can match it with digits 0-9
        if current_row0_sum > 36
          next # Try next digit for grid[0][3]
        end

        original_target_sum = @target_sum # Save current @target_sum (should be nil)
        @target_sum = current_row0_sum    # Set new @target_sum

        fill_cell_k(k + 1)                # Recurse for next cell

        @target_sum = original_target_sum # Backtrack @target_sum
      else
        # Standard recursion for other cells
        fill_cell_k(k + 1)
      end
    end
  end

  # Helper to determine possible digits for grid[r][c] based on constraints
  def get_possible_digits(r, c)
    # If @target_sum is not yet set (i.e., processing cells before grid[0][3] is set),
    # or if this cell (r,c) is grid[0][3] itself (which determines @target_sum),
    # then the digit is not constrained by @target_sum yet.
    if @target_sum.nil? || (r == 0 && c == 3)
      return (0..9).to_a
    end

    # Cell grid[3][3] is the most constrained if @target_sum is known.
    if r == 3 && c == 3
      # Determined by Row 3 sum
      val_row3 = @target_sum - (@grid[3][0] + @grid[3][1] + @grid[3][2])
      return [] unless (val_row3 >= 0 && val_row3 <= 9)

      # Must also match Column 3 sum
      val_col3 = @target_sum - (@grid[0][3] + @grid[1][3] + @grid[2][3])
      return [] unless val_col3 == val_row3

      # Must also match Main Diagonal sum
      val_diag1 = @target_sum - (@grid[0][0] + @grid[1][1] + @grid[2][2])
      return [] unless val_diag1 == val_row3

      return [val_row3] # Single determined digit
    end

    # Last cell in a row (r > 0 because r=0 case means @target_sum was nil or just being set)
    if c == 3 # && r > 0 (implicit from @target_sum != nil)
      req_digit = @target_sum - (@grid[r][0] + @grid[r][1] + @grid[r][2])
      return (req_digit >= 0 && req_digit <= 9) ? [req_digit] : []
    end

    # Last cell in a column (r == 3). This applies to g[3][0], g[3][1], g[3][2].
    # (g[3][3] is handled by the more specific case above).
    if r == 3
      req_digit = @target_sum - (@grid[0][c] + @grid[1][c] + @grid[2][c])
      return (req_digit >= 0 && req_digit <= 9) ? [req_digit] : []
    end

    # Otherwise, not a cell that completes a line for sum-checking at this point.
    # (e.g. grid[1][1], grid[2][0], etc.)
    return (0..9).to_a
  end

end

if __FILE__ == $PROGRAM_NAME
  counter = CrissCrossCounter.new
  result = counter.solve
  puts result
end
```
