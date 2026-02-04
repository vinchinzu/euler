"""Project Euler Problem 476: Circle Packing II.

R(a,b,c) = maximum area of 3 non-overlapping circles inside triangle with sides a,b,c.
S(n) = average R over all 1 <= a <= b <= c < a+b <= n.

Algorithm (greedy, proven optimal by Zalgaller & Los 1994):
1. Place incircle (radius r = area/s where s is semi-perimeter).
2. For each corner (angle alpha), a corner circle tangent to 2 sides and incircle
   has radius rho = r*(1-sin(alpha/2))/(1+sin(alpha/2)).
3. Pick the largest corner circle as the 2nd circle.
4. For the 3rd circle, choose the maximum among:
   - Other corner circles
   - Corner-of-corner: tangent to 2 sides and the 1st corner circle
     (same formula with rho instead of r)
   - Soddy gap: tangent to incircle, 1st corner circle, and one side
     (Descartes: k3 = k1+k2+2*sqrt(k1*k2), r3=1/k3)
"""

import subprocess, tempfile, os

def solve():
    c_code = r"""
#include <stdio.h>
#include <stdlib.h>
#include <math.h>

static double corner_radius(double angle, double r) {
    double sh = sin(angle / 2.0);
    return r * (1.0 - sh) / (1.0 + sh);
}

int main(void) {
    const int N = 1803;
    double total_area = 0.0;
    int count = 0;

    for (int a = 1; a <= N; a++) {
        for (int b = a; b <= N; b++) {
            if (a + b > N) break;
            for (int c = b; c < a + b; c++) {
                double s = (a + b + c) / 2.0;
                double area = sqrt(s * (s-a) * (s-b) * (s-c));
                if (area < 1e-12) continue;
                double r = area / s;

                double ab = (double)a*a, bb = (double)b*b, cb = (double)c*c;
                double cosA = (bb + cb - ab) / (2.0*b*c);
                double cosB = (ab + cb - bb) / (2.0*a*c);
                double cosC = (ab + bb - cb) / (2.0*a*b);
                if (cosA > 1) cosA = 1; if (cosA < -1) cosA = -1;
                if (cosB > 1) cosB = 1; if (cosB < -1) cosB = -1;
                if (cosC > 1) cosC = 1; if (cosC < -1) cosC = -1;
                double A = acos(cosA), B = acos(cosB), C = acos(cosC);

                double rA = corner_radius(A, r);
                double rB = corner_radius(B, r);
                double rC = corner_radius(C, r);

                /* Find the largest corner circle (2nd circle) */
                double first_r, first_angle;
                int first_idx;
                double corners[3] = {rA, rB, rC};
                double angles[3] = {A, B, C};
                first_idx = 0;
                if (corners[1] > corners[0]) first_idx = 1;
                if (corners[2] > corners[first_idx]) first_idx = 2;
                first_r = corners[first_idx];
                first_angle = angles[first_idx];

                /* Find the best 3rd circle */
                double best_third = 0.0;

                /* Other corner circles */
                for (int i = 0; i < 3; i++) {
                    if (i != first_idx && corners[i] > best_third)
                        best_third = corners[i];
                }

                /* Corner of first corner */
                double coc = corner_radius(first_angle, first_r);
                if (coc > best_third) best_third = coc;

                /* Soddy gap: tangent to incircle, first corner, and one side */
                double k1 = 1.0/r, k2 = 1.0/first_r;
                double k3 = k1 + k2 + 2.0 * sqrt(k1 * k2);
                double soddy = 1.0 / k3;
                if (soddy > best_third) best_third = soddy;

                total_area += M_PI * (r*r + first_r*first_r + best_third*best_third);
                count++;
            }
        }
    }

    printf("%.5f\n", total_area / count);
    return 0;
}
"""
    with tempfile.NamedTemporaryFile(suffix='.c', mode='w', delete=False) as src:
        src.write(c_code)
        src_path = src.name
    bin_path = src_path.replace('.c', '')
    try:
        subprocess.run(['gcc', '-O2', '-o', bin_path, src_path, '-lm'], check=True,
                       capture_output=True, text=True)
        result = subprocess.run([bin_path], capture_output=True, text=True, check=True, timeout=30)
        print(result.stdout.strip())
    except subprocess.CalledProcessError as e:
        print(f"Error: {e.stderr}", flush=True)
        raise
    finally:
        os.unlink(src_path)
        if os.path.exists(bin_path):
            os.unlink(bin_path)

if __name__ == "__main__":
    solve()
