"""Project Euler Problem 223: Almost right-angled triangles I.

A barely acute triangle has sides a<=b<=c with a^2+b^2=c^2+1. Find the number of
barely acute triangles with perimeter at most N.

Uses ctypes to call C code for speed, since Python loops are too slow for ~61M iterations.
"""

import ctypes
import tempfile
import os
import subprocess


def solve():
    N = 25_000_000

    # Write a small C program to do the computation
    c_code = r"""
#include <stdio.h>

int main() {
    long N = 25000000L;
    long stack[10000];
    int top = 0;

    // Seed 1: (1, 1, 1)
    stack[top++] = 1; stack[top++] = 1; stack[top++] = 1;
    // Seed 2: (1, 2, 2)
    stack[top++] = 1; stack[top++] = 2; stack[top++] = 2;

    long ans = 0;

    while (top > 0) {
        long c = stack[--top];
        long b = stack[--top];
        long a = stack[--top];
        if (a + b + c <= N) {
            ans++;
            // Child 1
            stack[top++] = a - 2*b + 2*c;
            stack[top++] = 2*a - b + 2*c;
            stack[top++] = 2*a - 2*b + 3*c;
            // Child 2 (only if a != b)
            if (a != b) {
                stack[top++] = -a + 2*b + 2*c;
                stack[top++] = -2*a + b + 2*c;
                stack[top++] = -2*a + 2*b + 3*c;
            }
            // Child 3
            stack[top++] = a + 2*b + 2*c;
            stack[top++] = 2*a + b + 2*c;
            stack[top++] = 2*a + 2*b + 3*c;
        }
    }

    printf("%ld\n", ans);
    return 0;
}
"""
    with tempfile.NamedTemporaryFile(suffix='.c', mode='w', delete=False) as f:
        f.write(c_code)
        c_file = f.name

    exe_file = c_file.replace('.c', '')
    try:
        subprocess.run(['gcc', '-O2', '-o', exe_file, c_file], check=True,
                       capture_output=True)
        result = subprocess.run([exe_file], capture_output=True, text=True, check=True)
        return int(result.stdout.strip())
    finally:
        os.unlink(c_file)
        if os.path.exists(exe_file):
            os.unlink(exe_file)


if __name__ == "__main__":
    print(solve())
