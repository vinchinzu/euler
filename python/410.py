"""Project Euler Problem 410 — Circle and tangent line.

Ported to embedded C for performance.
Sieve omega(j) for j up to A=10^8, then loop accumulating answer.
Expected answer: 799999783589946560
"""
import subprocess, tempfile, os

C_CODE = r'''
#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>

int main(void) {
    const int A = 100000000;  /* 10^8 */
    const int64_t B = 1000000000LL;  /* 10^9 */

    /* Sieve omega: number of distinct prime factors */
    uint8_t *omega = (uint8_t *)calloc(A + 1, sizeof(uint8_t));
    if (!omega) { fprintf(stderr, "alloc failed\n"); return 1; }

    for (int i = 2; i <= A; i++) {
        if (omega[i] == 0) {  /* i is prime */
            for (int j = i; j <= A; j += i) {
                omega[j]++;
            }
        }
    }

    int64_t ans = (int64_t)A * B;  /* b=c horizontal line solutions */

    for (int j = 2; j <= A; j++) {
        int64_t res;
        if (j % 2 == 0) {
            int64_t Aj = A / j;
            int64_t Bj = B / j;
            int64_t Aj_even = Aj / 2;
            int64_t Bj_even = Bj / 2;
            int64_t Aj_odd = (Aj + 1) / 2;
            int64_t Bj_odd = (Bj + 1) / 2;
            res = Aj_even * Bj_even + Aj_odd * Bj_odd;
        } else {
            res = (int64_t)(A / j) * (B / j);
        }
        ans += ((int64_t)1 << omega[j]) * res;
    }

    ans *= 4;
    printf("%lld\n", (long long)ans);

    free(omega);
    return 0;
}
'''

if __name__ == "__main__":
    with tempfile.NamedTemporaryFile(suffix='.c', delete=False) as f:
        f.write(C_CODE.encode())
        c_file = f.name
    exe = c_file[:-2]
    try:
        subprocess.run(['gcc', '-O2', '-o', exe, c_file, '-lm'], check=True, capture_output=True)
        result = subprocess.run([exe], capture_output=True, text=True, timeout=280)
        print(result.stdout.strip())
    finally:
        os.unlink(c_file)
        if os.path.exists(exe):
            os.unlink(exe)
