"""Project Euler Problem 177: Integer Angled Quadrilaterals."""

import math
from math import sin, pi

def main():
    # Attempt to implement correct fast check, as slow python is acceptable if correct.
    # The previous implementation (Step 71) was a partial C++ port that worked but was slow.
    # We will restore that logic but maybe optimize slightly.
    
    RAD = math.pi / 180.0
    EPS = 1e-9
    BD_EPS = 1e-8
    LENGTH_EPS = 1e-10
    RATIO_SCALE = 1_000_000_000

    SIN = [math.sin(i * RAD) for i in range(181)]
    COS = [math.cos(i * RAD) for i in range(181)]

    ratio_lookup = [[None] * 181 for _ in range(181)]

    for sum_val in range(2, 179):
        max_b = 179 - sum_val
        if max_b <= 0: continue

        for b in range(1, max_b + 1):
            ratio = SIN[sum_val + b] / SIN[b]
            ratio_lookup[sum_val][b] = ratio

    tri_a = []
    tri_c = []
    tri_b = []
    tri_side_a = []
    tri_side_c = []

    for a in range(1, 180):
        max_c = 179 - a - 1
        if max_c < 1: continue

        for c in range(1, max_c + 1):
            b = 180 - a - c
            if b <= 1: continue

            tri_a.append(a)
            tri_c.append(c)
            tri_b.append(b)
            denom = SIN[b]
            tri_side_a.append(SIN[a] / denom)
            tri_side_c.append(SIN[c] / denom)

    size = len(tri_a)
    # Mapping
    ad_map = [{} for _ in range(180)]

    for idx in range(size):
        a = tri_a[idx]
        key = int(round(tri_side_c[idx] * RATIO_SCALE))
        if key not in ad_map[a]:
            ad_map[a][key] = []
        ad_map[a][key].append(idx)

    total = 0

    for i in range(size):
        a1 = tri_a[i]
        c1 = tri_c[i]
        b_sum = tri_b[i]
        if b_sum <= 1: continue

        ab = tri_side_c[i]
        bc = tri_side_a[i]

        min_a2 = 1
        max_a2 = 178 - a1
        if max_a2 < min_a2: continue

        for a2 in range(min_a2, max_a2 + 1):
            if not ad_map[a2]: continue

            a_sum = a1 + a2
            if a_sum >= 180: continue

            ratio_list_a = ratio_lookup[a_sum]
            max_b1 = 179 - a_sum
            if max_b1 <= 0: continue

            for b1 in range(1, max_b1 + 1):
                ratio1 = ratio_list_a[b1]
                if ratio1 is None: continue

                required_ad = ab / ratio1
                key = int(round(required_ad * RATIO_SCALE))
                candidates = []
                for delta in range(-2, 3):
                    ckey = key + delta
                    if ckey in ad_map[a2]:
                        candidates.extend(ad_map[a2][ckey])

                if not candidates: continue

                b2_base = b_sum - b1
                if b2_base <= 0: continue

                for idx2 in candidates:
                    if idx2 <= i: continue

                    ad = tri_side_c[idx2]
                    if abs(ad - required_ad) > LENGTH_EPS: continue

                    c2 = tri_c[idx2]
                    d_sum = tri_b[idx2]
                    if d_sum <= 1: continue

                    c_sum = c1 + c2
                    if c_sum >= 180: continue

                    b2 = b2_base
                    max_b2 = 179 - c_sum
                    if b2 <= 0 or b2 > max_b2: continue

                    ratio2 = ratio_lookup[c_sum][b2]
                    if ratio2 is None: continue

                    dc = tri_side_a[idx2]
                    if dc == 0: continue
                    r2 = bc / dc
                    if abs(ratio2 - r2) > EPS: continue

                    d1 = 180 - a_sum - b1
                    if d1 <= 0: continue
                    d2 = 180 - c_sum - b2
                    if d2 <= 0: continue

                    cos_a_sum = COS[a_sum]
                    cos_c_sum = COS[c_sum]

                    bd_sq1 = ab * ab + ad * ad - 2.0 * ab * ad * cos_a_sum
                    if bd_sq1 <= 0: continue
                    bd_sq2 = bc * bc + dc * dc - 2.0 * bc * dc * cos_c_sum
                    if abs(bd_sq1 - bd_sq2) > BD_EPS: continue

                    total += 1

    return total

if __name__ == "__main__":
    print(main())
