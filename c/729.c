/* Project Euler 729: Range of Periodic Sequence.
 * Lyndon word enumeration with fixed-point iteration.
 */
#include <stdio.h>
#include <math.h>
#include <string.h>

#define MAXN 25

double ans = 0.0;

void process_lyndon(int *w, int len) {
    if (len < 2) return;

    unsigned int word = 0;
    for (int i = 0; i < len; i++) {
        if (w[i]) word |= (1u << i);
    }

    /* Find fixed point by iterating the composed map */
    double d = 1.0;
    for (int iter = 0; iter < 300; iter++) {
        double prev = d;
        for (int i = 0; i < len; i++) {
            int bit = (word >> i) & 1;
            double sign = (bit == 0) ? 1.0 : -1.0;
            d = (d + sign * sqrt(d * d + 4.0)) / 2.0;
        }
        if (fabs(d - prev) < 1e-13) break;
    }

    /* Compute range */
    double min_val = d, max_val = d;
    for (int i = 0; i < len; i++) {
        int bit = (word >> i) & 1;
        double sign = (bit == 0) ? 1.0 : -1.0;
        d = (d + sign * sqrt(d * d + 4.0)) / 2.0;
        if (d < min_val) min_val = d;
        if (d > max_val) max_val = d;
    }

    ans += len * (max_val - min_val);
}

int w_buf[MAXN + 2];

void gen(int t, int p, int n) {
    if (t > n) {
        if (p == n) {
            process_lyndon(w_buf + 1, n);
        }
    } else {
        w_buf[t] = w_buf[t - p];
        gen(t + 1, p, n);
        for (int j = w_buf[t - p] + 1; j <= 1; j++) {
            w_buf[t] = j;
            gen(t + 1, t, n);
        }
    }
}

int main() {
    ans = 0.0;

    for (int n = 2; n <= MAXN; n++) {
        memset(w_buf, 0, sizeof(w_buf));
        gen(1, 1, n);
    }

    printf("%.4f\n", ans);
    return 0;
}
