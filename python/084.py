#!/usr/bin/env python3
"""
Monopoly (Problem 84)

Simulate Monopoly board to find the three most popular squares when using
two 4-sided dice.
"""

import random
from typing import List

# Square indices
GO = 0
JAIL = 10
G2J_SQUARE = 30
C1 = 11
E3 = 24
H2 = 39
R1 = 5
CC1, CC2, CC3 = 2, 17, 33
CH1, CH2, CH3 = 7, 22, 36
R2, R3, R4 = 15, 25, 35
U1, U2 = 12, 28

CC_SQUARES = [CC1, CC2, CC3]
CH_SQUARES = [CH1, CH2, CH3]
RAILROADS = [R1, R2, R3, R4]
UTILITIES = [U1, U2]

SQUARE_COUNT = 40
DICE_SIDES = 4
NUM_ROLLS = 5_000_000

# Special action codes
R_NEXT = -1
U_NEXT = -2
BACK_3 = -3

# CC cards: 2/16 move
CC_CARD_EFFECTS = [GO, JAIL] + [None] * 14

# CH cards: 10/16 move
CH_CARD_EFFECTS = [
    GO, JAIL, C1, E3, H2, R1,
    R_NEXT, R_NEXT,
    U_NEXT,
    BACK_3
] + [None] * 6


def roll_dice(sides: int) -> tuple[int, int, int, bool]:
    """Roll two dice and return results."""
    die1 = random.randint(1, sides)
    die2 = random.randint(1, sides)
    is_double = (die1 == die2)
    return die1, die2, die1 + die2, is_double


def handle_cc_action(current_pos: int, card_action: int | None) -> int:
    """Handle Community Chest card action."""
    if card_action is None:
        return current_pos
    return card_action


def handle_ch_action(current_pos: int, card_action: int | None) -> int:
    """Handle Chance card action."""
    if card_action is None:
        return current_pos
    
    if card_action == R_NEXT:
        if current_pos == CH1:
            return R2
        elif current_pos == CH2:
            return R3
        else:  # CH3
            return R1
    elif card_action == U_NEXT:
        if current_pos == CH1:
            return U1
        elif current_pos == CH2:
            return U2
        else:  # CH3
            return U1
    elif card_action == BACK_3:
        return (current_pos - 3 + SQUARE_COUNT) % SQUARE_COUNT
    else:
        return card_action


def main() -> None:
    """Run Monopoly simulation."""
    square_visits = [0] * SQUARE_COUNT
    current_pos = GO
    consecutive_doubles = 0
    
    # Initialize and shuffle card decks
    cc_deck = CC_CARD_EFFECTS.copy()
    ch_deck = CH_CARD_EFFECTS.copy()
    random.shuffle(cc_deck)
    random.shuffle(ch_deck)
    
    for _ in range(NUM_ROLLS):
        _, _, roll_sum, is_double = roll_dice(DICE_SIDES)
        
        if is_double:
            consecutive_doubles += 1
        else:
            consecutive_doubles = 0
        
        if consecutive_doubles == 3:
            current_pos = JAIL
            consecutive_doubles = 0
        else:
            # Advance player
            current_pos = (current_pos + roll_sum) % SQUARE_COUNT
            
            # Process special squares
            while True:
                pos_before_action = current_pos
                
                if current_pos == G2J_SQUARE:
                    current_pos = JAIL
                    break
                elif current_pos in CC_SQUARES:
                    drawn_action = cc_deck[0]
                    cc_deck = cc_deck[1:] + [cc_deck[0]]
                    current_pos = handle_cc_action(current_pos, drawn_action)
                elif current_pos in CH_SQUARES:
                    drawn_action = ch_deck[0]
                    ch_deck = ch_deck[1:] + [ch_deck[0]]
                    current_pos = handle_ch_action(current_pos, drawn_action)
                else:
                    break
                
                if current_pos == pos_before_action:
                    break
        
        square_visits[current_pos] += 1
    
    # Find top 3 squares
    indexed_visits = [(idx, visits) for idx, visits in enumerate(square_visits)]
    sorted_visits = sorted(indexed_visits, key=lambda x: (-x[1], x[0]))
    
    top_3 = sorted_visits[:3]
    result_string = ''.join(f"{sq:02d}" for sq, _ in top_3)
    print(result_string)


if __name__ == "__main__":
    main()
