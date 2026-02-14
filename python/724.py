"""Project Euler Problem 724: Drone Delivery — embedded C."""

import subprocess, tempfile, os

def solve():
    c_code = r'''
#include <stdio.h>
#include <stdint.h>

int main() {
    int n = 100000000;
    double h = 0.0, ans = 0.0;
    for (int i = 1; i <= n; i++) {
        h += 1.0 / i;
        ans += h / i;
    }
    printf("%lld\n", (long long)(ans * n));
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
    return int(result)

if __name__ == "__main__":
    print(solve())
