"""Project Euler Problem 331 - Cross flips with C acceleration."""
import tempfile
import subprocess
import os

def solve():
    c_code = r'''
#include <stdio.h>
#include <stdint.h>
#include <math.h>

long long isqrt_ll(long long n) {
    if (n < 0) return 0;
    if (n < 2) return n;
    long long x = n, y = (x + 1) / 2;
    while (y < x) {
        x = y;
        y = (x + n / x) / 2;
    }
    return x;
}

long long T(long long n) {
    long long n2 = n * n;
    long long nm1_2 = (n - 1) * (n - 1);
    long long correction = 0;
    long long num_odd_rows = 0;
    long long x = 0;
    long long y = n - 1;
    int left_border = 0;

    while (1) {
        if (x * x + y * y < nm1_2)
            x++;

        long long prev_x = x;

        while ((x + 1) * (x + 1) + y * y < n2)
            x++;

        y--;

        int right_border = (x * x + y * y >= nm1_2);

        long long width = x - prev_x + 1;
        int odd_parity = width % 2;

        num_odd_rows += (x - prev_x - 1 +
                        (left_border ? 0 : 1) +
                        (right_border ? 0 : 1) +
                        odd_parity);

        correction += ((x - prev_x - 1 +
                       (left_border ? -1 : 1) +
                       (right_border ? -1 : 1)) *
                      (odd_parity == 1 ? 2 : -2));

        if (y <= x) {
            if (y == x) {
                if (x * x + y * y >= nm1_2)
                    correction++;
            } else {
                correction--;
                if (!left_border && !right_border)
                    num_odd_rows--;
            }
            break;
        }

        left_border = right_border;
    }

    return 2 * num_odd_rows * (n - num_odd_rows) + correction;
}

int main() {
    long long ans = 0;

    for (int i = 3; i <= 31; i++) {
        long long n = (1LL << i) - i;
        if (n == 5) {
            ans += 3;
        } else if (n % 2 == 0) {
            ans += T(n);
        }
    }

    printf("%lld\n", ans);
    return 0;
}
'''

    with tempfile.TemporaryDirectory() as tmpdir:
        src_file = os.path.join(tmpdir, "solve.c")
        bin_file = os.path.join(tmpdir, "solve")

        with open(src_file, 'w') as f:
            f.write(c_code)

        result = subprocess.run(
            ["gcc", "-O3", "-o", bin_file, src_file, "-lm"],
            capture_output=True,
            text=True
        )
        if result.returncode != 0:
            raise RuntimeError(f"Compilation failed: {result.stderr}")

        result = subprocess.run([bin_file], capture_output=True, text=True, timeout=180)
        if result.returncode != 0:
            raise RuntimeError(f"Execution failed: {result.stderr}")

        return int(result.stdout.strip())

if __name__ == "__main__":
    print(solve())
