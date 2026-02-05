"""Project Euler Problem 737: Coin Loops."""

import subprocess
import tempfile
import os

def solve():
    c_code = r'''
#include <stdio.h>
#include <math.h>

#define N 2020

int main() {
    double x = 1.0, y = 0.0;
    double last_cy = 0.0;
    int numLoops = 0;

    for (long long k = 2; ; k++) {
        double r2 = x * x + y * y;
        double l = sqrt(1.0 / r2 - 0.25);
        double cx = x / 2.0 - y * l;
        double cy = y / 2.0 + x * l;
        x += (cx - x) / k;
        y += (cy - y) / k;

        if (cy > 0 && last_cy < 0)
            numLoops++;
        last_cy = cy;

        if (numLoops == N) {
            printf("%lld\n", k);
            break;
        }
    }
    return 0;
}
'''
    with tempfile.NamedTemporaryFile(suffix='.c', delete=False) as f:
        f.write(c_code.encode())
        c_file = f.name
    exe = c_file[:-2]
    subprocess.run(['gcc', '-O3', '-march=native', '-lm', '-o', exe, c_file], check=True)
    result = subprocess.check_output([exe]).decode().strip()
    os.unlink(c_file)
    os.unlink(exe)
    print(result)

if __name__ == "__main__":
    solve()
