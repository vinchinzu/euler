# Solution for Project Euler Problem 190

def solve
  total_sum_of_floors = 0

  (2..15).each do |m|
    current_P_m = 1.0

    (1..m).each do |j|
      # Calculate x_j = (2*j) / (m+1) using floating point numbers
      x_j = (2.0 * j) / (m + 1.0)

      # Calculate term x_j^j
      term = x_j ** j

      # Multiply current_P_m by this term
      current_P_m *= term
    end

    # Take the floor of current_P_m
    floor_P_m = current_P_m.floor

    # Add this to the total sum
    total_sum_of_floors += floor_P_m

    # For verification against the example P_10
    # if m == 10
    #   puts "P_#{m} = #{current_P_m}, floor(P_#{m}) = #{floor_P_m}" # Expected: 4112
    # end
  end

  puts total_sum_of_floors
end

solve if __FILE__ == $PROGRAM_NAME
