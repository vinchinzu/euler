def solve():
    def next_turn(prev_turn, player):
        """Find smallest turn t > prev_turn where (t-1) % 3 == player."""
        base = player + 1  # smallest turn for this player
        if base > prev_turn:
            return base
        k = (prev_turn - base) // 3 + 1
        return base + 3 * k

    def F(A, B, C):
        """Compute the number of turns for triple (A, B, C)."""
        cur = [A, B, C]
        batches = []

        while True:
            max_val = max(cur)
            max_idx = cur.index(max_val)
            others = sorted([(cur[i], i) for i in range(3) if i != max_idx])
            min_val, min_idx = others[0]
            mid_val, mid_idx = others[1]

            if min_val == 0:
                break

            q = mid_val // min_val
            remainder = mid_val % min_val

            batches.append((max_idx, mid_idx, q))

            if q % 2 == 1:
                cur[max_idx] = remainder
                cur[mid_idx] = remainder + min_val
            else:
                cur[mid_idx] = remainder
                cur[max_idx] = remainder + min_val

        batches.reverse()

        prev_turn = 0

        for p_a, p_b, q in batches:
            # Reversed batch pattern
            if q % 2 == 1:
                first_player = p_a
                second_player = p_b
            else:
                first_player = p_b
                second_player = p_a

            # Process q alternating entries: first_player, second_player, first_player, ...
            # After the first 1-2 entries, the pattern stabilizes:
            # each pair (first, second) advances by exactly 3 turns total.

            if q == 0:
                continue

            # Process first entry
            prev_turn = next_turn(prev_turn, first_player)
            remaining = q - 1

            if remaining == 0:
                continue

            # Process second entry
            prev_turn = next_turn(prev_turn, second_player)
            remaining -= 1

            if remaining == 0:
                continue

            # Now we're in steady state. Each pair advances by exactly 3.
            # Number of complete pairs remaining: remaining // 2
            # Leftover: remaining % 2

            pairs = remaining // 2
            leftover = remaining % 2

            # Each pair: first_player, second_player
            # After the initial two entries, the gap between consecutive same-player turns is 3.
            # So each pair advances prev_turn by 3.
            prev_turn += pairs * 3

            if leftover:
                prev_turn = next_turn(prev_turn, first_player)

        return prev_turn

    # Verify
    assert F(2, 1, 1) == 1, f"F(2,1,1)={F(2,1,1)}"
    assert F(2, 7, 5) == 5, f"F(2,7,5)={F(2,7,5)}"

    total = 0
    for a in range(1, 8):
        for b in range(1, 20):
            A_val = a ** b
            B_val = b ** a
            C_val = A_val + B_val
            f = F(A_val, B_val, C_val)
            total += f

    print(total)

if __name__ == "__main__":
    solve()
