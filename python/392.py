"""Project Euler Problem 392 - Enmeshed Unit Circle.

Place N inner gridlines per axis in [-1,1]x[-1,1] to minimize the total area
of cells overlapping with the unit circle.

By 8-fold symmetry, we optimize gridlines in the first octant.
The optimal positions satisfy a recurrence derived from dA/dx_i = 0.
Binary search on x_1 to make x_{N/2+1} = 1.
"""

import math

def solve(N=400):
    half = N // 2 + 1  # number of grid intervals in [0,1] on each axis

    lo = 1e-15
    hi = 1.0 - 1e-15

    for _ in range(200):  # binary search iterations
        mid = (lo + hi) / 2.0
        x_prev2 = 0.0
        x_prev1 = mid
        area = x_prev1  # first strip: width=x_1, height=sqrt(1-0^2)=1

        ok = True
        for i in range(2, half + 1):
            s1 = math.sqrt(1.0 - x_prev1 * x_prev1)
            s2 = math.sqrt(1.0 - x_prev2 * x_prev2)
            x_new = x_prev1 - (s1 - s2) * s1 / x_prev1
            if x_new > 1.0:
                ok = False
                break
            area += (x_new - x_prev1) * s1
            x_prev2 = x_prev1
            x_prev1 = x_new

        if not ok or x_prev1 > 1.0:
            hi = mid
        else:
            lo = mid

    # Final computation with converged value
    mid = (lo + hi) / 2.0
    x_prev2 = 0.0
    x_prev1 = mid
    area = x_prev1
    for i in range(2, half + 1):
        s1 = math.sqrt(1.0 - x_prev1 * x_prev1)
        s2 = math.sqrt(1.0 - x_prev2 * x_prev2)
        x_new = x_prev1 - (s1 - s2) * s1 / x_prev1
        area += (x_new - x_prev1) * s1
        x_prev2 = x_prev1
        x_prev1 = x_new

    return 4.0 * area

if __name__ == "__main__":
    result = solve(400)
    print(f"{result:.10f}")
