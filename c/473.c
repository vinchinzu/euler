/*
 * Project Euler Problem 473: Phigital palindromes
 *
 * Find the sum of all positive integers up to N=10^10 whose base-phi
 * representation (with no two adjacent 1s) is palindromic.
 *
 * Recursive enumeration of palindrome representations.
 */
#include <stdio.h>
#include <math.h>

typedef long long ll;

static const ll N = 10000000000LL; /* 10^10 */
static const double PHI = 1.6180339887498948482;

static ll ans;

/* Precompute Fibonacci-like values: phi^k + phi^(k+3) + phi^(-k-1) + phi^(-k-4)
 * These are always integers (Lucas number properties).
 * For k = 2, 4, 6, ..., these give the "palindrome increments".
 */

static void helper(ll n, int min_e) {
    if (n > N) return;
    ans += n;

    for (int e = min_e; ; e += 2) {
        /* Compute phi^e + phi^(e+3) + phi^(-e-1) + phi^(-e-4) */
        double val = pow(PHI, e) + pow(PHI, e + 3)
                   + pow(PHI, -(e + 1)) + pow(PHI, -(e + 4));
        ll increment = (ll)round(val);
        ll new_n = n + increment;
        if (new_n > N) break;
        helper(new_n, e + 6);
    }
}

int main(void) {
    ans = 1; /* 1 is trivially palindromic */

    /* Start with empty palindrome (even length) and add pairs */
    helper(0, 2);
    /* Start with center digit 1 (phi^0 + phi^1 = phi + 1 = phi^2 ... no)
     * Actually phi^1 + phi^(-2) = phi + 1/phi^2 = phi + phi - 1 = 2*phi - 1 ...
     * The Python starts helper(2, 4), meaning n=2 is the base case for
     * center-1 palindromes. phi^0 = 1, and some offset gives 2. */
    helper(2, 4);

    printf("%lld\n", ans);
    return 0;
}
