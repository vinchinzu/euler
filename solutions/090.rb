#  <p>Each of the six faces on a cube has a different digit ($0$ to $9$) written on
#  it; the same is done to a second cube. By placing the two cubes side-by-side in
#  different positions we can form a variety of $2$-digit numbers.</p>
# 
# <p>For example, the square number $64$ could be formed:</p>
# 
# <div class="center">
# <img src="resources/images/0090.png?1678992052" class="dark_img" alt=""><br></di
# v>
# 
# <p>In fact, by carefully choosing the digits on both cubes it is possible to dis
# play all of the square numbers below one-hundred: $01$, $04$, $09$, $16$, $25$,
# $36$, $49$, $64$, and $81$.</p>
# 
# <p>For example, one way this can be achieved is by placing $\{0, 5, 6, 7, 8, 9\}
# $ on one cube and $\{1, 2, 3, 4, 8, 9\}$ on the other cube.</p>
# 
# <p>However, for this problem we shall allow the $6$ or $9$ to be turned upside-d
# own so that an arrangement like $\{0, 5, 6, 7, 8, 9\}$ and $\{1, 2, 3, 4, 6, 7\}
# $ allows for all nine square numbers to be displayed; otherwise it would be impo
# ssible to obtain $09$.</p>
# 
# <p>In determining a distinct arrangement we are interested in the digits on each
#  cube, not the order.</p>
# 
# <ul style="list-style-type:none;"><li>$\{1, 2, 3, 4, 5, 6\}$ is equivalent to $\
# {3, 6, 4, 1, 2, 5\}$</li>
# <li>$\{1, 2, 3, 4, 5, 6\}$ is distinct from $\{1, 2, 3, 4, 5, 9\}$</li></ul>
# 
# <p>But because we are allowing $6$ and $9$ to be reversed, the two distinct sets
#  in the last example both represent the extended set $\{1, 2, 3, 4, 5, 6, 9\}$ f
# or the purpose of forming $2$-digit numbers.</p>
# 
# <p>How many distinct arrangements of the two cubes allow for all of the square n
# umbers to be displayed?</p>

# Solution for Project Euler Problem 90
require 'set' # Not strictly necessary, but good for semantic clarity with sets of digits

# Step 2: Generate all possible distinct sets of 6 digits for a single cube.
# The digits available are 0 through 9.
all_digits = (0..9).to_a
# Ruby's `combination` method generates sorted arrays, which is ideal for representing
# distinct sets of digits on a cube. C(10,6) = 210 combinations.
all_cube_configurations = all_digits.combination(6).to_a

# Step 3: Define a helper function `can_display_digit`
def can_display_digit(cube_digits_array, digit_needed)
  # cube_digits_array is a sorted array of 6 digits on a cube's faces.
  # digit_needed is an integer (0-9).
  if digit_needed == 6 || digit_needed == 9
    # Can display a 6 or 9 if the cube physically has a 6 or a 9.
    return cube_digits_array.include?(6) || cube_digits_array.include?(9)
  else
    return cube_digits_array.include?(digit_needed)
  end
end

# Step 4: Define the target two-digit numbers (squares)
# These are 01, 04, 09, 16, 25, 36, 49, 64, 81.
target_squares = [
  [0, 1], [0, 4], [0, 9], # Note: 09 requires a 9
  [1, 6],                 # Note: 16 requires a 6
  [2, 5],
  [3, 6],                 # Note: 36 requires a 6
  [4, 9],                 # Note: 49 requires a 9
  [6, 4],                 # Note: 64 requires a 6 (or 9 as 6) and a 4
  [8, 1]
].freeze # Freeze to make it an immutable constant

# Step 5: Initialize valid_pair_count
valid_pair_count = 0

# Step 6: Iterate through all distinct pairs of cube configurations (CubeA, CubeB)
all_cube_configurations.each_with_index do |cube_a_digits, i|
  # Loop for CubeB, starting from i to ensure each pair {A,B} is considered once.
  # This correctly handles pairs where A=B and where A!=B without double counting.
  (i...all_cube_configurations.length).each do |j|
    cube_b_digits = all_cube_configurations[j]

    # Step 7a: Assume it's a valid pair
    current_pair_is_valid = true

    # Step 7b: For each target square [d1, d2]
    target_squares.each do |d1, d2|
      # Step 7b.i: Check if this square can be formed
      can_form_square = (can_display_digit(cube_a_digits, d1) && can_display_digit(cube_b_digits, d2)) || \
                        (can_display_digit(cube_a_digits, d2) && can_display_digit(cube_b_digits, d1))
      
      # Step 7b.ii: If not, this pair of cubes is not valid
      unless can_form_square
        current_pair_is_valid = false
        break # Break from the loop over target_squares
      end
    end

    # Step 7c: If, after checking all target squares, current_pair_is_valid is still true
    if current_pair_is_valid
      valid_pair_count += 1
    end
  end
end

# Step 8: Print valid_pair_count
puts "Number of distinct arrangements: #{valid_pair_count}"
