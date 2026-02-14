/*
 * Project Euler 262: Mountain Range
 *
 * A mosquito flies from A(200,200) to B(1400,1400) at constant elevation f_min.
 * f_min = max H(0,y) for y in [0,1600].
 * Shortest path = straight A->T1 + arc along contour H=f_min + straight T2->B.
 *
 * The contour H=f_min in the x>y region consists of an outer boundary
 * (close to the edges) and possibly inner boundaries near the diagonal.
 * The outer boundary is what the mosquito must fly around.
 *
 * We trace the outer contour from one diagonal crossing (~273,273) to the
 * other (~1293,1293), following the boundary that goes away from the diagonal
 * (toward small y / large x).
 */
#include <stdio.h>
#include <math.h>

static double H(double x, double y) {
    return (5000.0 - 0.005 * (x*x + y*y + x*y) + 12.5 * (x + y))
         * exp(-fabs(0.000001 * (x*x + y*y) - 0.0015 * (x + y) + 0.7));
}

static void grad_H(double x, double y, double *gx, double *gy) {
    double eps = 1e-7;
    *gx = (H(x + eps, y) - H(x - eps, y)) / (2.0 * eps);
    *gy = (H(x, y + eps) - H(x, y - eps)) / (2.0 * eps);
}

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

static void project_to_contour(double *x, double *y, double fmin) {
    for (int i = 0; i < 80; i++) {
        double val = H(*x, *y) - fmin;
        if (fabs(val) < 1e-14) break;
        double gx, gy;
        grad_H(*x, *y, &gx, &gy);
        double g2 = gx * gx + gy * gy;
        if (g2 < 1e-30) break;
        double t = val / g2;
        *x -= t * gx;
        *y -= t * gy;
    }
}

/* Find y on contour H=fmin at given x, starting from y0 */
static double find_y_on_contour(double x, double y0, double fmin) {
    double y = y0;
    for (int i = 0; i < 100; i++) {
        double val = H(x, y) - fmin;
        if (fabs(val) < 1e-14) break;
        double gx, gy;
        grad_H(x, y, &gx, &gy);
        if (fabs(gy) < 1e-15) break;
        y -= val / gy;
    }
    return y;
}

#define MAX_CONTOUR 500000

static double cx[MAX_CONTOUR], cy[MAX_CONTOUR];
static int contour_len = 0;

static void trace_contour(double fmin) {
    /* Find first diagonal crossing: H < fmin -> H > fmin transition near t~273 */
    double t0 = 200.0, t1 = 300.0;
    /* Ensure t0 is below fmin and t1 is above fmin */
    while (H(t1, t1) < fmin) t1 += 10;
    while (H(t0, t0) > fmin) t0 -= 10;
    for (int i = 0; i < 100; i++) {
        double tm = (t0 + t1) / 2.0;
        if (H(tm, tm) < fmin)
            t0 = tm;
        else
            t1 = tm;
    }
    double cross1 = (t0 + t1) / 2.0;

    /* Find last diagonal crossing: H > fmin -> H < fmin transition near t~1293 */
    t0 = 1200.0; t1 = 1400.0;
    while (H(t0, t0) < fmin) t0 -= 10;
    while (H(t1, t1) > fmin) t1 += 10;
    for (int i = 0; i < 100; i++) {
        double tm = (t0 + t1) / 2.0;
        if (H(tm, tm) > fmin)
            t0 = tm;
        else
            t1 = tm;
    }
    double cross2 = (t0 + t1) / 2.0;

    /* Start at the first diagonal crossing.
     * We want to trace the OUTER contour, which goes AWAY from the diagonal
     * (toward y=0 side). At the diagonal crossing, the gradient points inward
     * (toward the mountain peak). The outer contour tangent should go in the
     * direction that decreases y relative to the diagonal.
     */
    double px = cross1;
    double py = cross1;
    project_to_contour(&px, &py, fmin);

    double ds = 0.05; /* small step size for accuracy */
    contour_len = 0;
    cx[0] = px; cy[0] = py;
    contour_len = 1;

    /* At the diagonal crossing, choose direction that goes toward smaller y
     * (outer contour goes away from diagonal toward y=0 edge) */
    double gx, gy;
    grad_H(px, py, &gx, &gy);
    double gn = sqrt(gx * gx + gy * gy);

    /* Two tangent directions: (-gy, gx)/gn and (gy, -gx)/gn */
    /* Choose the one where dy < dx (moves away from diagonal downward) */
    double tx1 = -gy / gn, ty1 = gx / gn;
    double tx2 = gy / gn, ty2 = -gx / gn;

    double tx, ty;
    /* The outer contour goes toward smaller y, so ty < tx (moving below diagonal) */
    if (ty1 - tx1 < ty2 - tx2) {
        tx = tx1; ty = ty1;
    } else {
        tx = tx2; ty = ty2;
    }

    /* Take initial step */
    double nx = px + ds * tx;
    double ny = py + ds * ty;
    project_to_contour(&nx, &ny, fmin);
    px = nx; py = ny;
    cx[contour_len] = px; cy[contour_len] = py;
    contour_len++;

    /* Now march along the outer contour.
     * At each step, the tangent direction is perpendicular to gradient.
     * Choose direction that maintains continuity (dot product with previous tangent > 0).
     */
    double prev_tx = tx, prev_ty = ty;

    for (int step = 0; step < MAX_CONTOUR - 2; step++) {
        grad_H(px, py, &gx, &gy);
        gn = sqrt(gx * gx + gy * gy);
        if (gn < 1e-15) break;

        tx1 = -gy / gn; ty1 = gx / gn;
        tx2 = gy / gn; ty2 = -gx / gn;

        /* Choose direction consistent with previous */
        double dot1 = tx1 * prev_tx + ty1 * prev_ty;
        double dot2 = tx2 * prev_tx + ty2 * prev_ty;
        if (dot1 >= dot2) {
            tx = tx1; ty = ty1;
        } else {
            tx = tx2; ty = ty2;
        }

        nx = px + ds * tx;
        ny = py + ds * ty;
        project_to_contour(&nx, &ny, fmin);

        /* Check if we've reached the second diagonal crossing */
        if (nx > cross2 - 5 && ny > cross2 - 5 && fabs(nx - ny) < 5) {
            /* Close to second crossing, binary search for exact point */
            double ax = px, ay = py;
            double bx = nx, by = ny;
            for (int i = 0; i < 80; i++) {
                double mx = (ax + bx) / 2.0;
                double my = (ay + by) / 2.0;
                project_to_contour(&mx, &my, fmin);
                /* Still in x>y region? */
                if (mx > my + 0.001) {
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
        prev_tx = tx; prev_ty = ty;
        cx[contour_len] = px;
        cy[contour_len] = py;
        contour_len++;
    }
}

int main(void) {
    double fmin = find_fmin();

    trace_contour(fmin);

    if (contour_len < 10) {
        fprintf(stderr, "Error: contour too short (%d points)\n", contour_len);
        return 1;
    }

    double Ax = 200.0, Ay = 200.0;
    double Bx = 1400.0, By = 1400.0;

    /* Find tangent points on contour.
     * T1: from A, the tangent to the outer contour (first sign change of cross product)
     * T2: from B, the tangent to the outer contour (last sign change of cross product going from T1 to end)
     */
    int T1_idx = -1, T2_idx = -1;

    double prev_cross_a = 0, prev_cross_b = 0;
    int first_a = 1, first_b = 1;

    for (int i = 1; i < contour_len - 1; i++) {
        double tanx = cx[i+1] - cx[i-1];
        double tany = cy[i+1] - cy[i-1];

        double vax = cx[i] - Ax, vay = cy[i] - Ay;
        double cross_a = vax * tany - vay * tanx;

        double vbx = cx[i] - Bx, vby = cy[i] - By;
        double cross_b = vbx * tany - vby * tanx;

        if (!first_a) {
            if (T1_idx < 0 && prev_cross_a * cross_a < 0) {
                T1_idx = i;
            }
        }
        if (!first_b && T1_idx >= 0) {
            if (T2_idx < 0 && prev_cross_b * cross_b < 0) {
                T2_idx = i;
            }
        }

        prev_cross_a = cross_a;
        prev_cross_b = cross_b;
        first_a = 0;
        first_b = 0;
    }

    if (T1_idx < 0 || T2_idx < 0) {
        fprintf(stderr, "Error: tangent points not found (T1=%d, T2=%d, len=%d)\n",
                T1_idx, T2_idx, contour_len);
        fprintf(stderr, "First point: (%.2f, %.2f)\n", cx[0], cy[0]);
        fprintf(stderr, "Last point: (%.2f, %.2f)\n", cx[contour_len-1], cy[contour_len-1]);
        return 1;
    }

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
