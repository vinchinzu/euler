#include <stdio.h>
#include <stdlib.h>
#include <math.h>
#include <string.h>
#include <limits.h>

#define MAX_SIDES 500
#define MAX_BRANCHES 2000
#define MAX_NUM_SIDES 250

typedef struct {
    int x, y;
} Point;

typedef struct {
    int x;
    int twiceArea;
} Branch;

int gcd(int a, int b) {
    while (b) { int t = b; b = a % b; a = t; }
    return a;
}

int point_cmp(const void *a, const void *b) {
    Point *pa = (Point*)a, *pb = (Point*)b;
    double aa = atan2(pa->y, pa->x);
    double ab = atan2(pb->y, pb->x);
    if (aa < ab) return -1;
    if (aa > ab) return 1;
    return 0;
}

// Store branches for each numSides count
Branch branches_a[MAX_NUM_SIDES][MAX_BRANCHES];
int branches_a_len[MAX_NUM_SIDES];
Branch branches_b[MAX_NUM_SIDES][MAX_BRANCHES];
int branches_b_len[MAX_NUM_SIDES];
Branch merged[MAX_BRANCHES * 2];

int main() {
    int N = 1000;
    int L = 40;

    // Generate sides
    Point sides[MAX_SIDES];
    int numSides = 0;
    for (int x = 1; x <= L; x++)
        for (int y = 1; y < x; y++)
            if (gcd(x, y) == 1)
                sides[numSides++] = (Point){x, y};

    qsort(sides, numSides, sizeof(Point), point_cmp);

    // Initialize: allBranches[0] = [(0, 0)]
    // Use branches_a as current, branches_b as next
    memset(branches_a_len, 0, sizeof(branches_a_len));
    branches_a[0][0] = (Branch){0, 0};
    branches_a_len[0] = 1;

    for (int si = 0; si < numSides; si++) {
        int sx = sides[si].x, sy = sides[si].y;

        // Reset 0-branch
        branches_a[0][0] = (Branch){0, 0};
        branches_a_len[0] = 1;

        memset(branches_b_len, 0, sizeof(branches_b_len));

        for (int ns = 1; ns < N / 4; ns++) {
            int prevLen = branches_a_len[ns];
            Branch *prev = branches_a[ns];

            // Generate curr from branches_a[ns-1]
            int prevM1Len = branches_a_len[ns - 1];
            Branch *prevM1 = branches_a[ns - 1];
            int currLen = 0;
            Branch curr[MAX_BRANCHES];
            for (int k = 0; k < prevM1Len; k++) {
                int new_x = prevM1[k].x + sx;
                int new_area = prevM1[k].twiceArea + sy * (2 * prevM1[k].x + sx + 1);
                curr[currLen++] = (Branch){new_x, new_area};
            }

            // Merge prev and curr (both sorted by x)
            int mergedLen = 0;
            int i = 0, j = 0;
            while (i < prevLen || j < currLen) {
                Branch b;
                if (i < prevLen && j < currLen) {
                    if (prev[i].x < curr[j].x || (prev[i].x == curr[j].x && prev[i].twiceArea <= curr[j].twiceArea))
                        b = prev[i++];
                    else
                        b = curr[j++];
                } else if (i < prevLen) {
                    b = prev[i++];
                } else {
                    b = curr[j++];
                }
                if (mergedLen == 0 || b.twiceArea < merged[mergedLen-1].twiceArea)
                    merged[mergedLen++] = b;
            }

            branches_b_len[ns] = mergedLen;
            memcpy(branches_b[ns], merged, mergedLen * sizeof(Branch));
        }

        // Swap a and b
        memcpy(branches_a_len, branches_b_len, sizeof(branches_a_len));
        for (int ns = 1; ns < N / 4; ns++)
            memcpy(branches_a[ns], branches_b[ns], branches_b_len[ns] * sizeof(Branch));
    }

    // Find minimum area
    long long ans = LLONG_MAX;
    for (int ns1 = 1; ns1 < N / 8; ns1++) {
        int ns2 = N / 4 - 2 - ns1;
        if (ns2 < 0 || ns2 >= MAX_NUM_SIDES) continue;
        for (int i = 0; i < branches_a_len[ns1]; i++) {
            for (int j = 0; j < branches_a_len[ns2]; j++) {
                long long area = 2LL * branches_a[ns1][i].twiceArea
                    + 2LL * branches_a[ns2][j].twiceArea
                    + 4LL * branches_a[ns1][i].x * branches_a[ns2][j].x
                    + 6LL * branches_a[ns1][i].x
                    + 6LL * branches_a[ns2][j].x
                    + 7;
                if (area < ans) ans = area;
            }
        }
    }

    printf("%lld\n", ans);
    return 0;
}
