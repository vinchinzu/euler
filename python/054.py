#!/usr/bin/env python3
from collections import Counter

CARD_VALUES = {
    '2': 2, '3': 3, '4': 4, '5': 5, '6': 6, '7': 7, '8': 8, '9': 9, 'T': 10,
    'J': 11, 'Q': 12, 'K': 13, 'A': 14
}

def parse_hand(cards):
    values = sorted([CARD_VALUES[c[0]] for c in cards], reverse=True)
    suits = [c[1] for c in cards]
    return values, suits

def hand_rank(cards):
    values, suits = parse_hand(cards)
    counts = Counter(values)
    ordered_counts = sorted(counts.items(), key=lambda x: (-x[1], -x[0]))
    ordered = [v for v, c in ordered_counts]

    is_flush = len(set(suits)) == 1
    is_straight = values == list(range(values[0], values[0] - 5, -1))
    is_wheel = values == [14, 5, 4, 3, 2]

    if is_straight or is_wheel:
        straight_high = 5 if is_wheel else max(values)

        if is_flush:
            return [8, straight_high]
        else:
            return [4, straight_high]

    if ordered_counts[0][1] == 4:
        return [7, ordered_counts[0][0], ordered_counts[1][0]]
    if ordered_counts[0][1] == 3 and ordered_counts[1][1] == 2:
        return [6, ordered_counts[0][0], ordered_counts[1][0]]
    if is_flush:
        return [5] + values
    if ordered_counts[0][1] == 3:
        return [3, ordered_counts[0][0]] + ordered
    if ordered_counts[0][1] == 2 and ordered_counts[1][1] == 2:
        return [2, max(ordered_counts[0][0], ordered_counts[1][0]),
                min(ordered_counts[0][0], ordered_counts[1][0])] + ordered
    if ordered_counts[0][1] == 2:
        return [1, ordered_counts[0][0]] + ordered
    return [0] + values

from pathlib import Path

def main():
    script_dir = Path(__file__).parent
    data_file = script_dir.parent / 'data' / 'poker.txt'
    
    count = 0
    with open(data_file) as f:
        for line in f:
            cards = line.strip().split()
            hand1 = cards[:5]
            hand2 = cards[5:]
            if hand_rank(hand1) > hand_rank(hand2):
                count += 1
    
    print(count)

if __name__ == "__main__":
    main()
