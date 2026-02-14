/*
 * Project Euler Problem 514: Geoboard Shapes.
 * Expected area of convex hull of random pins on a lattice.
 */
#include <stdio.h>
#include <stdlib.h>
#include <math.h>

#define NN 100

static double pow_q_arr[10202]; /* (N+1)^2 + 1 */

static double E[NN + 1][NN + 1];

int main() {
    int N = NN;
    double P = 1.0 / (N + 1);
    double Q = 1.0 - P;

    int max_power = (N + 1) * (N + 1);
    pow_q_arr[0] = 1.0;
    for (int i = 1; i <= max_power; i++)
        pow_q_arr[i] = pow_q_arr[i - 1] * Q;

    /* Compute E[w][h] */
    for (int w = 0; w <= N; w++)
        for (int h = 0; h <= N; h++)
            E[w][h] = 0.0;

    for (int w = 1; w <= N; w++) {
        for (int h = 1; h <= N; h++) {
            int min_dim = (w < h - 1) ? w : (h - 1);
            for (int s = 0; s <= min_dim; s++) {
                for (int x = 0; x < s; x++) {
                    int tri_idx = s * (s - 1) / 2 + x;  /* tr(s-1) + x */
                    double prob = P * pow_q_arr[tri_idx];
                    double area = (double)s * s / 2.0 + E[x][h - s] + E[w - s][s - x];
                    E[w][h] += prob * area;
                }
            }

            if (min_dim >= 0) {
                int tri_idx = min_dim * (min_dim + 1) / 2;  /* tr(min_dim) */
                double prob = pow_q_arr[tri_idx];
                double area;
                if (w < h)
                    area = (double)w * w / 2.0 + E[w][h - w];
                else
                    area = (double)h * h / 2.0 + E[w - h][h];
                E[w][h] += prob * area;
            }
        }
    }

    double f_cache[NN + 1][NN + 1]; /* f(w, h) */
    for (int w = 0; w <= N; w++)
        for (int h = 0; h <= N; h++) {
            int idx = (N + 1) * (N + 1) - (w + 1) * (h + 1);
            f_cache[w][h] = pow_q_arr[idx];
        }

    /* Need f for indices up to N and down to -2, handle carefully */
    /* f(w, h) = pow_q[(N+1)^2 - (w+1)*(h+1)] */
    /* For w or h < 0, (w+1)*(h+1) could be 0 or negative, adjust */
    /* Use function approach */
    #define F(w, h) ({ \
        int _w = (w), _h = (h); \
        double _r; \
        if (_w < -1 || _h < -1) _r = 0.0; \
        else { \
            int _idx = (N+1)*(N+1) - (_w+1)*(_h+1); \
            if (_idx < 0 || _idx > max_power) _r = 0.0; \
            else _r = pow_q_arr[_idx]; \
        } \
        _r; \
    })

    double ans = 0.0;
    for (int w = 1; w <= N; w++) {
        for (int h = 1; h <= N; h++) {
            int num_regions = (N - w + 1) * (N - h + 1);
            double prob = F(w, h) - 2 * F(w, h-1) - 2 * F(w-1, h)
                        + F(w, h-2) + 4 * F(w-1, h-1) + F(w-2, h)
                        - 2 * F(w-1, h-2) - 2 * F(w-2, h-1) + F(w-2, h-2);
            ans += num_regions * prob * w * h;

            for (int x = 1; x <= w; x++) {
                for (int y = 1; y <= h; y++) {
                    double mult;
                    if (x == w && y == h)
                        mult = 1.0;
                    else if (x == w)
                        mult = 1.0 - pow_q_arr[w];
                    else if (y == h)
                        mult = 1.0 - pow_q_arr[h + 1];
                    else
                        mult = P + Q * (1 - pow_q_arr[w - 1]) * (1 - pow_q_arr[h]);
                    ans -= 4.0 * num_regions * mult * P * P
                           * pow_q_arr[x + h - y] * F(w, h) * E[x][y];
                }
            }
        }
    }

    printf("%.5f\n", ans);
    return 0;
}
