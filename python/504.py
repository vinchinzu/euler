"""Project Euler Problem 504: Square on the Inside.

Find the number of quadrilaterals with lattice points on (a, 0), (0, b),
(-c, 0), (0, -d), where 1 ≤ a,b,c,d ≤ N, that strictly contain a square
number of lattice points.
"""

import subprocess, tempfile, os, sys

def solve():
    N = 100
    c_code = r"""
#include <stdio.h>
#include <stdlib.h>

static int gcd(int a, int b) {
    while (b) { int t = b; b = a % b; a = t; }
    return a;
}

int main(void) {
    const int N = 100;
    /* f[a][b] = interior lattice points contributed by triangle quadrant (a,b) */
    int f[101][101];
    for (int a = 1; a <= N; a++)
        for (int b = 1; b <= N; b++)
            f[a][b] = ((a+1)*(b+1) - gcd(a,b) - 1) / 2 - a;

    /* Precompute perfect squares set */
    int max_val = 2*N*N;
    char *is_sq = (char*)calloc(max_val+1, 1);
    for (int i = 1; i*i <= max_val; i++)
        is_sq[i*i] = 1;

    /* Precompute pair sums: psum[b][s] counts how many (a,d) pairs give f[a][b]+f[d][b]=s
       But actually we need f[a][b]+f[b][c]+f[c][d]+f[d][a]+1.
       Rewrite as (f[a][b]+f[d][a]) + (f[b][c]+f[c][d]) + 1.
       For fixed b,c: sum over a of f[a][b]+f[d][a] ... no, a and d are independent of c.

       Better: for each (a,b) pair, val_ab = f[a][b]. Then total = val_ab + val_bc + val_cd + val_da + 1.
       Group as (val_ab + val_da) + (val_bc + val_cd) + 1.
       For fixed a: left = f[a][b] + f[d][a] depends on a,b,d.

       Actually simplest: precompute count of (x,y) pairs where f[x][y]+f[y][z] = S, for each y and S.
       Then for each b, for each S1 (from a: f[a][b]) and each d, S2=f[b][c]+f[c][d]... still complex.

       Simplest O(N^3) approach: for each (b,d), precompute how many values of
       f[a][b]+f[d][a] occur, then for each (b,d) and each (c), check if
       f[b][c]+f[c][d]+1 + f[a][b]+f[d][a] is a perfect square.

       Actually easier: precompute cnt[v] = number of (a,b,c,d) combos giving total=v,
       by splitting into two halves.

       Let left(a,b) = f[a][b] for all a,b pairs.
       Let right(c,d) similarly.
       total = left(a,b) + f[b][c] + f[c][d] + f[d][a] + 1
       This doesn't split cleanly.

       Just do O(N^4) in C - it's fast enough.
    */
    long long ans = 0;
    for (int a = 1; a <= N; a++)
        for (int b = 1; b <= N; b++) {
            int fab = f[a][b];
            for (int c = 1; c <= N; c++) {
                int fab_fbc = fab + f[b][c];
                for (int d = 1; d <= N; d++) {
                    int total = fab_fbc + f[c][d] + f[d][a] + 1;
                    if (total <= max_val && is_sq[total])
                        ans++;
                }
            }
        }
    printf("%lld\n", ans);
    free(is_sq);
    return 0;
}
"""
    with tempfile.NamedTemporaryFile(suffix='.c', mode='w', delete=False) as src:
        src.write(c_code)
        src_path = src.name
    bin_path = src_path.replace('.c', '')
    try:
        subprocess.run(['gcc', '-O2', '-o', bin_path, src_path, '-lm'], check=True,
                       capture_output=True)
        result = subprocess.run([bin_path], capture_output=True, text=True, check=True)
        print(result.stdout.strip())
    finally:
        os.unlink(src_path)
        if os.path.exists(bin_path):
            os.unlink(bin_path)

if __name__ == "__main__":
    solve()
