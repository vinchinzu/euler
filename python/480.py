"""Project Euler Problem 480: The Last Question.

The words formed by selecting at most K letters from the phrase S are arranged
in alphabetical order. Let P(w) be the position of word w in the list, and let
W(p) be the word at position p (1-indexed). Find W(Σ_i sign_i P(word_i)) for
the given words.
"""

from __future__ import annotations

from collections import Counter
from typing import Dict, Tuple


def solve() -> str:
    """Solve Problem 480."""
    S = "thereisasyetinsufficientdataforameaningfulanswer"
    K = 15
    WORDS = {
        "legionary": 1,
        "calorimeters": 1,
        "annihilate": -1,
        "orchestrated": 1,
        "fluttering": -1,
    }

    # Count letter frequencies in S
    freqs = Counter(S)

    def f(L: int, letter_freqs: Dict[str, int]) -> int:
        """Number of words of length <= L with given frequencies."""
        if L == 0:
            return 1
        total = 1  # Empty string
        for letter, count in letter_freqs.items():
            if count > 0:
                new_freqs = letter_freqs.copy()
                new_freqs[letter] = count - 1
                total += f(L - 1, new_freqs)
        return total

    def P(word: str) -> int:
        """Position of word."""
        pos = 1
        used = Counter()
        for i, char in enumerate(word):
            for letter in sorted(set(S)):
                if letter < char:
                    if used[letter] < freqs[letter]:
                        temp_used = used.copy()
                        temp_used[letter] += 1
                        remaining_freqs = {
                            c: freqs[c] - temp_used[c]
                            for c in freqs
                            if freqs[c] - temp_used[c] > 0
                        }
                        pos += f(K - i - 1, remaining_freqs)
                elif letter == char:
                    used[char] += 1
                    break
        return pos

    def W(p: int) -> str:
        """Word at position p."""
        result = []
        used = Counter()
        for _ in range(K):
            for letter in sorted(set(S)):
                if used[letter] >= freqs[letter]:
                    continue
                temp_used = used.copy()
                temp_used[letter] += 1
                remaining_freqs = {
                    c: freqs[c] - temp_used[c]
                    for c in freqs
                    if freqs[c] - temp_used[c] > 0
                }
                count = f(K - len(result) - 1, remaining_freqs)
                if count >= p:
                    result.append(letter)
                    used[letter] += 1
                    break
                p -= count
        return "".join(result)

    total_pos = 0
    for word, sign in WORDS.items():
        total_pos += sign * P(word)

    return W(total_pos)


def main() -> str:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
