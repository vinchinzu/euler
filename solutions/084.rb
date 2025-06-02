#  <p>In the game, <strong>Monopoly</strong>, the standard board is set up in the f
# ollowing way:</p>
# <div class="center">
# <img src="resources/images/0084_monopoly_board.png?1678992052" alt="0084_monopol
# y_board.png">
# </div>
# <p>A player starts on the GO square and adds the scores on two 6-sided dice to d
# etermine the number of squares they advance in a clockwise direction. Without an
# y further rules we would expect to visit each square with equal probability: 2.5
# %. However, landing on G2J (Go To Jail), CC (community chest), and CH (chance) c
# hanges this distribution.</p>
# <p>In addition to G2J, and one card from each of CC and CH, that orders the play
# er to go directly to jail, if a player rolls three consecutive doubles, they do
# not advance the result of their 3rd roll. Instead they proceed directly to jail.
# </p>
# <p>At the beginning of the game, the CC and CH cards are shuffled. When a player
#  lands on CC or CH they take a card from the top of the respective pile and, aft
# er following the instructions, it is returned to the bottom of the pile. There a
# re sixteen cards in each pile, but for the purpose of this problem we are only c
# oncerned with cards that order a movement; any instruction not concerned with mo
# vement will be ignored and the player will remain on the CC/CH square.</p>
# <ul><li>Community Chest (2/16 cards):
# <ol><li>Advance to GO</li>
# <li>Go to JAIL</li>
# </ol></li>
# <li>Chance (10/16 cards):
# <ol><li>Advance to GO</li>
# <li>Go to JAIL</li>
# <li>Go to C1</li>
# <li>Go to E3</li>
# <li>Go to H2</li>
# <li>Go to R1</li>
# <li>Go to next R (railway company)</li>
# <li>Go to next R</li>
# <li>Go to next U (utility company)</li>
# <li>Go back 3 squares.</li>
# </ol></li>
# </ul><p>The heart of this problem concerns the likelihood of visiting a particul
# ar square. That is, the probability of finishing at that square after a roll. Fo
# r this reason it should be clear that, with the exception of G2J for which the p
# robability of finishing on it is zero, the CH squares will have the lowest proba
# bilities, as 5/8 request a movement to another square, and it is the final squar
# e that the player finishes at on each roll that we are interested in. We shall m
# ake no distinction between "Just Visiting" and being sent to JAIL, and we shall
# also ignore the rule about requiring a double to "get out of jail", assuming tha
# t they pay to get out on their next turn.</p>
# <p>By starting at GO and numbering the squares sequentially from 00 to 39 we can
#  concatenate these two-digit numbers to produce strings that correspond with set
# s of squares.</p>
# <p>Statistically it can be shown that the three most popular squares, in order,
# are JAIL (6.24%) = Square 10, E3 (3.18%) = Square 24, and GO (3.09%) = Square 00
# . So these three most popular squares can be listed with the six-digit modal str
# ing: 102400.</p>
# <p>If, instead of using two 6-sided dice, two 4-sided dice are used, find the si
# x-digit modal string.</p> # NOTE: The current subtask uses 6-sided dice.

# Solution for Project Euler Problem 84

# Solution for Project Euler Problem 84 - Revised

# --- Constants for square indices ---
GO = 0; JAIL = 10; G2J_SQUARE = 30
C1 = 11; E3 = 24; H2 = 39; R1 = 5
# Square group definitions
CC1 = 2; CC2 = 17; CC3 = 33
CH1 = 7; CH2 = 22; CH3 = 36
R2 = 15; R3 = 25; R4 = 35 # R1 is already defined
U1 = 12; U2 = 28

CC_SQUARES = [CC1, CC2, CC3].freeze
CH_SQUARES = [CH1, CH2, CH3].freeze
RAILROADS = [R1, R2, R3, R4].freeze
UTILITIES = [U1, U2].freeze

R_NEXT = -1 # Placeholder for "Go to next R" logic
U_NEXT = -2 # Placeholder for "Go to next U" logic
BACK_3 = -3 # Placeholder for "Go back 3 squares"

# CC_SQUARES = .freeze # Original erroneous lines
# CH_SQUARES = .freeze

# RAILROADS = .freeze
# UTILITIES = .freeze

SQUARE_COUNT = 40
DICE_SIDES = 4 # For 6-sided dice validation; change to 4 for the main problem

# --- Card Definitions (destination square or special action code) ---
# Community Chest: 16 cards. Problem statement: 2/16 move.
# 1/16: Advance to GO
# 1/16: Go to JAIL
# 14/16: No movement
CC_CARD_EFFECTS = [GO, JAIL] + Array.new(14, nil) # nil means no move from card

# Chance: 16 cards. Problem statement: 10/16 move.
# 1/16: Advance to GO
# 1/16: Go to JAIL
# 1/16: Go to C1
# 1/16: Go to E3
# 1/16: Go to H2
# 1/16: Go to R1
# 2/16: Go to next R (represented by R_NEXT)
# 1/16: Go to next U (represented by U_NEXT)
# 1/16: Go back 3 squares (represented by BACK_3)
# 6/16: No movement
CH_CARD_EFFECTS = [
  GO, JAIL, C1, E3, H2, R1, # Specific destinations
  R_NEXT, R_NEXT,           # Go to next R (two cards)
  U_NEXT,                   # Go to next U
  BACK_3                    # Go back 3 squares
] + Array.new(6, nil)       # No movement

# --- Helper Functions ---
def roll_dice(sides)
  die1 = rand(1..sides)
  die2 = rand(1..sides)
  is_double = (die1 == die2)
  # Returns [die1, die2, sum, is_double_flag]
  [die1, die2, die1 + die2, is_double]
end

def handle_cc_action(current_pos, drawn_card_action)
  # If card action is nil, it means "no movement" from this card.
  return current_pos if drawn_card_action.nil?
  # Otherwise, the action is a direct move to GO or JAIL.
  drawn_card_action
end

def handle_ch_action(current_pos, drawn_card_action)
  # If card action is nil, it means "no movement" from this card.
  return current_pos if drawn_card_action.nil?

  case drawn_card_action
  when R_NEXT # Go to next Railroad
    # This logic depends on which CH square (current_pos) the player is on.
    if current_pos == CH1 # CH1 (7)
      return R2 # R2 (15)
    elsif current_pos == CH2 # CH2 (22)
      return R3 # R3 (25)
    else # CH3 (36)
      return R1 # R1 (5)
    end
  when U_NEXT # Go to next Utility
    # This logic depends on which CH square (current_pos) the player is on.
    if current_pos == CH1 # CH1 (7)
      return U1 # U1 (12)
    elsif current_pos == CH2 # CH2 (22)
      return U2 # U2 (28)
    else # CH3 (36)
      return U1 # U1 (12) (wraps around)
    end
  when BACK_3 # Go back 3 squares
    return (current_pos - 3 + SQUARE_COUNT) % SQUARE_COUNT
  else
    # Direct move to a specific square (GO, JAIL, C1, E3, H2, R1)
    return drawn_card_action
  end
end

# --- Main Simulation ---
NUM_ROLLS = 5_000_000 # Number of turns/rolls for simulation
square_visits = Array.new(SQUARE_COUNT, 0)
current_pos = GO
consecutive_doubles = 0

# Initialize and shuffle card decks (once at the beginning)
# This simulates the "shuffled once, then cards drawn and returned to bottom" mechanic.
cc_deck = CC_CARD_EFFECTS.shuffle
ch_deck = CH_CARD_EFFECTS.shuffle

NUM_ROLLS.times do
  _die1, _die2, roll_sum, is_double = roll_dice(DICE_SIDES)

  if is_double
    consecutive_doubles += 1
  else
    consecutive_doubles = 0 # Reset counter if not a double
  end

  if consecutive_doubles == 3
    current_pos = JAIL # Go to Jail directly
    consecutive_doubles = 0 # Reset counter after going to jail
  else
    # Advance player based on roll sum
    current_pos = (current_pos + roll_sum) % SQUARE_COUNT

    # --- Chained Event Resolution Loop ---
    # Process special squares (G2J, CC, CH) until landing on a non-special square
    # or a card results in no movement.
    loop do
      # Store position before card/G2J action to check if movement occurred
      pos_before_action = current_pos

      if current_pos == G2J_SQUARE
        current_pos = JAIL
        # G2J is a terminal action for this chain; JAIL is not a CC/CH square.
        # The loop will break in the 'else' or if pos_before_action == current_pos check.
        # Explicit break here is fine as JAIL doesn't trigger further cards.
        break # Player is in JAIL, turn processing for this square ends.
      elsif CC_SQUARES.include?(current_pos)
        # Draw a Community Chest card from the top of the cycling deck
        drawn_cc_action = cc_deck.first
        cc_deck.rotate! # Move the drawn card to the bottom of the deck

        current_pos = handle_cc_action(current_pos, drawn_cc_action)
      elsif CH_SQUARES.include?(current_pos)
        # Draw a Chance card from the top of the cycling deck
        drawn_ch_action = ch_deck.first
        ch_deck.rotate! # Move the drawn card to the bottom of the deck

        current_pos = handle_ch_action(current_pos, drawn_ch_action)
      else
        # Landed on a regular square or a square that resolved to a regular one.
        break # No further special actions from this square.
      end

      # If the card/action resulted in no change of position, the turn ends here.
      # (e.g. "Stay on square" card, or G2J already moved to JAIL and loop iterated once more)
      # Exception: If we were on G2J, pos_before_action was G2J_SQUARE, current_pos is JAIL. They are different.
      # This condition is primarily for CC/CH "no move" cards.
      if current_pos == pos_before_action
        break
      end
      # If position changed due to a card, loop again ('next' is implicit) to check the new square.
    end
  end
  # Record the final square visited for this turn
  square_visits[current_pos] += 1
end

# --- Result Processing ---
indexed_visits = square_visits.map.with_index { |visits, idx| [idx, visits] }
# Sort by visits descending, then by square index ascending for ties (though ties unlikely here)
sorted_visits = indexed_visits.sort_by { |item| [-item[1], item[0]] }

top_3_squares = sorted_visits.first(3)

# Format the result string (e.g., "102400")
result_string = top_3_squares.map { |sq_data| sprintf("%02d", sq_data[0]) }.join

puts "Using #{DICE_SIDES}-sided dice:"
puts "The six-digit modal string is: #{result_string}"
# For 6-sided dice, expected: 102400
# For 4-sided dice, this will compute the problem's required answer.

# Output probabilities for verification (optional)
puts "\nSquare Visit Frequencies (Top 10):"
sorted_visits.first(10).each do |sq_data| # sq_data is [index, visits]
  percentage = (sq_data[1].to_f / NUM_ROLLS) * 100
  puts "Square %02d: %.2f%% (%d visits)" % [sq_data[0], percentage, sq_data[1]]
end