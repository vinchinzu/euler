"""Project Euler Problem 507: Shortest Lattice Vector."""

import subprocess
import tempfile
import os

def solve():
    c_code = r'''
#include <stdio.h>
#include <stdint.h>
#include <stdlib.h>
#include <math.h>

#define N 20000000
#define M 10000000LL

int64_t L1(int64_t v1, int64_t v2, int64_t v3) {
    return llabs(v1) + llabs(v2) + llabs(v3);
}

int64_t gauss(int64_t u1, int64_t u2, int64_t u3, int64_t v1, int64_t v2, int64_t v3) {
    int64_t u_L1 = L1(u1, u2, u3);
    int64_t ms[6];
    ms[0] = u1 != 0 ? (int64_t)floor((double)v1 / u1) : 0;
    ms[1] = u1 != 0 ? (int64_t)ceil((double)v1 / u1) : 0;
    ms[2] = u2 != 0 ? (int64_t)floor((double)v2 / u2) : 0;
    ms[3] = u2 != 0 ? (int64_t)ceil((double)v2 / u2) : 0;
    ms[4] = u3 != 0 ? (int64_t)floor((double)v3 / u3) : 0;
    ms[5] = u3 != 0 ? (int64_t)ceil((double)v3 / u3) : 0;

    int64_t min_w1 = -1, min_w2 = -1, min_w3 = -1;
    int64_t minD = INT64_MAX;

    for (int i = 0; i < 6; i++) {
        int64_t m = ms[i];
        if (llabs(m) < INT64_MAX / (u_L1 > 0 ? u_L1 : 1)) {
            int64_t w1 = v1 - m * u1;
            int64_t w2 = v2 - m * u2;
            int64_t w3 = v3 - m * u3;
            int64_t D = L1(w1, w2, w3);
            if (D < minD) {
                min_w1 = w1;
                min_w2 = w2;
                min_w3 = w3;
                minD = D;
            }
        }
    }

    if (L1(min_w1, min_w2, min_w3) < u_L1)
        return gauss(min_w1, min_w2, min_w3, u1, u2, u3);
    return u_L1;
}

int main() {
    int64_t *r = (int64_t*)calloc(12 * N + 1, sizeof(int64_t));
    r[2] = 1;
    for (int n = 3; n <= 12 * N; n++)
        r[n] = (r[n-1] + r[n-2] + r[n-3]) % M;

    int64_t ans = 0;
    for (int n = 1; n <= N; n++) {
        int64_t v1 = r[12*n - 11] - r[12*n - 10];
        int64_t v2 = r[12*n - 9] + r[12*n - 8];
        int64_t v3 = r[12*n - 7] * r[12*n - 6];
        int64_t w1 = r[12*n - 5] - r[12*n - 4];
        int64_t w2 = r[12*n - 3] + r[12*n - 2];
        int64_t w3 = r[12*n - 1] * r[12*n];

        if (L1(v1, v2, v3) < L1(w1, w2, w3))
            ans += gauss(v1, v2, v3, w1, w2, w3);
        else
            ans += gauss(w1, w2, w3, v1, v2, v3);
    }

    free(r);
    printf("%lld\n", ans);
    return 0;
}
'''
    with tempfile.NamedTemporaryFile(suffix='.c', delete=False) as f:
        f.write(c_code.encode())
        c_file = f.name
    exe = c_file[:-2]
    subprocess.run(['gcc', '-O3', '-o', exe, c_file, '-lm'], check=True, capture_output=True)
    result = subprocess.check_output([exe]).decode().strip()
    os.unlink(c_file)
    os.unlink(exe)
    return int(result)

if __name__ == "__main__":
    print(solve())
