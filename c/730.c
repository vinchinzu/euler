/* Project Euler 730: Shifted Pythagorean Triples.
 * DFS using Barning matrices on primitive triples.
 */
#include <stdio.h>
#include <stdlib.h>
#include <math.h>
#include <stdbool.h>

#define N 100000000
#define K 100
#define L 200

int gcd(int a, int b) {
    while (b) { int t = b; b = a % b; a = t; }
    return a;
}

bool used[K+1][L][L];
long long ans = 0;

void helper(int k, int a, int b, int c) {
    if (a + b + c > N) return;
    if (a > b) { helper(k, b, a, c); return; }
    if (a < L && b < L && k <= K) {
        if (used[k][a][b]) return;
        used[k][a][b] = true;
    }
    ans++;
    helper(k, a - 2*b + 2*c, 2*a - b + 2*c, 2*a - 2*b + 3*c);
    helper(k, a + 2*b + 2*c, 2*a + b + 2*c, 2*a + 2*b + 3*c);
    if (a != b)
        helper(k, -a + 2*b + 2*c, -2*a + b + 2*c, -2*a + 2*b + 3*c);
}

int main() {
    for (int k = 0; k <= K; k++)
        for (int p = 1; p < L; p++)
            for (int q = p; q < L; q++) {
                long long r2 = (long long)p*p + (long long)q*q + k;
                int r = (int)sqrt((double)r2);
                while ((long long)r*r < r2) r++;
                while ((long long)r*r > r2) r--;
                if ((long long)r*r == r2 && p+q+r <= N && gcd(gcd(p,q),r) == 1)
                    helper(k, p, q, r);
            }
    printf("%lld\n", ans);
    return 0;
}
