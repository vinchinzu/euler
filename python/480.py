"""Project Euler Problem 480: The Last Question.

From the phrase "thereisasyetinsufficientdataforameaningfulanswer", consider
all words formed by selecting at most 15 letters (respecting available counts).
Words are listed alphabetically. Given P(w)=position of word w, find
W(P(legionary) + P(calorimeters) - P(annihilate) + P(orchestrated) - P(fluttering)).
"""

from collections import Counter
from math import factorial
from fractions import Fraction


def solve():
    S = "thereisasyetinsufficientdataforameaningfulanswer"
    K = 15
    WORDS = {
        "legionary": 1,
        "calorimeters": 1,
        "annihilate": -1,
        "orchestrated": 1,
        "fluttering": -1,
    }

    freqs = Counter(S)
    letters = sorted(freqs.keys())
    letter_to_idx = {c: i for i, c in enumerate(letters)}
    freq_list = [freqs[c] for c in letters]
    n_letters = len(letters)

    fact = [factorial(i) for i in range(K + 1)]
    # inv_fact as exact fractions
    inv_fact = [Fraction(1, factorial(i)) for i in range(K + 1)]

    def count_words_from_freqs(avail, max_len):
        """Count words of length 1..max_len formable from avail frequencies.
        Uses convolution DP with exact Fraction arithmetic."""
        if max_len <= 0:
            return 0
        # dp[j] = sum of prod(1/ci!) over all ways to pick j letters from available
        dp = [Fraction(0)] * (max_len + 1)
        dp[0] = Fraction(1)

        for i in range(n_letters):
            fi = avail[i]
            if fi == 0:
                continue
            new_dp = [Fraction(0)] * (max_len + 1)
            for j in range(max_len + 1):
                if dp[j] == 0:
                    continue
                for c in range(min(fi, max_len - j) + 1):
                    new_dp[j + c] += dp[j] * inv_fact[c]
            dp = new_dp

        total = 0
        for L in range(1, max_len + 1):
            total += int(fact[L] * dp[L])
        return total

    def P(word):
        """Position of word in the sorted list (1-indexed, no empty word)."""
        pos = 0
        avail = list(freq_list)

        for i, char in enumerate(word):
            ci = letter_to_idx[char]
            for li in range(ci):
                if avail[li] > 0:
                    avail[li] -= 1
                    pos += 1 + count_words_from_freqs(avail, K - i - 1)
                    avail[li] += 1
            avail[ci] -= 1
            if i < len(word) - 1:
                pos += 1

        pos += 1
        return pos

    def W(p):
        """Word at position p (1-indexed, no empty word)."""
        result = []
        avail = list(freq_list)

        for depth in range(K):
            found = False
            for li in range(n_letters):
                if avail[li] > 0:
                    avail[li] -= 1
                    cnt = 1 + count_words_from_freqs(avail, K - depth - 1)
                    if p <= cnt:
                        result.append(letters[li])
                        if p == 1:
                            return "".join(result)
                        p -= 1
                        found = True
                        break
                    p -= cnt
                    avail[li] += 1
            if not found:
                break

        return "".join(result)

    total_pos = 0
    for word, sign in WORDS.items():
        total_pos += sign * P(word)

    return W(total_pos)


def main():
    result = solve()
    print(result)


if __name__ == "__main__":
    main()
