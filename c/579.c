/*
 * Project Euler Problem 579: Lattice Points in Cubes
 *
 * Sum f(C) for all cubes with lattice point vertices, coordinates 0..N.
 * Uses quaternion parameterization of primitive cubes.
 * N = 5000, answer mod 10^9.
 */
#include <stdio.h>
#include <math.h>

static int my_gcd(int a, int b) {
    if (a < 0) a = -a;
    if (b < 0) b = -b;
    while (b) { int t = b; b = a % b; a = t; }
    return a;
}

static int my_abs(int x) { return x < 0 ? -x : x; }

static int isqrt_i(int n) {
    int r = (int)sqrt((double)n);
    while (r * r > n) r--;
    while ((r + 1) * (r + 1) <= n) r++;
    return r;
}

int main(void) {
    int N = 5000;
    long long M = 1000000000LL;

    long long ans = 0;

    for (int a = 0; a <= isqrt_i(N); a++) {
        for (int b = a; b <= isqrt_i(N - a * a); b++) {
            for (int c = b; c <= isqrt_i(N - a * a - b * b); c++) {
                for (int d = b; d <= isqrt_i(N - a * a - b * b - c * c); d++) {
                    int l_sq = a * a + b * b + c * c + d * d;
                    if (l_sq > N) break;
                    if (a + b + c + d == 0) continue;
                    if (d < c && (a == 0 || a == b || b == d)) continue;

                    /* Compute axes */
                    int axes[3][3];
                    axes[0][0] = a*a + b*b - c*c - d*d;
                    axes[0][1] = 2*(b*c + d*a);
                    axes[0][2] = 2*(b*d - c*a);
                    axes[1][0] = 2*(b*c - d*a);
                    axes[1][1] = a*a - b*b + c*c - d*d;
                    axes[1][2] = 2*(c*d + b*a);
                    axes[2][0] = 2*(b*d + c*a);
                    axes[2][1] = 2*(c*d - b*a);
                    axes[2][2] = a*a - b*b - c*c + d*d;

                    /* Compute GCDs */
                    int gcds[3];
                    for (int i = 0; i < 3; i++)
                        gcds[i] = my_abs(my_gcd(my_gcd(axes[i][0], axes[i][1]), axes[i][2]));

                    if (my_gcd(my_gcd(gcds[0], gcds[1]), gcds[2]) > 1) continue;

                    /* Compute bounds */
                    int mins[3] = {3*N, 3*N, 3*N};
                    int maxs[3] = {-3*N, -3*N, -3*N};
                    for (int subset = 0; subset < 8; subset++) {
                        int v[3] = {0, 0, 0};
                        for (int i = 0; i < 3; i++) {
                            if (subset & (1 << i)) {
                                for (int dim = 0; dim < 3; dim++)
                                    v[dim] += axes[i][dim];
                            }
                        }
                        for (int dim = 0; dim < 3; dim++) {
                            if (v[dim] < mins[dim]) mins[dim] = v[dim];
                            if (v[dim] > maxs[dim]) maxs[dim] = v[dim];
                        }
                    }

                    /* Compute symmetries */
                    int num_symmetries = 24;
                    if (a == 0 && (b == c || c == d))
                        num_symmetries /= 2;
                    if (b == c && (a == b || c == d))
                        num_symmetries /= 3;
                    if (b == 0)
                        num_symmetries /= 4;

                    int D = gcds[0] + gcds[1] + gcds[2];
                    int l = isqrt_i(l_sq);
                    for (int t = 1; t <= N; t++) {
                        long long lt = (long long)l * t;
                        long long num_points = lt * lt * lt + (long long)l * D * t * t + (long long)D * t + 1;
                        long long num_cubes = 1;
                        for (int i = 0; i < 3; i++) {
                            int size = (maxs[i] - mins[i]) * t;
                            long long nc = N - size + 1;
                            if (nc <= 0) { num_cubes = 0; break; }
                            num_cubes *= nc;
                        }
                        if (num_cubes == 0) break;
                        ans = (ans + (num_points % M) * (num_cubes % M) % M * num_symmetries) % M;
                    }
                }
            }
        }
    }

    printf("%lld\n", ans);
    return 0;
}
