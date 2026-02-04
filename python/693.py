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

int compute_g(int x) {
    if (x <= 2) return 0;

    int hash_size = 1;
    while (hash_size < 2 * x) hash_size <<= 1;
    int hash_mask = hash_size - 1;

    int *ht = (int*)malloc(hash_size * sizeof(int));
    int *v1 = (int*)malloc(x * sizeof(int));
    int *v2 = (int*)malloc(x * sizeof(int));
    memset(ht, -1, hash_size * sizeof(int));
    int count = 0;

    for (int y = 2; y < x; y++) {
        long long v = ((long long)y * y) % x;
        if (v <= 1) continue;
        unsigned int h = (unsigned int)v & hash_mask;
        while (ht[h] != -1 && ht[h] != (int)v)
            h = (h + 1) & hash_mask;
        if (ht[h] == -1) {
            ht[h] = (int)v;
            v1[count++] = (int)v;
        }
    }
    if (count == 0) { free(ht); free(v1); free(v2); return 2; }
    for (int i = 0; i < count; i++) {
        unsigned int h = (unsigned int)v1[i] & hash_mask;
        while (ht[h] != v1[i]) h = (h + 1) & hash_mask;
        ht[h] = -1;
    }
    int steps = 1, z = x + 1;
    while (count > 0) {
        int new_count = 0;
        for (int i = 0; i < count; i++) {
            long long nv = ((long long)v1[i] * v1[i]) % z;
            if (nv <= 1) continue;
            unsigned int h = (unsigned int)nv & hash_mask;
            while (ht[h] != -1 && ht[h] != (int)nv)
                h = (h + 1) & hash_mask;
            if (ht[h] == -1) {
                ht[h] = (int)nv;
                v2[new_count++] = (int)nv;
            }
        }
        for (int i = 0; i < new_count; i++) {
            unsigned int h = (unsigned int)v2[i] & hash_mask;
            while (ht[h] != v2[i]) h = (h + 1) & hash_mask;
            ht[h] = -1;
        }
        int *tmp = v1; v1 = v2; v2 = tmp;
        count = new_count;
        steps++;
        z++;
    }
    free(ht); free(v1); free(v2);
    return steps + 1;
}

int global_best;

void search(int low, int high, int depth) {
    if (low >= high) return;
    int g_high = compute_g(high);
    if (g_high > global_best) global_best = g_high;
    if (low + 1 >= high || depth <= 0) return;
    if (global_best >= g_high + high - low) return;
    int mid = (low + high) / 2;
    search(low, mid, depth - 1);
    search(mid, high, depth - 1);
}

int main(void) {
    int N = 3000000;
    global_best = 0;
    search(2, N, 44);
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
                                check=True, timeout=28)
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
