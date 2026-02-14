/*
 * Project Euler Problem 228: Minkowski Sums
 *
 * Count distinct edge angles in Minkowski sum of regular polygons S_1864..S_1909.
 * An angle k/n (reduced) appears if gcd(k,n)=1 effectively.
 * We count distinct reduced fractions k/n for k in [0,n-1], n in [L..M].
 */
#include <stdio.h>
#include <string.h>

static int gcd(int a, int b) {
    while (b) { int t = b; b = a % b; a = t; }
    return a;
}

/*
 * We need to count distinct pairs (k/g, n/g) for n in [1864,1909], k in [0,n-1].
 * The reduced denominator n/g ranges up to 1909.
 * For each reduced fraction p/q with q <= 1909, check if there exists n in [L,M]
 * that is a multiple of q, and p < q.
 *
 * Equivalently, count the number of distinct reduced fractions p/q with
 * 0 <= p < q, q divides some n in [L,M].
 *
 * Actually we need: for each n in [L..M], for each k in [0..n-1],
 * add (k/gcd(k,n), n/gcd(k,n)) to a set. Count the set size.
 *
 * The denominators q that appear are exactly those q such that q divides
 * some n in [L..M]. For each such q, all p with gcd(p,q)=1 and 0<=p<q contribute.
 * Plus p=0,q=1 always contributes.
 *
 * But wait: reduced (k/g, n/g) means q = n/g, so q must be a divisor of n.
 * So q ranges over all divisors of all n in [L..M].
 *
 * Actually no. Let me re-read: for polygon S_n, the edge angles are 2*pi*k/n
 * for k=0..n-1. In the Minkowski sum, we take the union of all edge angles.
 * Two angles are the same iff k1/n1 = k2/n2 as fractions.
 * So we count distinct fractions k/n (reduced) over all n in [L..M], k in [0..n-1].
 *
 * A reduced fraction p/q appears iff q divides some n in [L..M] and p < q, gcd(p,q)=1.
 * (Or p=0, q=1.)
 *
 * So: for each q from 1 to M, check if any multiple of q is in [L..M].
 * If so, add euler_phi(q) to the count (the number of valid p values).
 * Special case: q=1 contributes p=0 only, which is phi(1)=1.
 */

int main(void) {
    int L = 1864, M = 1909;
    int count = 0;

    for (int q = 1; q <= M; q++) {
        /* Check if any multiple of q is in [L, M] */
        int first_multiple = ((L + q - 1) / q) * q;
        if (first_multiple <= M) {
            /* Add euler_phi(q) */
            int phi = q;
            int temp = q;
            for (int p = 2; p * p <= temp; p++) {
                if (temp % p == 0) {
                    while (temp % p == 0) temp /= p;
                    phi -= phi / p;
                }
            }
            if (temp > 1) phi -= phi / temp;
            count += phi;
        }
    }

    printf("%d\n", count);
    return 0;
}
