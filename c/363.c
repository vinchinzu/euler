/*
 * Project Euler Problem 363: Bezier Curves
 *
 * Approximate a quarter circle using a cubic Bezier curve with control
 * points P0=(1,0), P1=(1,v), P2=(v,1), P3=(0,1). The parameter v is
 * chosen so that the area enclosed equals pi/4.
 *
 * Compute by how many percent the Bezier curve length differs from
 * the length of the quarter circle arc (pi/2).
 */

#include <stdio.h>
#include <math.h>

static const double PI = 3.14159265358979323846264338327950288419716939937510;

static double bezier_x(double t, double v) {
    double u = 1.0 - t;
    return u*u*u * 1.0 + 3.0*u*u*t * 1.0 + 3.0*u*t*t * v + t*t*t * 0.0;
}

static double bezier_y(double t, double v) {
    double u = 1.0 - t;
    return u*u*u * 0.0 + 3.0*u*u*t * v + 3.0*u*t*t * 1.0 + t*t*t * 1.0;
}

static double bezier_dx(double t, double v) {
    double u = 1.0 - t;
    return 6.0*u*t*(v - 1.0) - 3.0*v*t*t;
}

static double bezier_dy(double t, double v) {
    double u = 1.0 - t;
    return 3.0*v*u*u + 6.0*(1.0 - v)*u*t;
}

/* Adaptive Simpson's rule */
static double adaptive_simpson(double (*f)(double, double), double v,
                               double a, double b, double eps,
                               double whole, double fa, double fb, double fc,
                               int depth, int max_depth) {
    if (depth > max_depth) return whole;
    double c = (a + b) / 2.0;
    double h = b - a;
    double d = (a + c) / 2.0;
    double e = (c + b) / 2.0;

    double fd = f(d, v);
    double fe = f(e, v);

    double left = (h / 12.0) * (fa + 4.0*fd + fc);
    double right = (h / 12.0) * (fc + 4.0*fe + fb);
    double total = left + right;

    if (fabs(total - whole) < 15.0 * eps)
        return total + (total - whole) / 15.0;

    return adaptive_simpson(f, v, a, c, eps/2.0, left, fa, fc, fd, depth+1, max_depth) +
           adaptive_simpson(f, v, c, b, eps/2.0, right, fc, fb, fe, depth+1, max_depth);
}

static double area_integrand(double t, double v) {
    return bezier_x(t, v) * bezier_dy(t, v);
}

static double speed_func(double t, double v) {
    double dx = bezier_dx(t, v);
    double dy = bezier_dy(t, v);
    return sqrt(dx*dx + dy*dy);
}

static double integrate(double (*f)(double, double), double v, double eps) {
    double fa = f(0.0, v);
    double fb = f(1.0, v);
    double fc = f(0.5, v);
    double whole = (1.0 / 6.0) * (fa + 4.0*fc + fb);
    return adaptive_simpson(f, v, 0.0, 1.0, eps, whole, fa, fb, fc, 0, 30);
}

static double compute_area(double v) {
    return integrate(area_integrand, v, 1e-16);
}

static double arc_length(double v) {
    return integrate(speed_func, v, 1e-16);
}

int main(void) {
    double target = PI / 4.0;
    double low = 0.1, high = 2.0;

    /* Binary search for v */
    for (int i = 0; i < 200; i++) {
        double mid = (low + high) / 2.0;
        double area = compute_area(mid);
        if (area < target) low = mid;
        else high = mid;
    }
    double v = (low + high) / 2.0;

    double L = arc_length(v);
    double quarter_arc = PI / 2.0;
    double pct = 100.0 * (L - quarter_arc) / quarter_arc;

    printf("%.10f\n", pct);
    return 0;
}
