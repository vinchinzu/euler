/*
 * Project Euler Problem 594: Rhombus tilings of an octagon.
 *
 * Find the number of ways to tile an octagon with sides A=4, B=2
 * with unit squares and unit 45-degree rhombi.
 *
 * Uses brute force over all possible (x, y) values at interior grid points.
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>

typedef long long ll;

#define A 4
#define B 2
#define BSIZ (B + 2)

/* Binomial coefficient, returns 0 for invalid */
ll nCr(int n, int k) {
    if (k < 0 || n < 0 || k > n) return 0;
    if (k == 0 || k == n) return 1;
    if (k > n - k) k = n - k;
    ll result = 1;
    for (int i = 0; i < k; i++) {
        result = result * (n - i) / (i + 1);
    }
    return result;
}

/* Determinant of a BxB matrix */
ll det(ll mat[B][B]) {
    int n = B;
    if (n == 1) return mat[0][0];
    if (n == 2) return mat[0][0] * mat[1][1] - mat[0][1] * mat[1][0];
    /* General case for small matrices */
    ll result = 0;
    for (int j = 0; j < n; j++) {
        ll minor[B][B];
        for (int i = 1; i < n; i++) {
            int col = 0;
            for (int k = 0; k < n; k++) {
                if (k != j) {
                    minor[i - 1][col++] = mat[i][k];
                }
            }
        }
        ll sign = (j % 2 == 0) ? 1 : -1;
        result += sign * mat[0][j] * det(minor);
    }
    return result;
}

int main() {
    ll ans = 0;

    /* Total indices: B*B = 4 interior points, each with (A+1)^2 = 25 possibilities */
    int total = 1;
    for (int i = 0; i < B * B; i++) total *= (A + 1) * (A + 1);

    for (int idx = 0; idx < total; idx++) {
        int x[BSIZ][BSIZ], y[BSIZ][BSIZ];
        memset(x, 0, sizeof(x));
        memset(y, 0, sizeof(y));

        /* Set boundary conditions */
        for (int k = 1; k <= B; k++) {
            x[B + 1][k] = A;
            x[k][B + 1] = A;
            y[0][k] = A;
            y[k][B + 1] = A;
        }

        /* Set interior values from idx */
        int tmp = idx;
        for (int i = 1; i <= B; i++) {
            for (int j = 1; j <= B; j++) {
                int val = tmp % ((A + 1) * (A + 1));
                tmp /= (A + 1) * (A + 1);
                x[i][j] = val / (A + 1);
                y[i][j] = val % (A + 1);
            }
        }

        /* Compute number of tilings */
        ll num_tilings = 1;
        int valid = 1;

        for (int u = 1; u <= B + 1 && valid; u++) {
            ll M[B][B], P[B][B];

            for (int i = 1; i <= B; i++) {
                for (int j = 1; j <= B; j++) {
                    M[i - 1][j - 1] = nCr(
                        x[j][u] - x[i][u - 1] + y[j][u] - y[i][u - 1],
                        x[j][u] - x[i][u - 1] + j - i
                    );
                    P[i - 1][j - 1] = nCr(
                        x[u][j] - x[u - 1][i] + y[u - 1][i] - y[u][j],
                        x[u][j] - x[u - 1][i] + j - i
                    );
                }
            }

            ll dm = det(M);
            ll dp = det(P);
            num_tilings *= dm * dp;

            if (num_tilings == 0) {
                valid = 0;
            }
        }

        if (valid) {
            ans += num_tilings;
        }
    }

    printf("%lld\n", ans);
    return 0;
}
