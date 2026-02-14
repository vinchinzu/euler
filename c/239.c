/*
 * Project Euler Problem 239: Twenty-two Foolish Primes
 *
 * Find probability that exactly 22 of 25 primes <= 100 are not in their
 * natural positions in a random permutation of 1..100.
 *
 * P = C(25,22) * D_22(25) * (100-25+3)! / 100!
 * where D_k(n) = subfactorial-like count using inclusion-exclusion.
 *
 * Formula: sum over k of (-1)^k * C(k, num_primes-K) * C(num_primes, k) * (N-k)!
 * divided by N!
 */
#include <stdio.h>
#include <math.h>

static int is_prime(int n) {
    if (n < 2) return 0;
    if (n < 4) return 1;
    if (n % 2 == 0 || n % 3 == 0) return 0;
    for (int i = 5; i * i <= n; i += 6)
        if (n % i == 0 || n % (i + 2) == 0) return 0;
    return 1;
}

/* Compute C(a, b) as double */
static double fn_cr(double a, int b) {
    if (b < 0 || a < b) return 0.0;
    double result = 1.0;
    for (int i = 0; i < b; i++)
        result *= (a - i) / (i + 1);
    return result;
}

/* Compute n! as double */
static double ffact(int n) {
    double r = 1.0;
    for (int i = 2; i <= n; i++) r *= i;
    return r;
}

int main(void) {
    int N = 100, K = 22;
    int num_primes = 0;
    for (int i = 2; i <= N; i++)
        if (is_prime(i)) num_primes++;

    double ans = 0.0;
    for (int k = 0; k <= num_primes; k++) {
        int sign = (k % 2 == 0) ? -1 : 1;
        ans += sign * fn_cr(k, num_primes - K) * fn_cr(num_primes, k) * ffact(N - k);
    }
    /* Wait, the Python code uses: ans -= parity(k) * ... where parity returns 1 if even, -1 if odd.
     * So: ans = -sum_{k} parity(k) * C(k, P-K) * C(P, k) * (N-k)!
     * parity(k) = 1 if k even, -1 if k odd
     * So: ans = -sum_{k even} C(k,P-K)*C(P,k)*(N-k)! + sum_{k odd} C(k,P-K)*C(P,k)*(N-k)!
     */

    ans = 0.0;
    for (int k = 0; k <= num_primes; k++) {
        double parity = (k % 2 == 0) ? 1.0 : -1.0;
        ans -= parity * fn_cr(k, num_primes - K) * fn_cr(num_primes, k) * ffact(N - k);
    }
    ans /= ffact(N);

    printf("%.12f\n", ans);
    return 0;
}
