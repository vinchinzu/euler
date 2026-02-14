#include <stdio.h>
#include <math.h>

/*
 * Project Euler 901: Drilling problem.
 * Binary search for critical d1, then compute expected cost.
 * Uses long double for precision.
 */

int check_growth(long double d1_val) {
    long double d_prev = 0.0L;
    long double d_curr = d1_val;

    for (int i = 0; i < 1000; i++) {
        if (d_curr - d_prev > 20.0L)
            return 1;  /* Growth */

        long double d_next = expl(d_curr - d_prev);

        if (d_next <= d_curr)
            return 0;  /* Collapse */

        d_prev = d_curr;
        d_curr = d_next;

        if (d_curr > 1000.0L)
            return 1;
    }
    return 1;
}

int main(void) {
    long double low = 0.7L;
    long double high = 0.8L;

    for (int i = 0; i < 200; i++) {
        long double mid = (low + high) / 2.0L;
        if (check_growth(mid))
            high = mid;
        else
            low = mid;
    }

    long double optimal_d1 = high;

    /* Calculate Expected Cost: E = d1 + 1 + sum(exp(-d_k)) for k=1.. */
    long double cost = optimal_d1 + 1.0L;

    long double d_prev = 0.0L;
    long double d_curr = optimal_d1;

    while (1) {
        long double term = expl(-d_curr);
        cost += term;

        if (term < 1e-25L)
            break;

        long double d_next = expl(d_curr - d_prev);
        d_prev = d_curr;
        d_curr = d_next;
    }

    printf("%.9Lf\n", cost);
    return 0;
}
