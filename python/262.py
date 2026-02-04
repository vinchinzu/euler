"""Project Euler Problem 262: Mountain Range.

A mosquito flies from A(200,200) to B(1400,1400) at constant elevation f_min,
the minimum elevation allowing the trip within [0,1600]^2.

f_min equals the maximum of H along the domain boundary (specifically max H(0,y)
or equivalently max H(x,0) by symmetry), since the obstacle {H > f} must detach
from the boundary for A and B to be connected in the flyable region.

The shortest path at elevation f_min consists of:
  1. Straight line from A to tangent point T1 on the obstacle contour H=f_min
  2. Arc along the contour from T1 to T2
  3. Straight line from T2 to B
"""

import numpy as np
from scipy.optimize import minimize_scalar


def H_scalar(x, y):
    """Height function (scalar version)."""
    from math import exp
    return (
        (5000 - 0.005 * (x * x + y * y + x * y) + 12.5 * (x + y))
        * exp(-abs(0.000001 * (x * x + y * y) - 0.0015 * (x + y) + 0.7))
    )


def H_grid(X, Y):
    """Height function (vectorized numpy version)."""
    return (
        (5000 - 0.005 * (X * X + Y * Y + X * Y) + 12.5 * (X + Y))
        * np.exp(-np.abs(0.000001 * (X * X + Y * Y) - 0.0015 * (X + Y) + 0.7))
    )


def solve():
    # Find f_min = max of H(0, y) for y in [0, 1600]
    res = minimize_scalar(
        lambda y: -H_scalar(0, y), bounds=(800, 1000), method="bounded",
        options={"xatol": 1e-14}
    )
    f_min = -res.fun

    # Trace the contour H(x,y) = f_min using matplotlib
    import matplotlib
    matplotlib.use("Agg")
    import matplotlib.pyplot as plt

    N = 4000
    xs = np.linspace(0, 1600, N)
    ys = np.linspace(0, 1600, N)
    X, Y = np.meshgrid(xs, ys)
    Hvals = H_grid(X, Y)

    fig, ax = plt.subplots()
    cs = ax.contour(X, Y, Hvals, levels=[f_min])
    outer = cs.allsegs[0][0]
    plt.close()

    # Split contour at y=x crossings into lower (x>y) and upper (y>x) halves
    diffs = outer[:, 1] - outer[:, 0]
    crossings = np.where(np.diff(np.sign(diffs)))[0]
    c1, c2 = crossings[0], crossings[1]

    # Lower contour: from crossing near (273,273) through x>y region to (1293,1293)
    lower_indices = list(range(c2, len(outer))) + list(range(0, c1 + 1))
    lower_contour = outer[lower_indices]

    A = np.array([200.0, 200.0])
    B = np.array([1400.0, 1400.0])

    # Compute tangent directions along the contour
    tangents = np.zeros_like(lower_contour)
    tangents[1:-1] = (lower_contour[2:] - lower_contour[:-2]) / 2
    tangents[0] = lower_contour[1] - lower_contour[0]
    tangents[-1] = lower_contour[-1] - lower_contour[-2]

    # Find tangent points: where (P - Source) is parallel to contour tangent
    # Cross product (P-A) x tangent = 0
    vec_A = lower_contour - A
    cross_A = vec_A[:, 0] * tangents[:, 1] - vec_A[:, 1] * tangents[:, 0]
    vec_B = lower_contour - B
    cross_B = vec_B[:, 0] * tangents[:, 1] - vec_B[:, 1] * tangents[:, 0]

    T1_idx = np.where(np.diff(np.sign(cross_A)))[0][0]
    T2_idx = np.where(np.diff(np.sign(cross_B)))[0][0]

    T1 = lower_contour[T1_idx]
    T2 = lower_contour[T2_idx]

    # Path = straight A->T1 + arc T1->T2 along contour + straight T2->B
    d_A_T1 = np.linalg.norm(T1 - A)
    d_T2_B = np.linalg.norm(B - T2)

    arc_segment = lower_contour[T1_idx : T2_idx + 1]
    arc_diffs = np.diff(arc_segment, axis=0)
    arc_length = np.sqrt(arc_diffs[:, 0] ** 2 + arc_diffs[:, 1] ** 2).sum()

    return d_A_T1 + arc_length + d_T2_B


if __name__ == "__main__":
    print(f"{solve():.3f}")
