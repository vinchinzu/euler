"""Project Euler Problem 332 - Spherical triangles — Embedded C version.

Find sum_{r=1}^{50} A(r), where A(r) is the area of the smallest
non-degenerate spherical triangle with vertices on lattice points
of the sphere with radius r.

Uses L'Huilier's theorem for spherical excess.
"""

import subprocess, tempfile, os

C_CODE = r"""
#include <stdio.h>
#include <math.h>
#include <stdlib.h>

typedef struct { int x, y, z; } Point;

int main(void) {
    double total = 0.0;

    for (int r = 1; r <= 50; r++) {
        int r2 = r * r;

        // Count lattice points first
        int count = 0;
        for (int x = 0; x <= r; x++)
            for (int y = -r; y <= r; y++)
                for (int z = -r; z <= r; z++)
                    if (x*x + y*y + z*z == r2)
                        count++;

        if (count < 3) continue;

        Point *pts = (Point *)malloc(count * sizeof(Point));
        int idx = 0;
        for (int x = 0; x <= r; x++)
            for (int y = -r; y <= r; y++)
                for (int z = -r; z <= r; z++)
                    if (x*x + y*y + z*z == r2)
                        pts[idx++] = (Point){x, y, z};

        int n = count;

        // Precompute pairwise angles
        double *angles = (double *)malloc((long long)n * n * sizeof(double));
        for (int i = 0; i < n; i++) {
            for (int j = 0; j < n; j++) {
                double dot = (double)pts[i].x * pts[j].x +
                             (double)pts[i].y * pts[j].y +
                             (double)pts[i].z * pts[j].z;
                double cos_val = dot / r2;
                if (cos_val > 1.0) cos_val = 1.0;
                if (cos_val < -1.0) cos_val = -1.0;
                angles[i * n + j] = acos(cos_val);
            }
        }

        double min_area = 1e30;

        for (int i = 0; i < n; i++) {
            int ax = pts[i].x, ay = pts[i].y, az = pts[i].z;
            for (int j = i + 1; j < n; j++) {
                int bx = pts[j].x, by = pts[j].y, bz = pts[j].z;
                for (int k = j + 1; k < n; k++) {
                    int cx = pts[k].x, cy = pts[k].y, cz = pts[k].z;

                    // Cross product A x B dot C
                    long long cross_x = (long long)ay * bz - (long long)az * by;
                    long long cross_y = (long long)az * bx - (long long)ax * bz;
                    long long cross_z = (long long)ax * by - (long long)ay * bx;
                    long long det = cross_x * cx + cross_y * cy + cross_z * cz;
                    if (det == 0) continue;

                    double a = angles[i * n + j];
                    double b = angles[i * n + k];
                    double c = angles[j * n + k];
                    double s = (a + b + c) / 2.0;

                    double val = tan(s / 2.0) * tan((s - a) / 2.0) *
                                 tan((s - b) / 2.0) * tan((s - c) / 2.0);
                    if (val < 0.0) val = 0.0;
                    double E = 4.0 * atan(sqrt(val));
                    double area = (double)r2 * E;

                    if (area < min_area) min_area = area;
                }
            }
        }

        if (min_area < 1e29) total += min_area;

        free(angles);
        free(pts);
    }

    printf("%.6f\n", total);
    return 0;
}
"""

def solve():
    with tempfile.TemporaryDirectory() as tmpdir:
        src = os.path.join(tmpdir, "p332.c")
        exe = os.path.join(tmpdir, "p332")
        with open(src, "w") as f:
            f.write(C_CODE)
        subprocess.run(["gcc", "-O2", "-o", exe, src, "-lm"], check=True)
        result = subprocess.run([exe], capture_output=True, text=True, timeout=280)
        return result.stdout.strip()

if __name__ == "__main__":
    print(solve())
