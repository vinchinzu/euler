"""Project Euler Problem 719: Number Splitting — Embedded C version.

An S-number n is a perfect square whose decimal representation can be split
into 2 or more numbers that add up to sqrt(n). Find the sum of all S-numbers
up to 10^12.
"""

import subprocess, tempfile, os

C_CODE = r"""
#include <stdio.h>

typedef long long ll;

int can_make(ll target, ll digits) {
    if (target < 0 || digits < target) return 0;
    if (digits == 0) return target == 0;
    ll pow_val = 1;
    while (pow_val <= digits) {
        if (can_make(target - digits / pow_val, digits % pow_val))
            return 1;
        pow_val *= 10;
    }
    return 0;
}

int main(void) {
    ll n = 1000000000000LL; /* 10^12 */
    ll ans = 0;
    ll max_i = 1000000; /* isqrt(10^12) */

    for (ll i = 2; i <= max_i; i++) {
        ll i_sq = i * i;
        /* mod 9 filter */
        if (i % 9 == (i_sq % 9)) {
            if (can_make(i, i_sq))
                ans += i_sq;
        }
    }

    printf("%lld\n", ans);
    return 0;
}
"""

def solve():
    with tempfile.TemporaryDirectory() as tmpdir:
        src = os.path.join(tmpdir, "p719.c")
        exe = os.path.join(tmpdir, "p719")
        with open(src, "w") as f:
            f.write(C_CODE)
        subprocess.run(["gcc", "-O2", "-o", exe, src, "-lm"], check=True)
        result = subprocess.run([exe], capture_output=True, text=True, timeout=280)
        return int(result.stdout.strip())

if __name__ == "__main__":
    print(solve())
