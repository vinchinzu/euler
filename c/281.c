/*
 * Project Euler 281 - Pizza Toppings
 *
 * f(m,n) = number of ways to put m toppings on m*n slices (each topping on
 * exactly n slices), up to rotation. Uses Burnside's lemma:
 * f(m,n) = (1/(m*n)) * sum_{k: gcd(k,m*n) divisible by m} multinomial(g, [g/m]*m)
 * where g = gcd(k, m*n).
 *
 * Sum all f(m,n) <= 10^15 for m>=2, n>=1.
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef unsigned long long ull;
typedef __int128 u128;

static ull factorial_table[80];

static void build_factorial(int n) {
    factorial_table[0] = 1;
    for (int i = 1; i <= n; i++)
        factorial_table[i] = factorial_table[i - 1] * (ull)i;
}

static ull multinomial(int g, int count_per, int m) {
    /* multinomial(g; [count_per]*m) = g! / (count_per!)^m */
    u128 result = factorial_table[g];
    ull denom = factorial_table[count_per];
    for (int i = 0; i < m; i++)
        result /= denom;
    return (ull)result;
}

static int gcd(int a, int b) {
    while (b) { int t = b; b = a % b; a = t; }
    return a;
}

static ull f(int m, int n) {
    int mn = m * n;
    u128 total = 0;
    for (int k = 0; k < mn; k++) {
        int g = gcd(k, mn);
        if (g % m == 0) {
            int count_per = g / m;
            total += multinomial(g, count_per, m);
        }
    }
    return (ull)(total / mn);
}

int main(void) {
    ull N = 1000000000000000ULL; /* 10^15 */
    build_factorial(70);

    ull ans = 0;

    for (int m = 2; ; m++) {
        if (f(m, 1) > N)
            break;
        for (int n = 1; ; n++) {
            ull fv = f(m, n);
            if (fv > N)
                break;
            ans += fv;
        }
    }

    printf("%llu\n", ans);
    return 0;
}
