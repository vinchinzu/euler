/*
 * Project Euler 144 - Laser beam bouncing in an elliptical white cell
 *
 * Ellipse: 4x^2 + y^2 = 100. Beam enters at (0, 10.1), first hits (1.4, -9.6).
 * Count bounces before beam exits through hole at top (|x| < 0.01).
 */
#include <stdio.h>
#include <math.h>

int main(void) {
    double x = 1.4, y = -9.6;
    double dx = 1.4 - 0.0, dy = -9.6 - 10.1;

    int bounces = 1;

    for (int iter = 0; iter < 10000; iter++) {
        /* Reflect at (x, y) */
        /* Normal to ellipse at (x,y): (4x, y) (gradient of 4x^2+y^2) */
        double nx = 4.0 * x, ny = y;
        double nlen = sqrt(nx*nx + ny*ny);
        nx /= nlen; ny /= nlen;

        double dlen = sqrt(dx*dx + dy*dy);
        dx /= dlen; dy /= dlen;

        double dot = dx*nx + dy*ny;
        double rx = dx - 2.0*dot*nx;
        double ry = dy - 2.0*dot*ny;
        dx = rx; dy = ry;

        /* Find next intersection with ellipse */
        /* Line: X = x + t*dx, Y = y + t*dy */
        /* 4(x+t*dx)^2 + (y+t*dy)^2 = 100 */
        double a = 4.0*dx*dx + dy*dy;
        double b = 8.0*x*dx + 2.0*y*dy;
        double c = 4.0*x*x + y*y - 100.0;

        double disc = b*b - 4.0*a*c;
        double sq = sqrt(disc);
        double t1 = (-b - sq) / (2.0*a);
        double t2 = (-b + sq) / (2.0*a);
        double t = (fabs(t1) > fabs(t2)) ? t1 : t2;

        x = x + t*dx;
        y = y + t*dy;

        if (fabs(x) < 0.01 && y > 9.9) break;
        bounces++;
    }

    printf("%d\n", bounces);
    return 0;
}
