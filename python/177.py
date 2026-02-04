"""Project Euler Problem 177: Integer Angled Quadrilaterals.

Find the number of non-similar quadrilaterals where all 8 interior angles
(formed by the diagonals) are integer degrees.

Approach: Iterate over 4 angles (a,b,c,d), compute the remaining angle f
using atan, check if it's an integer. Canonicalize by rotating/reflecting
and store hashes to avoid counting duplicates. Ported from Java reference.
"""

import math

def main():
    C = 180
    RAD = math.pi / 180.0

    sin = [math.sin(i * RAD) for i in range(C)]
    cos = [math.cos(i * RAD) for i in range(C)]

    hashes = set()

    for a in range(1, C // 4 + 1):
        for b in range(a, C - 3 * a + 1):
            for c in range(a, C - 2 * a - b + 1):
                for d in range(a, C - a - b - c + 1):
                    AD = sin[c] / sin[a + b + c]
                    denom = sin[b + c + d]
                    if denom == 0:
                        continue
                    AC = sin[c + d] / denom
                    diff = AC - AD * cos[a]
                    if diff == 0:
                        continue
                    f = math.degrees(math.atan((AD * sin[a]) / diff))
                    fi = round(f)
                    if abs(f - fi) < 1e-9:
                        if fi < 0:
                            fi += C
                        angles = [
                            a, b, c, d,
                            C - b - c - d,
                            fi,
                            b + c - fi,
                            C - a - b - c
                        ]
                        # Check all angles are positive
                        if any(x <= 0 for x in angles):
                            continue

                        # Canonicalize: find minimum hash over all rotations and reflections
                        min_hash = None
                        for start in range(8):
                            h = 0
                            # parity: even start => forward, odd start => backward
                            if start % 2 == 0:
                                for i in range(8):
                                    h = h * C + angles[(start + i) % 8]
                            else:
                                for i in range(8):
                                    h = h * C + angles[(start - i) % 8]
                            if min_hash is None or h < min_hash:
                                min_hash = h
                        hashes.add(min_hash)

    return len(hashes)

if __name__ == "__main__":
    print(main())
