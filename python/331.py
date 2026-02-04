"""Project Euler Problem 331 - Cross flips.

For even n, T(n) = 2 * numOddRows * (n - numOddRows) + correction,
computed by walking along the circle of black disks.
For odd n != 5, T(n) = 0 (impossible).
T(5) = 3.
Sum T(2^i - i) for i = 3..31.
"""
from math import isqrt

def solve():
    MAX_I = 31

    def T(n):
        n2 = n * n
        nm1_2 = (n - 1) * (n - 1)
        correction = 0
        num_odd_rows = 0
        x = 0
        y = n - 1
        left_border = False

        while True:
            # Java: if (sq(x) + sq(y) < sq(n - 1)) x++;
            if x * x + y * y < nm1_2:
                x += 1

            prev_x = x

            # Java: while (sq(x + 1) + sq(y) < sq(n)) x++;
            # Find max x where (x+1)^2 + y^2 < n^2
            # (x+1)^2 < n^2 - y^2
            # x+1 <= isqrt(n^2 - y^2 - 1)  (if n^2 - y^2 >= 2)
            # x <= isqrt(n^2 - y^2 - 1) - 1
            gap = n2 - y * y
            if gap >= 2:
                max_xp1 = isqrt(gap - 1)  # largest x+1 with (x+1)^2 < gap
                candidate = max_xp1 - 1
                if candidate > x:
                    x = candidate
                # isqrt gives exact result, no fine-tuning needed

            # Java: y--;
            y -= 1

            # Java: boolean rightBorder = sq(x) + sq(y) >= sq(n - 1);
            right_border = (x * x + y * y >= nm1_2)

            width = x - prev_x + 1
            odd_parity = width % 2

            # Java: numOddRows += x - prevX - 1 + (leftBorder ? 0 : 1) + (rightBorder ? 0 : 1) + (x - prevX + 1) % 2;
            num_odd_rows += (x - prev_x - 1 +
                           (0 if left_border else 1) +
                           (0 if right_border else 1) +
                           odd_parity)

            # Java: correction += (x - prevX - 1 + (leftBorder ? -1 : 1) + (rightBorder ? -1 : 1)) * ((x - prevX + 1) % 2 == 1 ? 2 : -2);
            correction += ((x - prev_x - 1 +
                          (-1 if left_border else 1) +
                          (-1 if right_border else 1)) *
                         (2 if odd_parity == 1 else -2))

            if y <= x:
                if y == x:
                    if x * x + y * y >= nm1_2:
                        correction += 1
                else:
                    correction -= 1
                    if not left_border and not right_border:
                        num_odd_rows -= 1
                break

            left_border = right_border

        return 2 * num_odd_rows * (n - num_odd_rows) + correction

    ans = 0
    for i in range(3, MAX_I + 1):
        n = (1 << i) - i
        if n == 5:
            ans += 3
        elif n % 2 == 0:
            ans += T(n)

    return ans

if __name__ == "__main__":
    print(solve())
