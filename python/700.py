"""Project Euler Problem 700: Eulercoin — embedded C for speedup."""

from __future__ import annotations

import os
import subprocess
import sys
import tempfile

C_CODE = r"""
#include <stdio.h>
#include <math.h>

typedef unsigned long long ull;

/* Extended GCD to find modular inverse */
ull mod_inverse(ull a, ull m) {
    long long t = 0, new_t = 1;
    long long r = (long long)m, new_r = (long long)(a % m);
    while (new_r != 0) {
        long long q = r / new_r;
        long long tmp;
        tmp = new_t; new_t = t - q * new_t; t = tmp;
        tmp = new_r; new_r = r - q * new_r; r = tmp;
    }
    if (t < 0) t += (long long)m;
    return (ull)t;
}

int main(void) {
    ull N_val = 1504170715041707ULL;
    ull M = 4503599627370517ULL;
    ull L = (ull)sqrt((double)M);

    ull min_el = M;
    ull ans = 0;
    ull el = N_val;
    for (ull n = 1; n <= L; n++) {
        if (el < min_el) {
            min_el = el;
            ans += el;
        }
        el = (el + N_val) % M;
    }

    ull mod_inv = mod_inverse(N_val, M);
    ull min_n = M;
    ull n_val = mod_inv;
    for (ull e = 1; e < min_el; e++) {
        if (n_val < min_n) {
            min_n = n_val;
            ans += e;
        }
        n_val = (n_val + mod_inv) % M;
    }

    printf("%llu\n", ans);
    return 0;
}
"""


def main() -> int:
    """Main entry point."""
    with tempfile.NamedTemporaryFile(suffix=".c", mode="w", delete=False) as f:
        f.write(C_CODE)
        c_path = f.name
    bin_path = c_path.replace(".c", "")
    try:
        subprocess.run(
            ["gcc", "-O2", "-o", bin_path, c_path, "-lm"],
            check=True, capture_output=True,
        )
        result = subprocess.run(
            [bin_path],
            capture_output=True, text=True, timeout=280,
        )
        answer = int(result.stdout.strip())
        print(answer)
        return answer
    finally:
        for p in [c_path, bin_path]:
            if os.path.exists(p):
                os.unlink(p)


if __name__ == "__main__":
    main()
