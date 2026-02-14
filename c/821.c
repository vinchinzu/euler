/*
 * Project Euler 821: 123-Separable
 *
 * Find max elements in {S ∪ 2S ∪ 3S} ∩ {1..N} where S, 2S, 3S are disjoint.
 * For each c coprime to 6, independently find max coverage on the 2^a * 3^b grid.
 */
#include <stdio.h>
#include <stdlib.h>

static int cmp_ll(const void *a, const void *b) {
    long long va = *(const long long *)a;
    long long vb = *(const long long *)b;
    if (va < vb) return -1;
    if (va > vb) return 1;
    return 0;
}

int main(void) {
    long long N = 10000000000000000LL; /* 10^16 */

    long long nums[200];
    int cnt = 0;
    nums[cnt++] = 1;
    nums[cnt++] = 6;
    nums[cnt++] = 24;
    nums[cnt++] = 54;
    nums[cnt++] = N + 1;

    long long i = 384;
    while (i <= N) {
        nums[cnt++] = i;
        i *= 8;
    }
    i = 243;
    while (i <= N) {
        nums[cnt++] = i;
        i *= 27;
    }

    qsort(nums, cnt, sizeof(long long), cmp_ll);

    long long ans = N;
    for (int j = 0; j < cnt - 1; j++) {
        long long low = N / nums[j + 1];
        long long high = N / nums[j];
        low -= low / 2 + low / 3 - low / 6;
        high -= high / 2 + high / 3 - high / 6;
        ans -= (high - low) * (long long)j;
    }

    printf("%lld\n", ans);
    return 0;
}
