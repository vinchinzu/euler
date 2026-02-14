/*
 * Project Euler Problem 317: Firecracker
 *
 * Volume swept by fragments from explosion at height H0=100m,
 * initial speed V0=20 m/s, gravity g=9.81 m/s^2.
 *
 * Envelope: z(rho) = H0 + V0^2/(2g) - g*rho^2/(2*V0^2)
 * Volume = integral_0^rho_max 2*pi*rho*z(rho) d_rho
 *
 * This integral has a closed form:
 * V = pi * (H0 + V0^2/(2g)) * rho_max^2 / 2 - pi*g*rho_max^4 / (8*V0^2)
 * where rho_max = V0/g * (V0 + sqrt(V0^2 + 2*g*H0))
 *   (the maximum horizontal distance where z=0).
 *
 * Actually: z(rho) = z_top - g*rho^2/(2*V0^2) where z_top = H0 + V0^2/(2g)
 * z(rho_max) = 0 => rho_max^2 = 2*V0^2*z_top/g
 *
 * Volume = integral 2*pi*rho*(z_top - g*rho^2/(2*V0^2)) d_rho from 0 to rho_max
 *        = 2*pi * [z_top * rho^2/2 - g/(2*V0^2) * rho^4/4] from 0 to rho_max
 *        = 2*pi * (z_top * R^2/2 - g*R^4/(8*V0^2))
 * where R = rho_max.
 *
 * R^2 = 2*V0^2*z_top/g
 * So z_top * R^2/2 = V0^2 * z_top^2 / g
 *    g*R^4/(8*V0^2) = g/(8*V0^2) * 4*V0^4*z_top^2/g^2 = V0^2*z_top^2/(2*g)
 *
 * Volume = 2*pi * (V0^2*z_top^2/g - V0^2*z_top^2/(2*g))
 *        = 2*pi * V0^2*z_top^2/(2*g)
 *        = pi * V0^2 * z_top^2 / g
 */
#include <stdio.h>
#include <math.h>

int main(void) {
    double g = 9.81;
    double H0 = 100.0;
    double V0 = 20.0;

    double z_top = H0 + V0 * V0 / (2.0 * g);
    double volume = M_PI * V0 * V0 * z_top * z_top / g;

    printf("%.4f\n", volume);
    return 0;
}
