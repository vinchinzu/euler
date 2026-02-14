"""Project Euler Problem 796: Expected Cards Drawn — embedded C."""

import subprocess, tempfile, os

def solve():
    c_code = r'''
#include <stdio.h>

double nCr(int n, int k) {
    if (k < 0 || k > n) return 0.0;
    if (k == 0 || k == n) return 1.0;
    double result = 1.0;
    if (k > n - k) k = n - k;
    for (int i = 0; i < k; i++)
        result = result * (n - i) / (i + 1);
    return result;
}

int main() {
    int R = 13, S = 4, J = 2, D = 10;
    int L = (R * S + J) * D;

    double ans = 1.0;
    for (int n = 1; n <= L; n++) {
        for (int r = 0; r <= R; r++) {
            for (int s = 0; s <= S; s++) {
                for (int d = 0; d <= D; d++) {
                    if (r + s + d > 0) {
                        double res = 1.0;
                        int limit = ((R - r) * (S - s) + J) * (D - d);
                        for (int k = L; k > limit; k--)
                            res *= 1.0 * (k - n) / k;
                        int sign = ((r + s + d) % 2 == 0) ? 1 : -1;
                        ans -= sign * nCr(R, r) * nCr(S, s) * nCr(D, d) * res;
                    }
                }
            }
        }
    }
    printf("%.8f\n", ans);
    return 0;
}
'''
    with tempfile.NamedTemporaryFile(suffix='.c', delete=False) as f:
        f.write(c_code.encode())
        c_file = f.name
    exe = c_file[:-2]
    subprocess.run(['gcc', '-O3', '-o', exe, c_file, '-lm'], check=True, capture_output=True)
    result = subprocess.check_output([exe], timeout=280).decode().strip()
    os.unlink(c_file)
    os.unlink(exe)
    return result

if __name__ == "__main__":
    print(solve())
