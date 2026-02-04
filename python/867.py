import sys
sys.setrecursionlimit(100000)

def solve():
    N = 10
    M = 10**9 + 7
    cache = {}

    def tilings_for_dodecagon(a, b, allow_a, allow_b):
        if a == 0:
            return tilings_for_hexagon(b)
        if b == 0:
            return tilings_for_hexagon(a)
        key = ('D', a, b, allow_a, allow_b)
        if key in cache:
            return cache[key]
        res = 0
        if allow_a:
            for h in range(1, b + 1):
                res += pow(tilings_for_trapezoid(b, h), 6, M) * tilings_for_dodecagon(a, b - h, False, True) % M
        if allow_b:
            for h in range(1, a + 1):
                res += pow(tilings_for_trapezoid(a, h), 6, M) * tilings_for_dodecagon(a - h, b, True, False) % M
        if a == 1 and b == 1:
            res += 1
        res %= M
        cache[key] = res
        return res

    def tilings_for_hexagon(size):
        points = []
        for y in range(-size + 1, size):
            for x in range(-2 * size + abs(y) + 2, 2 * size - abs(y), 2):
                points.append((x, y))
        return tilings_with_tri_hex(points, ('hex', size), 0, 0, 2 * size - 1)

    def tilings_for_trapezoid(base, height):
        points = []
        for y in range(base - height, base - 1):
            for x in range(1 - y, y, 2):
                points.append((x, y))
        return tilings_with_tri_hex(points, ('trap', base, height), 0, 0, base - 1)

    def tilings_with_tri_hex(points, type_key, index, prev_bitset, window_len):
        if index == len(points):
            return 1
        key = ('TH', type_key, index, prev_bitset % (1 << window_len))
        if key in cache:
            return cache[key]
        p = points[index]
        good = True
        for i in range(min(window_len, index)):
            if (prev_bitset & (1 << i)) > 0:
                q = points[index - i - 1]
                if abs(p[1] - q[1]) <= 1 and abs(p[0] - q[0]) + abs(p[1] - q[1]) <= 2:
                    good = False
                    break
        res = tilings_with_tri_hex(points, type_key, index + 1, prev_bitset * 2, window_len)
        if good:
            res += tilings_with_tri_hex(points, type_key, index + 1, prev_bitset * 2 + 1, window_len)
        res %= M
        cache[key] = res
        return res

    print(tilings_for_dodecagon(N, N, True, True))

if __name__ == "__main__":
    solve()
