"""Project Euler Problem 144: Laser Beam in Elliptical White Cell.

In laser physics, a "white cell" is a mirror system that acts as a delay line for the laser beam.
The beam enters the cell, bounces around on the mirrors, and eventually works its way back out.

The specific white cell we will be considering is an ellipse with the equation 4x² + y² = 100.
The section corresponding to -0.01 ≤ x ≤ +0.01 at the top is missing, allowing the light to
enter and exit through the hole.

The light beam in this problem starts at the point (0.0, 10.1) just outside the white cell,
and the beam first impacts the mirror at (1.4, -9.6).

Each time the laser beam hits the surface of the ellipse, it follows the usual law of reflection
"angle of incidence equals angle of reflection." That is, both the incident and reflected beams
make the same angle with the normal line at the point of incidence.

The slope m of the tangent line at any point (x,y) of the given ellipse is: m = -4x/y.
The normal line is perpendicular to this tangent line at the point of incidence.

How many times does the beam hit the internal surface of the white cell before exiting?
"""

import math
from typing import Tuple

# Configuration constants
ELLIPSE_A = 2.0     # sqrt(100/4) = 2
ELLIPSE_B = 10.0    # sqrt(100) = 10

# Starting conditions
START_X = 0.0
START_Y = 10.1
FIRST_HIT_X = 1.4
FIRST_HIT_Y = -9.6

# Exit condition: beam exits through hole at top where |x| < 0.01 and y > 9.9
EXIT_X_THRESHOLD = 0.01
EXIT_Y_THRESHOLD = 9.9

# Floating point precision
EPS = 1e-10         # High precision for robust numerical stability
MAX_BOUNCES = 10000  # Safety limit to prevent infinite loops


def on_ellipse(x: float, y: float) -> bool:
    """Check if a point lies on the ellipse surface (within tolerance)."""
    ellipse_value = 4 * x * x + y * y - 100
    return abs(ellipse_value) < EPS


def find_intersection(x0: float, y0: float, dx: float, dy: float) -> Tuple[float, float]:
    """Find intersection point of line with ellipse."""
    # Parametric line: x = x0 + t*dx, y = y0 + t*dy
    # Substitute into ellipse: 4(x0 + t*dx)^2 + (y0 + t*dy)^2 = 100
    # Expand: 4(x0^2 + 2*x0*t*dx + t^2*dx^2) + (y0^2 + 2*y0*t*dy + t^2*dy^2) = 100
    # 4*x0^2 + 8*x0*t*dx + 4*t^2*dx^2 + y0^2 + 2*y0*t*dy + t^2*dy^2 = 100
    # t^2*(4*dx^2 + dy^2) + t*(8*x0*dx + 2*y0*dy) + (4*x0^2 + y0^2 - 100) = 0

    a_coeff = 4 * dx * dx + dy * dy
    b_coeff = 8 * x0 * dx + 2 * y0 * dy
    c_coeff = 4 * x0 * x0 + y0 * y0 - 100

    discriminant = b_coeff * b_coeff - 4 * a_coeff * c_coeff
    if discriminant < 0:
        raise ValueError("No intersection found")

    sqrt_disc = math.sqrt(discriminant)
    t1 = (-b_coeff - sqrt_disc) / (2 * a_coeff)
    t2 = (-b_coeff + sqrt_disc) / (2 * a_coeff)

    # Choose the t that's not at the starting point (larger absolute value)
    t = t1 if abs(t1) > abs(t2) else t2

    x = x0 + t * dx
    y = y0 + t * dy
    return (x, y)


def reflect(x: float, y: float, dx: float, dy: float) -> Tuple[float, float]:
    """Compute reflected direction vector."""
    # Slope of tangent: m_tangent = -4*x/y
    # Slope of normal: m_normal = y/(4*x) (perpendicular to tangent)
    # Normal vector: (4*x, y) normalized

    # Normal vector (not normalized)
    nx = 4 * x
    ny = y

    # Normalize normal vector
    norm_len = math.sqrt(nx * nx + ny * ny)
    nx /= norm_len
    ny /= norm_len

    # Incident direction vector (normalized)
    dir_len = math.sqrt(dx * dx + dy * dy)
    dx /= dir_len
    dy /= dir_len

    # Reflect: r = d - 2*(d·n)*n
    dot = dx * nx + dy * ny
    rx = dx - 2 * dot * nx
    ry = dy - 2 * dot * ny

    return (rx, ry)


def main() -> int:
    """Main function."""
    x, y = FIRST_HIT_X, FIRST_HIT_Y
    dx = FIRST_HIT_X - START_X
    dy = FIRST_HIT_Y - START_Y

    bounces = 1  # First hit is already counted

    while bounces < MAX_BOUNCES:
        # Reflect at current point
        dx, dy = reflect(x, y, dx, dy)

        # Find next intersection
        x, y = find_intersection(x, y, dx, dy)

        # Check exit condition AFTER finding next intersection
        if abs(x) < EXIT_X_THRESHOLD and y > EXIT_Y_THRESHOLD:
            break

        bounces += 1

    return bounces


if __name__ == "__main__":
    print(main())
