/*
 * Project Euler 262: Mountain Range
 *
 * A mosquito flies from A(200,200) to B(1400,1400) at constant elevation f_min.
 * f_min = max H(0,y) for y in [0,1600].
 * Shortest path = straight A->T1 + arc along contour H=f_min + straight T2->B.
 *
 * Pure C numerical approach: golden-section search for f_min, then trace contour
 * using marching, find tangent points, compute path length.
 */
#include <stdio.h>
#include <math.h>

static double H(double x, double y) {
    return (5000.0 - 0.005 * (x*x + y*y + x*y) + 12.5 * (x + y))
         * exp(-fabs(0.000001 * (x*x + y*y) - 0.0015 * (x + y) + 0.7));
}

/* Find max H(0, y) using golden section search */
static double find_fmin(void) {
    double a = 0.0, b = 1600.0;
    double gr = (sqrt(5.0) + 1.0) / 2.0;
    for (int i = 0; i < 200; i++) {
        double c = b - (b - a) / gr;
        double d = a + (b - a) / gr;
        if (H(0, c) > H(0, d))
            b = d;
        else
            a = c;
    }
    return H(0, (a + b) / 2.0);
}

/* Trace contour H(x,y) = fmin in the lower half (x > y region) */
/* Use parametric marching from the diagonal crossing */

#define MAX_CONTOUR 200000

static double cx[MAX_CONTOUR], cy[MAX_CONTOUR];
static int contour_len = 0;

/* Gradient of H */
static void grad_H(double x, double y, double *gx, double *gy) {
    double eps = 1e-6;
    *gx = (H(x + eps, y) - H(x - eps, y)) / (2.0 * eps);
    *gy = (H(x, y + eps) - H(x, y - eps)) / (2.0 * eps);
}

/* Find point on contour H=fmin near (x0,y0) by Newton on H(x,y0)=fmin */
static double find_x_on_contour(double x0, double y, double fmin) {
    double x = x0;
    for (int i = 0; i < 100; i++) {
        double val = H(x, y) - fmin;
        double gx, gy;
        grad_H(x, y, &gx, &gy);
        if (fabs(gx) < 1e-15) break;
        x -= val / gx;
    }
    return x;
}

/* Find y on contour at given x */
static double find_y_on_contour(double x, double y0, double fmin) {
    double y = y0;
    for (int i = 0; i < 100; i++) {
        double val = H(x, y) - fmin;
        double gx, gy;
        grad_H(x, y, &gx, &gy);
        if (fabs(gy) < 1e-15) break;
        y -= val / gy;
    }
    return y;
}

/* Trace contour in direction perpendicular to gradient */
static void trace_contour(double fmin) {
    /* Start near diagonal crossing point around (273, 273) */
    /* Find where H(t, t) = fmin for small t */
    double t0 = 200.0, t1 = 400.0;
    for (int i = 0; i < 100; i++) {
        double tm = (t0 + t1) / 2.0;
        if (H(tm, tm) > fmin)
            t0 = tm;
        else
            t1 = tm;
    }
    double start_t = (t0 + t1) / 2.0;

    /* Start slightly below diagonal (x > y) */
    double px = start_t + 0.1;
    double py = find_y_on_contour(px, start_t - 0.1, fmin);

    double ds = 0.5; /* step size */
    contour_len = 0;

    cx[0] = px; cy[0] = py;
    contour_len = 1;

    /* March along contour in x > y region */
    for (int step = 0; step < MAX_CONTOUR - 1; step++) {
        double gx, gy;
        grad_H(px, py, &gx, &gy);
        double gn = sqrt(gx * gx + gy * gy);
        if (gn < 1e-15) break;

        /* Tangent direction (perpendicular to gradient, going towards larger x) */
        double tx = -gy / gn;
        double ty = gx / gn;

        /* We want to go from small x to large x */
        if (tx < 0) { tx = -tx; ty = -ty; }

        double nx = px + ds * tx;
        double ny = py + ds * ty;

        /* Project back onto contour */
        ny = find_y_on_contour(nx, ny, fmin);

        /* Check if we've crossed the diagonal on the far side */
        if (ny > nx) {
            /* We've passed the second diagonal crossing, interpolate */
            /* Binary search for the crossing point */
            double ax = px, ay = py;
            double bx = nx, by = ny;
            for (int i = 0; i < 60; i++) {
                double mx = (ax + bx) / 2.0;
                double my = (ay + by) / 2.0;
                my = find_y_on_contour(mx, my, fmin);
                if (my < mx) {
                    ax = mx; ay = my;
                } else {
                    bx = mx; by = my;
                }
            }
            cx[contour_len] = (ax + bx) / 2.0;
            cy[contour_len] = (ay + by) / 2.0;
            contour_len++;
            break;
        }

        px = nx; py = ny;
        cx[contour_len] = px;
        cy[contour_len] = py;
        contour_len++;
    }
}

int main(void) {
    double fmin = find_fmin();

    trace_contour(fmin);

    double Ax = 200.0, Ay = 200.0;
    double Bx = 1400.0, By = 1400.0;

    /* Find tangent points T1 (from A) and T2 (to B) on the contour */
    /* T1: point where line A->P is tangent to contour
       Cross product (P - A) x tangent = 0 */

    int T1_idx = -1, T2_idx = -1;

    for (int i = 1; i < contour_len - 1; i++) {
        /* Tangent at i */
        double tx = cx[i+1] - cx[i-1];
        double ty = cy[i+1] - cy[i-1];

        /* Vector from A to point i */
        double vx = cx[i] - Ax;
        double vy = cy[i] - Ay;
        double cross_a = vx * ty - vy * tx;

        /* Same for i-1 */
        double vx_prev = cx[i-1] - Ax;
        double vy_prev = cy[i-1] - Ay;
        double tx_prev, ty_prev;
        if (i >= 2) {
            tx_prev = cx[i] - cx[i-2];
            ty_prev = cy[i] - cy[i-2];
        } else {
            tx_prev = cx[1] - cx[0];
            ty_prev = cy[1] - cy[0];
        }
        double cross_a_prev = vx_prev * ty_prev - vy_prev * tx_prev;

        if (T1_idx < 0 && cross_a_prev * cross_a <= 0 && (fabs(cross_a_prev) + fabs(cross_a)) > 0) {
            T1_idx = i;
        }

        /* Vector from B to point i */
        vx = cx[i] - Bx;
        vy = cy[i] - By;
        double cross_b = vx * ty - vy * tx;

        vx_prev = cx[i-1] - Bx;
        vy_prev = cy[i-1] - By;
        double cross_b_prev = vx_prev * ty_prev - vy_prev * tx_prev;

        if (T1_idx >= 0 && T2_idx < 0 && cross_b_prev * cross_b <= 0 && (fabs(cross_b_prev) + fabs(cross_b)) > 0) {
            T2_idx = i;
        }
    }

    if (T1_idx < 0 || T2_idx < 0) {
        /* Fallback: use finer trace */
        fprintf(stderr, "Error: tangent points not found\n");
        return 1;
    }

    /* Path = A->T1 + arc T1->T2 + T2->B */
    double d_A_T1 = sqrt((cx[T1_idx]-Ax)*(cx[T1_idx]-Ax) + (cy[T1_idx]-Ay)*(cy[T1_idx]-Ay));
    double d_T2_B = sqrt((Bx-cx[T2_idx])*(Bx-cx[T2_idx]) + (By-cy[T2_idx])*(By-cy[T2_idx]));

    double arc = 0.0;
    for (int i = T1_idx; i < T2_idx; i++) {
        double dx = cx[i+1] - cx[i];
        double dy = cy[i+1] - cy[i];
        arc += sqrt(dx*dx + dy*dy);
    }

    double total = d_A_T1 + arc + d_T2_B;
    printf("%.3f\n", total);
    return 0;
}
