#  <p>Su Doku (Japanese meaning <i>number place</i>) is the name given to a popular
#  puzzle concept. Its origin is unclear, but credit must be attributed to Leonhar
# d Euler who invented a similar, and much more difficult, puzzle idea called Lati
# n Squares. The objective of Su Doku puzzles, however, is to replace the blanks (
# or zeros) in a 9 by 9 grid in such that each row, column, and 3 by 3 box contain
# s each of the digits 1 to 9. Below is an example of a typical starting puzzle gr
# id and its solution grid.</p>
# <div class="center">
# <img src="project/images/p096_1.png" alt="p096_1.png" />     <img src="project/i
# mages/p096_2.png" alt="p096_2.png" /></div>
# <p>A well constructed Su Doku puzzle has a unique solution and can be solved by
# logic, although it may be necessary to employ "guess and test" methods in order
# to eliminate options (there is much contested opinion over this). The complexity
#  of the search determines the difficulty of the puzzle; the example above is con
# sidered <i>easy</i> because it can be solved by straight forward direct deductio
# n.</p>
# <p>The 6K text file, <a href="project/resources/p096_sudoku.txt">sudoku.txt</a>
# (right click and 'Save Link/Target As...'), contains fifty different Su Doku puz
# zles ranging in difficulty, but all with unique solutions (the first puzzle in t
# he file is the example above).</p>
# <p>By solving all fifty puzzles find the sum of the 3-digit numbers found in the
#  top left corner of each solution grid; for example, 483 is the 3-digit number f
# ound in the top left corner of the solution grid above.</p>

# Solves Project Euler Problem 96

class SudokuSolver
  def initialize(grid_str)
    @grid = parse_grid(grid_str)
  end

  def parse_grid(grid_str)
    grid_str.strip.split("\n").map { |row| row.chars.map(&:to_i) }
  end

  def solve
    solve_recursive(@grid)
  end

  def get_top_left_three_digit_number
    return 0 unless solved? && @grid.length == 9 && @grid[0].length == 9
    @grid[0][0] * 100 + @grid[0][1] * 10 + @grid[0][2]
  end

  def solved?
    # A simple check, assumes solve was called and successful if no zeros
    @grid.flatten.none?(&:zero?)
  end

  private

  def solve_recursive(grid)
    find = find_empty(grid)
    return true unless find # No empty spots, solved

    row, col = find

    (1..9).each do |num|
      if is_safe?(grid, row, col, num)
        grid[row][col] = num

        return true if solve_recursive(grid) # Recur

        grid[row][col] = 0 # Backtrack
      end
    end

    false # Trigger backtrack
  end

  def find_empty(grid)
    (0..8).each do |r|
      (0..8).each do |c|
        return [r, c] if grid[r][c] == 0
      end
    end
    nil
  end

  def is_safe?(grid, row, col, num)
    !used_in_row?(grid, row, num) &&
      !used_in_col?(grid, col, num) &&
      !used_in_box?(grid, row - row % 3, col - col % 3, num)
  end

  def used_in_row?(grid, row, num)
    (0..8).any? { |col| grid[row][col] == num }
  end

  def used_in_col?(grid, col, num)
    (0..8).any? { |row| grid[row][col] == num }
  end

  def used_in_box?(grid, box_start_row, box_start_col, num)
    (0..2).any? do |r_offset|
      (0..2).any? do |c_offset|
        grid[box_start_row + r_offset][box_start_col + c_offset] == num
      end
    end
  end
end

def load_puzzles(filename)
  puzzles_str = File.read(filename).strip.split("Grid ")
  puzzles_str.shift # Remove the empty string from the first split
  puzzles_str.map do |puzzle_block|
    # Each block has a header like "01\\n" followed by the grid
    puzzle_block.lines[1..-1].join.strip
  end
end

# Main execution
if __FILE__ == $PROGRAM_NAME
  puzzles = load_puzzles("p096_sudoku.txt")
  total_sum_of_top_left_numbers = 0

  puzzles.each_with_index do |puzzle_str, index|
    # puts "Solving Puzzle ##{index + 1}"
    solver = SudokuSolver.new(puzzle_str)
    if solver.solve
      # puts "Solved."
      top_left_num = solver.get_top_left_three_digit_number
      # puts "Top left 3-digit number: #{top_left_num}"
      total_sum_of_top_left_numbers += top_left_num
    else
      puts "Could not solve Puzzle ##{index + 1}"
    end
  end

  puts "Sum of the 3-digit numbers: #{total_sum_of_top_left_numbers}"
  # Expected: 24702
end
