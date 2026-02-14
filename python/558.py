"""Project Euler Problem 558: Irrational base — embedded C with GMP."""

import subprocess, tempfile, os

def solve():
    c_code = r'''
#include <stdio.h>
#include <gmp.h>

#define N_VAL 5000000
#define L_VAL 200
#define TOTAL (2 * L_VAL)

int main() {
    mpz_t a[TOTAL], target;
    for (int i = 0; i < TOTAL; i++) mpz_init(a[i]);
    mpz_init(target);

    for (int i = 0; i < TOTAL; i++) {
        if (i < 3)
            mpz_set_ui(a[i], (unsigned long)(i + 1));
        else
            mpz_add(a[i], a[i-1], a[i-3]);
    }

    long long ans = 0;
    for (int j = 1; j <= N_VAL; j++) {
        mpz_mul_ui(target, a[L_VAL], (unsigned long)j);
        mpz_mul_ui(target, target, (unsigned long)j);

        int count = 0;
        for (int i = TOTAL - 1; i >= 0; i--) {
            if (mpz_cmp(target, a[i]) >= 0) {
                mpz_sub(target, target, a[i]);
                count++;
            }
        }
        ans += count;
    }

    printf("%lld\n", ans);

    for (int i = 0; i < TOTAL; i++) mpz_clear(a[i]);
    mpz_clear(target);
    return 0;
}
'''
    with tempfile.NamedTemporaryFile(suffix='.c', delete=False) as f:
        f.write(c_code.encode())
        c_file = f.name
    exe = c_file[:-2]
    subprocess.run(['gcc', '-O3', '-o', exe, c_file, '-lgmp'], check=True, capture_output=True)
    result = subprocess.check_output([exe], timeout=280).decode().strip()
    os.unlink(c_file)
    os.unlink(exe)
    return int(result)

if __name__ == "__main__":
    print(solve())
