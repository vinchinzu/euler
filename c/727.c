/* Project Euler 727: Triangle of Circles.
 * Compute distance DE between incenter and equal detour point using
 * barycentric coordinates, for all coprime triples (ra, rb, rc).
 */
#include <stdio.h>
#include <math.h>

int gcd(int a, int b) {
    while (b) { int t = b; b = a % b; a = t; }
    return a;
}

double de(int ra, int rb, int rc) {
    double a = rb + rc;
    double b = ra + rc;
    double c = ra + rb;
    double s = (a + b + c) / 2.0;
    double k = sqrt(s * (s - a) * (s - b) * (s - c));

    double da = a, db = b, dc = c;
    double d_norm = da + db + dc;

    double ea = a + k / (s - a);
    double eb = b + k / (s - b);
    double ec = c + k / (s - c);
    double e_norm = ea + eb + ec;

    double x = da / d_norm - ea / e_norm;
    double y = db / d_norm - eb / e_norm;
    double z = dc / d_norm - ec / e_norm;

    return sqrt(-(a*a*y*z + b*b*x*z + c*c*x*y));
}

int main() {
    int n = 100;
    int count = 0;
    double total = 0.0;

    for (int ra = 1; ra <= n; ra++) {
        for (int rb = ra + 1; rb <= n; rb++) {
            int g = gcd(ra, rb);
            for (int rc = rb + 1; rc <= n; rc++) {
                if (gcd(g, rc) == 1) {
                    count++;
                    total += de(ra, rb, rc);
                }
            }
        }
    }

    printf("%.8f\n", total / count);
    return 0;
}
