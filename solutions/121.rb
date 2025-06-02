# Problem 121: Disc Game Prize Fund
#
# Problem Statement:
# A bag contains one red disc and one blue disc. In a game of chance a player
# takes a disc at random and its colour is noted. After each turn the disc is
# returned to the bag, an extra red disc is added, and another disc is taken at
# random.
#
# The player pays £1 to play and wins if they have taken more blue discs than
# red discs at the end of the game.
#
# If the game is played for four turns, the probability of a player winning is
# exactly 11/120, and so the maximum prize fund the banker should allocate for
# winning in this game would be £10 before they would expect to incur a loss.
# Note that any payout will be a whole number of pounds and also includes the
# original £1 paid to play the game, so in the example given the player actually
# wins £9.
#
# Find the maximum prize fund that should be allocated to a single game in which
# fifteen turns are played.
#
# Notes:
# The solution involves calculating the probability of winning (more blue discs than red) over 15 turns.
# This can be done by finding the coefficients representing choices, or by direct combinatorial calculation.
# The script uses an iterative approach to find numerators for the probabilities of picking k blue discs.
# The common denominator for the probability of any specific sequence of choices over N turns is (N+1)!.
# For N=15 turns, the denominator is (15+1)! = 16!.
# The probability of winning P(win) is calculated by summing the numerators of all winning scenarios
# (k blue discs, where k > N_TURNS - k) and dividing by this common denominator.
# The prize fund is floor(1 / P(win)).
# The script implements this logic to arrive at the solution 2269.

# Full Ruby script content from temp_problem_121.rb:

# N_TURNS is the total number of turns in the game.
N_TURNS = 15

# coeffs[k] will store the sum of numerators for scenarios resulting in k blue discs.
# The common denominator for these probabilities will be (N_TURNS + 1)!.
# Initialize with integers, as numerators will be integers.
# coeffs array size is N_TURNS + 1 to store counts for 0 to N_TURNS blue discs.
coeffs = Array.new(N_TURNS + 1, 0)

# Base case: Before any turns (0 turns completed), there is 1 way to have 0 blue discs,
# and its "numerator" part is 1.
coeffs[0] = 1

# Iterate through each turn of the game, from 1 to N_TURNS.
(1..N_TURNS).each do |turn_number|
  # In turn `turn_number`, there are `turn_number` red discs and 1 blue disc.
  # The "value" for picking red contributes `turn_number` to the numerator product.
  # The "value" for picking blue contributes `1` to the numerator product.
  num_red_value_for_turn = turn_number

  # Create a new array to store coefficients for the current turn, based on the previous turn's coeffs.
  new_coeffs = Array.new(N_TURNS + 1, 0)

  # Iterate over the possible number of blue discs accumulated *before* the current turn.
  # `k_blue_before_turn` can range from 0 up to `turn_number - 1`.
  (0..(turn_number - 1)).each do |k_blue_before_turn|
    # If `coeffs[k_blue_before_turn]` is greater than 0, it means this state was reachable.
    if coeffs[k_blue_before_turn] > 0
      # Option 1: Player picks a RED disc in the current `turn_number`.
      # The number of blue discs remains `k_blue_before_turn`.
      # The numerator sum is multiplied by `num_red_value_for_turn`.
      new_coeffs[k_blue_before_turn] += coeffs[k_blue_before_turn] * num_red_value_for_turn

      # Option 2: Player picks a BLUE disc in the current `turn_number`.
      # The number of blue discs increases to `k_blue_before_turn + 1`.
      # The numerator sum is multiplied by 1 (the "value" for picking blue).
      # Ensure we don't exceed the maximum possible blue discs (N_TURNS).
      if k_blue_before_turn + 1 <= N_TURNS
        new_coeffs[k_blue_before_turn + 1] += coeffs[k_blue_before_turn] * 1
      end
    end
  end
  # Update coeffs to be the results from this turn, for the next iteration.
  coeffs = new_coeffs
end

# Calculate the total winning numerator.
# Player wins if number of blue discs > number of red discs.
# Let k be number of blue discs. Number of red discs = N_TURNS - k.
# So, k > N_TURNS - k  =>  2k > N_TURNS  => k > N_TURNS / 2.
# For N_TURNS = 15, k > 7.5. So, player wins if k = 8, 9, ..., 15.
min_blue_to_win = (N_TURNS / 2) + 1 # Integer division gives floor(N_TURNS/2), then +1.

total_winning_numerator = 0
(min_blue_to_win..N_TURNS).each do |k_blue|
  total_winning_numerator += coeffs[k_blue]
end

# Calculate the common denominator for all probabilities: (N_TURNS + 1)!
denominator = (1..(N_TURNS + 1)).reduce(1, :*)

# The maximum prize fund is floor(1 / P_win).
# P_win = total_winning_numerator / denominator.
# So, prize_fund = floor(denominator / total_winning_numerator).
# Ruby's integer division `/` performs floor division when operands are integers.
if total_winning_numerator == 0
  # This case should not happen for this problem as there's a non-zero chance of winning.
  max_prize_fund_str = "Error: No winning scenarios or total_winning_numerator is zero."
else
  max_prize_fund = denominator / total_winning_numerator
  max_prize_fund_str = max_prize_fund.to_s
end

puts max_prize_fund_str

