/*
 * Project Euler 692 - Siegbert and Jo (Fibonacci Nim)
 *
 * G(N) = sum_{k=1}^N H(k) where H(k) is the smallest Fibonacci number
 * in the Zeckendorf representation of k.
 *
 * Recurrence: G(F_i) = G(F_{i-1}) + G(F_{i-2}) + F_{i-2}
 * (where F_1=1, F_2=1, F_3=2, ...)
 */
#include <stdio.h>

typedef long long ll;

int main(void) {
    ll N = 23416728348467685LL;

    /* Find Fibonacci numbers up to N */
    ll fibs[100];
    fibs[1] = 1; fibs[2] = 1;
    int cnt = 2;
    for (int i = 3; ; i++) {
        fibs[i] = fibs[i-1] + fibs[i-2];
        cnt = i;
        if (fibs[i] >= N) break;
    }

    /* G(F_2) = 1 (G(1) = H(1) = 1) */
    /* G(F_3) = H(1) + H(2) = 1 + 2 = 3 */
    /* Actually: a = G(F_{i-2}), b = G(F_{i-1}), c = G(F_i) */
    /* c = b + a + F_{i-2} */
    ll a = 1; /* G(F_2) = G(1) = 1 */
    ll b = 1; /* G(F_2) = G(1) = 1 initially, will be updated */

    /* Let me follow the Python more carefully:
       a, b start at 1, 1
       i starts at 2
       loop while fib(i) < N:
         c = a + b + fib(i)
         a = b
         b = c
         i++
       return b
    */
    a = 1; b = 1;
    int i = 2;
    while (1) {
        /* Compute fib(i) */
        ll f_a = 1, f_b = 1;
        for (int j = 2; j < i; j++) {
            ll f_c = f_a + f_b;
            f_a = f_b;
            f_b = f_c;
        }
        /* fib(i) = f_b */
        if (f_b >= N) break;
        ll c = a + b + f_b;
        a = b;
        b = c;
        i++;
    }

    printf("%lld\n", b);
    return 0;
}
