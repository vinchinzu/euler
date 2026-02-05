"""Project Euler Problem 816: Shortest distance among points."""

import subprocess
import tempfile
import os

def solve():
    c_code = r'''
#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <math.h>

#define N 2000000

typedef struct {
    uint64_t x, y;
} Point;

int cmp_x(const void *a, const void *b) {
    Point *pa = (Point *)a;
    Point *pb = (Point *)b;
    if (pa->x < pb->x) return -1;
    if (pa->x > pb->x) return 1;
    return 0;
}

int main() {
    Point *points = malloc(N * sizeof(Point));

    // Blum Blum Shub sequence
    // s_0 = 290797, s_{n+1} = s_n^2 mod 50515093
    // P_n = (s_{2n}, s_{2n+1})
    uint64_t s = 290797;
    uint64_t m = 50515093;
    for (int i = 0; i < N; i++) {
        // P_i.x = s_{2i}
        points[i].x = s;
        s = (s * s) % m;
        // P_i.y = s_{2i+1}
        points[i].y = s;
        s = (s * s) % m;
    }

    // Sort by x
    qsort(points, N, sizeof(Point), cmp_x);

    double ans_sq = 1e36;
    for (int i = 0; i < N; i++) {
        for (int j = i + 1; j < N; j++) {
            double dx = (double)points[j].x - (double)points[i].x;
            double dx_sq = dx * dx;
            if (dx_sq >= ans_sq) break;
            double dy = (double)points[j].y - (double)points[i].y;
            double d_sq = dx_sq + dy * dy;
            if (d_sq < ans_sq) ans_sq = d_sq;
        }
    }

    printf("%.9f\n", sqrt(ans_sq));
    free(points);
    return 0;
}
'''

    with tempfile.NamedTemporaryFile(suffix='.c', delete=False) as f:
        f.write(c_code.encode())
        c_file = f.name

    exe = c_file[:-2]
    subprocess.run(['gcc', '-O3', '-march=native', '-o', exe, c_file, '-lm'], check=True, capture_output=True)
    result = subprocess.check_output([exe]).decode().strip()
    os.unlink(c_file)
    os.unlink(exe)
    return result

if __name__ == "__main__":
    print(solve())
