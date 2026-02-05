"""Project Euler Problem 693: Finite Sequence Generator.

For positive integers x > y, define sequence a_x = y, a_{z+1} = a_z^2 mod z,
stopping when a term equals 0 or 1. l(x,y) = length of this sequence.
g(x) = max l(x,y) over all y < x.  f(n) = max g(x) over all x <= n.

Key insight: g(x+1) >= g(x) - 1, so divide-and-conquer with pruning.
For computing g(x), track distinct alive values (quadratic residues) at
each step. Dynamically-sized hash table to handle large x.

Implemented in C for performance.
"""

from __future__ import annotations

import subprocess
import tempfile
import os


def solve() -> int:
    """Solve Problem 693 using compiled C."""
    c_code = r"""
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define CACHE_SIZE 4000000
int cache[CACHE_SIZE];

int compute_g(int x) {
    if (x <= 2) return 0;
    if (x < CACHE_SIZE && cache[x] != -1) return cache[x];

    int *used = (int*)calloc(2 * x + 100, sizeof(int));
    int *ys = (int*)malloc(x * sizeof(int));
    int *new_ys = (int*)malloc(x * sizeof(int));

    int ys_size = 0;
    for (int i = 2; i < x; i++) {
        ys[ys_size++] = i;
    }

    int z = x;
    while (1) {
        if (ys_size == 0) {
            int result = z - x + 1;
            free(used);
            free(ys);
            free(new_ys);
            if (x < CACHE_SIZE) cache[x] = result;
            return result;
        }

        int new_ys_size = 0;
        for (int i = 0; i < ys_size; i++) {
            long long val = ((long long)ys[i] * ys[i]) % z;
            int new_y = (int)val;
            if (new_y > 1 && used[new_y] != z) {
                new_ys[new_ys_size++] = new_y;
            }
            used[new_y] = z;
        }

        int *temp = ys;
        ys = new_ys;
        ys_size = new_ys_size;
        new_ys = temp;
        z++;
    }
}

int global_best;

void helper(int low, int high, int depth) {
    if (low >= high) return;

    int g_high = compute_g(high);
    if (g_high > global_best) {
        global_best = g_high;
    }

    if (low + 1 == high || depth == 0) return;
    if (global_best >= g_high + high - low) return;

    int mid = (low + high) / 2;
    helper(low, mid, depth - 1);
    helper(mid, high, depth - 1);
}

int main(void) {
    int N = 3000000;
    global_best = 0;

    // Initialize cache
    memset(cache, -1, sizeof(cache));

    // Iteratively increase depth like the Java version
    for (int depth = 1; (1 << depth) < N; depth++) {
        helper(0, N, depth);
    }

    printf("%d\n", global_best);
    return 0;
}
"""
    with tempfile.NamedTemporaryFile(suffix='.c', mode='w', delete=False) as f:
        f.write(c_code)
        c_path = f.name

    bin_path = c_path.replace('.c', '')
    try:
        subprocess.run(['gcc', '-O2', '-o', bin_path, c_path, '-lm'],
                       check=True, capture_output=True)
        result = subprocess.run([bin_path], capture_output=True, text=True,
                                check=True, timeout=60)
        return int(result.stdout.strip())
    finally:
        os.unlink(c_path)
        if os.path.exists(bin_path):
            os.unlink(bin_path)


def main() -> int:
    """Main entry point."""
    result = solve()
    print(result)
    return result


if __name__ == "__main__":
    main()
