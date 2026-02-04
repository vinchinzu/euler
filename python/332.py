"""Project Euler Problem 332 - Spherical triangles.

Find sum_{r=1}^{50} A(r), where A(r) is the area of the smallest
non-degenerate spherical triangle with vertices on lattice points
of the sphere with radius r.

Uses L'Huilier's theorem for spherical excess.
"""
import math

def solve():
    N = 50

    def A(r):
        r2 = r * r
        # Generate lattice points on sphere x^2+y^2+z^2 = r^2
        # Only need half (x >= 0) since we check all triples
        points = []
        for x in range(r + 1):
            for y in range(-r, r + 1):
                for z in range(-r, r + 1):
                    if x * x + y * y + z * z == r2:
                        points.append((x, y, z))

        n = len(points)
        if n < 3:
            return 0.0

        # Precompute pairwise angles
        angles = [[0.0] * n for _ in range(n)]
        for i in range(n):
            ax, ay, az = points[i]
            for j in range(n):
                bx, by, bz = points[j]
                dot = ax * bx + ay * by + az * bz
                cos_val = dot / r2
                # Clamp to [-1, 1] for numerical safety
                cos_val = max(-1.0, min(1.0, cos_val))
                angles[i][j] = math.acos(cos_val)

        min_area = float('inf')
        for i in range(n):
            ax, ay, az = points[i]
            for j in range(i + 1, n):
                bx, by, bz = points[j]
                for k in range(j + 1, n):
                    cx, cy, cz = points[k]
                    # Check non-degeneracy: cross product A x B dot C != 0
                    # A x B = (ay*bz - az*by, az*bx - ax*bz, ax*by - ay*bx)
                    cross_x = ay * bz - az * by
                    cross_y = az * bx - ax * bz
                    cross_z = ax * by - ay * bx
                    det = cross_x * cx + cross_y * cy + cross_z * cz
                    if det == 0:
                        continue

                    a = angles[i][j]
                    b = angles[i][k]
                    c = angles[j][k]
                    s = (a + b + c) / 2.0

                    # L'Huilier's theorem
                    val = (math.tan(s / 2) * math.tan((s - a) / 2) *
                           math.tan((s - b) / 2) * math.tan((s - c) / 2))
                    if val < 0:
                        val = 0.0
                    E = 4 * math.atan(math.sqrt(val))
                    area = r2 * E

                    if area < min_area:
                        min_area = area

        return min_area if min_area != float('inf') else 0.0

    total = 0.0
    for r in range(1, N + 1):
        total += A(r)

    return f"{total:.6f}"

if __name__ == "__main__":
    print(solve())
