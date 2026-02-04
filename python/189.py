"""Project Euler Problem 189 - Tri-colouring a triangular grid.

Find number of valid 3-colourings of a triangular grid with 8 rows,
where no two triangles sharing an edge have the same colour.

Row r (0-indexed) contains 2r+1 small triangles:
  - Upward triangles at even positions: 0, 2, 4, ..., 2r
  - Downward triangles at odd positions: 1, 3, 5, ..., 2r-1

Adjacencies:
  - Consecutive triangles in the same row share an edge.
  - Downward triangle at position 2k+1 in row r shares its top edge
    with upward triangle at position 2k in row r-1.

DP state: colours of upward triangles in current row, encoded as a
base-3 integer. Transition: enumerate all valid colourings of the
next row given the upward-triangle colours of the current row.
"""


def solve(n_rows=8):
    # dp maps (base-3 encoded up-triangle colours in current row) -> count
    # Row 0 has 1 upward triangle, 3 colour choices
    dp = {0: 1, 1: 1, 2: 1}

    for row in range(1, n_rows):
        # Previous row had `row` upward triangles.
        # Current row has (row+1) upward and `row` downward triangles.
        # Positions: 0(up), 1(down), 2(up), 3(down), ..., 2*row(up)
        #
        # Downward triangle at position 2k+1 touches prev_ups[k] (above).
        #
        # Decode prev_ups[k]: (prev_state // 3^k) % 3

        # Precompute powers of 3
        pow3 = [3 ** i for i in range(row + 2)]

        new_dp = {}

        for prev_state, prev_count in dp.items():
            # Decode previous row's up-triangle colours
            prev_ups = []
            tmp = prev_state
            for _ in range(row):
                prev_ups.append(tmp % 3)
                tmp //= 3

            # Process current row left to right
            # State: (last_colour, encoded_up_colours_so_far, num_ups_so_far) -> count
            # Encode up colours in base-3 as we go, tracking how many ups placed.
            # Use dict: (last_colour, partial_up_encoding) -> count

            row_states = {}
            # Position 0: upward triangle, no left neighbour
            for c in range(3):
                key = (c, c)  # (last_colour, up_encoding with just c at position 0)
                row_states[key] = row_states.get(key, 0) + 1

            up_count = 1  # number of up-triangles placed so far

            for pos in range(1, 2 * row + 1):
                new_row_states = {}
                if pos % 2 == 1:
                    # Downward triangle; k = pos // 2
                    k = pos // 2
                    above_c = prev_ups[k]
                    for (last_c, up_enc), cnt in row_states.items():
                        for c in range(3):
                            if c == last_c or c == above_c:
                                continue
                            key = (c, up_enc)
                            new_row_states[key] = new_row_states.get(key, 0) + cnt
                else:
                    # Upward triangle
                    p3 = pow3[up_count]
                    for (last_c, up_enc), cnt in row_states.items():
                        for c in range(3):
                            if c == last_c:
                                continue
                            key = (c, up_enc + c * p3)
                            new_row_states[key] = new_row_states.get(key, 0) + cnt
                    up_count += 1

                row_states = new_row_states

            # Collect: sum over last_colour, group by up_encoding
            for (last_c, up_enc), cnt in row_states.items():
                new_dp[up_enc] = new_dp.get(up_enc, 0) + cnt * prev_count

        dp = new_dp

    return sum(dp.values())


if __name__ == "__main__":
    print(solve())
