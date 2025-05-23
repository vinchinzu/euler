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

# Constants for square indices
GO = 0; JAIL = 10; G2J_SQUARE = 30
C1 = 11; E3 = 24; H2 = 39; R1 = 5
R_NEXT = -1 # Placeholder for "Go to next R" logic
U_NEXT = -2 # Placeholder for "Go to next U" logic
BACK_3 = -3 # Placeholder for "Go back 3 squares"

CC_SQUARES = [2, 17, 33].freeze
CH_SQUARES = [7, 22, 36].freeze

RAILROADS = [5, 15, 25, 35].freeze
UTILITIES = [12, 28].freeze

SQUARE_COUNT = 40
DICE_SIDES = 6 # As per subtask, not 4-sided.

# Card definitions (destination square or special action code)
# Community Chest: 16 cards
# 1/16: Advance to GO
# 1/16: Go to JAIL
# 14/16: No movement (stay on square)
CC_CARDS = [GO, JAIL] + Array.new(14, nil) # nil means no move from card

# Chance: 16 cards
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
CH_CARDS = [GO, JAIL, C1, E3, H2, R1, R_NEXT, R_NEXT, U_NEXT, BACK_3] + Array.new(6, nil)

# --- Helper Functions ---
def roll_dice(sides)
  die1 = rand(1..sides)
  die2 = rand(1..sides)
  is_double = (die1 == die2)
  [die1, die2, die1 + die2, is_double]
end

def handle_cc_action(current_pos, drawn_card_action)
  return current_pos if drawn_card_action.nil?
  drawn_card_action # This will be GO or JAIL
end

def handle_ch_action(current_pos, drawn_card_action)
  return current_pos if drawn_card_action.nil?

  case drawn_card_action
  when R_NEXT
    # Find next railroad
    # CH1 (7) -> R2 (15)
    # CH2 (22) -> R3 (25)
    # CH3 (36) -> R1 (5)
    if current_pos == CH_SQUARES[0] # CH1 (7)
      return RAILROADS[1] # R2 (15)
    elsif current_pos == CH_SQUARES[1] # CH2 (22)
      return RAILROADS[2] # R3 (25)
    else # CH3 (36)
      return RAILROADS[0] # R1 (5)
    end
  when U_NEXT
    # Find next utility
    # CH1 (7) -> U1 (12)
    # CH2 (22) -> U2 (28)
    # CH3 (36) -> U1 (12) (wraps around)
    if current_pos == CH_SQUARES[0] # CH1 (7)
      return UTILITIES[0] # U1 (12)
    elsif current_pos == CH_SQUARES[1] # CH2 (22)
      return UTILITIES[1] # U2 (28)
    else # CH3 (36)
      return UTILITIES[0] # U1 (12)
    end
  when BACK_3
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

# Shuffle card decks (conceptually, as we draw with replacement by probability)
# No explicit shuffle needed if drawing randomly or cycling through a pre-shuffled fixed deck.
# The problem implies probabilities, so we can simulate drawing one of 16 cards.
cc_card_idx = 0
ch_card_idx = 0
# To truly shuffle and pick from bottom:
# cc_deck = CC_CARDS.shuffle
# ch_deck = CH_CARDS.shuffle

NUM_ROLLS.times do
  _die1, _die2, roll_sum, is_double = roll_dice(DICE_SIDES)

  if is_double
    consecutive_doubles += 1
  else
    consecutive_doubles = 0
  end

  if consecutive_doubles == 3
    current_pos = JAIL
    consecutive_doubles = 0
  else
    current_pos = (current_pos + roll_sum) % SQUARE_COUNT

    landed_on_special_square = true # Flag to prevent multiple square actions if card moves player
    
    while landed_on_special_square
      landed_on_special_square = false # Assume no further chained actions unless a card says so

      if current_pos == G2J_SQUARE
        current_pos = JAIL
      elsif CC_SQUARES.include?(current_pos)
        # Draw CC card (using probabilities, effectively cycling or random pick)
        drawn_cc_action = CC_CARDS[rand(0..15)] # Or use cc_deck[cc_card_idx]; cc_card_idx = (cc_card_idx+1)%16
        new_pos_after_cc = handle_cc_action(current_pos, drawn_cc_action)
        if new_pos_after_cc != current_pos
          current_pos = new_pos_after_cc
          # Problem implies card action is final for the turn.
          # However, if a card sends you to another special square (e.g. CH card to G2J),
          # standard Monopoly rules might trigger that.
          # For this problem: "it is the final square that the player finishes at on each roll that we are interested in."
          # This implies that card movements are final. If a card moves to G2J/CC/CH, those don't trigger again *this turn*.
        end
      elsif CH_SQUARES.include?(current_pos)
        # Draw CH card
        drawn_ch_action = CH_CARDS[rand(0..15)] # Or use ch_deck[ch_card_idx]; ch_card_idx = (ch_card_idx+1)%16
        new_pos_after_ch = handle_ch_action(current_pos, drawn_ch_action)
        if new_pos_after_ch != current_pos
          current_pos = new_pos_after_ch
          # If this new_pos_after_ch is G2J or another CC/CH square,
          # does it trigger again? The problem implies the card's destination is final for the turn.
          # "it is the final square that the player finishes at on each roll that we are interested in."
          # This might mean we need to check if the new_pos_after_ch is G2J.
          # If CH sends to G2J, then player goes to JAIL.
          if current_pos == G2J_SQUARE # Check if card action sent to G2J
             current_pos = JAIL
          end
          # The problem implies cards are powerful. If a CH card sends to CC, it might trigger CC.
          # However, for simplicity and "final square" rule, let's assume only one card action,
          # but G2J is a square property that always triggers.
          # The prompt "5/8 request a movement to another square" implies that if moved, that's it.
          # If a card moves to G2J, then that G2J rule applies.
          # If a card moves to another CH/CC, the problem is underspecified.
          # Most interpretations: card action is final, but G2J square property is absolute.
        end
      end
    end # end of while landed_on_special_square (not really looping here with current logic)
  end
  square_visits[current_pos] += 1
end

# --- Result Processing ---
indexed_visits = square_visits.map.with_index { |visits, idx| [idx, visits] }
sorted_visits = indexed_visits.sort_by { |item| -item[1] } # Sort by visits descending

top_3_squares = sorted_visits.first(3)

# Format the result string
# Example: JAIL (10), E3 (24), GO (00) -> "102400"
result_string = top_3_squares.map { |sq_data| sprintf("%02d", sq_data[0]) }.join

puts "The six-digit modal string is: #{result_string}"
