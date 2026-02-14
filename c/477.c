/*
 * Project Euler Problem 477: Number sequence game
 *
 * Sequence of N=10^8 numbers, two players pick from ends optimally.
 * Uses reduction algorithm: if b > a and b > c for consecutive a,b,c,
 * replace with a-b+c. Then greedy on reduced sequence.
 *
 * Uses __int128 for large intermediate sums.
 */
#include <stdio.h>
#include <stdlib.h>

typedef long long ll;
typedef __int128 lll;

#define M 1000000007LL
#define N 100000000LL

int main(void) {
    ll *nums = (ll *)malloc(N * sizeof(ll));
    if (!nums) { fprintf(stderr, "malloc failed\n"); return 1; }

    ll s = 0;
    for (ll i = 0; i < N; i++) {
        nums[i] = s;
        s = (s * s + 45) % M;
    }

    ll *reduced = (ll *)malloc(N * sizeof(ll));
    int idx = 0;
    lll sum = 0;

    for (ll i = 0; i < N; i++) {
        sum += nums[i];
        reduced[idx++] = nums[i];
        while (idx >= 3 && reduced[idx - 3] <= reduced[idx - 2] && reduced[idx - 2] >= reduced[idx - 1]) {
            reduced[idx - 3] += reduced[idx - 1] - reduced[idx - 2];
            idx -= 2;
        }
    }

    free(nums);

    lll reducedScore = 0;
    int start = 0, end = idx - 1;
    while (start <= end) {
        ll score;
        if (reduced[start] > reduced[end])
            score = reduced[start++];
        else
            score = reduced[end--];
        int parity = ((start + end) % 2 == 0) ? 1 : -1;
        reducedScore += (lll)parity * score;
    }

    free(reduced);

    lll ans = (sum + reducedScore) / 2;

    /* Print __int128 */
    ll hi = (ll)(ans / 1000000000000000LL);
    ll lo = (ll)(ans % 1000000000000000LL);
    if (lo < 0) { lo += 1000000000000000LL; hi--; }
    if (hi > 0)
        printf("%lld%015lld\n", hi, lo);
    else
        printf("%lld\n", lo);
    return 0;
}
