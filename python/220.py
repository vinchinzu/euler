"""Project Euler Problem 220: Heighway Dragon.

Find the position after 10^12 steps in D_50 of the Heighway dragon curve.

D_0 = "Fa", with rewriting rules: a -> aRbFR, b -> LFaLb.
F = move forward, L = turn left 90, R = turn right 90, a/b = ignored at final level.

Strategy: precompute for each level n, the full traversal summary of 'a' and 'b'
(total F-steps, net displacement, net direction change). Then recursively walk
the string, skipping full sub-trees when possible, and descending only when
we need a partial traversal.
"""


def solve():
    N = 10**12
    MAX_LEVEL = 50

    # Directions: 0=up(+y), 1=left(-x), 2=down(-y), 3=right(+x)
    DX = (0, -1, 0, 1)
    DY = (1, 0, -1, 0)

    def rotate(dx, dy, d):
        """Rotate vector (dx, dy) by d*90 degrees counterclockwise."""
        if d == 0:
            return dx, dy
        elif d == 1:
            return -dy, dx
        elif d == 2:
            return -dx, -dy
        else:
            return dy, -dx

    # Precompute full traversal summaries for 'a' and 'b' at each level.
    # summary[level]['a'] = (steps, dx, dy, ddir)
    # where steps = total F-steps, (dx, dy) = net displacement, ddir = net direction change (mod 4)

    # At level 0, 'a' and 'b' are just ignored (no steps, no movement, no rotation).
    summary = [None] * (MAX_LEVEL + 1)
    summary[0] = {'a': (0, 0, 0, 0), 'b': (0, 0, 0, 0)}

    for level in range(1, MAX_LEVEL + 1):
        prev = summary[level - 1]
        new = {}

        # a -> aRbFR at level means: expand 'a' at (level-1), then R, then expand 'b' at (level-1), then F, then R
        # Process "aRbFR":
        # Start with direction 0 (relative), accumulate
        steps, dx, dy, d = 0, 0, 0, 0

        # 'a' at level-1
        s, sx, sy, sd = prev['a']
        rx, ry = rotate(sx, sy, d)
        dx += rx
        dy += ry
        d = (d + sd) % 4
        steps += s

        # R
        d = (d + 3) % 4  # turn right = -1 mod 4 = +3

        # 'b' at level-1
        s, sx, sy, sd = prev['b']
        rx, ry = rotate(sx, sy, d)
        dx += rx
        dy += ry
        d = (d + sd) % 4
        steps += s

        # F
        dx += DX[d]
        dy += DY[d]
        steps += 1

        # R
        d = (d + 3) % 4

        new['a'] = (steps, dx, dy, d)

        # b -> LFaLb at level means: L, then F, then expand 'a' at (level-1), then L, then expand 'b' at (level-1)
        steps, dx, dy, d = 0, 0, 0, 0

        # L
        d = (d + 1) % 4

        # F
        dx += DX[d]
        dy += DY[d]
        steps += 1

        # 'a' at level-1
        s, sx, sy, sd = prev['a']
        rx, ry = rotate(sx, sy, d)
        dx += rx
        dy += ry
        d = (d + sd) % 4
        steps += s

        # L
        d = (d + 1) % 4

        # 'b' at level-1
        s, sx, sy, sd = prev['b']
        rx, ry = rotate(sx, sy, d)
        dx += rx
        dy += ry
        d = (d + sd) % 4
        steps += s

        new['b'] = (steps, dx, dy, d)
        summary[level] = new

    # Now walk D_50 = "Fa" at level 50, taking at most N steps.
    # We do this iteratively/recursively through the string expansion.

    # Walk the top-level string "Fa" at level MAX_LEVEL.
    # 'F' is a literal step, 'a' expands according to the rules.

    # We use an explicit stack to avoid deep recursion.
    # Stack entries: (string_to_process, level, char_index)
    # We process one character at a time.

    x, y, d = 0, 0, 0  # current position and direction
    steps_taken = 0

    # Instead of a stack of strings, we use a recursive function with
    # bounded depth (max 50 levels), which is safe.

    def walk(string, level, remaining):
        """Walk through string at given level, taking at most 'remaining' F-steps.
        Updates x, y, d (nonlocal). Returns number of steps taken."""
        nonlocal x, y, d
        taken = 0

        for c in string:
            if taken >= remaining:
                break

            if c == 'F':
                x += DX[d]
                y += DY[d]
                taken += 1
            elif c == 'L':
                d = (d + 1) % 4
            elif c == 'R':
                d = (d + 3) % 4
            elif level > 0:
                # c is 'a' or 'b', expand it
                expansion = 'aRbFR' if c == 'a' else 'LFaLb'
                sub_summary = summary[level - 1][c]
                sub_steps = sub_summary[0]
                left = remaining - taken

                if sub_steps <= left:
                    # Take the full sub-path
                    sdx, sdy = rotate(sub_summary[1], sub_summary[2], d)
                    x += sdx
                    y += sdy
                    d = (d + sub_summary[3]) % 4
                    taken += sub_steps
                else:
                    # Need partial traversal - recurse one level deeper
                    took = walk(expansion, level - 1, left)
                    taken += took
            # else: level == 0, 'a'/'b' are no-ops

        return taken

    walk("Fa", MAX_LEVEL, N)
    return x, y


if __name__ == "__main__":
    x, y = solve()
    print(f"{x},{y}")
