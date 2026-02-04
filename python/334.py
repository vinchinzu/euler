#!/usr/bin/env python3
"""Project Euler Problem 334 - Spilling the Beans

Uses the quadratic potential function approach for the 1D abelian sandpile.

Key insight: Each toppling at position i (remove 2, add 1 to neighbors) changes
the potential Phi = sum(j^2 * beans(j)) by exactly +2.
Therefore: total_topplings = (Phi_final - Phi_initial) / 2.

The final configuration is uniquely determined by:
- Total beans B (conserved)
- Moment M = sum(j * beans(j)) (conserved, since each topple changes it by 0)
- The final config has at most 1 bean per position

If M - B*(B-1)/2 is divisible by B: contiguous block at [s, s+B-1].
Otherwise: block at [s, s+B] with one gap at position g.
"""
import subprocess, tempfile, os

def solve():
    c_code = r"""
#include <stdio.h>
#include <stdlib.h>

#define NPOS 1500

typedef __int128 i128;

long long b[NPOS+2];

void generate_b() {
    long long t = 123456;
    for (int i = 1; i <= NPOS; i++) {
        if (t % 2 == 0) t /= 2;
        else t = (t / 2) ^ 926252;
        b[i] = (t % (1 << 11)) + 1;
    }
}

int main(void) {
    generate_b();

    long long B = 0;
    long long M = 0;
    i128 Phi_init = 0;
    for (int j = 1; j <= NPOS; j++) {
        B += b[j];
        M += (long long)j * b[j];
        Phi_init += (i128)j * j * b[j];
    }

    long long half_BB = B * (B - 1) / 2;
    long long s_num = M - half_BB;
    i128 Phi_final;

    if (s_num % B == 0) {
        /* Contiguous block at [s, s+B-1] */
        long long s = s_num / B;
        i128 ss = s;
        i128 BB = B;
        Phi_final = BB * ss * ss + ss * BB * (BB - 1) + BB * (BB - 1) * (2*BB - 1) / 6;
    } else {
        /* Block [s, s+B] with one gap at position g */
        long long s = s_num / B;
        if (s_num < 0 && s_num % B != 0) s--;

        long long g = (B + 1) * s + B * (B + 1) / 2 - M;

        i128 a = s;
        i128 e = s + B;
        i128 sum_sq = e * (e + 1) * (2*e + 1) / 6 - (a - 1) * a * (2*a - 1) / 6;
        Phi_final = sum_sq - (i128)g * g;
    }

    i128 total = (Phi_final - Phi_init) / 2;
    printf("%lld\n", (long long)total);
    return 0;
}
"""
    with tempfile.NamedTemporaryFile(suffix='.c', mode='w', delete=False) as f:
        f.write(c_code)
        c_file = f.name

    exe_file = c_file.replace('.c', '')
    try:
        subprocess.run(['gcc', '-O2', '-o', exe_file, c_file, '-lm'],
                      check=True, capture_output=True)
        result = subprocess.run([exe_file], capture_output=True, text=True, timeout=30)
        print(result.stdout.strip())
    finally:
        os.unlink(c_file)
        if os.path.exists(exe_file):
            os.unlink(exe_file)

solve()
