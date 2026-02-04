"""Project Euler Problem 563: Robot Welders.

Find sum of M(n) for n=2..100, where M(n) is the minimal area that can be
manufactured as w*h in exactly n variants with h/w <= 11/10.

Dimensions are 23-smooth numbers (products of integers 1..25).
"""

import subprocess, tempfile, os

def solve():
    c_code = r"""
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int primes[] = {2, 3, 5, 7, 11, 13, 17, 19, 23};
int nprimes = 9;

typedef long long ll;

ll *smooths;
int nsmooths;
int smooth_cap;

void gen_smooth(ll n, int pi, ll limit) {
    if (nsmooths >= smooth_cap - 1) {
        smooth_cap *= 2;
        smooths = (ll *)realloc(smooths, smooth_cap * sizeof(ll));
    }
    smooths[nsmooths++] = n;
    for (int i = pi; i < nprimes; i++) {
        ll next = n * primes[i];
        if (next > limit) break;
        gen_smooth(next, i, limit);
    }
}

int cmp_ll(const void *a, const void *b) {
    ll x = *(const ll *)a;
    ll y = *(const ll *)b;
    if (x < y) return -1;
    if (x > y) return 1;
    return 0;
}

int main() {
    ll LIMIT = 500000000LL; /* 5*10^8 */

    smooth_cap = 1000000;
    smooths = (ll *)malloc(smooth_cap * sizeof(ll));
    nsmooths = 0;
    gen_smooth(1, 0, LIMIT);

    qsort(smooths, nsmooths, sizeof(ll), cmp_ll);

    fprintf(stderr, "Smooth numbers up to %lld: %d\n", LIMIT, nsmooths);

    /* Count pairs */
    long long total_pairs = 0;
    for (int i = 0; i < nsmooths; i++) {
        ll w = smooths[i];
        ll h_max = w + w / 10;
        /* Correct: h_max = floor(11*w/10) */
        /* 11*w might overflow for w > ~8*10^17, but we're well within range */
        if (w <= 1000000000LL) h_max = 11 * w / 10;
        else h_max = w + w / 10;

        /* binary search for first smooth >= w starting from index i */
        int lo = i, hi = nsmooths - 1, start = nsmooths;
        while (lo <= hi) {
            int mid = (lo + hi) / 2;
            if (smooths[mid] >= w) { start = mid; hi = mid - 1; }
            else lo = mid + 1;
        }
        /* binary search for last smooth <= h_max */
        lo = start; hi = nsmooths - 1;
        int end = start - 1;
        while (lo <= hi) {
            int mid = (lo + hi) / 2;
            if (smooths[mid] <= h_max) { end = mid; lo = mid + 1; }
            else hi = mid - 1;
        }
        total_pairs += (end - start + 1);
    }

    fprintf(stderr, "Total pairs: %lld\n", total_pairs);

    ll *areas = (ll *)malloc(total_pairs * sizeof(ll));
    if (!areas) {
        fprintf(stderr, "Cannot allocate\n");
        return 1;
    }

    long long pidx = 0;
    for (int i = 0; i < nsmooths; i++) {
        ll w = smooths[i];
        ll h_max;
        if (w <= 1000000000LL) h_max = 11 * w / 10;
        else h_max = w + w / 10;

        int lo = i, hi = nsmooths - 1, start = nsmooths;
        while (lo <= hi) {
            int mid = (lo + hi) / 2;
            if (smooths[mid] >= w) { start = mid; hi = mid - 1; }
            else lo = mid + 1;
        }
        lo = start; hi = nsmooths - 1;
        int end = start - 1;
        while (lo <= hi) {
            int mid = (lo + hi) / 2;
            if (smooths[mid] <= h_max) { end = mid; lo = mid + 1; }
            else hi = mid - 1;
        }
        for (int j = start; j <= end; j++) {
            areas[pidx++] = w * smooths[j];
        }
    }

    qsort(areas, pidx, sizeof(ll), cmp_ll);

    ll found_area[101];
    for (int i = 0; i <= 100; i++) found_area[i] = 0;

    long long i = 0;
    while (i < pidx) {
        long long j = i + 1;
        while (j < pidx && areas[j] == areas[i]) j++;
        int cnt = (int)(j - i);
        if (cnt >= 2 && cnt <= 100) {
            if (found_area[cnt] == 0) {
                found_area[cnt] = areas[i];
            }
        }
        i = j;
    }

    /* Check if all found */
    int missing = 0;
    for (int n = 2; n <= 100; n++) {
        if (found_area[n] == 0) {
            fprintf(stderr, "Missing M(%d)\n", n);
            missing = 1;
        }
    }

    ll answer = 0;
    for (int n = 2; n <= 100; n++) {
        answer += found_area[n];
    }

    printf("%lld\n", answer);

    free(smooths);
    free(areas);
    return 0;
}
"""

    with tempfile.TemporaryDirectory() as tmpdir:
        src = os.path.join(tmpdir, "sol.c")
        exe = os.path.join(tmpdir, "sol")
        with open(src, "w") as f:
            f.write(c_code)
        r = subprocess.run(["gcc", "-O2", "-o", exe, src], capture_output=True, text=True)
        if r.returncode != 0:
            print(r.stderr)
            return -1
        result = subprocess.run([exe], capture_output=True, text=True, timeout=30)
        import sys
        sys.stderr.write(result.stderr)
        if result.returncode != 0:
            print(result.stderr)
            return -1
        return int(result.stdout.strip())

if __name__ == "__main__":
    print(solve())
